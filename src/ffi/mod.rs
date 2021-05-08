#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
pub use windows::ifaces;

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use unix::ifaces;
