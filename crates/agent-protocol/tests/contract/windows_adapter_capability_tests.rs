use super::{
    constants, constants::enforcement, constants::host_identity,
    constants::windows_adapter_capability as windows_adapter, policy_constants as policy,
    EnforcementAdapterKind, EnforcementBroadAdapterCapability, EnforcementCapabilityState,
    EnforcementMode, EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner,
    EnforcementReadinessState, ParentPlatform, WindowsAdapterCapabilityOutcome,
    WindowsAdapterCapabilityProof, WindowsAdapterCapabilityProofEntry,
    WindowsAdapterCapabilitySurface,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn windows_adapter_capability_proof_serializes_claim_boundaries() {
    let proof = WindowsAdapterCapabilityProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: windows_adapter::READ_MODEL_ID_V0_8.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        platform: ParentPlatform::Windows,
        entries: vec![app_entry(), unmanaged_entry()],
    };

    let serialized =
        serde_json::to_value(proof).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::READ_MODEL_ID],
        windows_adapter::READ_MODEL_ID_V0_8
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][0]["surface"],
        windows_adapter::SURFACE_APP_TARGET
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][0]["linkedHostIdentityEntryIds"][0],
        host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][1]["outcome"],
        windows_adapter::OUTCOME_PROCESS_ONLY_IMPLEMENTED
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][1]["exactUrlClaimed"],
        false
    );
}

#[test]
fn unsupported_windows_adapter_outcome_does_not_deserialize() {
    let payload = serde_json::json!({
        "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
        "proofEntryId": windows_adapter::ENTRY_ID_APP_TARGET,
        "surface": windows_adapter::SURFACE_APP_TARGET,
        "platform": enforcement::PLATFORM_WINDOWS,
        "primaryCapability": enforcement::BROAD_CAPABILITY_BROAD_APP_BLOCKING,
        "adapterKind": enforcement::ADAPTER_KIND_PROCESS_CONTROL,
        "capabilityState": enforcement::CAPABILITY_MANUAL_REQUIRED,
        "readinessState": enforcement::READINESS_MANUAL_REQUIRED,
        "proofLevel": enforcement::PROOF_MANUAL_REQUIRED,
        "runtimeOwner": enforcement::RUNTIME_OWNER_MANUAL_PROOF,
        "supportedModes": [enforcement::MODE_BLOCK_PROCESS],
        "linkedReadinessIds": [enforcement::READINESS_ID_BROAD_APP_BLOCKING],
        "linkedHostIdentityEntryIds": [host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY],
        "outcome": "product-ready",
        "claimBoundary": windows_adapter::CLAIM_BOUNDARY_APP_TARGET,
        "fallbackBehavior": windows_adapter::FALLBACK_APP_TARGET,
        "exactUrlClaimed": false,
        "broadBlockingClaimed": false,
        "requiredArtifacts": [windows_adapter::ARTIFACT_WINDOWS_APP_IDENTITY],
        "lastCheckedAt": policy::TEST_EVALUATED_AT
    });

    let parsed = serde_json::from_value::<WindowsAdapterCapabilityProofEntry>(payload);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

fn app_entry() -> WindowsAdapterCapabilityProofEntry {
    WindowsAdapterCapabilityProofEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: windows_adapter::ENTRY_ID_APP_TARGET.to_string(),
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        platform: ParentPlatform::Windows,
        primary_capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::ManualRequired,
        readiness_state: EnforcementReadinessState::ManualRequired,
        proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: vec![EnforcementMode::BlockProcess],
        linked_readiness_ids: vec![enforcement::READINESS_ID_BROAD_APP_BLOCKING.to_string()],
        linked_host_identity_entry_ids: vec![
            host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY.to_string(),
            host_identity::ENTRY_ID_PACKAGE_IDENTITY.to_string(),
        ],
        outcome: WindowsAdapterCapabilityOutcome::ManualRequired,
        claim_boundary: windows_adapter::CLAIM_BOUNDARY_APP_TARGET.to_string(),
        fallback_behavior: windows_adapter::FALLBACK_APP_TARGET.to_string(),
        exact_url_claimed: false,
        broad_blocking_claimed: false,
        required_artifacts: vec![windows_adapter::ARTIFACT_WINDOWS_APP_IDENTITY.to_string()],
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn unmanaged_entry() -> WindowsAdapterCapabilityProofEntry {
    WindowsAdapterCapabilityProofEntry {
        proof_entry_id: windows_adapter::ENTRY_ID_UNMANAGED_BROWSER_TARGET.to_string(),
        surface: WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
        primary_capability: EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
        capability_state: EnforcementCapabilityState::Supported,
        readiness_state: EnforcementReadinessState::Implemented,
        proof_level: EnforcementReadinessProofLevel::RealServiceProof,
        runtime_owner: EnforcementReadinessRuntimeOwner::OsAdapter,
        supported_modes: vec![
            EnforcementMode::TerminateProcess,
            EnforcementMode::ObserveOnly,
        ],
        linked_readiness_ids: vec![
            enforcement::READINESS_ID_UNMANAGED_BROWSER_PROCESS_ONLY.to_string(),
            enforcement::READINESS_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE.to_string(),
        ],
        linked_host_identity_entry_ids: Vec::new(),
        outcome: WindowsAdapterCapabilityOutcome::ProcessOnlyImplemented,
        claim_boundary: windows_adapter::CLAIM_BOUNDARY_UNMANAGED_BROWSER_TARGET.to_string(),
        fallback_behavior: windows_adapter::FALLBACK_UNMANAGED_BROWSER_TARGET.to_string(),
        required_artifacts: Vec::new(),
        ..app_entry()
    }
}
