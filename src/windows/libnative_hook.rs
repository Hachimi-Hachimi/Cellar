#![allow(non_snake_case, non_upper_case_globals)]

use windows::Win32::Foundation::HMODULE;

use crate::{core::Error, windows::{interceptor, utils}};

fn InitParameters() -> i64 { 1 }

fn init_internal(handle: HMODULE) -> Result<(), Error> {
    let InitParameters_addr = utils::get_proc_address(handle, cstr!("InitParameters"));
    if InitParameters_addr != 0 {
        unsafe {
            info!("Hooking even more fun stuff");
            interceptor::hook(InitParameters_addr, InitParameters as usize)?;
        }
    }

    Ok(())
}

pub fn init(handle: HMODULE) {
    init_internal(handle).unwrap_or_else(|e| {
        error!("Init failed: {}", e);
    });
}