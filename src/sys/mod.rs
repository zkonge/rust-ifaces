#[cfg(not(windows))]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(not(windows))]
pub use unix::local_ifaces;

#[cfg(windows)]
pub use windows::local_ifaces;
