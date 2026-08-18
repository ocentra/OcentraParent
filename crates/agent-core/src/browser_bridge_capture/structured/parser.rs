use ocentra_parent_agent_protocol::constants;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::{ExtractionError, Outcome, Payload};

const MAX_STRUCTURED_TEXT: usize = 480;
const MAX_SIGNAL_TEXT: usize = 480;
const CDP_FIELD_EXCEPTION_DETAILS: &str = "exceptionDetails";
const CDP_FIELD_VALUE: &str = "value";
const CDP_FIELD_RESULT: &str = "result";

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
        || visible_text_character_count > constants::browser::DEVTOOLS_MAX_RESPONSE_BYTES
    {
        return Err(ExtractionError::InvalidResponse);
    }
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
    let signal_digest = signal_digest(
        &visible_text,
        &meta_values,
        &accessibility_values,
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        protected_content_skipped,
    );
    let has_structured_signals = !meta_values.is_empty() || !accessibility_values.is_empty();
    let outcome = if protected_content_skipped {
        Outcome::ProtectedContentSkipped
    } else if dom_overflow_redacted || (visible_text.is_empty() && !has_structured_signals) {
        Outcome::NeedsScreenshot
    } else {
        Outcome::PolicySufficient
    };
    Ok(Payload {
        visible_text_summary: (!visible_text.is_empty()).then_some(visible_text),
        visible_text_character_count,
        dom_overflow_redacted,
        private_content_redacted,
        signal_digest,
        outcome,
    })
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

fn signal_digest(
    visible_text: &str,
    meta_values: &str,
    accessibility_values: &str,
    visible_text_character_count: usize,
    dom_overflow_redacted: bool,
    private_content_redacted: bool,
    protected_content_skipped: bool,
) -> String {
    digest(&[
        visible_text,
        meta_values,
        accessibility_values,
        &visible_text_character_count.to_string(),
        if dom_overflow_redacted {
            "overflow"
        } else {
            "bounded"
        },
        if private_content_redacted {
            "private-redacted"
        } else {
            "private-clear"
        },
        if protected_content_skipped {
            "protected-skipped"
        } else {
            "protected-absent"
        },
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
