//! Retained process mechanics and epoch/liveness observation.

#[path = "process_image.rs"]
mod image;
#[path = "process_token.rs"]
mod token;

use super::handles::{last_error, HandleInner, ProcessInner, TokenInner};
use crate::{Error, InputFault, OwnedProcess, OwnedToken, ProcessObservation, Result};
use std::ptr;
use windows_sys::Win32::Foundation::{FILETIME, HANDLE, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows_sys::Win32::Security::TOKEN_QUERY;
use windows_sys::Win32::System::Threading::{
    GetProcessTimes, OpenProcess, OpenProcessToken, WaitForSingleObject,
    PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
};

impl OwnedProcess {
    pub fn open_for_peer_observation(process_id: u32) -> Result<Self> {
        if process_id == 0 {
            return Err(Error::InvalidInput(InputFault::ProcessIdZero));
        }
        // All observations and OpenProcessToken require only limited query
        // access. SYNCHRONIZE is the least-privilege right needed for an
        // unambiguous nonblocking liveness check; no caller-selected access
        // mask crosses this boundary.
        let desired_access = PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE;
        let handle = HandleInner::new(unsafe { OpenProcess(desired_access, 0, process_id) })?;
        let creation_time_100ns = process_creation_time(handle.raw())?;
        let image = image::open_image(handle.raw())?;
        Ok(Self {
            inner: ProcessInner {
                handle,
                process_id,
                creation_time_100ns,
                image,
            },
        })
    }

    pub fn observation(&self) -> Result<ProcessObservation> {
        let alive = match unsafe { WaitForSingleObject(self.inner.handle.raw(), 0) } {
            WAIT_TIMEOUT => true,
            WAIT_OBJECT_0 => false,
            WAIT_FAILED => return Err(Error::Win32(last_error())),
            _ => return Err(Error::InvalidInput(InputFault::ProcessWaitResultInvalid)),
        };
        Ok(ProcessObservation {
            process_id: self.inner.process_id,
            creation_time_100ns: self.inner.creation_time_100ns,
            image: image::reobserve_image(&self.inner.image)?,
            alive,
        })
    }

    /// Re-observe the executable and every retained ancestor handle.
    pub fn reobserve_image(&self) -> Result<crate::ImageObservation> {
        image::reobserve_image(&self.inner.image)
    }

    pub fn open_token(&self) -> Result<OwnedToken> {
        let mut token = ptr::null_mut();
        if unsafe { OpenProcessToken(self.inner.handle.raw(), TOKEN_QUERY, &mut token) } == 0 {
            return Err(Error::Win32(last_error()));
        }
        Ok(OwnedToken {
            inner: TokenInner {
                handle: HandleInner::new(token)?,
            },
        })
    }
}

fn process_creation_time(handle: HANDLE) -> Result<u64> {
    let mut creation = FILETIME::default();
    let mut exit = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    if unsafe { GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user) } == 0 {
        return Err(Error::Win32(last_error()));
    }
    let value = (u64::from(creation.dwHighDateTime) << 32) | u64::from(creation.dwLowDateTime);
    if value == 0 {
        return Err(Error::InvalidInput(InputFault::ProcessEpochMissing));
    }
    Ok(value)
}
