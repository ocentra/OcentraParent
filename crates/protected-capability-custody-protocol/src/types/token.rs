use std::fmt;

use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::constants;

use super::{OpaquePreparedToken, ProtocolError};

impl OpaquePreparedToken {
    pub fn from_untrusted_wire_bytes(value: Vec<u8>) -> Result<Self, ProtocolError> {
        let value = Zeroizing::new(value);
        if value.len() != constants::OPAQUE_TOKEN_BYTES {
            return Err(ProtocolError::InvalidOpaqueToken);
        }
        let mut bytes = [0_u8; constants::OPAQUE_TOKEN_BYTES];
        bytes.copy_from_slice(value.as_slice());
        if bytes == [0_u8; constants::OPAQUE_TOKEN_BYTES] {
            return Err(ProtocolError::InvalidOpaqueToken);
        }
        Ok(Self(Zeroizing::new(bytes)))
    }

    pub fn digest(&self) -> [u8; constants::REQUEST_DIGEST_BYTES] {
        let mut digest = Sha256::new();
        digest.update(constants::OPAQUE_TOKEN_DIGEST_DOMAIN.as_bytes());
        digest.update((self.0.len() as u32).to_be_bytes());
        digest.update(self.0.as_ref());
        digest.finalize().into()
    }

    pub(crate) fn as_bytes(&self) -> &[u8; constants::OPAQUE_TOKEN_BYTES] {
        &self.0
    }
}

impl fmt::Debug for OpaquePreparedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_OPAQUE_TOKEN)
    }
}
