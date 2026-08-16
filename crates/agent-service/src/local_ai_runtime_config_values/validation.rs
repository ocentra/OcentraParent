use ocentra_parent_agent_protocol::constants;

use super::{LocalAiRuntimeRefPrefix, LocalAiRuntimeText};

pub(crate) fn safe_ref_or_default(
    candidate: Option<LocalAiRuntimeText>,
    prefix: LocalAiRuntimeRefPrefix,
    fallback: LocalAiRuntimeText,
) -> LocalAiRuntimeText {
    candidate
        .filter(|value| is_safe_local_model_ref(value, prefix))
        .unwrap_or(fallback)
}

fn is_safe_local_model_ref(
    candidate: &LocalAiRuntimeText,
    prefix: LocalAiRuntimeRefPrefix,
) -> bool {
    let Some(body) = candidate.0.strip_prefix(prefix.0) else {
        return false;
    };

    let body = LocalAiRuntimeText(body.to_string());
    starts_with_safe_ref_char(&body)
        && body.0.len() >= 3
        && body.0.len() <= 128
        && is_safe_ref_body(&body)
}

fn starts_with_safe_ref_char(body: &LocalAiRuntimeText) -> bool {
    body.0
        .chars()
        .next()
        .map(|first| first.is_ascii_lowercase() || first.is_ascii_digit())
        .unwrap_or(false)
}

fn is_safe_ref_body(body: &LocalAiRuntimeText) -> bool {
    body.0.chars().all(|value| {
        value.is_ascii_lowercase() || value.is_ascii_digit() || value == '_' || value == '-'
    })
}

pub(crate) fn is_safe_local_ai_model_id(candidate: &LocalAiRuntimeText) -> bool {
    candidate
        .0
        .chars()
        .next()
        .map(|first| first.is_ascii_alphanumeric())
        .unwrap_or(false)
        && candidate.0.len() <= 128
        && candidate.0.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::COLON
                || value == constants::delimiter::DOT
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::SLASH
                || value == constants::delimiter::UNDERSCORE
        })
}

pub(crate) fn is_safe_llama_selector(candidate: &LocalAiRuntimeText) -> bool {
    !candidate.0.is_empty()
        && candidate.0.len() <= 64
        && candidate.0.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::LIST
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::UNDERSCORE
        })
}

pub(crate) fn is_safe_llama_release_tag(candidate: &LocalAiRuntimeText) -> bool {
    !candidate.0.is_empty()
        && candidate.0.len() <= 32
        && candidate.0.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::DOT
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::UNDERSCORE
        })
}
