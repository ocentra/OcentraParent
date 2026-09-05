use super::{AiWorkItem, AiWorkState};
use crate::ai_contracts::AiDurabilityState;

impl AiWorkItem {
    pub fn request(&self) -> &super::AiWorkRequest {
        &self.request
    }

    pub fn state(&self) -> AiWorkState {
        self.state
    }

    pub fn attempt(&self) -> u16 {
        self.attempt
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn last_transition_sequence(&self) -> u64 {
        self.last_transition_sequence
    }
}
