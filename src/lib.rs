/*!
`rust_tmsn` is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Use rust_tmsn with Cargo

Please download the source code of `rust_tmsn` to your computer, and
append following lines to the `Cargo.toml` file in your project
with the path should be the actual location of `rust_tmsn`.

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

use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use packet::Packet;
use perfstats::PerfStats;


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
pub struct Network<T: 'static> {
    outbound_put: Sender<(Option<String>, Packet<T>)>,
    perf_stats: Arc<RwLock<PerfStats>>,
    heartbeat_interv_secs: Arc<RwLock<u64>>,
}


impl<T> Network<T> where T: 'static + Send + Serialize + DeserializeOwned {
    /// Create a new Network object
    ///
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    ///   * `remote_ips` - a list of IPs to which this computer makes a connection initially.
    ///   * `callback` - a callback function to be called when a new packet is received
    pub fn new(
        name: &str,
        port: u16,
        remote_ips: &Vec<String>,
        mut callback: Box<dyn FnMut(T) + Sync + Send>,
    ) -> Network<T> {
        assert!(remote_ips.len() > 0);

        // start the network
        let (outbound_put, outbound_pop):
            (Sender<(Option<String>, Packet<T>)>, Receiver<(Option<String>, Packet<T>)>)
            = mpsc::channel();
        let perf_stats = Arc::new(RwLock::new(PerfStats::new()));
        let ps = perf_stats.clone();
        network::start_network(
            name, remote_ips, port, true, outbound_put.clone(), outbound_pop,
            Box::new(move |packet| {
                let mut ps = ps.write().unwrap();
                ps.update(&packet);
                if packet.is_workload() {
                    callback(packet.content.unwrap());
                }
            })).unwrap();

        // send heart beat signals
        let heartbeat_interv_secs = Arc::new(RwLock::new(30));
        let head_ip = remote_ips[0].clone();
        let outbound = outbound_put.clone();
        let interval = heartbeat_interv_secs.clone();
        std::thread::spawn(move|| {
            loop {
                outbound.send((Some(head_ip.clone()), Packet::<T>::get_hb())).unwrap();
                {
                    let secs = interval.read().unwrap();
                    sleep(Duration::from_secs(*secs));
                }
            }
        });

        Network {
            outbound_put: outbound_put.clone(),
            perf_stats: perf_stats,
            heartbeat_interv_secs: heartbeat_interv_secs,
        }
    }

    /// Send out a packet
    pub fn send(&self, packet: T) -> Result<(), ()> {
        let ret = self.outbound_put.send((None, Packet::new(packet)));
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
    use super::Network;
    use std::thread::sleep;
    use std::time::Duration;
    use std::sync::Arc;
    use std::sync::RwLock;

    static MESSAGE: &str = "Hello, this is a test message.";

    #[test]
    fn test() {
        let neighbors = vec![String::from("127.0.0.1")];
        let output: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let t = output.clone();
        let mut network = Network::new("local", 8080, &neighbors, Box::new(move |msg: String| {
            let mut t = t.write().unwrap();
            *t = Some(msg.clone());
        }));
        network.set_health_parameter(1);
        sleep(Duration::from_millis(100));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from(MESSAGE);
        network.send(message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        sleep(Duration::from_millis(100));
        assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));

        sleep(Duration::from_secs(1));
        let health = network.get_health();
        assert_eq!(health.total, 2 + 2);
        assert_eq!(health.num_hb, 1);
    }
}
