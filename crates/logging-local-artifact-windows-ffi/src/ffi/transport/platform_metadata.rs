use super::*;

pub(crate) fn query_metadata(handle: HANDLE) -> Result<Metadata, ArtifactError> {
    let identity = query_file_id(handle)?;
    let standard = query_standard_info(handle)?;
    let attributes = query_attributes(handle)?;
    let modified_ms = query_basic_info(handle)?;
    Ok(Metadata {
        identity: Identity {
            volume_serial_number: identity.VolumeSerialNumber,
            file_id: identity.FileId.Identifier,
        },
        length: u64::try_from(standard.EndOfFile).map_err(|_| ArtifactError::SizeLimit)?,
        links: standard.NumberOfLinks,
        directory: standard.Directory,
        attributes: attributes.FileAttributes,
        reparse_tag: attributes.ReparseTag,
        modified_ms,
    })
}

fn query_file_id(handle: HANDLE) -> Result<FILE_ID_INFO, ArtifactError> {
    query_info::<FILE_ID_INFO>(handle, FileIdInfo, FILE_ID_INFO_FAILURE)
}

fn query_standard_info(handle: HANDLE) -> Result<FILE_STANDARD_INFO, ArtifactError> {
    let standard =
        query_info::<FILE_STANDARD_INFO>(handle, FileStandardInfo, FILE_STANDARD_INFO_FAILURE)?;
    if standard.EndOfFile < 0 {
        return Err(ArtifactError::Io(NEGATIVE_FILE_LENGTH.to_owned()));
    }
    Ok(standard)
}

fn query_attributes(handle: HANDLE) -> Result<FILE_ATTRIBUTE_TAG_INFO, ArtifactError> {
    query_info::<FILE_ATTRIBUTE_TAG_INFO>(
        handle,
        FileAttributeTagInfo,
        FILE_ATTRIBUTE_TAG_INFO_FAILURE,
    )
}

fn query_basic_info(handle: HANDLE) -> Result<i64, ArtifactError> {
    let basic: FILE_BASIC_INFO = query_info(handle, FileBasicInfo, FILE_BASIC_INFO_FAILURE)?;
    if basic.LastWriteTime < 0 {
        return Err(ArtifactError::Io(NEGATIVE_LAST_WRITE_TIME.to_owned()));
    }
    Ok(basic.LastWriteTime / 10_000)
}

fn query_info<T>(
    handle: HANDLE,
    class: windows_sys::Win32::Storage::FileSystem::FILE_INFO_BY_HANDLE_CLASS,
    error_prefix: &str,
) -> Result<T, ArtifactError> {
    let mut value = MaybeUninit::<T>::zeroed();
    let size = u32::try_from(size_of::<T>()).map_err(|_| ArtifactError::SizeLimit)?;
    if unsafe { GetFileInformationByHandleEx(handle, class, value.as_mut_ptr() as *mut _, size) }
        == 0
    {
        return Err(win32_error(error_prefix));
    }
    Ok(unsafe { value.assume_init() })
}
