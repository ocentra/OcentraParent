use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as browser_proof;
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

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct ProofEntryId(pub(crate) &'static str);

#[derive(Clone, Copy)]
struct EntryProofText {
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

pub(crate) fn v08_cross_platform_enforcement_capability_proof_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08CrossPlatformEnforcementCapabilityProofReadModel {
    let generated_at = generated_at.into();
    V08CrossPlatformEnforcementCapabilityProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
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
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_WINDOWS_APP_TIME_LIMIT),
            V08CrossPlatformEnforcementCapabilitySurface::WindowsAppTimeLimitLifecycle,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::AppTimeLimit,
            EntryProofText {
                manual_proof_requirements: &[
                    enforcement_constants::ARTIFACT_APP_TIME_LIMIT_EXECUTOR,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
                claim_boundary: proof::CLAIM_WINDOWS_APP_TIME_LIMIT,
                fallback_behavior: proof::FALLBACK_WINDOWS_APP_TIME_LIMIT,
            },
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_WINDOWS_MANAGED_BROWSER),
            V08CrossPlatformEnforcementCapabilitySurface::WindowsManagedBrowserBoundary,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::ManagedBrowserControl,
            EntryProofText {
                manual_proof_requirements: &[
                    browser_proof::REQUIREMENT_MANAGED_PROFILE,
                    browser_proof::REQUIREMENT_ACTIVE_TAB,
                    browser_proof::REQUIREMENT_ROLLBACK,
                    browser_proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
                claim_boundary: proof::CLAIM_WINDOWS_MANAGED_BROWSER,
                fallback_behavior: proof::FALLBACK_WINDOWS_MANAGED_BROWSER,
            },
        ),
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
            ProofEntryId(proof::ENTRY_ID_WINDOWS_BROAD_APP),
            V08CrossPlatformEnforcementCapabilitySurface::WindowsBroadInstalledAppBlocking,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::AppBlocking,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_OS_APP_IDENTITY,
                    proof::REQUIREMENT_BLOCK_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
                claim_boundary: proof::CLAIM_WINDOWS_BROAD_APP,
                fallback_behavior: proof::FALLBACK_WINDOWS_BROAD_APP,
            },
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_WINDOWS_NETWORK_DOMAIN),
            V08CrossPlatformEnforcementCapabilitySurface::WindowsNetworkDomainBlocking,
            ParentPlatform::Windows,
            V08CrossPlatformEnforcementCapabilityName::NetworkDomainBlocking,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_NETWORK_FILTER,
                    proof::REQUIREMENT_DOMAIN_APPLY,
                    proof::REQUIREMENT_ROLLBACK,
                    proof::REQUIREMENT_AUDIT_CUSTODY,
                ],
                claim_boundary: proof::CLAIM_WINDOWS_NETWORK_DOMAIN,
                fallback_behavior: proof::FALLBACK_WINDOWS_NETWORK_DOMAIN,
            },
        ),
    ]
}

fn desktop_scaffold_specs() -> Vec<EntrySpec> {
    vec![
        scaffold_spec(
            ProofEntryId(proof::ENTRY_ID_LINUX_ADAPTER_SCAFFOLD),
            V08CrossPlatformEnforcementCapabilitySurface::LinuxEnforcementAdapterScaffold,
            ParentPlatform::Linux,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_LINUX_SERVICE,
                    proof::REQUIREMENT_LINUX_ADAPTER,
                ],
                claim_boundary: proof::CLAIM_LINUX_SCAFFOLD,
                fallback_behavior: proof::FALLBACK_LINUX_SCAFFOLD,
            },
        ),
        scaffold_spec(
            ProofEntryId(proof::ENTRY_ID_MACOS_ADAPTER_SCAFFOLD),
            V08CrossPlatformEnforcementCapabilitySurface::MacosEnforcementAdapterScaffold,
            ParentPlatform::Macos,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_MACOS_PERMISSIONS,
                    proof::REQUIREMENT_MACOS_PACKAGE,
                    proof::REQUIREMENT_MACOS_ADAPTER,
                ],
                claim_boundary: proof::CLAIM_MACOS_SCAFFOLD,
                fallback_behavior: proof::FALLBACK_MACOS_SCAFFOLD,
            },
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
            ProofEntryId(proof::ENTRY_ID_ANDROID_DEVICE_OWNER),
            V08CrossPlatformEnforcementCapabilitySurface::AndroidDeviceOwnerPolicy,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::DeviceOwnerPolicy,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                    proof::REQUIREMENT_ANDROID_POLICY_APPLY,
                    proof::REQUIREMENT_ANDROID_PROFILE,
                ],
                claim_boundary: proof::CLAIM_ANDROID_DEVICE_OWNER,
                fallback_behavior: proof::FALLBACK_ANDROID_DEVICE_OWNER,
            },
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_ANDROID_PACKAGE_LIFECYCLE),
            V08CrossPlatformEnforcementCapabilitySurface::AndroidPackageLifecycle,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::PackageLifecycle,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_ANDROID_PACKAGE,
                    proof::REQUIREMENT_ANDROID_LIFECYCLE,
                    proof::REQUIREMENT_ANDROID_UNINSTALL,
                ],
                claim_boundary: proof::CLAIM_ANDROID_PACKAGE,
                fallback_behavior: proof::FALLBACK_ANDROID_PACKAGE,
            },
        ),
        planned_spec(
            ProofEntryId(proof::ENTRY_ID_ANDROID_STORE),
            V08CrossPlatformEnforcementCapabilitySurface::AndroidStoreDistribution,
            ParentPlatform::Android,
            V08CrossPlatformEnforcementCapabilityName::StoreDistribution,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_GOOGLE_PLAY,
                    proof::REQUIREMENT_RELEASE_TRACK,
                    proof::REQUIREMENT_POLICY_REVIEW,
                ],
                claim_boundary: proof::CLAIM_ANDROID_STORE,
                fallback_behavior: proof::FALLBACK_ANDROID_STORE,
            },
        ),
    ]
}

fn ios_specs() -> Vec<EntrySpec> {
    vec![
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_FAMILY_CONTROLS),
            V08CrossPlatformEnforcementCapabilitySurface::IosFamilyControls,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::FamilyControlsEntitlement,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_IOS_FAMILY,
                    proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
                    proof::REQUIREMENT_IOS_DEVICE,
                ],
                claim_boundary: proof::CLAIM_IOS_FAMILY,
                fallback_behavior: proof::FALLBACK_IOS_FAMILY,
            },
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_SIGNING),
            V08CrossPlatformEnforcementCapabilitySurface::IosSigningEntitlements,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::SigningEntitlements,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_APPLE_SIGNING,
                    proof::REQUIREMENT_IOS_ENTITLEMENTS,
                    proof::REQUIREMENT_IOS_INSTALL,
                ],
                claim_boundary: proof::CLAIM_IOS_SIGNING,
                fallback_behavior: proof::FALLBACK_IOS_SIGNING,
            },
        ),
        manual_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_TESTFLIGHT),
            V08CrossPlatformEnforcementCapabilitySurface::IosTestflightDistribution,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::TestflightDistribution,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_TESTFLIGHT,
                    proof::REQUIREMENT_APP_STORE_CONNECT,
                    proof::REQUIREMENT_IOS_INSTALL,
                ],
                claim_boundary: proof::CLAIM_IOS_TESTFLIGHT,
                fallback_behavior: proof::FALLBACK_IOS_TESTFLIGHT,
            },
        ),
        planned_spec(
            ProofEntryId(proof::ENTRY_ID_IOS_STORE),
            V08CrossPlatformEnforcementCapabilitySurface::IosStoreDistribution,
            ParentPlatform::Ios,
            V08CrossPlatformEnforcementCapabilityName::StoreDistribution,
            EntryProofText {
                manual_proof_requirements: &[
                    proof::REQUIREMENT_APPLE_SIGNING,
                    proof::REQUIREMENT_APP_STORE_REVIEW,
                    proof::REQUIREMENT_APPLE_RELEASE,
                ],
                claim_boundary: proof::CLAIM_IOS_STORE,
                fallback_behavior: proof::FALLBACK_IOS_STORE,
            },
        ),
    ]
}

fn manual_spec(
    proof_entry_id: ProofEntryId,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    capability: V08CrossPlatformEnforcementCapabilityName,
    text: EntryProofText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
        surface,
        platform,
        capability,
        capability_status: V08CrossPlatformCapabilityStatus::ManualRequired,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::ReturnsManualRequired,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: text.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn scaffold_spec(
    proof_entry_id: ProofEntryId,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    text: EntryProofText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
        surface,
        platform,
        capability: V08CrossPlatformEnforcementCapabilityName::HeadlessAgentService,
        capability_status: V08CrossPlatformCapabilityStatus::PreviewScaffold,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::Scaffold,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::ScaffoldOnly,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: text.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn planned_spec(
    proof_entry_id: ProofEntryId,
    surface: V08CrossPlatformEnforcementCapabilitySurface,
    platform: ParentPlatform,
    capability: V08CrossPlatformEnforcementCapabilityName,
    text: EntryProofText,
) -> EntrySpec {
    EntrySpec {
        proof_entry_id: proof_entry_id.0,
        surface,
        platform,
        capability,
        capability_status: V08CrossPlatformCapabilityStatus::Planned,
        product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState::Planned,
        adapter_execution_state: V08CrossPlatformAdapterExecutionState::NotInvoked,
        linked_proof_commands: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements: text.manual_proof_requirements,
        claim_boundary: text.claim_boundary,
        fallback_behavior: text.fallback_behavior,
    }
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: GeneratedAtTextRef<'_>,
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
        last_checked_at: generated_at.0.to_string(),
    }
}
