#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;

mod active;
mod terminal;

pub(super) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    match status {
        PolicySourceStatus::Draft
        | PolicySourceStatus::Preview
        | PolicySourceStatus::Confirmed
        | PolicySourceStatus::Queued
        | PolicySourceStatus::Delivered
        | PolicySourceStatus::Acknowledged
        | PolicySourceStatus::Active => active::policy_status_name(status),
        PolicySourceStatus::PartiallyActive
        | PolicySourceStatus::Rejected
        | PolicySourceStatus::Superseded
        | PolicySourceStatus::RolledBack
        | PolicySourceStatus::Stale
        | PolicySourceStatus::Expired
        | PolicySourceStatus::ManualRequired => terminal::policy_status_name(status),
    }
}
