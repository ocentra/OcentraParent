//! Bounded token SID, integrity, and session mechanics.

#[path = "process_token_sid.rs"]
mod sid;

use super::super::handles::{last_error, HandleInner, TokenInner};
use crate::{Error, OwnedToken, Result, TokenObservation};
use std::ptr;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Security::{
    GetTokenInformation, TokenIntegrityLevel, TokenSessionId, TokenUser, TOKEN_INFORMATION_CLASS,
    TOKEN_MANDATORY_LABEL, TOKEN_QUERY, TOKEN_USER,
};
use windows_sys::Win32::System::Threading::{GetCurrentThread, OpenThreadToken};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const TOKEN_INFORMATION_MAX: usize = 64 * 1024;

impl OwnedToken {
    pub fn open_current_thread() -> Result<Self> {
        let mut token = ptr::null_mut();
        if unsafe { OpenThreadToken(GetCurrentThread(), TOKEN_QUERY, 1, &mut token) } == 0 {
            return Err(Error::Win32(last_error()));
        }
        Ok(Self {
            inner: TokenInner {
                handle: HandleInner::new(token)?,
            },
        })
    }

    pub fn observation(&self) -> Result<TokenObservation> {
        token_observation(self.inner.handle.raw())
    }
}

fn token_observation(handle: HANDLE) -> Result<TokenObservation> {
    let user_bytes = query_token_information(handle, TokenUser)?;
    if user_bytes.len() < core::mem::size_of::<TOKEN_USER>() {
        return Err(Error::InvalidInput("token user response is too small"));
    }
    let user = unsafe { ptr::read_unaligned(user_bytes.as_ptr() as *const TOKEN_USER) };
    let sid = sid::copy_sid_in_buffer(user.User.Sid, &user_bytes)?;
    let integrity_bytes = query_token_information(handle, TokenIntegrityLevel)?;
    if integrity_bytes.len() < core::mem::size_of::<TOKEN_MANDATORY_LABEL>() {
        return Err(Error::InvalidInput("token integrity response is too small"));
    }
    let integrity =
        unsafe { ptr::read_unaligned(integrity_bytes.as_ptr() as *const TOKEN_MANDATORY_LABEL) };
    let integrity_level = sid::integrity_rid_in_buffer(integrity.Label.Sid, &integrity_bytes)?;
    let session = query_token_information(handle, TokenSessionId)?;
    if session.len() != core::mem::size_of::<u32>() {
        return Err(Error::InvalidInput(
            "token session response has an invalid size",
        ));
    }
    let session_id = unsafe { ptr::read_unaligned(session.as_ptr() as *const u32) };
    Ok(TokenObservation {
        sid,
        integrity_level,
        session_id,
    })
}

fn query_token_information(handle: HANDLE, class: TOKEN_INFORMATION_CLASS) -> Result<Vec<u8>> {
    let mut length = 0u32;
    let first_ok = unsafe { GetTokenInformation(handle, class, ptr::null_mut(), 0, &mut length) };
    let first_error = last_error();
    if first_ok != 0 || first_error != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(first_error));
    }
    let length = usize::try_from(length).map_err(|_| Error::BufferTooLarge)?;
    if length == 0 || length > TOKEN_INFORMATION_MAX {
        return Err(Error::BufferTooLarge);
    }
    let mut buffer = vec![0u8; length];
    let mut returned_length = u32::try_from(buffer.len()).map_err(|_| Error::BufferTooLarge)?;
    if unsafe {
        GetTokenInformation(
            handle,
            class,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
            u32::try_from(buffer.len()).map_err(|_| Error::BufferTooLarge)?,
            &mut returned_length,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length).map_err(|_| Error::BufferTooLarge)?;
    if returned_length > buffer.len() {
        return Err(Error::InvalidInput(
            "token information response exceeds its buffer",
        ));
    }
    buffer.truncate(returned_length);
    Ok(buffer)
}
