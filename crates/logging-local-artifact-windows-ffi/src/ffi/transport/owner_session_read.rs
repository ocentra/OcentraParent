use super::*;

#[path = "owner_session_read_snapshot_path.rs"]
mod snapshot_path;

impl<'a> MutationSession<'a> {
    pub fn read(&self, relative_path: &str, max_bytes: u64) -> Result<Vec<u8>, ArtifactError> {
        let (chain, target, _) =
            crate::owner_paths::parent_and_leaf(&self.owner.root_path, relative_path)?;
        self.verify_chain(&chain)?;
        let mut file = OwnedFile::open_existing_mutation_file(&target)?;
        let bytes = file.read_bounded(max_bytes)?;
        self.owner.verify_current()?;
        Ok(bytes)
    }

    pub fn read_snapshot(
        &self,
        relative_path: &str,
        max_bytes: u64,
    ) -> Result<Option<ReadSnapshot>, ArtifactError> {
        let Some((chain, target)) = snapshot_path::resolve(self, relative_path)? else {
            return Ok(None);
        };
        let mut file = match OwnedFile::open_existing_mutation_file(&target) {
            Ok(file) => file,
            Err(ArtifactError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        let before = verify_metadata(&file, false)?;
        let bytes = file.read_bounded(max_bytes)?;
        let after = verify_metadata(&file, false)?;
        if before.identity != after.identity
            || before.length != after.length
            || before.modified_ms != after.modified_ms
        {
            return Err(ArtifactError::OwnershipChanged);
        }
        self.verify_chain(&chain)?;
        Ok(Some(ReadSnapshot::new(
            FileStat::from_platform(after),
            bytes,
        )))
    }
}
