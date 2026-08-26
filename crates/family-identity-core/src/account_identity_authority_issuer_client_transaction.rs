use ocentra_schema::account_identity_authority::AccountIdentityProvider;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer_v2::{
    self, AccountIdentityAuthorityProducerV2Request,
};

use super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerReceiptProof, AccountIdentityIssuerV2KeyId,
    AccountIdentityIssuerV2ServiceBindingId, ProtectedAccountIssuerKeyRegistration,
};
use super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};

#[path = "account_identity_authority_issuer_client_transaction_receipt.rs"]
mod receipt;
#[path = "account_identity_authority_issuer_client_transaction_receipt_load.rs"]
mod receipt_load;
#[path = "account_identity_authority_issuer_client_transaction_replay.rs"]
mod replay;

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub fn ensure_current(
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
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)?
            .ok_or(AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)?;
        let stored: ocentra_schema::account_identity_authority::
            AccountIdentityCurrentMemberDeviceAuthorityHandoff = serde_json::from_str(&row.4)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
        let authority_generation = i64::try_from(currentness.authority().authority_generation())
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
        let session_generation = i64::try_from(currentness.authority().session_generation())
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)?;
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
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessRejected)
    }

    pub fn register_protected_key(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        registration: &ProtectedAccountIssuerKeyRegistration,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let public_key = *registration.public_key();
        account_identity_authority_producer_v2::validate_public_key(&public_key)?;
        let key_id = account_identity_authority_producer_v2::expected_key_id(&public_key);
        let service_binding_id = service_binding_id(currentness.authority(), &key_id);
        let latest: Option<i64> = self
            .transaction
            .query_row(
                "SELECT MAX(key_generation) FROM account_identity_issuer_v2_key_registry
                 WHERE account_id = ?1 AND household_id = ?2 AND service = ?3",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
                ],
                |row| row.get(0),
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        let key_generation = latest
            .unwrap_or(0)
            .checked_add(1)
            .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let key_generation_u64 = u64::try_from(key_generation)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let authority_generation = i64::try_from(currentness.authority_generation())
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        self.transaction
            .execute(
                "UPDATE account_identity_issuer_v2_key_registry
                    SET key_state = 'revoked'
                  WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
                    AND key_state = 'active'",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
                ],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        self.transaction
            .execute(
                "INSERT INTO account_identity_issuer_v2_key_registry (
                    account_id, household_id, service, service_binding_id, key_id, key_generation,
                    public_key, authority_generation, key_state
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'active')",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                    service_binding_id,
                    key_id,
                    key_generation,
                    public_key.as_slice(),
                    authority_generation
                ],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        Ok(AccountIdentityIssuerV2KeyRecord {
            key_id: AccountIdentityIssuerV2KeyId::from_value(key_id),
            key_generation: key_generation_u64,
            public_key,
            authority_generation: currentness.authority_generation(),
            service_binding_id: AccountIdentityIssuerV2ServiceBindingId::from_value(
                service_binding_id,
            ),
        })
    }

    pub fn current_key(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let row = self
            .transaction
            .query_row(
                "SELECT service, key_id, key_generation, public_key, authority_generation,
                        service_binding_id
                   FROM account_identity_issuer_v2_key_registry
                  WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
                    AND key_state = 'active'
                  ORDER BY key_generation DESC LIMIT 1",
                params![
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
                ],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, Vec<u8>>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?
            .ok_or(AccountIdentityAuthorityIssuerClientError::KeyUnavailable)?;
        let public_key: [u8; 65] = row
            .3
            .try_into()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let key_generation = u64::try_from(row.2)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let authority_generation = u64::try_from(row.4)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidKey)?;
        let expected_service_binding_id = service_binding_id(currentness.authority(), &row.1);
        if row.0 != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
            || authority_generation != currentness.authority_generation()
            || account_identity_authority_producer_v2::expected_key_id(&public_key) != row.1
            || expected_service_binding_id != row.5
        {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidKey);
        }
        Ok(AccountIdentityIssuerV2KeyRecord {
            key_id: AccountIdentityIssuerV2KeyId::from_value(row.1),
            key_generation,
            public_key,
            authority_generation,
            service_binding_id: AccountIdentityIssuerV2ServiceBindingId::from_value(row.5),
        })
    }

    pub fn prepare_issue_current_authority(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityIssuerClientError>
    {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let (_, now) = super::clock::now(&self.transaction)?;
        let now = super::clock::parse_timestamp(&now)?;
        account_identity_authority_producer_v2::issue_request(
            currentness.authority(),
            key.key_id().as_str(),
            key.key_generation(),
            key.public_key(),
            key.service_binding_id().as_str(),
            correlation_id,
            idempotency_key,
            now,
        )
        .map_err(Into::into)
    }

    pub fn prepare_acknowledge_receipt(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        receipt_id: &str,
    ) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityIssuerClientError>
    {
        let proof = self.prepare_receipt_proof(currentness, receipt_id)?;
        let receipt = proof.receipt();
        let key = self.current_key(currentness)?;
        let (_, now) = super::clock::now(&self.transaction)?;
        let now = super::clock::parse_timestamp(&now)?;
        account_identity_authority_producer_v2::acknowledge_request(receipt, key.public_key(), now)
            .map_err(Into::into)
    }

    pub fn load_receipt_proof(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        receipt_id: &str,
    ) -> Result<AccountIdentityIssuerReceiptProof, AccountIdentityAuthorityIssuerClientError> {
        self.prepare_receipt_proof(currentness, receipt_id)
    }

    pub fn pending_outbox_count(&self) -> Result<u64, AccountIdentityAuthorityIssuerClientError> {
        count_transaction(
            &self.transaction,
            "SELECT COUNT(*) FROM account_identity_issuer_v2_outbox
              WHERE delivery_state IN ('pending','claimed','failed')",
        )
    }

    pub fn commit(self) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        self.transaction
            .commit()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)
    }
}

fn count_transaction(
    transaction: &Transaction<'_>,
    query: &str,
) -> Result<u64, AccountIdentityAuthorityIssuerClientError> {
    let value: i64 = transaction
        .query_row(query, [], |row| row.get(0))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
    u64::try_from(value).map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

fn service_binding_id(authority: &VerifiedAccountIdentityAuthority, key_id: &str) -> String {
    let mut binding = Vec::new();
    binding.extend_from_slice(b"ocentra.account-authority-producer.v2.binding\0");
    binding.extend_from_slice(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.as_bytes());
    binding.push(0);
    binding.extend_from_slice(authority.account_id().to_string().as_bytes());
    binding.push(0);
    binding.extend_from_slice(authority.household_id().to_string().as_bytes());
    binding.push(0);
    binding.extend_from_slice(key_id.as_bytes());
    format!("sha256:binding:{}", sha256_hex(binding.as_slice()))
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
