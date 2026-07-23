use rusqlite::{params, Transaction, TransactionBehavior};

use crate::parent_presence::ParentPresenceStorageFailureReason;
use crate::parent_presence_event_delivery::PendingCustodyDecision;

use super::{ParentPresenceStore, ParentPresenceStoreError, INSERT_PENDING_DECISION};

const SELECT_PENDING_DECISIONS: &str = r#"
SELECT decision_id, envelope_json
FROM parent_presence_decision_outbox
WHERE delivery_state = 'pending'
ORDER BY rowid
"#;

const MARK_DECISION_DELIVERED: &str = r#"
UPDATE parent_presence_decision_outbox
SET delivery_state = 'delivered'
WHERE decision_id = ?1 AND delivery_state = 'pending'
"#;

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
        let transaction = self.immediate_transaction()?;
        let pending = load_pending(&transaction)?;
        for decision in pending {
            deliver(&decision).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
            mark_delivered(&transaction, &decision.decision_id)?;
        }
        commit(transaction)
    }

    fn immediate_transaction(&mut self) -> Result<Transaction<'_>, ParentPresenceStoreError> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)
    }
}

fn load_pending(
    transaction: &Transaction<'_>,
) -> Result<Vec<PendingCustodyDecision>, ParentPresenceStoreError> {
    let mut statement = transaction
        .prepare(SELECT_PENDING_DECISIONS)
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok(PendingCustodyDecision {
                decision_id: row.get(0)?,
                envelope_json: row.get(1)?,
            })
        })
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}

fn mark_delivered(
    transaction: &Transaction<'_>,
    decision_id: &str,
) -> Result<(), ParentPresenceStoreError> {
    let changed = transaction
        .execute(MARK_DECISION_DELIVERED, [decision_id])
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    if changed == 1 {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn commit(transaction: Transaction<'_>) -> Result<(), ParentPresenceStoreError> {
    transaction
        .commit()
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}
