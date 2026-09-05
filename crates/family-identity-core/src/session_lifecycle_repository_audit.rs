#![forbid(unsafe_code)]

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProviderSubject,
};
use ocentra_schema::report_query_custody::ParentAccountId;
use rusqlite::{params, ErrorCode, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::session_lifecycle_custody::audit_delivery::{
    SessionAuditDeliveryAttemptId, SessionAuditEventId,
};
use crate::session_lifecycle_custody::record::SessionCredentialRecord;
use crate::session_lifecycle_record::SessionId;

use super::{
    clock, invariants, labels, PendingSessionAuditDelivery, SessionAuditAction, SessionAuditEvent,
    SessionLifecycleRepositoryError,
};

pub(crate) fn insert_audit(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
    action: SessionAuditAction,
    occurred_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    invariants::validate_record(record)?;
    if !audit_transition_is_valid(record, action, occurred_at_epoch_millis) {
        return Err(SessionLifecycleRepositoryError::InvalidAuditRecord);
    }
    let event_id = SessionAuditEventId::generate()
        .map_err(|_error| SessionLifecycleRepositoryError::EntropyUnavailable)?;
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_session_audit_outbox (
                 event_id, session_id, account_id, provider_subject, member_id,
                 device_id, action, occurred_at_epoch_millis, delivery_state,
                 delivery_attempt_id, delivery_claimed_at_epoch_millis,
                 delivered_at_epoch_millis
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'pending', NULL, NULL, NULL)",
            params![
                event_id.as_str(),
                record.session_id.as_str(),
                record.binding.account_id.to_string(),
                record.binding.provider_subject.as_str(),
                record.binding.member_id.as_str(),
                record.binding.device_id.as_str(),
                labels::audit_label(action).0,
                occurred_at_epoch_millis,
            ],
        )
        .map_err(|error| map_audit_insert_error(&error))?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::AuditConflict)
}

fn audit_transition_is_valid(
    record: &SessionCredentialRecord,
    action: SessionAuditAction,
    occurred_at_epoch_millis: i64,
) -> bool {
    match action {
        SessionAuditAction::Created => {
            record.refresh_generation == 1
                && record.activity_state == crate::session_lifecycle::SessionActivityState::Active
                && occurred_at_epoch_millis == record.issued_at_epoch_millis
        }
        SessionAuditAction::Rotated => {
            record.refresh_generation > 1
                && record.activity_state == crate::session_lifecycle::SessionActivityState::Active
                && occurred_at_epoch_millis == record.issued_at_epoch_millis
        }
        SessionAuditAction::LoggedOut
        | SessionAuditAction::Revoked
        | SessionAuditAction::GloballyRevoked => {
            record.activity_state == crate::session_lifecycle::SessionActivityState::Active
                && occurred_at_epoch_millis > record.last_transition_at_epoch_millis
        }
    }
}

impl SqliteAccountIdentityAuthorityRepository {
    pub fn claim_next_session_audit_delivery(
        &mut self,
    ) -> Result<Option<PendingSessionAuditDelivery>, SessionLifecycleRepositoryError> {
        let trusted_now_epoch_millis = clock::trusted_now_epoch_millis()?;
        let stale_before_epoch_millis = trusted_now_epoch_millis
            .checked_sub(self.session_policy.audit_delivery_lease_millis)
            .ok_or(SessionLifecycleRepositoryError::ClockUnavailable)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        requeue_stale_deliveries(&transaction, stale_before_epoch_millis)?;
        let Some(event) = read_next_pending(&transaction)? else {
            transaction
                .commit()
                .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
            return Ok(None);
        };
        let delivery_attempt_id = SessionAuditDeliveryAttemptId::generate()
            .map_err(|_error| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let claimed_at_epoch_millis = trusted_now_epoch_millis.max(event.occurred_at_epoch_millis);
        let changed = transaction
            .execute(
                "UPDATE account_identity_session_audit_outbox
                 SET delivery_state = 'in-flight', delivery_attempt_id = ?2
                   , delivery_claimed_at_epoch_millis = ?3
                 WHERE event_id = ?1 AND delivery_state = 'pending'
                   AND delivery_attempt_id IS NULL
                   AND delivery_claimed_at_epoch_millis IS NULL
                   AND delivered_at_epoch_millis IS NULL",
                params![
                    event.event_id.as_str(),
                    delivery_attempt_id.as_str(),
                    claimed_at_epoch_millis
                ],
            )
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        transaction
            .commit()
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(Some(PendingSessionAuditDelivery {
            event,
            delivery_attempt_id,
        }))
    }

    pub fn acknowledge_session_audit_delivery(
        &mut self,
        delivery: PendingSessionAuditDelivery,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let delivered_at_epoch_millis =
            clock::trusted_now_epoch_millis()?.max(delivery.event.occurred_at_epoch_millis);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_session_audit_outbox
                 SET delivery_state = 'delivered', delivery_attempt_id = NULL,
                     delivery_claimed_at_epoch_millis = NULL,
                     delivered_at_epoch_millis = ?3
                 WHERE event_id = ?1 AND delivery_attempt_id = ?2
                   AND delivery_state = 'in-flight'
                   AND delivery_claimed_at_epoch_millis IS NOT NULL
                   AND delivered_at_epoch_millis IS NULL",
                params![
                    delivery.event.event_id.as_str(),
                    delivery.delivery_attempt_id.as_str(),
                    delivered_at_epoch_millis,
                ],
            )
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        drop(delivery);
        transaction
            .commit()
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)
    }

    pub fn release_session_audit_delivery(
        &mut self,
        delivery: PendingSessionAuditDelivery,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_session_audit_outbox
                 SET delivery_state = 'pending', delivery_attempt_id = NULL
                   , delivery_claimed_at_epoch_millis = NULL
                 WHERE event_id = ?1 AND delivery_attempt_id = ?2
                   AND delivery_state = 'in-flight'
                   AND delivery_claimed_at_epoch_millis IS NOT NULL
                   AND delivered_at_epoch_millis IS NULL",
                params![
                    delivery.event.event_id.as_str(),
                    delivery.delivery_attempt_id.as_str()
                ],
            )
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        drop(delivery);
        transaction
            .commit()
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)
    }
}

fn requeue_stale_deliveries(
    transaction: &Transaction<'_>,
    stale_before_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    transaction
        .execute(
            "UPDATE account_identity_session_audit_outbox
             SET delivery_state = 'pending', delivery_attempt_id = NULL,
                 delivery_claimed_at_epoch_millis = NULL
             WHERE delivery_state = 'in-flight'
               AND delivery_claimed_at_epoch_millis <= ?1
               AND delivered_at_epoch_millis IS NULL",
            [stale_before_epoch_millis],
        )
        .map(|_| ())
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)
}

fn read_next_pending(
    transaction: &Transaction<'_>,
) -> Result<Option<SessionAuditEvent>, SessionLifecycleRepositoryError> {
    let row = transaction
        .query_row(
            "SELECT event_id, session_id, account_id, provider_subject,
                    member_id, device_id, action, occurred_at_epoch_millis
             FROM account_identity_session_audit_outbox
             WHERE delivery_state = 'pending' AND delivery_attempt_id IS NULL
               AND delivered_at_epoch_millis IS NULL
             ORDER BY sequence LIMIT 1",
            [],
            |row| {
                Ok(StoredAuditRow {
                    event_id: row.get(0)?,
                    session_id: row.get(1)?,
                    account_id: row.get(2)?,
                    provider_subject: row.get(3)?,
                    member_id: row.get(4)?,
                    device_id: row.get(5)?,
                    action: row.get(6)?,
                    occurred_at_epoch_millis: row.get(7)?,
                })
            },
        )
        .optional()
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
    row.map(stored_audit_into_event).transpose()
}

struct StoredAuditRow {
    event_id: String,
    session_id: String,
    account_id: String,
    provider_subject: String,
    member_id: String,
    device_id: String,
    action: String,
    occurred_at_epoch_millis: i64,
}

fn stored_audit_into_event(
    row: StoredAuditRow,
) -> Result<SessionAuditEvent, SessionLifecycleRepositoryError> {
    if row.occurred_at_epoch_millis <= 0 {
        return Err(SessionLifecycleRepositoryError::InvalidAuditRecord);
    }
    Ok(SessionAuditEvent {
        event_id: SessionAuditEventId::parse(row.event_id)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        session_id: SessionId::parse(row.session_id)
            .map_err(|_error| SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        account_id: ParentAccountId::parse(row.account_id)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        provider_subject: AccountIdentityProviderSubject::parse(row.provider_subject)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        member_id: AccountIdentityMemberId::parse(row.member_id)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        device_id: AccountIdentityDeviceId::parse(row.device_id)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        action: labels::parse_audit_action(row.action.as_bytes())?,
        occurred_at_epoch_millis: row.occurred_at_epoch_millis,
    })
}

fn map_audit_insert_error(error: &rusqlite::Error) -> SessionLifecycleRepositoryError {
    match error {
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == ErrorCode::ConstraintViolation =>
        {
            SessionLifecycleRepositoryError::AuditConflict
        }
        _ => SessionLifecycleRepositoryError::Unavailable,
    }
}
