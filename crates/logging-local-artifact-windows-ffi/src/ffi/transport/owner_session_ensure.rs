use super::*;

pub(super) fn ensure_child_directory(path: &Path) -> Result<bool, ArtifactError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(false),
        Ok(_) => Err(ArtifactError::InvalidPath(PATH_NOT_DIRECTORY)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(io_error)?;
            Ok(true)
        }
        Err(error) => Err(io_error(error)),
    }
}
