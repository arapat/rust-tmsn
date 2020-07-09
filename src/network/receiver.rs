use bufstream::BufStream;
use std::collections::HashSet;
use std::io::BufRead;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use serde::de::DeserializeOwned;

use std::thread::sleep;
use std::thread::spawn;

use packet::JsonFormat;
use packet::Packet;


// Start all receiver routines
pub fn start_receiver<T: 'static + Send + DeserializeOwned>(
        name: String, port: u16,
        outbound_send: Sender<(Option<String>, Packet<T>)>,
        callback: Box<dyn FnMut(Packet<T>) + Sync + Send>,
        remote_ip_recv: Receiver<SocketAddr>) {
    spawn(move|| {
        // If a new neighbor occurs, launch receiver to receive data from it
        info!("now entering receivers listener");
        let mut receivers = HashSet::new();
        let f = Arc::new(RwLock::new(callback));
        while let Ok(mut remote_addr) = remote_ip_recv.recv() {
            remote_addr.set_port(port);
            if !receivers.contains(&remote_addr) {
                let name_clone = name.clone();
                let callback = f.clone();
                let addr = remote_addr.clone();
                let outbound = outbound_send.clone();
                receivers.insert(remote_addr.clone());
                spawn(move || {
                    let mut tcp_stream = None;
                    while tcp_stream.is_none() {
                        tcp_stream = match TcpStream::connect(remote_addr) {
                            Ok(_tcp_stream) => Some(_tcp_stream),
                            Err(error) => {
                                info!("(retry in 2 secs) Error: {}.
                                    Failed to connect to remote address {}", error, remote_addr);
                                sleep(Duration::from_secs(2));
                                None
                            }
                        };
                    }
                    let stream = BufStream::new(tcp_stream.unwrap());
                    receiver(name_clone, addr, stream, outbound, callback);
                });
            } else {
                info!("(Skipped) Receiver exists for {}", remote_addr);
            }
        }
    });
}


// Core receiver routine
pub fn receiver<T: 'static + DeserializeOwned>(
        name: String, remote_ip: SocketAddr, mut stream: BufStream<TcpStream>,
        outbound_send: Sender<(Option<String>, Packet<T>)>,
        callback: Arc<RwLock<Box<dyn FnMut(Packet<T>) + Sync + Send>>>,
) {
    info!("Receiver started from {} to {}", name, remote_ip);
    let mut idx = 0;
    loop {
        let mut json = String::new();
        let read_result = stream.read_line(&mut json);
        if let Err(_) = read_result {
            error!("Cannot read the remote model from network.");
            continue;
        }

        if json.trim().len() != 0 {
            let remote_packet = serde_json::from_str(&json);
            if let Err(err) = remote_packet {
                error!("Cannot parse the JSON description of the remote model from {}. \
                        Message ID {}, JSON string is `{}`. Error: {}", remote_ip, idx, json, err);
            } else {
                let (remote_name, remote_idx, mut packet): JsonFormat<T> = remote_packet.unwrap();
                debug!("message-received, {}, {}, {}, {}, {}, {}",
                       name, idx, remote_name, remote_idx, remote_ip, json.len());
                packet.mark_received();
                let f = &mut *(callback.write().unwrap());
                let receipt = packet.get_receipt();
                f(packet);
                if receipt.is_some() {
                    outbound_send.send((Some(remote_ip.to_string()), receipt.unwrap())).unwrap();
                }
            }
            idx += 1;
        } else {
            trace!("Received an empty message from {}, message ID {}", remote_ip, idx)
        }
    }
}
