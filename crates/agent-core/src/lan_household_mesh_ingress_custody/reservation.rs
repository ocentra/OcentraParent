use ocentra_lan_core::lan_pairing::signed_household_mesh_ingress::{
    replay_identity::LanHouseholdMeshDurableReplayIdentity,
    LanCryptographicallyVerifiedHouseholdMeshIngress,
};
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgeState, HouseholdMeshPolicyAuthority,
};
use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::LanSignedHouseholdMeshMessageType;
use ocentra_parent_agent_protocol::lan_pairing::{
    signed_household_mesh_ingress::transport::LanSignedHouseholdMeshTransportClaimDto,
    LanSignedChildAgentMessageKind,
};
use rusqlite::{params, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};

use super::{
    LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressAuthorizationScope,
    LanHouseholdMeshIngressCustodyError, LanHouseholdMeshIngressReceiptStore,
};
use crate::trusted_device_registry::signer_authority_types::LanRegisteredSignedChildAuthority;

#[path = "persistence_verification.rs"]
mod persistence_verification;
#[path = "sequence_progression.rs"]
mod sequence_progression;

const RECEIPT_DOMAIN: &[u8] = b"ocentra.lan.household-mesh.receipt.v1\0";

struct Candidate {
    receipt_id: String,
    family_hash: String,
    child_device_id: String,
    target_device_id: String,
    parent_device_id: String,
    signer_public_key_id: String,
    signer_public_key_sha256: String,
    message_kind: LanSignedChildAgentMessageKind,
    local_event_ref: String,
    lan_message_type: LanSignedHouseholdMeshMessageType,
    message_id: String,
    idempotency_key: String,
    route_id: String,
    nonce: String,
    sequence: i64,
    payload_digest: String,
    install_id: String,
    pairing_id: String,
    registry_proof_digest: String,
    authority_generation: i64,
    issued_at: String,
    expires_at: String,
    reserved_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredReceipt {
    receipt_id: String,
    family_hash: String,
    child_device_id: String,
    target_device_id: String,
    parent_device_id: String,
    signer_public_key_id: String,
    signer_public_key_sha256: String,
    message_kind: String,
    local_event_ref: String,
    lan_message_type: String,
    message_id: String,
    idempotency_key: String,
    route_id: String,
    nonce: String,
    sequence: i64,
    payload_digest: String,
    install_id: String,
    pairing_id: String,
    registry_proof_digest: String,
    authority_generation: i64,
    issued_at: String,
    expires_at: String,
    reserved_at: String,
}

pub(super) fn reserve_selected_event_republish(
    store: &mut LanHouseholdMeshIngressReceiptStore,
    ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
    authority: &LanRegisteredSignedChildAuthority,
    current_authority: &LanRegisteredSignedChildAuthority,
) -> Result<LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressCustodyError> {
    if !same_authority(authority, current_authority) {
        return Err(LanHouseholdMeshIngressCustodyError::AuthorityStale);
    }
    let candidate = candidate_for(ingress, current_authority)?;
    let tx = store
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(storage_error)?;
    if let Some(existing) = find_conflicts(&tx, &candidate)? {
        return Err(classify_conflict(&existing, &candidate));
    }
    sequence_progression::enforce_monotonic_sequence(&tx, &candidate)?;

    let inserted = tx.execute(
        "INSERT INTO lan_household_mesh_ingress_receipts_v2 (receipt_id,family_hash,child_device_id,target_device_id,parent_device_id,signer_public_key_id,signer_public_key_sha256,message_kind,local_event_ref,lan_message_type,message_id,idempotency_key,route_id,nonce,sequence,payload_digest,install_id,pairing_id,registry_proof_digest,authority_generation,issued_at,expires_at,reserved_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23)",
        params![
            candidate.receipt_id,
            candidate.family_hash,
            candidate.child_device_id,
            candidate.target_device_id,
            candidate.parent_device_id,
            candidate.signer_public_key_id,
            candidate.signer_public_key_sha256,
            message_kind_value(&candidate.message_kind),
            candidate.local_event_ref,
            candidate.lan_message_type.as_str(),
            candidate.message_id,
            candidate.idempotency_key,
            candidate.route_id,
            candidate.nonce,
            candidate.sequence,
            candidate.payload_digest,
            candidate.install_id,
            candidate.pairing_id,
            candidate.registry_proof_digest,
            candidate.authority_generation,
            candidate.issued_at,
            candidate.expires_at,
            candidate.reserved_at,
        ],
    );
    if inserted.is_err() {
        if let Some(existing) = find_conflicts(&tx, &candidate)? {
            return Err(classify_conflict(&existing, &candidate));
        }
        return Err(LanHouseholdMeshIngressCustodyError::StorageUnavailable);
    }
    tx.commit().map_err(storage_error)?;
    persistence_verification::verify_committed_receipt(
        &store.connection,
        candidate.receipt_id.as_str(),
        &candidate,
    )?;
    authorization_from_candidate(candidate)
}

fn same_authority(
    expected: &LanRegisteredSignedChildAuthority,
    current: &LanRegisteredSignedChildAuthority,
) -> bool {
    expected.pairing_id() == current.pairing_id()
        && expected.child_device_id() == current.child_device_id()
        && expected.target_device_id() == current.target_device_id()
        && expected.install_id() == current.install_id()
        && expected.family_hash() == current.family_hash()
        && expected.parent_device_id() == current.parent_device_id()
        && expected.route_id() == current.route_id()
        && expected.registry_proof_digest() == current.registry_proof_digest()
        && expected.message_kind() == current.message_kind()
        && expected.message_id() == current.message_id()
        && expected.idempotency_key() == current.idempotency_key()
        && expected.nonce() == current.nonce()
        && expected.sequence() == current.sequence()
        && expected.authority_generation() == current.authority_generation()
        && expected.public_key_id() == current.public_key_id()
        && expected.public_key_sha256() == current.public_key_sha256()
}

fn candidate_for(
    ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
    authority: &LanRegisteredSignedChildAuthority,
) -> Result<Candidate, LanHouseholdMeshIngressCustodyError> {
    let claim = ingress.claim();
    let payload = ingress.payload();
    let identity = ingress.durable_replay_identity();
    validate_candidate_identity(&identity, authority)?;
    validate_candidate_payload(payload, claim)?;
    let sequence = i64::try_from(identity.sequence().value())
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    let authority_generation = i64::try_from(authority.authority_generation())
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    validate_candidate_fields(claim)?;
    let reserved_at = chrono::Utc::now().to_rfc3339();
    let issued_at = chrono::DateTime::parse_from_rfc3339(claim.issued_at.as_str())
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    let expires_at = chrono::DateTime::parse_from_rfc3339(claim.expires_at.as_str())
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    let reserved_at_value = chrono::DateTime::parse_from_rfc3339(&reserved_at)
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    (expires_at > issued_at && reserved_at_value >= issued_at && reserved_at_value < expires_at)
        .then_some(())
        .ok_or(LanHouseholdMeshIngressCustodyError::AuthorityStale)?;
    let receipt_id = receipt_id_for(&identity, authority.authority_generation());
    Ok(Candidate {
        receipt_id,
        family_hash: identity.family_hash().to_string(),
        child_device_id: identity.child_device_id().to_string(),
        target_device_id: identity.target_device_id().to_string(),
        parent_device_id: identity.parent_device_id().to_string(),
        signer_public_key_id: identity.signer_public_key_id().to_string(),
        signer_public_key_sha256: identity.signer_public_key_sha256().to_string(),
        message_kind: identity.message_kind(),
        local_event_ref: identity.local_event_ref().to_string(),
        lan_message_type: identity.lan_message_type(),
        message_id: identity.message_id().to_string(),
        idempotency_key: identity.idempotency_key().to_string(),
        route_id: identity.route_id().to_string(),
        nonce: identity.nonce().to_string(),
        sequence,
        payload_digest: identity.canonical_payload_sha256().to_string(),
        install_id: identity.install_id().to_string(),
        pairing_id: identity.pairing_id().to_string(),
        registry_proof_digest: identity.registry_proof_digest().to_string(),
        authority_generation,
        issued_at: claim.issued_at.as_str().to_string(),
        expires_at: claim.expires_at.as_str().to_string(),
        reserved_at,
    })
}

fn validate_candidate_identity(
    identity: &LanHouseholdMeshDurableReplayIdentity<'_>,
    authority: &LanRegisteredSignedChildAuthority,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    if authority.authority_generation() == 0
        || identity.family_hash() != authority.family_hash()
        || identity.child_device_id() != authority.child_device_id()
        || identity.target_device_id() != authority.target_device_id()
        || identity.parent_device_id() != authority.parent_device_id()
        || identity.message_kind() != authority.message_kind()
        || identity.install_id() != authority.install_id()
        || identity.route_id() != authority.route_id()
        || identity.pairing_id() != authority.pairing_id()
        || identity.registry_proof_digest() != authority.registry_proof_digest()
        || identity.signer_public_key_id() != authority.public_key_id()
        || identity.signer_public_key_sha256() != authority.public_key_sha256()
        || identity.message_id() != authority.message_id()
        || identity.idempotency_key() != authority.idempotency_key()
        || identity.nonce() != authority.nonce()
        || identity.sequence().value() != authority.sequence()
    {
        return Err(LanHouseholdMeshIngressCustodyError::IdentityMismatch);
    }
    Ok(())
}

fn validate_candidate_payload(
    payload: &ocentra_parent_agent_protocol::household_mesh::HouseholdMeshTransportEnvelope,
    claim: &LanSignedHouseholdMeshTransportClaimDto,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    if payload.message_id != claim.message_id.as_str()
        || payload.idempotency_key != claim.idempotency_key.as_str()
        || payload.family_id != claim.family_hash.as_str()
        || payload.source_peer_id != claim.child_device_id.as_str()
        || payload.target_child_device_id != claim.target_device_id.as_str()
        || payload.bridge_state != HouseholdMeshBridgeState::ExportSelected
        || payload.authentication_state != HouseholdMeshAuthenticationState::PairedTrustedDevice
        || payload.policy_authority != HouseholdMeshPolicyAuthority::ChildAgentOnly
        || payload.direct_remote_publish_requested
        || payload.raw_payload_included
    {
        return Err(LanHouseholdMeshIngressCustodyError::IdentityMismatch);
    }
    Ok(())
}

fn validate_candidate_fields(
    claim: &LanSignedHouseholdMeshTransportClaimDto,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    if [
        claim.family_hash.as_str(),
        claim.child_device_id.as_str(),
        claim.message_id.as_str(),
        claim.idempotency_key.as_str(),
        claim.route_id.as_str(),
        claim.nonce.as_str(),
        claim.canonical_payload_sha256.as_str(),
        claim.install_id.as_str(),
        claim.pairing_id.as_str(),
        claim.registry_proof_digest.as_str(),
        claim.issued_at.as_str(),
    ]
    .iter()
    .any(|value| value.trim().is_empty())
    {
        return Err(LanHouseholdMeshIngressCustodyError::InvalidInput);
    }
    Ok(())
}

fn receipt_id_for(
    identity: &LanHouseholdMeshDurableReplayIdentity<'_>,
    authority_generation: u64,
) -> String {
    let mut digest = Sha256::new();
    digest.update(RECEIPT_DOMAIN);
    for value in [
        identity.family_hash(),
        identity.child_device_id(),
        identity.target_device_id(),
        identity.parent_device_id(),
        identity.signer_public_key_id(),
        identity.signer_public_key_sha256(),
        message_kind_value(&identity.message_kind()),
        identity.local_event_ref(),
        identity.lan_message_type().as_str(),
        identity.message_id(),
        identity.idempotency_key(),
        identity.route_id(),
        identity.nonce(),
        identity.canonical_payload_sha256(),
        identity.install_id(),
        identity.pairing_id(),
        identity.registry_proof_digest(),
    ] {
        digest.update(value.as_bytes());
        digest.update([0]);
    }
    digest.update(identity.sequence().value().to_string().as_bytes());
    digest.update([0]);
    digest.update(authority_generation.to_string().as_bytes());
    format!("lan-receipt-{:x}", digest.finalize())
}

fn message_kind_value(kind: &LanSignedChildAgentMessageKind) -> &'static str {
    match kind {
        LanSignedChildAgentMessageKind::Hello => "hello",
        LanSignedChildAgentMessageKind::Heartbeat => "heartbeat",
    }
}

fn find_conflicts(
    tx: &Transaction<'_>,
    candidate: &Candidate,
) -> Result<Option<Vec<StoredReceipt>>, LanHouseholdMeshIngressCustodyError> {
    let mut statement = tx
        .prepare("SELECT receipt_id,family_hash,child_device_id,target_device_id,parent_device_id,signer_public_key_id,signer_public_key_sha256,message_kind,local_event_ref,lan_message_type,message_id,idempotency_key,route_id,nonce,sequence,payload_digest,install_id,pairing_id,registry_proof_digest,authority_generation,issued_at,expires_at,reserved_at FROM lan_household_mesh_ingress_receipts_v2 WHERE (family_hash=?1 AND child_device_id=?2 AND message_id=?3) OR (family_hash=?1 AND child_device_id=?2 AND idempotency_key=?4) OR (family_hash=?1 AND child_device_id=?2 AND pairing_id=?5 AND authority_generation=?6 AND signer_public_key_sha256=?7 AND route_id=?8 AND nonce=?9 AND sequence=?10) OR receipt_id=?11")
        .map_err(storage_error)?;
    let rows = statement
        .query_map(
            params![
                candidate.family_hash.as_str(),
                candidate.child_device_id.as_str(),
                candidate.message_id.as_str(),
                candidate.idempotency_key.as_str(),
                candidate.pairing_id.as_str(),
                candidate.authority_generation,
                candidate.signer_public_key_sha256.as_str(),
                candidate.route_id.as_str(),
                candidate.nonce.as_str(),
                candidate.sequence,
                candidate.receipt_id.as_str(),
            ],
            stored_receipt_from_row,
        )
        .map_err(storage_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_error)?;
    Ok((!rows.is_empty()).then_some(rows))
}

fn stored_receipt_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredReceipt> {
    Ok(StoredReceipt {
        receipt_id: row.get(0)?,
        family_hash: row.get(1)?,
        child_device_id: row.get(2)?,
        target_device_id: row.get(3)?,
        parent_device_id: row.get(4)?,
        signer_public_key_id: row.get(5)?,
        signer_public_key_sha256: row.get(6)?,
        message_kind: row.get(7)?,
        local_event_ref: row.get(8)?,
        lan_message_type: row.get(9)?,
        message_id: row.get(10)?,
        idempotency_key: row.get(11)?,
        route_id: row.get(12)?,
        nonce: row.get(13)?,
        sequence: row.get(14)?,
        payload_digest: row.get(15)?,
        install_id: row.get(16)?,
        pairing_id: row.get(17)?,
        registry_proof_digest: row.get(18)?,
        authority_generation: row.get(19)?,
        issued_at: row.get(20)?,
        expires_at: row.get(21)?,
        reserved_at: row.get(22)?,
    })
}

fn classify_conflict(
    existing: &[StoredReceipt],
    candidate: &Candidate,
) -> LanHouseholdMeshIngressCustodyError {
    let same = existing.len() == 1 && same_receipt(&existing[0], candidate);
    if same {
        LanHouseholdMeshIngressCustodyError::DuplicateReceipt {
            receipt_id: existing[0].receipt_id.clone(),
        }
    } else {
        LanHouseholdMeshIngressCustodyError::ReconciliationRequired {
            receipt_id: existing[0].receipt_id.clone(),
        }
    }
}

fn same_receipt(existing: &StoredReceipt, candidate: &Candidate) -> bool {
    existing.receipt_id == candidate.receipt_id
        && existing.family_hash == candidate.family_hash
        && existing.child_device_id == candidate.child_device_id
        && existing.target_device_id == candidate.target_device_id
        && existing.parent_device_id == candidate.parent_device_id
        && existing.signer_public_key_id == candidate.signer_public_key_id
        && existing.signer_public_key_sha256 == candidate.signer_public_key_sha256
        && existing.message_kind == message_kind_value(&candidate.message_kind)
        && existing.local_event_ref == candidate.local_event_ref
        && existing.lan_message_type == candidate.lan_message_type.as_str()
        && existing.message_id == candidate.message_id
        && existing.idempotency_key == candidate.idempotency_key
        && existing.route_id == candidate.route_id
        && existing.nonce == candidate.nonce
        && existing.sequence == candidate.sequence
        && existing.payload_digest == candidate.payload_digest
        && existing.install_id == candidate.install_id
        && existing.pairing_id == candidate.pairing_id
        && existing.registry_proof_digest == candidate.registry_proof_digest
        && existing.authority_generation == candidate.authority_generation
        && existing.issued_at == candidate.issued_at
        && existing.expires_at == candidate.expires_at
}

fn authorization_from_candidate(
    candidate: Candidate,
) -> Result<LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressCustodyError> {
    let sequence = u64::try_from(candidate.sequence)
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    let authority_generation = u64::try_from(candidate.authority_generation)
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?;
    Ok(LanHouseholdMeshIngressAuthorization {
        receipt_id: candidate.receipt_id,
        scope: LanHouseholdMeshIngressAuthorizationScope::SelectedEventRepublish,
        family_hash: candidate.family_hash,
        child_device_id: candidate.child_device_id,
        target_device_id: candidate.target_device_id,
        parent_device_id: candidate.parent_device_id,
        signer_public_key_id: candidate.signer_public_key_id,
        signer_public_key_sha256: candidate.signer_public_key_sha256,
        message_kind: candidate.message_kind,
        local_event_ref: candidate.local_event_ref,
        lan_message_type: candidate.lan_message_type,
        route_id: candidate.route_id,
        message_id: candidate.message_id,
        idempotency_key: candidate.idempotency_key,
        nonce: candidate.nonce,
        sequence,
        payload_digest: candidate.payload_digest,
        install_id: candidate.install_id,
        pairing_id: candidate.pairing_id,
        registry_proof_digest: candidate.registry_proof_digest,
        authority_generation,
        issued_at: candidate.issued_at,
        expires_at: candidate.expires_at,
        reserved_at: candidate.reserved_at,
    })
}

fn storage_error(_error: rusqlite::Error) -> LanHouseholdMeshIngressCustodyError {
    LanHouseholdMeshIngressCustodyError::StorageUnavailable
}
