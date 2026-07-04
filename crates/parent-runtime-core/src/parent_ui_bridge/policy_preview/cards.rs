use super::access_summary::policy_preview_access_summary_impl;
use super::access_write::policy_preview_access_write_authority_impl;
use super::helpers::{
    policy_preview_detail, policy_preview_optional_display_value, policy_preview_optional_value,
    policy_preview_parent_access_readable_value, policy_preview_product_claim,
    policy_preview_readable_value, policy_preview_required_readable_value,
    policy_preview_reviewed_by_value,
};
use super::summary::{policy_preview_source_lifecycle_summary_impl, policy_preview_summary_impl};
use super::*;

pub(super) fn policy_preview_cards_impl(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelCardSnapshot> {
    match read_model {
        None => vec![
            policy_preview_access_card_impl(parent_access_state, None),
            policy_preview_boundary_card_impl(),
        ],
        Some(read_model) if read_model.returned == 0 => vec![
            policy_preview_access_card_impl(parent_access_state, None),
            policy_preview_boundary_card_impl(),
        ],
        Some(read_model) => vec![
            policy_preview_state_card_impl(read_model),
            policy_preview_source_card_impl(read_model),
            policy_preview_access_card_impl(parent_access_state, Some(read_model)),
            policy_preview_boundary_card_impl(),
        ],
    }
}

pub(super) fn policy_preview_state_card_impl(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> ParentPolicyPreviewPanelCardSnapshot {
    ParentPolicyPreviewPanelCardSnapshot {
        title: "Preview state".to_string(),
        summary: policy_preview_summary_impl(read_model),
        details: policy_preview_state_card_details_impl(read_model),
    }
}

fn policy_preview_state_card_details_impl(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    let mut details = policy_preview_state_card_identity_details_impl(read_model);
    details.extend(policy_preview_state_card_review_details_impl(read_model));
    details
}

fn policy_preview_state_card_identity_details_impl(
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

fn policy_preview_state_card_review_details_impl(
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
            policy_preview_access_write_authority_impl(
                &ParentPortalParentAccessState::ActiveController,
                Some(read_model),
            ),
        ),
    ]
}

pub(super) fn policy_preview_source_card_impl(
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
            summary: policy_preview_source_lifecycle_summary_impl(source_status),
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

pub(super) fn policy_preview_access_card_impl(
    parent_access_state: &ParentPortalParentAccessState,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> ParentPolicyPreviewPanelCardSnapshot {
    ParentPolicyPreviewPanelCardSnapshot {
        title: "Approval authority".to_string(),
        summary: policy_preview_access_summary_impl(parent_access_state, read_model),
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
                policy_preview_access_write_authority_impl(parent_access_state, read_model),
            ),
        ],
    }
}

pub(super) fn policy_preview_boundary_card_impl() -> ParentPolicyPreviewPanelCardSnapshot {
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
