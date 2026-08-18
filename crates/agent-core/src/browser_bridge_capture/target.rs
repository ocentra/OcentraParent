use ocentra_parent_agent_protocol::constants;
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpEvidenceRefs, MANAGED_BROWSER_CDP_TARGET_REF_PREFIX,
    MANAGED_BROWSER_CDP_TITLE_REF_PREFIX, MANAGED_BROWSER_CDP_URL_REF_PREFIX,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::{
    authority::LaunchBinding,
    identity::{validate_websocket_endpoint, verify_browser_identity},
    ManagedBrowserCdpCaptureError,
};
use crate::browser_bridge_http::read_devtools_body;

#[derive(Clone)]
pub(super) struct TargetSnapshot {
    pub(super) websocket_url: String,
    pub(super) url_digest: String,
    pub(super) title_digest: String,
    pub(super) browser_identity_digest: String,
}

pub(super) struct LiveTarget {
    pub(super) snapshot: TargetSnapshot,
}

pub(super) fn poll_and_verify(
    binding: &LaunchBinding,
    target_id: &str,
    expected: Option<&TargetSnapshot>,
) -> Result<LiveTarget, ManagedBrowserCdpCaptureError> {
    let browser_identity_digest = verify_browser_identity(binding)?;
    let list_body = read_devtools_body(&binding.endpoint, constants::browser::HTTP_GET_JSON_LIST)?;
    let target = target_from_list(&list_body, target_id, &browser_identity_digest, binding)?;
    if let Some(expected) = expected {
        let same_target = expected.websocket_url == target.snapshot.websocket_url
            && expected.url_digest == target.snapshot.url_digest
            && expected.title_digest == target.snapshot.title_digest
            && expected.browser_identity_digest == target.snapshot.browser_identity_digest;
        if !same_target {
            return Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch);
        }
    }
    Ok(target)
}

fn target_from_list(
    body: &str,
    target_id: &str,
    browser_identity_digest: &str,
    binding: &LaunchBinding,
) -> Result<LiveTarget, ManagedBrowserCdpCaptureError> {
    let value: Value = serde_json::from_str(body)
        .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidResponse)?;
    let targets = value
        .as_array()
        .ok_or(ManagedBrowserCdpCaptureError::InvalidResponse)?;
    let target = targets
        .iter()
        .find(|target| {
            target
                .get(constants::browser::DEVTOOLS_FIELD_ID)
                .and_then(Value::as_str)
                == Some(target_id)
        })
        .ok_or(ManagedBrowserCdpCaptureError::TargetNotFound)?;
    let target_type = target
        .get(constants::browser::DEVTOOLS_FIELD_TYPE)
        .and_then(Value::as_str);
    if target_type != Some(constants::browser::DEVTOOLS_TARGET_TYPE_PAGE) {
        return Err(ManagedBrowserCdpCaptureError::TargetNotPage);
    }
    let url = target
        .get(constants::browser::DEVTOOLS_FIELD_URL)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(ManagedBrowserCdpCaptureError::TargetNotObservable)?;
    if !observable_url(url) {
        return Err(ManagedBrowserCdpCaptureError::TargetNotObservable);
    }
    let title = target
        .get(constants::browser::DEVTOOLS_FIELD_TITLE)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(ManagedBrowserCdpCaptureError::TargetNotObservable)?;
    let websocket_url = target
        .get(constants::browser::DEVTOOLS_FIELD_WEB_SOCKET_DEBUGGER_URL)
        .and_then(Value::as_str)
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    validate_websocket_endpoint(websocket_url, binding.endpoint)?;
    Ok(LiveTarget {
        snapshot: TargetSnapshot {
            websocket_url: websocket_url.to_owned(),
            url_digest: text_digest(url),
            title_digest: text_digest(title),
            browser_identity_digest: browser_identity_digest.to_owned(),
        },
    })
}

fn observable_url(url: &str) -> bool {
    url != constants::browser::CHROMIUM_DEFAULT_URL
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_CHROME_PREFIX)
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_DEVTOOLS_PREFIX)
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_EDGE_PREFIX)
}

pub(super) fn opaque_evidence_refs(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
) -> ManagedBrowserCdpEvidenceRefs {
    ManagedBrowserCdpEvidenceRefs {
        target_ref: opaque_ref(
            MANAGED_BROWSER_CDP_TARGET_REF_PREFIX,
            binding,
            target_id,
            snapshot,
        ),
        url_ref: opaque_ref(
            MANAGED_BROWSER_CDP_URL_REF_PREFIX,
            binding,
            target_id,
            snapshot,
        ),
        title_ref: opaque_ref(
            MANAGED_BROWSER_CDP_TITLE_REF_PREFIX,
            binding,
            target_id,
            snapshot,
        ),
    }
}

fn opaque_ref(
    prefix: &str,
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
) -> String {
    let mut digest = Sha256::new();
    digest.update(binding.session_secret);
    digest.update(binding.managed_browser_session_id.as_bytes());
    digest.update([0]);
    digest.update(binding.profile_id.as_bytes());
    digest.update([0]);
    digest.update(binding.generation.to_be_bytes());
    digest.update(binding.process_id.to_be_bytes());
    digest.update(target_id.as_bytes());
    digest.update([0]);
    digest.update(snapshot.url_digest.as_bytes());
    digest.update([0]);
    digest.update(snapshot.title_digest.as_bytes());
    let mut reference = String::from(prefix);
    reference.push('-');
    for byte in digest.finalize() {
        reference.push_str(&format!("{byte:02x}"));
    }
    reference
}

fn text_digest(value: &str) -> String {
    let mut digest = String::new();
    for byte in Sha256::digest(value.as_bytes()) {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
