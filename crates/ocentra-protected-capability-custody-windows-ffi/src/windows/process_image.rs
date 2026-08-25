//! Pinned executable image mechanics.

#[path = "process_image_ancestor.rs"]
mod ancestor;
#[path = "process_image_path.rs"]
mod path;
#[path = "process_image_security.rs"]
mod security;

use super::super::handles::{last_error, HandleInner, ImageInner};
use crate::{Error, ImageIdentity, ImageObservation, InputFault, Result};
use sha2::{Digest, Sha256};
use std::ptr;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FileAttributeTagInfo, FileIdInfo, FileStandardInfo, GetFileInformationByHandleEx,
    ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_TAG_INFO, FILE_FLAG_OPEN_REPARSE_POINT,
    FILE_FLAG_SEQUENTIAL_SCAN, FILE_GENERIC_READ, FILE_ID_INFO, FILE_READ_ATTRIBUTES,
    FILE_SHARE_READ, FILE_STANDARD_INFO, OPEN_EXISTING,
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
            FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_SEQUENTIAL_SCAN,
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
        return Err(Error::InvalidInput(InputFault::ImageChangedDuringRead));
    }
    let (file_attributes, reparse_tag) = query_attribute_tag(handle.raw())?;
    let security = security::query_file_security(handle.raw())?;
    let (ancestor_handles, ancestors) = ancestor::open_ancestors(&path)?;
    Ok(ImageInner {
        handle,
        ancestor_handles,
        observation: ImageObservation {
            path,
            identity,
            sha256,
            security,
            ancestors,
            file_attributes,
            reparse_tag,
        },
    })
}

pub(super) fn reobserve_image(image: &ImageInner) -> Result<ImageObservation> {
    let (identity, size) = query_image_file(image.handle.raw())?;
    if identity != image.observation.identity || size == 0 || size > MAX_IMAGE_BYTES {
        return Err(Error::InvalidInput(InputFault::ImageChangedDuringRead));
    }
    let (file_attributes, reparse_tag) = query_attribute_tag(image.handle.raw())?;
    let security = security::query_file_security(image.handle.raw())?;
    let ancestors =
        ancestor::reobserve_ancestors(&image.ancestor_handles, &image.observation.ancestors)?;
    Ok(ImageObservation {
        path: image.observation.path.clone(),
        identity,
        sha256: image.observation.sha256,
        security,
        ancestors,
        file_attributes,
        reparse_tag,
    })
}

fn query_image_file(handle: HANDLE) -> Result<(ImageIdentity, u64)> {
    let mut identity = FILE_ID_INFO::default();
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileIdInfo,
            &mut identity as *mut FILE_ID_INFO as *mut core::ffi::c_void,
            u32::try_from(core::mem::size_of::<FILE_ID_INFO>())?,
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
            u32::try_from(core::mem::size_of::<FILE_STANDARD_INFO>())?,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    if standard.EndOfFile < 0 {
        return Err(Error::InvalidInput(InputFault::ImageSizeNegative));
    }
    Ok((
        ImageIdentity {
            volume_serial_number: identity.VolumeSerialNumber,
            file_id: identity.FileId.Identifier,
        },
        u64::try_from(standard.EndOfFile)?,
    ))
}

pub(super) fn query_identity(handle: HANDLE) -> Result<ImageIdentity> {
    query_image_file(handle).map(|(identity, _size)| identity)
}

pub(super) fn query_attribute_tag(handle: HANDLE) -> Result<(u32, u32)> {
    let mut tag = FILE_ATTRIBUTE_TAG_INFO::default();
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileAttributeTagInfo,
            &mut tag as *mut FILE_ATTRIBUTE_TAG_INFO as *mut core::ffi::c_void,
            u32::try_from(core::mem::size_of::<FILE_ATTRIBUTE_TAG_INFO>())?,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    Ok((tag.FileAttributes, tag.ReparseTag))
}

fn read_image_digest(handle: HANDLE, size: u64) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut remaining = size;
    let mut buffer = vec![0u8; IMAGE_READ_CHUNK];
    while remaining > 0 {
        let requested = usize::try_from(remaining)?.min(buffer.len());
        let mut read = 0u32;
        if unsafe {
            ReadFile(
                handle,
                buffer.as_mut_ptr(),
                u32::try_from(requested)?,
                &mut read,
                ptr::null_mut(),
            )
        } == 0
        {
            return Err(Error::Win32(last_error()));
        }
        let read = usize::try_from(read)?;
        if read == 0 || read > requested {
            return Err(Error::InvalidInput(InputFault::ImageReadSizeInvalid));
        }
        hasher.update(&buffer[..read]);
        remaining = remaining
            .checked_sub(u64::try_from(read)?)
            .ok_or(Error::InvalidInput(InputFault::ImageReadExceededSize))?;
    }
    let digest = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}
