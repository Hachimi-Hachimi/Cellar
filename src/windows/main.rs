use std::os::raw::{c_ulong, c_void};

use windows::Win32::Foundation::{BOOL, HINSTANCE, TRUE};

use crate::windows::log_impl;

use super::hook;

const DLL_PROCESS_ATTACH: c_ulong = 1;
//const DLL_PROCESS_DETACH: c_ulong = 0;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DllMain(_dll_module: HINSTANCE, call_reason: c_ulong, _reserved: *mut c_void) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        log_impl::init(log::LevelFilter::Debug);
        info!("Cellar initializing");
        hook::init();
        info!("Attach completed");
    }
    TRUE
}