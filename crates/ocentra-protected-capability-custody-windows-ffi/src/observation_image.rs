//! Pinned executable observation accessors.

use super::super::{
    ImageAncestorObservation, ImageIdentity, ImageObservation, SecurityDescriptorObservation,
    WindowsText,
};

impl ImageIdentity {
    pub fn volume_serial_number(&self) -> u64 {
        self.volume_serial_number
    }

    pub fn file_id(&self) -> &[u8; 16] {
        &self.file_id
    }
}

impl ImageObservation {
    pub fn path(&self) -> &WindowsText {
        &self.path
    }

    pub fn identity(&self) -> ImageIdentity {
        self.identity
    }

    pub fn sha256(&self) -> &[u8; 32] {
        &self.sha256
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }

    pub fn ancestors(&self) -> &[ImageAncestorObservation] {
        &self.ancestors
    }

    pub fn file_attributes(&self) -> u32 {
        self.file_attributes
    }

    pub fn reparse_tag(&self) -> u32 {
        self.reparse_tag
    }
}
