use ocentra_parent_agent_protocol::constants;

use super::{adapter::ScreenAiAnalysisAdapterOutput, config::ScreenOcrRedactionPolicy};

pub(super) fn apply_service_ocr_redaction(
    output: ScreenAiAnalysisAdapterOutput,
    policy: &ScreenOcrRedactionPolicy,
) -> ScreenAiAnalysisAdapterOutput {
    if !policy.ocr_text_enabled
        || policy.text_retention_mode
            == constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_DISABLED
    {
        return ScreenAiAnalysisAdapterOutput {
            ocr_text_snippets: Vec::new(),
            redaction_notes: redaction_notes_with(
                output.redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_DISABLED,
            ),
            ..output
        };
    }
    let mut redaction_notes = output.redaction_notes;
    let mut snippets = Vec::new();
    for snippet in output.ocr_text_snippets {
        if policy.credential_suppression_enabled && credential_like(&snippet) {
            push_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_CREDENTIAL,
            );
            continue;
        }
        let (redacted, pii_redacted) = redact_pii_tokens(&snippet, policy);
        if pii_redacted {
            push_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_PII,
            );
        }
        if snippets.len() < policy.snippet_limit {
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

fn redact_pii_tokens(snippet: &str, policy: &ScreenOcrRedactionPolicy) -> (String, bool) {
    if !policy.pii_redaction_enabled
        || policy.text_retention_mode
            != constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_REDACTED_SNIPPETS
    {
        return (snippet.to_string(), false);
    }
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

fn redaction_notes_with(mut redaction_notes: Vec<String>, note: &str) -> Vec<String> {
    push_note(&mut redaction_notes, note);
    redaction_notes
}
