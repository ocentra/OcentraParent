use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlCapabilityName;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlClaimState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlDevicePolicyState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlExecutionState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineEntry;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurface;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurfaceKind;
use ocentra_parent_agent_protocol::policy_constants;

pub(super) fn linked_entry(spec: LinkedEntrySpec<'_>) -> V08EnforcementProductControlSpineEntry {
    linked_with_manual_entry(LinkedManualEntrySpec {
        entry_id: spec.entry_id,
        surface: spec.surface,
        surface_kind: spec.surface_kind,
        capability: spec.capability,
        capability_status: V08EnforcementProductControlCapabilityStatus::Implemented,
        product_claim_state: spec.product_claim_state,
        adapter_execution_state: spec.adapter_execution_state,
        device_policy_state: spec.device_policy_state,
        parent_visible_actions: spec.parent_visible_actions,
        linked_proof_commands: spec.linked_proof_commands,
        linked_proof_artifacts: spec.linked_proof_artifacts,
        manual_proof_requirements: &[],
        claim_boundary: spec.claim_boundary,
        fallback_behavior: spec.fallback_behavior,
        generated_at: spec.generated_at,
    })
}

pub(super) fn manual_entry(spec: ManualEntrySpec<'_>) -> V08EnforcementProductControlSpineEntry {
    linked_with_manual_entry(LinkedManualEntrySpec {
        entry_id: spec.entry_id,
        surface: spec.surface,
        surface_kind: spec.surface_kind,
        capability: spec.capability,
        capability_status: V08EnforcementProductControlCapabilityStatus::ManualRequired,
        product_claim_state: V08EnforcementProductControlClaimState::ManualRequired,
        adapter_execution_state: V08EnforcementProductControlExecutionState::ReturnsManualRequired,
        device_policy_state: V08EnforcementProductControlDevicePolicyState::ManualRequired,
        parent_visible_actions: &[V08EnforcementProductControlParentAction::ReportOnly],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: spec.manual_proof_requirements,
        claim_boundary: spec.claim_boundary,
        fallback_behavior: spec.fallback_behavior,
        generated_at: spec.generated_at,
    })
}

pub(super) fn linked_with_manual_entry(
    spec: LinkedManualEntrySpec<'_>,
) -> V08EnforcementProductControlSpineEntry {
    product_entry(ProductEntrySpec {
        entry_id: spec.entry_id,
        surface: spec.surface,
        surface_kind: spec.surface_kind,
        capability: spec.capability,
        capability_status: spec.capability_status,
        product_claim_state: spec.product_claim_state,
        adapter_execution_state: spec.adapter_execution_state,
        device_policy_state: spec.device_policy_state,
        parent_visible_actions: spec.parent_visible_actions,
        linked_proof_commands: spec.linked_proof_commands,
        linked_proof_artifacts: spec.linked_proof_artifacts,
        manual_proof_requirements: spec.manual_proof_requirements,
        claim_boundary: spec.claim_boundary,
        fallback_behavior: spec.fallback_behavior,
        generated_at: spec.generated_at,
    })
}

#[derive(Clone, Copy)]
pub(super) struct LinkedEntrySpec<'a> {
    pub(super) entry_id: &'static str,
    pub(super) surface: V08EnforcementProductControlSurface,
    pub(super) surface_kind: V08EnforcementProductControlSurfaceKind,
    pub(super) capability: V08EnforcementProductControlCapabilityName,
    pub(super) product_claim_state: V08EnforcementProductControlClaimState,
    pub(super) adapter_execution_state: V08EnforcementProductControlExecutionState,
    pub(super) device_policy_state: V08EnforcementProductControlDevicePolicyState,
    pub(super) parent_visible_actions: &'static [V08EnforcementProductControlParentAction],
    pub(super) linked_proof_commands: &'static [&'static str],
    pub(super) linked_proof_artifacts: &'static [&'static str],
    pub(super) claim_boundary: &'static str,
    pub(super) fallback_behavior: &'static str,
    pub(super) generated_at: &'a str,
}

#[derive(Clone, Copy)]
pub(super) struct ManualEntrySpec<'a> {
    pub(super) entry_id: &'static str,
    pub(super) surface: V08EnforcementProductControlSurface,
    pub(super) surface_kind: V08EnforcementProductControlSurfaceKind,
    pub(super) capability: V08EnforcementProductControlCapabilityName,
    pub(super) manual_proof_requirements: &'static [&'static str],
    pub(super) claim_boundary: &'static str,
    pub(super) fallback_behavior: &'static str,
    pub(super) generated_at: &'a str,
}

#[derive(Clone, Copy)]
pub(super) struct LinkedManualEntrySpec<'a> {
    pub(super) entry_id: &'static str,
    pub(super) surface: V08EnforcementProductControlSurface,
    pub(super) surface_kind: V08EnforcementProductControlSurfaceKind,
    pub(super) capability: V08EnforcementProductControlCapabilityName,
    pub(super) capability_status: V08EnforcementProductControlCapabilityStatus,
    pub(super) product_claim_state: V08EnforcementProductControlClaimState,
    pub(super) adapter_execution_state: V08EnforcementProductControlExecutionState,
    pub(super) device_policy_state: V08EnforcementProductControlDevicePolicyState,
    pub(super) parent_visible_actions: &'static [V08EnforcementProductControlParentAction],
    pub(super) linked_proof_commands: &'static [&'static str],
    pub(super) linked_proof_artifacts: &'static [&'static str],
    pub(super) manual_proof_requirements: &'static [&'static str],
    pub(super) claim_boundary: &'static str,
    pub(super) fallback_behavior: &'static str,
    pub(super) generated_at: &'a str,
}

#[derive(Clone, Copy)]
pub(super) struct ProductEntrySpec<'a> {
    pub(super) entry_id: &'static str,
    pub(super) surface: V08EnforcementProductControlSurface,
    pub(super) surface_kind: V08EnforcementProductControlSurfaceKind,
    pub(super) capability: V08EnforcementProductControlCapabilityName,
    pub(super) capability_status: V08EnforcementProductControlCapabilityStatus,
    pub(super) product_claim_state: V08EnforcementProductControlClaimState,
    pub(super) adapter_execution_state: V08EnforcementProductControlExecutionState,
    pub(super) device_policy_state: V08EnforcementProductControlDevicePolicyState,
    pub(super) parent_visible_actions: &'static [V08EnforcementProductControlParentAction],
    pub(super) linked_proof_commands: &'static [&'static str],
    pub(super) linked_proof_artifacts: &'static [&'static str],
    pub(super) manual_proof_requirements: &'static [&'static str],
    pub(super) claim_boundary: &'static str,
    pub(super) fallback_behavior: &'static str,
    pub(super) generated_at: &'a str,
}

pub(super) fn product_entry(spec: ProductEntrySpec<'_>) -> V08EnforcementProductControlSpineEntry {
    V08EnforcementProductControlSpineEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        entry_id: spec.entry_id.to_string(),
        surface: spec.surface,
        surface_kind: spec.surface_kind,
        platform: ParentPlatform::Windows,
        capability: spec.capability,
        capability_status: spec.capability_status,
        product_claim_state: spec.product_claim_state,
        adapter_execution_state: spec.adapter_execution_state,
        device_policy_state: spec.device_policy_state,
        parent_visible_actions: spec.parent_visible_actions.to_vec(),
        linked_proof_commands: spec
            .linked_proof_commands
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        linked_proof_artifacts: spec
            .linked_proof_artifacts
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        manual_proof_requirements: spec
            .manual_proof_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        broad_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        managed_exact_url_blocking_claimed: false,
        unmanaged_exact_url_claimed: false,
        tamper_resistance_claimed: false,
        notification_delivery_claimed: false,
        last_checked_at: spec.generated_at.to_string(),
    }
}
