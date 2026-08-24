use super::UntrustedRequest;

/// A move-only request whose exact handshake transcript, expiry, sequence, and
/// authentication tag have been verified. The decoded wire shape alone never
/// carries this authority.
pub struct AuthenticatedRequest {
    request: UntrustedRequest,
}

impl AuthenticatedRequest {
    pub(super) fn from_verified(request: UntrustedRequest) -> Self {
        Self { request }
    }

    pub fn as_untrusted(&self) -> &UntrustedRequest {
        &self.request
    }
}
