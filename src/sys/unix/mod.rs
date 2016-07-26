use std::io;
use std::net::SocketAddr;

mod ffi;
mod interface;

pub use sys::unix::interface::{Interface, Kind, NextHop};

/// Query the local system for all interface addresses.
pub fn local_ifaces() -> io::Result<Vec<SocketAddr>> {
    let iface_iter = try!(Interface::get_all()).into_iter();
    
    Ok(iface_iter.filter(|iface| iface.kind != Kind::Packet)
        .filter_map(|iface| iface.addr)
        .collect())
}