use sha2::{Digest, Sha256};

use crate::constants::{OPAQUE_TOKEN_DIGEST_DOMAIN, REQUEST_DIGEST_BYTES, REQUEST_DIGEST_DOMAIN};
use crate::target::{Action, TargetDescriptor};

use super::{Request, RequestKind};

impl Request {
    pub fn request_digest(&self) -> [u8; REQUEST_DIGEST_BYTES] {
        let mut digest = Sha256::new();
        digest.update(REQUEST_DIGEST_DOMAIN.as_bytes());
        digest.update(self.version().value().to_be_bytes());
        digest.update([self.kind() as u8]);
        append_digest_field(&mut digest, self.operation());
        digest.update([self.action() as u8]);
        digest.update([self.target().kind() as u8]);
        append_digest_field(&mut digest, self.target().household());
        append_digest_field(&mut digest, self.target().device());
        append_digest_field(&mut digest, self.target().target());
        digest.update(opaque_token_digest(self.opaque_token()));
        digest.finalize().into()
    }

    pub fn kind(&self) -> RequestKind {
        self.kind
    }

    pub fn operation(&self) -> &[u8] {
        &self.operation
    }

    pub fn action(&self) -> Action {
        self.action
    }

    pub fn target(&self) -> &TargetDescriptor {
        &self.target
    }

    pub fn opaque_token(&self) -> &[u8] {
        &self.opaque_token
    }
}

fn append_digest_field(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u32).to_be_bytes());
    digest.update(value);
}

fn opaque_token_digest(value: &[u8]) -> [u8; REQUEST_DIGEST_BYTES] {
    let mut digest = Sha256::new();
    digest.update(OPAQUE_TOKEN_DIGEST_DOMAIN.as_bytes());
    append_digest_field(&mut digest, value);
    digest.finalize().into()
}
