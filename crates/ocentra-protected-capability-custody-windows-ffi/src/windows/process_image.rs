//! Pinned executable image mechanics.

#[path = "process_image_path.rs"]
mod path;

use super::super::handles::{last_error, HandleInner, ImageInner};
use crate::{Error, ImageIdentity, ImageObservation, Result};
use sha2::{Digest, Sha256};
use std::ptr;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FileIdInfo, FileStandardInfo, GetFileInformationByHandleEx, ReadFile,
    FILE_ATTRIBUTE_NORMAL, FILE_FLAG_SEQUENTIAL_SCAN, FILE_GENERIC_READ, FILE_ID_INFO,
    FILE_READ_ATTRIBUTES, FILE_SHARE_READ, FILE_STANDARD_INFO, OPEN_EXISTING,
};

const MAX_IMAGE_BYTES: u64 = 256 * 1024 * 1024;
const IMAGE_READ_CHUNK: usize = 64 * 1024;

pub(super) fn open_image(process: HANDLE) -> Result<ImageInner> {
    let path = path::query_image_path(process)?;
    let path_wide = path::wide_string(&path)?;
    let handle = HandleInner::new(unsafe {
        CreateFileW(
            path_wide.as_ptr(),
            FILE_GENERIC_READ | FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ,
            ptr::null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL | FILE_FLAG_SEQUENTIAL_SCAN,
            ptr::null_mut(),
        )
    })?;
    let (identity, size) = query_image_file(handle.raw())?;
    if size == 0 || size > MAX_IMAGE_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let sha256 = read_image_digest(handle.raw(), size)?;
    let (final_identity, final_size) = query_image_file(handle.raw())?;
    if final_identity != identity || final_size != size {
        return Err(Error::InvalidInput(
            "executable changed while its digest was read",
        ));
    }
    Ok(ImageInner {
        _handle: handle,
        observation: ImageObservation {
            path,
            identity,
            sha256,
        },
    })
}

fn query_image_file(handle: HANDLE) -> Result<(ImageIdentity, u64)> {
    let mut identity = FILE_ID_INFO::default();
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileIdInfo,
            &mut identity as *mut FILE_ID_INFO as *mut core::ffi::c_void,
            u32::try_from(core::mem::size_of::<FILE_ID_INFO>())
                .map_err(|_| Error::BufferTooLarge)?,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let mut standard = FILE_STANDARD_INFO::default();
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileStandardInfo,
            &mut standard as *mut FILE_STANDARD_INFO as *mut core::ffi::c_void,
            u32::try_from(core::mem::size_of::<FILE_STANDARD_INFO>())
                .map_err(|_| Error::BufferTooLarge)?,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    if standard.EndOfFile < 0 {
        return Err(Error::InvalidInput(
            "Windows returned a negative executable size",
        ));
    }
    Ok((
        ImageIdentity {
            volume_serial_number: identity.VolumeSerialNumber,
            file_id: identity.FileId.Identifier,
        },
        u64::try_from(standard.EndOfFile).map_err(|_| Error::BufferTooLarge)?,
    ))
}

fn read_image_digest(handle: HANDLE, size: u64) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut remaining = size;
    let mut buffer = vec![0u8; IMAGE_READ_CHUNK];
    while remaining > 0 {
        let requested = usize::try_from(remaining)
            .map_err(|_| Error::BufferTooLarge)?
            .min(buffer.len());
        let mut read = 0u32;
        if unsafe {
            ReadFile(
                handle,
                buffer.as_mut_ptr(),
                u32::try_from(requested).map_err(|_| Error::BufferTooLarge)?,
                &mut read,
                ptr::null_mut(),
            )
        } == 0
        {
            return Err(Error::Win32(last_error()));
        }
        let read = usize::try_from(read).map_err(|_| Error::BufferTooLarge)?;
        if read == 0 || read > requested {
            return Err(Error::InvalidInput(
                "executable read returned an invalid size",
            ));
        }
        hasher.update(&buffer[..read]);
        remaining = remaining
            .checked_sub(u64::try_from(read).map_err(|_| Error::BufferTooLarge)?)
            .ok_or(Error::InvalidInput(
                "executable read exceeded its pinned size",
            ))?;
    }
    let digest = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}
