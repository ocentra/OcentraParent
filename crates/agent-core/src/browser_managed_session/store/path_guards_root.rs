use std::path::{Path, PathBuf};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard::StablePathGuard;
use super::path_guards_root_component::open_or_create_guard;
use super::path_guards_root_component_parse::append_component;

pub(super) fn ensure_directory_chain(
    path: &Path,
) -> Result<StablePathGuard, BrowserManagedProfileStoreError> {
    let mut current = PathBuf::new();
    let mut guards = Vec::new();
    for component in path.components() {
        if !append_component(&mut current, component)? {
            continue;
        }
        let guard = open_or_create_guard(&current)?;
        guard.validate()?;
        guards.push(guard);
    }
    let root_guard = guards
        .pop()
        .ok_or(BrowserManagedProfileStoreError::UnsafePath)?;
    for guard in guards {
        guard.validate()?;
    }
    root_guard.validate()?;
    Ok(root_guard)
}
