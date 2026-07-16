use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;

use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

#[path = "activity_store_policy_preview_targets_aliases.rs"]
mod activity_store_policy_preview_targets_aliases;
#[path = "activity_store_policy_preview_targets_selection.rs"]
mod activity_store_policy_preview_targets_selection;

pub(crate) struct PolicyPreviewTargets {
    pub primary: PolicyTarget,
    pub aliases: Vec<PolicyTarget>,
}

pub(crate) fn targets_from_row(row: &PolicyPreviewStoreRow) -> Option<PolicyPreviewTargets> {
    let primary = activity_store_policy_preview_targets_selection::target_from_row(row)?;
    let aliases =
        activity_store_policy_preview_targets_aliases::target_aliases_from_row(row, &primary);
    Some(PolicyPreviewTargets { primary, aliases })
}
