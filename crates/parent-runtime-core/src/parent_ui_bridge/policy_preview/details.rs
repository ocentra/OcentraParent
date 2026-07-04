use super::helpers::{
    policy_preview_detail, policy_preview_optional_display_value,
    policy_preview_optional_numeric_value, policy_preview_parent_access_readable_value,
    policy_preview_product_claim, policy_preview_required_readable_value,
};
use super::*;

pub(super) fn policy_preview_summary_details_impl(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    summary: &str,
    parent_access_state: &ParentPortalParentAccessState,
) -> Vec<ParentPolicyPreviewPanelDetailSnapshot> {
    match read_model {
        None => policy_preview_summary_details_unavailable_impl(summary, parent_access_state),
        Some(read_model) if read_model.returned == 0 => {
            policy_preview_summary_details_unavailable_impl(summary, parent_access_state)
        }
        Some(read_model) => {
            policy_preview_summary_details_available_impl(read_model, summary, parent_access_state)
        }
    }
}

fn policy_preview_summary_details_unavailable_impl(
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

fn policy_preview_summary_details_available_impl(
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
