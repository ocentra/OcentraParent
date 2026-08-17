use std::{
    fs, io,
    path::{Component, Path},
};

pub(crate) fn delete_local_file(root: &Path, relative_path: &str) -> io::Result<()> {
    let relative = validate_relative_path(relative_path)?;
    let target = root.join(relative);
    reject_symlink_components(root, relative)?;
    ensure_target_inside_root(root, &target)?;
    let metadata = fs::symlink_metadata(&target)?;
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local custody delete only handles an explicit regular file",
        ));
    }
    fs::remove_file(&target)?;
    if let Some(parent) = target.parent() {
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

fn validate_relative_path(relative_path: &str) -> io::Result<&Path> {
    let relative = Path::new(relative_path);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local custody target must be a relative non-traversing path",
        ));
    }
    Ok(relative)
}

fn reject_symlink_components(root: &Path, relative: &Path) -> io::Result<()> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(component) = component else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local custody target contains an invalid path component",
            ));
        };
        current.push(component);
        if fs::symlink_metadata(&current)?.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local custody target or ancestor must not be a symlink",
            ));
        }
    }
    Ok(())
}

fn ensure_target_inside_root(root: &Path, target: &Path) -> io::Result<()> {
    let canonical_root = root.canonicalize()?;
    let canonical_target = target.canonicalize()?;
    if !canonical_target.starts_with(&canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local custody target escapes the child data root",
        ));
    }
    Ok(())
}
