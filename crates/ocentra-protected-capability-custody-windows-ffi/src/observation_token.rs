//! Token observation accessors.

use super::super::TokenObservation;

impl TokenObservation {
    pub fn sid(&self) -> &[u8] {
        &self.sid
    }

    pub fn integrity_level(&self) -> u32 {
        self.integrity_level
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }
}
