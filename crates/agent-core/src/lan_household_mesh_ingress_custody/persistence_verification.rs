use rusqlite::{params, Connection, OptionalExtension};

use super::{
    same_receipt, stored_receipt_from_row, Candidate, LanHouseholdMeshIngressCustodyError,
};

pub(super) fn verify_committed_receipt(
    connection: &Connection,
    receipt_id: &str,
    candidate: &Candidate,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let persisted = connection
        .query_row(
            "SELECT receipt_id,family_hash,child_device_id,target_device_id,parent_device_id,signer_public_key_id,signer_public_key_sha256,message_kind,local_event_ref,lan_message_type,message_id,idempotency_key,route_id,nonce,sequence,payload_digest,install_id,pairing_id,registry_proof_digest,authority_generation,issued_at,expires_at,reserved_at FROM lan_household_mesh_ingress_receipts_v2 WHERE receipt_id=?1",
            params![receipt_id],
            stored_receipt_from_row,
        )
        .optional()
        .map_err(storage_error)?
        .ok_or(LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
    if !same_receipt(&persisted, candidate) || persisted.reserved_at != candidate.reserved_at {
        return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
    }
    Ok(())
}

fn storage_error(_error: rusqlite::Error) -> LanHouseholdMeshIngressCustodyError {
    LanHouseholdMeshIngressCustodyError::StorageUnavailable
}
