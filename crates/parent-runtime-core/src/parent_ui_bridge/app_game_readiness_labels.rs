use super::*;

const APP_GAME_POLICY_KIND_LABELS: &[(&str, &str)] = &[
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
        "Policy evidence",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
        "Approval authority",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT,
        "Approval action result",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
        "Platform authority",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
        "AI classifier context",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE,
        "Category candidate",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_CATEGORY_RISK_ROUTING,
        "Category/risk routing",
    ),
    (
        app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW,
        "Unknown review",
    ),
];
const APP_GAME_PARENT_ACCESS_LABELS: &[(&str, ParentPortalParentAccessState)] = &[
    (
        "active-controller",
        ParentPortalParentAccessState::ActiveController,
    ),
    ("observer", ParentPortalParentAccessState::ObserverOnly),
    (
        "unauthenticated",
        ParentPortalParentAccessState::Unauthenticated,
    ),
];

pub(super) fn app_game_notification_load_state(
    read_model: &AppGameNotificationReadinessReadModel,
) -> String {
    if read_model.ready_intent_count > 0 {
        "ready".to_string()
    } else if read_model.manual_required_count > 0 {
        "warn".to_string()
    } else {
        "unavailable".to_string()
    }
}

pub(super) fn app_game_policy_load_state(read_model: &AppGamePolicyReadinessReadModel) -> String {
    if read_model.policy_evaluation_ready && !read_model.manual_review_required {
        "ready".to_string()
    } else if read_model.manual_review_required {
        "warn".to_string()
    } else {
        "unavailable".to_string()
    }
}

pub(super) fn app_game_claimed_value(value: bool) -> String {
    if value {
        "ready".to_string()
    } else {
        "not-claimed".to_string()
    }
}

pub(super) fn app_game_ready_warn_value(value: bool) -> String {
    if value {
        "ready".to_string()
    } else {
        "warn".to_string()
    }
}

pub(super) fn app_game_manual_required_value(value: bool) -> String {
    if value {
        "manual-required".to_string()
    } else {
        "false".to_string()
    }
}

pub(super) fn app_game_join_strings(values: &[String]) -> String {
    if values.is_empty() {
        "Not reported".to_string()
    } else {
        values.join(" | ")
    }
}

pub(super) fn app_game_join_policy_refs(row: &AppGamePolicyReadinessRow) -> String {
    let mut refs = row.evidence_reference_ids.clone();
    refs.extend(
        row.evidence
            .iter()
            .map(|evidence| evidence.evidence_id.clone()),
    );
    app_game_join_strings(&refs)
}

pub(super) fn app_game_join_notification_refs(row: &AppGameNotificationReadinessRow) -> String {
    let mut refs = row.evidence_reference_ids.clone();
    refs.push(row.minimal_payload_ref.clone());
    refs.extend(
        row.evidence
            .iter()
            .map(|evidence| evidence.evidence_id.clone()),
    );
    app_game_join_strings(&refs)
}

pub(super) fn app_game_policy_kind_label(kind: &str) -> String {
    APP_GAME_POLICY_KIND_LABELS
        .iter()
        .find(|(raw, _)| *raw == kind)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| kind.to_string())
}

pub(super) fn app_game_policy_row_reason(row: &AppGamePolicyReadinessRow) -> String {
    if row.readiness_state == app_game_policy_readiness::APP_GAME_POLICY_READINESS_STATE_READY {
        return "ready".to_string();
    }
    let kind = app_game_policy_kind_label(row.readiness_kind.as_str());
    if row.readiness_state
        == app_game_policy_readiness::APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED
    {
        return format!("{kind} requires manual review");
    }
    format!("{kind} is missing")
}

pub(super) fn app_game_notification_reason_label(reason: &str) -> String {
    reason.replace('-', " ")
}

pub(super) fn parent_access_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    serialized_enum_label(&read_model.controller_authority)
}

pub(super) fn parent_access_state_for_read_model(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalParentAccessState {
    let value = read_model.map(parent_access_summary);
    APP_GAME_PARENT_ACCESS_LABELS
        .iter()
        .find(|(raw, _)| value.as_deref() == Some(*raw))
        .map(|(_, state)| state.clone())
        .unwrap_or(ParentPortalParentAccessState::ProofMissing)
}

pub(super) fn parent_access_detail(read_model: Option<&LanBrowserAddDeviceReadModel>) -> String {
    if let Some(read_model) = read_model {
        return format!(
            "controller authority: {} / observer authority: {}",
            serialized_enum_label(&read_model.controller_authority),
            serialized_enum_label(&read_model.observer_authority)
        );
    }
    "No LAN authority proof is attached because the local agent-service route is unavailable."
        .to_string()
}
