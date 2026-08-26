//! Broker lifetime and dispatch boundary for AccountIssuer v2.
//!
//! The typed issuer is mounted once with the broker custody service and is
//! serialized behind a mutex because a durable issue operation mutates the
//! Account-owned repository. A failed mount remains an explicit deployment
//! state; it is never replaced by a memory-only signer or synthetic receipt.

use std::sync::Mutex;

use ocentra_protected_capability_custody_protocol::account_issuer::AccountIssuerReceipt;
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;

use crate::account_issuer::BrokerAccountIssuer;
use crate::BrokerError;

pub(crate) struct BrokerAccountIssuerRpc {
    state: Mutex<BrokerAccountIssuerState>,
}

enum BrokerAccountIssuerState {
    Ready(BrokerAccountIssuer),
    DeploymentRequired,
}

impl BrokerAccountIssuerRpc {
    pub(crate) fn open() -> Self {
        let state = match BrokerAccountIssuer::mount() {
            Ok(issuer) => BrokerAccountIssuerState::Ready(issuer),
            Err(BrokerError::DeploymentRequired) | Err(BrokerError::UnsupportedPlatform) => {
                BrokerAccountIssuerState::DeploymentRequired
            }
            Err(_) => BrokerAccountIssuerState::DeploymentRequired,
        };
        Self {
            state: Mutex::new(state),
        }
    }

    pub(crate) fn execute(
        &self,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerReceipt, BrokerError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| BrokerError::DeploymentRequired)?;
        match &mut *state {
            BrokerAccountIssuerState::Ready(issuer) => issuer.execute(request),
            BrokerAccountIssuerState::DeploymentRequired => Err(BrokerError::DeploymentRequired),
        }
    }
}
