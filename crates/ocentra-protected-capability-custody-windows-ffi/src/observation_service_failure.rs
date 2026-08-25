//! Service failure-action observation accessors.

use super::super::{ServiceFailureAction, ServiceObservation};

impl ServiceFailureAction {
    pub fn action_type(&self) -> i32 {
        self.action_type
    }

    pub fn delay_ms(&self) -> u32 {
        self.delay_ms
    }
}

impl ServiceObservation {
    pub fn failure_actions_reset_period(&self) -> u32 {
        self.failure_actions_reset_period
    }

    pub fn failure_actions_reboot_message(&self) -> Option<&str> {
        self.failure_actions_reboot_message.as_deref()
    }

    pub fn failure_actions_command(&self) -> Option<&str> {
        self.failure_actions_command.as_deref()
    }

    pub fn failure_actions(&self) -> &[ServiceFailureAction] {
        &self.failure_actions
    }

    pub fn failure_actions_on_non_crash_failures(&self) -> bool {
        self.failure_actions_on_non_crash_failures
    }
}
