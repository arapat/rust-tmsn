use bufstream::BufStream;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::spawn;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use packet::JsonFormat;
use packet::Packet;


type Stream = Vec<(String, BufStream<TcpStream>)>;
type LockedStream = Arc<RwLock<Stream>>;


// Start all sender routines - start local sender and also accept remote senders
pub fn start_sender<T: 'static + Send + Serialize + DeserializeOwned>(
    name: String, port: u16, model_recv: Receiver<(Option<String>, Packet<T>)>,
    remote_ip_send: Option<Sender<SocketAddr>>,
) -> Result<(), &'static str> where T: Send + Serialize {
    // Vec<BufStream<TcpStream>>
    let streams = Arc::new(RwLock::new(vec![]));
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
    let (name_lstn, streams_lstn) = (name.clone(), streams.clone());
    spawn(move|| {
        income_conn_listener(name_lstn, streams_lstn, remote_ip_send, listener);
    });

    // Repeatedly sending local data out to the remote connections
    spawn(move|| {
        sender(name, streams, model_recv);
    });
    Ok(())
}


// Sender listener (i.e. the listener of the sender) is responsible for:
//     1. Add new incoming stream to sender (via streams RwLock)
//     2. Send new incoming address to receiver so that it connects to the new machine
fn income_conn_listener(
        name: String,
        sender_streams: LockedStream,
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
                let local_addr = stream.local_addr().expect(
                    "Cannot unwrap the local address from the incoming stream.");
                info!("Sender received a connection, {}, ->, {}", remote_addr, local_addr);
                // append the new stream to sender
                let mut lock_w = sender_streams.write().expect(
                    "Failed to obtain the lock for expanding sender_streams."
                );
                lock_w.push((remote_addr.to_string(), BufStream::new(stream)));
                drop(lock_w);
                info!("Remote server {} will receive our model from now on.", remote_addr);
                // subscribe to the remote machine
                if let Some(ref receivers) = receiver_ips {
                    receivers.send(remote_addr.clone()).expect(
                        "Cannot send the received IP to the channel."
                    );
                    info!("Remote server {} will be subscribed soon.", remote_addr);
                }
            }
        }
    }
}


// Core sender routine - 1 to many
fn sender<'a, T>(name: String, streams: LockedStream, chan: Receiver<(Option<String>, Packet<T>)>)
where T: 'static + Send + Serialize + DeserializeOwned {
    info!("1-to-many Sender has started.");

    let mut idx = 0;
    loop {
        let data = chan.recv();
        if let Err(err) = data {
            error!("Network module cannot receive the local model. Error: {}", err);
            continue;
        }
        debug!("network-to-send-out, {}, {}", name, idx);

        let (remote_ip, data) = data.unwrap();
        let packet_load: JsonFormat<T> = (name.clone(), idx, data);
        let safe_json = serde_json::to_string(&packet_load);
        if let Err(err) = safe_json {
            error!("Local model cannot be serialized. Error: {}", err);
            continue;
        }
        let json = safe_json.unwrap();
        let num_computers = {
            let streams = streams.write();
            if let Err(err) = streams {
                error!("Failed to obtain the lock for writing to sender_streams. Error: {}", err);
                0
            } else {
                let mut streams = streams.unwrap();
                let mut sent_out = 0;
                streams.iter_mut().for_each(|(addr, stream)| {
                    if remote_ip.is_some() && remote_ip.as_ref().unwrap() != addr {
                        return;
                    }
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
