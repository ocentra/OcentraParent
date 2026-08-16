use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewConfirmationContext;
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_policy_preview_fields::{number_field, string_field};
use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

pub(crate) fn confirmation_context_projection(
    row: &PolicyPreviewStoreRow,
) -> Option<PolicyPreviewConfirmationContext> {
    let context = PolicyPreviewConfirmationContext {
        request_id: request_string(row, constants::policy_control::request::FIELD_REQUEST_ID),
        submission_key: request_string(
            row,
            constants::policy_control::request::FIELD_SUBMISSION_KEY,
        ),
        household_id: request_scope(row).0,
        child_profile_id: source_string(
            row,
            constants::policy_control::source::FIELD_CHILD_PROFILE_ID,
        ),
        device_id: source_string(row, constants::policy_control::source::FIELD_DEVICE_ID),
        source_document_id: source_string(
            row,
            constants::policy_control::source::FIELD_DOCUMENT_ID,
        ),
        policy_version: request_scope(row).1,
        target_reference_id: source_string(
            row,
            constants::policy_control::source::FIELD_TARGET_REFERENCE_ID,
        ),
        rule_id: source_string(row, constants::policy_control::source::FIELD_RULE_ID),
        requested_at: first_string(
            row,
            &[
                constants::field::REQUESTED_AT,
                constants::policy_control::request::FIELD_TIMESTAMP,
            ],
        ),
        expires_at: string_field(&row.fields, constants::field::EXPIRES_AT),
        assistant_preview_id: request_string(
            row,
            constants::policy_control::request::FIELD_ASSISTANT_PREVIEW_ID,
        ),
        audit_reference_ids: actor_context(row).0,
        actor_id: source_string(row, constants::policy_control::source::FIELD_ACTOR_ID),
        actor_role: actor_context(row).1,
        actor_state: actor_context(row).2,
        confirmation_audit_reference_id: first_string(
            row,
            &[
                constants::field::POLICY_AUDIT_REFERENCE_ID,
                constants::policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            ],
        ),
    };

    context_has_any_value(&context).then_some(context)
}

fn request_string(row: &PolicyPreviewStoreRow, key: &str) -> Option<String> {
    string_field(&row.fields, key)
}

fn source_string(row: &PolicyPreviewStoreRow, key: &str) -> Option<String> {
    string_field(&row.fields, key)
}

fn request_scope(row: &PolicyPreviewStoreRow) -> (Option<String>, Option<u64>) {
    (
        first_string(
            row,
            &[
                constants::policy_control::request::FIELD_HOUSEHOLD_ID,
                constants::policy_control::source::FIELD_HOUSEHOLD_ID,
            ],
        ),
        first_number(
            row,
            &[
                constants::policy_control::request::FIELD_POLICY_VERSION,
                constants::policy_control::source::FIELD_POLICY_VERSION,
            ],
        ),
    )
}

fn actor_context(row: &PolicyPreviewStoreRow) -> (Option<String>, Option<String>, Option<String>) {
    (
        first_string(
            row,
            &[
                constants::policy_control::request::FIELD_AUDIT_REFERENCE_IDS,
                constants::policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
            ],
        ),
        first_string(
            row,
            &[
                constants::policy_control::request::FIELD_ACTOR_ROLE,
                constants::policy_control::source::FIELD_ACTOR_ROLE,
            ],
        ),
        first_string(
            row,
            &[
                constants::policy_control::request::FIELD_ACTOR_STATE,
                constants::policy_control::source::FIELD_ACTOR_STATE,
            ],
        ),
    )
}

fn first_string(row: &PolicyPreviewStoreRow, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| string_field(&row.fields, key))
}

fn first_number(row: &PolicyPreviewStoreRow, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| number_field(&row.fields, key))
}

fn context_has_any_value(context: &PolicyPreviewConfirmationContext) -> bool {
    [
        context.request_id.is_some(),
        context.submission_key.is_some(),
        context.household_id.is_some(),
        context.child_profile_id.is_some(),
        context.device_id.is_some(),
        context.source_document_id.is_some(),
        context.policy_version.is_some(),
        context.target_reference_id.is_some(),
        context.rule_id.is_some(),
        context.requested_at.is_some(),
        context.expires_at.is_some(),
        context.assistant_preview_id.is_some(),
        context.audit_reference_ids.is_some(),
        context.actor_id.is_some(),
        context.actor_role.is_some(),
        context.actor_state.is_some(),
        context.confirmation_audit_reference_id.is_some(),
    ]
    .into_iter()
    .any(|value| value)
}
