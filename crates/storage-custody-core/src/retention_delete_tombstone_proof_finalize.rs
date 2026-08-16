use ocentra_schema::retention_delete_tombstone as contracts;

use super::RetentionDeleteDerivationError;

pub(super) fn build_retention_delete_tombstone_proof_finalize(
    request: &contracts::RetentionDeleteRequest,
    rows: Vec<contracts::RetentionDeleteRow>,
    updated_at: contracts::RetentionDeleteTimestamp,
) -> Result<contracts::RetentionDeleteTombstoneContractProof, RetentionDeleteDerivationError> {
    validate_required_rows(&rows)?;

    Ok(contracts::RetentionDeleteTombstoneContractProof {
        schema_version: contracts::RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION.to_string(),
        contract_version: contracts::RetentionDeleteContractVersion::parse("v0.4")
            .ok_or(RetentionDeleteDerivationError::InvalidContractVersion)?,
        request: request.clone(),
        retention_matrix: contracts::retention_delete_policy_matrix(),
        rows,
        non_claims: contracts::required_retention_delete_non_claims(),
        report_runtime_claimed: false,
        notification_runtime_claimed: false,
        restore_runtime_claimed: false,
        ts_business_owner_claimed: false,
        updated_at,
    })
}

fn validate_required_rows(
    rows: &[contracts::RetentionDeleteRow],
) -> Result<(), RetentionDeleteDerivationError> {
    for required_state in contracts::required_retention_delete_states() {
        if !rows.iter().any(|row| row.state == required_state) {
            return Err(RetentionDeleteDerivationError::MissingRequiredState(
                required_state,
            ));
        }
    }
    Ok(())
}
