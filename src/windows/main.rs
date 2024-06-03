use std::{fs, os::raw::{c_ulong, c_void}};

use widestring::U16CString;
use windows::{core::PCWSTR, Win32::{Foundation::{BOOL, HINSTANCE, TRUE}, System::LibraryLoader::LoadLibraryW}};

use crate::windows::log_impl;

use super::{hook, utils};

const DLL_PROCESS_ATTACH: c_ulong = 1;
//const DLL_PROCESS_DETACH: c_ulong = 0;

pub fn load_libraries() {
    let Some(mut path) = utils::get_game_dir() else {
        return;
    };
    path.push("cellar.txt");

    let Ok(s) = fs::read_to_string(path) else {
        warn!("Cannot read cellar.txt");
        return;
    };

    for name in s.lines() {
        if name.is_empty() { continue; }
        let Ok(name_cstr) = U16CString::from_str(name) else {
            warn!("Invalid library name: {}", name);
            continue;
        };
        let res = unsafe { LoadLibraryW(PCWSTR(name_cstr.as_ptr())) };

        if let Ok(handle) = res {
            if !handle.is_invalid() {
                info!("Loaded library: {}", name);
                continue;
            }
        }

        warn!("Failed to load library: {}", name);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DllMain(_dll_module: HINSTANCE, call_reason: c_ulong, _reserved: *mut c_void) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        log_impl::init(log::LevelFilter::Debug);
        info!("Cellar initializing");
        load_libraries();
        hook::init();
        info!("Attach completed");
    }
    TRUE
}