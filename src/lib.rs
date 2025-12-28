pub mod backend;
pub mod message;
pub mod device;
pub mod mapping;
mod virtual_input;

#[cfg(target_os = "linux")]
mod input;

#[cfg(not(target_os = "linux"))]
mod input;