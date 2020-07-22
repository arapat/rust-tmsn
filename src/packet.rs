use std::time::SystemTime;

use PerfStats;


// local machine name, Packet index, packet
pub type JsonFormat = (String, u32, Packet);

/// Types of packets
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
pub struct Packet {
    /// Actual workload of the packet
    pub content: Option<String>,
    /// Packet sent out time
    pub sent_time: SystemTime,
    /// Packet receive time
    pub receive_time: Option<SystemTime>,
    /// Type of the packet
    pub packet_type: PacketType,
}


impl Packet {
    pub fn new(msg: String) -> Packet {
        Packet {
            content: Some(msg),
            sent_time: SystemTime::now(),
            receive_time: None,
            packet_type: PacketType::Message,
        }
    }

    pub fn get_hb(perf_stats: &PerfStats) -> Packet {
        let safe_json = serde_json::to_string(perf_stats).unwrap();
        Packet {
            content: Some(safe_json),
            sent_time: SystemTime::now(),
            receive_time: None,
            packet_type: PacketType::Heartbeat,
        }
    }

    pub fn get_hb_workload(&self) -> PerfStats {
        assert_eq!(self.packet_type, PacketType::Heartbeat);
        serde_json::from_str(self.content.as_ref().unwrap())
            .unwrap()
    }

    pub fn mark_received(&mut self) {
        self.receive_time = Some(SystemTime::now());
    }

    pub fn get_receipt(&self) -> Option<Packet> {
        if !self.is_workload() && self.packet_type != PacketType::Heartbeat {
            return None;
        }
        let echo_type =
            if self.is_workload() {
                PacketType::Echo
            } else {
                PacketType::HeartbeatEcho
            };
        Some(Packet {
            content: None,
            sent_time: self.sent_time.clone(),
            receive_time: self.receive_time.clone(),
            packet_type: echo_type,
        })
    }

    pub fn is_workload(&self) -> bool {
        self.packet_type == PacketType::Message
    }

    pub fn get_duration(&self) -> u128 {
        self.receive_time.unwrap()
            .duration_since(self.sent_time)
            .unwrap()
            .as_micros()
    }
}
