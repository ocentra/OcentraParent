use super::*;

impl OwnedFile {
    pub(crate) fn rename_into(
        &self,
        parent: &OwnedFile,
        destination_name: &str,
    ) -> Result<(), ArtifactError> {
        self.rename_into_with_replacement(parent, destination_name, false)
    }

    pub(crate) fn replace_into(
        &self,
        parent: &OwnedFile,
        destination_name: &str,
    ) -> Result<(), ArtifactError> {
        self.rename_into_with_replacement(parent, destination_name, true)
    }

    fn rename_into_with_replacement(
        &self,
        parent: &OwnedFile,
        destination_name: &str,
        replace_if_exists: bool,
    ) -> Result<(), ArtifactError> {
        validate_leaf(destination_name)?;
        let _rename_parent = Self::open_rename_directory(&parent.path)?;
        let destination_path = parent.path.join(destination_name);
        let wide: Vec<u16> = destination_path.as_os_str().encode_wide().collect();
        let name_bytes = wide
            .len()
            .checked_mul(size_of::<u16>())
            .ok_or(ArtifactError::SizeLimit)?;
        let name_bytes_u32 = u32::try_from(name_bytes).map_err(|_| ArtifactError::SizeLimit)?;
        let header_bytes = std::mem::offset_of!(FILE_RENAME_INFO, FileName);
        let total_bytes = header_bytes
            .checked_add(name_bytes)
            .and_then(|size| size.checked_add(size_of::<u16>()))
            .ok_or(ArtifactError::SizeLimit)?;
        let total_bytes_u32 = u32::try_from(total_bytes).map_err(|_| ArtifactError::SizeLimit)?;
        let info = FILE_RENAME_INFO {
            Anonymous: FILE_RENAME_INFO_0 {
                // The ABI's first union member is a one-byte BOOLEAN.
                ReplaceIfExists: replace_if_exists,
            },
            RootDirectory: std::ptr::null_mut(),
            FileNameLength: name_bytes_u32,
            FileName: [0],
        };
        let mut buffer = vec![0u8; total_bytes];
        unsafe {
            ptr::write_unaligned(buffer.as_mut_ptr() as *mut FILE_RENAME_INFO, info);
            ptr::copy_nonoverlapping(
                wide.as_ptr() as *const u8,
                buffer.as_mut_ptr().add(header_bytes),
                name_bytes,
            );
            if SetFileInformationByHandle(
                self.raw(),
                FileRenameInfo,
                buffer.as_ptr() as *const _,
                total_bytes_u32,
            ) == 0
            {
                return Err(win32_error(FILE_RENAME_INFO_FAILURE));
            }
        }
        Ok(())
    }

    pub(crate) fn mark_deleted(&self) -> Result<(), ArtifactError> {
        let disposition = FILE_DISPOSITION_INFO_EX {
            Flags: FILE_DISPOSITION_FLAG_DELETE | FILE_DISPOSITION_FLAG_POSIX_SEMANTICS,
        };
        let size = u32::try_from(size_of::<FILE_DISPOSITION_INFO_EX>())
            .map_err(|_| ArtifactError::SizeLimit)?;
        let ex_result = unsafe {
            SetFileInformationByHandle(
                self.raw(),
                FileDispositionInfoEx,
                &disposition as *const FILE_DISPOSITION_INFO_EX as *const _,
                size,
            )
        };
        if ex_result != 0 {
            return Ok(());
        }
        let classic = FILE_DISPOSITION_INFO { DeleteFile: true };
        let classic_size = u32::try_from(size_of::<FILE_DISPOSITION_INFO>())
            .map_err(|_| ArtifactError::SizeLimit)?;
        if unsafe {
            SetFileInformationByHandle(
                self.raw(),
                FileDispositionInfo,
                &classic as *const FILE_DISPOSITION_INFO as *const _,
                classic_size,
            )
        } == 0
        {
            return Err(win32_error(FILE_DISPOSITION_INFO_FAILURE));
        }
        Ok(())
    }
}
