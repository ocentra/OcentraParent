//! Browser-owned authority and bounded CDP structured extraction.
//!
//! A capture authority is minted only from the private evidence carried by a
//! real managed-browser launch. The authority re-authenticates the owned
//! process and bridge, then re-polls the exact target before any owner action.
//! URLs, titles, debugger endpoints, and image bytes stay inside this boundary
//! and are never represented by public Debug output. Screenshot capture is
//! fail-closed until the owner proves an atomic or frozen-page guard.

use std::{
    fmt,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use crate::{
    browser_bridge_poll::BrowserBridgePollError, browser_managed_session::BrowserManagedLaunch,
};
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureMode, ManagedBrowserCdpCaptureReceipt, ManagedBrowserCdpCaptureRequest,
    ManagedBrowserCdpEvidenceRefs, MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION,
};

#[path = "browser_bridge_capture/authority.rs"]
mod authority;
#[path = "browser_bridge_capture/binding.rs"]
mod binding;
#[path = "browser_bridge_capture/identity.rs"]
mod identity;
#[path = "browser_bridge_capture/identity_match.rs"]
mod identity_match;
#[path = "browser_bridge_capture/port_owner.rs"]
mod port_owner;
#[path = "browser_bridge_capture/process.rs"]
mod process;
#[path = "browser_bridge_capture/structured.rs"]
mod structured;
#[path = "browser_bridge_capture/target.rs"]
mod target;

#[derive(Clone)]
pub struct ManagedBrowserCdpTargetAuthority {
    pub(super) endpoint: std::net::SocketAddr,
    target_id: String,
    verified_snapshot: target::TargetSnapshot,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
    launch_authority: authority::LaunchBinding,
    last_observed_epoch_ms: Arc<AtomicU64>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ManagedBrowserCdpCaptureBytes {
    png_bytes: Vec<u8>,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagedBrowserCdpCaptureError {
    Bridge(BrowserBridgePollError),
    AuthorityExpired,
    TargetNotFound,
    TargetNotPage,
    TargetNotObservable,
    TargetAuthorityMismatch,
    InvalidWebSocketEndpoint,
    RequestRejected,
    Transport,
    ResponseTooLarge,
    InvalidResponse,
    InvalidImage,
    ScreenshotSafetyGuardUnavailable,
}

pub type ManagedBrowserCdpStructuredExtraction = structured::ManagedBrowserCdpStructuredExtraction;

impl From<BrowserBridgePollError> for ManagedBrowserCdpCaptureError {
    fn from(error: BrowserBridgePollError) -> Self {
        Self::Bridge(error)
    }
}

impl fmt::Debug for ManagedBrowserCdpTargetAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedBrowserCdpTargetAuthority")
            .field("target_id", &self.target_id)
            .field("evidence_refs", &self.evidence_refs)
            .field("generation", &self.launch_authority.generation)
            .field(
                "expires_at_epoch_ms",
                &self.launch_authority.expires_at_epoch_ms,
            )
            .field("verified_snapshot", &"opaque")
            .finish()
    }
}

impl fmt::Debug for ManagedBrowserCdpCaptureBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedBrowserCdpCaptureBytes")
            .field("png_byte_size", &self.png_bytes.len())
            .field("evidence_refs", &self.evidence_refs)
            .finish()
    }
}

pub(crate) fn verify_managed_browser_cdp_endpoint(
    endpoint: std::net::SocketAddr,
    process_id: u32,
    executable_path: &Path,
) -> Result<(), BrowserBridgePollError> {
    port_owner::verify_endpoint_owner(endpoint, process_id)?;
    process::verify_process_executable(process_id, executable_path)
}

pub fn authorize_managed_browser_cdp_target(
    launch: &BrowserManagedLaunch,
    target_id: &str,
) -> Result<ManagedBrowserCdpTargetAuthority, ManagedBrowserCdpCaptureError> {
    let launch_authority = authority::from_launch(launch)?;
    process::revalidate(&launch_authority)?;
    let live_target = target::poll_and_verify(&launch_authority, target_id, None)?;
    let evidence_refs =
        target::opaque_evidence_refs(&launch_authority, target_id, &live_target.snapshot);
    let created_at_epoch_ms = launch_authority.created_at_epoch_ms;
    Ok(ManagedBrowserCdpTargetAuthority {
        endpoint: launch_authority.endpoint,
        target_id: target_id.to_owned(),
        verified_snapshot: live_target.snapshot,
        evidence_refs,
        launch_authority,
        last_observed_epoch_ms: Arc::new(AtomicU64::new(created_at_epoch_ms)),
    })
}

impl ManagedBrowserCdpTargetAuthority {
    pub fn extract_structured(
        &self,
    ) -> Result<ManagedBrowserCdpStructuredExtraction, ManagedBrowserCdpCaptureError> {
        process::revalidate(&self.launch_authority)?;
        let live_target = target::poll_and_verify(
            &self.launch_authority,
            &self.target_id,
            Some(&self.verified_snapshot),
        )?;
        let captured_at_epoch_ms = binding::unix_epoch_millis()?;
        let captured_at_monotonic = self.launch_authority.authority_started_at.elapsed();
        let monotonic_lower_bound = self
            .launch_authority
            .authority_started_epoch_ms
            .saturating_add(u64::try_from(captured_at_monotonic.as_millis()).unwrap_or(u64::MAX));
        let previous_epoch_ms = self.last_observed_epoch_ms.load(Ordering::Acquire);
        let clock_rolled_back = captured_at_epoch_ms < previous_epoch_ms
            || captured_at_epoch_ms < monotonic_lower_bound;
        self.last_observed_epoch_ms
            .fetch_max(captured_at_epoch_ms, Ordering::AcqRel);
        let payload = if clock_rolled_back {
            structured::Payload::unavailable()
        } else {
            structured::extract(self.endpoint, &live_target.snapshot.websocket_url)
                .unwrap_or_else(|_error| structured::Payload::unavailable())
        };
        Ok(structured::bind_extraction(
            &self.launch_authority,
            &self.target_id,
            &live_target.snapshot,
            captured_at_epoch_ms,
            captured_at_monotonic,
            payload,
        ))
    }

    pub fn capture(
        &self,
        request: &ManagedBrowserCdpCaptureRequest,
    ) -> Result<ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError> {
        request
            .validate()
            .map_err(|_error| ManagedBrowserCdpCaptureError::RequestRejected)?;
        if request.target_id != self.target_id {
            return Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch);
        }
        // A separate probe followed by an image command is not an atomic or
        // frozen-page guarantee. Until the managed-browser owner supplies
        // that guard, screenshots remain unavailable by design.
        Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable)
    }
}

impl ManagedBrowserCdpCaptureBytes {
    pub fn png_bytes(&self) -> &[u8] {
        &self.png_bytes
    }

    pub fn evidence_refs(&self) -> &ManagedBrowserCdpEvidenceRefs {
        &self.evidence_refs
    }
}

pub fn capture_receipt(
    capture_ref: String,
    capture: &ManagedBrowserCdpCaptureBytes,
    capture_mode: ManagedBrowserCdpCaptureMode,
    width: u32,
    height: u32,
    image_digest: String,
) -> ManagedBrowserCdpCaptureReceipt {
    ManagedBrowserCdpCaptureReceipt {
        schema_version: MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION.to_owned(),
        capture_ref,
        target_ref: capture.evidence_refs.target_ref.clone(),
        evidence_refs: capture.evidence_refs.clone(),
        capture_mode,
        width,
        height,
        image_digest,
        image_byte_size: capture.png_bytes.len() as u64,
        image_format: ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_IMAGE_FORMAT
            .to_owned(),
        custody_state: ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE
            .to_owned(),
        raw_image_retained: false,
    }
}
