use rusqlite::{params, Transaction};

use super::{Candidate, LanHouseholdMeshIngressCustodyError};

pub(super) fn enforce_monotonic_sequence(
    transaction: &Transaction<'_>,
    candidate: &Candidate,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let highest_sequence = transaction
        .query_row(
            "SELECT MAX(sequence) FROM lan_household_mesh_ingress_receipts_v2 WHERE family_hash=?1 AND child_device_id=?2 AND target_device_id=?3 AND parent_device_id=?4 AND signer_public_key_id=?5 AND signer_public_key_sha256=?6 AND install_id=?7 AND pairing_id=?8 AND registry_proof_digest=?9 AND authority_generation=?10 AND route_id=?11",
            params![
                candidate.family_hash.as_str(),
                candidate.child_device_id.as_str(),
                candidate.target_device_id.as_str(),
                candidate.parent_device_id.as_str(),
                candidate.signer_public_key_id.as_str(),
                candidate.signer_public_key_sha256.as_str(),
                candidate.install_id.as_str(),
                candidate.pairing_id.as_str(),
                candidate.registry_proof_digest.as_str(),
                candidate.authority_generation,
                candidate.route_id.as_str(),
            ],
            |row| row.get::<_, Option<i64>>(0),
        )
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::StorageUnavailable)?;
    if highest_sequence.is_some_and(|highest| candidate.sequence <= highest) {
        return Err(LanHouseholdMeshIngressCustodyError::SequenceRegression);
    }
    Ok(())
}
