use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

pub(crate) fn env_path(key: &str) -> Option<PathBuf> {
    env_value(key).map(PathBuf::from)
}

pub(crate) fn env_value(key: &str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.trim().is_empty())
}

pub(crate) fn env_flag(key: &str) -> bool {
    env_value(key)
        .map(|value| value.eq_ignore_ascii_case(constants::value::TRUE))
        .unwrap_or(false)
}

pub(crate) fn env_u64(key: &str, fallback: u64) -> u64 {
    env_value(key)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(fallback)
}

pub(crate) fn env_u32(key: &str, fallback: u32) -> u32 {
    env_value(key)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(fallback)
}

pub(crate) fn env_llama_device(key: &str) -> Option<String> {
    env_value(key).filter(|value| is_safe_llama_selector(value))
}

pub(crate) fn env_llama_gpu_layers(key: &str) -> Option<String> {
    env_value(key).filter(|value| {
        value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_ALL
            || value == constants::local_ai_runtime::LLAMA_GPU_LAYERS_AUTO
            || value.parse::<u32>().is_ok()
    })
}

pub(crate) fn env_llama_release_tag(key: &str) -> String {
    env_value(key)
        .filter(|value| is_safe_llama_release_tag(value))
        .unwrap_or_else(|| constants::local_ai_runtime::DEFAULT_LLAMA_CPP_RELEASE_TAG.to_string())
}

pub(crate) fn env_local_ai_model_id(key: &str) -> String {
    env_value(key)
        .filter(|value| is_safe_local_ai_model_id(value))
        .unwrap_or_else(|| constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string())
}

pub(crate) fn safe_ref_or_default(
    candidate: Option<String>,
    prefix: &str,
    fallback: &str,
) -> String {
    candidate
        .filter(|value| is_safe_local_model_ref(value, prefix))
        .unwrap_or_else(|| fallback.to_string())
}

fn is_safe_local_model_ref(candidate: &str, prefix: &str) -> bool {
    let Some(body) = candidate.strip_prefix(prefix) else {
        return false;
    };

    starts_with_safe_ref_char(body)
        && body.len() >= 3
        && body.len() <= 128
        && is_safe_ref_body(body)
}

fn starts_with_safe_ref_char(body: &str) -> bool {
    body.chars()
        .next()
        .map(|first| first.is_ascii_lowercase() || first.is_ascii_digit())
        .unwrap_or(false)
}

fn is_safe_ref_body(body: &str) -> bool {
    body.chars().all(|value| {
        value.is_ascii_lowercase() || value.is_ascii_digit() || value == '_' || value == '-'
    })
}

pub(crate) fn is_safe_local_ai_model_id(candidate: &str) -> bool {
    candidate
        .chars()
        .next()
        .map(|first| first.is_ascii_alphanumeric())
        .unwrap_or(false)
        && candidate.len() <= 128
        && candidate.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::COLON
                || value == constants::delimiter::DOT
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::SLASH
                || value == constants::delimiter::UNDERSCORE
        })
}

fn is_safe_llama_selector(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.len() <= 64
        && candidate.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::LIST
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::UNDERSCORE
        })
}

fn is_safe_llama_release_tag(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.len() <= 32
        && candidate.chars().all(|value| {
            value.is_ascii_alphanumeric()
                || value == constants::delimiter::DOT
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::UNDERSCORE
        })
}
