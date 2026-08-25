//! Retained registry-value observation accessors.

use super::super::{RegistryValue, RegistryValueName, RegistryValueObservation};

impl RegistryValueObservation {
    pub fn name(&self) -> &RegistryValueName {
        &self.name
    }

    pub fn value(&self) -> &RegistryValue {
        &self.value
    }
}
