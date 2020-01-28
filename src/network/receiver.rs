use bufstream::BufStream;
use std::collections::HashSet;
use std::io::BufRead;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use serde::de::DeserializeOwned;

use std::thread::sleep;
use std::thread::spawn;


// Start all receiver routines
pub fn start_receiver<T: 'static + Send + DeserializeOwned>(
        name: String, port: u16, model_send: Sender<T>, remote_ip_recv: Receiver<SocketAddr>) {
    spawn(move|| {
        receivers_launcher(name, port, model_send, remote_ip_recv);
    });
}


// If a new neighbor occurs, launch receiver to receive data from it
fn receivers_launcher<T: 'static + Send + DeserializeOwned>(
        name: String, port: u16, model_send: Sender<T>, remote_ip_recv: Receiver<SocketAddr>) {
    info!("now entering receivers listener");
    let mut receivers = HashSet::new();
    while let Ok(mut remote_addr) = remote_ip_recv.recv() {
        remote_addr.set_port(port);
        if !receivers.contains(&remote_addr) {
            let name_clone = name.clone();
            let chan = model_send.clone();
            let addr = remote_addr.clone();
            receivers.insert(remote_addr.clone());
            spawn(move|| {
                let mut tcp_stream = None;
                while tcp_stream.is_none() {
                    tcp_stream = match TcpStream::connect(remote_addr) {
                        Ok(_tcp_stream) => Some(_tcp_stream),
                        Err(error) => {
                            info!("(retry in 2 secs) Error: {}.
                                  Failed to connect to remote address {}",
                                  error, remote_addr);
                            sleep(Duration::from_secs(2));
                            None
                        }
                    };
                }
                let stream = BufStream::new(tcp_stream.unwrap());
                receiver(name_clone, addr, stream, chan);
            });
        } else {
            info!("(Skipped) Receiver exists for {}", remote_addr);
        }
    }
}


// Core receiver routine
fn receiver<T: DeserializeOwned>(
        name: String, remote_ip: SocketAddr, mut stream: BufStream<TcpStream>, chan: Sender<T>) {
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
                let (remote_name, remote_idx, data): (String, u32, T) = remote_packet.unwrap();
                debug!("message-received, {}, {}, {}, {}, {}, {}",
                       name, idx, remote_name, remote_idx, remote_ip, json.len());
                let send_result = chan.send(data);
                if let Err(err) = send_result {
                    error!("Failed to send the received model from the network
                            to local channel. Error: {}", err);
                }
            }
            idx += 1;
        } else {
            trace!("Received an empty message from {}, message ID {}", remote_ip, idx)
        }
    }
}
