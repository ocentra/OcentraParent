use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rusqlite::{params, Connection, ErrorCode, OptionalExtension, TransactionBehavior};

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceReceiptRef, ParentPresenceVerificationFailureReason,
};

const CHALLENGE_STATE_ISSUED: &str = "issued";
const CHALLENGE_STATE_CONSUMED: &str = "consumed";

const INITIALIZE_PARENT_PRESENCE_STORE: &str = r#"
PRAGMA journal_mode = WAL;
PRAGMA synchronous = FULL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS parent_presence_challenges (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    challenge_json TEXT NOT NULL,
    privileged_action_json TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    nonce_ref TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    )
) STRICT;

CREATE TABLE IF NOT EXISTS parent_presence_receipts (
    receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_ref TEXT NOT NULL UNIQUE,
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
) STRICT;
"#;

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

const INSERT_RECEIPT: &str = r#"
INSERT INTO parent_presence_receipts (challenge_ref)
VALUES (?1)
"#;

#[derive(Clone)]
pub(crate) struct ParentPresenceStore {
    path: PathBuf,
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
    Duplicate,
    Store(ParentPresenceStoreError),
}

#[derive(Clone, Copy)]
pub(crate) enum ParentPresenceStoreError {
    Unavailable,
    IntegrityRejected,
}

struct StoredChallengeRow {
    challenge_json: String,
    privileged_action_json: String,
    expires_at: String,
    nonce_ref: String,
    lifecycle_state: String,
}

impl ParentPresenceStore {
    pub(crate) fn open(path: impl Into<PathBuf>) -> Result<Self, ParentPresenceStoreError> {
        let path = path.into();
        create_parent_directory(&path)?;
        let store = Self { path };
        let connection = store.connection()?;
        connection
            .execute_batch(INITIALIZE_PARENT_PRESENCE_STORE)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        Ok(store)
    }

    pub(crate) fn issue_challenge(
        &self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceStoreIssueError> {
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
        let connection = self
            .connection()
            .map_err(ParentPresenceStoreIssueError::Store)?;
        match connection.execute(
            INSERT_CHALLENGE,
            params![
                challenge_ref,
                challenge_json,
                privileged_action_json,
                expires_at,
                nonce_ref,
            ],
        ) {
            Ok(_) => Ok(()),
            Err(error) if is_constraint_violation(&error) => {
                Err(ParentPresenceStoreIssueError::Duplicate)
            }
            Err(_error) => Err(ParentPresenceStoreIssueError::Store(
                ParentPresenceStoreError::Unavailable,
            )),
        }
    }

    pub(crate) fn consume_challenge(
        &self,
        challenge_ref: &str,
        validate: impl FnOnce(
            &ParentPresenceChallenge,
        ) -> Option<ParentPresenceVerificationFailureReason>,
    ) -> Result<ConsumeChallengeResult, ParentPresenceStoreError> {
        let mut connection = self.connection()?;
        let transaction = connection
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
        transaction
            .execute(INSERT_RECEIPT, params![challenge_ref])
            .map_err(|error| {
                if is_constraint_violation(&error) {
                    ParentPresenceStoreError::IntegrityRejected
                } else {
                    ParentPresenceStoreError::Unavailable
                }
            })?;
        let receipt_sequence = transaction.last_insert_rowid();
        transaction
            .commit()
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;

        Ok(ConsumeChallengeResult::Accepted(Box::new(
            ConsumedParentPresenceChallenge {
                receipt_ref: ParentPresenceReceiptRef::from_string(format!(
                    "parent-presence-receipt-{receipt_sequence}"
                )),
                challenge,
            },
        )))
    }

    fn connection(&self) -> Result<Connection, ParentPresenceStoreError> {
        let connection =
            Connection::open(&self.path).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        connection
            .busy_timeout(Duration::from_secs(10))
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        connection
            .execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        Ok(connection)
    }
}

fn create_parent_directory(path: &Path) -> Result<(), ParentPresenceStoreError> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    fs::create_dir_all(parent).map_err(|_error| ParentPresenceStoreError::Unavailable)
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

fn verified_challenge(
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

fn is_constraint_violation(error: &rusqlite::Error) -> bool {
    matches!(
        error,
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == ErrorCode::ConstraintViolation
    )
}
