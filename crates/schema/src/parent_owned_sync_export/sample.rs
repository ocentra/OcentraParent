use super::identifiers::{
    child_profile_id, contract_version, family_id, manifest_id, parent_action_id, parent_actor_id,
    parent_device_id, parent_device_label, parent_policy_version, parent_timestamp, version_label,
};
use super::*;

pub(super) fn parent_owned_sync_export_known_gaps() -> [&'static str; 6] {
    [
        KNOWN_GAP_NO_PROVIDER_OAUTH_RUNTIME,
        KNOWN_GAP_NO_PROVIDER_UPLOAD_DELETE_RUNTIME,
        KNOWN_GAP_PARENT_SYNC_REMAINS_SEPARATE,
        KNOWN_GAP_MANIFEST_INTEGRITY_ONLY_CONTRACT_EVIDENCE,
        KNOWN_GAP_TOMBSTONE_PROPAGATION_MODELED_SEPARATELY,
        KNOWN_GAP_OCENTRA_NOT_DEFAULT_EVIDENCE_STORE,
    ]
}

pub(super) fn required_parent_owned_sync_export_non_claims() -> Vec<ParentOwnedSyncExportNonClaim> {
    vec![
        ParentOwnedSyncExportNonClaim::NoTransferRuntime,
        ParentOwnedSyncExportNonClaim::NoConnectorOAuth,
        ParentOwnedSyncExportNonClaim::NoUploadRuntime,
        ParentOwnedSyncExportNonClaim::NoDeleteRuntime,
        ParentOwnedSyncExportNonClaim::NoDefaultOcentraCustody,
        ParentOwnedSyncExportNonClaim::NoRawChildEvidenceUploadByDefault,
    ]
}

pub(super) fn sample_parent_owned_sync_export_contract_proof() -> ParentOwnedSyncExportContractProof
{
    let timestamp = parent_timestamp("2026-06-28T18:40:00.000Z".to_string());
    let family = FamilyReference {
        family_id: family_id("family-parent-owned-sync-proof-1".to_string()),
    };
    let device = ParentDeviceReference {
        device_id: parent_device_id("windows-parent-owned-sync-proof-1".to_string()),
        child_profile_id: Some(child_profile_id(
            "child-parent-owned-sync-proof-1".to_string(),
        )),
        label: parent_device_label("Windows parent-owned sync proof device".to_string()),
        platform: ParentPlatform::Windows,
    };
    let parent_action = ParentActionReference {
        action_reference_id: parent_action_id(
            "parent-action-parent-owned-sync-proof-1".to_string(),
        ),
        actor: ParentActorReference {
            actor_id: parent_actor_id("parent-owned-sync-proof-actor-1".to_string()),
            role: ParentActorRole::Parent,
        },
        policy_version: parent_policy_version("parent-owned-sync-proof-policy-v1".to_string()),
        created_at: timestamp.clone(),
    };
    let manifest = ParentOwnedSyncExportManifest {
        schema_version: PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION.to_string(),
        manifest_id: manifest_id("parent-owned-sync-manifest-proof-1".to_string()),
        family,
        device,
        parent_action,
        product_version: version_label("0.1.1".to_string()),
        manifest_version: version_label("parent-owned-sync.manifest.v1".to_string()),
        generated_at: timestamp.clone(),
        items: manifest::sample_manifest_items(),
    };

    ParentOwnedSyncExportContractProof {
        schema_version: PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION.to_string(),
        contract_version: contract_version("v0.6".to_string()),
        manifest,
        provider_statuses: providers::sample_provider_statuses(&timestamp),
        sync_states: sync_states::sample_sync_states(),
        tombstones: tombstones::sample_tombstones(&timestamp),
        non_claims: required_parent_owned_sync_export_non_claims(),
        transfer_runtime_claimed: false,
        connector_o_auth_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_child_evidence_stored: false,
        updated_at: timestamp,
    }
}
