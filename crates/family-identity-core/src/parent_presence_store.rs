use std::path::PathBuf;

use rusqlite::{params, Connection, ErrorCode, OptionalExtension, TransactionBehavior};

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceObservedAt, ParentPresenceReceiptRef,
    ParentPresenceVerificationFailureReason,
};
use crate::parent_presence_event_delivery::PendingCustodyDecision;
use crate::parent_presence_store_file::StoreFileGuard;
use crate::parent_presence_store_integrity::verified_challenge;
use crate::parent_presence_store_path::validate_caller_custody_path;
use crate::parent_presence_store_receipt::verify_consumed_receipt;
use crate::parent_presence_store_schema::open_initialized_store;

#[path = "parent_presence_store_outbox.rs"]
mod outbox;
#[path = "parent_presence_store_step_up.rs"]
mod step_up;

const CHALLENGE_STATE_ISSUED: &str = "issued";
const CHALLENGE_STATE_CONSUMED: &str = "consumed";

const INSERT_CHALLENGE: &str = r#"
INSERT INTO parent_presence_challenges (
    challenge_ref,
    challenge_json,
    privileged_action_json,
    expires_at,
    nonce_ref,
    lifecycle_state
) VALUES (?1, ?2, ?3, ?4, ?5, 'issued')
"#;

const SELECT_CHALLENGE: &str = r#"
SELECT
    challenge_json,
    privileged_action_json,
    expires_at,
    nonce_ref,
    lifecycle_state
FROM parent_presence_challenges
WHERE challenge_ref = ?1
"#;

const MARK_CHALLENGE_CONSUMED: &str = r#"
UPDATE parent_presence_challenges
SET lifecycle_state = 'consumed'
WHERE challenge_ref = ?1 AND lifecycle_state = 'issued'
"#;

const INSERT_PENDING_DECISION: &str = r#"
INSERT INTO parent_presence_decision_outbox (
    decision_id,
    envelope_json,
    delivery_state
) VALUES (?1, ?2, 'pending')
"#;

pub(crate) struct ParentPresenceStore {
    connection: Connection,
    _file_guard: StoreFileGuard,
}

pub(crate) struct ConsumedParentPresenceChallenge {
    pub(crate) receipt_ref: ParentPresenceReceiptRef,
    pub(crate) challenge: ParentPresenceChallenge,
}

pub(crate) enum ConsumeChallengeResult {
    Accepted(Box<ConsumedParentPresenceChallenge>),
    Rejected(ParentPresenceVerificationFailureReason),
}

pub(crate) enum ParentPresenceStoreIssueError {
    TimestampInvalid,
    DuplicateChallenge,
    DuplicateNonce,
    Store(ParentPresenceStoreError),
}

#[derive(Clone, Copy)]
pub(crate) enum ParentPresenceStoreError {
    Unavailable,
    IntegrityRejected,
}

pub(crate) struct StoredChallengeRow {
    pub(crate) challenge_json: String,
    pub(crate) privileged_action_json: String,
    pub(crate) expires_at: String,
    pub(crate) nonce_ref: String,
    pub(crate) lifecycle_state: String,
}

pub(crate) struct StoredParentStepUpIntent {
    pub(crate) challenge_ref: String,
    pub(crate) nonce_ref: String,
    pub(crate) intent_digest: String,
    pub(crate) family_id: String,
    pub(crate) trust_subject: String,
    pub(crate) parent_account_id: String,
    pub(crate) parent_device_id: String,
    pub(crate) child_device_id: String,
    pub(crate) installation_id: String,
    pub(crate) pairing_id: String,
    pub(crate) route_id: String,
    pub(crate) signer_public_key: Vec<u8>,
    pub(crate) lifecycle_generation: i64,
    pub(crate) installation_binding_generation: i64,
    pub(crate) authority_generation: i64,
    pub(crate) correlation_id: String,
    pub(crate) expires_at: String,
}

impl ParentPresenceStore {
    pub(crate) fn open(path: impl Into<PathBuf>) -> Result<Self, ParentPresenceStoreError> {
        let path = path.into();
        validate_caller_custody_path(&path)?;
        let (connection, file_guard) = open_initialized_store(&path)?;
        Ok(Self {
            connection,
            _file_guard: file_guard,
        })
    }

    pub(crate) fn issue_challenge(
        &mut self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceStoreIssueError> {
        self.issue_challenge_inner(challenge, None)
    }

    fn issue_challenge_inner(
        &mut self,
        challenge: ParentPresenceChallenge,
        intent: Option<StoredParentStepUpIntent>,
    ) -> Result<(), ParentPresenceStoreIssueError> {
        ParentPresenceObservedAt::from_canonical_utc(&challenge.expires_at)
            .map_err(|_error| ParentPresenceStoreIssueError::TimestampInvalid)?;
        let challenge_json = serde_json::to_string(&challenge).map_err(|_error| {
            ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
        })?;
        let privileged_action_json =
            serde_json::to_string(&challenge.privileged_action).map_err(|_error| {
                ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
            })?;
        let ParentPresenceChallenge {
            challenge_ref,
            expires_at,
            nonce_ref,
            ..
        } = challenge;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| {
                ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
            })?;
        let duplicate = transaction
            .query_row(
                "SELECT challenge_ref, nonce_ref FROM parent_presence_challenges WHERE challenge_ref = ?1 OR nonce_ref = ?2 LIMIT 1",
                params![challenge_ref, nonce_ref],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
            .map_err(|_error| ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable))?;
        if let Some((stored_challenge_ref, _stored_nonce_ref)) = duplicate {
            return if stored_challenge_ref == challenge_ref {
                Err(ParentPresenceStoreIssueError::DuplicateChallenge)
            } else {
                Err(ParentPresenceStoreIssueError::DuplicateNonce)
            };
        }
        transaction
            .execute(
                INSERT_CHALLENGE,
                params![
                    challenge_ref,
                    challenge_json,
                    privileged_action_json,
                    expires_at,
                    nonce_ref,
                ],
            )
            .map_err(|error| {
                if is_constraint_violation(&error) {
                    ParentPresenceStoreIssueError::DuplicateNonce
                } else {
                    ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
                }
            })?;
        intent
            .map(|intent| step_up::insert_intent(&transaction, intent))
            .transpose()
            .map_err(|_error| {
                ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
            })?;
        transaction.commit().map_err(|_error| {
            ParentPresenceStoreIssueError::Store(ParentPresenceStoreError::Unavailable)
        })
    }

    pub(crate) fn consume_challenge(
        &mut self,
        challenge_ref: &str,
        accepted_decision: &PendingCustodyDecision,
        validate: impl FnOnce(
            &ParentPresenceChallenge,
        ) -> Option<ParentPresenceVerificationFailureReason>,
        verified_credential: Option<(&str, i32, u32)>,
    ) -> Result<ConsumeChallengeResult, ParentPresenceStoreError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        let stored = transaction
            .query_row(
                SELECT_CHALLENGE,
                params![challenge_ref],
                stored_challenge_row,
            )
            .optional()
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        let Some(stored) = stored else {
            return Ok(ConsumeChallengeResult::Rejected(
                ParentPresenceVerificationFailureReason::ChallengeNotIssued,
            ));
        };
        if stored.lifecycle_state == CHALLENGE_STATE_CONSUMED {
            verified_challenge(challenge_ref, &stored)?;
            verify_consumed_receipt(&transaction, challenge_ref)?;
            return Ok(ConsumeChallengeResult::Rejected(
                ParentPresenceVerificationFailureReason::ReplayRejected,
            ));
        }
        if stored.lifecycle_state != CHALLENGE_STATE_ISSUED {
            return Err(ParentPresenceStoreError::IntegrityRejected);
        }

        let challenge = verified_challenge(challenge_ref, &stored)?;
        if let Some(failure_reason) = validate(&challenge) {
            return Ok(ConsumeChallengeResult::Rejected(failure_reason));
        }

        let changed = transaction
            .execute(MARK_CHALLENGE_CONSUMED, params![challenge_ref])
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if changed != 1 {
            return Err(ParentPresenceStoreError::IntegrityRejected);
        }
        let receipt_ref = step_up::insert_receipt_and_mark(
            &transaction,
            challenge_ref,
            &challenge.privileged_action,
            &challenge.nonce_ref,
            verified_credential,
        )?;
        transaction
            .execute(
                INSERT_PENDING_DECISION,
                params![
                    accepted_decision.decision_id.as_str(),
                    accepted_decision.envelope_json.as_str()
                ],
            )
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        transaction
            .commit()
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;

        Ok(ConsumeChallengeResult::Accepted(Box::new(
            ConsumedParentPresenceChallenge {
                receipt_ref: ParentPresenceReceiptRef::from_string(receipt_ref),
                challenge,
            },
        )))
    }
}

fn stored_challenge_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredChallengeRow> {
    Ok(StoredChallengeRow {
        challenge_json: row.get(0)?,
        privileged_action_json: row.get(1)?,
        expires_at: row.get(2)?,
        nonce_ref: row.get(3)?,
        lifecycle_state: row.get(4)?,
    })
}

fn is_constraint_violation(error: &rusqlite::Error) -> bool {
    matches!(
        error,
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == ErrorCode::ConstraintViolation
    )
}
