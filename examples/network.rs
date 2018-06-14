extern crate rust_tmsn;

use std::sync::mpsc;
use std::thread::sleep;
use rust_tmsn::network::start_network;

use std::time::Duration;


fn main() {
    // Remote data queue, where the data received from network would be put in
    let (remote_data_send, remote_data_recv) = mpsc::channel();
    // Local data queue, where the data generated locally would be put in
    let (local_data_send, local_data_recv) = mpsc::channel();

    let network = vec![String::from("127.0.0.1")];
    start_network("local", &network, 8000, false, remote_data_send, local_data_recv);

    // Put a test message in the local_data 
    let message = String::from("Hello, this is a test message.");
    sleep(Duration::from_millis(100));  // add waiting in case network is not ready
    local_data_send.send(message.clone()).unwrap();
    println!("Sent out this message: {}", message);

    // The message above is supposed to send out to all the neighbors computers specified
    // in the `network` vector, which contains only the localhost.
    // The network module running on the local host should have received the message
    // and put it into the remote data queue.
    let data_received = remote_data_recv.recv().unwrap();
    assert_eq!(data_received, message);
    println!("Received this message: {}", data_received);
}