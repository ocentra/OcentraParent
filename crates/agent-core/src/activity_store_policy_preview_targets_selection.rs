use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

pub(crate) fn target_from_row(row: &PolicyPreviewStoreRow) -> Option<PolicyTarget> {
    let (target_type, target_value) =
        target_type_and_value(row.subject_kind.as_str(), &row.fields, row)?;
    Some(PolicyTarget {
        target_id: row.subject_id.clone(),
        target_type,
        target_value,
    })
}

fn target_type_and_value(
    subject_kind: &str,
    fields: &LogFields,
    row: &PolicyPreviewStoreRow,
) -> Option<(PolicyTargetType, String)> {
    explicit_target_type_and_value(fields)
        .or_else(|| target_type_and_value_for_subject_kind(subject_kind, fields, row))
}

fn target_type_and_value_for_subject_kind(
    subject_kind: &str,
    fields: &LogFields,
    row: &PolicyPreviewStoreRow,
) -> Option<(PolicyTargetType, String)> {
    match subject_kind {
        constants::activity_subject_kind::PROCESS => Some((
            PolicyTargetType::Process,
            field_or_subject_value(fields, constants::field::PROCESS_NAME, row),
        )),
        constants::activity_subject_kind::WINDOW => Some((
            PolicyTargetType::Window,
            field_or_subject_value(fields, constants::field::WINDOW_TITLE, row),
        )),
        constants::activity_subject_kind::DOMAIN => Some((
            PolicyTargetType::Domain,
            domain_or_subject_value(fields, row),
        )),
        constants::activity_subject_kind::URL => url_target(fields, row),
        constants::activity_subject_kind::VIDEO => {
            Some((PolicyTargetType::Video, subject_value(row)))
        }
        constants::activity_subject_kind::DEVICE => {
            Some((PolicyTargetType::Device, subject_value(row)))
        }
        _ => None,
    }
}

fn explicit_target_type_and_value(fields: &LogFields) -> Option<(PolicyTargetType, String)> {
    let target_type = string_field(fields, constants::field::POLICY_TARGET_TYPE)
        .and_then(|value| PolicyTargetType::from_protocol_str(&value))?;
    let target_value = string_field(fields, constants::field::POLICY_TARGET_VALUE)?;
    Some((target_type, target_value))
}

fn url_target(
    fields: &LogFields,
    row: &PolicyPreviewStoreRow,
) -> Option<(PolicyTargetType, String)> {
    if let Some(domain) = string_field(fields, constants::field::DOMAIN) {
        return Some((PolicyTargetType::Domain, domain));
    }

    Some((
        PolicyTargetType::Site,
        string_field(fields, constants::field::URL).unwrap_or_else(|| subject_value(row)),
    ))
}

fn domain_or_subject_value(fields: &LogFields, row: &PolicyPreviewStoreRow) -> String {
    string_field(fields, constants::field::DESTINATION_DOMAIN)
        .or_else(|| string_field(fields, constants::field::DOMAIN))
        .unwrap_or_else(|| subject_value(row))
}

fn field_or_subject_value(fields: &LogFields, key: &str, row: &PolicyPreviewStoreRow) -> String {
    string_field(fields, key).unwrap_or_else(|| subject_value(row))
}

fn subject_value(row: &PolicyPreviewStoreRow) -> String {
    row.subject_display_name
        .clone()
        .unwrap_or_else(|| row.subject_id.clone())
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
