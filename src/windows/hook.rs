#![allow(non_snake_case, non_upper_case_globals)]

use windows::{core::{BOOL, PCWSTR}, Win32::Foundation::{SetLastError, ERROR_FILE_NOT_FOUND}};

use crate::{core::Error, windows::interceptor};

use super::ffi;

static mut PathFileExistsW_orig: usize = 0;
type PathFileExistsWFn = extern "C" fn(filename: PCWSTR) -> BOOL;
unsafe extern "C" fn PathFileExistsW(filename: PCWSTR) -> BOOL {
    let filename_str = unsafe { filename.to_string().expect("valid utf-16 filename") };
    if filename_str.ends_with("\\umamusume.exe.local") {
        info!("Unhooking PathFileExistsW");
        _ = interceptor::unhook(ffi::PathFileExistsW as usize);

        SetLastError(ERROR_FILE_NOT_FOUND);
        return false.into();
    }

    let orig_fn: PathFileExistsWFn = unsafe { std::mem::transmute(PathFileExistsW_orig) };
    orig_fn(filename)
}

fn init_internal() -> Result<(), Error> {
    unsafe {
        info!("Hooking PathFileExistsW");
        PathFileExistsW_orig = interceptor::hook(ffi::PathFileExistsW as usize, PathFileExistsW as usize)?;
    }

    Ok(())
}

pub fn init() {
    init_internal().unwrap_or_else(|e| {
        error!("Init failed: {}", e);
    });
}