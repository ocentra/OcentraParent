use super::*;

pub(super) fn open(path: &Path, directory: bool) -> Result<Option<OwnedFile>, ArtifactError> {
    if directory {
        return open_directory(path);
    }
    open_file(path)
}

fn open_directory(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    match OwnedFile::open_mutation_directory(path) {
        Ok(file) => Ok(Some(file)),
        Err(ArtifactError::NotFound) => Ok(None),
        Err(error) => Err(error),
    }
}

fn open_file(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    match OwnedFile::open_existing_mutation_file(path) {
        Ok(file) => Ok(Some(file)),
        Err(ArtifactError::NotFound) => Ok(None),
        Err(error) => Err(error),
    }
}
