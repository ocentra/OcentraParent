//! Security-descriptor and ACE observation accessors.

use super::super::{AceObservation, SecurityDescriptorObservation};

impl AceObservation {
    pub fn ace_type(&self) -> u8 {
        self.ace_type
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn access_mask(&self) -> u32 {
        self.access_mask
    }

    pub fn sid(&self) -> &[u8] {
        &self.sid
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

impl SecurityDescriptorObservation {
    pub fn descriptor(&self) -> &[u8] {
        &self.descriptor
    }

    pub fn owner_sid(&self) -> &[u8] {
        &self.owner_sid
    }

    pub fn owner_was_defaulted(&self) -> bool {
        self.owner_defaulted
    }

    pub fn dacl_is_present(&self) -> bool {
        self.dacl_present
    }

    pub fn dacl_was_defaulted(&self) -> bool {
        self.dacl_defaulted
    }

    pub fn dacl(&self) -> &[AceObservation] {
        &self.dacl
    }

    pub fn dacl_is_protected(&self) -> bool {
        self.dacl_protected
    }
}
