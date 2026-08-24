use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::{
    currentness, key_registry, outbox, service_binding, AccountIdentityIssuer,
    AccountIdentityIssuerError,
};

impl AccountIdentityIssuer {
    /// Claim one pending durable handoff. A failed/missing owner adapter leaves
    /// the claimed row recoverable after its lease; only owner-minted evidence
    /// can transition it to acknowledged.
    pub(crate) fn deliver_next_pending(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<bool, AccountIdentityIssuerError> {
        if self.delivery_owner.is_none() {
            return Err(AccountIdentityIssuerError::DeliveryUnavailable);
        }
        let attempt = self.claim_pending_delivery(authority, service)?;
        let Some(attempt) = attempt else {
            return Ok(false);
        };
        let acknowledgement = self.deliver_or_release(authority, service, &attempt)?;
        self.acknowledge_delivery(authority, service, &attempt, &acknowledgement)?;
        Ok(true)
    }

    fn deliver_or_release(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
        attempt: &outbox::AccountIdentityIssuerDeliveryAttempt,
    ) -> Result<outbox::AccountIdentityIssuerDeliveryAcknowledgement, AccountIdentityIssuerError>
    {
        let delivery_result = self
            .delivery_owner
            .as_deref()
            .ok_or(AccountIdentityIssuerError::DeliveryUnavailable)
            .and_then(|owner| owner.deliver(attempt));
        match delivery_result {
            Ok(acknowledgement) => Ok(acknowledgement),
            Err(error) => {
                self.release_delivery_claim(authority, service, attempt)?;
                Err(error)
            }
        }
    }

    fn release_delivery_claim(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
        attempt: &outbox::AccountIdentityIssuerDeliveryAttempt,
    ) -> Result<(), AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        outbox::release(&transaction, authority, &binding, attempt)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)
    }

    fn claim_pending_delivery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<Option<outbox::AccountIdentityIssuerDeliveryAttempt>, AccountIdentityIssuerError>
    {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        let registered = key_registry::current(&transaction, authority, &binding)?;
        let attempt =
            outbox::claim::claim_next(&transaction, authority, &binding, &registered, now)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        Ok(attempt)
    }

    fn acknowledge_delivery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
        attempt: &outbox::AccountIdentityIssuerDeliveryAttempt,
        acknowledgement: &outbox::AccountIdentityIssuerDeliveryAcknowledgement,
    ) -> Result<(), AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        let registered = key_registry::current(&transaction, authority, &binding)?;
        super::transport::verify(attempt.wire_bytes(), authority, &binding, &registered, now)?;
        outbox::reconcile::reconcile_for_current(&transaction, authority, &binding, now)?;
        outbox::acknowledge_claim(
            &transaction,
            authority,
            &binding,
            attempt,
            acknowledgement,
            now,
        )?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)
    }
}
