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

const ACCOUNT_OWNER_UNAVAILABLE_REASON: &str = "parent-local bridge Account owner unavailable";

#[derive(Clone)]
pub(crate) struct ParentLocalBridgeAdmission {
    account_owner: Arc<Mutex<AccountIdentityAuthorityService>>,
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
    pub(crate) fn mount_for_service(network: &NetworkPolicy) -> Option<Self> {
        match Self::mount_account_owned() {
            Ok(admission) if admission.is_ready() => Some(admission),
            Ok(_) => {
                let _ = crate::dev_log::write_agent_error(
                    constants::error::AGENT_SERVICE_RUNS,
                    startup_error_log_fields(
                        network,
                        StartupErrorReason(ACCOUNT_OWNER_UNAVAILABLE_REASON.to_owned()),
                    ),
                );
                None
            }
            Err(_) => {
                let _ = crate::dev_log::write_agent_error(
                    constants::error::AGENT_SERVICE_RUNS,
                    startup_error_log_fields(
                        network,
                        StartupErrorReason(ACCOUNT_OWNER_UNAVAILABLE_REASON.to_owned()),
                    ),
                );
                None
            }
        }
    }

    pub(crate) fn mount_account_owned() -> Result<Self, ParentLocalBridgeAdmissionError> {
        let account_owner = AccountIdentityAuthorityService::mount_account_owned()
            .map_err(|_| ParentLocalBridgeAdmissionError::OwnerUnavailable)?;
        Ok(Self {
            account_owner: Arc::new(Mutex::new(account_owner)),
            startup_recovered: false,
        })
    }

    /// Complete service admission only from an external provider-composed
    /// Account authority. Recovery runs before the admission becomes ready,
    /// so the service cannot expose a listener with unreconciled audit state.
    /// The authority is borrowed only for this bounded recovery call and is
    /// never retained or reconstructed from transport input.
    pub(crate) fn from_owner_composed(
        mut account_owner: AccountIdentityAuthorityService,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Self, ParentLocalBridgeAdmissionError> {
        account_owner
            .recover_parent_local_bridge_startup(current_authority)
            .map_err(|_| ParentLocalBridgeAdmissionError::StartupRecoveryRejected)?;
        Ok(Self {
            account_owner: Arc::new(Mutex::new(account_owner)),
            startup_recovered: true,
        })
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.startup_recovered
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
            .map_err(|_| ParentLocalBridgeAdmissionError::AuthenticationRejected)
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
            .map_err(|_| ParentLocalBridgeAdmissionError::CurrentnessRejected)
    }

    fn owner(
        &self,
    ) -> Result<MutexGuard<'_, AccountIdentityAuthorityService>, ParentLocalBridgeAdmissionError>
    {
        self.account_owner
            .lock()
            .map_err(|_| ParentLocalBridgeAdmissionError::OwnerUnavailable)
    }
}
