use ocentra_parent_agent_protocol::constants::v08_cross_platform_enforcement_capability_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformAdapterExecutionState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityClaimState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityName;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofEntry;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilitySurface;
use ocentra_parent_agent_protocol::policy_constants;

pub(crate) fn v08_cross_platform_enforcement_capability_proof_read_model(
    generated_at: &str,
) -> V08CrossPlatformEnforcementCapabilityProofReadModel {
    V08CrossPlatformEnforcementCapabilityProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_BROAD_PROOF.to_string(),
            proof::SOURCE_PRODUCT_PROOF.to_string(),
            proof::SOURCE_PRODUCT_AGGREGATE.to_string(),
            proof::SOURCE_PLATFORM_CAPABILITIES.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    proof_entry_id: &'static str,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    capability: V08CrossPlatformEnforcementCapabilityName,
    capability_status: V08CrossPlatformCapabilityStatus,
    product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState,
    adapter_execution_state: V08CrossPlatformAdapterExecutionState,
    linked_proof_commands: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    let mut specs = Vec::new();
    specs.extend(implemented_boundary_specs());
    specs.extend(windows_manual_specs());
    specs.extend(desktop_scaffold_specs());
    specs.extend(mobile_specs());
    specs
}

fn implemented_boundary_specs() -> Vec<EntrySpec> {
    vec![
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_WINDOWS_OWNED_PROCESS,
            surface: V08CrossPlatformEnforcementCapabilitySurface::WindowsOwnedProcessTerminate,
            platform: ParentPlatform::Windows,
            capability: V08CrossPlatformEnforcementCapabilityName::OwnedProcessTerminate,
            capability_status: V08CrossPlatformCapabilityStatus::Implemented,
            product_claim_state:
                V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary,
            adapter_execution_state: V08CrossPlatformAdapterExecutionState::ExecutesRealService,
            linked_proof_commands: &[proof::COMMAND_WINDOWS_UNMANAGED_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_UNMANAGED_PROOF],
            manual_proof_requirements: &[],
            claim_boundary: proof::CLAIM_WINDOWS_OWNED_PROCESS,
            fallback_behavior: proof::FALLBACK_WINDOWS_OWNED_PROCESS,
        },
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_WINDOWS_APP_TIME_LIMIT,
            surface: V08CrossPlatformEnforcementCapabilitySurface::WindowsAppTimeLimitLifecycle,
            platform: ParentPlatform::Windows,
            capability: V08CrossPlatformEnforcementCapabilityName::AppTimeLimit,
            capability_status: V08CrossPlatformCapabilityStatus::Implemented,
            product_claim_state:
                V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary,
            adapter_execution_state: V08CrossPlatformAdapterExecutionState::ExecutesRealService,
            linked_proof_commands: &[proof::COMMAND_WINDOWS_TIMER_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_TIMER_PROOF],
            manual_proof_requirements: &[],
            claim_boundary: proof::CLAIM_WINDOWS_APP_TIME_LIMIT,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_TIME_LIMIT,
        },
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_WINDOWS_MANAGED_BROWSER,
            surface: V08CrossPlatformEnforcementCapabilitySurface::WindowsManagedBrowserBoundary,
            platform: ParentPlatform::Windows,
            capability: V08CrossPlatformEnforcementCapabilityName::ManagedBrowserControl,
            capability_status: V08CrossPlatformCapabilityStatus::Implemented,
            product_claim_state:
                V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary,
            adapter_execution_state: V08CrossPlatformAdapterExecutionState::ExecutesRealService,
            linked_proof_commands: &[proof::COMMAND_MANAGED_BROWSER_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_MANAGED_BROWSER_PROOF],
            manual_proof_requirements: &[],
            claim_boundary: proof::CLAIM_WINDOWS_MANAGED_BROWSER,
            fallback_behavior: proof::FALLBACK_WINDOWS_MANAGED_BROWSER,
        },
        EntrySpec {
            proof_entry_id: proof::ENTRY_ID_WINDOWS_UNMANAGED_BROWSER,
            surface:
                V08CrossPlatformEnforcementCapabilitySurface::WindowsUnmanagedBrowserProcessBoundary,
            platform: ParentPlatform::Windows,
            capability: V08CrossPlatformEnforcementCapabilityName::UnmanagedBrowserDetection,
            capability_status: V08CrossPlatformCapabilityStatus::Implemented,
            product_claim_state:
                V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary,
            adapter_execution_state: V08CrossPlatformAdapterExecutionState::ExecutesRealService,
            linked_proof_commands: &[proof::COMMAND_WINDOWS_UNMANAGED_PROOF],
            linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_UNMANAGED_PROOF],
            manual_proof_requirements: &[],
            claim_boundary: proof::CLAIM_WINDOWS_UNMANAGED_BROWSER,
            fallback_behavior: proof::FALLBACK_WINDOWS_UNMANAGED_BROWSER,
        },
    ]
}

fn windows_manual_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            proof::ENTRY_ID_WINDOWS_BROAD_APP,
            V08CrossPlatformEnforcementCapabilitySurface::WindowsBroadInstalledAppBlocking,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::AppBlocking,
            &[
                proof::REQUIREMENT_OS_APP_IDENTITY,
                proof::REQUIREMENT_BLOCK_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            proof::CLAIM_WINDOWS_BROAD_APP,
            proof::FALLBACK_WINDOWS_BROAD_APP,
        ),
        manual_spec(
            proof::ENTRY_ID_WINDOWS_NETWORK_DOMAIN,
            V08CrossPlatformEnforcementCapabilitySurface::WindowsNetworkDomainBlocking,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::NetworkDomainBlocking,
            &[
                proof::REQUIREMENT_NETWORK_FILTER,
                proof::REQUIREMENT_DOMAIN_APPLY,
                proof::REQUIREMENT_ROLLBACK,
                proof::REQUIREMENT_AUDIT_CUSTODY,
            ],
            proof::CLAIM_WINDOWS_NETWORK_DOMAIN,
            proof::FALLBACK_WINDOWS_NETWORK_DOMAIN,
        ),
    ]
}

fn desktop_scaffold_specs() -> Vec<EntrySpec> {
    vec![
        scaffold_spec(
            proof::ENTRY_ID_LINUX_ADAPTER_SCAFFOLD,
            V08CrossPlatformEnforcementCapabilitySurface::LinuxEnforcementAdapterScaffold,
            ParentPlatform::Linux,
            &[
                proof::REQUIREMENT_LINUX_SERVICE,
                proof::REQUIREMENT_LINUX_ADAPTER,
            ],
            proof::CLAIM_LINUX_SCAFFOLD,
            proof::FALLBACK_LINUX_SCAFFOLD,
        ),
        scaffold_spec(
            proof::ENTRY_ID_MACOS_ADAPTER_SCAFFOLD,
            V08CrossPlatformEnforcementCapabilitySurface::MacosEnforcementAdapterScaffold,
            ParentPlatform::Macos,
            &[
                proof::REQUIREMENT_MACOS_PERMISSIONS,
                proof::REQUIREMENT_MACOS_PACKAGE,
                proof::REQUIREMENT_MACOS_ADAPTER,
            ],
            proof::CLAIM_MACOS_SCAFFOLD,
            proof::FALLBACK_MACOS_SCAFFOLD,
        ),
    ]
}

fn mobile_specs() -> Vec<EntrySpec> {
    let mut specs = Vec::new();
    specs.extend(android_specs());
    specs.extend(ios_specs());
    specs
}

fn android_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            proof::ENTRY_ID_ANDROID_DEVICE_OWNER,
            V08CrossPlatformEnforcementCapabilitySurface::AndroidDeviceOwnerPolicy,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::DeviceOwnerPolicy,
            &[
                proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                proof::REQUIREMENT_ANDROID_POLICY_APPLY,
                proof::REQUIREMENT_ANDROID_PROFILE,
            ],
            proof::CLAIM_ANDROID_DEVICE_OWNER,
            proof::FALLBACK_ANDROID_DEVICE_OWNER,
        ),
        manual_spec(
            proof::ENTRY_ID_ANDROID_PACKAGE_LIFECYCLE,
            V08CrossPlatformEnforcementCapabilitySurface::AndroidPackageLifecycle,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::PackageLifecycle,
            &[
                proof::REQUIREMENT_ANDROID_PACKAGE,
                proof::REQUIREMENT_ANDROID_LIFECYCLE,
                proof::REQUIREMENT_ANDROID_UNINSTALL,
            ],
            proof::CLAIM_ANDROID_PACKAGE,
            proof::FALLBACK_ANDROID_PACKAGE,
        ),
        planned_spec(
            proof::ENTRY_ID_ANDROID_STORE,
            V08CrossPlatformEnforcementCapabilitySurface::AndroidStoreDistribution,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::StoreDistribution,
            &[
                proof::REQUIREMENT_GOOGLE_PLAY,
                proof::REQUIREMENT_RELEASE_TRACK,
                proof::REQUIREMENT_POLICY_REVIEW,
            ],
            proof::CLAIM_ANDROID_STORE,
            proof::FALLBACK_ANDROID_STORE,
        ),
    ]
}

fn ios_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            proof::ENTRY_ID_IOS_FAMILY_CONTROLS,
            V08CrossPlatformEnforcementCapabilitySurface::IosFamilyControls,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::FamilyControlsEntitlement,
            &[
                proof::REQUIREMENT_IOS_FAMILY,
                proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                proof::REQUIREMENT_IOS_DEVICE,
            ],
            proof::CLAIM_IOS_FAMILY,
            proof::FALLBACK_IOS_FAMILY,
        ),
        manual_spec(
            proof::ENTRY_ID_IOS_SIGNING,
            V08CrossPlatformEnforcementCapabilitySurface::IosSigningEntitlements,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::SigningEntitlements,
            &[
                proof::REQUIREMENT_APPLE_SIGNING,
                proof::REQUIREMENT_IOS_ENTITLEMENTS,
                proof::REQUIREMENT_IOS_INSTALL,
            ],
            proof::CLAIM_IOS_SIGNING,
            proof::FALLBACK_IOS_SIGNING,
        ),
        manual_spec(
            proof::ENTRY_ID_IOS_TESTFLIGHT,
            V08CrossPlatformEnforcementCapabilitySurface::IosTestflightDistribution,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::TestflightDistribution,
            &[
                proof::REQUIREMENT_TESTFLIGHT,
                proof::REQUIREMENT_APP_STORE_CONNECT,
                proof::REQUIREMENT_IOS_INSTALL,
            ],
            proof::CLAIM_IOS_TESTFLIGHT,
            proof::FALLBACK_IOS_TESTFLIGHT,
        ),
        planned_spec(
            proof::ENTRY_ID_IOS_STORE,
            V08CrossPlatformEnforcementCapabilitySurface::IosStoreDistribution,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::StoreDistribution,
            &[
                proof::REQUIREMENT_APPLE_SIGNING,
                proof::REQUIREMENT_APP_STORE_REVIEW,
                proof::REQUIREMENT_APPLE_RELEASE,
            ],
            proof::CLAIM_IOS_STORE,
            proof::FALLBACK_IOS_STORE,
        ),
    ]
}

fn manual_spec(
    proof_entry_id: &'static str,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    capability: V08CrossPlatformEnforcementCapabilityName,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability,
        capability_status: V08CrossPlatformCapabilityStatus::ManualRequired,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::ReturnsManualRequired,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn scaffold_spec(
    proof_entry_id: &'static str,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability: V08CrossPlatformEnforcementCapabilityName::HeadlessAgentService,
        capability_status: V08CrossPlatformCapabilityStatus::PreviewScaffold,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::Scaffold,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::ScaffoldOnly,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn planned_spec(
    proof_entry_id: &'static str,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    capability: V08CrossPlatformEnforcementCapabilityName,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id,
        surface,
        platform,
        capability,
        capability_status: V08CrossPlatformCapabilityStatus::Planned,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::Planned,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::NotInvoked,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: &str,
) -> V08CrossPlatformEnforcementCapabilityProofEntry {
    V08CrossPlatformEnforcementCapabilityProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        surface: spec.surface,
        platform: spec.platform,
        capability: spec.capability,
        capability_status: spec.capability_status,
        product_claim_state: spec.product_claim_state,
        adapter_execution_state: spec.adapter_execution_state,
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
        broad_blocking_claimed: false,
        exact_url_claimed: false,
        privileged_mobile_claimed: false,
        production_distribution_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}
