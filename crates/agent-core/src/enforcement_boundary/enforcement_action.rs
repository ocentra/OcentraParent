use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementMode,
    ParentPlatform,
};

use super::enforcement_adapter_kind::adapter_kind;
use super::EnforcementBoundaryInput;

pub(super) fn enforcement_action(
    input: &EnforcementBoundaryInput,
    mode: EnforcementMode,
) -> EnforcementAction {
    EnforcementAction {
        schema_version: input.decision.schema_version.clone(),
        action_id: input.action_id.clone(),
        intent_id: input.intent.intent_id.clone(),
        policy_decision_id: input.decision.decision_id.clone(),
        policy_action: input.decision.action,
        adapter_kind: adapter_kind(mode, input.intent.target.target_type),
        platform: capability_platform(&input.capability, &input.intent.device.platform),
        target: input.intent.target.clone(),
        mode,
        capability: input.capability.clone(),
        reason_codes: input.decision.reason_codes.clone(),
        evidence_references: input.intent.evidence_references.clone(),
        local_ai_result_id: input.decision.local_ai_result_id.clone(),
        parent_approval: input.intent.parent_approval.clone(),
        dry_run: input.decision.dry_run
            || input.capability.capability_state == EnforcementCapabilityState::DryRun,
        requested_at: input.requested_at.clone(),
        expires_at: input.decision.expires_at.clone(),
        rollback_token: input.rollback_token.clone(),
    }
}

fn capability_platform(
    capability: &EnforcementCapabilityStatus,
    device_platform: &str,
) -> ParentPlatform {
    if capability.capability_state != EnforcementCapabilityState::Unavailable {
        return capability.platform;
    }

    match device_platform {
        enforcement_constants::PLATFORM_WINDOWS => Some(ParentPlatform::Windows),
        enforcement_constants::PLATFORM_LINUX => Some(ParentPlatform::Linux),
        enforcement_constants::PLATFORM_MACOS => Some(ParentPlatform::Macos),
        enforcement_constants::PLATFORM_ANDROID => Some(ParentPlatform::Android),
        enforcement_constants::PLATFORM_IOS => Some(ParentPlatform::Ios),
        _ => None,
    }
    .unwrap_or(capability.platform)
}
