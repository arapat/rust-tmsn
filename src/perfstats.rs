use std::collections::HashMap;

use packet::Packet;
use packet::PacketType;


#[derive(Clone, Serialize, Deserialize)]
pub struct PerfStats {
    /// total number of packets received (messages + heartbeats + echos)
    pub total: usize,
    /// total number of messages received
    pub num_msg: usize,
    /// total number of echo messages received
    pub num_msg_echo: usize,
    /// total number of heatbeats received
    pub num_hb: usize,
    /// total number of echo messages to the hearbeats received
    pub num_hb_echo: usize,
    /// total roundtrip time for sending a packet
    pub msg_duration: u128,
    /// total roundtrip time for sending a heartbeat
    pub hb_duration: u128,
    /// perf stats of other machines
    pub others: HashMap<String, PerfStats>,
}


impl PerfStats {
    /// create a new performance monitor
    pub fn new() -> PerfStats {
        PerfStats {
            total: 0,
            num_msg: 0,
            num_msg_echo: 0,
            num_hb: 0,
            num_hb_echo: 0,
            msg_duration: 0,
            hb_duration: 0,
            others: HashMap::new(),
        }
    }

    pub fn from_json(json: &String) -> PerfStats {
        serde_json::from_str(json).unwrap()
    }

    fn new_local(ps: &PerfStats) -> PerfStats {
        PerfStats {
            total: ps.total,
            num_msg: ps.num_msg,
            num_msg_echo: ps.num_msg_echo,
            num_hb: ps.num_hb,
            num_hb_echo: ps.num_hb_echo,
            msg_duration: ps.msg_duration,
            hb_duration: ps.hb_duration,
            others: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&PerfStats::new_local(self)).unwrap()
    }

    /// update the health stats
    pub fn update(&mut self, name: String, packet: &Packet) {
        self.total += 1;
        match packet.packet_type {
            PacketType::Message => {
                self.num_msg += 1;
            },
            PacketType::Echo => {
                self.msg_duration += packet.get_duration();
                self.num_msg_echo += 1;
            },
            PacketType::Heartbeat => {
                self.num_hb += 1;
                self.others.insert(
                    name,
                    PerfStats::from_json(packet.content.as_ref().unwrap())
                );
            },
            PacketType::HeartbeatEcho => {
                self.hb_duration += packet.get_duration();
                self.num_hb_echo += 1;
            },
        }
    }

    pub fn get_avg_roundtrip_time_msg(&self) -> f64 {
        self.msg_duration as f64 / self.num_msg as f64
    }

    pub fn get_avg_roundtrip_time_hb(&self) -> f64 {
        self.hb_duration as f64 / self.num_hb as f64
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{},{},{},{},{},{}",
            self.total, self.num_msg, self.num_msg_echo, self.num_hb, self.num_hb_echo,
            self.msg_duration, self.hb_duration,
            self.get_avg_roundtrip_time_msg(), self.get_avg_roundtrip_time_hb())
    }
}
