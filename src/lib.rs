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
/// let mut network = Network::new("local", 8080, callback);
/// let neighbors = vec![String::from("127.0.0.1")];
/// // start the network
/// network.start_network(&neighbors).unwrap();
/// sleep(Duration::from_millis(100));  // add waiting in case network is not ready
///
/// // To send out a text message
/// let message = String::from("Hello, this is a test message.");
/// network.send(message.clone()).unwrap();
///
/// fn callback(msg: String) {
/// }
/// ```
pub struct Network<T> {
    name: String,
    port: u16,
    outbound_put: Sender<Packet<T>>,
    outbound_pop: Option<Receiver<Packet<T>>>,
    callback: fn(Packet<T>) -> (),
}


impl<T> Network<T> where T: 'static + Send + Serialize + DeserializeOwned {
    /// Create a new Network object
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    ///   * `callback` - a callback function to be called when a new packet is received
    pub fn new(name: &str, port: u16, callback: fn(Packet<T>) -> ()) -> Network<T> {
        let (outbound_put, outbound_pop): (Sender<Packet<T>>, Receiver<Packet<T>>) =
            mpsc::channel();
        Network {
            name: name.to_string(),
            port: port,
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
            self.name.as_str(), init_remote_ips, self.port, true, self.outbound_pop.take().unwrap(),
            self.callback)
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

    static MESSAGE: &str = "Hello, this is a test message.";
    static mut CALLBACK_MSG: Option<String> = None;

    #[test]
    fn test() {
        let mut network = Network::new("local", 8080, callback);
        let neighbors = vec![String::from("127.0.0.1")];
        network.start_network(&neighbors).unwrap();
        sleep(Duration::from_millis(100));  // add waiting in case network is not ready

        // To send out a text message
        let message = String::from(MESSAGE);
        network.send(message.clone()).unwrap();

        // The message above is supposed to send out to all the neighbors computers specified
        // in the `network` vector, which contains only the localhost.
        sleep(Duration::from_millis(100));
        assert_eq!(unsafe { &CALLBACK_MSG }, &Some(String::from(MESSAGE)));
    }

    fn callback(msg: String) {
        unsafe { CALLBACK_MSG = Some(msg) };
    }
}
