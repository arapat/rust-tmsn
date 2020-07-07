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
mod packet;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use packet::Packet;


/// A structure for communicating over the network in an asynchronous, non-blocking manner
///
/// Example:
/// ```
/// use tmsn::Network;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let mut network = Network::new("local", 8080, true);
/// let neighbors = vec![String::from("127.0.0.1")];
/// // start the network
/// network.start_network(&neighbors).unwrap();
/// sleep(Duration::from_millis(100));  // add waiting in case network is not ready
///
/// // To send out a text message
/// let message = String::from("Hello, this is a test message.");
/// network.send(message.clone()).unwrap();
///
/// // The message above is supposed to send out to all the neighbors computers specified
/// // in the `network` vector, which contains only the localhost.
/// let data_received = network.receive().unwrap();
/// assert_eq!(data_received, message);
/// ```
pub struct Network<T> {
    name: String,
    port: u16,
    inbound_put: Sender<Packet<T>>,
    inbound_pop: Receiver<Packet<T>>,
    outbound_put: Sender<Packet<T>>,
    outbound_pop: Option<Receiver<Packet<T>>>,
    callback: fn(T) -> (),
}


impl<T> Network<T> where T: 'static + Send + Serialize + DeserializeOwned {
    /// Create a new Network object
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    pub fn new(name: &str, port: u16, callback: fn(T) -> ()) -> Network<T> {
        let (inbound_put, inbound_pop): (Sender<Packet<T>>, Receiver<Packet<T>>) = mpsc::channel();
        let (outbound_put, outbound_pop): (Sender<Packet<T>>, Receiver<Packet<T>>) =
            mpsc::channel();
        Network {
            name: name.to_string(),
            port: port,
            inbound_put: inbound_put,
            inbound_pop: inbound_pop,
            outbound_put: outbound_put,
            outbound_pop: Some(outbound_pop),
            callback: callback,
        }
    }

    /// Start the network
    /// Parameter:
    ///   * `init_remote_ips` - a list of IPs to which this computer makes a connection initially.
    pub fn start_network(&mut self, init_remote_ips: &Vec<String>) -> Result<(), &'static str> {
        network::start_network(
            self.name.as_str(), init_remote_ips, self.port, true,
            self.inbound_put.clone(), self.outbound_pop.take().unwrap())
    }

    /// Send out a packet
    pub fn send(&self, packet: T) -> Result<(), ()> {
        let ret = self.outbound_put.send(Packet::new(packet));
        if ret.is_ok() {
            Ok(())
        } else {
            Err(())
        }
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

    #[test]
    fn test() {
        let mut network = Network::new("local", 8080, true);
        let neighbors = vec![String::from("127.0.0.1")];
        network.start_network(&neighbors).unwrap();
        sleep(Duration::from_millis(100));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from("Hello, this is a test message.");
        network.send(message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        let data_received = network.receive().unwrap();
        assert_eq!(data_received, message);
    }
}
