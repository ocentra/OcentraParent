use crate::parent_presence::ParentPresenceChallenge;
use crate::parent_presence_store::{ParentPresenceStoreError, StoredChallengeRow};

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
