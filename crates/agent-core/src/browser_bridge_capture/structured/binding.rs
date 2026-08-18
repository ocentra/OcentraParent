use chrono::{DateTime, SecondsFormat, Utc};
use ocentra_schema::managed_browser_cdp_capture::{
    MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION, MANAGED_BROWSER_CDP_SOURCE_ID,
};
use sha2::{Digest, Sha256};

use super::{Freshness, Outcome, Payload};
use crate::browser_bridge_capture::{
    authority::LaunchBinding, target::TargetSnapshot, ManagedBrowserCdpStructuredExtraction,
};

const STRUCTURED_CUSTODY_STATE: &str = "live-local-child-agent";
const STRUCTURED_EXTRACTION_ID_PREFIX: &str = "browser-extraction-";

pub(super) fn bind_extraction(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    captured_at_epoch_ms: u64,
    payload: Payload,
) -> ManagedBrowserCdpStructuredExtraction {
    let Payload {
        visible_text_summary,
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        signal_digest,
        outcome,
    } = payload;
    let evidence_refs =
        crate::browser_bridge_capture::target::opaque_evidence_refs(binding, target_id, snapshot);
    let evidence_digest = extraction_digest(
        binding,
        target_id,
        snapshot,
        captured_at_epoch_ms,
        &signal_digest,
    );
    let extraction_id = format!("{STRUCTURED_EXTRACTION_ID_PREFIX}{evidence_digest}");
    let captured_at = trusted_timestamp(captured_at_epoch_ms);
    let freshness = if captured_at_epoch_ms <= binding.expires_at_epoch_ms {
        Freshness::Fresh
    } else {
        Freshness::Stale
    };
    ManagedBrowserCdpStructuredExtraction {
        source_id: MANAGED_BROWSER_CDP_SOURCE_ID,
        extraction_id,
        captured_at,
        managed_browser_session_ref: binding.managed_browser_session_id.clone(),
        target_ref: evidence_refs.target_ref.clone(),
        evidence_refs,
        evidence_digest,
        visible_text_summary,
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        freshness,
        outcome,
        custody_state: STRUCTURED_CUSTODY_STATE,
    }
}

fn extraction_digest(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    captured_at_epoch_ms: u64,
    signal_digest: &str,
) -> String {
    let generation = binding.generation.to_string();
    let process_id = binding.process_id.to_string();
    let captured_at = captured_at_epoch_ms.to_string();
    digest(&[
        MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION,
        binding.managed_browser_session_id.as_str(),
        binding.profile_id.as_str(),
        target_id,
        snapshot.url_digest.as_str(),
        snapshot.title_digest.as_str(),
        snapshot.browser_identity_digest.as_str(),
        &generation,
        &process_id,
        &captured_at,
        signal_digest,
    ])
}

fn digest(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    let mut digest = String::new();
    for byte in hasher.finalize() {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}

fn trusted_timestamp(epoch_ms: u64) -> String {
    i64::try_from(epoch_ms)
        .ok()
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|timestamp| timestamp.to_rfc3339_opts(SecondsFormat::Millis, true))
        .unwrap_or_else(|| String::from("1970-01-01T00:00:00.000Z"))
}
