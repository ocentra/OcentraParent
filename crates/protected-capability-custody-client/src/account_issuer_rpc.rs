//! AccountIssuer v2 RPC facade over one authenticated broker session.

use crate::account_issuer::{
    AccountIssuerReceipt, AcknowledgeReceiptRequest, IssueCurrentAuthorityRequest,
};
use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;
use ocentra_protected_capability_custody_protocol::constants;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;

/// Errors deliberately distinguish deployment/manual-required states from a
/// transport or authentication failure. No unavailable broker path is turned
/// into a synthetic success receipt.
#[derive(Debug)]
pub enum AccountIssuerClientError {
    DeploymentRequired,
    ManualRequired,
    Transport,
    PeerAuthentication,
    Protocol(ProtocolError),
}

impl AccountIssuerClientError {
    pub(crate) fn from_client_error(error: ClientError) -> Self {
        match error {
            ClientError::BrokerUnavailable | ClientError::DeploymentRequired => {
                Self::DeploymentRequired
            }
            ClientError::UnsupportedPlatform => Self::ManualRequired,
            ClientError::Transport => Self::Transport,
            ClientError::PeerAuthentication => Self::PeerAuthentication,
            ClientError::Protocol(error) => Self::Protocol(error),
        }
    }
}

impl std::fmt::Display for AccountIssuerClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants::ERROR_CLIENT_PROTOCOL)
    }
}

impl std::error::Error for AccountIssuerClientError {}

/// One-shot typed AccountIssuer transport. The underlying authenticated pipe
/// session is consumed per operation so sequence and transcript bindings are
/// never reused accidentally.
pub struct AccountIssuerRpc {
    session: AuthenticatedBrokerSession,
}

impl AccountIssuerRpc {
    pub fn connect() -> Result<Self, AccountIssuerClientError> {
        crate::connect()
            .map(|session| Self { session })
            .map_err(AccountIssuerClientError::from_client_error)
    }

    pub fn issue_current_authority(
        self,
        request: IssueCurrentAuthorityRequest,
    ) -> Result<AccountIssuerReceipt, AccountIssuerClientError> {
        let request = request.into_protocol()?;
        self.session
            .execute_account_issuer(request)
            .map(AccountIssuerReceipt::from_protocol)
            .map_err(AccountIssuerClientError::from_client_error)
    }

    pub fn acknowledge_receipt(
        self,
        request: AcknowledgeReceiptRequest,
    ) -> Result<AccountIssuerReceipt, AccountIssuerClientError> {
        let request = request.into_protocol()?;
        self.session
            .execute_account_issuer(request)
            .map(AccountIssuerReceipt::from_protocol)
            .map_err(AccountIssuerClientError::from_client_error)
    }
}
