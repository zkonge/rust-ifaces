extern crate libc;

#[cfg(not(windows))]
extern crate nix;

#[cfg(windows)]
#[macro_use]
extern crate bitflags;
#[cfg(windows)]
extern crate winapi;

mod sys;

#[cfg(not(windows))]
/// Unix specific network interface querying.
pub mod unix {
    pub use sys::{
        Interface,
        Kind,
        NextHop,
        local_ifaces
    };
}

#[cfg(windows)]
/// Windows specific network interface querying.
pub mod windows {
    pub use sys::{
        local_ifaces
    };
}

pub use sys::local_ifaces;