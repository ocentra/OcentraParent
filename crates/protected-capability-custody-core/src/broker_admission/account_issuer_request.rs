//! One-shot binding between a retained broker transcript and one AccountIssuer request.

use std::time::{Duration, Instant};

use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;

use super::BrokerRuntimeError;

pub(super) struct AccountIssuerRequestBinding {
    request_digest:
        [u8; ocentra_protected_capability_custody_protocol::constants::REQUEST_DIGEST_BYTES],
    authorized_at: Instant,
}

/// Move-only Protected proof for one exact authenticated AccountIssuer request.
///
/// The sole constructor is the broker runtime after it revalidates the retained
/// OS peer, immutable enrollment, and broker service. No request field, boolean,
/// path, identity selector, or generation can construct this admission.
pub struct ProtectedAccountIssuerRequestAdmission {
    #[cfg(windows)]
    _platform: super::platform::BrokerAuthorizedPeer,
    binding: AccountIssuerRequestBinding,
}

impl AccountIssuerRequestBinding {
    pub(super) fn new(request: &AuthenticatedAccountIssuerRequest, authorized_at: Instant) -> Self {
        Self {
            request_digest: request.request_digest(),
            authorized_at,
        }
    }

    pub(super) fn verify_at(
        &self,
        request: &AuthenticatedAccountIssuerRequest,
        now: Instant,
    ) -> Result<(), BrokerRuntimeError> {
        let maximum_age = Duration::from_millis(
            ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS,
        );
        let current = now
            .checked_duration_since(self.authorized_at)
            .is_some_and(|age| age <= maximum_age);
        if !current || request.request_digest() != self.request_digest {
            return Err(BrokerRuntimeError::InvalidRequest);
        }
        Ok(())
    }
}

impl ProtectedAccountIssuerRequestAdmission {
    #[cfg(windows)]
    pub(super) fn from_authorized_peer(
        platform: super::platform::BrokerAuthorizedPeer,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Self {
        Self {
            _platform: platform,
            binding: AccountIssuerRequestBinding::new(request, Instant::now()),
        }
    }

    /// Consume the admission only for the authenticated request whose full
    /// session-and-operation digest was bound at authorization time.
    pub fn verify_and_consume(
        self,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<(), BrokerRuntimeError> {
        self.binding.verify_at(request, Instant::now())
    }
}
