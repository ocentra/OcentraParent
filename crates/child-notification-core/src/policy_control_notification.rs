use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord;
use ocentra_policy_control_core::policy_request::{
    ChildPolicyRequest, PolicyApprovalId, PolicyOverrideId, PolicyRequestId,
    PolicyRequestTimestamp, PolicyTemporaryOverride,
};
use serde::{Deserialize, Serialize};

#[path = "policy_control_notification_audit.rs"]
mod policy_control_notification_audit;
#[path = "policy_control_notification_delivery_state.rs"]
mod policy_control_notification_delivery_state;
#[path = "policy_control_notification_id.rs"]
mod policy_control_notification_id;
#[path = "policy_control_notification_request_state.rs"]
mod policy_control_notification_request_state;
#[path = "policy_control_notification_state.rs"]
mod policy_control_notification_state;
#[path = "policy_control_notification_validation.rs"]
mod policy_control_notification_validation;

use policy_control_notification_audit::{build_audit_reference_ids, recorded_at_for};
use policy_control_notification_id::PolicyControlNotificationId;
use policy_control_notification_state::notification_state_for;
use policy_control_notification_validation::{
    assert_request_matches_delivery, assert_request_override_shape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyControlNotificationState {
    #[serde(rename = "preview-only")]
    PreviewOnly,
    #[serde(rename = "pending-parent-review")]
    PendingParentReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "expired-request")]
    ExpiredRequest,
    #[serde(rename = "delivery-pending")]
    DeliveryPending,
    #[serde(rename = "delivery-applied")]
    DeliveryApplied,
    #[serde(rename = "delivery-degraded")]
    DeliveryDegraded,
    #[serde(rename = "delivery-manual-required")]
    DeliveryManualRequired,
    #[serde(rename = "delivery-superseded")]
    DeliverySuperseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyControlParentNotification {
    pub notification_id: PolicyControlNotificationId,
    pub household_id: ocentra_policy_control_core::policy_source::PolicyHouseholdId,
    pub child_profile_id: ocentra_policy_control_core::policy_source::PolicyChildProfileId,
    pub source_request_id: PolicyRequestId,
    pub source_approval_id: Option<PolicyApprovalId>,
    pub source_override_id: Option<PolicyOverrideId>,
    pub state: PolicyControlNotificationState,
    pub delivery_parent_visible_state:
        Option<ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState>,
    pub audit_reference_ids:
        Vec<ocentra_policy_control_core::policy_source::PolicyAuditReferenceId>,
    pub reason_code: Option<ocentra_policy_control_core::policy_source::PolicyReasonCode>,
    pub recorded_at: PolicyRequestTimestamp,
}

pub fn build_policy_control_parent_notification(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
    delivery: Option<&PolicyDeliveryRecord>,
) -> Result<PolicyControlParentNotification, EventingError> {
    assert_request_override_shape(request, temporary_override)?;
    delivery.map_or(Ok(()), |delivery| {
        assert_request_matches_delivery(request, delivery)
    })?;

    Ok(PolicyControlParentNotification {
        notification_id: PolicyControlNotificationId::parse(format!(
            "{}{}",
            "policy-control-notification:",
            request.request_id.as_str()
        ))?,
        household_id: request.household_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        source_request_id: request.request_id.clone(),
        source_approval_id: request.resolved_approval_id.clone(),
        source_override_id: temporary_override.map(|value| value.override_id.clone()),
        state: notification_state_for(request, delivery)?,
        delivery_parent_visible_state: delivery.map(PolicyDeliveryRecord::parent_visible_state),
        audit_reference_ids: build_audit_reference_ids(request, temporary_override, delivery),
        reason_code: delivery.and_then(|value| value.reason_code.clone()),
        recorded_at: recorded_at_for(request),
    })
}
