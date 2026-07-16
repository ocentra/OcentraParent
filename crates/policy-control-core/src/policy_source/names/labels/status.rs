#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

mod active;
mod terminal;

pub(super) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    if let Some(active_status) = active::status_slot(status) {
        active::policy_status_name(active_status)
    } else if let Some(terminal_status) = terminal::status_slot(status) {
        terminal::policy_status_name(terminal_status)
    } else {
        policy_control::source::STATUS_MANUAL_REQUIRED
    }
}
