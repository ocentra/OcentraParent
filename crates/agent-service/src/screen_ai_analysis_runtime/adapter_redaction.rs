use ocentra_parent_agent_protocol::constants;

use super::adapter::ScreenAiAnalysisAdapterOutput;

pub(super) fn apply_service_ocr_redaction(
    output: ScreenAiAnalysisAdapterOutput,
) -> ScreenAiAnalysisAdapterOutput {
    let mut redaction_notes = output.redaction_notes;
    let mut snippets = Vec::new();
    for snippet in output.ocr_text_snippets {
        if credential_like(&snippet) {
            push_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_CREDENTIAL,
            );
            continue;
        }
        let (redacted, pii_redacted) = redact_pii_tokens(&snippet);
        if pii_redacted {
            push_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_PII,
            );
        }
        if snippets.len() < constants::local_ai_runtime::SCREEN_SERVICE_OCR_SNIPPET_LIMIT {
            snippets.push(redacted);
        }
    }
    ScreenAiAnalysisAdapterOutput {
        ocr_text_snippets: snippets,
        redaction_notes,
        ..output
    }
}

fn credential_like(snippet: &str) -> bool {
    let normalized = snippet.to_lowercase();
    normalized.contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_PASSWORD)
        || normalized.contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_TOKEN)
        || normalized.contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_SECRET)
}

fn redact_pii_tokens(snippet: &str) -> (String, bool) {
    let mut redacted = Vec::new();
    let mut pii_redacted = false;
    for token in snippet.split_whitespace() {
        if email_like(token) {
            redacted.push(constants::local_ai_runtime::SCREEN_OCR_REDACTION_TOKEN_EMAIL);
            pii_redacted = true;
        } else if phone_like(token) {
            redacted.push(constants::local_ai_runtime::SCREEN_OCR_REDACTION_TOKEN_PHONE);
            pii_redacted = true;
        } else {
            redacted.push(token);
        }
    }
    (
        redacted.join(constants::local_ai_runtime::SCREEN_OCR_TOKEN_SEPARATOR),
        pii_redacted,
    )
}

fn email_like(token: &str) -> bool {
    token.contains(constants::local_ai_runtime::SCREEN_OCR_EMAIL_AT)
        && token.contains(constants::local_ai_runtime::SCREEN_OCR_EMAIL_PERIOD)
}

fn phone_like(token: &str) -> bool {
    let digit_count = token
        .chars()
        .filter(|candidate| candidate.is_ascii_digit())
        .count();
    digit_count >= constants::local_ai_runtime::SCREEN_SERVICE_OCR_PHONE_MIN_DIGITS
}

fn push_note(redaction_notes: &mut Vec<String>, note: &str) {
    if !redaction_notes.iter().any(|existing| existing == note) {
        redaction_notes.push(note.to_string());
    }
}
