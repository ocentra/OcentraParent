use std::sync::atomic::Ordering;

use ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureRequest;

use super::super::{
    structured, target, transport, ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError,
    ManagedBrowserCdpTargetAuthority,
};

pub(super) fn run(
    authority: &ManagedBrowserCdpTargetAuthority,
    request: &ManagedBrowserCdpCaptureRequest,
    live_target: &target::LiveTarget,
    session: &mut transport::CdpSession,
    frozen: &mut transport::FrozenPageGuard,
) -> Result<ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError> {
    let preflight = structured::extract_on_session(session)
        .map_err(structured::capture_error)
        .map_err(|error| release_or(error, authority, session, frozen))?;
    if !preflight.payload.capture_safe
        || !target::document_identity_matches_snapshot(
            &live_target.snapshot,
            &preflight.document_identity,
        )
    {
        let error = if preflight.payload.capture_safe {
            ManagedBrowserCdpCaptureError::TargetAuthorityMismatch
        } else {
            ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected
        };
        return Err(release_or(error, authority, session, frozen));
    }
    let png_bytes = transport::capture_screenshot(session, request)
        .map_err(transport::capture_error)
        .map_err(|error| release_or(error, authority, session, frozen))?;
    let postflight = structured::extract_on_session(session)
        .map_err(structured::capture_error)
        .map_err(|error| release_or(error, authority, session, frozen))?;
    let post_target_is_same = target::poll_and_verify(
        &authority.launch_authority,
        &authority.target_id,
        Some(&live_target.snapshot),
    )
    .is_ok();
    let post_authority_is_valid = super::super::binding::validate(&authority.launch_authority)
        .is_ok()
        && super::super::process::revalidate(&authority.launch_authority).is_ok();
    let checks = CaptureChecks {
        image_is_valid: image_is_valid(&png_bytes),
        identity_is_same: preflight.document_identity == postflight.document_identity,
        target_is_same: post_target_is_same,
        authority_is_valid: post_authority_is_valid,
        signal_is_same: preflight.payload.signal_digest == postflight.payload.signal_digest,
        sensitivity_is_same: preflight.payload.capture_safe
            && postflight.payload.capture_safe
            && preflight.payload.sensitivity_digest == postflight.payload.sensitivity_digest,
        postflight_is_safe: postflight.payload.capture_safe,
    };
    if let Err(error) = release(authority, frozen, session) {
        return Err(error);
    }
    checks.error().map_or_else(
        || {
            Ok(ManagedBrowserCdpCaptureBytes {
                png_bytes,
                evidence_refs: authority.evidence_refs.clone(),
            })
        },
        Err,
    )
}

struct CaptureChecks {
    image_is_valid: bool,
    identity_is_same: bool,
    target_is_same: bool,
    authority_is_valid: bool,
    signal_is_same: bool,
    sensitivity_is_same: bool,
    postflight_is_safe: bool,
}

impl CaptureChecks {
    fn error(&self) -> Option<ManagedBrowserCdpCaptureError> {
        if !self.postflight_is_safe {
            Some(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
        } else if !self.identity_is_same || !self.target_is_same || !self.authority_is_valid {
            Some(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)
        } else if !self.signal_is_same || !self.sensitivity_is_same {
            Some(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
        } else if !self.image_is_valid {
            Some(ManagedBrowserCdpCaptureError::InvalidImage)
        } else {
            None
        }
    }
}

fn release(
    authority: &ManagedBrowserCdpTargetAuthority,
    frozen: &mut transport::FrozenPageGuard,
    session: &mut transport::CdpSession,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    match frozen.release(session) {
        Ok(()) => Ok(()),
        Err(_error) => {
            let _session_retired = super::super::process::retire(&authority.launch_authority);
            authority
                .capture_safety_revoked
                .store(true, Ordering::Release);
            Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable)
        }
    }
}

fn release_or(
    error: ManagedBrowserCdpCaptureError,
    authority: &ManagedBrowserCdpTargetAuthority,
    session: &mut transport::CdpSession,
    frozen: &mut transport::FrozenPageGuard,
) -> ManagedBrowserCdpCaptureError {
    release(authority, frozen, session).err().unwrap_or(error)
}

fn image_is_valid(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes.len() <= 32 * 1024 * 1024
        && bytes.get(0..8) == Some(b"\x89PNG\r\n\x1a\n".as_slice())
}
