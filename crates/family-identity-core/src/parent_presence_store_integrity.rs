use crate::parent_presence::ParentPresenceChallenge;
use crate::parent_presence_store::{ParentPresenceStoreError, StoredChallengeRow};

pub(crate) fn validate_store_schema(
    connection: &rusqlite::Connection,
) -> Result<(), ParentPresenceStoreError> {
    let receipt_ref_column_count = connection
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('parent_presence_receipts') WHERE name = 'receipt_ref'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    if receipt_ref_column_count != 1 {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    Ok(())
}

pub(crate) fn verified_challenge(
    expected_challenge_ref: &str,
    stored: &StoredChallengeRow,
) -> Result<ParentPresenceChallenge, ParentPresenceStoreError> {
    let challenge = serde_json::from_str::<ParentPresenceChallenge>(&stored.challenge_json)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let privileged_action_json = serde_json::to_string(&challenge.privileged_action)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    if challenge.challenge_ref != expected_challenge_ref
        || privileged_action_json != stored.privileged_action_json
        || challenge.expires_at != stored.expires_at
        || challenge.nonce_ref != stored.nonce_ref
    {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    Ok(challenge)
}
