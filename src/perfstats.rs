use packet::Packet;
use packet::PacketType;


#[derive(Clone)]
pub struct PerfStats {
    /// total number of packets received
    pub total: usize,
    /// total number of heatbeats received
    pub num_hb: usize,
}


impl PerfStats { // where T: 'static + Send + Serialize + DeserializeOwned {
    pub fn new() -> PerfStats {
        PerfStats {
            total: 0,
            num_hb: 0,
        }
    }

    pub fn update<T>(&mut self, packet: &Packet<T>) {
        self.total += 1;
        if packet.packet_type == PacketType::Heartbeat {
            self.num_hb += 1;
        }
    }
}
