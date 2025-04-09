#[macro_use] extern crate log;

pub mod core;

/** Windows **/
#[cfg(target_os = "windows")]
mod windows;