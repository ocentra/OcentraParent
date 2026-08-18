use ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureRequest;

use super::{
    binding, process, target, transport, ManagedBrowserCdpCaptureBytes,
    ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};

#[path = "capture/frozen.rs"]
mod frozen;

pub(super) fn capture(
    authority: &ManagedBrowserCdpTargetAuthority,
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError> {
    request
        .validate()
        .map_err(|_error| ManagedBrowserCdpCaptureError::RequestRejected)?;
    if request.target_id != authority.target_id {
        return Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch);
    }
    process::revalidate(&authority.launch_authority)?;
    binding::validate(&authority.launch_authority)?;
    let live_target = target::poll_and_verify(
        &authority.launch_authority,
        &authority.target_id,
        Some(&authority.verified_snapshot),
    )?;
    let mut session =
        transport::CdpSession::connect(authority.endpoint, &live_target.snapshot.websocket_url)
            .map_err(|_error| ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable)?;
    let mut frozen = match session.freeze() {
        Ok(frozen) => frozen,
        Err(error) => {
            // A transport/response failure after the freeze command may leave
            // the page frozen without a returned guard. Retire that owned
            // browser before invalidating screenshot authority. An explicit
            // protocol rejection is an unsupported capability, not proof that
            // the page was frozen, so keep it typed-unavailable and retryable.
            if !matches!(error, transport::CdpTransportError::Protocol) {
                process::retire(&authority.launch_authority);
                authority
                    .capture_safety_revoked
                    .store(true, std::sync::atomic::Ordering::Release);
            }
            return Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable);
        }
    };
    frozen::run(authority, request, &live_target, &mut session, &mut frozen)
}
