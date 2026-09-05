use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserCustodyLabel, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedSessionStatus, BrowserManagedState, BrowserQueryVisibilityLabel,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BrowserRuntimeText(String);

impl<T> From<T> for BrowserRuntimeText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

pub(crate) fn status_with_error(
    checked_at: impl Into<BrowserRuntimeText>,
    reason: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let reason = reason.into();
    let mut status = base_managed_status(checked_at);
    status.managed_state = BrowserManagedState::Error;
    status.capability_status = BrowserCapabilityStatus::AdapterError;
    status.degraded_reason = Some(reason.0);
    status
}

fn base_managed_status(checked_at: impl Into<BrowserRuntimeText>) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.into();
    BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: checked_at.0,
        managed_browser_session_id: None,
        browser_family: None,
        browser_channel: None,
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        profile_root_ref: None,
        profile_scope_id: None,
        profile_lifecycle_state: None,
        policy_revision: None,
        process_id: None,
        bridge_kind: None,
        bridge_endpoint_ref: None,
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::Error,
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::Unavailable,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    }
}
