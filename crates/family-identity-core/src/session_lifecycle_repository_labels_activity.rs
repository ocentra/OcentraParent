use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use super::StoredLabel;
use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::record::SessionCredentialClass;

pub(super) fn credential_class(value: SessionCredentialClass) -> StoredLabel {
    match value {
        SessionCredentialClass::BrowserUserSession => StoredLabel("browser-user-session"),
    }
}

pub(super) fn provider(value: &AccountIdentityProvider) -> StoredLabel {
    match value {
        AccountIdentityProvider::Authjs => StoredLabel("authjs"),
        AccountIdentityProvider::Firebase => StoredLabel("firebase"),
    }
}

pub(super) fn activity(value: SessionActivityState) -> StoredLabel {
    match value {
        SessionActivityState::Active => StoredLabel("active"),
        SessionActivityState::LoggedOut => StoredLabel("logged-out"),
        SessionActivityState::Revoked => StoredLabel("revoked"),
        SessionActivityState::GloballyRevoked => StoredLabel("globally-revoked"),
    }
}
