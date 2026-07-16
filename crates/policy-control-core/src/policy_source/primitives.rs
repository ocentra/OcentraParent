#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

mod document_actor;
mod schedule_reason;
mod target_rule;
mod version;

fn parse_text_id(value: impl Into<String>, field: &'static str) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}

pub type ParentPolicyDocumentId = document_actor::ParentPolicyDocumentId;
pub type PolicyHouseholdId = document_actor::PolicyHouseholdId;
pub type PolicyActorId = document_actor::PolicyActorId;

pub type PolicyChildProfileId = target_rule::PolicyChildProfileId;
pub type PolicyDeviceId = target_rule::PolicyDeviceId;
pub type PolicyRuleId = target_rule::PolicyRuleId;
pub type PolicyTargetReferenceId = target_rule::PolicyTargetReferenceId;

pub type PolicyScheduleId = schedule_reason::PolicyScheduleId;
pub type PolicyTimezoneName = schedule_reason::PolicyTimezoneName;
pub type PolicyReasonCode = schedule_reason::PolicyReasonCode;
pub type PolicyAuditReferenceId = schedule_reason::PolicyAuditReferenceId;

pub type PolicyVersion = version::PolicyVersion;
