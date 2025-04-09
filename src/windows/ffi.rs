use windows::core::{BOOL, PCWSTR};

#[link(name = "shlwapi")]
extern "C" {
    pub fn PathFileExistsW(lpfilename: PCWSTR) -> BOOL;
}