use serde_json::Value;
use sha2::{Digest, Sha256};

use ocentra_schema::managed_browser_cdp_capture::{
    MANAGED_BROWSER_CDP_SENSITIVITY_PROTECTED, MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE,
    MANAGED_BROWSER_CDP_SENSITIVITY_UNKNOWN,
};

use super::{ExtractionError, Outcome, Payload};

const MAX_STRUCTURED_TEXT: usize = 480;
const MAX_SIGNAL_TEXT: usize = 480;
const MAX_DOCUMENT_URL: usize = 4096;
const CDP_FIELD_EXCEPTION_DETAILS: &str = "exceptionDetails";
const CDP_FIELD_VALUE: &str = "value";
const CDP_FIELD_RESULT: &str = "result";
const BODY_DIGEST_PREFIX: &str = "managed-browser-body-sha256-v1-";

pub(super) fn parse_payload(value: &Value) -> Result<Payload, ExtractionError> {
    let result = value
        .get(CDP_FIELD_RESULT)
        .and_then(Value::as_object)
        .ok_or(ExtractionError::InvalidResponse)?;
    if result.contains_key(CDP_FIELD_EXCEPTION_DETAILS) {
        return Err(ExtractionError::InvalidResponse);
    }
    let value = result
        .get(CDP_FIELD_RESULT)
        .and_then(|result| result.get(CDP_FIELD_VALUE))
        .ok_or(ExtractionError::InvalidResponse)?;
    let visible_text = bounded_string(value, "visibleText", MAX_STRUCTURED_TEXT)?;
    let visible_text_character_count = value
        .get("visibleTextCharacterCount")
        .and_then(Value::as_u64)
        .and_then(|count| usize::try_from(count).ok())
        .ok_or(ExtractionError::InvalidResponse)?;
    if visible_text_character_count < visible_text.chars().count()
        || visible_text_character_count > MAX_STRUCTURED_TEXT
    {
        return Err(ExtractionError::InvalidResponse);
    }
    let document_url = bounded_string(value, "documentUrl", MAX_DOCUMENT_URL)?;
    let document_url_digest = digest(&[&document_url]);
    let dom_overflow_redacted = value
        .get("domOverflowRedacted")
        .and_then(Value::as_bool)
        .ok_or(ExtractionError::InvalidResponse)?;
    let private_content_redacted = value
        .get("privateContentRedacted")
        .and_then(Value::as_bool)
        .ok_or(ExtractionError::InvalidResponse)?;
    let protected_content_skipped = value
        .get("protectedContentSkipped")
        .and_then(Value::as_bool)
        .ok_or(ExtractionError::InvalidResponse)?;
    let meta_values = bounded_string(value, "metaValues", MAX_SIGNAL_TEXT)?;
    let accessibility_values = bounded_string(value, "accessibilityValues", MAX_SIGNAL_TEXT)?;
    let body_digest = bounded_string(value, "bodyDigest", MAX_SIGNAL_TEXT)?;
    let (capture_safe, sensitivity_digest) = parse_sensitivity(value)?;
    if !visible_text.is_empty()
        || visible_text_character_count != 0
        || !meta_values.is_empty()
        || !accessibility_values.is_empty()
        || !private_content_redacted
    {
        return Err(ExtractionError::InvalidResponse);
    }
    let protected = protected_content_skipped
        || sensitivity_digest == MANAGED_BROWSER_CDP_SENSITIVITY_PROTECTED;
    if !protected && !body_digest_is_valid(&body_digest) {
        return Err(ExtractionError::InvalidResponse);
    }
    let outcome = if protected {
        Outcome::ProtectedContentSkipped
    } else {
        Outcome::ReviewRequired
    };
    Ok(Payload {
        visible_text_summary: None,
        visible_text_character_count: 0,
        dom_overflow_redacted,
        private_content_redacted: true,
        signal_digest: if protected {
            String::from("protected-content-redacted-v1")
        } else {
            digest(&["unknown-static-sensitivity-redacted-v1", &body_digest])
        },
        body_digest,
        sensitivity_digest,
        capture_safe: capture_safe && !protected,
        document_url_digest,
        outcome,
    })
}

fn parse_sensitivity(value: &Value) -> Result<(bool, String), ExtractionError> {
    let capture_safe = value
        .get("captureSafe")
        .and_then(Value::as_bool)
        .ok_or(ExtractionError::InvalidResponse)?;
    let sensitivity_digest = bounded_string(value, "sensitivityDigest", MAX_SIGNAL_TEXT)?;
    if !matches!(
        sensitivity_digest.as_str(),
        MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE
            | MANAGED_BROWSER_CDP_SENSITIVITY_PROTECTED
            | MANAGED_BROWSER_CDP_SENSITIVITY_UNKNOWN
    ) {
        return Err(ExtractionError::InvalidResponse);
    }
    Ok((
        capture_safe && sensitivity_digest == MANAGED_BROWSER_CDP_SENSITIVITY_STRUCTURAL_SAFE,
        sensitivity_digest,
    ))
}

fn bounded_string(value: &Value, field: &str, limit: usize) -> Result<String, ExtractionError> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or(ExtractionError::InvalidResponse)?;
    if value.chars().count() > limit {
        return Err(ExtractionError::InvalidResponse);
    }
    Ok(value.to_owned())
}

fn body_digest_is_valid(value: &str) -> bool {
    value
        .strip_prefix(BODY_DIGEST_PREFIX)
        .is_some_and(|suffix| {
            suffix.len() == 64 && suffix.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
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
