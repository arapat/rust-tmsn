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
extern crate bufstream;
extern crate serde;
extern crate serde_json;

/// Establish network connections between the workers in the cluster
mod network;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;


pub struct Network<T> {
    name: String,
    port: u16,
    always_confirm: bool,
    inbound_put: Sender<T>,
    inbound_pop: Receiver<T>,
    outbound_put: Sender<T>,
    outbound_pop: Receiver<T>,
}


impl<T> Network<T> where T: 'static + Send + Serialize + DeserializeOwned {
    /// Create a new Network object
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    ///   * `always_confirm` - set to `true` to collect the success/drop rate of the communication
    pub fn new(name: &str, port: u16, always_confirm: bool) -> Network<T> {
        let (inbound_put, inbound_pop): (Sender<T>, Receiver<T>) = mpsc::channel();
        let (outbound_put, outbound_pop): (Sender<T>, Receiver<T>) = mpsc::channel();
        Network {
            name: name.to_string(),
            port: port,
            always_confirm: always_confirm,
            inbound_put: inbound_put,
            inbound_pop: inbound_pop,
            outbound_put: outbound_put,
            outbound_pop: outbound_pop,
        }
    }

    /// Start the network
    /// Parameter:
    ///   * `init_remote_ips` - a list of IPs to which this computer makes a connection initially.
    pub fn start_network(self, init_remote_ips: &Vec<String>) -> Result<(), &'static str> {
        network::start_network(
            self.name.as_str(), init_remote_ips, self.port, self.always_confirm, self.inbound_put,
            self.outbound_pop)
    }

    /// Send out a packet
    pub fn send(self, packet: T) -> Result<(), ()> {
        let ret = self.outbound_put.send(packet);
        if ret.is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Receive a packet
    pub fn receive(self) -> Result<T, ()> {
        let ret = self.inbound_pop.recv();
        if ret.is_ok() {
            Ok(ret.unwrap())
        } else {
            Err(())
        }
    }

    /// Return a summary of the network communication
    pub fn get_health() {
    }
}
