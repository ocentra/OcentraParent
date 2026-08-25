//! Registry observation accessors.

use super::super::{
    RegistryAncestorObservation, RegistryValue, SecurityDescriptorObservation, WindowsText,
};

impl RegistryValue {
    pub fn value_type(&self) -> u32 {
        self.value_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl RegistryAncestorObservation {
    pub fn path(&self) -> &WindowsText {
        &self.path
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }
}
