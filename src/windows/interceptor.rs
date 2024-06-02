use std::os::raw::c_void;

use crate::core::Error;

use minhook::MinHook;

pub unsafe fn hook(orig_addr: usize, hook_addr: usize) -> Result<usize, Error> {
    let trampoline_addr = MinHook::create_hook(orig_addr as *mut c_void, hook_addr as *mut c_void)? as usize;
    MinHook::enable_hook(orig_addr as *mut c_void)?;
    Ok(trampoline_addr)
}

impl From<minhook::MH_STATUS> for Error {
    fn from(e: minhook::MH_STATUS) -> Self {
        Error::HookingError(format!("MinHook returned status: {:?}", e))
    }
}

pub unsafe fn unhook(orig_addr: usize) -> Result<(), Error> {
    MinHook::disable_hook(orig_addr as *mut c_void)?;
    MinHook::remove_hook(orig_addr as *mut c_void)?;
    Ok(())
}