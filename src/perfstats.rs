use packet::Packet;
use packet::PacketType;


#[derive(Clone)]
pub struct PerfStats {
    /// total number of packets received (messages + heartbeats + echos)
    pub total: usize,
    /// total number of messages received
    pub num_msg: usize,
    /// total number of heatbeats received
    pub num_hb: usize,
    /// total roundtrip time for sending a packet
    pub msg_duration: f32,
    /// total roundtrip time for sending a heartbeat
    pub hb_duration: f32,
}


impl PerfStats {
    /// create a new performance monitor
    pub fn new() -> PerfStats {
        PerfStats {
            total: 0,
            num_msg: 0,
            num_hb: 0,
            msg_duration: 0.0,
            hb_duration: 0.0,
        }
    }

    /// update the health stats
    pub fn update(&mut self, packet: &Packet) {
        self.total += 1;
        match packet.packet_type {
            PacketType::Message => {
                self.num_msg += 1;
            },
            PacketType::Echo => {
                self.msg_duration = packet.get_duration();
            },
            PacketType::Heartbeat => {
                self.num_hb += 1;
            },
            PacketType::HeartbeatEcho => {
                self.hb_duration = packet.get_duration();
            },
        }
    }

    pub fn get_avg_roundtrip_time_msg(&self) -> f32 {
        self.msg_duration / self.num_msg as f32
    }

    pub fn get_avg_roundtrip_time_hb(&self) -> f32 {
        self.hb_duration / self.num_hb as f32
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{},{},{},{}",
            self.total, self.num_msg, self.num_hb, self.msg_duration, self.hb_duration,
            self.get_avg_roundtrip_time_msg(), self.get_avg_roundtrip_time_hb())
    }
}
