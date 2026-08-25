use crate::constants::{PROTOCOL_GENERATION, PROTOCOL_VERSION};

use std::fmt;

use super::{
    AttestationDigest, AuthenticationTag, BindingEpochs, ProtocolError, ProtocolGeneration,
    ProtocolVersion, SessionTranscriptDigest,
};

impl ProtocolVersion {
    pub const CURRENT: Self = Self(PROTOCOL_VERSION);

    pub fn current() -> Self {
        Self::CURRENT
    }

    pub fn value(self) -> u16 {
        self.0
    }

    pub(crate) fn decode(value: u16) -> Result<Self, ProtocolError> {
        if value != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion(value));
        }
        Ok(Self(value))
    }
}

impl ProtocolGeneration {
    pub const CURRENT: Self = Self(PROTOCOL_GENERATION);

    pub fn current() -> Self {
        Self::CURRENT
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub(crate) fn decode(value: u64) -> Result<Self, ProtocolError> {
        if value != PROTOCOL_GENERATION {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self(value))
    }
}

impl BindingEpochs {
    pub(crate) fn validate(&self) -> Result<(), ProtocolError> {
        if self.client_process_epoch == 0
            || self.broker_epoch == 0
            || self.broker_key_epoch == 0
            || self.writer_lease_epoch == 0
            || self.authority_generation == 0
            || self.target_generation == 0
            || self.key_generation == 0
            || self.writer_generation == 0
        {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(())
    }
}

impl AttestationDigest {
    pub(crate) fn from_authentication_tag(tag: AuthenticationTag) -> Self {
        Self(tag.0)
    }

    pub(crate) fn try_from_untrusted_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_attestation_length)?;
        if bytes == [0_u8; crate::constants::ATTESTATION_DIGEST_BYTES] {
            return Err(ProtocolError::InvalidAttestationDigest);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; crate::constants::ATTESTATION_DIGEST_BYTES] {
        &self.0
    }
}

impl fmt::Debug for AttestationDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(crate::constants::DEBUG_ATTESTATION_DIGEST)
    }
}

impl SessionTranscriptDigest {
    pub(crate) fn from_digest(bytes: [u8; crate::constants::TRANSCRIPT_DIGEST_BYTES]) -> Self {
        Self(bytes)
    }

    pub(crate) fn try_from_untrusted_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_transcript_length)?;
        if bytes == [0_u8; crate::constants::TRANSCRIPT_DIGEST_BYTES] {
            return Err(ProtocolError::InvalidTranscriptDigest);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; crate::constants::TRANSCRIPT_DIGEST_BYTES] {
        &self.0
    }
}

impl fmt::Debug for SessionTranscriptDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(crate::constants::DEBUG_TRANSCRIPT_DIGEST)
    }
}
