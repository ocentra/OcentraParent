use std::sync::atomic::Ordering;

use ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE;

use super::super::super::{
    binding, process, structured, target, transport, ManagedBrowserCdpCaptureContext,
    ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};

pub(super) fn after_release(
    authority: &ManagedBrowserCdpTargetAuthority,
    live_target: &target::LiveTarget,
    session: &mut transport::CdpSession,
    preflight: structured::EvaluatedPayload,
    postflight: &structured::EvaluatedPayload,
) -> Result<ManagedBrowserCdpCaptureContext, ManagedBrowserCdpCaptureError> {
    verify_post_release_structured(authority, session, &preflight, postflight)?;
    verify_post_release_authority(authority, live_target)?;
    let (captured_at_epoch_ms, captured_at_monotonic) = trusted_capture_time(authority)?;
    let structured_extraction = bind_capture_extraction(
        authority,
        live_target,
        preflight,
        captured_at_epoch_ms,
        captured_at_monotonic,
    )?;
    ManagedBrowserCdpCaptureContext::from_extraction(
        &structured_extraction,
        &authority.evidence_refs.target_ref,
    )
    .inspect_err(|_error| {
        super::checks::invalidate(authority);
    })
}

fn verify_post_release_structured(
    authority: &ManagedBrowserCdpTargetAuthority,
    session: &mut transport::CdpSession,
    preflight: &structured::EvaluatedPayload,
    postflight: &structured::EvaluatedPayload,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    let post_release = match structured::extract_on_session(session) {
        Ok(value) => value,
        Err(error) => {
            super::checks::invalidate(authority);
            return Err(structured::capture_error(error));
        }
    };
    if preflight.document_identity != post_release.document_identity
        || !structured_payload_matches(&post_release, postflight)
    {
        super::checks::invalidate(authority);
        return Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch);
    }
    Ok(())
}

fn structured_payload_matches(
    actual: &structured::EvaluatedPayload,
    expected: &structured::EvaluatedPayload,
) -> bool {
    actual.payload.capture_safe
        && actual.payload.sensitivity_digest == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
        && expected.payload.sensitivity_digest == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
        && actual.payload.signal_digest == expected.payload.signal_digest
        && actual.payload.body_digest == expected.payload.body_digest
        && actual.payload.sensitivity_digest == expected.payload.sensitivity_digest
}

fn verify_post_release_authority(
    authority: &ManagedBrowserCdpTargetAuthority,
    live_target: &target::LiveTarget,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    let target_is_same = target::poll_and_verify(
        &authority.launch_authority,
        &authority.target_id,
        Some(&live_target.snapshot),
    )
    .is_ok();
    let authority_is_valid = binding::validate(&authority.launch_authority).is_ok()
        && process::revalidate(&authority.launch_authority).is_ok()
        && !authority.capability_revoked.load(Ordering::Acquire);
    if target_is_same && authority_is_valid {
        Ok(())
    } else {
        super::checks::invalidate(authority);
        Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)
    }
}

fn trusted_capture_time(
    authority: &ManagedBrowserCdpTargetAuthority,
) -> Result<(u64, std::time::Duration), ManagedBrowserCdpCaptureError> {
    let captured_at_epoch_ms = match binding::unix_epoch_millis() {
        Ok(value) => value,
        Err(error) => {
            super::checks::invalidate(authority);
            return Err(error);
        }
    };
    let captured_at_monotonic = authority.launch_authority.authority_started_at.elapsed();
    let monotonic_lower_bound = authority
        .launch_authority
        .authority_started_epoch_ms
        .saturating_add(u64::try_from(captured_at_monotonic.as_millis()).unwrap_or(u64::MAX));
    let wall_time_is_valid = captured_at_epoch_ms >= authority.launch_authority.created_at_epoch_ms
        && captured_at_epoch_ms <= authority.launch_authority.expires_at_epoch_ms
        && captured_at_epoch_ms >= authority.last_observed_epoch_ms.load(Ordering::Acquire)
        && captured_at_epoch_ms >= monotonic_lower_bound;
    let monotonic_time_is_valid = captured_at_monotonic.as_millis()
        <= u128::from(
            authority
                .launch_authority
                .expires_at_epoch_ms
                .saturating_sub(authority.launch_authority.created_at_epoch_ms),
        );
    if wall_time_is_valid && monotonic_time_is_valid {
        authority
            .last_observed_epoch_ms
            .store(captured_at_epoch_ms, Ordering::Release);
        Ok((captured_at_epoch_ms, captured_at_monotonic))
    } else {
        super::checks::invalidate(authority);
        Err(ManagedBrowserCdpCaptureError::AuthorityExpired)
    }
}

fn bind_capture_extraction(
    authority: &ManagedBrowserCdpTargetAuthority,
    live_target: &target::LiveTarget,
    preflight: structured::EvaluatedPayload,
    captured_at_epoch_ms: u64,
    captured_at_monotonic: std::time::Duration,
) -> Result<structured::ManagedBrowserCdpStructuredExtraction, ManagedBrowserCdpCaptureError> {
    let signal_digest = preflight.payload.signal_digest.clone();
    let body_digest = preflight.payload.body_digest.clone();
    let sensitivity_digest = preflight.payload.sensitivity_digest.clone();
    let extraction = structured::bind_extraction(structured::BindingInput {
        binding: &authority.launch_authority,
        target_id: &authority.target_id,
        snapshot: &live_target.snapshot,
        capability_revoked: std::sync::Arc::clone(&authority.capability_revoked),
        captured_at_epoch_ms,
        captured_at_monotonic,
        document_identity: Some(&preflight.document_identity),
        payload: preflight.payload,
    });
    let matches_postflight = extraction.is_fresh()
        && !extraction.protected_content_skipped()
        && extraction.structured_signal_digest() == signal_digest
        && extraction.structured_body_digest() == body_digest
        && extraction.structured_sensitivity_digest() == sensitivity_digest;
    if matches_postflight {
        Ok(extraction)
    } else {
        super::checks::invalidate(authority);
        Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)
    }
}
