use bufstream::BufStream;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use serde::ser::Serialize;

use std::thread::spawn;


type StreamLockVec = Arc<RwLock<Vec<BufStream<TcpStream>>>>;


// Start all sender routines - start local sender and also accept remote senders
pub fn start_sender<T: 'static + Send + Serialize>(
        name: String, port: u16, model_recv: Receiver<T>,
        remote_ip_send: Option<Sender<SocketAddr>>) -> Result<(), &'static str> {
    let streams: Vec<BufStream<TcpStream>> = vec![];
    let streams_arc = Arc::new(RwLock::new(streams));

    let arc_w = streams_arc.clone();
    let name_clone = name.clone();
    // accepts remote connections
    let listener = {
        let local_addr: SocketAddr =
            (String::from("0.0.0.0:") + port.to_string().as_str()).parse().expect(
                &format!("Cannot parse the port number `{}`.", port)
            );
        let listener = TcpListener::bind(local_addr);
        if listener.is_err() {
            return Err("Failed to bind the listening port");
        }
        listener.unwrap()
    };
    spawn(move|| {
        sender_listener(name_clone, arc_w, remote_ip_send, listener);
    });

    // Repeatedly sending local data out to the remote connections
    spawn(move|| {
        sender(name, streams_arc, model_recv);
    });
    Ok(())
}


// Sender listener (i.e. the listener of the sender) is responsible for:
//     1. Add new incoming stream to sender (via streams RwLock)
//     2. Send new incoming address to receiver so that it connects to the new machine
fn sender_listener(
        name: String,
        sender_streams: StreamLockVec,
        receiver_ips: Option<Sender<SocketAddr>>,
        listener: TcpListener) {
    info!("{} entering sender listener", name);
    for stream in listener.incoming() {
        match stream {
            Err(_) => error!("Sender received an error connection."),
            Ok(stream) => {
                let remote_addr = stream.peer_addr().expect(
                    "Cannot unwrap the remote address from the incoming stream."
                );
                info!("Sender received a connection, {}, ->, {}",
                      remote_addr, stream.local_addr().expect(
                          "Cannot unwrap the local address from the incoming stream."
                      ));
                {
                    let mut lock_w = sender_streams.write().expect(
                        "Failed to obtain the lock for expanding sender_streams."
                    );
                    lock_w.push(BufStream::new(stream));
                }
                info!("Remote server {} will receive our model from now on.", remote_addr);
                if let Some(ref receivers) = receiver_ips {
                    receivers.send(remote_addr.clone()).expect(
                        "Cannot send the received IP to the channel."
                    );
                }
                info!("Remote server {} will be subscribed soon (if not already).", remote_addr);
            }
        }
    }
}


// Core sender routine - 1 to many
fn sender<T: Serialize>(name: String, streams: StreamLockVec, chan: Receiver<T>) {
    info!("1-to-many Sender has started.");

    let mut idx = 0;
    loop {
        let data = chan.recv();
        if let Err(err) = data {
            error!("Network module cannot receive the local model. Error: {}", err);
            continue;
        }
        debug!("network-to-send-out, {}, {}", name, idx);

        let packet_load: (String, u32, T) = (name.clone(), idx, data.unwrap());
        let safe_json = serde_json::to_string(&packet_load);
        if let Err(err) = safe_json {
            error!("Local model cannot be serialized. Error: {}", err);
            continue;
        }
        let json = safe_json.unwrap();
        let num_computers = {
            let safe_lock_r = streams.write();
            if let Err(err) = safe_lock_r {
                error!("Failed to obtain the lock for writing to sender_streams. Error: {}", err);
                0
            } else {
                let mut lock_r = safe_lock_r.unwrap();
                let mut sent_out = 0;
                lock_r.iter_mut().for_each(|stream| {
                    if let Err(err) = stream.write_fmt(format_args!("{}\n", json)) {
                        error!("Cannot write into one of the streams. Error: {}", err);
                    } else {
                        if let Err(err) = stream.flush() {
                            error!("Cannot flush one of the streams. Error: {}", err);
                        } else {
                            sent_out += 1;
                        }
                    }
                });
                sent_out
            }
        };
        debug!("network-sent-out, {}, {}, {}", name, idx, num_computers);
        idx += 1;
    }
}
