use super::*;
#[path = "policy_preview/helpers.rs"]
mod helpers;
use self::helpers::*;

pub(super) fn policy_preview_panel_snapshot(
    event: Option<&ParentRouteEventSnapshot>,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> ParentPolicyPreviewPanelSnapshot {
    let summary = match read_model {
        None => policy_preview_unavailable_summary(event),
        Some(read_model) if read_model.returned == 0 => {
            "No policy preview rows have been reported yet.".to_string()
        }
        Some(read_model) => policy_preview_summary(read_model),
    };

    let summary_details = policy_preview_summary_details(read_model, &summary, parent_access_state);
    let cards = policy_preview_cards(read_model, parent_access_state);

    ParentPolicyPreviewPanelSnapshot {
        title: "Policy preview parent authoring".to_string(),
        body: "Preview stays advisory until a parent confirms the request and a child-device contract applies it."
            .to_string(),
        summary,
        summary_details,
        cards,
        empty_message: "No policy preview has been reported yet.".to_string(),
        product_claim: policy_preview_product_claim(),
    }
}

fn policy_preview_summary_details(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    summary: &str,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    match read_model {
        None => policy_preview_summary_details_unavailable(summary, parent_access_state),
        Some(read_model) if read_model.returned == 0 => {
            policy_preview_summary_details_unavailable(summary, parent_access_state)
        }
        Some(read_model) => {
            policy_preview_summary_details_available(read_model, summary, parent_access_state)
        }
    }
}

fn policy_preview_summary_details_unavailable(
    summary: &str,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    vec![
        policy_preview_detail(
            "Status",
            policy_preview_required_readable_value("unavailable"),
        ),
        policy_preview_detail("Reason", summary.to_string()),
        policy_preview_detail(
            "Parent access",
            policy_preview_parent_access_readable_value(parent_access_state),
        ),
        policy_preview_detail(
            "Privacy mode",
            policy_preview_required_readable_value("local-only"),
        ),
        policy_preview_detail(
            "Adapter boundary",
            policy_preview_required_readable_value("local-adapter-unavailable"),
        ),
        policy_preview_detail(
            "Execution state",
            policy_preview_required_readable_value("disabled"),
        ),
        policy_preview_detail(
            "Provider source",
            policy_preview_required_readable_value("unavailable"),
        ),
        policy_preview_detail("Product claim", policy_preview_product_claim()),
    ]
}

fn policy_preview_summary_details_available(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    summary: &str,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    vec![
        policy_preview_detail("Decision status", summary.to_string()),
        policy_preview_detail(
            "Policy check",
            policy_preview_optional_display_value(read_model.preview_id.as_ref()),
        ),
        policy_preview_detail(
            "Parent rule context references",
            policy_preview_optional_numeric_value(read_model.parent_rule_context_reference_count),
        ),
        policy_preview_detail(
            "Parent rule context ref IDs",
            policy_preview_optional_display_value(read_model.parent_rule_context_ref_ids.as_ref()),
        ),
        policy_preview_detail(
            "Parent access",
            policy_preview_parent_access_readable_value(parent_access_state),
        ),
        policy_preview_detail(
            "Privacy mode",
            policy_preview_required_readable_value("local-only"),
        ),
        policy_preview_detail(
            "Adapter boundary",
            policy_preview_required_readable_value("local-adapter-unavailable"),
        ),
        policy_preview_detail(
            "Execution state",
            policy_preview_required_readable_value("disabled"),
        ),
        policy_preview_detail(
            "Provider source",
            policy_preview_required_readable_value("unavailable"),
        ),
        policy_preview_detail("Product claim", policy_preview_product_claim()),
    ]
}

fn policy_preview_cards(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelCardSnapshot> {
    match read_model {
        None => vec![
            policy_preview_access_card(parent_access_state, None),
            policy_preview_boundary_card(),
        ],
        Some(read_model) if read_model.returned == 0 => vec![
            policy_preview_access_card(parent_access_state, None),
            policy_preview_boundary_card(),
        ],
        Some(read_model) => vec![
            policy_preview_state_card(read_model),
            policy_preview_source_card(read_model),
            policy_preview_access_card(parent_access_state, Some(read_model)),
            policy_preview_boundary_card(),
        ],
    }
}

pub(super) fn policy_preview_state_card(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> ParentPolicyPreviewPanelCardSnapshot {
    ParentPolicyPreviewPanelCardSnapshot {
        title: "Preview state".to_string(),
        summary: policy_preview_summary(read_model),
        details: policy_preview_state_card_details(read_model),
    }
}

fn policy_preview_state_card_details(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    let mut details = policy_preview_state_card_identity_details(read_model);
    details.extend(policy_preview_state_card_review_details(read_model));
    details
}

fn policy_preview_state_card_identity_details(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    vec![
        policy_preview_detail(
            "Target type",
            policy_preview_optional_value(read_model.target_type.as_deref()),
        ),
        policy_preview_detail(
            "Target value",
            policy_preview_optional_value(read_model.target_value.as_deref()),
        ),
        policy_preview_detail(
            "Decision action",
            policy_preview_optional_display_value(read_model.decision_action.as_ref()),
        ),
        policy_preview_detail(
            "Save state",
            policy_preview_readable_value(read_model.policy_preview_save_state.as_deref()),
        ),
        policy_preview_detail(
            "Manual review",
            policy_preview_readable_value(read_model.policy_preview_manual_review_state.as_deref()),
        ),
        policy_preview_detail(
            "Target state",
            policy_preview_readable_value(read_model.policy_preview_target_state.as_deref()),
        ),
        policy_preview_detail(
            "Target explanation code",
            policy_preview_optional_value(
                read_model.policy_preview_target_explanation_code.as_deref(),
            ),
        ),
        policy_preview_detail(
            "Finding kinds",
            policy_preview_optional_value(read_model.policy_preview_finding_kinds.as_deref()),
        ),
        policy_preview_detail(
            "Request origin",
            policy_preview_readable_value(read_model.policy_request_origin.as_deref()),
        ),
        policy_preview_detail(
            "Assistant confirmation",
            policy_preview_readable_value(
                read_model.policy_assistant_confirmation_state.as_deref(),
            ),
        ),
    ]
}

fn policy_preview_state_card_review_details(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    vec![
        policy_preview_detail(
            "Request status",
            policy_preview_readable_value(read_model.policy_request_status.as_deref()),
        ),
        policy_preview_detail(
            "Approval ID",
            policy_preview_optional_display_value(read_model.policy_approval_id.as_ref()),
        ),
        policy_preview_detail(
            "Override ID",
            policy_preview_optional_display_value(read_model.policy_override_id.as_ref()),
        ),
        policy_preview_detail(
            "Replay of approval",
            policy_preview_optional_display_value(read_model.policy_replay_of_approval_id.as_ref()),
        ),
        policy_preview_detail(
            "Reviewed by",
            policy_preview_reviewed_by_value(Some(read_model)),
        ),
        policy_preview_detail(
            "Reviewed at",
            policy_preview_optional_value(read_model.policy_reviewed_at.as_deref()),
        ),
        policy_preview_detail(
            "Audit reference",
            policy_preview_optional_display_value(read_model.policy_audit_reference_id.as_ref()),
        ),
        policy_preview_detail(
            "Write authority",
            policy_preview_access_write_authority(
                &ParentPortalParentAccessState::ActiveController,
                Some(read_model),
            ),
        ),
    ]
}

pub(super) fn policy_preview_source_card(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> ParentPolicyPreviewPanelCardSnapshot {
    match read_model.policy_source_status.as_deref() {
        None => ParentPolicyPreviewPanelCardSnapshot {
            title: "Source lifecycle".to_string(),
            summary: "No source lifecycle has been reported.".to_string(),
            details: vec![
                policy_preview_detail("Source status", policy_preview_optional_value(None)),
                policy_preview_detail(
                    "Source surface",
                    policy_preview_optional_value(read_model.policy_source_surface.as_deref()),
                ),
            ],
        },
        Some(source_status) => ParentPolicyPreviewPanelCardSnapshot {
            title: "Source lifecycle".to_string(),
            summary: policy_preview_source_lifecycle_summary(source_status),
            details: vec![
                policy_preview_detail(
                    "Source status",
                    policy_preview_readable_value(Some(source_status)),
                ),
                policy_preview_detail(
                    "Source surface",
                    policy_preview_readable_value(read_model.policy_source_surface.as_deref()),
                ),
            ],
        },
    }
}

pub(super) fn policy_preview_access_card(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> ParentPolicyPreviewPanelCardSnapshot {
    ParentPolicyPreviewPanelCardSnapshot {
        title: "Approval authority".to_string(),
        summary: policy_preview_access_summary(parent_access_state, read_model),
        details: vec![
            policy_preview_detail(
                "Parent access",
                policy_preview_parent_access_readable_value(parent_access_state),
            ),
            policy_preview_detail(
                "Assistant confirmation",
                policy_preview_readable_value(
                    read_model
                        .and_then(|value| value.policy_assistant_confirmation_state.as_deref()),
                ),
            ),
            policy_preview_detail(
                "Request status",
                policy_preview_readable_value(
                    read_model.and_then(|value| value.policy_request_status.as_deref()),
                ),
            ),
            policy_preview_detail(
                "Approval ID",
                policy_preview_optional_display_value(
                    read_model.and_then(|value| value.policy_approval_id.as_ref()),
                ),
            ),
            policy_preview_detail(
                "Override ID",
                policy_preview_optional_display_value(
                    read_model.and_then(|value| value.policy_override_id.as_ref()),
                ),
            ),
            policy_preview_detail(
                "Replay of approval",
                policy_preview_optional_display_value(
                    read_model.and_then(|value| value.policy_replay_of_approval_id.as_ref()),
                ),
            ),
            policy_preview_detail("Reviewed by", policy_preview_reviewed_by_value(read_model)),
            policy_preview_detail(
                "Reviewed at",
                policy_preview_optional_value(
                    read_model.and_then(|value| value.policy_reviewed_at.as_deref()),
                ),
            ),
            policy_preview_detail(
                "Audit reference",
                policy_preview_optional_display_value(
                    read_model.and_then(|value| value.policy_audit_reference_id.as_ref()),
                ),
            ),
            policy_preview_detail(
                "Write authority",
                policy_preview_access_write_authority(parent_access_state, read_model),
            ),
        ],
    }
}

pub(super) fn policy_preview_boundary_card() -> ParentPolicyPreviewPanelCardSnapshot {
    ParentPolicyPreviewPanelCardSnapshot {
        title: "Boundary".to_string(),
        summary: "No enforcement claim".to_string(),
        details: vec![
            policy_preview_detail(
                "Privacy mode",
                policy_preview_required_readable_value("local-only"),
            ),
            policy_preview_detail(
                "Adapter boundary",
                policy_preview_required_readable_value("local-adapter-unavailable"),
            ),
            policy_preview_detail(
                "Execution state",
                policy_preview_required_readable_value("disabled"),
            ),
            policy_preview_detail(
                "Provider source",
                policy_preview_required_readable_value("unavailable"),
            ),
            policy_preview_detail("Product claim", policy_preview_product_claim()),
        ],
    }
}

pub(super) fn policy_preview_summary(read_model: &ParentPolicyPreviewReadModelSnapshot) -> String {
    if read_model.policy_request_origin.as_deref() == Some("assistant-draft")
        && read_model.policy_assistant_confirmation_state.as_deref() != Some("parent-confirmed")
    {
        return "Assistant draft remains preview-only until parent confirmation.".to_string();
    }

    if read_model.policy_preview_save_state.as_deref() == Some("blocked")
        || policy_preview_has_conflict_finding(read_model)
    {
        return "Preview is blocked and conflict details stay visible for parent review."
            .to_string();
    }

    if matches!(
        read_model.policy_preview_manual_review_state.as_deref(),
        Some("required")
    ) || matches!(
        read_model.policy_preview_target_state.as_deref(),
        Some("unsupported" | "manual-required" | "offline" | "stale")
    ) {
        return "Preview stays visible, but it is not ready to save.".to_string();
    }

    if read_model.policy_preview_save_state.as_deref() == Some("ready-to-save") {
        return "Preview is ready to save, but it is still not enforced.".to_string();
    }

    "Preview remains advisory and not enforced.".to_string()
}

pub(super) fn policy_preview_source_lifecycle_summary(source_status: &str) -> String {
    match source_status {
        "delivered" => "Delivered is reported, but active enforcement is separate.".to_string(),
        "acknowledged" => {
            "Acknowledged delivery is reported, but active enforcement is separate.".to_string()
        }
        "active" | "partially-active" => {
            "Active lifecycle is adapter-owned and stays distinct from delivery or audit claims."
                .to_string()
        }
        _ => format!(
            "Source lifecycle: {}",
            policy_preview_readable_value(Some(source_status))
        ),
    }
}

pub(super) fn policy_preview_access_summary(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> String {
    if let Some(summary) = policy_preview_parent_access_summary_text(parent_access_state) {
        return summary;
    }
    if policy_preview_is_replay_rejected(read_model) {
        return "The latest approval attempt was rejected as a replay, so no new override was created."
            .to_string();
    }
    if policy_preview_requires_parent_confirmation(read_model) {
        return "Controller authority is present, but parent confirmation is still required before any write."
            .to_string();
    }
    if policy_preview_has_recorded_audit_decision(read_model) {
        return "Controller review is recorded with reviewer and audit details, but delivery and enforcement remain separate states."
            .to_string();
    }
    if policy_preview_has_confirmed_controller_decision(read_model) {
        return "Controller confirmation is recorded, but delivery and enforcement remain separate states."
            .to_string();
    }
    "Controller authority is present, but the portal still treats this policy path as preview-only."
        .to_string()
}

pub(super) fn policy_preview_parent_access_summary_text(
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<String> {
    match parent_access_state {
        ParentPortalParentAccessState::ObserverOnly => Some(
            "Observer-only parents can review policy explanation but cannot confirm or save writes."
                .to_string(),
        ),
        ParentPortalParentAccessState::Unauthenticated => Some(
            "Sign-in is required before reviewing or confirming policy changes.".to_string(),
        ),
        ParentPortalParentAccessState::ProofMissing => Some(
            "Parent authority proof is missing, so the portal cannot claim write permission."
                .to_string(),
        ),
        ParentPortalParentAccessState::ActiveController => None,
    }
}

pub(super) fn policy_preview_access_write_authority(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> String {
    match parent_access_state {
        ParentPortalParentAccessState::ObserverOnly => {
            return "Observer scope is read-only and cannot confirm or save policy writes."
                .to_string()
        }
        ParentPortalParentAccessState::Unauthenticated => {
            return "Sign-in required before any review or confirmation action.".to_string()
        }
        ParentPortalParentAccessState::ProofMissing => {
            return "Write authority is unavailable until household role proof is visible."
                .to_string()
        }
        ParentPortalParentAccessState::ActiveController => {}
    }

    if read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref())
        == Some("parent-confirmed")
    {
        return "Parent-confirmed preview is visible, but the portal still has no typed write command."
            .to_string();
    }
    if read_model.and_then(|value| value.policy_assistant_confirmation_state.as_deref())
        == Some("parent-confirmation-required")
    {
        return "Parent confirmation is required before any write.".to_string();
    }
    "Preview-only route; no typed write command is exposed from this surface.".to_string()
}
