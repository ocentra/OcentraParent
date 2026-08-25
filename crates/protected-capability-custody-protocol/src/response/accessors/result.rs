use crate::request::RequestKind;
use crate::types::{AuthenticationTag, OpaquePreparedToken};

use super::super::{ObservedGenerations, ResponseStatus, UntrustedResponse};

impl UntrustedResponse {
    pub fn request_kind(&self) -> RequestKind {
        self.request_kind
    }

    pub fn request_digest(&self) -> [u8; crate::constants::REQUEST_DIGEST_BYTES] {
        self.request_digest
    }

    pub fn status(&self) -> ResponseStatus {
        self.status
    }

    pub fn observed_generations(&self) -> Option<ObservedGenerations> {
        self.observed_generations
    }

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
