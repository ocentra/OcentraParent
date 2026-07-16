use ocentra_parent_agent_protocol::constants;

use super::{adapter::ScreenAiAnalysisAdapterOutput, config::ScreenOcrRedactionPolicy};

struct OcrRedactionResult {
    snippets: Vec<String>,
    notes: Vec<String>,
}

struct RedactionNotes(Vec<String>);
struct OcrSnippets(Vec<String>);

pub(super) fn apply_service_ocr_redaction(
    output: ScreenAiAnalysisAdapterOutput,
    policy: &ScreenOcrRedactionPolicy,
) -> ScreenAiAnalysisAdapterOutput {
    if ocr_text_disabled(policy) {
        return ScreenAiAnalysisAdapterOutput {
            ocr_text_snippets: Vec::new(),
            redaction_notes: disabled_notes(RedactionNotes(output.redaction_notes)).0,
            ..output
        };
    }
    let result = redacted_ocr_output(
        OcrSnippets(output.ocr_text_snippets),
        RedactionNotes(output.redaction_notes),
        policy,
    );
    ScreenAiAnalysisAdapterOutput {
        ocr_text_snippets: result.snippets,
        redaction_notes: result.notes,
        ..output
    }
}

fn disabled_notes(mut redaction_notes: RedactionNotes) -> RedactionNotes {
    if !redaction_notes
        .0
        .iter()
        .any(|existing| existing == constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_DISABLED)
    {
        redaction_notes
            .0
            .push(constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_DISABLED.to_string());
    }
    redaction_notes
}

fn redacted_ocr_output(
    ocr_text_snippets: OcrSnippets,
    mut redaction_notes: RedactionNotes,
    policy: &ScreenOcrRedactionPolicy,
) -> OcrRedactionResult {
    let push_unique_note = |redaction_notes: &mut RedactionNotes, note: &'static str| {
        if !redaction_notes.0.iter().any(|existing| existing == note) {
            redaction_notes.0.push(note.to_string());
        }
    };
    let redact_pii_tokens = |snippet: &str| {
        let looks_like_email = |token: &str| {
            token.contains(constants::local_ai_runtime::SCREEN_OCR_EMAIL_AT)
                && token.contains(constants::local_ai_runtime::SCREEN_OCR_EMAIL_PERIOD)
        };
        let looks_like_phone_number = |token: &str| {
            token
                .chars()
                .filter(|candidate| candidate.is_ascii_digit())
                .count()
                >= constants::local_ai_runtime::SCREEN_SERVICE_OCR_PHONE_MIN_DIGITS
        };
        if !policy.pii_redaction_enabled
            || policy.text_retention_mode
                != constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_REDACTED_SNIPPETS
        {
            return (snippet.to_string(), false);
        }
        let mut redacted = Vec::new();
        let mut pii_redacted = false;
        for token in snippet.split_whitespace() {
            if looks_like_email(token) {
                redacted.push(constants::local_ai_runtime::SCREEN_OCR_REDACTION_TOKEN_EMAIL);
                pii_redacted = true;
                continue;
            }
            if looks_like_phone_number(token) {
                redacted.push(constants::local_ai_runtime::SCREEN_OCR_REDACTION_TOKEN_PHONE);
                pii_redacted = true;
                continue;
            }
            redacted.push(token);
        }
        (
            redacted.join(constants::local_ai_runtime::SCREEN_OCR_TOKEN_SEPARATOR),
            pii_redacted,
        )
    };
    let mut snippets = Vec::new();
    for snippet in ocr_text_snippets.0 {
        let normalized = snippet.to_lowercase();
        let credential_like = normalized
            .contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_PASSWORD)
            || normalized.contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_TOKEN)
            || normalized
                .contains(constants::local_ai_runtime::SCREEN_OCR_CREDENTIAL_MARKER_SECRET);
        if policy.credential_suppression_enabled && credential_like {
            push_unique_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_CREDENTIAL,
            );
            continue;
        }
        let (redacted, pii_redacted) = redact_pii_tokens(&snippet);
        if pii_redacted {
            push_unique_note(
                &mut redaction_notes,
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_PII,
            );
        }
        if snippets.len() < policy.snippet_limit {
            snippets.push(redacted);
        }
    }
    OcrRedactionResult {
        snippets,
        notes: redaction_notes.0,
    }
}

fn ocr_text_disabled(policy: &ScreenOcrRedactionPolicy) -> bool {
    !policy.ocr_text_enabled
        || policy.text_retention_mode
            == constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_DISABLED
}
