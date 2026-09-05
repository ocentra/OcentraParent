use std::error::Error;

use ocentra_ai_contracts::ai_contracts::{
    identity::{AiActorId, AiActorIdentity, AiFamilyId, AiSchemaIdentity, AiTimestamp},
    remote_assistant::AiRemoteAssistantWireRequest,
    work::{AiRetryPolicy, AiWorkKind, AiWorkRequest},
    AiConfidence, AI_CONTRACT_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

type TestResult = Result<(), Box<dyn Error>>;

fn test_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        message.into(),
    ))
}

fn assert_rejected<T: DeserializeOwned>(
    value: Value,
    expected_fragment: &str,
    failure_message: &str,
) -> TestResult {
    let error = match serde_json::from_value::<T>(value) {
        Ok(_) => return Err(test_error(failure_message)),
        Err(error) => error,
    };
    if !error.to_string().contains(expected_fragment) {
        return Err(test_error(format!(
            "expected error containing {expected_fragment:?}, got {error}"
        )));
    }
    Ok(())
}

fn actor_id(value: &str) -> Result<AiActorId, Box<dyn Error>> {
    AiActorId::parse(value).ok_or_else(|| test_error("actor id fixture"))
}

fn identity_json() -> Value {
    json!({
        "schemaVersion": AI_CONTRACT_SCHEMA_VERSION,
        "family": "family-1",
        "requestId": "request-1",
        "subject": {
            "familyId": "family-1",
            "childProfileId": "child-1",
            "deviceId": "device-1"
        }
    })
}

fn identity_fixture() -> Result<AiSchemaIdentity, Box<dyn Error>> {
    serde_json::from_value(identity_json()).map_err(Into::into)
}

fn timestamp(value: &str) -> Result<AiTimestamp, Box<dyn Error>> {
    AiTimestamp::parse(value).ok_or_else(|| test_error("timestamp fixture"))
}

fn remote_wire_request_json() -> Value {
    json!({
        "schemaVersion": AI_CONTRACT_SCHEMA_VERSION,
        "requestId": "remote-request-1",
        "familyId": "family-1",
        "authorizationReferenceId": "authorization-1",
        "prompt": {
            "templateId": "template-1",
            "version": "prompt-v1",
            "task": "Summarize redacted references"
        },
        "requestedAt": "2026-08-28T09:00:00Z",
        "state": "submitted"
    })
}

#[test]
fn confidence_rejects_values_outside_the_finite_unit_interval() -> TestResult {
    assert_eq!(AiConfidence::parse(-0.01), None);
    assert_eq!(AiConfidence::parse(1.01), None);
    assert_eq!(AiConfidence::parse(f64::NAN), None);
    assert_rejected::<AiConfidence>(
        json!(-0.01),
        "finite and between 0 and 1",
        "negative confidence must be rejected",
    )?;
    assert_rejected::<AiConfidence>(
        json!(1.01),
        "finite and between 0 and 1",
        "confidence above one must be rejected",
    )?;
    Ok(())
}

#[test]
fn identifiers_and_timestamps_reject_hostile_or_noncanonical_values() {
    assert_eq!(AiFamilyId::parse(""), None);
    assert_eq!(AiFamilyId::parse(" family-1"), None);
    assert_eq!(AiFamilyId::parse("family\n1"), None);
    assert_eq!(
        AiFamilyId::parse("x".repeat(257)),
        None,
        "identifier length must remain bounded"
    );

    assert_eq!(AiTimestamp::parse("2026-08-28T09:00:00"), None);
    assert_eq!(AiTimestamp::parse("2026-02-29T09:00:00Z"), None);
    assert_eq!(
        AiTimestamp::parse("2024-02-29T09:00:00Z").map(|timestamp| timestamp.to_string()),
        Some("2024-02-29T09:00:00Z".to_owned())
    );
    assert_eq!(AiTimestamp::parse("2026-08-28T24:00:00Z"), None);
    assert_eq!(AiTimestamp::parse("2026-08-28T09:00:00+00:00"), None);
}

#[test]
fn schema_identity_rejects_stale_versions_and_cross_family_subjects() -> TestResult {
    let mut stale = identity_json();
    stale["schemaVersion"] = json!("ai-contracts-v0");
    assert_rejected::<AiSchemaIdentity>(
        stale,
        "AI contract schema version is not current",
        "stale AI schema identity must be rejected",
    )?;

    let mut mismatched = identity_json();
    mismatched["subject"]["familyId"] = json!("family-other");
    assert_rejected::<AiSchemaIdentity>(
        mismatched,
        "family does not match its subject",
        "cross-family identity must be rejected",
    )?;
    Ok(())
}

#[test]
fn actor_identity_rejects_parent_authority_and_role_subject_mismatches() -> TestResult {
    assert_eq!(
        AiActorIdentity::new(
            actor_id("actor-parent")?,
            ocentra_ai_contracts::ai_contracts::identity::AiActorRole::Parent,
            None,
        )
        .err(),
        Some("parent authority identity requires a trusted issuer")
    );

    assert_rejected::<AiActorIdentity>(
        json!({
            "actorId": "actor-parent",
            "role": "parent",
            "subject": null
        }),
        "parent authority identity requires a trusted issuer",
        "wire parent authority must be rejected",
    )?;
    assert_rejected::<AiActorIdentity>(
        json!({
            "actorId": "actor-runtime",
            "role": "local-runtime",
            "subject": null
        }),
        "does not match the actor role",
        "runtime actors require a subject",
    )?;
    assert_rejected::<AiActorIdentity>(
        json!({
            "actorId": "actor-system",
            "role": "system",
            "subject": identity_json()["subject"]
        }),
        "does not match the actor role",
        "system actors cannot carry a subject",
    )?;
    Ok(())
}

#[test]
fn retry_and_work_request_boundaries_reject_zero_or_nonadvancing_values() -> TestResult {
    assert_rejected::<AiRetryPolicy>(
        json!({ "maxAttempts": 0, "retryAfterMs": null }),
        "requires at least one attempt",
        "zero retry attempts must be rejected",
    )?;

    let identity = identity_fixture()?;
    let retry = AiRetryPolicy::new(1, None).map_err(test_error)?;
    let requested_at = timestamp("2026-08-28T09:00:00Z")?;
    let equal_deadline = timestamp("2026-08-28T09:00:00Z")?;
    let equal_error = AiWorkRequest::new(
        identity.clone(),
        ocentra_ai_contracts::ai_contracts::identity::AiWorkItemId::parse("work-equal")
            .ok_or_else(|| test_error("work item fixture"))?,
        AiWorkKind::Classification,
        requested_at.clone(),
        Some(equal_deadline),
        retry.clone(),
        None,
    )
    .err()
    .ok_or_else(|| test_error("equal work deadline must be rejected"))?;
    assert_eq!(
        equal_error,
        "AI work request has an invalid requested/deadline timestamp"
    );

    let earlier_deadline = timestamp("2026-08-28T08:59:59Z")?;
    let earlier_error = AiWorkRequest::new(
        identity,
        ocentra_ai_contracts::ai_contracts::identity::AiWorkItemId::parse("work-earlier")
            .ok_or_else(|| test_error("work item fixture"))?,
        AiWorkKind::Classification,
        requested_at,
        Some(earlier_deadline),
        retry,
        None,
    )
    .err()
    .ok_or_else(|| test_error("earlier work deadline must be rejected"))?;
    assert_eq!(
        earlier_error,
        "AI work request has an invalid requested/deadline timestamp"
    );
    Ok(())
}

#[test]
fn remote_wire_boundary_rejects_stale_state_untrusted_text_and_owner_fields() -> TestResult {
    let mut wrong_state = remote_wire_request_json();
    wrong_state["state"] = json!("authorized");
    assert_rejected::<AiRemoteAssistantWireRequest>(
        wrong_state,
        "not fail-closed safe",
        "remote wire authorization state must be rejected",
    )?;

    let mut empty_task = remote_wire_request_json();
    empty_task["prompt"]["task"] = json!("");
    assert_rejected::<AiRemoteAssistantWireRequest>(
        empty_task,
        "untrusted text is empty",
        "empty remote wire text must be rejected",
    )?;

    let mut unknown_owner_field = remote_wire_request_json();
    unknown_owner_field["evidenceReferenceIds"] = json!(["caller-evidence"]);
    assert_rejected::<AiRemoteAssistantWireRequest>(
        unknown_owner_field,
        "unknown field",
        "caller-supplied owner evidence must be rejected",
    )?;

    let mut unknown_prompt_field = remote_wire_request_json();
    unknown_prompt_field["prompt"]["safeText"] = json!({
        "text": "caller-minted safe text",
        "redaction": "redacted"
    });
    assert_rejected::<AiRemoteAssistantWireRequest>(
        unknown_prompt_field,
        "unknown field",
        "caller-minted safe prompt fields must be rejected",
    )?;
    Ok(())
}

#[test]
fn remote_wire_request_rejects_stale_schema_version() -> TestResult {
    let mut stale = remote_wire_request_json();
    stale["schemaVersion"] = json!("ai-contracts-v0");
    assert_rejected::<AiRemoteAssistantWireRequest>(
        stale,
        "AI contract schema version is not current",
        "stale remote wire schema must be rejected",
    )?;
    Ok(())
}
