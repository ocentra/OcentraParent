use std::fmt;

use ring::hmac;

use crate::constants;

use super::{AuthenticationDomain, AuthenticationTag, BootstrapAuthenticator, ProtocolError};

impl AuthenticationTag {
    pub(crate) fn from_tag(bytes: [u8; constants::AUTHENTICATION_TAG_BYTES]) -> Self {
        Self(bytes)
    }

    pub(crate) fn from_attestation_digest(digest: super::AttestationDigest) -> Self {
        Self(digest.0)
    }

    pub(crate) fn try_from_untrusted_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_authentication_tag_length)?;
        if bytes == [0_u8; constants::AUTHENTICATION_TAG_BYTES] {
            return Err(ProtocolError::InvalidAuthenticationTag);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; constants::AUTHENTICATION_TAG_BYTES] {
        &self.0
    }
}

impl fmt::Debug for AuthenticationTag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_AUTHENTICATION_TAG)
    }
}

impl AuthenticationDomain {
    fn label(self) -> &'static [u8] {
        match self {
            Self::BrokerAttestation => constants::BROKER_ATTESTATION_DOMAIN.as_bytes(),
            Self::Request => constants::REQUEST_DIGEST_DOMAIN.as_bytes(),
            Self::Response => constants::RESPONSE_DIGEST_DOMAIN.as_bytes(),
        }
    }
}

impl BootstrapAuthenticator {
    pub(crate) fn generate() -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; constants::BOOTSTRAP_AUTHENTICATOR_BYTES];
        getrandom::fill(&mut bytes).map_err(ProtocolError::from_randomness)?;
        if bytes == [0_u8; constants::BOOTSTRAP_AUTHENTICATOR_BYTES] {
            return Err(ProtocolError::InvalidBootstrap);
        }
        Ok(Self(zeroize::Zeroizing::new(bytes)))
    }

    pub(crate) fn try_from_bootstrap_bytes(value: &[u8]) -> Result<Self, ProtocolError> {
        let bytes = value
            .try_into()
            .map_err(ProtocolError::from_bootstrap_length)?;
        if bytes == [0_u8; constants::BOOTSTRAP_AUTHENTICATOR_BYTES] {
            return Err(ProtocolError::InvalidBootstrap);
        }
        Ok(Self(zeroize::Zeroizing::new(bytes)))
    }

    pub(crate) fn bootstrap_bytes(&self) -> &[u8; constants::BOOTSTRAP_AUTHENTICATOR_BYTES] {
        &self.0
    }

    pub(crate) fn authenticate(
        &self,
        domain: AuthenticationDomain,
        message: &[u8],
    ) -> Result<AuthenticationTag, ProtocolError> {
        let tag = hmac::sign(
            &self.authentication_key(),
            &authentication_message(domain, message)?,
        );
        AuthenticationTag::try_from_untrusted_bytes(tag.as_ref())
    }

    pub(crate) fn verify(
        &self,
        domain: AuthenticationDomain,
        message: &[u8],
        expected: AuthenticationTag,
    ) -> Result<(), ProtocolError> {
        hmac::verify(
            &self.authentication_key(),
            &authentication_message(domain, message)?,
            expected.as_bytes(),
        )
        .map_err(ProtocolError::from_authentication_failure)
    }

    fn authentication_key(&self) -> hmac::Key {
        hmac::Key::new(hmac::HMAC_SHA256, self.0.as_ref())
    }
}

impl fmt::Debug for BootstrapAuthenticator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(constants::DEBUG_BOOTSTRAP_AUTHENTICATOR)
    }
}

fn authentication_message(
    domain: AuthenticationDomain,
    message: &[u8],
) -> Result<Vec<u8>, ProtocolError> {
    if message.len() > constants::MAX_FRAME_BYTES {
        return Err(ProtocolError::FieldTooLarge);
    }
    let domain_label = domain.label();
    let mut canonical = Vec::with_capacity(
        constants::BOOTSTRAP_AUTH_DOMAIN.len()
            + domain_label.len()
            + message.len()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<u64>(),
    );
    canonical.extend_from_slice(constants::BOOTSTRAP_AUTH_DOMAIN.as_bytes());
    canonical.extend_from_slice(&(domain_label.len() as u32).to_be_bytes());
    canonical.extend_from_slice(domain_label);
    canonical.extend_from_slice(&(message.len() as u64).to_be_bytes());
    canonical.extend_from_slice(message);
    Ok(canonical)
}
