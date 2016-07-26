#[cfg(not(windows))]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
pub use self::unix::{
    Interface,
    Kind,
    NextHop,
    local_ifaces
};

#[cfg(windows)]
pub use self::windows::{
    local_ifaces
};