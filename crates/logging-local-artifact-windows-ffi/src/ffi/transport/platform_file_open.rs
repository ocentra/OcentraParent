use super::*;

impl OwnedFile {
    pub(crate) fn open_directory(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::Directory, false)
    }

    pub(crate) fn open_sync_directory(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::SyncDirectory, false)
    }

    pub(crate) fn open_rename_directory(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::RenameDirectory, false)
    }

    pub(crate) fn open_mutation_directory(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::MutationDirectory, false)
    }

    pub(crate) fn open_existing_file(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::File { consistent: false }, false)
    }

    pub(crate) fn open_process_image_file(path: &Path) -> Result<Self, ArtifactError> {
        let file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(io_error)?;
        let owned = Self {
            file,
            path: path.to_owned(),
        };
        verify_process_image_metadata(&owned)?;
        Ok(owned)
    }

    pub(crate) fn open_existing_mutation_file(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::File { consistent: true }, false)
    }

    pub(crate) fn create_new_file(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::File { consistent: true }, true)
    }

    pub(crate) fn create_new_mutation_file(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::File { consistent: true }, true)
    }

    pub(crate) fn create_new_replace_file(path: &Path) -> Result<Self, ArtifactError> {
        Self::open(path, OpenKind::StagingFile, true)
    }

    pub(crate) fn open_lock_file(path: &Path) -> Result<Self, ArtifactError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(io_error)?;
        let owned = Self {
            file,
            path: path.to_owned(),
        };
        verify_metadata(&owned, false)?;
        Ok(owned)
    }

    pub(crate) fn open_existing_lock_file(path: &Path) -> Result<Self, ArtifactError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(io_error)?;
        let owned = Self {
            file,
            path: path.to_owned(),
        };
        verify_metadata(&owned, false)?;
        Ok(owned)
    }

    fn open(path: &Path, kind: OpenKind, create_new: bool) -> Result<Self, ArtifactError> {
        let mut options = OpenOptions::new();
        let access_mode = match kind {
            OpenKind::File { .. } | OpenKind::StagingFile | OpenKind::MutationDirectory => {
                FILE_GENERIC_READ | FILE_GENERIC_WRITE | DELETE
            }
            OpenKind::Directory => FILE_GENERIC_READ,
            OpenKind::SyncDirectory | OpenKind::RenameDirectory => {
                FILE_GENERIC_READ | FILE_GENERIC_WRITE
            }
        };
        options
            .read(true)
            .write(true)
            .access_mode(access_mode)
            .share_mode(match kind {
                OpenKind::File { consistent: true } => FILE_SHARE_READ,
                OpenKind::StagingFile => FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                OpenKind::File { consistent: false }
                | OpenKind::Directory
                | OpenKind::SyncDirectory
                | OpenKind::MutationDirectory => FILE_SHARE_READ | FILE_SHARE_WRITE,
                OpenKind::RenameDirectory => FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            });
        let mut flags = FILE_FLAG_OPEN_REPARSE_POINT;
        if matches!(
            kind,
            OpenKind::Directory
                | OpenKind::SyncDirectory
                | OpenKind::RenameDirectory
                | OpenKind::MutationDirectory
        ) {
            flags |= FILE_FLAG_BACKUP_SEMANTICS;
        }
        options.custom_flags(flags);
        options.create_new(create_new);
        let file = options.open(path).map_err(io_error)?;
        let owned = Self {
            file,
            path: path.to_owned(),
        };
        verify_metadata(
            &owned,
            matches!(
                kind,
                OpenKind::Directory
                    | OpenKind::SyncDirectory
                    | OpenKind::RenameDirectory
                    | OpenKind::MutationDirectory
            ),
        )?;
        Ok(owned)
    }
}
