//! Owned Windows kernel, registry, SCM, and TBS handles.

use crate::{Error, ImageObservation, RegistryAncestorObservation, Result};
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Registry::{RegCloseKey, HKEY};
use windows_sys::Win32::System::Services::{CloseServiceHandle, SC_HANDLE};
use windows_sys::Win32::System::TpmBaseServices::Tbsip_Context_Close;

pub(crate) struct HandleInner(pub(crate) HANDLE);

impl HandleInner {
    pub(crate) fn new(handle: HANDLE) -> Result<Self> {
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return Err(Error::Win32(last_error()));
        }
        Ok(Self(handle))
    }

    pub(crate) fn raw(&self) -> HANDLE {
        self.0
    }
}

impl Drop for HandleInner {
    fn drop(&mut self) {
        if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub(crate) struct ProcessInner {
    pub(crate) handle: HandleInner,
    pub(crate) process_id: u32,
    pub(crate) creation_time_100ns: u64,
    pub(crate) image: ImageInner,
}

pub(crate) struct TokenInner {
    pub(crate) handle: HandleInner,
}

pub(crate) struct ImageInner {
    pub(crate) _handle: HandleInner,
    pub(crate) observation: ImageObservation,
}

pub(crate) struct RegistryKeyInner {
    pub(crate) handle: HKEY,
}

pub(crate) struct RegistryChainInner {
    pub(crate) keys: Vec<RegistryKeyInner>,
    pub(crate) observations: Vec<RegistryAncestorObservation>,
}

pub(crate) struct ScManagerInner {
    pub(crate) handle: SC_HANDLE,
}

pub(crate) struct ServiceInner {
    pub(crate) handle: SC_HANDLE,
}

pub(crate) struct TbsContextInner {
    pub(crate) context: *mut core::ffi::c_void,
}

impl Drop for RegistryKeyInner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { RegCloseKey(self.handle) };
        }
    }
}

impl Drop for ScManagerInner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseServiceHandle(self.handle) };
        }
    }
}

impl Drop for ServiceInner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseServiceHandle(self.handle) };
        }
    }
}

impl Drop for TbsContextInner {
    fn drop(&mut self) {
        if !self.context.is_null() {
            unsafe { Tbsip_Context_Close(self.context) };
        }
    }
}

pub(crate) fn last_error() -> u32 {
    unsafe { GetLastError() }
}
