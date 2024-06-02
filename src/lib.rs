#[macro_use] extern crate log;
#[macro_use] extern crate cstr;

pub mod core;

/** Windows **/
#[cfg(target_os = "windows")]
mod windows;