extern crate pyo3;
extern crate tmsn;

use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::Exception;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use tmsn::network;


create_exception!(tmsn, AddrInUse, Exception);


#[pyclass]
pub struct TmsnNetwork {
    remote_recv:  Option<Receiver<Vec<u8>>>,
    local_sender: Option<Sender<Vec<u8>>>,
}


#[pymethods]
impl TmsnNetwork {
    /// send out a packet
    /// Example: network.send(packet)
    pub fn send(&mut self, packet: &[u8]) -> PyResult<()> {
        let ret = self.local_sender.as_mut().unwrap().send(packet.to_vec());
        Ok(ret.unwrap())
    }

    /// receive a packet
    /// Example: packet = network.recv()
    /// If there is no new packet, the method returns an empty list (i.e., [])
    pub fn recv(&mut self) -> PyResult<Vec<u8>> {
        let ret = self.remote_recv.as_mut().unwrap().try_recv();
        // TODO: handle exception?
        if ret.is_ok() {
            Ok(ret.unwrap())
        } else {
            Ok(vec![])
        }
    }
}


/// Starts a broadcast network using a subscription list.
///
/// Example: start_network("machine_name", ["remote_ip_1", "remote_ip_2"], 8080)
///
/// The network recieves as input a sender and a receiver of two channels, respectively,
/// one for incoming packets and the other for outgoing packets.
///
/// Each machine maintains a list of subscriptions. The list defines
/// the IPs that this machine is listening to.
/// Initially, this list is provided as the parameter `init_remote_ips`
/// of the function `start_network`.
///
/// The network structure between the machines are decided by your program, specifically by
/// explicitly setting the list of IPs to be subscribed from each machine.
///
/// ## Parameters
/// * `name` - the local computer name.
/// * `init_remote_ips` - a list of IPs to which this computer makes a connection initially.
/// * `port` - the port number that the machines in the network are listening to.
/// `port` has to be the same value for all machines.
///
/// Design
///
/// Initially, the local computer only connects to the computers specificed by the
/// `init_remote_ips` vector in the function parameters (neighbors), and *receive* data from
/// these computers.
/// Specifically, a **Receiver** is created for each neighbor. The connection is initiated by the
/// Receiver. The number of Receivers on a computer is always equal to the number of neighbors.
/// On the other end, only one **Sender** is created for a computer, which send data to all other
/// computers that connected to it.
///
#[pyfunction]
pub fn start_network(
    name: String, init_remote_ips: Vec<String>, port: u16,
) -> PyResult<TmsnNetwork> {
    let (remote_s, remote_r) = mpsc::channel();
    let (local_s, local_r) = mpsc::channel();
    let is_network_on = network::start_network(
        name.as_str(), &init_remote_ips, port, false, remote_s, local_r);
    if is_network_on.is_err() {
        return Err(AddrInUse::py_err(is_network_on.err().unwrap()));
    }
    let tmsn = TmsnNetwork {
        remote_recv: Some(remote_r),
        local_sender: Some(local_s),
    };
    Ok(tmsn)
}


/// Start the network in the send-out-data only mode.
///
/// Example: start_network("machine_name", 8080)
#[pyfunction]
pub fn start_network_only_send(name: String, port: u16) -> PyResult<TmsnNetwork> {
    let (local_s, local_r) = mpsc::channel();
    let is_network_on = network::start_network_only_send(name.as_str(), port, local_r);
    if is_network_on.is_err() {
        return Err(AddrInUse::py_err(is_network_on.err().unwrap()));
    }
    let tmsn = TmsnNetwork {
        remote_recv: None,
        local_sender: Some(local_s),
    };
    Ok(tmsn)
}


/// Start the network in the receive-in-data only mode.
/// The addresses of the remote machines to be listened to needs to be provided in
/// the list of parameters.
///
/// Example: start_network("machine_name", ["remote_ip_1", "remote_ip_2"], 8080)
#[pyfunction]
pub fn start_network_only_recv(
    name: String, remote_ips: Vec<String>, port: u16,
) -> PyResult<TmsnNetwork> {
    let (remote_s, remote_r) = mpsc::channel();
    let is_network_on =
        network::start_network_only_recv(name.as_str(), &remote_ips, port, remote_s);
    is_network_on.unwrap();
    let tmsn = TmsnNetwork {
        remote_recv: Some(remote_r),
        local_sender: None,
    };
    Ok(tmsn)
}


/// Create the network connection in a cluster.
/// Please see the description of the methods for more details.
///
/// Methods:
///     - start_network
///     - start_network_only_send
///     - start_network_only_recv
#[pymodule]
fn tmsn(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(start_network))?;
    m.add_wrapped(wrap_pyfunction!(start_network_only_recv))?;
    m.add_wrapped(wrap_pyfunction!(start_network_only_send))?;
    m.add("AddrInUse", py.get_type::<AddrInUse>())?;

    Ok(())
}
