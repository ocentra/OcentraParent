use std::{sync::Arc, time::Duration};

use chrono::{DateTime, SecondsFormat, Utc};
use ocentra_schema::managed_browser_cdp_capture::{
    MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION, MANAGED_BROWSER_CDP_SOURCE_ID,
};
use sha2::{Digest, Sha256};

use super::{Freshness, Outcome, Payload};
use crate::browser_bridge_capture::{
    authority::LaunchBinding,
    target::{document_identity_digest, DocumentIdentity, TargetSnapshot},
    ManagedBrowserCdpStructuredExtraction,
};

// The producer does not claim child delivery or enforcement authority. A
// later service owner may replace this with a custody state backed by runtime
// proof; until then the handoff remains explicitly unavailable.
const STRUCTURED_CUSTODY_STATE: &str = "unavailable";
const STRUCTURED_EXTRACTION_ID_PREFIX: &str = "browser-extraction-";

pub(super) fn bind_extraction(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    capability_revoked: Arc<std::sync::atomic::AtomicBool>,
    captured_at_epoch_ms: u64,
    captured_at_monotonic: Duration,
    document_identity: Option<&DocumentIdentity>,
    payload: Payload,
) -> ManagedBrowserCdpStructuredExtraction {
    let Payload {
        visible_text_summary,
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        signal_digest,
        body_digest,
        sensitivity_digest,
        capture_safe: _,
        document_url_digest: _,
        outcome,
    } = payload;
    let redact_page_identity = matches!(
        &outcome,
        Outcome::ProtectedContentSkipped | Outcome::ReviewRequired | Outcome::Unavailable
    ) || private_content_redacted;
    let evidence_refs = if redact_page_identity {
        crate::browser_bridge_capture::target::opaque_redacted_evidence_refs(
            binding, target_id, snapshot,
        )
    } else {
        crate::browser_bridge_capture::target::opaque_evidence_refs(binding, target_id, snapshot)
    };
    let evidence_digest = extraction_digest(
        binding,
        target_id,
        snapshot,
        captured_at_epoch_ms,
        &signal_digest,
        &sensitivity_digest,
        document_identity,
        redact_page_identity,
    );
    let authority_digest =
        crate::browser_bridge_capture::target::authority_digest(binding, target_id, snapshot);
    let extraction_id = format!("{STRUCTURED_EXTRACTION_ID_PREFIX}{evidence_digest}");
    let captured_at = trusted_timestamp(captured_at_epoch_ms);
    let freshness = freshness_for(binding, captured_at_epoch_ms, captured_at_monotonic);
    ManagedBrowserCdpStructuredExtraction {
        source_id: MANAGED_BROWSER_CDP_SOURCE_ID,
        extraction_id,
        captured_at,
        managed_browser_session_ref: binding.managed_browser_session_id.clone(),
        target_ref: evidence_refs.target_ref.clone(),
        evidence_refs,
        evidence_digest,
        structured_signal_digest: signal_digest,
        structured_body_digest: body_digest,
        structured_sensitivity_digest: sensitivity_digest,
        document_frame_id: document_identity.map(|identity| identity.frame_id.clone()),
        document_loader_id: document_identity.map(|identity| identity.loader_id.clone()),
        document_url_digest: document_identity.map(|identity| identity.url_digest.clone()),
        authority_digest,
        capability_revoked,
        visible_text_summary,
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        freshness,
        outcome,
        custody_state: STRUCTURED_CUSTODY_STATE,
    }
}

fn freshness_for(
    binding: &LaunchBinding,
    captured_at_epoch_ms: u64,
    captured_at_monotonic: Duration,
) -> Freshness {
    let monotonic_lower_bound = binding
        .authority_started_epoch_ms
        .saturating_add(u64::try_from(captured_at_monotonic.as_millis()).unwrap_or(u64::MAX));
    let wall_time_is_valid = captured_at_epoch_ms >= binding.created_at_epoch_ms
        && captured_at_epoch_ms <= binding.expires_at_epoch_ms
        && captured_at_epoch_ms >= monotonic_lower_bound;
    let monotonic_time_is_valid = captured_at_monotonic.as_millis()
        <= u128::from(
            binding
                .expires_at_epoch_ms
                .saturating_sub(binding.created_at_epoch_ms),
        );
    if wall_time_is_valid && monotonic_time_is_valid {
        Freshness::Fresh
    } else {
        Freshness::Unavailable
    }
}

fn extraction_digest(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    captured_at_epoch_ms: u64,
    signal_digest: &str,
    sensitivity_digest: &str,
    document_identity: Option<&DocumentIdentity>,
    redact_page_identity: bool,
) -> String {
    let generation = binding.generation.to_string();
    let process_id = binding.process_id.to_string();
    let captured_at = captured_at_epoch_ms.to_string();
    let document_identity = document_identity
        .map(document_identity_digest)
        .unwrap_or_else(|| String::from("document-identity-unavailable-v1"));
    let page_identity = if redact_page_identity {
        "protected-content-redacted-v1"
    } else {
        snapshot.url_digest.as_str()
    };
    let title_identity = if redact_page_identity {
        "protected-content-redacted-v1"
    } else {
        snapshot.title_digest.as_str()
    };
    digest(&[
        MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION,
        binding.managed_browser_session_id.as_str(),
        binding.profile_id.as_str(),
        target_id,
        page_identity,
        title_identity,
        snapshot.browser_identity_digest.as_str(),
        &generation,
        &process_id,
        &captured_at,
        &document_identity,
        signal_digest,
        sensitivity_digest,
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
