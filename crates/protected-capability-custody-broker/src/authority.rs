use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_protected_capability_custody_core::broker_admission::BrokerPlatformSessionState;
use ocentra_protected_capability_custody_protocol::constants::SESSION_TTL_MILLIS;
use ocentra_protected_capability_custody_protocol::handshake::BrokerSessionWireValues;
use ocentra_protected_capability_custody_protocol::types::{Nonce, SessionHandle};
use sysinfo::{Pid, System};

use crate::BrokerError;

#[derive(Clone, Copy)]
pub(crate) struct ProcessIdentity {
    pub(crate) process_id: u32,
    pub(crate) process_epoch: u64,
    pub(crate) session_id: u32,
}

pub(crate) struct BrokerSessionAuthority {
    values: BrokerSessionWireValues,
}

impl BrokerSessionAuthority {
    pub(crate) fn generate(
        identity: ProcessIdentity,
        now_unix_millis: u64,
        platform: Option<BrokerPlatformSessionState>,
    ) -> Result<Self, BrokerError> {
        let expires_at = now_unix_millis
            .checked_add(SESSION_TTL_MILLIS)
            .ok_or(BrokerError::PeerAuthentication)?;
        let state = platform.ok_or(BrokerError::DeploymentRequired)?;
        let broker_key_epoch = state.key_epoch();
        let writer_lease_epoch = state.writer_lease_epoch();
        let watermark = state.watermark();
        Ok(Self {
            values: BrokerSessionWireValues {
                broker_nonce: Nonce::generate()?,
                broker_process_id: identity.process_id,
                broker_session_id: identity.session_id,
                broker_epoch: identity.process_epoch,
                broker_key_epoch,
                writer_lease_epoch,
                watermark,
                session_handle: SessionHandle::generate()?,
                session_expires_at_unix_millis: expires_at,
            },
        })
    }

    pub(crate) fn wire_values(&self) -> BrokerSessionWireValues {
        self.values
    }
}

pub(crate) fn current_process_identity() -> Result<ProcessIdentity, BrokerError> {
    let process_id = std::process::id();
    process_identity(process_id)
}

pub(crate) fn process_identity(process_id: u32) -> Result<ProcessIdentity, BrokerError> {
    if process_id == 0 {
        return Err(BrokerError::PeerAuthentication);
    }
    let system = System::new_all();
    let process = system
        .process(Pid::from_u32(process_id))
        .ok_or(BrokerError::PeerAuthentication)?;
    let process_epoch = process.start_time();
    let session_id = process
        .session_id()
        .map(Pid::as_u32)
        .ok_or(BrokerError::PeerAuthentication)?;
    if process_epoch == 0 || session_id == 0 {
        return Err(BrokerError::PeerAuthentication);
    }
    Ok(ProcessIdentity {
        process_id,
        process_epoch,
        session_id,
    })
}

pub(crate) fn unix_now_millis() -> Result<u64, BrokerError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(map_clock_error)?;
    u64::try_from(duration.as_millis()).map_err(map_clock_overflow)
}

fn map_clock_error(_error: std::time::SystemTimeError) -> BrokerError {
    BrokerError::PeerAuthentication
}

fn map_clock_overflow(_error: std::num::TryFromIntError) -> BrokerError {
    BrokerError::PeerAuthentication
}
