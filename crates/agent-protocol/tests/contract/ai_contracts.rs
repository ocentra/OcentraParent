use std::collections::BTreeSet;

use ocentra_ai_contracts::ai_contracts::{
    context::{AiContextBuildState, AiEvidenceKind, AiProvenanceKind, AiReferenceValidationState},
    journal::{AiJournalEntryKind, AiJournalPayloadKind},
    result::{AiOutputValidationState, AiResultKind},
    work::AiWorkKind,
    AiAuthorityBoundary, AiCustodyState, AiDegradedState, AiDurabilityState, AiRedactionState,
    AiRetentionState, AiValidationState, AI_CONTRACT_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::ai_contracts::{
    decode_work_request, encode_work_request, AiProtocolContractError,
    AI_PROTOCOL_CONTRACT_SCHEMA_VERSION,
};
use serde_json::{json, Value};

fn work_request_fixture() -> Value {
    json!({
        "identity": {
            "schemaVersion": AI_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            "family": "family-1",
            "requestId": "request-1",
            "subject": {
                "familyId": "family-1",
                "childProfileId": "child-1",
                "deviceId": "device-1"
            }
        },
        "workItemId": "work-1",
        "workKind": "classification",
        "requestedAt": "2026-08-28T09:00:00Z",
        "deadlineAt": "2026-08-28T09:05:00Z",
        "retryPolicy": {
            "maxAttempts": 2,
            "retryAfterMs": 1000
        }
    })
}

fn encoded(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("AI protocol fixture must encode")
}

#[test]
fn current_work_request_round_trips_through_the_protocol_adapter() {
    let request = decode_work_request(&encoded(&work_request_fixture()))
        .expect("current AI work request must decode");
    let encoded_request =
        encode_work_request(&request).expect("decoded AI work request must re-encode");
    let value: Value =
        serde_json::from_slice(&encoded_request).expect("encoded AI work request must be JSON");

    assert_eq!(
        value["identity"]["schemaVersion"],
        AI_PROTOCOL_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(value["identity"]["family"], "family-1");
    assert_eq!(value["identity"]["requestId"], "request-1");
    assert_eq!(value["identity"]["subject"]["familyId"], "family-1");
    assert_eq!(value["workItemId"], "work-1");
    assert_eq!(value["workKind"], "classification");
    assert_eq!(value["requestedAt"], "2026-08-28T09:00:00Z");
    assert_eq!(value["deadlineAt"], "2026-08-28T09:05:00Z");
    assert_eq!(value["retryPolicy"]["maxAttempts"], 2);
    assert_eq!(value["retryPolicy"]["retryAfterMs"], 1000);
    assert_eq!(value["prompt"], Value::Null);
    assert_eq!(value["runtime"], Value::Null);

    assert_eq!(
        object_keys(&value),
        BTreeSet::from([
            "identity".to_owned(),
            "workItemId".to_owned(),
            "workKind".to_owned(),
            "requestedAt".to_owned(),
            "deadlineAt".to_owned(),
            "retryPolicy".to_owned(),
            "prompt".to_owned(),
            "runtime".to_owned(),
        ])
    );
    assert_eq!(
        object_keys(&value["identity"]),
        BTreeSet::from([
            "schemaVersion".to_owned(),
            "family".to_owned(),
            "requestId".to_owned(),
            "subject".to_owned(),
        ])
    );
    assert_eq!(
        object_keys(&value["identity"]["subject"]),
        BTreeSet::from([
            "familyId".to_owned(),
            "childProfileId".to_owned(),
            "deviceId".to_owned(),
        ])
    );
    assert_eq!(
        object_keys(&value["retryPolicy"]),
        BTreeSet::from(["maxAttempts".to_owned(), "retryAfterMs".to_owned()])
    );
}

fn object_keys(value: &Value) -> BTreeSet<String> {
    value
        .as_object()
        .expect("AI contract value must be an object")
        .keys()
        .cloned()
        .collect()
}

#[test]
fn malformed_protocol_bytes_fail_before_contract_decoding() {
    assert_eq!(
        decode_work_request(b"{not-json"),
        Err(AiProtocolContractError::InvalidEncoding)
    );
}

#[test]
fn stale_or_missing_schema_versions_fail_closed() {
    let mut stale = work_request_fixture();
    stale["identity"]["schemaVersion"] = json!("ai-contracts-v0");
    assert_eq!(
        decode_work_request(&encoded(&stale)),
        Err(AiProtocolContractError::StaleSchemaVersion)
    );

    let mut missing = work_request_fixture();
    missing
        .get_mut("identity")
        .and_then(Value::as_object_mut)
        .expect("fixture identity must be an object")
        .remove("schemaVersion");
    assert_eq!(
        decode_work_request(&encoded(&missing)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );
}

#[test]
fn caller_supplied_prompt_and_runtime_attachments_are_rejected() {
    for (field, attachment) in [
        ("prompt", json!({"text": "caller supplied prompt"})),
        ("runtime", json!({"providerId": "caller supplied provider"})),
    ] {
        let mut value = work_request_fixture();
        value[field] = attachment;

        assert_eq!(
            decode_work_request(&encoded(&value)),
            Err(AiProtocolContractError::OwnerResolvedAttachment),
            "caller-supplied {field} must not cross the protocol boundary"
        );
    }
}

#[test]
fn unknown_wire_fields_are_rejected_by_the_protocol_shape() {
    let mut value = work_request_fixture();
    value["callerAuthority"] = json!("not-an-owner-issued-capability");

    assert_eq!(
        decode_work_request(&encoded(&value)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );
}

#[test]
fn invalid_timestamp_deadline_and_retry_values_are_rejected() {
    let mut invalid_timestamp = work_request_fixture();
    invalid_timestamp["requestedAt"] = json!("2026-08-28T09:00:00");
    assert_eq!(
        decode_work_request(&encoded(&invalid_timestamp)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );

    let mut invalid_deadline = work_request_fixture();
    invalid_deadline["deadlineAt"] = json!("2026-08-28T08:59:59Z");
    assert_eq!(
        decode_work_request(&encoded(&invalid_deadline)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );

    let mut invalid_retry = work_request_fixture();
    invalid_retry["retryPolicy"]["maxAttempts"] = json!(0);
    assert_eq!(
        decode_work_request(&encoded(&invalid_retry)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );
}

#[test]
fn protocol_schema_version_is_borrowed_from_the_neutral_leaf() {
    assert_eq!(
        AI_PROTOCOL_CONTRACT_SCHEMA_VERSION,
        AI_CONTRACT_SCHEMA_VERSION
    );
}

#[test]
fn canonical_ai_enum_values_match_the_shared_wire_contract() {
    assert_wire_values([
        (AiAuthorityBoundary::EvidenceOnly, "evidence-only"),
        (
            AiAuthorityBoundary::DeterministicPolicyRequired,
            "deterministic-policy-required",
        ),
        (
            AiAuthorityBoundary::ManualReviewRequired,
            "manual-review-required",
        ),
    ]);
    assert_wire_values([
        (AiCustodyState::ChildLocalEncrypted, "child-local-encrypted"),
        (
            AiCustodyState::ParentLocalEncrypted,
            "parent-local-encrypted",
        ),
        (
            AiCustodyState::ParentAuthorizedRedacted,
            "parent-authorized-redacted",
        ),
        (AiCustodyState::EphemeralLocal, "ephemeral-local"),
        (AiCustodyState::Deleted, "deleted"),
        (AiCustodyState::Unavailable, "unavailable"),
    ]);
    assert_wire_values([
        (AiRetentionState::Active, "active"),
        (AiRetentionState::Expired, "expired"),
        (AiRetentionState::Tombstoned, "tombstoned"),
        (AiRetentionState::Deleted, "deleted"),
        (AiRetentionState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiRedactionState::NotApplicable, "not-applicable"),
        (AiRedactionState::Redacted, "redacted"),
        (AiRedactionState::FullyRedacted, "fully-redacted"),
        (
            AiRedactionState::RejectedPrivatePayload,
            "rejected-private-payload",
        ),
    ]);
    assert_wire_values([
        (AiDegradedState::None, "none"),
        (AiDegradedState::MissingEvidence, "missing-evidence"),
        (AiDegradedState::InvalidOutput, "invalid-output"),
        (AiDegradedState::Timeout, "timeout"),
        (AiDegradedState::ModelUnavailable, "model-unavailable"),
        (AiDegradedState::ProviderUnavailable, "provider-unavailable"),
        (AiDegradedState::CustodyUnavailable, "custody-unavailable"),
        (AiDegradedState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiDurabilityState::Durable, "durable"),
        (AiDurabilityState::AppendPending, "append-pending"),
        (AiDurabilityState::ReplayOnly, "replay-only"),
        (AiDurabilityState::NotDurable, "not-durable"),
        (AiDurabilityState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiValidationState::Accepted, "accepted"),
        (AiValidationState::Rejected, "rejected"),
        (AiValidationState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiEvidenceKind::Browser, "browser"),
        (AiEvidenceKind::App, "app"),
        (AiEvidenceKind::Game, "game"),
        (AiEvidenceKind::Network, "network"),
        (AiEvidenceKind::ScreenSummary, "screen-summary"),
        (AiEvidenceKind::Activity, "activity"),
        (AiEvidenceKind::ParentRule, "parent-rule"),
        (AiEvidenceKind::Audit, "audit"),
    ]);
    assert_wire_values([
        (AiProvenanceKind::DirectObservation, "direct-observation"),
        (
            AiProvenanceKind::DerivedFromEvidence,
            "derived-from-evidence",
        ),
        (AiProvenanceKind::DerivedFromResult, "derived-from-result"),
        (AiProvenanceKind::ParentAuthoredRule, "parent-authored-rule"),
    ]);
    assert_wire_values([
        (AiContextBuildState::Ready, "ready"),
        (AiContextBuildState::Partial, "partial"),
        (AiContextBuildState::Rejected, "rejected"),
        (AiContextBuildState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiReferenceValidationState::Validated, "validated"),
        (AiReferenceValidationState::MissingSource, "missing-source"),
        (
            AiReferenceValidationState::CustodyBlocked,
            "custody-blocked",
        ),
        (AiReferenceValidationState::Stale, "stale"),
        (AiReferenceValidationState::Rejected, "rejected"),
    ]);
    assert_wire_values([
        (AiResultKind::Observation, "observation"),
        (AiResultKind::Classification, "classification"),
        (AiResultKind::Summary, "summary"),
        (AiResultKind::Explanation, "explanation"),
        (AiResultKind::NoClaim, "no-claim"),
    ]);
    assert_wire_values([
        (AiOutputValidationState::SchemaValid, "schema-valid"),
        (AiOutputValidationState::SchemaInvalid, "schema-invalid"),
        (AiOutputValidationState::EvidenceMissing, "evidence-missing"),
        (
            AiOutputValidationState::ConfidenceInvalid,
            "confidence-invalid",
        ),
        (
            AiOutputValidationState::PolicyHandoffRequired,
            "policy-handoff-required",
        ),
        (AiOutputValidationState::ManualRequired, "manual-required"),
    ]);
    assert_wire_values([
        (AiJournalEntryKind::WorkLifecycle, "work-lifecycle"),
        (AiJournalEntryKind::ContextBuilt, "context-built"),
        (AiJournalEntryKind::ResultValidated, "result-validated"),
        (
            AiJournalEntryKind::ExplanationPublished,
            "explanation-published",
        ),
        (AiJournalEntryKind::RemoteAssistant, "remote-assistant"),
    ]);
    assert_wire_values([
        (AiJournalPayloadKind::WorkItem, "work-item"),
        (AiJournalPayloadKind::EvidenceContext, "evidence-context"),
        (AiJournalPayloadKind::Result, "result"),
        (AiJournalPayloadKind::Explanation, "explanation"),
        (AiJournalPayloadKind::RemoteAssistant, "remote-assistant"),
    ]);
    assert_wire_values([
        (AiWorkKind::ContextBuild, "context-build"),
        (AiWorkKind::Classification, "classification"),
        (AiWorkKind::Explanation, "explanation"),
        (AiWorkKind::MemoryDerivation, "memory-derivation"),
        (AiWorkKind::GraphDerivation, "graph-derivation"),
        (AiWorkKind::ParentAssistant, "parent-assistant"),
        (AiWorkKind::RemoteAssistant, "remote-assistant"),
    ]);
}

fn assert_wire_values<T, const N: usize>(values: [(T, &str); N])
where
    T: serde::Serialize,
{
    for (value, expected) in values {
        assert_eq!(
            serde_json::to_value(value).expect("AI enum must serialize"),
            Value::String(expected.to_owned())
        );
    }
}

#[test]
fn unknown_work_kind_and_owner_authority_values_fail_closed() {
    let mut unknown_kind = work_request_fixture();
    unknown_kind["workKind"] = json!("caller-minted-authority");
    assert_eq!(
        decode_work_request(&encoded(&unknown_kind)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );

    let mut unknown_boundary = work_request_fixture();
    unknown_boundary["authorityBoundary"] = json!("manual-review-required");
    assert_eq!(
        decode_work_request(&encoded(&unknown_boundary)),
        Err(AiProtocolContractError::InvalidWorkRequest)
    );
}
