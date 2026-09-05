#![forbid(unsafe_code)]

//! Account-owned admission for the parent desktop transport.

use std::sync::{Arc, Mutex, MutexGuard};

use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_family_identity_core::account_identity_authority_repository::AccountIdentityAuthorityService;
use ocentra_family_identity_core::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use ocentra_parent_agent_protocol::constants;
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeHandshake;

use crate::{
    network::NetworkPolicy,
    service_runtime::{startup_error_log_fields, StartupErrorReason},
};

#[path = "parent_local_bridge_admission_startup_recovery.rs"]
mod startup_recovery;

const ACCOUNT_OWNER_UNAVAILABLE_REASON: &str = "parent-local bridge Account owner unavailable";

#[derive(Clone)]
pub(crate) struct ParentLocalBridgeAdmission {
    account_owner: Option<Arc<Mutex<AccountIdentityAuthorityService>>>,
    startup_recovered: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParentLocalBridgeAdmissionError {
    OwnerUnavailable,
    AuthenticationRejected,
    CurrentnessRejected,
    StartupRecoveryRejected,
}

impl ParentLocalBridgeAdmission {
    pub(crate) fn mount_for_service(network: &NetworkPolicy) -> Self {
        match Self::mount_account_owned() {
            Ok(admission) if admission.is_ready() => admission,
            Ok(_) | Err(_) => {
                let _ = crate::dev_log::write_agent_error(
                    constants::error::AGENT_SERVICE_RUNS,
                    startup_error_log_fields(
                        network,
                        StartupErrorReason(ACCOUNT_OWNER_UNAVAILABLE_REASON.to_owned()),
                    ),
                );
                Self::unavailable()
            }
        }
    }

    pub(crate) fn mount_account_owned() -> Result<Self, ParentLocalBridgeAdmissionError> {
        let account_owner = AccountIdentityAuthorityService::mount_account_owned()
            .map_err(|_error| ParentLocalBridgeAdmissionError::OwnerUnavailable)?;
        Ok(Self {
            account_owner: Some(Arc::new(Mutex::new(account_owner))),
            startup_recovered: false,
        })
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.startup_recovered && self.account_owner.is_some()
    }

    pub(crate) fn authenticate(
        &self,
        handshake: &AccountIdentityParentLocalBridgeHandshake,
    ) -> Result<AuthenticatedParentLocalBridgeSession, ParentLocalBridgeAdmissionError> {
        if !self.is_ready() {
            return Err(ParentLocalBridgeAdmissionError::OwnerUnavailable);
        }
        self.owner()?
            .authenticate_parent_local_bridge_handshake(handshake)
            .map_err(|_error| ParentLocalBridgeAdmissionError::AuthenticationRejected)
    }

    pub(crate) fn revalidate(
        &self,
        authenticated: &AuthenticatedParentLocalBridgeSession,
    ) -> Result<(), ParentLocalBridgeAdmissionError> {
        if !self.is_ready() {
            return Err(ParentLocalBridgeAdmissionError::OwnerUnavailable);
        }
        self.owner()?
            .revalidate_parent_local_bridge_session(authenticated)
            .map_err(|_error| ParentLocalBridgeAdmissionError::CurrentnessRejected)
    }

    fn owner(
        &self,
    ) -> Result<MutexGuard<'_, AccountIdentityAuthorityService>, ParentLocalBridgeAdmissionError>
    {
        self.account_owner
            .as_ref()
            .ok_or(ParentLocalBridgeAdmissionError::OwnerUnavailable)?
            .lock()
            .map_err(|_error| ParentLocalBridgeAdmissionError::OwnerUnavailable)
    }

    fn unavailable() -> Self {
        Self {
            account_owner: None,
            startup_recovered: false,
        }
    }
}

/// Complete admission only from an external provider-composed Account owner
/// and opaque current authority. Recovery finishes before the result becomes
/// ready; the authority is borrowed only for that bounded recovery operation.
impl<'a>
    TryFrom<(
        AccountIdentityAuthorityService,
        &'a VerifiedAccountIdentityAuthority,
    )> for ParentLocalBridgeAdmission
{
    type Error = ParentLocalBridgeAdmissionError;

    fn try_from(
        (mut account_owner, current_authority): (
            AccountIdentityAuthorityService,
            &'a VerifiedAccountIdentityAuthority,
        ),
    ) -> Result<Self, Self::Error> {
        startup_recovery::complete(&mut account_owner, current_authority)?;
        Ok(Self {
            account_owner: Some(Arc::new(Mutex::new(account_owner))),
            startup_recovered: true,
        })
    }
}
