use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

pub(crate) fn target_aliases_from_row(
    row: &PolicyPreviewStoreRow,
    primary: &PolicyTarget,
) -> Vec<PolicyTarget> {
    let mut aliases = Vec::new();
    match row.subject_kind.as_str() {
        constants::activity_subject_kind::PROCESS => push_target_alias(
            &mut aliases,
            primary,
            row,
            PolicyTargetType::App,
            string_field(&row.fields, constants::field::PROCESS_NAME),
        ),
        constants::activity_subject_kind::WINDOW => {
            push_target_alias(
                &mut aliases,
                primary,
                row,
                PolicyTargetType::App,
                string_field(&row.fields, constants::field::APP_NAME),
            );
            push_target_alias(
                &mut aliases,
                primary,
                row,
                PolicyTargetType::Process,
                string_field(&row.fields, constants::field::APP_NAME),
            );
        }
        constants::activity_subject_kind::DOMAIN => push_target_alias(
            &mut aliases,
            primary,
            row,
            PolicyTargetType::Process,
            string_field(&row.fields, constants::field::PROCESS_NAME),
        ),
        constants::activity_subject_kind::URL => push_target_alias(
            &mut aliases,
            primary,
            row,
            PolicyTargetType::Site,
            string_field(&row.fields, constants::field::URL),
        ),
        constants::activity_subject_kind::DEVICE => push_target_alias(
            &mut aliases,
            primary,
            row,
            PolicyTargetType::Category,
            string_field(&row.fields, constants::field::SCREEN_PRIMARY_CATEGORY),
        ),
        _ => {}
    }
    aliases
}

fn push_target_alias(
    aliases: &mut Vec<PolicyTarget>,
    primary: &PolicyTarget,
    row: &PolicyPreviewStoreRow,
    target_type: PolicyTargetType,
    target_value: Option<String>,
) {
    if let Some(target_value) = target_value {
        let target = PolicyTarget {
            target_id: row.subject_id.clone(),
            target_type,
            target_value,
        };
        if !same_target(&target, primary)
            && !aliases.iter().any(|alias| same_target(alias, &target))
        {
            aliases.push(target);
        }
    }
}

fn same_target(left: &PolicyTarget, right: &PolicyTarget) -> bool {
    left.target_type == right.target_type && left.target_value == right.target_value
}

fn string_field(
    fields: &ocentra_parent_agent_protocol::logging::LogFields,
    key: &str,
) -> Option<String> {
    match fields.get(key) {
        Some(ocentra_parent_agent_protocol::logging::LogFieldValue::String(value)) => {
            Some(value.clone())
        }
        _ => None,
    }
}
