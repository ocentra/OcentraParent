use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport;

use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};
use super::receipt::validate_verified_currentness;

pub(super) struct StoredIssue {
    pub(super) receipt_id: String,
    pub(super) account_id: String,
    pub(super) household_id: String,
    pub(super) provider: Option<String>,
    pub(super) provider_subject: Option<String>,
    pub(super) provenance_state: String,
    pub(super) service: String,
    pub(super) service_binding_id: String,
    pub(super) key_id: String,
    pub(super) key_generation: i64,
    pub(super) enrollment_generation: i64,
    pub(super) authority_generation: i64,
    pub(super) session_generation: i64,
    pub(super) correlation_id: String,
    pub(super) idempotency_key: String,
    pub(super) payload_digest: String,
    pub(super) issued_at: String,
    pub(super) expires_at: String,
    pub(super) wire: Vec<u8>,
}

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub(crate) fn existing_issued_transport(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<
        Option<AccountIdentityAuthorityProducerV2Transport>,
        AccountIdentityAuthorityIssuerClientError,
    > {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let stored =
            load_stored_issue_by_idempotency(&self.transaction, currentness, idempotency_key)?;
        let Some(stored) = stored else {
            return Ok(None);
        };
        if stored.provenance_state != "exact"
            || stored.provider.as_deref()
                != Some(super::provider_label(currentness.authority().provider()))
            || stored.provider_subject.as_deref()
                != Some(currentness.authority().provider_subject().as_str())
        {
            // v3 rows were intentionally migrated without fabricated
            // provider provenance.  They cannot authorize exact replay.
            return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
        }
        if stored.correlation_id != correlation_id || stored.idempotency_key != idempotency_key {
            return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
        }
        Ok(Some(verify_stored_issue(
            &self.transaction,
            currentness,
            &key,
            stored,
        )?))
    }
}

pub(super) fn load_stored_issue_by_idempotency(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    idempotency_key: &str,
) -> Result<Option<StoredIssue>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            "SELECT receipt_id, account_id, household_id, provider, provider_subject,
                    provenance_state, service, service_binding_id, key_id, key_generation,
                    enrollment_generation, authority_generation, session_generation,
                    correlation_id, idempotency_key, payload_digest, issued_at,
                    expires_at, wire
               FROM account_identity_issuer_v2_receipt
              WHERE account_id = ?1 AND idempotency_key = ?2 AND service = ?3",
            params![
                currentness.account_id().as_str(),
                idempotency_key,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
            ],
            |row| {
                Ok(StoredIssue {
                    receipt_id: row.get(0)?,
                    account_id: row.get(1)?,
                    household_id: row.get(2)?,
                    provider: row.get(3)?,
                    provider_subject: row.get(4)?,
                    provenance_state: row.get(5)?,
                    service: row.get(6)?,
                    service_binding_id: row.get(7)?,
                    key_id: row.get(8)?,
                    key_generation: row.get(9)?,
                    enrollment_generation: row.get(10)?,
                    authority_generation: row.get(11)?,
                    session_generation: row.get(12)?,
                    correlation_id: row.get(13)?,
                    idempotency_key: row.get(14)?,
                    payload_digest: row.get(15)?,
                    issued_at: row.get(16)?,
                    expires_at: row.get(17)?,
                    wire: row.get(18)?,
                })
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
}

pub(super) fn verify_stored_issue(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    stored: StoredIssue,
) -> Result<AccountIdentityAuthorityProducerV2Transport, AccountIdentityAuthorityIssuerClientError>
{
    let receipt = receipt_from_stored(&stored)?;
    validate_current_receipt(currentness, key, &receipt)?;
    let transport = crate::account_identity_authority_producer_v2::from_durable_transport(
        stored.wire,
        receipt.clone(),
    );
    let (_, now) = super::super::clock::now(transaction)?;
    let now = super::super::clock::parse_timestamp(&now)?;
    let verified = crate::account_identity_authority_producer_v2::verify(
        transport.wire_bytes(),
        key.public_key(),
        now,
    )?;
    validate_verified_currentness(currentness, &verified)?;
    if verified.operation() != AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority
        || verified.key_id() != key.key_id().as_str()
        || verified.service_binding_id() != key.service_binding_id().as_str()
        || verified.key_generation() != key.key_generation()
        || verified.enrollment_generation() != key.enrollment_generation()
        || verified.authority_generation() != key.authority_generation()
        || verified.session_generation() != receipt.session_generation
        || verified.correlation_id() != receipt.correlation_id
        || verified.idempotency_key() != receipt.idempotency_key
        || verified.payload_digest() != receipt.payload_digest
        || verified.receipt_id() != receipt.receipt_id
        || verified.claims().account_id != receipt.account_id
        || verified.claims().household_id != receipt.household_id
    {
        return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
    }
    Ok(transport)
}

fn receipt_from_stored(
    stored: &StoredIssue,
) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError> {
    if stored.service != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let receipt = AccountIdentityAuthorityProducerV2Receipt {
        receipt_id: stored.receipt_id.clone(),
        operation: AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        account_id: stored.account_id.clone(),
        household_id: stored.household_id.clone(),
        service_binding_id: stored.service_binding_id.clone(),
        correlation_id: stored.correlation_id.clone(),
        idempotency_key: stored.idempotency_key.clone(),
        payload_digest: stored.payload_digest.clone(),
        key_id: stored.key_id.clone(),
        key_generation: u64::try_from(stored.key_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?,
        enrollment_generation: u64::try_from(stored.enrollment_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?,
        authority_generation: u64::try_from(stored.authority_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?,
        session_generation: u64::try_from(stored.session_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?,
        issued_at: stored.issued_at.clone(),
        expires_at: stored.expires_at.clone(),
    };
    receipt
        .validate_shape()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
    Ok(receipt)
}

fn validate_current_receipt(
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if receipt.account_id != currentness.account_id().as_str()
        || receipt.household_id != currentness.household_id().as_str()
        || receipt.authority_generation != currentness.authority_generation()
        || receipt.session_generation != currentness.session_generation()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
    }
    if receipt.key_id != key.key_id().as_str()
        || receipt.key_generation != key.key_generation()
        || receipt.enrollment_generation != key.enrollment_generation()
        || receipt.service_binding_id != key.service_binding_id().as_str()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
    }
    Ok(())
}
