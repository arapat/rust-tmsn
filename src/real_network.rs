use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use network;
use packet::Packet;
use perfstats::PerfStats;
use HEAD_NODE;
use LockedStream;


pub struct RealNetwork {
    outbound_put: Sender<(Option<String>, Packet)>,
    perf_stats: Arc<RwLock<PerfStats>>,
    heartbeat_interv_secs: Arc<RwLock<u64>>,
    send_streams: LockedStream,
}


impl RealNetwork {
    /// Create a new Network object
    ///
    /// Parameters:
    ///   * `name` - the local computer name.
    ///   * `port` - the port number that the machines in the network are listening to.
    ///   `port` has to be the same value for all machines.
    ///   * `remote_ips` - a list of IPs to which this computer makes a connection initially.
    ///   * `callback` - a callback function to be called when a new packet is received
    pub fn new<T: 'static + DeserializeOwned>(
        port: u16,
        remote_ips: &Vec<String>,
        mut callback: Box<dyn FnMut(String, T) + Sync + Send>,
    ) -> RealNetwork {
        // start the network
        let (outbound_put, outbound_pop):
            (Sender<(Option<String>, Packet)>, Receiver<(Option<String>, Packet)>)
            = mpsc::channel();
        let perf_stats = Arc::new(RwLock::new(PerfStats::new()));
        let ps = perf_stats.clone();
        let sender_state = network::start_network(
            remote_ips, port, true, outbound_put.clone(), outbound_pop,
            Box::new(move |sender_name, packet| {
                let mut ps = ps.write().unwrap();
                ps.update(sender_name.clone(), &packet);
                drop(ps);
                if packet.is_workload() {
                    let content: T = serde_json::from_str(&packet.content.unwrap()).unwrap();
                    callback(sender_name, content);
                }
            }));

        // check if network is ready
        let send_streams = sender_state.unwrap();
        loop {
            let s = send_streams.read().unwrap();
            if s.len() == remote_ips.len() {
                break;
            }
            drop(s);
            sleep(Duration::from_millis(500));
        }

        // send heart beat signals
        let heartbeat_interv_secs = Arc::new(RwLock::new(30));
        let head_ip = HEAD_NODE.to_string();
        let outbound = outbound_put.clone();
        let interval = heartbeat_interv_secs.clone();
        let ps = perf_stats.clone();
        std::thread::spawn(move|| {
            loop {
                let ps = ps.read().unwrap();
                outbound.send((Some(head_ip.clone()), Packet::get_hb(&ps))).unwrap();
                drop(ps);

                let interval = interval.read().unwrap();
                let secs = *interval;
                drop(interval);
                sleep(Duration::from_secs(secs));
            }
        });

        RealNetwork {
            outbound_put: outbound_put.clone(),
            perf_stats: perf_stats,
            heartbeat_interv_secs: heartbeat_interv_secs,
            send_streams: send_streams,
        }
    }

    /// Get the list of the address of the subscribed machines
    pub fn get_subscribers(&mut self) -> Vec<String> {
        let streams = self.send_streams.read().unwrap();
        let subscribers: Vec<String> = streams.iter().map(|(s, _)| s.clone()).collect();
        drop(streams);
        subscribers
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

    /// Set heartbeat interval
    ///
    /// Parameter:
    ///   * hb_interval_secs: the time interval between sending out the heartbeat signals
    ///     (unit: seconds)
    pub fn set_health_parameter(&mut self, hb_interval_secs: u64) {
        let mut val = self.heartbeat_interv_secs.write().unwrap();
        *val = hb_interval_secs;
    }

    /// Return a summary of the network communication
    pub fn get_health(&self) -> PerfStats {
        let ps = self.perf_stats.read().unwrap();
        (*ps).clone()
    }
}