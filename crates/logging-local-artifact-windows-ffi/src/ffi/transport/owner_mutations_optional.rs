use super::*;

pub(super) fn directory(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    match OwnedFile::open_mutation_directory(path) {
        Ok(directory) => Ok(Some(directory)),
        Err(ArtifactError::NotFound) => Ok(None),
        Err(error) => Err(error),
    }
}

pub(super) fn file(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    match OwnedFile::open_existing_mutation_file(path) {
        Ok(file) => Ok(Some(file)),
        Err(ArtifactError::NotFound) => Ok(None),
        Err(error) => Err(error),
    }
}
