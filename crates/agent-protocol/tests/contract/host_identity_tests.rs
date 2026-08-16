use super::{
    constants, constants::host_identity, policy_constants as policy, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementCapabilityState, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, HostIdentityEvidenceClass,
    HostIdentityEvidenceKind, HostIdentityReadModel, HostIdentityReadModelEntry, ParentPlatform,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn host_identity_read_model_serializes_evidence_boundaries() {
    let read_model_entry = |read_model_entry_id: &'static str,
                            evidence_kind,
                            evidence_class,
                            readiness_state,
                            proof_level,
                            runtime_owner| {
        let capability_state = match readiness_state {
            EnforcementReadinessState::Implemented => EnforcementCapabilityState::Supported,
            EnforcementReadinessState::ManualRequired | EnforcementReadinessState::NotClaimed => {
                EnforcementCapabilityState::ManualRequired
            }
            EnforcementReadinessState::Unavailable => EnforcementCapabilityState::Unavailable,
        };

        HostIdentityReadModelEntry {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            read_model_entry_id: read_model_entry_id.to_string(),
            evidence_kind,
            evidence_class,
            capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            capability_state,
            readiness_state,
            proof_level,
            runtime_owner,
            host_evidence_requirement: host_identity::REQUIREMENT_INSTALLED_APP_INVENTORY
                .to_string(),
            required_evidence_artifacts: host_identity::ARTIFACTS_INSTALLED_APP_INVENTORY
                .iter()
                .map(|artifact| (*artifact).to_string())
                .collect(),
            acceptance_signals: host_identity::SIGNALS_INSTALLED_APP_INVENTORY
                .iter()
                .map(|signal| (*signal).to_string())
                .collect(),
            fallback_behavior: host_identity::FALLBACK_INSTALLED_APP_INVENTORY.to_string(),
            safe_for_broad_app_blocking: false,
            last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
        }
    };
    let model = HostIdentityReadModel {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: host_identity::READ_MODEL_ID_V0_8.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        platform: ParentPlatform::Windows,
        entries: vec![
            read_model_entry(
                host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY,
                HostIdentityEvidenceKind::InstalledAppInventory,
                HostIdentityEvidenceClass::Inventory,
                EnforcementReadinessState::ManualRequired,
                EnforcementReadinessProofLevel::ManualProofRequired,
                EnforcementReadinessRuntimeOwner::ManualProof,
            ),
            read_model_entry(
                host_identity::ENTRY_ID_UNSUPPORTED_IDENTITY,
                HostIdentityEvidenceKind::UnsupportedIdentity,
                HostIdentityEvidenceClass::Package,
                EnforcementReadinessState::Unavailable,
                EnforcementReadinessProofLevel::ManualProofRequired,
                EnforcementReadinessRuntimeOwner::ManualProof,
            ),
            read_model_entry(
                host_identity::ENTRY_ID_ROLLBACK_READINESS,
                HostIdentityEvidenceKind::RollbackReadiness,
                HostIdentityEvidenceClass::Rollback,
                EnforcementReadinessState::NotClaimed,
                EnforcementReadinessProofLevel::NotProved,
                EnforcementReadinessRuntimeOwner::NotImplemented,
            ),
        ],
    };

    let serialized =
        serde_json::to_value(model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["readModelId"], host_identity::READ_MODEL_ID_V0_8);
    assert_eq!(
        serialized["entries"][0]["evidenceKind"],
        host_identity::KIND_INSTALLED_APP_INVENTORY
    );
    assert_eq!(
        serialized["entries"][1]["readinessState"],
        constants::enforcement::READINESS_UNAVAILABLE
    );
    assert_eq!(
        serialized["entries"][2]["proofLevel"],
        constants::enforcement::PROOF_NOT_PROVED
    );
    assert_eq!(serialized["entries"][2]["safeForBroadAppBlocking"], false);
}

#[test]
fn host_identity_rejects_unknown_evidence_kind() {
    let payload = serde_json::json!({
        "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
        "readModelEntryId": host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY,
        "evidenceKind": "global-app-blocking-proof",
        "evidenceClass": host_identity::CLASS_INVENTORY,
        "capability": constants::enforcement::BROAD_CAPABILITY_BROAD_APP_BLOCKING,
        "platform": constants::enforcement::PLATFORM_WINDOWS,
        "adapterKind": constants::enforcement::ADAPTER_KIND_PROCESS_CONTROL,
        "capabilityState": constants::enforcement::CAPABILITY_MANUAL_REQUIRED,
        "readinessState": constants::enforcement::READINESS_MANUAL_REQUIRED,
        "proofLevel": constants::enforcement::PROOF_MANUAL_REQUIRED,
        "runtimeOwner": constants::enforcement::RUNTIME_OWNER_MANUAL_PROOF,
        "hostEvidenceRequirement": host_identity::REQUIREMENT_INSTALLED_APP_INVENTORY,
        "requiredEvidenceArtifacts": host_identity::ARTIFACTS_INSTALLED_APP_INVENTORY,
        "acceptanceSignals": host_identity::SIGNALS_INSTALLED_APP_INVENTORY,
        "fallbackBehavior": host_identity::FALLBACK_INSTALLED_APP_INVENTORY,
        "safeForBroadAppBlocking": false,
        "lastCheckedAt": policy::TEST_EVALUATED_AT
    });

    let parsed = serde_json::from_value::<HostIdentityReadModelEntry>(payload);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}
