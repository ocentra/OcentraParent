//! Protocol adapters for the Rust-owned `ocentra-ai-contracts` leaf.
//!
//! This module is deliberately a seam, not a second contract owner. Work
//! requests are decoded into the leaf through its public constructors, while
//! context, result, and journal values are accepted only after an owner has
//! already materialized them. The agent protocol does not mint redaction,
//! runtime, authorization, lifecycle, or policy authority.

use ocentra_ai_contracts::ai_contracts::{
    context::AiEvidenceContext,
    identity::{AiSchemaIdentity, AiTimestamp, AiWorkItemId},
    journal::AiJournalEntry,
    result::AiResult,
    work::{AiRetryPolicy, AiWorkKind, AiWorkRequest},
    AiAuthorityBoundary, AiDurabilityState, AI_CONTRACT_SCHEMA_VERSION,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// The schema version is borrowed from the neutral leaf instead of being
/// copied into the protocol crate.
pub const AI_PROTOCOL_CONTRACT_SCHEMA_VERSION: &str = AI_CONTRACT_SCHEMA_VERSION;

/// Fail-closed errors returned by the protocol seam.
#[derive(Debug)]
pub enum AiProtocolContractError {
    InvalidEncoding(serde_json::Error),
    InvalidWorkRequest(AiProtocolWorkRequestError),
    StaleSchemaVersion,
    OwnerResolvedAttachment,
    UnsafeAuthorityBoundary,
    UnsafeJournalDurability,
    DigestNotPreserved,
    SerializationFailed(serde_json::Error),
}

/// Exact source retained when a wire request cannot become a leaf contract.
#[derive(Debug)]
pub enum AiProtocolWorkRequestError {
    Encoding(serde_json::Error),
    Contract(&'static str),
    MissingSchemaVersion,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AiWorkRequestWire {
    identity: AiSchemaIdentity,
    work_item_id: AiWorkItemId,
    work_kind: AiWorkKind,
    requested_at: AiTimestamp,
    deadline_at: Option<AiTimestamp>,
    retry_policy: AiRetryPolicy,
    prompt: Option<Value>,
    runtime: Option<Value>,
}

/// Decode a protocol work request into the canonical leaf contract.
///
/// The leaf validates identifiers, current schema, timestamps, deadlines, and
/// retry policy. Prompt and runtime attachments are rejected here because the
/// protocol has no owner-issued redaction or runtime capability to attach.
pub fn decode_work_request(bytes: &[u8]) -> Result<AiWorkRequest, AiProtocolContractError> {
    let value: Value =
        serde_json::from_slice(bytes).map_err(AiProtocolContractError::InvalidEncoding)?;
    ensure_current_schema(&value)?;
    let wire: AiWorkRequestWire =
        serde_json::from_value(value).map_err(invalid_work_request_encoding)?;
    if wire.prompt.is_some() || wire.runtime.is_some() {
        return Err(AiProtocolContractError::OwnerResolvedAttachment);
    }
    AiWorkRequest::new(
        wire.identity,
        wire.work_item_id,
        wire.work_kind,
        wire.requested_at,
        wire.deadline_at,
        wire.retry_policy,
        None,
    )
    .map_err(invalid_work_request_contract)
}

/// Encode a canonical work request without changing its leaf-owned shape.
pub fn encode_work_request(request: &AiWorkRequest) -> Result<Vec<u8>, AiProtocolContractError> {
    ensure_current_schema_value(request.identity().schema_version())?;
    encode_json(request)
}

/// Encode an owner-issued evidence context for the protocol boundary.
pub fn encode_evidence_context(
    context: &AiEvidenceContext,
) -> Result<Vec<u8>, AiProtocolContractError> {
    ensure_current_schema_value(context.schema_version())?;
    if context.authority_boundary() != AiAuthorityBoundary::EvidenceOnly {
        return Err(AiProtocolContractError::UnsafeAuthorityBoundary);
    }
    encode_json(context)
}

/// Encode an owner-issued result while preserving its exact leaf digest.
pub fn encode_result(result: &AiResult) -> Result<Vec<u8>, AiProtocolContractError> {
    ensure_current_schema_value(result.schema_version())?;
    if result.authority_boundary() != AiAuthorityBoundary::EvidenceOnly {
        return Err(AiProtocolContractError::UnsafeAuthorityBoundary);
    }
    encode_json_with_digest(result, result.digest().as_str())
}

/// Encode a durable owner-issued journal entry while preserving its exact
/// leaf digest. Journal entries do not carry a separate schema-version field;
/// their canonical digest and durable state are the validation seam here.
pub fn encode_journal_entry(entry: &AiJournalEntry) -> Result<Vec<u8>, AiProtocolContractError> {
    if entry.durability() != AiDurabilityState::Durable {
        return Err(AiProtocolContractError::UnsafeJournalDurability);
    }
    encode_json_with_digest(entry, entry.digest().as_str())
}

fn ensure_current_schema(value: &Value) -> Result<(), AiProtocolContractError> {
    let schema_version = value
        .get("identity")
        .and_then(|identity| identity.get("schemaVersion"))
        .and_then(Value::as_str)
        .ok_or(AiProtocolContractError::InvalidWorkRequest(
            AiProtocolWorkRequestError::MissingSchemaVersion,
        ))?;
    if schema_version != AI_PROTOCOL_CONTRACT_SCHEMA_VERSION {
        return Err(AiProtocolContractError::StaleSchemaVersion);
    }
    Ok(())
}

fn ensure_current_schema_value(
    value: &ocentra_ai_contracts::ai_contracts::identity::AiSchemaVersion,
) -> Result<(), AiProtocolContractError> {
    value
        .is_current()
        .then_some(())
        .ok_or(AiProtocolContractError::StaleSchemaVersion)
}

fn encode_json<T: Serialize>(value: &T) -> Result<Vec<u8>, AiProtocolContractError> {
    serde_json::to_vec(value).map_err(AiProtocolContractError::SerializationFailed)
}

fn encode_json_with_digest<T: Serialize>(
    value: &T,
    expected_digest: &str,
) -> Result<Vec<u8>, AiProtocolContractError> {
    let bytes = encode_json(value)?;
    let encoded: Value =
        serde_json::from_slice(&bytes).map_err(AiProtocolContractError::SerializationFailed)?;
    let actual_digest = encoded
        .get("digest")
        .and_then(Value::as_str)
        .ok_or(AiProtocolContractError::DigestNotPreserved)?;
    (actual_digest == expected_digest)
        .then_some(bytes)
        .ok_or(AiProtocolContractError::DigestNotPreserved)
}

fn invalid_work_request_encoding(error: serde_json::Error) -> AiProtocolContractError {
    AiProtocolContractError::InvalidWorkRequest(AiProtocolWorkRequestError::Encoding(error))
}

fn invalid_work_request_contract(error: &'static str) -> AiProtocolContractError {
    AiProtocolContractError::InvalidWorkRequest(AiProtocolWorkRequestError::Contract(error))
}
