//! Process observation accessors.

use super::super::{ImageObservation, ProcessObservation};

impl ProcessObservation {
    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn creation_time_100ns(&self) -> u64 {
        self.creation_time_100ns
    }

    pub fn image(&self) -> &ImageObservation {
        &self.image
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}
