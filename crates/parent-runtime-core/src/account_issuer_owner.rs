//! Parent-runtime client boundary for the protected AccountIssuer service.
//!
//! Each operation opens and consumes one authenticated client RPC. The
//! parent facade retains no session, signer, authority, storage, or lifecycle
//! state and does not reinterpret the typed client contract.

use ocentra_protected_capability_custody_client::account_issuer::{
    AccountIssuerReceipt, AcknowledgeReceiptRequest, IssueCurrentAuthorityRequest,
};
use ocentra_protected_capability_custody_client::account_issuer_rpc::{
    AccountIssuerClientError, AccountIssuerRpc,
};

/// Issue the current Account authority through one fresh protected RPC.
pub fn issue_current_authority(
    request: IssueCurrentAuthorityRequest,
) -> Result<AccountIssuerReceipt, AccountIssuerClientError> {
    AccountIssuerRpc::connect()?.issue_current_authority(request)
}

/// Acknowledge one Account authority receipt through one fresh protected RPC.
pub fn acknowledge_receipt(
    request: AcknowledgeReceiptRequest,
) -> Result<AccountIssuerReceipt, AccountIssuerClientError> {
    AccountIssuerRpc::connect()?.acknowledge_receipt(request)
}
