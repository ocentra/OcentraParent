use std::sync::atomic::Ordering;

use super::super::super::{
    transport, ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};

pub(super) struct CaptureChecks {
    pub(super) image_is_valid: bool,
    pub(super) identity_is_same: bool,
    pub(super) target_is_same: bool,
    pub(super) authority_is_valid: bool,
    pub(super) signal_is_same: bool,
    pub(super) body_is_same: bool,
    pub(super) sensitivity_is_same: bool,
    pub(super) postflight_is_safe: bool,
}

impl CaptureChecks {
    pub(super) fn error(&self) -> Option<ManagedBrowserCdpCaptureError> {
        if !self.postflight_is_safe {
            Some(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
        } else if !self.identity_is_same || !self.target_is_same || !self.authority_is_valid {
            Some(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)
        } else if !self.signal_is_same || !self.body_is_same || !self.sensitivity_is_same {
            Some(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
        } else if !self.image_is_valid {
            Some(ManagedBrowserCdpCaptureError::InvalidImage)
        } else {
            None
        }
    }
}

pub(super) fn release(
    authority: &ManagedBrowserCdpTargetAuthority,
    frozen: &mut transport::FrozenPageGuard,
    session: &mut transport::CdpSession,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    match frozen.release(session) {
        Ok(()) => Ok(()),
        Err(_error) => {
            let _session_retired =
                super::super::super::process::retire(&authority.launch_authority);
            authority.capability_revoked.store(true, Ordering::Release);
            Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable)
        }
    }
}

pub(super) fn release_or(
    error: ManagedBrowserCdpCaptureError,
    authority: &ManagedBrowserCdpTargetAuthority,
    session: &mut transport::CdpSession,
    frozen: &mut transport::FrozenPageGuard,
) -> ManagedBrowserCdpCaptureError {
    release(authority, frozen, session).err().unwrap_or(error)
}

pub(super) fn invalidate(authority: &ManagedBrowserCdpTargetAuthority) {
    authority.capability_revoked.store(true, Ordering::Release);
    let _retired = super::super::super::process::retire(&authority.launch_authority);
}

pub(super) fn image_is_valid(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes.len() <= 32 * 1024 * 1024
        && bytes.get(0..8) == Some(b"\x89PNG\r\n\x1a\n".as_slice())
}
