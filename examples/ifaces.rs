extern crate ifaces;

use std::net::SocketAddr;

fn main() {
    for addr in ifaces::local_ifaces().unwrap() {
        match addr {
            SocketAddr::V4(v4_addr) => println!("Found V4 Address {:?}", v4_addr),
            SocketAddr::V6(v6_addr) => println!("Found V6 Address {:?}%{:?}", v6_addr, v6_addr.scope_id())
        }
    }
}
