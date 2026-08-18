//! Browser-owned authority and bounded CDP structured extraction.
//!
//! A capture authority is minted only from the private evidence carried by a
//! real managed-browser launch. The authority re-authenticates the owned
//! process and bridge, then re-polls the exact target before any owner action.
//! URLs, titles, debugger endpoints, and image bytes stay inside this boundary
//! and are never represented by public Debug output. Screenshot capture stays
//! inside the owner-controlled frozen-page guard and is discarded on any
//! identity, sensitivity, authority, or guard-restoration mismatch.

use std::{
    fmt,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
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
use sha2::{Digest, Sha256};

#[path = "browser_bridge_capture/authority.rs"]
mod authority;
#[path = "browser_bridge_capture/binding.rs"]
mod binding;
#[path = "browser_bridge_capture/capture.rs"]
mod capture;
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
#[path = "browser_bridge_capture/transport.rs"]
mod transport;

#[derive(Clone)]
pub struct ManagedBrowserCdpTargetAuthority {
    pub(super) endpoint: std::net::SocketAddr,
    target_id: String,
    verified_snapshot: target::TargetSnapshot,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
    launch_authority: authority::LaunchBinding,
    last_observed_epoch_ms: Arc<AtomicU64>,
    pub(super) capability_revoked: Arc<AtomicBool>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ManagedBrowserCdpCaptureBytes {
    png_bytes: Vec<u8>,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
    capture_context: ManagedBrowserCdpCaptureContext,
}

#[derive(Clone, PartialEq, Eq)]
struct ManagedBrowserCdpCaptureContext {
    captured_at: String,
    structured_extraction_id: String,
    structured_evidence_digest: String,
    structured_signal_digest: String,
    structured_body_digest: String,
    structured_sensitivity_digest: String,
    document_frame_id: String,
    document_loader_id: String,
    document_url_digest: String,
    authority_digest: String,
    context_digest: String,
}

impl ManagedBrowserCdpCaptureContext {
    pub(super) fn from_extraction(
        extraction: &ManagedBrowserCdpStructuredExtraction,
        target_ref: &str,
    ) -> Result<Self, ManagedBrowserCdpCaptureError> {
        let document_frame_id = extraction
            .document_frame_id()
            .filter(|value| !value.trim().is_empty())
            .ok_or(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)?
            .to_owned();
        let document_loader_id = extraction
            .document_loader_id()
            .filter(|value| !value.trim().is_empty())
            .ok_or(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)?
            .to_owned();
        let document_url_digest = extraction
            .document_url_digest()
            .filter(|value| value.len() == 64)
            .ok_or(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch)?
            .to_owned();
        let captured_at = extraction.captured_at().to_owned();
        let mut digest = Sha256::new();
        for value in [
            extraction.extraction_id(),
            extraction.evidence_digest(),
            extraction.structured_signal_digest(),
            extraction.structured_body_digest(),
            extraction.structured_sensitivity_digest(),
            target_ref,
            &document_frame_id,
            &document_loader_id,
            &document_url_digest,
            extraction.authority_digest(),
            &captured_at,
        ] {
            digest.update(value.as_bytes());
            digest.update([0]);
        }
        let context_digest = hex_digest(digest.finalize());
        if !extraction.is_fresh() {
            return Err(ManagedBrowserCdpCaptureError::AuthorityExpired);
        }
        Ok(Self {
            captured_at,
            structured_extraction_id: extraction.extraction_id().to_owned(),
            structured_evidence_digest: extraction.evidence_digest().to_owned(),
            structured_signal_digest: extraction.structured_signal_digest().to_owned(),
            structured_body_digest: extraction.structured_body_digest().to_owned(),
            structured_sensitivity_digest: extraction.structured_sensitivity_digest().to_owned(),
            document_frame_id,
            document_loader_id,
            document_url_digest,
            authority_digest: extraction.authority_digest().to_owned(),
            context_digest,
        })
    }
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
    ProtectedSurfaceRejected,
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
        capability_revoked: Arc::new(AtomicBool::new(false)),
    })
}

impl ManagedBrowserCdpTargetAuthority {
    pub fn extract_structured(
        &self,
    ) -> Result<ManagedBrowserCdpStructuredExtraction, ManagedBrowserCdpCaptureError> {
        if self.capability_revoked.load(Ordering::Acquire) {
            return Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable);
        }
        process::revalidate(&self.launch_authority)?;
        let live_target = target::poll_and_verify(
            &self.launch_authority,
            &self.target_id,
            Some(&self.verified_snapshot),
        )?;
        let evaluation = structured::extract(self.endpoint, &live_target.snapshot.websocket_url);
        let post_target = target::poll_and_verify(
            &self.launch_authority,
            &self.target_id,
            Some(&live_target.snapshot),
        );
        let authority_is_fresh = binding::validate(&self.launch_authority).is_ok()
            && process::revalidate(&self.launch_authority).is_ok()
            && !self.capability_revoked.load(Ordering::Acquire);
        // Sample authoritative wall time only after evaluation and all
        // document/target/authority revalidation have returned.
        let captured_at_epoch_ms = binding::unix_epoch_millis()?;
        let captured_at_monotonic = self.launch_authority.authority_started_at.elapsed();
        let monotonic_lower_bound = self
            .launch_authority
            .authority_started_epoch_ms
            .saturating_add(u64::try_from(captured_at_monotonic.as_millis()).unwrap_or(u64::MAX));
        let previous_epoch_ms = self.last_observed_epoch_ms.load(Ordering::Acquire);
        let timestamp_is_trusted = captured_at_epoch_ms >= previous_epoch_ms
            && captured_at_epoch_ms >= self.launch_authority.created_at_epoch_ms
            && captured_at_epoch_ms <= self.launch_authority.expires_at_epoch_ms
            && captured_at_epoch_ms >= monotonic_lower_bound;
        self.last_observed_epoch_ms
            .fetch_max(captured_at_epoch_ms, Ordering::AcqRel);
        let (payload, document_identity) = match evaluation {
            Ok(evaluated)
                if timestamp_is_trusted
                    && authority_is_fresh
                    && post_target.is_ok()
                    && target::document_identity_matches_snapshot(
                        &live_target.snapshot,
                        &evaluated.document_identity,
                    ) =>
            {
                (evaluated.payload, Some(evaluated.document_identity))
            }
            _ => (structured::Payload::unavailable(), None),
        };
        Ok(structured::bind_extraction(
            &self.launch_authority,
            &self.target_id,
            &live_target.snapshot,
            self.capability_revoked.clone(),
            captured_at_epoch_ms,
            captured_at_monotonic,
            document_identity.as_ref(),
            payload,
        ))
    }

    pub fn capture(
        &self,
        request: &ManagedBrowserCdpCaptureRequest,
    ) -> Result<ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError> {
        if self.capability_revoked.load(Ordering::Acquire) {
            return Err(ManagedBrowserCdpCaptureError::ScreenshotSafetyGuardUnavailable);
        }
        capture::capture(self, request)
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
    capture: &ManagedBrowserCdpCaptureBytes,
    capture_mode: ManagedBrowserCdpCaptureMode,
    width: u32,
    height: u32,
) -> ManagedBrowserCdpCaptureReceipt {
    let image_digest = hex_digest(Sha256::digest(&capture.png_bytes));
    let capture_ref = capture_ref(&capture.capture_context.context_digest, &image_digest);
    ManagedBrowserCdpCaptureReceipt {
        schema_version: MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION.to_owned(),
        capture_ref,
        target_ref: capture.evidence_refs.target_ref.clone(),
        evidence_refs: capture.evidence_refs.clone(),
        captured_at: capture.capture_context.captured_at.clone(),
        structured_extraction_id: capture.capture_context.structured_extraction_id.clone(),
        structured_evidence_digest: capture.capture_context.structured_evidence_digest.clone(),
        structured_signal_digest: capture.capture_context.structured_signal_digest.clone(),
        structured_body_digest: capture.capture_context.structured_body_digest.clone(),
        structured_sensitivity_digest: capture
            .capture_context
            .structured_sensitivity_digest
            .clone(),
        document_frame_id: capture.capture_context.document_frame_id.clone(),
        document_loader_id: capture.capture_context.document_loader_id.clone(),
        document_url_digest: capture.capture_context.document_url_digest.clone(),
        authority_digest: capture.capture_context.authority_digest.clone(),
        capture_context_digest: capture.capture_context.context_digest.clone(),
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

fn capture_ref(context_digest: &str, image_digest: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(context_digest.as_bytes());
    digest.update([0]);
    digest.update(image_digest.as_bytes());
    let mut value = String::from(
        ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_CAPTURE_REF_PREFIX,
    );
    value.push_str(&hex_digest(digest.finalize()));
    value
}

fn hex_digest(bytes: impl IntoIterator<Item = u8>) -> String {
    let mut value = String::new();
    for byte in bytes {
        value.push_str(&format!("{byte:02x}"));
    }
    value
}
