use std::time::SystemTime;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;


#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum PacketType {
    Message,
    Echo,
    Heartbeat,
    HeartbeatEcho,
}


#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    content: Option<T>,
    sent_time: SystemTime,
    receive_time: Option<SystemTime>,
    packet_type: PacketType,
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
