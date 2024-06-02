#![allow(non_snake_case, non_upper_case_globals)]

use widestring::{U16CString, Utf16Str};
use windows::{core::PCWSTR, Win32::System::LibraryLoader::LoadLibraryW};

use crate::windows::utils;

proxy_proc!(WinHttpAddRequestHeaders, WinHttpAddRequestHeaders_orig);
proxy_proc!(WinHttpCheckPlatform, WinHttpCheckPlatform_orig);
proxy_proc!(WinHttpCloseHandle, WinHttpCloseHandle_orig);
proxy_proc!(WinHttpConnect, WinHttpConnect_orig);
proxy_proc!(WinHttpCrackUrl, WinHttpCrackUrl_orig);
proxy_proc!(WinHttpCreateProxyResolver, WinHttpCreateProxyResolver_orig);
proxy_proc!(WinHttpCreateUrl, WinHttpCreateUrl_orig);
proxy_proc!(WinHttpDetectAutoProxyConfigUrl, WinHttpDetectAutoProxyConfigUrl_orig);
proxy_proc!(WinHttpFreeProxyResult, WinHttpFreeProxyResult_orig);
proxy_proc!(WinHttpFreeProxyResultEx, WinHttpFreeProxyResultEx_orig);
proxy_proc!(WinHttpFreeProxySettings, WinHttpFreeProxySettings_orig);
proxy_proc!(WinHttpGetDefaultProxyConfiguration, WinHttpGetDefaultProxyConfiguration_orig);
proxy_proc!(WinHttpGetIEProxyConfigForCurrentUser, WinHttpGetIEProxyConfigForCurrentUser_orig);
proxy_proc!(WinHttpGetProxyForUrl, WinHttpGetProxyForUrl_orig);
proxy_proc!(WinHttpGetProxyForUrlEx, WinHttpGetProxyForUrlEx_orig);
proxy_proc!(WinHttpGetProxyForUrlEx2, WinHttpGetProxyForUrlEx2_orig);
proxy_proc!(WinHttpGetProxyResult, WinHttpGetProxyResult_orig);
proxy_proc!(WinHttpGetProxyResultEx, WinHttpGetProxyResultEx_orig);
proxy_proc!(WinHttpGetProxySettingsVersion, WinHttpGetProxySettingsVersion_orig);
proxy_proc!(WinHttpOpen, WinHttpOpen_orig);
proxy_proc!(WinHttpOpenRequest, WinHttpOpenRequest_orig);
proxy_proc!(WinHttpQueryAuthSchemes, WinHttpQueryAuthSchemes_orig);
proxy_proc!(WinHttpQueryDataAvailable, WinHttpQueryDataAvailable_orig);
proxy_proc!(WinHttpQueryHeaders, WinHttpQueryHeaders_orig);
proxy_proc!(WinHttpQueryOption, WinHttpQueryOption_orig);
proxy_proc!(WinHttpReadData, WinHttpReadData_orig);
proxy_proc!(WinHttpReadProxySettings, WinHttpReadProxySettings_orig);
proxy_proc!(WinHttpReceiveResponse, WinHttpReceiveResponse_orig);
proxy_proc!(WinHttpResetAutoProxy, WinHttpResetAutoProxy_orig);
proxy_proc!(WinHttpSendRequest, WinHttpSendRequest_orig);
proxy_proc!(WinHttpSetCredentials, WinHttpSetCredentials_orig);
proxy_proc!(WinHttpSetDefaultProxyConfiguration, WinHttpSetDefaultProxyConfiguration_orig);
proxy_proc!(WinHttpSetOption, WinHttpSetOption_orig);
proxy_proc!(WinHttpSetStatusCallback, WinHttpSetStatusCallback_orig);
proxy_proc!(WinHttpSetTimeouts, WinHttpSetTimeouts_orig);
proxy_proc!(WinHttpTimeFromSystemTime, WinHttpTimeFromSystemTime_orig);
proxy_proc!(WinHttpTimeToSystemTime, WinHttpTimeToSystemTime_orig);
proxy_proc!(WinHttpWebSocketClose, WinHttpWebSocketClose_orig);
proxy_proc!(WinHttpWebSocketCompleteUpgrade, WinHttpWebSocketCompleteUpgrade_orig);
proxy_proc!(WinHttpWebSocketQueryCloseStatus, WinHttpWebSocketQueryCloseStatus_orig);
proxy_proc!(WinHttpWebSocketReceive, WinHttpWebSocketReceive_orig);
proxy_proc!(WinHttpWebSocketSend, WinHttpWebSocketSend_orig);
proxy_proc!(WinHttpWebSocketShutdown, WinHttpWebSocketShutdown_orig);
proxy_proc!(WinHttpWriteData, WinHttpWriteData_orig);
proxy_proc!(WinHttpWriteProxySettings, WinHttpWriteProxySettings_orig);

pub fn init(system_dir: &Utf16Str) {
    unsafe {
        let dll_path = system_dir.to_owned() + "\\winhttp.dll";
        let dll_path_cstr = U16CString::from_vec(dll_path.into_vec()).unwrap();
        let handle = LoadLibraryW(PCWSTR(dll_path_cstr.as_ptr())).expect("winhttp.dll");

        WinHttpAddRequestHeaders_orig = utils::get_proc_address(handle, cstr!("WinHttpAddRequestHeaders"));
        WinHttpCheckPlatform_orig = utils::get_proc_address(handle, cstr!("WinHttpCheckPlatform"));
        WinHttpCloseHandle_orig = utils::get_proc_address(handle, cstr!("WinHttpCloseHandle"));
        WinHttpConnect_orig = utils::get_proc_address(handle, cstr!("WinHttpConnect"));
        WinHttpCrackUrl_orig = utils::get_proc_address(handle, cstr!("WinHttpCrackUrl"));
        WinHttpCreateProxyResolver_orig = utils::get_proc_address(handle, cstr!("WinHttpCreateProxyResolver"));
        WinHttpCreateUrl_orig = utils::get_proc_address(handle, cstr!("WinHttpCreateUrl"));
        WinHttpDetectAutoProxyConfigUrl_orig = utils::get_proc_address(handle, cstr!("WinHttpDetectAutoProxyConfigUrl"));
        WinHttpFreeProxyResult_orig = utils::get_proc_address(handle, cstr!("WinHttpFreeProxyResult"));
        WinHttpFreeProxyResultEx_orig = utils::get_proc_address(handle, cstr!("WinHttpFreeProxyResultEx"));
        WinHttpFreeProxySettings_orig = utils::get_proc_address(handle, cstr!("WinHttpFreeProxySettings"));
        WinHttpGetDefaultProxyConfiguration_orig = utils::get_proc_address(handle, cstr!("WinHttpGetDefaultProxyConfiguration"));
        WinHttpGetIEProxyConfigForCurrentUser_orig = utils::get_proc_address(handle, cstr!("WinHttpGetIEProxyConfigForCurrentUser"));
        WinHttpGetProxyForUrl_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxyForUrl"));
        WinHttpGetProxyForUrlEx_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxyForUrlEx"));
        WinHttpGetProxyForUrlEx2_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxyForUrlEx2"));
        WinHttpGetProxyResult_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxyResult"));
        WinHttpGetProxyResultEx_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxyResultEx"));
        WinHttpGetProxySettingsVersion_orig = utils::get_proc_address(handle, cstr!("WinHttpGetProxySettingsVersion"));
        WinHttpOpen_orig = utils::get_proc_address(handle, cstr!("WinHttpOpen"));
        WinHttpOpenRequest_orig = utils::get_proc_address(handle, cstr!("WinHttpOpenRequest"));
        WinHttpQueryAuthSchemes_orig = utils::get_proc_address(handle, cstr!("WinHttpQueryAuthSchemes"));
        WinHttpQueryDataAvailable_orig = utils::get_proc_address(handle, cstr!("WinHttpQueryDataAvailable"));
        WinHttpQueryHeaders_orig = utils::get_proc_address(handle, cstr!("WinHttpQueryHeaders"));
        WinHttpQueryOption_orig = utils::get_proc_address(handle, cstr!("WinHttpQueryOption"));
        WinHttpReadData_orig = utils::get_proc_address(handle, cstr!("WinHttpReadData"));
        WinHttpReadProxySettings_orig = utils::get_proc_address(handle, cstr!("WinHttpReadProxySettings"));
        WinHttpReceiveResponse_orig = utils::get_proc_address(handle, cstr!("WinHttpReceiveResponse"));
        WinHttpResetAutoProxy_orig = utils::get_proc_address(handle, cstr!("WinHttpResetAutoProxy"));
        WinHttpSendRequest_orig = utils::get_proc_address(handle, cstr!("WinHttpSendRequest"));
        WinHttpSetCredentials_orig = utils::get_proc_address(handle, cstr!("WinHttpSetCredentials"));
        WinHttpSetDefaultProxyConfiguration_orig = utils::get_proc_address(handle, cstr!("WinHttpSetDefaultProxyConfiguration"));
        WinHttpSetOption_orig = utils::get_proc_address(handle, cstr!("WinHttpSetOption"));
        WinHttpSetStatusCallback_orig = utils::get_proc_address(handle, cstr!("WinHttpSetStatusCallback"));
        WinHttpSetTimeouts_orig = utils::get_proc_address(handle, cstr!("WinHttpSetTimeouts"));
        WinHttpTimeFromSystemTime_orig = utils::get_proc_address(handle, cstr!("WinHttpTimeFromSystemTime"));
        WinHttpTimeToSystemTime_orig = utils::get_proc_address(handle, cstr!("WinHttpTimeToSystemTime"));
        WinHttpWebSocketClose_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketClose"));
        WinHttpWebSocketCompleteUpgrade_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketCompleteUpgrade"));
        WinHttpWebSocketQueryCloseStatus_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketQueryCloseStatus"));
        WinHttpWebSocketReceive_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketReceive"));
        WinHttpWebSocketSend_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketSend"));
        WinHttpWebSocketShutdown_orig = utils::get_proc_address(handle, cstr!("WinHttpWebSocketShutdown"));
        WinHttpWriteData_orig = utils::get_proc_address(handle, cstr!("WinHttpWriteData"));
        WinHttpWriteProxySettings_orig = utils::get_proc_address(handle, cstr!("WinHttpWriteProxySettings"));
    }
}