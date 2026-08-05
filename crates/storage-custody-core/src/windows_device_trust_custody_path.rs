use std::{fs, path::Path};

use super::Error;

pub(super) fn validate_seal_material(material: &[u8]) -> Result<(), Error> {
    (!material.is_empty()).then_some(()).ok_or(Error::Invalid)
}

pub(super) fn sealed_content_present(root: &Path) -> Result<bool, Error> {
    for entry in fs::read_dir(root).map_err(|_error| Error::Io)? {
        let entry = entry.map_err(|_error| Error::Io)?;
        let file_type = entry.file_type().map_err(|_error| Error::Io)?;
        if file_type.is_file()
            && !file_type.is_symlink()
            && entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "sealed")
        {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(windows)]
pub(super) fn validate_custody_root_and_ancestors(root: &Path) -> Result<(), Error> {
    for ancestor in root.ancestors() {
        let metadata = match fs::symlink_metadata(ancestor) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_error) => return Err(Error::Io),
        };
        if !metadata.is_dir() || path_entry_is_reparse_point(&metadata) {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn path_entry_is_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}
