use crate::types::{AuthenticationTag, OpaquePreparedToken};

use super::UntrustedRequest;

mod operation;
mod session;

impl UntrustedRequest {
    pub fn opaque_token_digest(&self) -> Option<[u8; 32]> {
        self.opaque_token.as_ref().map(OpaquePreparedToken::digest)
    }

    pub fn into_opaque_token(self) -> Option<OpaquePreparedToken> {
        self.opaque_token
    }

    pub(crate) fn opaque_token(&self) -> Option<&OpaquePreparedToken> {
        self.opaque_token.as_ref()
    }

    pub(crate) fn authentication_tag(&self) -> AuthenticationTag {
        self.authentication_tag
    }
}
