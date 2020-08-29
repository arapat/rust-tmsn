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

use std::thread::sleep;
use std::thread::spawn;

use packet::JsonFormat;
use packet::Packet;


// Start all receiver routines
pub fn start_receiver(
        port: u16,
        outbound_send: Sender<(Option<String>, Packet)>,
        callback: Box<dyn FnMut(String, Packet) + Sync + Send>,
        remote_ip_recv: Receiver<SocketAddr>) {
    spawn(move|| {
        // If a new neighbor occurs, launch receiver to receive data from it
        info!("now entering receivers listener");
        let mut receivers = HashSet::new();
        let f = Arc::new(RwLock::new(callback));
        while let Ok(mut remote_addr) = remote_ip_recv.recv() {
            remote_addr.set_port(port);
            if !receivers.contains(&remote_addr) {
                let callback = f.clone();
                let addr = remote_addr.clone();
                let outbound = outbound_send.clone();
                receivers.insert(remote_addr.clone());
                spawn(move || {
                    let mut tcp_stream = None;
                    let mut attempt = 0;
                    while tcp_stream.is_none() && attempt < 3 {
                        attempt += 1;
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
                    if tcp_stream.is_some() {
                        let stream = BufStream::new(tcp_stream.unwrap());
                        receiver(addr, stream, outbound, callback);
                    } else {
                        info!("Failed to connect to remote address {}. Quit.", remote_addr);
                    }
                });
            } else {
                info!("(Skipped) Receiver exists for {}", remote_addr);
            }
        }
    });
}


// Core receiver routine
pub fn receiver(
    remote_ip: SocketAddr, mut stream: BufStream<TcpStream>,
    outbound_send: Sender<(Option<String>, Packet)>,
    callback: Arc<RwLock<Box<dyn FnMut(String, Packet) + Sync + Send>>>,
) {
    let remote_ip_str = remote_ip.ip().to_string();
    info!("Receiver started, {}, {}", remote_ip, remote_ip_str);
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
                let sender_name = remote_ip_str.clone();
                let (remote_idx, mut packet): JsonFormat = remote_packet.unwrap();
                trace!("message-received, {}, {}, {}, {}, {}",
                       idx, sender_name, remote_idx, remote_ip, json.len());
                packet.mark_received();
                let f = &mut *(callback.write().unwrap());
                let receipt = packet.get_receipt();
                f(sender_name.clone(), packet);
                if receipt.is_some() {
                    outbound_send.send((Some(sender_name), receipt.unwrap())).unwrap();
                }
            }
            idx += 1;
        } else {
            trace!("Received an empty message from {}, message ID {}", remote_ip, idx)
        }
    }
}
