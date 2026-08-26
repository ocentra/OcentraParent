use ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Receipt;

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;

use super::super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerOutboxClaim, ProtectedAccountIssuerKeyRegistration,
};
use super::super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};

impl AccountIdentityAuthorityIssuerClient {
    pub fn current_key(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        let transaction = self.begin_transaction()?;
        let result = transaction.current_key(currentness)?;
        transaction.commit()?;
        Ok(result)
    }

    pub fn register_protected_key(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        registration: &ProtectedAccountIssuerKeyRegistration,
    ) -> Result<AccountIdentityIssuerV2KeyRecord, AccountIdentityAuthorityIssuerClientError> {
        let mut transaction = self.begin_transaction()?;
        let result = transaction.register_protected_key(currentness, registration)?;
        transaction.commit()?;
        Ok(result)
    }

    pub fn prepare_acknowledge_receipt(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
    ) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityIssuerClientError>
    {
        let transaction = self.begin_transaction()?;
        let request = transaction.prepare_acknowledge_receipt(currentness, claim)?;
        transaction.commit()?;
        Ok(request)
    }

    pub fn acknowledge_receipt(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError>
    {
        let mut transaction = self.begin_transaction()?;
        let receipt =
            transaction.acknowledge_receipt(currentness, claim, protected_receipt_wire)?;
        transaction.commit()?;
        Ok(receipt)
    }
}
