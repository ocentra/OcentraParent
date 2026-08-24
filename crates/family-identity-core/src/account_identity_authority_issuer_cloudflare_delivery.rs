//! Account-side Cloudflare delivery boundary.
//!
//! The Cloudflare consumer remains owned by its own workpack.  This module
//! only constructs a fully bound request from Account durable state and
//! accepts an acknowledgement after the owner port echoes every context
//! field.  The port is sealed and has no default implementation: an echo or
//! caller-selected marker cannot become authenticated evidence.

use std::sync::Arc;

use super::current_key_record::AccountIdentityIssuerCurrentPublicKeyRecord;
use super::outbox::{
    AccountIdentityIssuerDeliveryAcknowledgement, AccountIdentityIssuerDeliveryAttempt,
    AccountIdentityIssuerDeliveryOwnerAdapter,
};
use super::service_binding::{
    AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerServiceBinding,
    AccountIdentityIssuerServiceBindingAuthenticator,
};
use super::{AccountIdentityIssuer, AccountIdentityIssuerError};

mod sealed {
    pub trait Owner {}
}

/// The accepted implementation must be the authenticated Account-to-
/// Cloudflare service owner.  This crate intentionally supplies no process-
/// local, environment-key, or fallback implementation.
pub(crate) trait AccountIdentityIssuerCloudflareOwnerPort:
    sealed::Owner + Send + Sync
{
    fn authenticate_service_binding(
        &self,
        request: &AccountIdentityIssuerCloudflareBindingRequest,
    ) -> Result<AccountIdentityIssuerCloudflareBindingResponse, AccountIdentityIssuerError>;

    fn deliver_current_authority(
        &self,
        request: &AccountIdentityIssuerCloudflareDeliveryRequest,
    ) -> Result<AccountIdentityIssuerCloudflareDeliveryResponse, AccountIdentityIssuerError>;
}

pub(crate) struct AccountIdentityIssuerCloudflareBindingRequest {
    service_label: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    binding_id: String,
}

impl AccountIdentityIssuerCloudflareBindingRequest {
    fn from_binding(binding: &AccountIdentityIssuerServiceBinding) -> Self {
        Self {
            service_label: binding.service().label().to_owned(),
            account_id: binding.account_id().to_owned(),
            household_id: binding.household_id().to_owned(),
            authority_generation: binding.authority_generation(),
            binding_id: binding.binding_id().to_owned(),
        }
    }
}

pub(crate) struct AccountIdentityIssuerCloudflareBindingResponse {
    service_label: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    binding_id: String,
}

impl AccountIdentityIssuerCloudflareBindingResponse {
    fn matches(&self, request: &AccountIdentityIssuerCloudflareBindingRequest) -> bool {
        self.service_label == request.service_label
            && self.account_id == request.account_id
            && self.household_id == request.household_id
            && self.authority_generation == request.authority_generation
            && self.binding_id == request.binding_id
    }
}

pub(crate) struct AccountIdentityIssuerCloudflareDeliveryRequest {
    receipt_id: String,
    claim_id: String,
    service_label: String,
    binding_id: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    wire: Vec<u8>,
    wire_digest: String,
    current_key_record: AccountIdentityIssuerCurrentPublicKeyRecord,
}

impl AccountIdentityIssuerCloudflareDeliveryRequest {
    fn from_attempt(
        attempt: &AccountIdentityIssuerDeliveryAttempt,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let (
            receipt_id,
            claim_id,
            service,
            binding_id,
            account_id,
            household_id,
            authority_generation,
            wire_bytes,
            current_key_record,
        ) = attempt.cloudflare_delivery_parts();
        let current_key_record = current_key_record.clone();
        current_key_record.validate()?;
        if !current_key_record.matches_context(
            service,
            binding_id,
            account_id,
            household_id,
            authority_generation,
        ) {
            return Err(AccountIdentityIssuerError::BindingMismatch);
        }
        let wire = wire_bytes.to_vec();
        let wire_digest = digest_wire(&wire);
        Ok(Self {
            receipt_id: receipt_id.to_owned(),
            claim_id: claim_id.to_owned(),
            service_label: service.label().to_owned(),
            binding_id: binding_id.to_owned(),
            account_id: account_id.to_owned(),
            household_id: household_id.to_owned(),
            authority_generation,
            wire,
            wire_digest,
            current_key_record,
        })
    }
}

pub(crate) struct AccountIdentityIssuerCloudflareDeliveryResponse {
    receipt_id: String,
    claim_id: String,
    binding_id: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    wire_digest: String,
    current_key_record_digest: String,
    acknowledgement_id: String,
}

impl AccountIdentityIssuerCloudflareDeliveryResponse {
    fn matches(&self, request: &AccountIdentityIssuerCloudflareDeliveryRequest) -> bool {
        self.receipt_id == request.receipt_id
            && self.claim_id == request.claim_id
            && self.binding_id == request.binding_id
            && self.account_id == request.account_id
            && self.household_id == request.household_id
            && self.authority_generation == request.authority_generation
            && self.wire_digest == request.wire_digest
            && self.current_key_record_digest == request.current_key_record.record_digest()
    }
}

/// A paired owner adapter for binding authentication and idempotent delivery.
/// Both adapters share one owner port so a delivery cannot be installed with a
/// different authentication authority by accident.
pub(crate) struct AccountIdentityIssuerCloudflareDelivery {
    owner: Arc<dyn AccountIdentityIssuerCloudflareOwnerPort>,
}

impl AccountIdentityIssuerCloudflareDelivery {
    pub(crate) fn from_owner(owner: Arc<dyn AccountIdentityIssuerCloudflareOwnerPort>) -> Self {
        Self { owner }
    }

    pub(crate) fn install_into(self, issuer: &mut AccountIdentityIssuer) {
        issuer.install_binding_authenticator(Box::new(CloudflareBindingAuthenticator {
            owner: Arc::clone(&self.owner),
        }));
        issuer.install_delivery_owner(Box::new(CloudflareDeliveryOwner { owner: self.owner }));
    }
}

struct CloudflareBindingAuthenticator {
    owner: Arc<dyn AccountIdentityIssuerCloudflareOwnerPort>,
}

impl AccountIdentityIssuerServiceBindingAuthenticator for CloudflareBindingAuthenticator {
    fn authenticate(
        &self,
        binding: &AccountIdentityIssuerServiceBinding,
    ) -> Result<AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerError> {
        let request = AccountIdentityIssuerCloudflareBindingRequest::from_binding(binding);
        let response = self.owner.authenticate_service_binding(&request)?;
        if !response.matches(&request) {
            return Err(AccountIdentityIssuerError::ServiceBindingRejected);
        }
        Ok(AccountIdentityIssuerAuthenticatedBinding::from_authenticated_owner(binding))
    }
}

struct CloudflareDeliveryOwner {
    owner: Arc<dyn AccountIdentityIssuerCloudflareOwnerPort>,
}

impl AccountIdentityIssuerDeliveryOwnerAdapter for CloudflareDeliveryOwner {
    fn deliver(
        &self,
        attempt: &AccountIdentityIssuerDeliveryAttempt,
    ) -> Result<AccountIdentityIssuerDeliveryAcknowledgement, AccountIdentityIssuerError> {
        let request = AccountIdentityIssuerCloudflareDeliveryRequest::from_attempt(attempt)?;
        let response = self.owner.deliver_current_authority(&request)?;
        if !response.matches(&request) {
            return Err(AccountIdentityIssuerError::DeliveryAcknowledgementRejected);
        }
        AccountIdentityIssuerDeliveryAcknowledgement::new(attempt, response.acknowledgement_id)
    }
}

fn digest_wire(wire: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let mut digest = Sha256::new();
    digest.update(b"ocentra.account-issuer.cloudflare-wire.v1\0");
    digest.update((wire.len() as u64).to_be_bytes());
    digest.update(wire);
    format!("sha256:{:x}", digest.finalize())
}
