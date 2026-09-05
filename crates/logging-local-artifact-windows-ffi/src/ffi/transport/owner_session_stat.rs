use super::*;

#[path = "owner_session_stat_path.rs"]
mod stat_path;

impl<'a> MutationSession<'a> {
    pub fn stat(&self, relative_path: &str) -> Result<Option<FileStat>, ArtifactError> {
        if relative_path.is_empty() {
            self.owner.verify_current()?;
            let root = self.owner.root_chain.leaf()?;
            let metadata = verify_metadata(root, true)?;
            return Ok(Some(FileStat::from_platform(metadata)));
        }
        let Some((_, target)) = stat_path::resolve(self, relative_path)? else {
            return Ok(None);
        };
        let file = match open_stat_target(&target) {
            Ok(file) => file,
            Err(ArtifactError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        let metadata = verify_metadata(&file, file.metadata()?.directory)?;
        self.owner.verify_current()?;
        Ok(Some(FileStat::from_platform(metadata)))
    }
}

fn open_stat_target(path: &Path) -> Result<OwnedFile, ArtifactError> {
    match OwnedFile::open_directory(path) {
        Ok(file) => Ok(file),
        Err(ArtifactError::InvalidPath(_)) => OwnedFile::open_existing_file(path),
        Err(error) => Err(error),
    }
}
