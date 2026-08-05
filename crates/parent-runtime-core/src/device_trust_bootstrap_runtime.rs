//! Parent-runtime composition for the Windows-local device-trust sealing slice.

use std::{collections::HashMap, path::Path, sync::Mutex};

use getrandom::fill;
use ocentra_family_identity_core::{
    parent_presence::ParentPresenceVerificationAccepted,
    trust_bootstrap::{
        begin_parent_device_key_sealing, TrustBootstrapDecision, TrustBootstrapManualRequirement,
        TrustBootstrapRejection,
    },
};
use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error as CustodyError, WindowsDeviceTrustCustody,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentDeviceTrustBootstrapResult {
    Sealed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentDeviceTrustBootstrapError {
    ParentPresenceRejected(TrustBootstrapRejection),
    ManualRequired(TrustBootstrapManualRequirement),
    Custody(CustodyError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentDeviceTrustCommandError {
    InvalidStagingRequest,
    HandleGeneration,
    UnknownOrConsumedCeremony,
    Runtime(ParentDeviceTrustBootstrapError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentDeviceTrustStagedCeremonyRef(String);

impl ParentDeviceTrustStagedCeremonyRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

struct StagedParentDeviceTrustCeremony {
    trust_bootstrap_ref: String,
    parent_presence: ParentPresenceVerificationAccepted,
}

/// Production parent-runtime owner for the accepted-ceremony to platform-custody boundary.
///
/// The runtime accepts the opaque consumed ceremony rather than caller-provided family,
/// account, or device bindings. The custody adapter receives only the resulting authorized
/// sealing request and makes the record active only after its durable write succeeds.
pub struct ParentDeviceTrustBootstrapRuntime {
    custody: WindowsDeviceTrustCustody,
}

impl ParentDeviceTrustBootstrapRuntime {
    pub fn open(custody_root: impl AsRef<Path>) -> Result<Self, ParentDeviceTrustBootstrapError> {
        WindowsDeviceTrustCustody::open(custody_root)
            .map(|custody| Self { custody })
            .map_err(ParentDeviceTrustBootstrapError::Custody)
    }

    pub fn seal_verified_parent_device_trust(
        &self,
        trust_bootstrap_ref: String,
        parent_presence: ParentPresenceVerificationAccepted,
        parent_device_trust_material: &[u8],
    ) -> Result<ParentDeviceTrustBootstrapResult, ParentDeviceTrustBootstrapError> {
        match begin_parent_device_key_sealing(trust_bootstrap_ref, parent_presence) {
            TrustBootstrapDecision::AwaitingPlatformKeySealing(request) => self
                .custody
                .seal_persist_activate(request, parent_device_trust_material)
                .map(|()| ParentDeviceTrustBootstrapResult::Sealed)
                .map_err(ParentDeviceTrustBootstrapError::Custody),
            TrustBootstrapDecision::Rejected(rejection) => Err(
                ParentDeviceTrustBootstrapError::ParentPresenceRejected(rejection),
            ),
            TrustBootstrapDecision::ManualRequired(requirement) => {
                Err(ParentDeviceTrustBootstrapError::ManualRequired(requirement))
            }
        }
    }

    pub fn unseal_current_parent_device_trust(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<Vec<u8>, ParentDeviceTrustBootstrapError> {
        self.custody
            .unseal_current(family_id, parent_account_id, device_ref)
            .map_err(ParentDeviceTrustBootstrapError::Custody)
    }

    pub fn revoke_or_reset_parent_device_trust(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<(), ParentDeviceTrustBootstrapError> {
        self.custody
            .revoke_or_reset(family_id, parent_account_id, device_ref)
            .map_err(ParentDeviceTrustBootstrapError::Custody)
    }
}

/// Native parent-command facade for a one-shot accepted ceremony.
///
/// An upstream native parent-presence owner stages the opaque accepted ceremony.
/// The parent desktop dispatch path can consume only that random handle; it never
/// receives an accepted ceremony or trust material from the webview payload.
pub struct ParentDeviceTrustCommandFacade {
    runtime: ParentDeviceTrustBootstrapRuntime,
    staged_ceremonies: Mutex<HashMap<String, StagedParentDeviceTrustCeremony>>,
}

impl ParentDeviceTrustCommandFacade {
    pub fn open(custody_root: impl AsRef<Path>) -> Result<Self, ParentDeviceTrustCommandError> {
        ParentDeviceTrustBootstrapRuntime::open(custody_root)
            .map(|runtime| Self {
                runtime,
                staged_ceremonies: Mutex::new(HashMap::new()),
            })
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }

    pub fn stage_accepted_parent_device_trust_ceremony(
        &self,
        trust_bootstrap_ref: String,
        parent_presence: ParentPresenceVerificationAccepted,
    ) -> Result<ParentDeviceTrustStagedCeremonyRef, ParentDeviceTrustCommandError> {
        if trust_bootstrap_ref.trim().is_empty() {
            return Err(ParentDeviceTrustCommandError::InvalidStagingRequest);
        }

        let mut staged_ceremonies = self
            .staged_ceremonies
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        loop {
            let ceremony_ref = random_ceremony_ref()?;
            if staged_ceremonies.contains_key(&ceremony_ref) {
                continue;
            }
            staged_ceremonies.insert(
                ceremony_ref.clone(),
                StagedParentDeviceTrustCeremony {
                    trust_bootstrap_ref,
                    parent_presence,
                },
            );
            return Ok(ParentDeviceTrustStagedCeremonyRef(ceremony_ref));
        }
    }

    pub fn seal_staged_parent_device_trust(
        &self,
        ceremony_ref: &str,
    ) -> Result<ParentDeviceTrustBootstrapResult, ParentDeviceTrustCommandError> {
        let staged = self
            .staged_ceremonies
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(ceremony_ref)
            .ok_or(ParentDeviceTrustCommandError::UnknownOrConsumedCeremony)?;
        let mut material = [0_u8; 32];
        fill(&mut material).map_err(|_error| ParentDeviceTrustCommandError::HandleGeneration)?;
        self.runtime
            .seal_verified_parent_device_trust(
                staged.trust_bootstrap_ref,
                staged.parent_presence,
                &material,
            )
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }

    pub fn unseal_current_parent_device_trust(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<Vec<u8>, ParentDeviceTrustCommandError> {
        self.runtime
            .unseal_current_parent_device_trust(family_id, parent_account_id, device_ref)
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }

    pub fn revoke_or_reset_parent_device_trust(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<(), ParentDeviceTrustCommandError> {
        self.runtime
            .revoke_or_reset_parent_device_trust(family_id, parent_account_id, device_ref)
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }
}

fn random_ceremony_ref() -> Result<String, ParentDeviceTrustCommandError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| ParentDeviceTrustCommandError::HandleGeneration)?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
