use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};

use crate::parent_presence::ParentPresenceStorageFailureReason;
use crate::parent_presence_event_delivery::PendingCustodyDecision;

use super::{ParentPresenceStore, ParentPresenceStoreError, INSERT_PENDING_DECISION};

const CLAIM_STALE_AFTER_MILLIS: i64 = 300_000;

const SELECT_CLAIMABLE_DECISION: &str = r#"
SELECT decision_id, envelope_json
FROM parent_presence_decision_outbox
WHERE delivery_state = 'pending'
   OR (
        delivery_state = 'claimed'
    AND (
            delivery_claimed_at IS NULL
         OR delivery_claimed_at < ?1
    )
   )
ORDER BY rowid
LIMIT 1
"#;

const CLAIM_DECISION: &str = r#"
UPDATE parent_presence_decision_outbox
SET delivery_state = 'claimed', delivery_claim = ?1, delivery_claimed_at = ?2
WHERE decision_id = ?3
  AND (
        delivery_state = 'pending'
     OR (
            delivery_state = 'claimed'
        AND (
                delivery_claimed_at IS NULL
             OR delivery_claimed_at < ?4
        )
     )
  )
"#;

const MARK_DECISION_DELIVERED: &str = r#"
UPDATE parent_presence_decision_outbox
SET delivery_state = 'delivered', delivery_claim = NULL, delivery_claimed_at = NULL
WHERE decision_id = ?1 AND delivery_state = 'claimed' AND delivery_claim = ?2
"#;

const RELEASE_DECISION_CLAIM: &str = r#"
UPDATE parent_presence_decision_outbox
SET delivery_state = 'pending', delivery_claim = NULL, delivery_claimed_at = NULL
WHERE decision_id = ?1 AND delivery_state = 'claimed' AND delivery_claim = ?2
"#;

struct ClaimedCustodyDecision {
    decision: PendingCustodyDecision,
    claim: String,
}

impl ParentPresenceStore {
    pub(crate) fn enqueue_decision(
        &mut self,
        decision: &PendingCustodyDecision,
    ) -> Result<(), ParentPresenceStoreError> {
        let transaction = self.immediate_transaction()?;
        transaction
            .execute(
                INSERT_PENDING_DECISION,
                params![
                    decision.decision_id.as_str(),
                    decision.envelope_json.as_str()
                ],
            )
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        commit(transaction)
    }

    pub(crate) fn deliver_pending(
        &mut self,
        mut deliver: impl FnMut(
            &PendingCustodyDecision,
        ) -> Result<(), ParentPresenceStorageFailureReason>,
    ) -> Result<(), ParentPresenceStoreError> {
        while let Some(claimed) = {
            let transaction = self.immediate_transaction()?;
            let claimed = claim_next(&transaction)?;
            commit(transaction)?;
            claimed
        } {
            if deliver(&claimed.decision).is_err() {
                let transaction = self.immediate_transaction()?;
                release_claim(&transaction, &claimed)?;
                commit(transaction)?;
                return Err(ParentPresenceStoreError::Unavailable);
            }
            let transaction = self.immediate_transaction()?;
            mark_delivered(&transaction, &claimed)?;
            commit(transaction)?;
        }
        Ok(())
    }

    fn immediate_transaction(&mut self) -> Result<Transaction<'_>, ParentPresenceStoreError> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)
    }
}

fn claim_next(
    transaction: &Transaction<'_>,
) -> Result<Option<ClaimedCustodyDecision>, ParentPresenceStoreError> {
    let claimed_at = current_time_millis()?;
    let stale_before = claimed_at.saturating_sub(CLAIM_STALE_AFTER_MILLIS);
    let pending = transaction
        .query_row(SELECT_CLAIMABLE_DECISION, [stale_before], |row| {
            Ok(PendingCustodyDecision {
                decision_id: row.get(0)?,
                envelope_json: row.get(1)?,
            })
        })
        .optional()
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    let Some(decision) = pending else {
        return Ok(None);
    };
    let claim = delivery_claim()?;
    let changed = transaction
        .execute(
            CLAIM_DECISION,
            params![
                claim,
                claimed_at,
                decision.decision_id.as_str(),
                stale_before
            ],
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    if changed != 1 {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    Ok(Some(ClaimedCustodyDecision { decision, claim }))
}

fn mark_delivered(
    transaction: &Transaction<'_>,
    claimed: &ClaimedCustodyDecision,
) -> Result<(), ParentPresenceStoreError> {
    let changed = transaction
        .execute(
            MARK_DECISION_DELIVERED,
            params![claimed.decision.decision_id.as_str(), claimed.claim],
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    if changed == 1 {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn release_claim(
    transaction: &Transaction<'_>,
    claimed: &ClaimedCustodyDecision,
) -> Result<(), ParentPresenceStoreError> {
    let changed = transaction
        .execute(
            RELEASE_DECISION_CLAIM,
            params![claimed.decision.decision_id.as_str(), claimed.claim],
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    if changed == 1 {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn current_time_millis() -> Result<i64, ParentPresenceStoreError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| i64::try_from(duration.as_millis()))
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}

fn delivery_claim() -> Result<String, ParentPresenceStoreError> {
    let mut entropy = [0_u8; 16];
    getrandom::fill(&mut entropy).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    Ok(entropy.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn commit(transaction: Transaction<'_>) -> Result<(), ParentPresenceStoreError> {
    transaction
        .commit()
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}
