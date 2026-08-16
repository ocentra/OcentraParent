use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;

#[derive(Clone)]
pub(super) struct SupportedPolicyPreviewTarget {
    pub(super) subject_kind: ActivitySubjectKind,
    pub(super) target_type: PolicyTargetType,
    pub(super) target_value: String,
    pub(super) subject_display_name: String,
    pub(super) subject_field: Option<(&'static str, String)>,
}

#[derive(Clone, Copy)]
struct SupportedPolicyPreviewSubjectField(&'static str);

pub(super) fn supported_policy_preview_target(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Option<SupportedPolicyPreviewTarget> {
    if matches!(
        request.target_kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::App
    ) {
        return Some(build_supported_policy_preview_target(
            request,
            ActivitySubjectKind::Process,
            PolicyTargetType::App,
            Some(SupportedPolicyPreviewSubjectField(
                constants::field::PROCESS_NAME,
            )),
        ));
    }

    if matches!(
        request.target_kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::Site
    ) {
        return Some(build_supported_policy_preview_target(
            request,
            ActivitySubjectKind::Url,
            PolicyTargetType::Site,
            Some(SupportedPolicyPreviewSubjectField(constants::field::URL)),
        ));
    }

    if matches!(
        request.target_kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::Category
    ) {
        return Some(build_supported_policy_preview_target(
            request,
            ActivitySubjectKind::Device,
            PolicyTargetType::Category,
            Some(SupportedPolicyPreviewSubjectField(
                constants::field::SCREEN_PRIMARY_CATEGORY,
            )),
        ));
    }

    if matches!(
        request.target_kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::Device
    ) {
        return Some(build_supported_policy_preview_target(
            request,
            ActivitySubjectKind::Device,
            PolicyTargetType::Device,
            None,
        ));
    }

    None
}

fn build_supported_policy_preview_target(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    subject_kind: ActivitySubjectKind,
    target_type: PolicyTargetType,
    subject_field: Option<SupportedPolicyPreviewSubjectField>,
) -> SupportedPolicyPreviewTarget {
    let target_value = request.target_reference_id.clone();
    let subject_display_name = target_value.clone();

    SupportedPolicyPreviewTarget {
        subject_kind,
        target_type,
        target_value: target_value.clone(),
        subject_display_name,
        subject_field: subject_field.map(|field| (field.0, target_value)),
    }
}
