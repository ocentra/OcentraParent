//! Parent-runtime composition for the Windows-local device-trust sealing slice.

use std::{
    collections::HashMap,
    fmt,
    path::Path,
    sync::Mutex,
    time::{Duration, Instant},
};

use getrandom::fill;
use ocentra_family_identity_core::trust_bootstrap::{
    begin_parent_device_key_sealing, AuthorizedParentDeviceTrustCeremony, DeviceTrustRef,
    TrustBootstrapDecision, TrustBootstrapManualRequirement, TrustBootstrapRejection,
};
use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error as CustodyError, WindowsDeviceTrustCustody,
};
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentDeviceTrustBootstrapResult {
    /// Opaque correlation reference; this is never platform-sealed key material.
    pub device_trust_ref: DeviceTrustRef,
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

#[derive(Clone, PartialEq, Eq)]
pub struct ParentDeviceTrustStagedCeremonyRef(String);

const STAGED_CEREMONY_TTL: Duration = Duration::from_secs(300);

impl ParentDeviceTrustStagedCeremonyRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ParentDeviceTrustStagedCeremonyRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ParentDeviceTrustStagedCeremonyRef([redacted])")
    }
}

struct StagedParentDeviceTrustCeremony {
    trust_bootstrap_ref: String,
    ceremony: AuthorizedParentDeviceTrustCeremony,
}

struct TimedStagedCeremony<T> {
    staged_at: Instant,
    value: T,
}

/// Bounded one-shot opaque-handle cache in native runtime composition.
///
/// Callers own the payload type; this cache only owns handle expiry and removal.
pub struct ExpiringStagedCeremonies<T> {
    entries: HashMap<String, TimedStagedCeremony<T>>,
}

impl<T> Default for ExpiringStagedCeremonies<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<T> ExpiringStagedCeremonies<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, ceremony_ref: String, value: T, staged_at: Instant) {
        self.entries
            .insert(ceremony_ref, TimedStagedCeremony { staged_at, value });
    }

    pub fn reap_expired(&mut self, now: Instant) {
        self.entries.retain(|_ceremony_ref, staged| {
            !STAGED_CEREMONY_TTL
                .saturating_sub(now.saturating_duration_since(staged.staged_at))
                .is_zero()
        });
    }

    pub fn remove(&mut self, ceremony_ref: &str) -> Option<T> {
        self.entries.remove(ceremony_ref).map(|staged| staged.value)
    }
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
        ceremony: AuthorizedParentDeviceTrustCeremony,
        parent_device_trust_material: &[u8],
    ) -> Result<ParentDeviceTrustBootstrapResult, ParentDeviceTrustBootstrapError> {
        match begin_parent_device_key_sealing(trust_bootstrap_ref, ceremony) {
            TrustBootstrapDecision::AwaitingPlatformKeySealing(request) => {
                self.seal_request(*request, parent_device_trust_material)
            }
            TrustBootstrapDecision::Rejected(rejection) => Err(
                ParentDeviceTrustBootstrapError::ParentPresenceRejected(rejection),
            ),
            TrustBootstrapDecision::ManualRequired(requirement) => {
                Err(ParentDeviceTrustBootstrapError::ManualRequired(requirement))
            }
        }
    }

    fn seal_request(
        &self,
        request: ocentra_family_identity_core::trust_bootstrap::AwaitingPlatformKeySealingRequest,
        parent_device_trust_material: &[u8],
    ) -> Result<ParentDeviceTrustBootstrapResult, ParentDeviceTrustBootstrapError> {
        let device_trust_ref = request.device_trust_ref.clone();
        self.custody
            .seal_persist_activate(request, parent_device_trust_material)
            .map_err(ParentDeviceTrustBootstrapError::Custody)?;
        Ok(ParentDeviceTrustBootstrapResult { device_trust_ref })
    }

    pub fn current_parent_device_trust_is_available(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<(), ParentDeviceTrustBootstrapError> {
        self.custody
            .unseal_current(family_id, parent_account_id, device_ref)
            .map(|_sealed_material| ())
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
    staged_ceremonies: Mutex<ExpiringStagedCeremonies<StagedParentDeviceTrustCeremony>>,
}

impl ParentDeviceTrustCommandFacade {
    pub fn open(custody_root: impl AsRef<Path>) -> Result<Self, ParentDeviceTrustCommandError> {
        ParentDeviceTrustBootstrapRuntime::open(custody_root)
            .map(|runtime| Self {
                runtime,
                staged_ceremonies: Mutex::new(ExpiringStagedCeremonies::new()),
            })
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }

    pub fn stage_authorized_parent_device_trust_ceremony(
        &self,
        trust_bootstrap_ref: String,
        ceremony: AuthorizedParentDeviceTrustCeremony,
    ) -> Result<ParentDeviceTrustStagedCeremonyRef, ParentDeviceTrustCommandError> {
        if trust_bootstrap_ref.trim().is_empty() {
            return Err(ParentDeviceTrustCommandError::InvalidStagingRequest);
        }
        let staged = StagedParentDeviceTrustCeremony {
            trust_bootstrap_ref,
            ceremony,
        };
        let mut staged_ceremonies = self
            .staged_ceremonies
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let now = Instant::now();
        staged_ceremonies.reap_expired(now);
        loop {
            let ceremony_ref = random_ceremony_ref()?;
            if staged_ceremonies.entries.contains_key(&ceremony_ref) {
                continue;
            }
            staged_ceremonies.insert(ceremony_ref.clone(), staged, now);
            return Ok(ParentDeviceTrustStagedCeremonyRef(ceremony_ref));
        }
    }

    pub fn seal_staged_parent_device_trust(
        &self,
        ceremony_ref: &str,
    ) -> Result<ParentDeviceTrustBootstrapResult, ParentDeviceTrustCommandError> {
        let mut staged_ceremonies = self
            .staged_ceremonies
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        staged_ceremonies.reap_expired(Instant::now());
        let staged = staged_ceremonies
            .remove(ceremony_ref)
            .ok_or(ParentDeviceTrustCommandError::UnknownOrConsumedCeremony)?;
        let mut material = Zeroizing::new([0_u8; 32]);
        fill(&mut *material).map_err(|_error| ParentDeviceTrustCommandError::HandleGeneration)?;
        self.runtime
            .seal_verified_parent_device_trust(
                staged.trust_bootstrap_ref,
                staged.ceremony,
                &material[..],
            )
            .map_err(ParentDeviceTrustCommandError::Runtime)
    }

    pub fn current_parent_device_trust_is_available(
        &self,
        family_id: &str,
        parent_account_id: &str,
        device_ref: &str,
    ) -> Result<(), ParentDeviceTrustCommandError> {
        self.runtime
            .current_parent_device_trust_is_available(family_id, parent_account_id, device_ref)
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
