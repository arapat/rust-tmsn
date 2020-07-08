/*!
`rust_tmsn` is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Use rust_tmsn with Cargo

Please download the source code of `rust_tmsn` to your computer, and
append following lines to the `Cargo.toml` file in your project
with the path should be the actual location of `rust_tmsn`.

```ignore
[dependencies]
rust_tmsn = { path = "../rust-tmsn" }
```
*/
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate bufstream;
extern crate serde;
extern crate serde_json;

/// Establish network connections between the workers in the cluster
mod network;
/// The packet sent out via network
pub mod packet;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;


/// A structure for communicating over the network in an asynchronous, non-blocking manner
///
/// Example:
/// ```
/// ```
pub struct Network<T: 'static> {
    outbound_put: Sender<T>,
}


impl<T> Network<T> where T: 'static + Send + Serialize + DeserializeOwned {
    /// Create a new Network object
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
        callback: Box<dyn FnMut(T) + Sync + Send>,
    ) -> Network<T> {
        let (outbound_put, outbound_pop): (Sender<T>, Receiver<T>) = mpsc::channel();
        network::start_network(name, remote_ips, port, true, outbound_pop, callback).unwrap();
        Network {
            outbound_put: outbound_put,
        }
    }

    /// Send out a packet
    pub fn send(&self, packet: T) -> Result<(), ()> {
        let ret = self.outbound_put.send(packet);
        if ret.is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Set heartbeat interval
    pub fn set_health_parameter() {
    }

    /// Return a summary of the network communication
    pub fn get_health() {
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
        let network = Network::new("local", 8080, &neighbors, Box::new(move |msg: String| {
            let mut t = t.write().unwrap();
            *t = Some(msg.clone());
        }));
        sleep(Duration::from_millis(100));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from(MESSAGE);
        network.send(message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        sleep(Duration::from_millis(100));
        assert_eq!(*(output.read().unwrap()), Some(String::from(MESSAGE)));
    }
}
