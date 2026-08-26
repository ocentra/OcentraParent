//! Durable receipt/outbox transition facade.

use ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityAuthorityIssuerTransaction;
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::{
        AccountIdentityIssuerReceiptProof, AccountIdentityIssuerRecordedTransport,
    };
use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport;

use crate::currentness::CurrentAuthority;
use crate::repository::AccountIssuerRepositoryError;

pub struct IssueTransaction<'a> {
    pub(crate) inner: AccountIdentityAuthorityIssuerTransaction<'a>,
}

impl<'a> IssueTransaction<'a> {
    pub fn record_transport(
        &mut self,
        current: &CurrentAuthority,
        transport: &AccountIdentityAuthorityProducerV2Transport,
    ) -> Result<AccountIdentityIssuerRecordedTransport, AccountIssuerRepositoryError> {
        self.inner
            .record_issued_transport(&current.inner, transport)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn acknowledge_receipt(
        &mut self,
        current: &CurrentAuthority,
        proof: &AccountIdentityIssuerReceiptProof,
    ) -> Result<(), AccountIssuerRepositoryError> {
        self.inner
            .acknowledge_receipt(&current.inner, proof)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn pending_count(&self) -> Result<u64, AccountIssuerRepositoryError> {
        self.inner
            .pending_outbox_count()
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn commit(self) -> Result<(), AccountIssuerRepositoryError> {
        self.inner
            .commit()
            .map_err(AccountIssuerRepositoryError::from)
    }
}
