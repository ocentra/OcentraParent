use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_policy_control_core::policy_delivery::{
    validate_policy_delivery_record, PolicyDeliveryParentVisibleState, PolicyDeliveryRecord,
    PolicyDeliveryState,
};
use ocentra_policy_control_core::policy_request::{
    validate_child_policy_request, ChildPolicyRequest, PolicyApprovalId, PolicyOverrideId,
    PolicyRequestId, PolicyRequestTimestamp, PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::{
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyHouseholdId, PolicyReasonCode,
};
use serde::{Deserialize, Serialize};

const POLICY_CONTROL_NOTIFICATION_ID_PREFIX: &str = "policy-control-notification:";

macro_rules! policy_control_notification_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

policy_control_notification_text_id!(
    PolicyControlNotificationId,
    "policy_control_notification.notification_id"
);

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
    pub household_id: PolicyHouseholdId,
    pub child_profile_id: PolicyChildProfileId,
    pub source_request_id: PolicyRequestId,
    pub source_approval_id: Option<PolicyApprovalId>,
    pub source_override_id: Option<PolicyOverrideId>,
    pub state: PolicyControlNotificationState,
    pub delivery_parent_visible_state: Option<PolicyDeliveryParentVisibleState>,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub reason_code: Option<PolicyReasonCode>,
    pub recorded_at: PolicyRequestTimestamp,
}

pub fn build_policy_control_parent_notification(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
    delivery: Option<&PolicyDeliveryRecord>,
) -> Result<PolicyControlParentNotification, EventingError> {
    validate_child_policy_request(request)?;
    assert_request_override_shape(request, temporary_override)?;
    if let Some(delivery) = delivery {
        validate_policy_delivery_record(delivery)?;
        assert_request_matches_delivery(request, delivery)?;
    }

    let mut audit_reference_ids = request.audit_reference_ids.clone();
    if let Some(temporary_override) = temporary_override {
        extend_unique_audit_refs(
            &mut audit_reference_ids,
            &temporary_override.audit_reference_ids,
        );
    }
    if let Some(delivery) = delivery {
        extend_unique_audit_refs(&mut audit_reference_ids, &delivery.audit_reference_ids);
    }

    Ok(PolicyControlParentNotification {
        notification_id: PolicyControlNotificationId::parse(format!(
            "{}{}",
            POLICY_CONTROL_NOTIFICATION_ID_PREFIX,
            request.request_id.as_str()
        ))?,
        household_id: request.household_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        source_request_id: request.request_id.clone(),
        source_approval_id: request.resolved_approval_id.clone(),
        source_override_id: temporary_override.map(|value| value.override_id.clone()),
        state: notification_state_for(request, delivery)?,
        delivery_parent_visible_state: delivery.map(PolicyDeliveryRecord::parent_visible_state),
        audit_reference_ids,
        reason_code: delivery.and_then(|value| value.reason_code.clone()),
        recorded_at: request
            .resolved_at
            .clone()
            .unwrap_or_else(|| request.requested_at.clone()),
    })
}

fn notification_state_for(
    request: &ChildPolicyRequest,
    delivery: Option<&PolicyDeliveryRecord>,
) -> Result<PolicyControlNotificationState, EventingError> {
    if let Some(delivery) = delivery {
        return match delivery.parent_visible_state() {
            PolicyDeliveryParentVisibleState::Pending => {
                Ok(PolicyControlNotificationState::DeliveryPending)
            }
            PolicyDeliveryParentVisibleState::Applied => {
                Ok(PolicyControlNotificationState::DeliveryApplied)
            }
            PolicyDeliveryParentVisibleState::Degraded => {
                Ok(PolicyControlNotificationState::DeliveryDegraded)
            }
            PolicyDeliveryParentVisibleState::ManualRequired => {
                Ok(PolicyControlNotificationState::DeliveryManualRequired)
            }
            PolicyDeliveryParentVisibleState::Superseded => {
                Ok(PolicyControlNotificationState::DeliverySuperseded)
            }
        };
    }

    match request.status {
        PolicyRequestStatus::PreviewOnly => Ok(PolicyControlNotificationState::PreviewOnly),
        PolicyRequestStatus::PendingParentReview => {
            Ok(PolicyControlNotificationState::PendingParentReview)
        }
        PolicyRequestStatus::Approved => Ok(PolicyControlNotificationState::Approved),
        PolicyRequestStatus::Modified => Ok(PolicyControlNotificationState::Modified),
        PolicyRequestStatus::Denied => Ok(PolicyControlNotificationState::Denied),
        PolicyRequestStatus::Expired => Ok(PolicyControlNotificationState::ExpiredRequest),
        PolicyRequestStatus::ReplayRejected => Err(invalid_request_status_error(
            "policy_request.status",
            request.status,
        )),
    }
}

fn assert_request_override_shape(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
) -> Result<(), EventingError> {
    match request.status {
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified => {
            let temporary_override =
                temporary_override.ok_or_else(|| EventingError::InvalidValue {
                    field: "policy_control_notification.override_id",
                    value: "missing override for resolved request".to_string(),
                })?;

            if temporary_override.source_request_id != request.request_id {
                return Err(EventingError::InvalidValue {
                    field: "policy_control_notification.override_id",
                    value: temporary_override.override_id.as_str().to_string(),
                });
            }
        }
        PolicyRequestStatus::PreviewOnly
        | PolicyRequestStatus::PendingParentReview
        | PolicyRequestStatus::Denied
        | PolicyRequestStatus::Expired => {
            if let Some(temporary_override) = temporary_override {
                return Err(EventingError::InvalidValue {
                    field: "policy_control_notification.override_id",
                    value: temporary_override.override_id.as_str().to_string(),
                });
            }
        }
        PolicyRequestStatus::ReplayRejected => {
            return Err(invalid_request_status_error(
                "policy_request.status",
                request.status,
            ));
        }
    }

    Ok(())
}

fn assert_request_matches_delivery(
    request: &ChildPolicyRequest,
    delivery: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    if !matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified
    ) {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.delivery_state",
            value: request.status.as_protocol_str().to_string(),
        });
    }

    if request.household_id != delivery.household_id
        || request.policy_version != delivery.policy_version
        || request.source_document_id != delivery.source_document_id
        || request.child_profile_id != delivery.target.child_profile_id
    {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.delivery_id",
            value: delivery.delivery_id.as_str().to_string(),
        });
    }

    if let Some(device_id) = &request.device_id {
        if *device_id != delivery.target.device_id {
            return Err(EventingError::InvalidValue {
                field: "policy_control_notification.device_id",
                value: delivery.target.device_id.as_str().to_string(),
            });
        }
    }

    if delivery.state == PolicyDeliveryState::Queued
        || delivery.state == PolicyDeliveryState::Delivering
        || delivery.state == PolicyDeliveryState::Delivered
        || delivery.state == PolicyDeliveryState::Acknowledged
        || delivery.state == PolicyDeliveryState::Applied
        || delivery.state == PolicyDeliveryState::Rejected
        || delivery.state == PolicyDeliveryState::Superseded
        || delivery.state == PolicyDeliveryState::RolledBack
        || delivery.state == PolicyDeliveryState::Degraded
        || delivery.state == PolicyDeliveryState::Offline
        || delivery.state == PolicyDeliveryState::ExpiredBeforeDelivery
        || delivery.state == PolicyDeliveryState::RetryScheduled
        || delivery.state == PolicyDeliveryState::PartialDomainApply
        || delivery.state == PolicyDeliveryState::BlockedByPermission
        || delivery.state == PolicyDeliveryState::BlockedByCapability
        || delivery.state == PolicyDeliveryState::ManualRequired
    {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: "policy_control_notification.delivery_state",
        value: format!("{:?}", delivery.state),
    })
}

fn extend_unique_audit_refs(
    audit_reference_ids: &mut Vec<PolicyAuditReferenceId>,
    additional: &[PolicyAuditReferenceId],
) {
    for audit_reference_id in additional {
        if !audit_reference_ids.contains(audit_reference_id) {
            audit_reference_ids.push(audit_reference_id.clone());
        }
    }
}

fn invalid_request_status_error(field: &'static str, status: PolicyRequestStatus) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: status.as_protocol_str().to_string(),
    }
}
