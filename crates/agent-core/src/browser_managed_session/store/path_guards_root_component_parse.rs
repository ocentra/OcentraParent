use std::path::{Component, PathBuf};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn append_component(
    current: &mut PathBuf,
    component: Component<'_>,
) -> Result<bool, BrowserManagedProfileStoreError> {
    match component {
        Component::Prefix(prefix) => {
            current.push(prefix.as_os_str());
            Ok(false)
        }
        Component::RootDir => {
            current.push(component.as_os_str());
            Ok(true)
        }
        Component::CurDir => Ok(false),
        Component::Normal(name) => {
            current.push(name);
            Ok(true)
        }
        Component::ParentDir => Err(BrowserManagedProfileStoreError::UnsafePath),
    }
}
