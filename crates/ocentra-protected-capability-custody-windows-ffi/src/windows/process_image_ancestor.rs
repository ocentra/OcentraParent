//! Retained executable-ancestor handles and observations.

use super::super::super::handles::HandleInner;
use super::security;
use super::{query_attribute_tag, query_identity};
use crate::{Error, ImageAncestorObservation, InputFault, Result, WindowsText};
use std::path::Path;
use std::ptr;
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
    FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    READ_CONTROL,
};

const MAX_IMAGE_ANCESTORS: usize = 64;

pub(super) fn open_ancestors(
    image_path: &WindowsText,
) -> Result<(Vec<HandleInner>, Vec<ImageAncestorObservation>)> {
    let mut handles = Vec::new();
    let mut observations = Vec::new();
    for ancestor_path in Path::new(image_path.as_str()).ancestors().skip(1) {
        if ancestor_path.as_os_str().is_empty() {
            continue;
        }
        if handles.len() >= MAX_IMAGE_ANCESTORS {
            return Err(Error::BufferTooLarge);
        }
        let path_text = ancestor_path
            .to_str()
            .ok_or(Error::InvalidInput(InputFault::ImageAncestorPathInvalid))?;
        let path_text = WindowsText::try_from_str(path_text)?;
        let path_wide = path_text.wide_nul()?;
        let handle = HandleInner::new(unsafe {
            CreateFileW(
                path_wide.as_ptr(),
                FILE_READ_ATTRIBUTES | READ_CONTROL,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL | FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                ptr::null_mut(),
            )
        })?;
        let observation = observe_ancestor(&handle, path_text)?;
        handles.push(handle);
        observations.push(observation);
    }
    if handles.is_empty() {
        return Err(Error::InvalidInput(InputFault::ImageAncestorPathInvalid));
    }
    Ok((handles, observations))
}

pub(super) fn reobserve_ancestors(
    handles: &[HandleInner],
    expected: &[ImageAncestorObservation],
) -> Result<Vec<ImageAncestorObservation>> {
    if handles.len() != expected.len() || handles.is_empty() {
        return Err(Error::InvalidInput(InputFault::ImageAncestorChanged));
    }
    let mut observations = Vec::with_capacity(handles.len());
    for (handle, expected) in handles.iter().zip(expected) {
        let current = observe_ancestor(handle, expected.path.clone())?;
        if current.identity != expected.identity {
            return Err(Error::InvalidInput(InputFault::ImageAncestorChanged));
        }
        observations.push(current);
    }
    Ok(observations)
}

fn observe_ancestor(handle: &HandleInner, path: WindowsText) -> Result<ImageAncestorObservation> {
    let identity = query_identity(handle.raw())?;
    let (file_attributes, reparse_tag) = query_attribute_tag(handle.raw())?;
    let security = security::query_file_security(handle.raw())?;
    Ok(ImageAncestorObservation {
        path,
        identity,
        security,
        file_attributes,
        reparse_tag,
    })
}
