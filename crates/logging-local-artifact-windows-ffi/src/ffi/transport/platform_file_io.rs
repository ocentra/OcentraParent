use super::*;

impl OwnedFile {
    pub(crate) fn raw(&self) -> HANDLE {
        self.file.as_raw_handle() as HANDLE
    }

    pub(crate) fn metadata(&self) -> Result<Metadata, ArtifactError> {
        query_metadata(self.raw())
    }

    pub(crate) fn process_image_metadata(&self) -> Result<Metadata, ArtifactError> {
        verify_process_image_metadata(self)
    }

    pub(crate) fn sync_file(&self) -> Result<(), ArtifactError> {
        self.file
            .sync_all()
            .map_err(|error| ArtifactError::DurabilityFailureWith(error.to_string()))
    }

    pub(crate) fn sync_directory(&self) -> Result<(), ArtifactError> {
        if unsafe { windows_sys::Win32::Storage::FileSystem::FlushFileBuffers(self.raw()) } == 0 {
            return Err(ArtifactError::DurabilityFailureWith(
                std::io::Error::last_os_error().to_string(),
            ));
        }
        Ok(())
    }

    pub(crate) fn try_lock_exclusive(&self) -> Result<(), ArtifactError> {
        self.file.try_lock_exclusive().map_err(io_error)
    }

    pub(crate) fn unlock(&self) -> Result<(), ArtifactError> {
        self.file.unlock().map_err(io_error)
    }

    pub(crate) fn read_bounded(&mut self, max_bytes: u64) -> Result<Vec<u8>, ArtifactError> {
        let before = verify_metadata(self, false)?;
        if before.length > max_bytes || before.length > MAX_ARTIFACT_BYTES {
            return Err(ArtifactError::SizeLimit);
        }
        let length = usize::try_from(before.length).map_err(|_| ArtifactError::SizeLimit)?;
        let mut bytes = vec![0u8; length];
        self.file.seek(SeekFrom::Start(0)).map_err(io_error)?;
        self.file.read_exact(&mut bytes).map_err(io_error)?;
        let after = verify_metadata(self, false)?;
        if after.identity != before.identity
            || after.length != before.length
            || after.modified_ms != before.modified_ms
        {
            return Err(ArtifactError::OwnershipChanged);
        }
        Ok(bytes)
    }

    pub(crate) fn append_bounded(&mut self, bytes: &[u8]) -> Result<u64, ArtifactError> {
        let before = verify_metadata(self, false)?;
        let append_len = u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeLimit)?;
        let end = before
            .length
            .checked_add(append_len)
            .ok_or(ArtifactError::SizeLimit)?;
        if end > MAX_ARTIFACT_BYTES {
            return Err(ArtifactError::SizeLimit);
        }
        self.file.seek(SeekFrom::End(0)).map_err(io_error)?;
        let actual_end = self.file.stream_position().map_err(io_error)?;
        if actual_end != before.length {
            return Err(ArtifactError::OwnershipChanged);
        }
        self.file.write_all(bytes).map_err(io_error)?;
        Ok(before.length)
    }

    pub(crate) fn write_bounded(&mut self, bytes: &[u8]) -> Result<(), ArtifactError> {
        let length = u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeLimit)?;
        if length > MAX_ARTIFACT_BYTES {
            return Err(ArtifactError::SizeLimit);
        }
        self.file.set_len(0).map_err(io_error)?;
        self.file.seek(SeekFrom::Start(0)).map_err(io_error)?;
        self.file.write_all(bytes).map_err(io_error)
    }

    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), ArtifactError> {
        self.file.write_all(bytes).map_err(io_error)
    }
}
