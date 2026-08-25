//! Bounded QueryServiceConfig and QueryServiceConfig2 mechanics.

#[path = "registry_service_strings.rs"]
pub(crate) mod strings;

use super::super::super::handles::last_error;
use crate::{Error, InputFault, Result, WindowsText, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::System::Services::{
    QueryServiceConfig2W, QueryServiceConfigW, QUERY_SERVICE_CONFIGW, SC_HANDLE,
    SERVICE_CONFIG_DELAYED_AUTO_START_INFO, SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO,
    SERVICE_CONFIG_SERVICE_SID_INFO, SERVICE_DELAYED_AUTO_START_INFO,
    SERVICE_REQUIRED_PRIVILEGES_INFOW, SERVICE_SID_INFO,
};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

pub(super) struct ServiceConfigSnapshot {
    pub(super) service_type: u32,
    pub(super) start_type: u32,
    pub(super) error_control: u32,
    pub(super) binary_path: Option<WindowsText>,
    pub(super) load_order_group: Option<WindowsText>,
    pub(super) tag_id: u32,
    pub(super) dependencies: Vec<WindowsText>,
    pub(super) start_name: Option<WindowsText>,
    pub(super) display_name: Option<WindowsText>,
}

pub(super) fn query_service_config(handle: SC_HANDLE) -> Result<ServiceConfigSnapshot> {
    let buffer = query_config_buffer(handle)?;
    let returned_length = buffer.len();
    if returned_length < core::mem::size_of::<QUERY_SERVICE_CONFIGW>() {
        return Err(Error::InvalidInput(
            InputFault::ServiceConfigurationSizeInvalid,
        ));
    }
    let config = unsafe { ptr::read_unaligned(buffer.as_ptr() as *const QUERY_SERVICE_CONFIGW) };
    Ok(ServiceConfigSnapshot {
        service_type: config.dwServiceType,
        start_type: config.dwStartType,
        error_control: config.dwErrorControl,
        binary_path: strings::copy_optional_wide_ptr(config.lpBinaryPathName, &buffer)?,
        load_order_group: strings::copy_optional_wide_ptr(config.lpLoadOrderGroup, &buffer)?,
        tag_id: config.dwTagId,
        dependencies: strings::copy_multi_sz_ptr(config.lpDependencies, &buffer)?,
        start_name: strings::copy_optional_wide_ptr(config.lpServiceStartName, &buffer)?,
        display_name: strings::copy_optional_wide_ptr(config.lpDisplayName, &buffer)?,
    })
}

pub(super) fn query_service_config2(handle: SC_HANDLE, level: u32) -> Result<Vec<u8>> {
    let mut length = 0u32;
    let first = unsafe { QueryServiceConfig2W(handle, level, ptr::null_mut(), 0, &mut length) };
    if first != 0 || last_error() != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(last_error()));
    }
    let length = usize::try_from(length)?;
    if length == 0 || length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut buffer = vec![0u8; length];
    let mut returned_length = u32::try_from(buffer.len())?;
    if unsafe {
        QueryServiceConfig2W(
            handle,
            level,
            buffer.as_mut_ptr(),
            u32::try_from(buffer.len())?,
            &mut returned_length,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length)?;
    if returned_length == 0 || returned_length > buffer.len() {
        return Err(Error::InvalidInput(InputFault::ServiceExtendedSizeInvalid));
    }
    buffer.truncate(returned_length);
    Ok(buffer)
}

pub(super) fn query_service_sid_type(handle: SC_HANDLE) -> Result<u32> {
    let buffer = query_service_config2(handle, SERVICE_CONFIG_SERVICE_SID_INFO)?;
    if buffer.len() != core::mem::size_of::<SERVICE_SID_INFO>() {
        return Err(Error::InvalidInput(InputFault::ServiceSidSizeInvalid));
    }
    Ok(unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_SID_INFO) }.dwServiceSidType)
}

pub(super) fn query_required_privileges(handle: SC_HANDLE) -> Result<Vec<WindowsText>> {
    let buffer = query_service_config2(handle, SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO)?;
    if buffer.len() < core::mem::size_of::<SERVICE_REQUIRED_PRIVILEGES_INFOW>() {
        return Err(Error::InvalidInput(InputFault::ServicePrivilegeSizeInvalid));
    }
    let value =
        unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_REQUIRED_PRIVILEGES_INFOW) };
    strings::copy_multi_sz_ptr(value.pmszRequiredPrivileges, &buffer)
}

pub(super) fn query_delayed_auto_start(handle: SC_HANDLE) -> Result<bool> {
    let buffer = query_service_config2(handle, SERVICE_CONFIG_DELAYED_AUTO_START_INFO)?;
    if buffer.len() != core::mem::size_of::<SERVICE_DELAYED_AUTO_START_INFO>() {
        return Err(Error::InvalidInput(
            InputFault::ServiceDelayedStartSizeInvalid,
        ));
    }
    Ok(
        unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_DELAYED_AUTO_START_INFO) }
            .fDelayedAutostart
            != 0,
    )
}

fn query_config_buffer(handle: SC_HANDLE) -> Result<Vec<u8>> {
    let mut length = 0u32;
    let first = unsafe { QueryServiceConfigW(handle, ptr::null_mut(), 0, &mut length) };
    if first != 0 || last_error() != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(last_error()));
    }
    let length = usize::try_from(length)?;
    if length == 0 || length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut buffer = vec![0u8; length];
    let mut returned_length = u32::try_from(buffer.len())?;
    if unsafe {
        QueryServiceConfigW(
            handle,
            buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW,
            u32::try_from(buffer.len())?,
            &mut returned_length,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length)?;
    if returned_length < core::mem::size_of::<QUERY_SERVICE_CONFIGW>()
        || returned_length > buffer.len()
    {
        return Err(Error::InvalidInput(
            InputFault::ServiceConfigurationSizeInvalid,
        ));
    }
    buffer.truncate(returned_length);
    Ok(buffer)
}
