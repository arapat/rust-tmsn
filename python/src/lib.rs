extern crate pyo3;
extern crate tmsn;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use tmsn::network;


#[pyclass]
pub struct TmsnNetwork {
    remote_recv:  Option<Receiver<Vec<u8>>>,
    local_sender: Option<Sender<Vec<u8>>>,
}


#[pymethods]
impl TmsnNetwork {
    pub fn send(&mut self, packet: &[u8]) -> PyResult<()> {
        let ret = self.local_sender.as_mut().unwrap().send(packet.to_vec());
        Ok(ret.unwrap())
    }

    pub fn recv(&mut self) -> PyResult<Vec<u8>> {
        let ret = self.remote_recv.as_mut().unwrap().try_recv();
        // TODO: handle exception?
        Ok(ret.unwrap())
    }
}


#[pyfunction]
pub fn start_network(
    name: String, init_remote_ips: Vec<String>, port: u16, is_two_way: bool,
) -> PyResult<TmsnNetwork> {
    let (remote_s, remote_r) = mpsc::channel();
    let (local_s, local_r) = mpsc::channel();
    network::start_network(
        name.as_str(), &init_remote_ips, port, is_two_way, remote_s, local_r);
    let tmsn = TmsnNetwork {
        remote_recv: Some(remote_r),
        local_sender: Some(local_s),
    };
    Ok(tmsn)
}


#[pyfunction]
pub fn start_network_only_send(name: String, port: u16) -> PyResult<TmsnNetwork> {
    let (local_s, local_r) = mpsc::channel();
    network::start_network_only_send(name.as_str(), port, local_r);
    let tmsn = TmsnNetwork {
        remote_recv: None,
        local_sender: Some(local_s),
    };
    Ok(tmsn)
}


#[pyfunction]
pub fn start_network_only_recv(
    name: String, remote_ips: Vec<String>, port: u16,
) -> PyResult<TmsnNetwork> {
    let (remote_s, remote_r) = mpsc::channel();
    network::start_network_only_recv(name.as_str(), &remote_ips, port, remote_s);
    let tmsn = TmsnNetwork {
        remote_recv: Some(remote_r),
        local_sender: None,
    };
    Ok(tmsn)
}


#[pymodule]
fn tmsn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(start_network))?;
    m.add_wrapped(wrap_pyfunction!(start_network_only_recv))?;
    m.add_wrapped(wrap_pyfunction!(start_network_only_send))?;

    Ok(())
}
