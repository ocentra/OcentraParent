use super::{
    constants, constants::enforcement, policy_constants as policy, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementBroadAdapterReadinessEntry,
    EnforcementBroadOsAdapterReadinessMatrix, EnforcementCapabilityState, EnforcementMode,
    EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
    ParentPlatform,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn broad_adapter_readiness_serializes_contract_boundaries() {
    let readiness_entry =
        |readiness_id: &'static str, capability, readiness_state, proof_level, runtime_owner| {
            let capability_state = match readiness_state {
                EnforcementReadinessState::Implemented => EnforcementCapabilityState::Supported,
                EnforcementReadinessState::ManualRequired
                | EnforcementReadinessState::NotClaimed => {
                    EnforcementCapabilityState::ManualRequired
                }
                EnforcementReadinessState::Unavailable => EnforcementCapabilityState::Unavailable,
            };

            EnforcementBroadAdapterReadinessEntry {
                schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                readiness_id: readiness_id.to_string(),
                capability,
                platform: ParentPlatform::Windows,
                adapter_kind: EnforcementAdapterKind::ProcessControl,
                capability_state,
                readiness_state,
                proof_level,
                runtime_owner,
                supported_modes: vec![EnforcementMode::BlockProcess],
                claim_boundary: enforcement::CLAIM_BOUNDARY_BROAD_APP_BLOCKING.to_string(),
                fallback_behavior: enforcement::FALLBACK_BROAD_APP_BLOCKING.to_string(),
                required_artifacts: vec![enforcement::ARTIFACT_OS_APP_IDENTITY.to_string()],
                last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
            }
        };
    let matrix = EnforcementBroadOsAdapterReadinessMatrix {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        matrix_id: enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        entries: vec![
            readiness_entry(
                enforcement::READINESS_ID_OWNED_PROCESS_TERMINATE,
                EnforcementBroadAdapterCapability::OwnedProcessTerminate,
                EnforcementReadinessState::Implemented,
                EnforcementReadinessProofLevel::RealServiceProof,
                EnforcementReadinessRuntimeOwner::OsAdapter,
            ),
            readiness_entry(
                enforcement::READINESS_ID_BROAD_APP_BLOCKING,
                EnforcementBroadAdapterCapability::BroadAppBlocking,
                EnforcementReadinessState::ManualRequired,
                EnforcementReadinessProofLevel::ManualProofRequired,
                EnforcementReadinessRuntimeOwner::ManualProof,
            ),
            readiness_entry(
                enforcement::READINESS_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE,
                EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence,
                EnforcementReadinessState::NotClaimed,
                EnforcementReadinessProofLevel::NotProved,
                EnforcementReadinessRuntimeOwner::NotImplemented,
            ),
        ],
    };

    let serialized =
        serde_json::to_value(matrix).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["matrixId"],
        enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER
    );
    assert_eq!(
        serialized["entries"][0]["capability"],
        enforcement::BROAD_CAPABILITY_OWNED_PROCESS_TERMINATE
    );
    assert_eq!(
        serialized["entries"][1]["readinessState"],
        enforcement::READINESS_MANUAL_REQUIRED
    );
    assert_eq!(
        serialized["entries"][2]["proofLevel"],
        enforcement::PROOF_NOT_PROVED
    );
    assert_eq!(
        serialized["entries"][2]["runtimeOwner"],
        enforcement::RUNTIME_OWNER_NOT_IMPLEMENTED
    );
}

#[test]
fn unsupported_readiness_state_does_not_deserialize() {
    let payload = serde_json::json!({
        "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
        "readinessId": enforcement::READINESS_ID_BROAD_APP_BLOCKING,
        "capability": enforcement::BROAD_CAPABILITY_BROAD_APP_BLOCKING,
        "platform": enforcement::PLATFORM_WINDOWS,
        "adapterKind": enforcement::ADAPTER_KIND_PROCESS_CONTROL,
        "capabilityState": enforcement::CAPABILITY_MANUAL_REQUIRED,
        "readinessState": "product-ready",
        "proofLevel": enforcement::PROOF_MANUAL_REQUIRED,
        "runtimeOwner": enforcement::RUNTIME_OWNER_MANUAL_PROOF,
        "supportedModes": [enforcement::MODE_BLOCK_PROCESS],
        "claimBoundary": enforcement::CLAIM_BOUNDARY_BROAD_APP_BLOCKING,
        "fallbackBehavior": enforcement::FALLBACK_BROAD_APP_BLOCKING,
        "requiredArtifacts": [enforcement::ARTIFACT_OS_APP_IDENTITY],
        "lastCheckedAt": policy::TEST_EVALUATED_AT
    });

    let parsed = serde_json::from_value::<EnforcementBroadAdapterReadinessEntry>(payload);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}
