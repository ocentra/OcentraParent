//! Client-side retained anchor for the one fixed broker pipe.
//!
//! This module deliberately owns no wire authority. It retains the process,
//! token, image, SCM, service-SID, and enrollment observations required to
//! validate the OS-reported pipe server before a client emits bootstrap bytes.

use std::time::{Duration, Instant};

use ocentra_protected_capability_custody_protocol::handshake::UntrustedBrokerHello;
use ocentra_protected_capability_custody_windows_ffi::{
    OwnedProcess, OwnedToken, ProcessObservation, TokenObservation,
};

use crate::platform::PlatformError;

use super::enrollment::VerifiedEnrollment;
use super::peer::image_digest;
use super::scm::VerifiedBrokerService;
use super::{map_ffi_error, service_sid, token_groups};

const WINDOWS_TO_UNIX_EPOCH_100NS: u64 = 116_444_736_000_000_000;
const HUNDRED_NS_PER_SECOND: u64 = 10_000_000;
const SYSTEM_SID_BYTES: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
const SYSTEM_INTEGRITY_RID: u32 = 0x0000_4000;

pub(in crate::broker_admission) struct ClientAnchor {
    enrollment: VerifiedEnrollment,
    service: VerifiedBrokerService,
    broker_process: OwnedProcess,
    broker_token: OwnedToken,
    broker_initial_process: ProcessObservation,
    broker_initial_token: TokenObservation,
    broker_process_id: u32,
    broker_session_id: u32,
    service_sid: Vec<u8>,
    client_process: OwnedProcess,
    client_token: OwnedToken,
    client_initial_process: ProcessObservation,
    client_initial_token: TokenObservation,
    observed_at: Instant,
}

impl ClientAnchor {
    pub(in crate::broker_admission) fn open(
        registry_id: &str,
        broker_process_id: u32,
        broker_session_id: u32,
    ) -> Result<Self, PlatformError> {
        if broker_process_id == 0 {
            return Err(PlatformError::InvalidAttestation);
        }
        let enrollment = VerifiedEnrollment::open(registry_id)?;
        let service = VerifiedBrokerService::open(&enrollment)?;
        let broker_process =
            OwnedProcess::open_for_peer_observation(broker_process_id).map_err(map_ffi_error)?;
        let broker_initial_process = broker_process.observation().map_err(map_ffi_error)?;
        let broker_token = broker_process.open_token().map_err(map_ffi_error)?;
        let broker_initial_token = broker_token.observation().map_err(map_ffi_error)?;
        let service_sid = service_sid::observe(service.service())?;

        let client_process =
            OwnedProcess::open_for_peer_observation(std::process::id()).map_err(map_ffi_error)?;
        let client_initial_process = client_process.observation().map_err(map_ffi_error)?;
        let client_token = client_process.open_token().map_err(map_ffi_error)?;
        let client_initial_token = client_token.observation().map_err(map_ffi_error)?;
        let anchor = Self {
            enrollment,
            service,
            broker_process,
            broker_token,
            broker_initial_process,
            broker_initial_token,
            broker_process_id,
            broker_session_id,
            service_sid,
            client_process,
            client_token,
            client_initial_process,
            client_initial_token,
            observed_at: Instant::now(),
        };
        anchor.revalidate()?;
        Ok(anchor)
    }

    pub(in crate::broker_admission) fn revalidate(&self) -> Result<(), PlatformError> {
        if self.observed_at.elapsed()
            > Duration::from_millis(
                ocentra_protected_capability_custody_protocol::constants::
                    BROKER_ACCEPT_DEADLINE_MILLIS,
            )
        {
            return Err(PlatformError::InvalidAttestation);
        }
        self.enrollment.revalidate()?;
        self.service.revalidate(&self.enrollment)?;

        let broker_process = self.broker_process.observation().map_err(map_ffi_error)?;
        let broker_token = self.broker_token.observation().map_err(map_ffi_error)?;
        if broker_process != self.broker_initial_process
            || broker_token != self.broker_initial_token
            || !broker_process.is_alive()
            || broker_process.process_id() != self.broker_process_id
            || broker_token.sid() != SYSTEM_SID_BYTES
            || broker_token.integrity_level() != SYSTEM_INTEGRITY_RID
            || broker_token.session_id() != self.broker_session_id
            || image_digest(broker_process.image()) != *self.enrollment.broker_image_digest()
        {
            return Err(PlatformError::WrongBinding);
        }
        let current_service_sid = service_sid::observe(self.service.service())?;
        if current_service_sid != self.service_sid
            || token_groups::require_member(&self.broker_token, &self.service_sid).is_err()
        {
            return Err(PlatformError::WrongBinding);
        }

        let client_process = self.client_process.observation().map_err(map_ffi_error)?;
        let client_token = self.client_token.observation().map_err(map_ffi_error)?;
        if client_process != self.client_initial_process
            || client_token != self.client_initial_token
            || !client_process.is_alive()
            || client_process.process_id() != std::process::id()
            || client_token.sid() != self.enrollment.client_sid()
            || client_token.integrity_level() != self.enrollment.client_integrity()
            || client_token.session_id() != self.enrollment.client_session()
            || image_digest(client_process.image()) != *self.enrollment.client_image_digest()
        {
            return Err(PlatformError::WrongBinding);
        }
        Ok(())
    }

    pub(in crate::broker_admission) fn client_identity(
        &self,
    ) -> Result<(u32, u64, u32), PlatformError> {
        self.revalidate()?;
        Ok((
            self.client_initial_process.process_id(),
            process_epoch_seconds(self.client_initial_process.creation_time_100ns())?,
            self.client_initial_token.session_id(),
        ))
    }

    pub(in crate::broker_admission) fn authorize_broker_hello(
        &self,
        hello: &UntrustedBrokerHello,
        broker_process_id: u32,
        broker_session_id: u32,
    ) -> Result<(), PlatformError> {
        self.revalidate()?;
        let broker_epoch =
            process_epoch_seconds(self.broker_initial_process.creation_time_100ns())?;
        if broker_process_id != self.broker_process_id
            || broker_session_id != self.broker_session_id
            || hello.broker_process_id() != broker_process_id
            || hello.broker_session_id() != broker_session_id
            || hello.broker_epoch() != broker_epoch
        {
            return Err(PlatformError::WrongBinding);
        }
        Ok(())
    }
}

fn process_epoch_seconds(creation_time_100ns: u64) -> Result<u64, PlatformError> {
    creation_time_100ns
        .checked_sub(WINDOWS_TO_UNIX_EPOCH_100NS)
        .map(|value| value / HUNDRED_NS_PER_SECOND)
        .filter(|value| *value != 0)
        .ok_or(PlatformError::InvalidAttestation)
}
