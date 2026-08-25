use ocentra_parent_agent_protocol::{
    constants, logging::LogFieldValue, transport::AgentCommandEnvelope,
};
use sha2::{Digest, Sha256};

pub(crate) struct RequestNonceDigest(pub(crate) String);

pub(crate) struct HealthEventIdSuffix(pub(crate) String);

pub(crate) fn request_nonce_digest(command: &AgentCommandEnvelope) -> RequestNonceDigest {
    RequestNonceDigest(
        command
            .payload
            .get(constants::field::REQUEST_NONCE)
            .and_then(|value| match value {
                LogFieldValue::String(value) if !value.is_empty() => Some(value),
                _ => None,
            })
            .map(|value| format!("{:x}", Sha256::digest(value.as_bytes())))
            .unwrap_or_else(|| constants::value::EMPTY.to_string()),
    )
}

pub(crate) fn health_event_id_suffix(command: &AgentCommandEnvelope) -> HealthEventIdSuffix {
    let mut event_id = String::from(constants::event_id::HEALTH_REPORTED);
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&request_nonce_digest(command).0);
    HealthEventIdSuffix(event_id)
}
