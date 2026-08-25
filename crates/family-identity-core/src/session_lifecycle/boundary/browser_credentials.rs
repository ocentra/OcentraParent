#![forbid(unsafe_code)]

//! Raw browser credential transport boundary.

use crate::session_lifecycle_record::SessionId;

const ACCESS_BEARER_PREFIX: &str = "ocentra_access_";
const REFRESH_BEARER_PREFIX: &str = "ocentra_refresh_";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionCredentialInputError {
    WrongCredentialClass,
    InvalidEncoding,
}

pub struct PresentedBrowserAccessCredential(String);

impl PresentedBrowserAccessCredential {
    pub fn parse(value: impl Into<String>) -> Result<Self, SessionCredentialInputError> {
        parse_presented_credential(value.into(), ACCESS_BEARER_PREFIX).map(Self)
    }

    pub(crate) fn bearer(&self) -> &str {
        &self.0
    }
}

pub struct PresentedBrowserRefreshCredential(String);

impl PresentedBrowserRefreshCredential {
    pub fn parse(value: impl Into<String>) -> Result<Self, SessionCredentialInputError> {
        parse_presented_credential(value.into(), REFRESH_BEARER_PREFIX).map(Self)
    }

    pub(crate) fn bearer(&self) -> &str {
        &self.0
    }
}

pub struct IssuedBrowserAccessCredential(String);

impl IssuedBrowserAccessCredential {
    pub(crate) fn from_bearer(value: String) -> Self {
        Self(value)
    }

    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

pub struct IssuedBrowserRefreshCredential(String);

impl IssuedBrowserRefreshCredential {
    pub(crate) fn from_bearer(value: String) -> Self {
        Self(value)
    }

    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

pub struct IssuedBrowserSession {
    session_id: SessionId,
    access_credential: IssuedBrowserAccessCredential,
    refresh_credential: IssuedBrowserRefreshCredential,
    access_expires_at_epoch_millis: i64,
    refresh_expires_at_epoch_millis: i64,
}

impl IssuedBrowserSession {
    pub(crate) fn new(
        session_id: SessionId,
        access_credential: IssuedBrowserAccessCredential,
        refresh_credential: IssuedBrowserRefreshCredential,
        access_expires_at_epoch_millis: i64,
        refresh_expires_at_epoch_millis: i64,
    ) -> Self {
        Self {
            session_id,
            access_credential,
            refresh_credential,
            access_expires_at_epoch_millis,
            refresh_expires_at_epoch_millis,
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn access_credential(&self) -> &IssuedBrowserAccessCredential {
        &self.access_credential
    }

    pub fn refresh_credential(&self) -> &IssuedBrowserRefreshCredential {
        &self.refresh_credential
    }

    pub fn access_expires_at_epoch_millis(&self) -> i64 {
        self.access_expires_at_epoch_millis
    }

    pub fn refresh_expires_at_epoch_millis(&self) -> i64 {
        self.refresh_expires_at_epoch_millis
    }
}

fn parse_presented_credential(
    value: String,
    prefix: &str,
) -> Result<String, SessionCredentialInputError> {
    if !value.starts_with(prefix) {
        return Err(SessionCredentialInputError::WrongCredentialClass);
    }
    let suffix = &value[prefix.len()..];
    if !super::storage_values::digest_is_valid(suffix) {
        return Err(SessionCredentialInputError::InvalidEncoding);
    }
    Ok(value)
}
