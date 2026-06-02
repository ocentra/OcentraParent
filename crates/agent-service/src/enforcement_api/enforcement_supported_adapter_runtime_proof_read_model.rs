use ocentra_parent_agent_protocol::{
    constants::v08_supported_adapter_runtime_proof as proof, policy_constants, ParentPlatform,
    V08SupportedAdapterAuditReferenceState, V08SupportedAdapterCapability,
    V08SupportedAdapterPlatformSupportState, V08SupportedAdapterRefusalReason,
    V08SupportedAdapterResult, V08SupportedAdapterRollbackReferenceState,
    V08SupportedAdapterRuntimeBoundary, V08SupportedAdapterRuntimeProofEntry,
    V08SupportedAdapterRuntimeProofReadModel, V08SupportedAdapterRuntimeState,
    V08SupportedAdapterTargetIdentityState,
};

pub(crate) fn v08_supported_adapter_runtime_proof_read_model(
    generated_at: &str,
) -> V08SupportedAdapterRuntimeProofReadModel {
    V08SupportedAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_BROAD_ADAPTER_PROOF.to_string(),
            proof::SOURCE_POLICY_DISPATCH_PROOF.to_string(),
            proof::SOURCE_PRODUCT_CONTROL_PROOF.to_string(),
            proof::SOURCE_NETWORK_FLOW_EVIDENCE.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    runtime_state: V08SupportedAdapterRuntimeState,
    adapter_result: V08SupportedAdapterResult,
    platform_support_state: V08SupportedAdapterPlatformSupportState,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    rollback_reference_state: V08SupportedAdapterRollbackReferenceState,
    audit_reference_state: V08SupportedAdapterAuditReferenceState,
    refusal_reason: V08SupportedAdapterRefusalReason,
    evidence_refs: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    vec![
        app_game_timer_spec(),
        network_observe_spec(),
        manual_spec(
            proof::ENTRY_ID_BROAD_APP_MANUAL,
            V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
            ParentPlatform::Windows,
            V08SupportedAdapterCapability::BroadInstalledAppBlocking,
            V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
            &[
                proof::REQUIREMENT_SAME_APP_IDENTITY,
                proof::REQUIREMENT_HOST_BLOCK_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            proof::CLAIM_BROAD_APP_MANUAL,
            proof::FALLBACK_BROAD_APP_MANUAL,
        ),
        manual_spec(
            proof::ENTRY_ID_HOST_NETWORK_MANUAL,
            V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainBlockingManualGate,
            ParentPlatform::Windows,
            V08SupportedAdapterCapability::HostNetworkDomainBlocking,
            V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
            &[
                proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            proof::CLAIM_HOST_NETWORK_MANUAL,
            proof::FALLBACK_HOST_NETWORK_MANUAL,
        ),
        exact_active_tab_not_claimed_spec(),
        permission_dependency_degraded_spec(),
        unavailable_spec(),
        unsupported_spec(),
        mobile_manual_spec(
            proof::ENTRY_ID_ANDROID_MANUAL,
            V08SupportedAdapterRuntimeBoundary::AndroidMobileControlManualGate,
            ParentPlatform::Android,
            &[
                proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                proof::REQUIREMENT_ANDROID_USAGE_STATS,
                proof::REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS,
            ],
        ),
        mobile_manual_spec(
            proof::ENTRY_ID_IOS_MANUAL,
            V08SupportedAdapterRuntimeBoundary::IosMobileControlManualGate,
            ParentPlatform::Ios,
            &[
                proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                proof::REQUIREMENT_IOS_NETWORK_EXTENSION,
            ],
        ),
    ]
}

fn app_game_timer_spec() -> EntrySpec {
    implemented_spec(
        proof::ENTRY_ID_APP_GAME_TIMER,
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        V08SupportedAdapterCapability::AppGameOwnedProcessTimeLimit,
        V08SupportedAdapterTargetIdentityState::ProcessSessionEvidenceBacked,
        V08SupportedAdapterRollbackReferenceState::TimerRecoveryBacked,
        &[
            proof::REF_APP_SESSION_EVIDENCE,
            proof::REF_OWNED_PROCESS_IDENTITY,
            proof::REF_TIMER_STATE,
        ],
        &[
            proof::COMMAND_APP_TIME_LIMIT_ADAPTER,
            proof::COMMAND_ENFORCEMENT_TIMER_CARGO,
        ],
        &[
            proof::ARTIFACT_APP_TIME_LIMIT_PROOF,
            proof::ARTIFACT_ENFORCEMENT_TIMER_STATE,
        ],
        proof::CLAIM_APP_GAME_TIMER,
        proof::FALLBACK_APP_GAME_TIMER,
    )
}

fn network_observe_spec() -> EntrySpec {
    implemented_spec(
        proof::ENTRY_ID_NETWORK_OBSERVE,
        V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        V08SupportedAdapterCapability::NetworkFlowObservePolicyHandoff,
        V08SupportedAdapterTargetIdentityState::NetworkFlowEvidenceBacked,
        V08SupportedAdapterRollbackReferenceState::ObserveOnlyNotNeeded,
        &[
            proof::REF_NETWORK_FLOW_SUMMARY,
            proof::REF_DOMAIN_ATTRIBUTION_STATE,
            proof::REF_POLICY_PREVIEW,
        ],
        &[
            proof::COMMAND_NETWORK_FLOW_CARGO,
            proof::COMMAND_POLICY_DISPATCH_PROOF,
        ],
        &[
            proof::ARTIFACT_NETWORK_FLOW_DIGEST,
            proof::ARTIFACT_POLICY_DISPATCH_PROOF,
        ],
        proof::CLAIM_NETWORK_OBSERVE,
        proof::FALLBACK_NETWORK_OBSERVE,
    )
}

fn exact_active_tab_not_claimed_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED,
        runtime_boundary:
            V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        platform: ParentPlatform::Windows,
        adapter_capability: V08SupportedAdapterCapability::ManagedExactActiveTabEnforcement,
        runtime_state: V08SupportedAdapterRuntimeState::NotClaimed,
        adapter_result: V08SupportedAdapterResult::NotClaimed,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state: V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::NotClaimed,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::NotClaimed,
        refusal_reason: V08SupportedAdapterRefusalReason::NotClaimedBoundary,
        evidence_refs: &[],
        linked_proof_commands: &[proof::COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF],
        linked_proof_artifacts: &[proof::ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF],
        manual_proof_requirements: &[
            proof::REQUIREMENT_MANAGED_ACTIVE_TAB,
            proof::REQUIREMENT_EXACT_URL_APPLY,
            proof::REQUIREMENT_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_EXACT_ACTIVE_TAB_NOT_CLAIMED,
        fallback_behavior: proof::FALLBACK_EXACT_ACTIVE_TAB_NOT_CLAIMED,
    }
}

fn permission_dependency_degraded_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_PERMISSION_DEGRADED,
        runtime_boundary:
            V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        platform: ParentPlatform::Windows,
        adapter_capability: V08SupportedAdapterCapability::AdapterPermissionDependency,
        runtime_state: V08SupportedAdapterRuntimeState::Degraded,
        adapter_result: V08SupportedAdapterResult::DegradedPermissionOrDependency,
        platform_support_state: V08SupportedAdapterPlatformSupportState::Degraded,
        target_identity_state: V08SupportedAdapterTargetIdentityState::NotApplicable,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::AuditReferenceBacked,
        refusal_reason: V08SupportedAdapterRefusalReason::PermissionOrDependencyDegraded,
        evidence_refs: &[proof::REF_ADAPTER_CAPABILITY_STATE],
        linked_proof_commands: &[proof::COMMAND_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        manual_proof_requirements: &[
            proof::REQUIREMENT_PERMISSION_RESTORE,
            proof::REQUIREMENT_DEPENDENCY_REINSTALL,
            proof::REQUIREMENT_OPERATOR_DEGRADED_STATE,
        ],
        claim_boundary: proof::CLAIM_PERMISSION_DEGRADED,
        fallback_behavior: proof::FALLBACK_PERMISSION_DEGRADED,
    }
}

fn unavailable_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_LINUX_UNAVAILABLE,
        runtime_boundary: V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        platform: ParentPlatform::Linux,
        adapter_capability: V08SupportedAdapterCapability::DesktopHostPlatformAdapter,
        runtime_state: V08SupportedAdapterRuntimeState::Unavailable,
        adapter_result: V08SupportedAdapterResult::TargetUnavailable,
        platform_support_state: V08SupportedAdapterPlatformSupportState::UnavailableOnTarget,
        target_identity_state: V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::Unavailable,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::Unavailable,
        refusal_reason: V08SupportedAdapterRefusalReason::TargetUnavailable,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: &[
            proof::REQUIREMENT_LINUX_SERVICE,
            proof::REQUIREMENT_LINUX_PERMISSION,
            proof::REQUIREMENT_LINUX_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_LINUX_UNAVAILABLE,
        fallback_behavior: proof::FALLBACK_LINUX_UNAVAILABLE,
    }
}

fn unsupported_spec() -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof::ENTRY_ID_MACOS_UNSUPPORTED,
        runtime_boundary: V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        platform: ParentPlatform::Macos,
        adapter_capability: V08SupportedAdapterCapability::DesktopHostPlatformAdapter,
        runtime_state: V08SupportedAdapterRuntimeState::Unsupported,
        adapter_result: V08SupportedAdapterResult::UnsupportedPlatform,
        platform_support_state: V08SupportedAdapterPlatformSupportState::UnsupportedPlatform,
        target_identity_state: V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::Unavailable,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::Unavailable,
        refusal_reason: V08SupportedAdapterRefusalReason::UnsupportedPlatform,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: &[
            proof::REQUIREMENT_MACOS_PERMISSION,
            proof::REQUIREMENT_MACOS_PACKAGE_IDENTITY,
            proof::REQUIREMENT_MACOS_ROLLBACK,
        ],
        claim_boundary: proof::CLAIM_MACOS_UNSUPPORTED,
        fallback_behavior: proof::FALLBACK_MACOS_UNSUPPORTED,
    }
}

fn implemented_spec(
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    adapter_capability: V08SupportedAdapterCapability,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    rollback_reference_state: V08SupportedAdapterRollbackReferenceState,
    evidence_refs: &'static [&'static str],
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        runtime_boundary,
        platform: ParentPlatform::Windows,
        adapter_capability,
        runtime_state: V08SupportedAdapterRuntimeState::ImplementedBoundary,
        adapter_result: V08SupportedAdapterResult::SupportedBoundaryProved,
        platform_support_state: V08SupportedAdapterPlatformSupportState::SupportedOnWindows,
        target_identity_state,
        rollback_reference_state,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::AuditReferenceBacked,
        refusal_reason: V08SupportedAdapterRefusalReason::None,
        evidence_refs,
        linked_proof_commands,
        linked_proof_artifacts,
        manual_proof_requirements: &[],
        claim_boundary,
        fallback_behavior,
    }
}

fn manual_spec(
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    target_identity_state: V08SupportedAdapterTargetIdentityState,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        runtime_boundary,
        platform,
        adapter_capability,
        runtime_state: V08SupportedAdapterRuntimeState::ManualRequired,
        adapter_result: V08SupportedAdapterResult::ManualProofRequired,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::ManualRequired,
        refusal_reason: V08SupportedAdapterRefusalReason::ManualArtifactRequired,
        evidence_refs: &[],
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn mobile_manual_spec(
    proof_entry_id: &'static str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    manual_proof_requirements: &'static [&'static str],
) -> EntrySpec {
    manual_spec(
        proof_entry_id,
        runtime_boundary,
        platform,
        V08SupportedAdapterCapability::MobileChildControlAdapter,
        V08SupportedAdapterTargetIdentityState::UnsupportedPlatformTarget,
        manual_proof_requirements,
        proof::CLAIM_MOBILE_MANUAL,
        proof::FALLBACK_MOBILE_MANUAL,
    )
}

fn entry_from_spec(spec: &EntrySpec, generated_at: &str) -> V08SupportedAdapterRuntimeProofEntry {
    V08SupportedAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        runtime_boundary: spec.runtime_boundary,
        platform: spec.platform,
        adapter_capability: spec.adapter_capability,
        runtime_state: spec.runtime_state,
        adapter_result: spec.adapter_result,
        platform_support_state: spec.platform_support_state,
        target_identity_state: spec.target_identity_state,
        rollback_reference_state: spec.rollback_reference_state,
        audit_reference_state: spec.audit_reference_state,
        refusal_reason: spec.refusal_reason,
        evidence_refs: to_strings(spec.evidence_refs),
        linked_proof_commands: to_strings(spec.linked_proof_commands),
        linked_proof_artifacts: to_strings(spec.linked_proof_artifacts),
        manual_proof_requirements: to_strings(spec.manual_proof_requirements),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        broad_installed_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        exact_active_tab_enforcement_claimed: false,
        notification_delivery_claimed: false,
        tamper_hardening_claimed: false,
        mobile_control_claimed: false,
        unsupported_platform_behavior_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn to_strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
