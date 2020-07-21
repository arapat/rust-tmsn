/*!
`tmsn` is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Use `tmsn` with Cargo

Please download the source code of `tmsn` to your computer, and
append following lines to the `Cargo.toml` file in your project
with the path should be the actual location of `tmsn`.

```ignore
[dependencies]
tmsn = { path = "../tmsn" }
```
*/
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate bufstream;
extern crate serde;
extern crate serde_json;

/// Establish network connections between the workers in the cluster
mod network;
/// Struct for reporting the health of the network
pub mod perfstats;
/// The packet sent out via network
pub mod packet;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use bufstream::BufStream;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use packet::Packet;
use perfstats::PerfStats;


type Stream = Vec<(String, BufStream<TcpStream>)>;
type LockedStream = Arc<RwLock<Stream>>;
const HEAD_NODE: &str = "HEAD_NODE";

/// A structure for communicating over the network in an asynchronous, non-blocking manner
///
/// Example:
/// ```
/// use tmsn::Network;
/// use std::thread::sleep;
/// use std::time::Duration;
/// use std::sync::Arc;
/// use std::sync::RwLock;
///
/// static MESSAGE: &str = "Hello, this is a test message.";
///
/// let neighbors = vec![String::from("127.0.0.1")];
/// let output: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
/// let t = output.clone();
/// let network = Network::new("local", 8080, &neighbors, Box::new(move |msg: String| {
///     let mut t = t.write().unwrap();
///     *t = Some(msg.clone());
/// }));
/// sleep(Duration::from_millis(100));  // add waiting in case network is not ready
///
/// // To send out a text message
/// let message = String::from(MESSAGE);
/// network.send(message.clone()).unwrap();
///
/// // The message above is supposed to send out to all the neighbors computers specified
/// // in the `network` vector, which contains only the localhost.
/// sleep(Duration::from_millis(100));
/// assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));
/// ```
pub struct Network {
    outbound_put: Sender<(Option<String>, Packet)>,
    perf_stats: Arc<RwLock<PerfStats>>,
    heartbeat_interv_secs: Arc<RwLock<u64>>,
    _send_streams: LockedStream,
}


impl Network {
    /// Create a new Network object
    ///
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    ///   * `remote_ips` - a list of IPs to which this computer makes a connection initially.
    ///   * `callback` - a callback function to be called when a new packet is received
    pub fn new<T: 'static + DeserializeOwned>(
        name: &str,
        port: u16,
        remote_ips: &Vec<String>,
        mut callback: Box<dyn FnMut(T) + Sync + Send>,
    ) -> Network {
        // start the network
        let (outbound_put, outbound_pop):
            (Sender<(Option<String>, Packet)>, Receiver<(Option<String>, Packet)>)
            = mpsc::channel();
        let perf_stats = Arc::new(RwLock::new(PerfStats::new()));
        let ps = perf_stats.clone();
        let sender_state = network::start_network(
            name, remote_ips, port, true, outbound_put.clone(), outbound_pop,
            Box::new(move |packet| {
                let mut ps = ps.write().unwrap();
                ps.update(&packet);
                drop(ps);
                if packet.is_workload() {
                    let content: T = serde_json::from_str(&packet.content.unwrap()).unwrap();
                    callback(content);
                }
            }));

        // send heart beat signals
        let heartbeat_interv_secs = Arc::new(RwLock::new(30));
        let head_ip = HEAD_NODE.to_string();
        let outbound = outbound_put.clone();
        let interval = heartbeat_interv_secs.clone();
        let ps = perf_stats.clone();
        std::thread::spawn(move|| {
            loop {
                let ps = ps.read().unwrap();
                outbound.send((Some(head_ip.clone()), Packet::get_hb(ps.to_string()))).unwrap();
                drop(ps);
                {
                    let secs = interval.read().unwrap();
                    sleep(Duration::from_secs(*secs));
                }
            }
        });
        let send_streams = sender_state.unwrap();
        loop {
            let s = send_streams.read().unwrap();
            if s.len() > 0 {
                break;
            }
            drop(s);
            sleep(Duration::from_millis(500));
        }

        Network {
            outbound_put: outbound_put.clone(),
            perf_stats: perf_stats,
            heartbeat_interv_secs: heartbeat_interv_secs,
            _send_streams: send_streams,
        }
    }

    /// Send out a packet
    pub fn send<T: Serialize>(&self, packet_load: T) -> Result<(), ()> {
        let safe_json = serde_json::to_string(&packet_load).unwrap();
        let ret = self.outbound_put.send((None, Packet::new(safe_json)));
        if ret.is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Set heartbeat interval
    ///
    /// Parameter:
    ///   * hb_interval_secs: the time interval between sending out the heartbeat signals
    ///     (unit: seconds)
    pub fn set_health_parameter(&mut self, hb_interval_secs: u64) {
        let mut val = self.heartbeat_interv_secs.write().unwrap();
        *val = hb_interval_secs;
    }

    /// Return a summary of the network communication
    pub fn get_health(&self) -> PerfStats {
        let ps = self.perf_stats.read().unwrap();
        (*ps).clone()
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use super::Network;
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;
    use std::thread::sleep;
    use std::time::Duration;
    use std::sync::Arc;
    use std::sync::RwLock;
    use tests::rand::{thread_rng, Rng};
    use tests::rand::distributions::Alphanumeric;

    static MESSAGE: &str = "Hello, this is a test message.";

    fn test(neighbors: Vec<String>, port: u16) {
        let output: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let t = output.clone();
        let mut network = Network::new("local", port, &neighbors, Box::new(move |msg: String| {
            let mut t = t.write().unwrap();
            *t = Some(msg.clone());
        }));
        network.set_health_parameter(1);
        sleep(Duration::from_millis(1000));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from(MESSAGE);
        network.send(message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        sleep(Duration::from_millis(100));
        assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));

        sleep(Duration::from_secs(1));
        let health = network.get_health();
        assert_eq!(health.total, 2 + 2 * 2);
        assert_eq!(health.num_hb, 2);
        // println!("roundtrip time, {}, {}",
        //     health.get_avg_roundtrip_time_msg(), health.get_avg_roundtrip_time_msg());
    }

    fn stress_test(neighbors: Vec<String>, port: u16, load_size: usize) {
        let output: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let t = output.clone();
        let mut network = Network::new("local", port, &neighbors, Box::new(move |msg: String| {
            let mut t = t.write().unwrap();
            *t = Some(msg.clone());
        }));
        network.set_health_parameter(1);

        // To send out a text message
        let message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(load_size)
            .collect();

        // only scanners send out packets
        if neighbors.len() == 0 {
            for _ in 0..100 {
                network.send(message.clone()).unwrap();
                sleep(Duration::from_millis(50));
            }
        } else {
            sleep(Duration::from_millis(8000));  // add waiting in case network is not ready
        }
        let health = network.get_health();
        println!("stress perf,{},{}", load_size, health.to_string());
    }

    #[test]
    fn test_local() {
        test(vec![String::from("127.0.0.1")], 8080);
    }

    #[test]
    fn test_network() {
        let mut neighbors = vec![];
        if let Ok(lines) = read_lines("./neighbors.txt") {
            lines.for_each(|line| {
                neighbors.push(line.unwrap());
            });
            test(neighbors, 8081);
        }
    }

    // #[test]
    // fn test_stress_local() {
    //     stress_test(vec![String::from("127.0.0.1")], 8082, 1024);
    // }

    #[test]
    fn stress_test_network() {
        let mut neighbors = vec![];
        if let Ok(lines) = read_lines("./neighbors.txt") {
            lines.for_each(|line| {
                neighbors.push(line.unwrap());
            });
            for (index, load_size) in ((0..200).step_by(20)).enumerate() {
                stress_test(neighbors.clone(), 8082 + index as u16, 1024 * load_size);
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
