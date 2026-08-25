use ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureRequest;
use ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE;

#[path = "frozen/checks.rs"]
mod checks;
#[path = "frozen/context.rs"]
mod context;

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
        .map_err(|error| checks::release_or(error, authority, session, frozen))?;
    if let Some(error) = preflight_error(&preflight, live_target) {
        return Err(checks::release_or(error, authority, session, frozen));
    }
    let png_bytes = transport::capture_screenshot(session, request)
        .map_err(transport::capture_error)
        .map_err(|error| checks::release_or(error, authority, session, frozen))?;
    let postflight = structured::extract_on_session(session)
        .map_err(structured::capture_error)
        .map_err(|error| checks::release_or(error, authority, session, frozen))?;
    let checks = checks_for(authority, live_target, &preflight, &postflight, &png_bytes);
    if let Err(error) = checks::release(authority, frozen, session) {
        return Err(error);
    }
    if let Some(error) = checks.error() {
        checks::invalidate(authority);
        return Err(error);
    }
    let capture_context =
        context::after_release(authority, live_target, session, preflight, &postflight)?;
    Ok(ManagedBrowserCdpCaptureBytes {
        png_bytes,
        evidence_refs: authority.evidence_refs.clone(),
        capture_context,
    })
}

fn preflight_error(
    preflight: &structured::EvaluatedPayload,
    live_target: &target::LiveTarget,
) -> Option<ManagedBrowserCdpCaptureError> {
    if !preflight.payload.capture_safe
        || preflight.payload.sensitivity_digest != MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
    {
        Some(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
    } else if target::document_identity_matches_snapshot(
        &live_target.snapshot,
        &preflight.document_identity,
    ) {
        None
    } else {
        Some(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)
    }
}

fn checks_for(
    authority: &ManagedBrowserCdpTargetAuthority,
    live_target: &target::LiveTarget,
    preflight: &structured::EvaluatedPayload,
    postflight: &structured::EvaluatedPayload,
    png_bytes: &[u8],
) -> checks::CaptureChecks {
    let post_target_is_same = target::poll_and_verify(
        &authority.launch_authority,
        &authority.target_id,
        Some(&live_target.snapshot),
    )
    .is_ok();
    let post_authority_is_valid = super::super::binding::validate(&authority.launch_authority)
        .is_ok()
        && super::super::process::revalidate(&authority.launch_authority).is_ok();
    checks::CaptureChecks {
        image_is_valid: checks::image_is_valid(png_bytes),
        identity_is_same: preflight.document_identity == postflight.document_identity,
        target_is_same: post_target_is_same,
        authority_is_valid: post_authority_is_valid,
        signal_is_same: preflight.payload.signal_digest == postflight.payload.signal_digest,
        body_is_same: preflight.payload.body_digest == postflight.payload.body_digest,
        sensitivity_is_same: preflight.payload.capture_safe
            && postflight.payload.capture_safe
            && preflight.payload.sensitivity_digest
                == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
            && postflight.payload.sensitivity_digest
                == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
            && preflight.payload.sensitivity_digest == postflight.payload.sensitivity_digest,
        postflight_is_safe: postflight.payload.capture_safe
            && postflight.payload.sensitivity_digest
                == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE,
    }
}
