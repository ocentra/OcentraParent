use ocentra_schema::account_identity_authority::AccountIdentityProvider;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer_v2::{
    self, AccountIdentityAuthorityProducerV2Request,
};

use super::account_identity_authority_issuer_client_owner_admission::AccountIdentityIssuerOwnerAdmission;
use super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerOutboxClaim, AccountIdentityIssuerV2KeyId,
    AccountIdentityIssuerV2ServiceBindingId, ProtectedAccountIssuerKeyRegistration,
};
use super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};

#[path = "account_identity_authority_issuer_client_transaction_receipt.rs"]
mod receipt;
#[path = "account_identity_authority_issuer_client_transaction_receipt_ack.rs"]
mod receipt_ack;
#[path = "account_identity_authority_issuer_client_transaction_receipt_load.rs"]
mod receipt_load;
#[path = "account_identity_authority_issuer_client_transaction_recovery.rs"]
mod recovery;
#[path = "account_identity_authority_issuer_client_transaction_replay.rs"]
mod replay;
#[path = "account_identity_authority_issuer_client_transaction_reservation.rs"]
mod reservation;
#[path = "account_identity_authority_issuer_client_transaction_reservation_validation.rs"]
mod reservation_validation;

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub(crate) fn validate_owner_admission(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        admission: &AccountIdentityIssuerOwnerAdmission,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        admission.validate_currentness(currentness, correlation_id, idempotency_key)?;
        let key = self.current_key(currentness)?;
        admission.validate_key(&key)
    }

    pub(crate) fn ensure_current(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        let row = self
            .transaction
            .query_row(
                "SELECT mapping_status, authority_generation, session_id,
                        session_generation, authority_json
                   FROM account_identity_current_authority
                  WHERE provider = ?1 AND provider_subject = ?2",
                params![
                    provider_label(currentness.authority().provider()),
                    currentness.authority().provider_subject().as_str()
                ],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, String>(4)?,
                    ))
                },
            )
            .optional()
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)?
            .ok_or(AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)?;
        let stored: ocentra_schema::account_identity_authority::
            AccountIdentityCurrentMemberDeviceAuthorityHandoff = serde_json::from_str(&row.4)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
        let authority_generation = i64::try_from(currentness.authority().authority_generation())
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
        let session_generation = i64::try_from(currentness.authority().session_generation())
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
        if row.0 != "active"
            || row.1 != authority_generation
            || row.2 != currentness.authority().session_id().as_str()
            || row.3 != session_generation
            || stored != *currentness.authority().handoff()
        {
            return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
        }
        stored
            .validate_shape()
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)
    }

    pub(crate) fn register_protected_key(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        registration: &ProtectedAccountIssuerKeyRegistration,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let public_key = *registration.public_key();
        account_identity_authority_producer_v2::validate_public_key(&public_key)?;
        let key_id = account_identity_authority_producer_v2::expected_key_id(&public_key);
        let service_binding_id = service_binding_id(currentness.authority());
        let enrollment_generation = registration.enrollment_generation();
        let enrollment_generation_sql = i64::try_from(enrollment_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let key_generation = next_key_generation(
            &self.transaction,
            currentness.account_id().as_str(),
            currentness.household_id().as_str(),
            &service_binding_id,
        )?;
        let key_generation_u64 = u64::try_from(key_generation)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let authority_generation = i64::try_from(currentness.authority_generation())
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        revoke_active_keys(
            &self.transaction,
            currentness.account_id().as_str(),
            currentness.household_id().as_str(),
            &service_binding_id,
        )?;
        self.transaction
            .execute(
                "INSERT INTO account_identity_issuer_v2_key_registry (
                    account_id, household_id, service, service_binding_id, key_id, key_generation,
                    enrollment_generation, public_key, authority_generation, key_state
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'active')",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                    service_binding_id,
                    key_id,
                    key_generation,
                    enrollment_generation_sql,
                    public_key.as_slice(),
                    authority_generation
                ],
            )
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        Ok(AccountIdentityIssuerV2KeyRecord {
            key_id: AccountIdentityIssuerV2KeyId::from_value(key_id),
            key_generation: key_generation_u64,
            enrollment_generation,
            public_key,
            authority_generation: currentness.authority_generation(),
            service_binding_id: AccountIdentityIssuerV2ServiceBindingId::from_value(
                service_binding_id,
            ),
        })
    }

    pub(crate) fn current_key(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let expected_service_binding_id = service_binding_id(currentness.authority());
        let row = self
            .transaction
            .query_row(
                "SELECT service, key_id, key_generation, enrollment_generation, public_key,
                        authority_generation, service_binding_id
                   FROM account_identity_issuer_v2_key_registry
                  WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
                    AND service_binding_id = ?4
                    AND key_state = 'active'
                  ORDER BY key_generation DESC LIMIT 1",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                    expected_service_binding_id
                ],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, Vec<u8>>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, String>(6)?,
                    ))
                },
            )
            .optional()
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::Unavailable)?
            .ok_or(AccountIdentityAuthorityIssuerClientError::KeyUnavailable)?;
        let public_key: [u8; 65] = row
            .4
            .try_into()
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let key_generation = u64::try_from(row.2)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let enrollment_generation = u64::try_from(row.3)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let authority_generation = u64::try_from(row.5)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        if row.0 != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
            || authority_generation != currentness.authority_generation()
            || enrollment_generation == 0
            || account_identity_authority_producer_v2::expected_key_id(&public_key) != row.1
            || expected_service_binding_id != row.6
        {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidKey);
        }
        account_identity_authority_producer_v2::validate_public_key(&public_key)?;
        Ok(AccountIdentityIssuerV2KeyRecord {
            key_id: AccountIdentityIssuerV2KeyId::from_value(row.1),
            key_generation,
            enrollment_generation,
            public_key,
            authority_generation,
            service_binding_id: AccountIdentityIssuerV2ServiceBindingId::from_value(row.6),
        })
    }

    pub(crate) fn prepare_issue_current_authority(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<
        (
            AccountIdentityAuthorityProducerV2Request,
            AccountIdentityIssuerReservation,
        ),
        AccountIdentityAuthorityIssuerClientError,
    > {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let (now, now_text) = super::clock::now(&self.transaction)?;
        let issued_at = super::clock::parse_timestamp(&now_text)?;
        let request = account_identity_authority_producer_v2::issue_request(
            &account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2IssueInput {
                authority: currentness.authority(),
                key_id: key.key_id().as_str(),
                key_generation: key.key_generation(),
                enrollment_generation: key.enrollment_generation(),
                public_key: key.public_key(),
                service_binding_id: key.service_binding_id().as_str(),
                correlation_id,
                idempotency_key,
                issued_at,
            },
        )
        .map_err(AccountIdentityAuthorityIssuerClientError::from)?;
        let reservation =
            reservation::reserve_issue(&self.transaction, currentness, &request, now)?;
        // The immediate transaction is still open here. Re-resolve both
        // currentness and the active key immediately before the one-way
        // signing transition so a request cannot cross a rotation or session
        // change between preparation and CNG.
        self.ensure_current(currentness)?;
        let latest_key = self.current_key(currentness)?;
        let binding = request.binding();
        if binding.account_id.as_str() != currentness.account_id().as_str()
            || binding.household_id.as_str() != currentness.household_id().as_str()
            || binding.key_id.as_str() != latest_key.key_id().as_str()
            || binding.key_generation != latest_key.key_generation()
            || binding.enrollment_generation != latest_key.enrollment_generation()
            || binding.authority_generation != latest_key.authority_generation()
            || binding.session_generation != currentness.session_generation()
            || binding.service_binding_id.as_str() != latest_key.service_binding_id().as_str()
        {
            return Err(AccountIdentityAuthorityIssuerClientError::KeyUnavailable);
        }
        let (signing_now, _) = super::clock::now(&self.transaction)?;
        recovery::mark_signing(&self.transaction, &reservation, signing_now)?;
        Ok((request, reservation))
    }

    pub(crate) fn record_signing_failure(
        &self,
        request: &AccountIdentityAuthorityProducerV2Request,
        reservation: &AccountIdentityIssuerReservation,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        let (now, _) = super::clock::now(&self.transaction)?;
        recovery::mark_signing_failure(&self.transaction, request, reservation, now)
    }

    pub(crate) fn prepare_acknowledge_receipt(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
    ) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityIssuerClientError>
    {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let stored =
            receipt_load::load_verified_claimed_issue(&self.transaction, currentness, &key, claim)?;
        let (_, now) = super::clock::now(&self.transaction)?;
        let now = super::clock::parse_timestamp(&now)?;
        account_identity_authority_producer_v2::acknowledge_request(
            &stored.receipt,
            key.public_key(),
            now,
        )
        .map_err(Into::into)
    }

    pub(crate) fn commit(self) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        self.transaction
            .commit()
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::Unavailable)
    }
}

pub(super) fn reconcile_issue_reservations(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    recovery::reconcile_issue_reservations(transaction, now)
}

fn next_key_generation(
    transaction: &Transaction<'_>,
    account_id: &str,
    household_id: &str,
    service_binding_id: &str,
) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    let latest: Option<i64> = transaction
        .query_row(
            "SELECT MAX(key_generation) FROM account_identity_issuer_v2_key_registry
             WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
               AND service_binding_id = ?4",
            params![
                account_id,
                household_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                service_binding_id
            ],
            |row| row.get(0),
        )
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
    latest
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidKey)
}

fn revoke_active_keys(
    transaction: &Transaction<'_>,
    account_id: &str,
    household_id: &str,
    service_binding_id: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transaction
        .execute(
            "UPDATE account_identity_issuer_v2_key_registry
                SET key_state = 'revoked'
              WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
                AND service_binding_id = ?4
                AND key_state = 'active'",
            params![
                account_id,
                household_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                service_binding_id
            ],
        )
        .map(|_| ())
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::Unavailable)
}

fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

fn service_binding_id(authority: &VerifiedAccountIdentityAuthority) -> String {
    service_binding_id_for_values(
        authority.account_id().to_string().as_str(),
        authority.household_id().to_string().as_str(),
    )
}

pub(super) fn service_binding_id_for_values(account_id: &str, household_id: &str) -> String {
    let mut binding = Vec::new();
    binding.extend_from_slice(b"ocentra.account-authority-producer.v2.binding\0");
    append_length_prefixed(
        &mut binding,
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.as_bytes(),
    );
    append_length_prefixed(&mut binding, account_id.as_bytes());
    append_length_prefixed(&mut binding, household_id.as_bytes());
    format!("sha256:binding:{}", sha256_hex(binding.as_slice()))
}

fn append_length_prefixed(target: &mut Vec<u8>, value: &[u8]) {
    target.extend_from_slice(&(value.len() as u64).to_be_bytes());
    target.extend_from_slice(value);
}

fn sha256_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = digest(&SHA256, value);
    let mut text = String::with_capacity(digest.as_ref().len() * 2);
    for byte in digest.as_ref() {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}
