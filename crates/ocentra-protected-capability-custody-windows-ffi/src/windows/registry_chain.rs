//! Retained, relative HKLM registry-chain mechanics.

#[path = "registry_chain_security.rs"]
mod security_snapshot;
#[path = "registry_chain_value.rs"]
mod value;

use super::super::handles::{RegistryChainInner, RegistryKeyInner};
use crate::security;
use crate::{
    Error, InputFault, OwnedRegistryChain, RegistryAncestorObservation, RegistryPath,
    RegistryValueName, RegistryValueObservation, Result, SecurityDescriptorObservation,
    WindowsText,
};
use std::ptr;
use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_sys::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY, REG_LINK,
    REG_OPTION_OPEN_LINK, REG_VALUE_TYPE,
};

const ERROR_MORE_DATA: u32 = 234;

impl OwnedRegistryChain {
    pub fn open_hklm(path: &RegistryPath) -> Result<Self> {
        let components = path.components()?;
        Self::open_components(&components)
    }

    fn open_components(components: &[WindowsText]) -> Result<Self> {
        let mut keys = Vec::with_capacity(components.len());
        let mut observations = Vec::with_capacity(components.len());
        let mut parent = HKEY_LOCAL_MACHINE;
        for end in 1..=components.len() {
            let component_wide = components[end - 1].wide_nul()?;
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
            let path = WindowsText::join(&components[..end])?;
            keys.push(key);
            observations.push(RegistryAncestorObservation { path, security });
        }
        Ok(Self {
            inner: RegistryChainInner { keys, observations },
        })
    }

    pub fn observe_value(&self, name: &RegistryValueName) -> Result<RegistryValueObservation> {
        let key = self
            .inner
            .keys
            .last()
            .ok_or(Error::InvalidInput(InputFault::RegistryChainEmpty))?;
        let value = value::read_value_handle(key.handle, name)?;
        Ok(RegistryValueObservation {
            name: name.clone(),
            value,
        })
    }

    pub fn reobserve_value(
        &self,
        previous: &RegistryValueObservation,
    ) -> Result<RegistryValueObservation> {
        self.observe_value(&previous.name)
    }

    pub fn security(&self) -> Result<SecurityDescriptorObservation> {
        let key = self
            .inner
            .keys
            .last()
            .ok_or(Error::InvalidInput(InputFault::RegistryChainEmpty))?;
        security::copy_descriptor(security_snapshot::query_registry_security(key.handle)?)
    }

    pub fn observations(&self) -> Result<&[RegistryAncestorObservation]> {
        Ok(&self.inner.observations)
    }

    pub fn revalidate(&self) -> Result<()> {
        for (key, expected) in self.inner.keys.iter().zip(&self.inner.observations) {
            reject_symbolic_link(key.handle)?;
            let current =
                security::copy_descriptor(security_snapshot::query_registry_security(key.handle)?)?;
            if current != expected.security {
                return Err(Error::InvalidInput(
                    InputFault::RegistryAncestorSecurityChanged,
                ));
            }
        }
        Ok(())
    }
}

fn reject_symbolic_link(key: HKEY) -> Result<()> {
    const SYMBOLIC_LINK_VALUE: [u16; 18] = [
        83, 121, 109, 98, 111, 108, 105, 99, 76, 105, 110, 107, 86, 97, 108, 117, 101, 0,
    ];
    let mut value_type: REG_VALUE_TYPE = 0;
    let mut length = 0u32;
    let status = unsafe {
        RegQueryValueExW(
            key,
            SYMBOLIC_LINK_VALUE.as_ptr(),
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
        return Err(Error::InvalidInput(InputFault::RegistrySymbolicLink));
    }
    Ok(())
}
