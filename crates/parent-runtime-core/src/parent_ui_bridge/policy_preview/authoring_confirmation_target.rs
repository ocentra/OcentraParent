use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;

use super::super::StagedPolicyPreviewDraft;

pub(super) fn target_kind(
    draft: &StagedPolicyPreviewDraft,
) -> Result<PolicyRequestAssistantPreviewConfirmTargetKind, String> {
    match draft.read_model.target_type.as_deref() {
        Some("app") => Ok(PolicyRequestAssistantPreviewConfirmTargetKind::App),
        Some("device") => Ok(PolicyRequestAssistantPreviewConfirmTargetKind::Device),
        Some("site") | Some("domain") => Ok(PolicyRequestAssistantPreviewConfirmTargetKind::Site),
        Some("category") => Ok(PolicyRequestAssistantPreviewConfirmTargetKind::Category),
        Some(value) => Err(format!(
            "policy preview target kind cannot be confirmed: {value}"
        )),
        None => Err("policy preview target kind is missing".to_string()),
    }
}
