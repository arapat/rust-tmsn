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

/// Struct for reporting the health of the network
pub mod perfstats;
/// The packet sent out via network
pub mod packet;
/// Network module
pub mod real_network;
/// Mock network module for the debugging purpose
pub mod mock_network;
/// Establish network connections between the workers in the cluster
mod network;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::TryRecvError;

use bufstream::BufStream;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use mock_network::MockNetwork;
use real_network::RealNetwork;
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
/// let network = Network::new(
///     8080, &neighbors,
///     Box::new(move |sender: String, msg: String| {
///         let mut t = t.write().unwrap();
///         *t = Some(msg.clone());
///     }),
///     false,
/// );
/// sleep(Duration::from_millis(100));  // add waiting in case network is not ready
///
/// // To send out a text message
/// let message = String::from(MESSAGE);
/// network.send(None, message.clone()).unwrap();
///
/// // The message above is supposed to send out to all the neighbors computers specified
/// // in the `network` vector, which contains only the localhost.
/// sleep(Duration::from_millis(100));
/// assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));
/// ```
pub enum Network {
	Real(RealNetwork),
	Mocked(MockNetwork),
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
    ///   * `debug` - set to true to run in the debugging mode (see MockNetwork)
    pub fn new<T: 'static + DeserializeOwned>(
        port: u16,
        remote_ips: &Vec<String>,
        callback: Box<dyn FnMut(String, T) + Sync + Send>,
        debug: bool,
    ) -> Network {
        if debug {
            Network::Mocked(MockNetwork::new(port, remote_ips, callback))
        } else {
            Network::Real(RealNetwork::new(port, remote_ips, callback))
        }
    }

    /// Get the list of the address of the subscribed machines
    pub fn get_subscribers(&mut self) -> Vec<String> {
        match self {
            Network::Real(network) => network.get_subscribers(),
            Network::Mocked(mocked) => mocked.get_subscribers(),
        }
    }

    /// Send out a packet
    ///
    /// Parameter:
    ///     * dest: the address of the destination machine. Set to `None` for broadcasting
    ///     * packet_load: the workload message to be sent out
    pub fn send<T: Serialize>(&self, dest: Option<String>, packet_load: T) -> Result<(), ()> {
        match self {
            Network::Real(network) => network.send(dest, packet_load),
            Network::Mocked(mocked) => mocked.send(dest, packet_load),
        }
    }

    /// Set heartbeat interval
    ///
    /// Parameter:
    ///   * hb_interval_secs: the time interval between sending out the heartbeat signals
    ///     (unit: seconds)
    pub fn set_health_parameter(&mut self, hb_interval_secs: u64) {
        match self {
            Network::Real(network) => network.set_health_parameter(hb_interval_secs),
            Network::Mocked(_) => {},
        }
    }

    /// Return a summary of the network communication
    pub fn get_health(&self) -> PerfStats {
        match self {
            Network::Real(network) => network.get_health(),
            Network::Mocked(mocked) => mocked._perf_stats.clone(),
        }
    }

    pub fn mock_get(&mut self) -> Result<(Option<String>, Packet), TryRecvError> {
        match self {
            Network::Real(_) => Err(TryRecvError::Empty),
            Network::Mocked(mocked) => mocked.mock_get(),
        }
    }


    pub fn mock_send(&mut self, source: &String, packet: Option<String>) {
        match self {
            Network::Real(_) => {},
            Network::Mocked(mocked) => mocked.mock_send(source, packet),
        }
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
        let mut network = Network::new(
            port, &neighbors,
            Box::new(move |_s: String, msg: String| {
                let mut t = t.write().unwrap();
                *t = Some(msg.clone());
            }),
            false,
        );
        network.set_health_parameter(1);
        sleep(Duration::from_millis(1000));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from(MESSAGE);
        network.send(None, message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        sleep(Duration::from_secs(1));
        assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));

        sleep(Duration::from_secs(1));
        let health = network.get_health();
        assert_eq!(health.num_msg, 1);
        assert_eq!(health.num_msg_echo, 1);
        assert!(health.num_hb > 0);
    }

    fn stress_test(neighbors: Vec<String>, port: u16, load_size: usize, pkg_interval: u64) {
        println!("load_size,{},pkg_interval,{}", load_size, pkg_interval);

        let output: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let t = output.clone();
        let mut network = Network::new(port, &neighbors,
            Box::new(move |_s: String, msg: String| {
                let mut t = t.write().unwrap();
                *t = Some(msg.clone());
            }),
            false,
        );
        network.set_health_parameter(1);

        // To send out a text message
        let message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(load_size)
            .collect();

        // only scanners send out packets
        if neighbors.len() == 0 {
            for _ in 0..100 {
                network.send(None, message.clone()).unwrap();
                sleep(Duration::from_millis(pkg_interval));
            }
        } else {
            sleep(Duration::from_millis(8000));  // add waiting in case network is not ready
        }
        let health = network.get_health();

        println!("stress perf,{},local,{}", load_size, health.to_string());
        for (addr, health) in health.others.iter() {
            println!("stress perf,{},{},{}", load_size, addr, health.to_string());
        }
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
                let addr = line.unwrap();
                if addr.trim().len() > 0 {
                    neighbors.push(addr.to_string());
                }
            });
            test(neighbors, 8081);
        }
    }

    // #[test]
    // fn test_stress_local() {
    //     stress_test(vec![String::from("127.0.0.1")], 8082, 1024);
    // }

    fn stress_test_network(pkg_interval: u64) {
        let load_mul: Vec<usize> = vec![1, 1, 5, 10, 100, 200, 1];
        let num_loads = load_mul.len();
        let mut neighbors = vec![];
        if let Ok(lines) = read_lines("./neighbors.txt") {
            lines.for_each(|line| {
                neighbors.push(line.unwrap());
            });
            for repeat in 0..10 {
                println!("\nstart new test, {}", repeat);
                for (index, load_size) in load_mul.iter().enumerate() {
                    let port = 8082 + (repeat * num_loads + index);
                    stress_test(neighbors.clone(), port as u16, 1024 * load_size, pkg_interval);
                }
            }
        }
    }

    #[test]
    fn stress_test_network_10() {
        stress_test_network(10);
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
