use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus, PolicySourceStatus,
    PolicySourceSurface,
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_policy_preview_fields::{protocol_field, string_field};
use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

pub(crate) struct PolicyLifecycleProjection {
    pub policy_source_status: Option<PolicySourceStatus>,
    pub policy_source_surface: Option<PolicySourceSurface>,
    pub policy_request_origin: Option<PolicyRequestOrigin>,
    pub policy_assistant_confirmation_state: Option<PolicyAssistantConfirmationState>,
    pub policy_request_status: Option<PolicyRequestStatus>,
    pub policy_approval_id: Option<String>,
    pub policy_override_id: Option<String>,
    pub policy_replay_of_approval_id: Option<String>,
    pub policy_reviewed_by_actor_id: Option<String>,
    pub policy_reviewed_by_actor_role: Option<String>,
    pub policy_reviewed_at: Option<String>,
    pub policy_audit_reference_id: Option<String>,
}

pub(crate) fn policy_lifecycle_projection_from_row(
    row: &PolicyPreviewStoreRow,
) -> PolicyLifecycleProjection {
    PolicyLifecycleProjection {
        policy_source_status: protocol_field(
            &row.fields,
            constants::field::POLICY_SOURCE_STATUS,
            |value| PolicySourceStatus::from_protocol_str(value),
        ),
        policy_source_surface: protocol_field(
            &row.fields,
            constants::field::POLICY_SOURCE_SURFACE,
            |value| PolicySourceSurface::from_protocol_str(value),
        ),
        policy_request_origin: protocol_field(
            &row.fields,
            constants::field::POLICY_REQUEST_ORIGIN,
            |value| PolicyRequestOrigin::from_protocol_str(value),
        ),
        policy_assistant_confirmation_state: protocol_field(
            &row.fields,
            constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE,
            |value| PolicyAssistantConfirmationState::from_protocol_str(value),
        ),
        policy_request_status: protocol_field(
            &row.fields,
            constants::field::POLICY_REQUEST_STATUS,
            |value| PolicyRequestStatus::from_protocol_str(value),
        ),
        policy_approval_id: string_field(&row.fields, constants::field::POLICY_APPROVAL_ID),
        policy_override_id: string_field(&row.fields, constants::field::POLICY_OVERRIDE_ID),
        policy_replay_of_approval_id: string_field(
            &row.fields,
            constants::field::POLICY_REPLAY_OF_APPROVAL_ID,
        ),
        policy_reviewed_by_actor_id: string_field(
            &row.fields,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ID,
        ),
        policy_reviewed_by_actor_role: string_field(
            &row.fields,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE,
        ),
        policy_reviewed_at: string_field(&row.fields, constants::field::POLICY_REVIEWED_AT),
        policy_audit_reference_id: string_field(
            &row.fields,
            constants::field::POLICY_AUDIT_REFERENCE_ID,
        ),
    }
}
