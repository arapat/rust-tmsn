use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use packet::Packet;
use perfstats::PerfStats;


/// A mock network module for the debugging purpose.
/// It bypasses the network and allows interacting with the application (that uses tmsn) through
/// the `mock_get` and `mock_send` methods.
pub struct MockNetwork {
    outbound_put: Sender<(Option<String>, Packet)>,
    outbound_get: Receiver<(Option<String>, Packet)>,
    callback: Box<dyn FnMut(String, String, Option<String>) + Sync + Send>,
    pub _perf_stats: PerfStats,
}


impl MockNetwork {
    pub fn new<T: 'static + DeserializeOwned>(
        _port: u16,
        _remote_ips: &Vec<String>,
        mut callback: Box<dyn FnMut(String, String, T) + Sync + Send>,
    ) -> MockNetwork {
        let (outbound_put, outbound_get) = channel();
        let callback: Box<dyn FnMut(String, String, Option<String>) + Sync + Send> =
            Box::new(move |sender_name, receiver_name, packet_content| {
                if packet_content.is_some() {
                    let content: T = serde_json::from_str(&packet_content.unwrap()).unwrap();
                    callback(sender_name, receiver_name, content);
                }
            });
        MockNetwork {
            outbound_put: outbound_put,
            outbound_get: outbound_get,
            callback: callback,
            _perf_stats: PerfStats::new(),
        }
    }

    pub fn get_subscribers(&self) -> Vec<String> {
        vec!["mock".to_string()]
    }

    /// Send out a packet
    pub fn send<T: Serialize>(&self, dest: Option<String>, packet_load: T) -> Result<(), ()> {
        let safe_json = serde_json::to_string(&packet_load).unwrap();
        let ret = self.outbound_put.send((dest, Packet::new(safe_json)));
        if ret.is_ok() {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Get the packet sent out by the application
    pub fn mock_get(&mut self) -> Result<(Option<String>, Packet), TryRecvError> {
        self.outbound_get.try_recv()
    }

    /// Send a packet to the application
    pub fn mock_send(&mut self, source: &String, target: &String, packet: Option<String>) {
        (self.callback)(source.clone(), target.clone(), packet)
    }
}