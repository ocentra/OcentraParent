use ocentra_schema::account_identity_authority::AccountIdentityProvider;
use ocentra_schema::parent_storage_settings_apply_flow::{
    ParentStorageApplyIntentDigest, ParentStoragePreviewId,
};
use rusqlite::{Connection, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use crate::device_trust_lifecycle::DeviceTrustLifecycleState;
use crate::household_authority_runtime_composer::HouseholdAuthorityDeviceTrustSource;

use super::account_identity_authority_repository_invariants::provider_label;

const TABLE: &str = "account_identity_parent_storage_confirmation";
const MAX_CONFIRMATION_TTL_MILLIS: i64 = 5 * 60 * 1_000;
#[path = "parent_storage_confirmation_store_device_failure.rs"]
mod device_failure;
#[path = "parent_storage_confirmation_store_owner_failure.rs"]
mod owner_failure;
#[path = "parent_storage_confirmation_store_runtime.rs"]
mod runtime;
#[path = "parent_storage_confirmation_store_schema.rs"]
mod schema;
#[path = "parent_storage_confirmation_store_step_up_failure.rs"]
mod step_up_failure;
#[path = "parent_storage_confirmation_store_support.rs"]
mod support;
#[path = "parent_storage_confirmation_store_terminal_failure.rs"]
mod terminal_failure;

pub(crate) type ParentStorageConfirmationStoreError =
    crate::household_authority_runtime_composer::HouseholdAuthorityParentStorageStoreFailure;

pub(crate) struct ParentStorageConfirmationBinding<'a> {
    pub(crate) provider: &'a AccountIdentityProvider,
    pub(crate) provider_subject: &'a str,
    pub(crate) household_id: &'a str,
    pub(crate) account_id: &'a str,
    pub(crate) parent_device_id: &'a str,
    pub(crate) child_profile_id: &'a str,
    pub(crate) child_device_id: &'a str,
    pub(crate) installation_id: &'a str,
    pub(crate) pairing_id: &'a str,
    pub(crate) route_id: &'a str,
    pub(crate) authority_generation: u64,
    pub(crate) session_generation: u64,
    pub(crate) device_trust_subject: &'a str,
    pub(crate) device_lifecycle_generation: u64,
    pub(crate) device_installation_binding_generation: u64,
    pub(crate) device_authority_generation: u64,
}

pub(crate) struct StagedParentStorageConfirmation {
    pub(crate) receipt_id: String,
    pub(crate) nonce_id: String,
    pub(crate) receipt_epoch: u64,
    pub(crate) expires_at_epoch_millis: i64,
}

pub(crate) struct ConsumedParentStorageConfirmation {
    pub(crate) receipt_id: String,
    pub(crate) nonce_id: String,
    pub(crate) receipt_epoch: u64,
    pub(crate) expires_at_epoch_millis: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoredLifecycleState {
    Staged,
    Consumed,
    Expired,
}

pub(crate) fn initialize(
    connection: &Connection,
) -> Result<(), super::AccountIdentityAuthorityRepositoryError> {
    connection
        .execute_batch(schema::SCHEMA_SQL)
        .map_err(|_| super::AccountIdentityAuthorityRepositoryError::Unavailable)?;
    validate(connection)
        .map_err(|_| super::AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)
}

pub(crate) fn validate(connection: &Connection) -> Result<(), ParentStorageConfirmationStoreError> {
    let integrity = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    (integrity == "ok")
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)?;
    let foreign_keys = connection
        .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    (foreign_keys == 1)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)?;
    let mut foreign_key_statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let mut foreign_key_rows = foreign_key_statement
        .query([])
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let foreign_keys_are_clean = foreign_key_rows
        .next()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .is_none();
    foreign_keys_are_clean
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)?;
    schema::validate_table(connection)?;
    schema::validate_related_objects(connection)?;
    schema::validate_rows(connection)
}

pub(crate) fn stage(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    binding: ParentStorageConfirmationBinding<'_>,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
) -> Result<StagedParentStorageConfirmation, ParentStorageConfirmationStoreError> {
    runtime::stage(
        connection,
        authority,
        device_trust_source,
        binding,
        preview_id,
        apply_intent_digest,
    )
}

pub(crate) fn consume(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    binding: ParentStorageConfirmationBinding<'_>,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: u64,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
) -> Result<ConsumedParentStorageConfirmation, ParentStorageConfirmationStoreError> {
    runtime::consume(
        connection,
        authority,
        device_trust_source,
        binding,
        receipt_id,
        nonce_id,
        receipt_epoch,
        preview_id,
        apply_intent_digest,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoredBinding {
    provider: String,
    provider_subject: String,
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    authority_generation: i64,
    session_generation: i64,
    device_trust_subject: String,
    device_lifecycle_generation: i64,
    device_installation_binding_generation: i64,
    device_authority_generation: i64,
    preview_id: String,
    apply_intent_digest: String,
}

impl StoredBinding {
    fn matches(
        &self,
        expected: &ParentStorageConfirmationBinding<'_>,
        preview_id: &ParentStoragePreviewId,
        apply_intent_digest: &ParentStorageApplyIntentDigest,
    ) -> bool {
        self.provider == provider_label(expected.provider)
            && self.provider_subject == expected.provider_subject
            && self.household_id == expected.household_id
            && self.account_id == expected.account_id
            && self.parent_device_id == expected.parent_device_id
            && self.child_profile_id == expected.child_profile_id
            && self.child_device_id == expected.child_device_id
            && self.installation_id == expected.installation_id
            && self.pairing_id == expected.pairing_id
            && self.route_id == expected.route_id
            && self.authority_generation == expected.authority_generation as i64
            && self.session_generation == expected.session_generation as i64
            && self.device_trust_subject == expected.device_trust_subject
            && self.device_lifecycle_generation == expected.device_lifecycle_generation as i64
            && self.device_installation_binding_generation
                == expected.device_installation_binding_generation as i64
            && self.device_authority_generation == expected.device_authority_generation as i64
            && self.preview_id == preview_id.as_str()
            && self.apply_intent_digest == apply_intent_digest.as_str()
    }
}

struct StoredRow {
    receipt_id: String,
    nonce_id: String,
    provider: String,
    provider_subject: String,
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    authority_generation: i64,
    session_generation: i64,
    device_trust_subject: String,
    device_lifecycle_generation: i64,
    device_installation_binding_generation: i64,
    device_authority_generation: i64,
    preview_id: String,
    apply_intent_digest: String,
    receipt_epoch: i64,
    issued_at_epoch_millis: i64,
    expires_at_epoch_millis: i64,
    consumed_at_epoch_millis: Option<i64>,
    lifecycle_state: String,
}

impl StoredRow {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            receipt_id: row.get(0)?,
            nonce_id: row.get(1)?,
            provider: row.get(2)?,
            provider_subject: row.get(3)?,
            household_id: row.get(4)?,
            account_id: row.get(5)?,
            parent_device_id: row.get(6)?,
            child_profile_id: row.get(7)?,
            child_device_id: row.get(8)?,
            installation_id: row.get(9)?,
            pairing_id: row.get(10)?,
            route_id: row.get(11)?,
            authority_generation: row.get(12)?,
            session_generation: row.get(13)?,
            device_trust_subject: row.get(14)?,
            device_lifecycle_generation: row.get(15)?,
            device_installation_binding_generation: row.get(16)?,
            device_authority_generation: row.get(17)?,
            preview_id: row.get(18)?,
            apply_intent_digest: row.get(19)?,
            receipt_epoch: row.get(20)?,
            issued_at_epoch_millis: row.get(21)?,
            expires_at_epoch_millis: row.get(22)?,
            consumed_at_epoch_millis: row.get(23)?,
            lifecycle_state: row.get(24)?,
        })
    }

    fn validate(&self) -> Result<(), ParentStorageConfirmationStoreError> {
        schema::validate_hex_id(&self.receipt_id)?;
        schema::validate_hex_id(&self.nonce_id)?;
        schema::validate_provider(&self.provider)?;
        [
            self.provider_subject.as_str(),
            self.household_id.as_str(),
            self.account_id.as_str(),
            self.parent_device_id.as_str(),
            self.child_profile_id.as_str(),
            self.child_device_id.as_str(),
            self.installation_id.as_str(),
            self.pairing_id.as_str(),
            self.route_id.as_str(),
            self.device_trust_subject.as_str(),
            self.preview_id.as_str(),
        ]
        .into_iter()
        .try_for_each(schema::validate_identity)?;
        schema::validate_lower_hex(&self.apply_intent_digest, 64)?;
        [
            self.authority_generation,
            self.session_generation,
            self.device_lifecycle_generation,
            self.device_installation_binding_generation,
            self.device_authority_generation,
            self.receipt_epoch,
        ]
        .into_iter()
        .try_for_each(|generation| {
            (generation > 0)
                .then_some(())
                .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
        })?;
        (self.issued_at_epoch_millis > 0
            && self.expires_at_epoch_millis > self.issued_at_epoch_millis
            && self.expires_at_epoch_millis
                <= self.issued_at_epoch_millis + MAX_CONFIRMATION_TTL_MILLIS)
            .then_some(())
            .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)?;
        let lifecycle_is_valid = (self.lifecycle_state == "staged"
            && self.consumed_at_epoch_millis.is_none())
            || (self.lifecycle_state == "expired" && self.consumed_at_epoch_millis.is_none())
            || (self.lifecycle_state == "consumed"
                && self
                    .consumed_at_epoch_millis
                    .is_some_and(|consumed_at| consumed_at >= self.issued_at_epoch_millis));
        lifecycle_is_valid
            .then_some(())
            .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
    }

    fn binding(&self) -> StoredBinding {
        StoredBinding {
            provider: self.provider.clone(),
            provider_subject: self.provider_subject.clone(),
            household_id: self.household_id.clone(),
            account_id: self.account_id.clone(),
            parent_device_id: self.parent_device_id.clone(),
            child_profile_id: self.child_profile_id.clone(),
            child_device_id: self.child_device_id.clone(),
            installation_id: self.installation_id.clone(),
            pairing_id: self.pairing_id.clone(),
            route_id: self.route_id.clone(),
            authority_generation: self.authority_generation,
            session_generation: self.session_generation,
            device_trust_subject: self.device_trust_subject.clone(),
            device_lifecycle_generation: self.device_lifecycle_generation,
            device_installation_binding_generation: self.device_installation_binding_generation,
            device_authority_generation: self.device_authority_generation,
            preview_id: self.preview_id.clone(),
            apply_intent_digest: self.apply_intent_digest.clone(),
        }
    }
}

fn load_binding(
    transaction: &Transaction<'_>,
    receipt_id: &str,
) -> Result<StoredBinding, ParentStorageConfirmationStoreError> {
    let row = transaction
        .query_row(
            "SELECT provider, provider_subject, household_id, account_id, parent_device_id,
                    child_profile_id, child_device_id, installation_id, pairing_id, route_id,
                    authority_generation, session_generation, device_trust_subject,
                    device_lifecycle_generation, device_installation_binding_generation,
                    device_authority_generation, preview_id, apply_intent_digest
             FROM account_identity_parent_storage_confirmation WHERE receipt_id = ?1",
            [receipt_id],
            |row| {
                Ok(StoredBinding {
                    provider: row.get(0)?,
                    provider_subject: row.get(1)?,
                    household_id: row.get(2)?,
                    account_id: row.get(3)?,
                    parent_device_id: row.get(4)?,
                    child_profile_id: row.get(5)?,
                    child_device_id: row.get(6)?,
                    installation_id: row.get(7)?,
                    pairing_id: row.get(8)?,
                    route_id: row.get(9)?,
                    authority_generation: row.get(10)?,
                    session_generation: row.get(11)?,
                    device_trust_subject: row.get(12)?,
                    device_lifecycle_generation: row.get(13)?,
                    device_installation_binding_generation: row.get(14)?,
                    device_authority_generation: row.get(15)?,
                    preview_id: row.get(16)?,
                    apply_intent_digest: row.get(17)?,
                })
            },
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    Ok(row)
}

fn parse_state(value: &str) -> Result<StoredLifecycleState, ParentStorageConfirmationStoreError> {
    (value == "staged")
        .then_some(StoredLifecycleState::Staged)
        .or_else(|| (value == "consumed").then_some(StoredLifecycleState::Consumed))
        .or_else(|| (value == "expired").then_some(StoredLifecycleState::Expired))
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
}

fn validate_input(
    binding: &ParentStorageConfirmationBinding<'_>,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
) -> Result<(), ParentStorageConfirmationStoreError> {
    schema::validate_provider(provider_label(binding.provider))?;
    [
        binding.provider_subject,
        binding.household_id,
        binding.account_id,
        binding.parent_device_id,
        binding.child_profile_id,
        binding.child_device_id,
        binding.installation_id,
        binding.pairing_id,
        binding.route_id,
        binding.device_trust_subject,
        preview_id.as_str(),
    ]
    .into_iter()
    .try_for_each(schema::validate_identity)?;
    (binding.authority_generation > 0
        && binding.session_generation > 0
        && binding.device_lifecycle_generation > 0
        && binding.device_installation_binding_generation > 0
        && binding.device_authority_generation > 0)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::BindingMismatch)?;
    schema::validate_lower_hex(apply_intent_digest.as_str(), 64)
}

fn validate_current_binding(
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    expected: &ParentStorageConfirmationBinding<'_>,
) -> Result<(), ParentStorageConfirmationStoreError> {
    (device_binding.state() == DeviceTrustLifecycleState::Trusted
        && device_binding.lifecycle_generation() > 0
        && device_binding.installation_binding_generation() > 0
        && device_binding.authority_generation() > 0)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::DeviceTrustNotCurrent)?;
    let authority_matches = provider_label(authority.provider()) == expected.provider_label()
        && authority.provider_subject().as_str() == expected.provider_subject
        && authority.household_id().as_str() == expected.household_id
        && authority.account_id().as_str() == expected.account_id
        && authority.device_id().as_str() == expected.parent_device_id
        && authority.child_profile_id().as_str() == expected.child_profile_id
        && authority.child_device_id().as_str() == expected.child_device_id
        && authority.current_binding().installation_id.as_str() == expected.installation_id
        && authority.current_binding().pairing_id.as_str() == expected.pairing_id
        && authority.current_binding().selected_route_id.as_str() == expected.route_id
        && authority.authority_generation() == expected.authority_generation
        && authority.session_generation() == expected.session_generation;
    let device_matches = device_binding.family_id() == expected.household_id
        && device_binding.trust_subject() == expected.device_trust_subject
        && device_binding.parent_device_id() == expected.parent_device_id
        && device_binding.child_device_id() == expected.child_device_id
        && device_binding.installation_id() == expected.installation_id
        && device_binding.lifecycle_generation() == expected.device_lifecycle_generation
        && device_binding.installation_binding_generation()
            == expected.device_installation_binding_generation
        && device_binding.authority_generation() == expected.device_authority_generation;
    (authority_matches && device_matches)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::BindingMismatch)
}

impl ParentStorageConfirmationBinding<'_> {
    fn provider_label(&self) -> &'static str {
        provider_label(self.provider)
    }
}
