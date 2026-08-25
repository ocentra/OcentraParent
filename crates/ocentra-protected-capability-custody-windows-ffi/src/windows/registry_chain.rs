//! Retained, relative HKLM registry-chain mechanics.

#[path = "registry_chain_security.rs"]
mod security_snapshot;
#[path = "registry_chain_value.rs"]
mod value;

use super::super::handles::{RegistryChainInner, RegistryKeyInner};
use crate::security;
use crate::{
    Error, OwnedRegistryChain, RegistryAncestorObservation, Result, SecurityDescriptorObservation,
    MAX_WIDE_CHARS,
};
use std::ptr;
use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_sys::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY, REG_LINK,
    REG_OPTION_OPEN_LINK, REG_VALUE_TYPE,
};

const ERROR_MORE_DATA: u32 = 234;

impl OwnedRegistryChain {
    pub fn open_hklm(path: &str) -> Result<Self> {
        let components = split_registry_path(path, 64)?;
        Self::open_components(&components)
    }

    fn open_components(components: &[&str]) -> Result<Self> {
        let mut keys = Vec::with_capacity(components.len());
        let mut observations = Vec::with_capacity(components.len());
        let mut parent = HKEY_LOCAL_MACHINE;
        for end in 1..=components.len() {
            let component_wide = wide_string(components[end - 1])?;
            let mut key: HKEY = ptr::null_mut();
            let status = unsafe {
                RegOpenKeyExW(
                    parent,
                    component_wide.as_ptr(),
                    REG_OPTION_OPEN_LINK,
                    KEY_READ | KEY_WOW64_64KEY,
                    &mut key,
                )
            };
            if status != 0 {
                return Err(Error::Win32(status));
            }
            let key = RegistryKeyInner { handle: key };
            reject_symbolic_link(key.handle)?;
            parent = key.handle;
            let security =
                security::copy_descriptor(security_snapshot::query_registry_security(key.handle)?)?;
            let path = components[..end].join("\\");
            keys.push(key);
            observations.push(RegistryAncestorObservation { path, security });
        }
        Ok(Self {
            inner: RegistryChainInner { keys, observations },
        })
    }

    pub fn read_value(&self, name: &str) -> Result<crate::RegistryValue> {
        let key = self
            .inner
            .keys
            .last()
            .ok_or(Error::InvalidInput("registry chain is empty"))?;
        value::read_value_handle(key.handle, name)
    }

    pub fn security(&self) -> Result<SecurityDescriptorObservation> {
        let key = self
            .inner
            .keys
            .last()
            .ok_or(Error::InvalidInput("registry chain is empty"))?;
        security::copy_descriptor(security_snapshot::query_registry_security(key.handle)?)
    }

    pub fn observations(&self) -> Result<&[RegistryAncestorObservation]> {
        Ok(&self.inner.observations)
    }

    pub fn revalidate(&self) -> Result<()> {
        for (key, expected) in self.inner.keys.iter().zip(&self.inner.observations) {
            let current =
                security::copy_descriptor(security_snapshot::query_registry_security(key.handle)?)?;
            if current != expected.security {
                return Err(Error::InvalidInput(
                    "registry ancestor security changed during admission",
                ));
            }
        }
        Ok(())
    }
}

fn split_registry_path(path: &str, max_depth: usize) -> Result<Vec<&str>> {
    let components: Vec<&str> = path
        .split('\\')
        .filter(|component| !component.is_empty())
        .collect();
    if components.is_empty() || components.len() > max_depth {
        return Err(Error::InvalidInput(
            "registry path has invalid ancestor depth",
        ));
    }
    if components
        .iter()
        .any(|component| *component == "." || *component == "..")
    {
        return Err(Error::InvalidInput(
            "registry path contains a traversal component",
        ));
    }
    Ok(components)
}

fn reject_symbolic_link(key: HKEY) -> Result<()> {
    let value_name = wide_string("SymbolicLinkValue")?;
    let mut value_type: REG_VALUE_TYPE = 0;
    let mut length = 0u32;
    let status = unsafe {
        RegQueryValueExW(
            key,
            value_name.as_ptr(),
            ptr::null(),
            &mut value_type,
            ptr::null_mut(),
            &mut length,
        )
    };
    if status == ERROR_FILE_NOT_FOUND {
        return Ok(());
    }
    if status != 0 && status != ERROR_MORE_DATA {
        return Err(Error::Win32(status));
    }
    if value_type == REG_LINK {
        return Err(Error::InvalidInput(
            "registry symbolic links are not accepted for custody",
        ));
    }
    Ok(())
}

fn wide_string(value: &str) -> Result<Vec<u16>> {
    if value.is_empty() || value.contains('\0') {
        return Err(Error::InvalidInput(
            "Windows string is empty or contains NUL",
        ));
    }
    if value.encode_utf16().count() >= MAX_WIDE_CHARS {
        return Err(Error::BufferTooLarge);
    }
    Ok(value.encode_utf16().chain(core::iter::once(0)).collect())
}
