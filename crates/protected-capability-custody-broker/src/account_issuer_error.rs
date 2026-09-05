use ocentra_account_issuer_owner::repository::AccountIssuerRepositoryError;
use ocentra_account_issuer_owner::rpc::AccountIssuerRpcError;
use ocentra_account_issuer_owner::signing::AccountIssuerSigningError;

use crate::BrokerError;

pub(super) fn map_owner_error(error: &AccountIssuerRpcError) -> BrokerError {
    match error {
        AccountIssuerRpcError::ProtectedAdmissionRejected => BrokerError::Request,
        AccountIssuerRpcError::Signing(AccountIssuerSigningError::OwnerUnavailable)
        | AccountIssuerRpcError::Repository(AccountIssuerRepositoryError::Unavailable) => {
            BrokerError::DeploymentRequired
        }
        AccountIssuerRpcError::Signing(AccountIssuerSigningError::Rejected)
        | AccountIssuerRpcError::Repository(AccountIssuerRepositoryError::SigningRejected)
        | AccountIssuerRpcError::Repository(_)
        | AccountIssuerRpcError::Delivery(_) => BrokerError::Request,
    }
}
