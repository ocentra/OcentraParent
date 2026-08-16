use ocentra_parent_agent_protocol::activity::policy_preview::{
    policy_preview_finding_kinds_csv, PolicyPreviewFindingKind, PolicyPreviewTargetState,
};
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_policy_preview_fields::string_field;
use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

const TARGET_STATE_RULES: &[(&[&str], PolicyPreviewTargetState)] = &[
    (
        &[
            constants::browser::CAPABILITY_STATUS_STALE,
            constants::tracking_runtime::CAPABILITY_STATUS_STALE,
            APP_GAME_CAPABILITY_STATUS_STALE,
        ],
        PolicyPreviewTargetState::Stale,
    ),
    (
        &[constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY],
        PolicyPreviewTargetState::Offline,
    ),
    (
        &[
            constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
            APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
        ],
        PolicyPreviewTargetState::Unsupported,
    ),
    (
        &[
            constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
            constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING,
            constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
            constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
            constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
            constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        ],
        PolicyPreviewTargetState::ManualRequired,
    ),
];

const FINDING_KIND_RULES: &[(PolicyPreviewTargetState, PolicyPreviewFindingKind)] = &[
    (
        PolicyPreviewTargetState::Unsupported,
        PolicyPreviewFindingKind::UnsupportedTarget,
    ),
    (
        PolicyPreviewTargetState::ManualRequired,
        PolicyPreviewFindingKind::ManualRequiredTarget,
    ),
    (
        PolicyPreviewTargetState::Offline,
        PolicyPreviewFindingKind::OfflineTarget,
    ),
    (
        PolicyPreviewTargetState::Stale,
        PolicyPreviewFindingKind::StaleTarget,
    ),
];

pub(crate) fn target_state_from_row(
    row: &PolicyPreviewStoreRow,
) -> Option<PolicyPreviewTargetState> {
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS)?;
    TARGET_STATE_RULES.iter().find_map(|(statuses, state)| {
        statuses
            .contains(&capability_status.as_str())
            .then_some(*state)
    })
}

pub(crate) fn target_explanation_code_from_row(
    row: &PolicyPreviewStoreRow,
    target_state: Option<PolicyPreviewTargetState>,
) -> Option<String> {
    target_state.and_then(|_| {
        string_field(&row.fields, constants::field::DEGRADED_REASON)
            .or_else(|| string_field(&row.fields, constants::field::CAPABILITY_STATUS))
    })
}

pub(crate) fn target_finding_kinds(
    target_state: Option<PolicyPreviewTargetState>,
) -> Option<String> {
    target_state.and_then(|target_state| {
        FINDING_KIND_RULES
            .iter()
            .find_map(|(candidate, kind)| (*candidate == target_state).then_some(*kind))
            .and_then(|kind| policy_preview_finding_kinds_csv(&[kind]))
    })
}
