use std::time::{Duration, Instant};

use ocentra_protected_capability_custody_windows_ffi::{
    ImageObservation, OwnedProcess, OwnedToken, ProcessObservation, TokenObservation,
};

use crate::platform::PlatformError;

use super::enrollment::VerifiedEnrollment;
use super::enrollment_security::append_security;
use super::{map_ffi_error, ObservationDigest};

const IMAGE_OBSERVATION_DOMAIN: &[u8] = b"ocentra.pcc.image-observation.v1";
const WINDOWS_TO_UNIX_EPOCH_100NS: u64 = 116_444_736_000_000_000;
const HUNDRED_NS_PER_SECOND: u64 = 10_000_000;
const SYSTEM_SID_BYTES: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
const SYSTEM_INTEGRITY_RID: u32 = 0x0000_4000;

pub(super) struct VerifiedBrokerProcess {
    process: OwnedProcess,
    token: OwnedToken,
    initial_process: ProcessObservation,
    initial_token: TokenObservation,
}

pub(super) struct PeerObservation {
    process: OwnedProcess,
    process_token: OwnedToken,
    impersonated_token: OwnedToken,
    initial_process: ProcessObservation,
    initial_process_token: TokenObservation,
    initial_impersonated_token: TokenObservation,
    observed_at: Instant,
}

pub(super) struct AuthorizedPeer {
    _peer: PeerObservation,
}

impl VerifiedBrokerProcess {
    pub(super) fn open(enrollment: &VerifiedEnrollment) -> Result<Self, PlatformError> {
        let process =
            OwnedProcess::open_for_peer_observation(std::process::id()).map_err(map_ffi_error)?;
        let initial_process = process.observation().map_err(map_ffi_error)?;
        let token = process.open_token().map_err(map_ffi_error)?;
        let initial_token = token.observation().map_err(map_ffi_error)?;
        let owner = Self {
            process,
            token,
            initial_process,
            initial_token,
        };
        owner.revalidate(enrollment)?;
        Ok(owner)
    }

    pub(super) fn revalidate(&self, enrollment: &VerifiedEnrollment) -> Result<(), PlatformError> {
        let process = self.process.observation().map_err(map_ffi_error)?;
        let token = self.token.observation().map_err(map_ffi_error)?;
        if process != self.initial_process
            || token != self.initial_token
            || !process.is_alive()
            || process.process_id() != std::process::id()
            || token.sid() != SYSTEM_SID_BYTES
            || token.integrity_level() != SYSTEM_INTEGRITY_RID
            || token.session_id() != 0
            || image_digest(process.image()) != *enrollment.broker_image_digest()
        {
            return Err(PlatformError::Tampered);
        }
        Ok(())
    }
}

impl PeerObservation {
    pub(super) fn observe(
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<Self, PlatformError> {
        if pipe_process_id == 0 || pipe_session_id == 0 {
            return Err(PlatformError::InvalidAttestation);
        }
        let process =
            OwnedProcess::open_for_peer_observation(pipe_process_id).map_err(map_ffi_error)?;
        let initial_process = process.observation().map_err(map_ffi_error)?;
        let process_token = process.open_token().map_err(map_ffi_error)?;
        let initial_process_token = process_token.observation().map_err(map_ffi_error)?;
        let impersonated_token = OwnedToken::open_current_thread().map_err(map_ffi_error)?;
        let initial_impersonated_token = impersonated_token.observation().map_err(map_ffi_error)?;
        if !initial_process.is_alive()
            || initial_process.process_id() != pipe_process_id
            || initial_process_token != initial_impersonated_token
            || initial_impersonated_token.session_id() != pipe_session_id
        {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self {
            process,
            process_token,
            impersonated_token,
            initial_process,
            initial_process_token,
            initial_impersonated_token,
            observed_at: Instant::now(),
        })
    }

    pub(super) fn revalidate(&self, enrollment: &VerifiedEnrollment) -> Result<(), PlatformError> {
        enrollment.revalidate()?;
        let process = self.process.observation().map_err(map_ffi_error)?;
        let process_token = self.process_token.observation().map_err(map_ffi_error)?;
        let impersonated_token = self
            .impersonated_token
            .observation()
            .map_err(map_ffi_error)?;
        if process != self.initial_process
            || process_token != self.initial_process_token
            || impersonated_token != self.initial_impersonated_token
            || process_token != impersonated_token
            || !process.is_alive()
            || impersonated_token.sid() != enrollment.client_sid()
            || impersonated_token.integrity_level() != enrollment.client_integrity()
            || impersonated_token.session_id() != enrollment.client_session()
            || image_digest(process.image()) != *enrollment.client_image_digest()
        {
            return Err(PlatformError::Tampered);
        }
        Ok(())
    }

    pub(super) fn authorize_transcript(
        self,
        enrollment: &VerifiedEnrollment,
        bootstrap: &ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket,
        hello: &ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<AuthorizedPeer, PlatformError> {
        self.revalidate(enrollment)?;
        let maximum_age = Duration::from_millis(
            ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS,
        );
        let process_epoch = process_epoch_seconds(self.initial_process.creation_time_100ns())?;
        let identity = bootstrap.identity();
        if self.observed_at.elapsed() > maximum_age
            || pipe_process_id != self.initial_process.process_id()
            || pipe_session_id != self.initial_impersonated_token.session_id()
            || identity.client_process_id() != pipe_process_id
            || identity.client_process_epoch() != process_epoch
            || identity.client_session_id() != pipe_session_id
            || hello.client_process_id() != pipe_process_id
            || hello.client_process_epoch() != process_epoch
            || hello.client_session_id() != pipe_session_id
            || hello.nonce() != identity.pipe_nonce()
        {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(AuthorizedPeer { _peer: self })
    }
}

pub(super) fn image_digest(image: &ImageObservation) -> [u8; 32] {
    let mut digest = ObservationDigest::new(IMAGE_OBSERVATION_DOMAIN);
    digest.text(image.path().as_str());
    append_image_identity(&mut digest, image);
    digest.field(image.sha256());
    append_security(&mut digest, image.security());
    digest.u32(image.file_attributes());
    digest.u32(image.reparse_tag());
    digest.u32(image.ancestors().len() as u32);
    for ancestor in image.ancestors() {
        digest.text(ancestor.path().as_str());
        digest.u64(ancestor.identity().volume_serial_number());
        digest.field(ancestor.identity().file_id());
        append_security(&mut digest, ancestor.security());
        digest.u32(ancestor.file_attributes());
        digest.u32(ancestor.reparse_tag());
    }
    digest.finish()
}

fn append_image_identity(digest: &mut ObservationDigest, image: &ImageObservation) {
    digest.u64(image.identity().volume_serial_number());
    digest.field(image.identity().file_id());
}

fn process_epoch_seconds(creation_time_100ns: u64) -> Result<u64, PlatformError> {
    creation_time_100ns
        .checked_sub(WINDOWS_TO_UNIX_EPOCH_100NS)
        .map(|value| value / HUNDRED_NS_PER_SECOND)
        .filter(|value| *value != 0)
        .ok_or(PlatformError::InvalidAttestation)
}
