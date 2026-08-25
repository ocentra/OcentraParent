//! Pinned executable observation accessors.

use super::super::{ImageIdentity, ImageObservation};

impl ImageIdentity {
    pub fn volume_serial_number(&self) -> u64 {
        self.volume_serial_number
    }

    pub fn file_id(&self) -> &[u8; 16] {
        &self.file_id
    }
}

impl ImageObservation {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn identity(&self) -> ImageIdentity {
        self.identity
    }

    pub fn sha256(&self) -> &[u8; 32] {
        &self.sha256
    }
}
