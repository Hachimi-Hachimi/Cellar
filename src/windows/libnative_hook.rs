#![allow(non_snake_case, non_upper_case_globals)]

use std::os::raw::c_void;

use windows::Win32::Foundation::HMODULE;

use crate::{core::Error, windows::{interceptor, utils}};

extern "C" fn InitParameters(_param_1: i32, _param_2: *mut c_void) -> i32 { 1 }

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