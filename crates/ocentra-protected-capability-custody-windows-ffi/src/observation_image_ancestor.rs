//! Pinned executable ancestor observation accessors.

use super::super::{
    ImageAncestorObservation, ImageIdentity, SecurityDescriptorObservation, WindowsText,
};

impl ImageAncestorObservation {
    pub fn path(&self) -> &WindowsText {
        &self.path
    }

    pub fn identity(&self) -> ImageIdentity {
        self.identity
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }

    pub fn file_attributes(&self) -> u32 {
        self.file_attributes
    }

    pub fn reparse_tag(&self) -> u32 {
        self.reparse_tag
    }
}
