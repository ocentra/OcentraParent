use super::*;

impl<'a> MutationSession<'a> {
    pub fn list(&self, relative_path: &str) -> Result<Vec<DirectoryEntry>, ArtifactError> {
        let target = self.directory_path(relative_path)?;
        let chain = open_directory_chain(&target)?;
        self.verify_chain(&chain)?;
        let mut entries = Vec::new();
        for entry in fs::read_dir(&target).map_err(io_error)? {
            entries.push(read_directory_entry(&target, entry.map_err(io_error)?)?);
        }
        self.owner.verify_current()?;
        Ok(entries)
    }
}

fn read_directory_entry(
    target: &Path,
    entry: fs::DirEntry,
) -> Result<DirectoryEntry, ArtifactError> {
    let name = entry
        .file_name()
        .to_str()
        .ok_or(ArtifactError::InvalidPath(DIRECTORY_ENTRY_NOT_UNICODE))?
        .to_owned();
    crate::platform::windows::validate_leaf(&name)?;
    let entry_path = target.join(&name);
    let file = open_directory_entry(&entry_path)?;
    let metadata = verify_metadata(&file, file.metadata()?.directory)?;
    Ok(DirectoryEntry::new(name, FileStat::from_platform(metadata)))
}

fn open_directory_entry(path: &Path) -> Result<OwnedFile, ArtifactError> {
    match OwnedFile::open_directory(path) {
        Ok(file) => Ok(file),
        Err(ArtifactError::InvalidPath(_)) => OwnedFile::open_existing_file(path),
        Err(ArtifactError::LinkOrReparseDetected) => Err(ArtifactError::LinkOrReparseDetected),
        Err(error) => Err(error),
    }
}
