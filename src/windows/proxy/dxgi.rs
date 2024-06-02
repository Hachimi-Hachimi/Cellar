#![allow(non_snake_case, non_upper_case_globals)]

use std::ffi::c_uint;

use widestring::{U16CString, Utf16Str};
use windows::{core::{HRESULT, PCWSTR}, Win32::{Foundation::E_NOTIMPL, System::LibraryLoader::LoadLibraryW}};

use crate::windows::utils;

proxy_proc!(CreateDXGIFactory, CreateDXGIFactory_orig);
proxy_proc!(CreateDXGIFactory1, CreateDXGIFactory1_orig);
proxy_proc!(CreateDXGIFactory2, CreateDXGIFactory2_orig);
proxy_proc!(DXGIGetDebugInterface1, DXGIGetDebugInterface1_orig);

static mut DXGID3D10CreateDevice_orig: usize = 0;
// These are called internally by the Direct3D driver on some versions of Windows (even when using d3d11)
// Bogus but compatible fn typedef, dont mind it
type DXGID3D10CreateDeviceFn = extern "C" fn(a: usize, b: usize, c: usize, d: c_uint, e: usize, f: c_uint, g: usize) -> HRESULT;
#[no_mangle]
pub extern "C" fn DXGID3D10CreateDevice(a: usize, b: usize, c: usize, d: c_uint, e: usize, f: c_uint, g: usize) -> HRESULT {
    let trampoline_addr = unsafe { DXGID3D10CreateDevice_orig };

    if trampoline_addr == 0 {
        return E_NOTIMPL;
    }

    unsafe { std::mem::transmute::<usize, DXGID3D10CreateDeviceFn>(trampoline_addr)(a, b, c, d, e, f, g) }
}

static mut DXGID3D10RegisterLayers_orig: usize = 0;
type DXGID3D10RegisterLayersFn = extern "C" fn(a: usize, b: c_uint) -> HRESULT;
#[no_mangle]
pub extern "C" fn DXGID3D10RegisterLayers(a: usize, b: c_uint) -> HRESULT {
    let trampoline_addr = unsafe { DXGID3D10RegisterLayers_orig };

    if trampoline_addr == 0 {
        return E_NOTIMPL;
    }

    unsafe { std::mem::transmute::<usize, DXGID3D10RegisterLayersFn>(trampoline_addr)(a, b) }
}

pub fn init(system_dir: &Utf16Str) {
    unsafe {
        let dll_path = system_dir.to_owned() + "\\dxgi.dll";
        let dll_path_cstr = U16CString::from_vec(dll_path.into_vec()).unwrap();
        let handle = LoadLibraryW(PCWSTR(dll_path_cstr.as_ptr())).expect("dxgi.dll");

        CreateDXGIFactory_orig = utils::get_proc_address(handle, cstr!("CreateDXGIFactory"));
        CreateDXGIFactory1_orig = utils::get_proc_address(handle, cstr!("CreateDXGIFactory1"));
        CreateDXGIFactory2_orig = utils::get_proc_address(handle, cstr!("CreateDXGIFactory2"));
        DXGID3D10CreateDevice_orig = utils::get_proc_address(handle, cstr!("DXGID3D10CreateDevice"));
        DXGID3D10RegisterLayers_orig = utils::get_proc_address(handle, cstr!("DXGID3D10RegisterLayers"));
        DXGIGetDebugInterface1_orig = utils::get_proc_address(handle, cstr!("DXGIGetDebugInterface1"));
    }
}