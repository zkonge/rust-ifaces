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
pub use sys::unix;
#[cfg(windows)]
pub use sys::windows;

pub use sys::local_ifaces;
