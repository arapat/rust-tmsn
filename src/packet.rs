use std::time::SystemTime;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;


pub type JsonFormat<T> = (String, u32, Packet<T>);

/// Types of packets
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum PacketType {
    /// actual information load to be sent out
    Message,
    /// echo message to indicates a message was received
    Echo,
    /// heartbeat message to monitor the network health
    Heartbeat,
    /// echo message to indicates a hearbeat was received
    HeartbeatEcho,
}


/// Packet 
#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    /// Actual workload of the packet
    pub content: Option<T>,
    /// Packet sent out time
    pub sent_time: SystemTime,
    /// Packet receive time
    pub receive_time: Option<SystemTime>,
    /// Type of the packet
    pub packet_type: PacketType,
}


impl<T> Packet<T> where T: 'static + Send + Serialize + DeserializeOwned {
    pub fn new(msg: T) -> Packet<T> {
        Packet {
            content: Some(msg),
            sent_time: SystemTime::now(),
            receive_time: None,
            packet_type: PacketType::Message,
        }
    }

    pub fn get_hb() -> Packet<T> {
        Packet {
            content: None,
            sent_time: SystemTime::now(),
            receive_time: None,
            packet_type: PacketType::Heartbeat,
        }
    }

    pub fn mark_received(&mut self) {
        self.receive_time = Some(SystemTime::now());
    }

    pub fn get_receipt(&self) -> Packet<T> {
        assert!(self.receive_time.is_some());
        assert!(self.packet_type == PacketType::Message ||
                self.packet_type == PacketType::Heartbeat);
        let echo_type =
            if self.packet_type == PacketType::Message {
                PacketType::Echo
            } else {
                PacketType::HeartbeatEcho
            };
        Packet {
            content: None,
            sent_time: self.sent_time.clone(),
            receive_time: self.receive_time.clone(),
            packet_type: echo_type,
        }
    }
}
