use std::ptr;
use std::io;
use std::net::{SocketAddr, SocketAddrV6, Ipv4Addr, SocketAddrV4};
use std::mem;

use sys::windows::ffi::{IpAdapterAddresses, IpAdapterUnicastAddress};

use winapi::winerror::{ERROR_SUCCESS, ERROR_ADDRESS_NOT_ASSOCIATED, ERROR_BUFFER_OVERFLOW,
                       ERROR_INVALID_PARAMETER, ERROR_NOT_ENOUGH_MEMORY, ERROR_NO_DATA};
use winapi::ws2def::{AF_UNSPEC, SOCKADDR_IN, AF_INET, AF_INET6};
use winapi::ws2ipdef::sockaddr_in6;

mod ffi;

const PREALLOC_ADAPTERS_LEN: usize = 15 * 1024;

pub fn local_ifaces() -> io::Result<Vec<SocketAddr>> {
    let mut adapters_list = Vec::with_capacity(PREALLOC_ADAPTERS_LEN);

    unsafe {
        try!(local_ifaces_with_buffer(&mut adapters_list));

        Ok(map_adapter_addresses(mem::transmute(adapters_list.as_ptr())))
    }
}

unsafe fn map_adapter_addresses(mut adapter_addr: *const IpAdapterAddresses) -> Vec<SocketAddr> {
    let mut adapter_addresses = Vec::new();

    while adapter_addr != ptr::null() {
        let curr_adapter_addr = &*adapter_addr;

        let mut unicast_addr = curr_adapter_addr.all.first_unicast_address;
        while unicast_addr != ptr::null() {
            let curr_unicast_addr = &*unicast_addr;

            if is_ipv4_enabled(&curr_unicast_addr) {
                adapter_addresses.push(SocketAddr::V4(v4_socket_from_adapter(&curr_unicast_addr)));
            } else if is_ipv6_enabled(&curr_unicast_addr) {
                let mut v6_sock = v6_socket_from_adapter(&curr_unicast_addr);
                // Make sure the scope id is set for ALL interfaces, not just link-local
                v6_sock.set_scope_id(curr_adapter_addr.xp.ipv6_if_index);

                adapter_addresses.push(SocketAddr::V6(v6_sock));
            }

            unicast_addr = curr_unicast_addr.next;
        }

        adapter_addr = curr_adapter_addr.all.next;
    }

    adapter_addresses
}

unsafe fn v4_socket_from_adapter(unicast_addr: &IpAdapterUnicastAddress) -> SocketAddrV4 {
    let socket_addr = &unicast_addr.address;

    let in_addr: SOCKADDR_IN = mem::transmute((*socket_addr.lpSockaddr));
    let sin_addr = in_addr.sin_addr.S_un;

    let v4_addr = Ipv4Addr::new((sin_addr >> 0) as u8,
                                (sin_addr >> 8) as u8,
                                (sin_addr >> 16) as u8,
                                (sin_addr >> 24) as u8);

    SocketAddrV4::new(v4_addr, 0)
}

unsafe fn is_ipv4_enabled(unicast_addr: &IpAdapterUnicastAddress) -> bool {
    if unicast_addr.length != 0 {
        let socket_addr = &unicast_addr.address;
        let sa_family = (*socket_addr.lpSockaddr).sa_family;

        sa_family == AF_INET as u16
    } else {
        false
    }
}

unsafe fn v6_socket_from_adapter(unicast_addr: &IpAdapterUnicastAddress) -> SocketAddrV6 {
    let socket_addr = &unicast_addr.address;

    let sock_addr6: *const sockaddr_in6 = mem::transmute(socket_addr.lpSockaddr);
    let in6_addr: sockaddr_in6 = *sock_addr6;

    let v6_addr = in6_addr.sin6_addr.s6_addr.into();

    SocketAddrV6::new(v6_addr, 0, in6_addr.sin6_flowinfo, in6_addr.sin6_scope_id)
}

unsafe fn is_ipv6_enabled(unicast_addr: &IpAdapterUnicastAddress) -> bool {
    if unicast_addr.length != 0 {
        let socket_addr = &unicast_addr.address;
        let sa_family = (*socket_addr.lpSockaddr).sa_family;

        sa_family == AF_INET6 as u16
    } else {
        false
    }
}

unsafe fn local_ifaces_with_buffer(buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut length = buffer.capacity() as u32;

    let ret_code = ffi::GetAdaptersAddresses(AF_UNSPEC as u32,
                                             0,
                                             ptr::null_mut(),
                                             buffer.as_mut_ptr(),
                                             &mut length);
    match ret_code {
        ERROR_SUCCESS => Ok(()),
        ERROR_ADDRESS_NOT_ASSOCIATED => Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "An address has not yet been associated with the network endpoint.")),
        ERROR_BUFFER_OVERFLOW => {
            buffer.reserve_exact(length as usize);

            local_ifaces_with_buffer(buffer)
        }
        ERROR_INVALID_PARAMETER => Err(io::Error::new(io::ErrorKind::InvalidInput, "One of the parameters is invalid.")),
        ERROR_NOT_ENOUGH_MEMORY => Err(io::Error::new(io::ErrorKind::Other, "Insufficient memory resources are available to complete the operation.")),
        ERROR_NO_DATA => Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "No addresses were found for the requested parameters.")),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Some Other Error Occured.")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let addrs = super::local_ifaces().unwrap();

        for addr in addrs {
            println!("{:?}", addr);
        }
    }
}
