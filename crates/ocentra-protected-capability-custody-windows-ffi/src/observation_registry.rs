//! Registry observation accessors.

use super::super::{RegistryAncestorObservation, RegistryValue, SecurityDescriptorObservation};

impl RegistryValue {
    pub fn value_type(&self) -> u32 {
        self.value_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl RegistryAncestorObservation {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }
}
