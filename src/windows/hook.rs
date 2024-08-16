#![allow(non_snake_case, non_upper_case_globals)]

use std::{os::raw::c_void, sync::Mutex};

use fnv::FnvHashSet;
use once_cell::sync::Lazy;
use widestring::{utf16str, Utf16Str, Utf16String};
use windows::{
    core::{wcslen, PCWSTR},
    Win32::{
        Foundation::{BOOL, HANDLE, HMODULE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS, FIND_FIRST_EX_FLAGS, WIN32_FIND_DATAW}
    }
};

use crate::{core::Error, windows::interceptor};

use super::{ffi, libnative_hook, proxy, utils};

static mut LoadLibraryW_orig: usize = 0;
type LoadLibraryWFn = extern "C" fn(filename: PCWSTR) -> HMODULE;
extern "C" fn LoadLibraryW(filename: PCWSTR) -> HMODULE {
    let orig_fn: LoadLibraryWFn = unsafe { std::mem::transmute(LoadLibraryW_orig) };

    let handle = orig_fn(filename);
    let filename_str = unsafe { filename.to_string().expect("valid utf-16 filename") };

    if filename_str.ends_with("libnative.dll") {
        info!("Got libnative.dll");
        libnative_hook::init(handle);
    }
    handle
}

static ALLOWED_FILES: Lazy<FnvHashSet<Utf16String>> = Lazy::new(|| 
    [
        utf16str!(".").to_owned(),
        utf16str!("..").to_owned(),
        utf16str!("baselib.dll").to_owned(),
        utf16str!("GameAssembly.dll").to_owned(),
        utf16str!("umamusume_Data").to_owned(),
        utf16str!("umamusume.exe").to_owned(),
        utf16str!("umamusume.exe._").to_owned(),
        utf16str!("UmamusumeUninstaller.exe").to_owned(),
        utf16str!("UnityCrashHandler64.exe").to_owned(),
        utf16str!("UnityPlayer.dll").to_owned()
    ]
    .into_iter().collect()
);

static FIND_FILE_HANDLES: Lazy<Mutex<FnvHashSet<usize>>> = Lazy::new(|| Mutex::default());

static mut FindFirstFileExW_orig: usize = 0;
type FindFirstFileExW = extern "C" fn(
    PCWSTR, FINDEX_INFO_LEVELS, *mut WIN32_FIND_DATAW,
    FINDEX_SEARCH_OPS, *const c_void, FIND_FIRST_EX_FLAGS
) -> HANDLE;
extern "C" fn FindFirstFileExW(
    filename: PCWSTR,
    info_level_id: FINDEX_INFO_LEVELS,
    ffd_: *mut WIN32_FIND_DATAW,
    search_op: FINDEX_SEARCH_OPS,
    search_filter: *const c_void,
    additional_flags: FIND_FIRST_EX_FLAGS
) -> HANDLE {
    let orig_fn: FindFirstFileExW = unsafe { std::mem::transmute(FindFirstFileExW_orig) };
    let handle = orig_fn(filename, info_level_id, ffd_, search_op, search_filter, additional_flags);
    if handle == INVALID_HANDLE_VALUE {
        return handle;
    }

    let Some(mut game_dir_pattern) = utils::get_game_dir_str() else {
        return handle;
    };
    game_dir_pattern += "\\*.*";
    let current_pattern = unsafe { filename.to_string().unwrap() };
    if current_pattern.to_ascii_lowercase() != game_dir_pattern.to_ascii_lowercase() {
        return handle;
    }

    let next_fn: FindNextFileWFn = unsafe { std::mem::transmute(FindNextFileW_orig) };

    loop {
        let Some(filename_str) = get_ffd_filename_str(ffd_) else {
            return handle;
        };
        if ALLOWED_FILES.contains(filename_str) {
            break;
        }

        if !next_fn(handle, ffd_).as_bool() {
            return INVALID_HANDLE_VALUE;
        }
    }

    FIND_FILE_HANDLES.lock().unwrap().insert(handle.0 as usize);
    handle
}

static mut FindNextFileW_orig: usize = 0;
type FindNextFileWFn = extern "C" fn(handle: HANDLE, ffd: *mut WIN32_FIND_DATAW) -> BOOL;
extern "C" fn FindNextFileW(handle: HANDLE, ffd_: *mut WIN32_FIND_DATAW) -> BOOL {
    let orig_fn: FindNextFileWFn = unsafe { std::mem::transmute(FindNextFileW_orig) };

    let mut res = orig_fn(handle, ffd_);
    let mut handles = FIND_FILE_HANDLES.lock().unwrap();
    if !handles.contains(&(handle.0 as usize)) {
        return res;
    }

    loop {
        if !res.as_bool() {
            handles.remove(&(handle.0 as usize));
            return res;
        }

        let Some(filename_str) = get_ffd_filename_str(ffd_) else {
            return res;
        };
        if ALLOWED_FILES.contains(filename_str) {
            break;
        }

        res = orig_fn(handle, ffd_);
    }

    res
}

fn get_ffd_filename_str(ffd_: *mut WIN32_FIND_DATAW) -> Option<&'static Utf16Str> {
    let ffd = unsafe { ffd_.as_ref() }?;

    unsafe {
        Some(Utf16Str::from_slice_unchecked(
            &ffd.cFileName[..wcslen(PCWSTR::from_raw(ffd.cFileName.as_ptr()))]
        ))
    }
}


fn init_internal() -> Result<(), Error> {
    let system_dir = utils::get_system_directory();

    info!("Init dxgi.dll proxy");
    proxy::dxgi::init(&system_dir);

    info!("Init version.dll proxy");
    proxy::version::init(&system_dir);

    info!("Init winhttp.dll proxy");
    proxy::winhttp::init(&system_dir);

    unsafe {
        info!("Hooking LoadLibraryW");
        LoadLibraryW_orig = interceptor::hook(ffi::LoadLibraryW as usize, LoadLibraryW as usize)?;    

        info!("Hooking fun stuff");
        FindFirstFileExW_orig = interceptor::hook(ffi::FindFirstFileExW as usize, FindFirstFileExW as usize)?;
        FindNextFileW_orig = interceptor::hook(ffi::FindNextFileW as usize, FindNextFileW as usize)?;
    }

    Ok(())
}

pub fn init() {
    init_internal().unwrap_or_else(|e| {
        error!("Init failed: {}", e);
    });
}