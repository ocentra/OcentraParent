use std::error::Error;

use ocentra_ai_contracts::ai_contracts::{
    context::{AiContextBuildState, AiEvidenceKind, AiProvenanceKind, AiReferenceValidationState},
    explanation::{AiExplanationState, AiExplanationSurface},
    identity::{
        AiActorRole, AiChildProfileId, AiDeviceId, AiFamilyId, AiRequestId, AiSchemaIdentity,
        AiSchemaVersion, AiTimestamp, AiWorkItemId,
    },
    journal::{AiJournalEntryKind, AiJournalPayloadKind},
    memory::{AiGraphEdgeKind, AiGraphNodeKind, AiMemoryReferenceKind},
    remote_assistant::{
        AiRemoteAssistantRedactionPolicy, AiRemoteAssistantSafetyBoundary, AiRemoteAssistantState,
        AiRemoteAssistantWireRequest,
    },
    result::{AiOutputValidationState, AiResultKind},
    work::{AiRetryPolicy, AiWorkKind, AiWorkRequest, AiWorkState},
    AiAuthorityBoundary, AiCustodyState, AiDegradedState, AiDurabilityState, AiRedactionState,
    AiRetentionState, AiValidationState, AI_CONTRACT_SCHEMA_VERSION,
};
use serde::Serialize;
use serde_json::{json, to_value};

type TestResult = Result<(), Box<dyn Error>>;

fn test_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        message.into(),
    ))
}

macro_rules! parse_identifier {
    ($function:ident, $type:ty) => {
        fn $function(value: &str) -> Result<$type, Box<dyn Error>> {
            <$type>::parse(value).ok_or_else(|| test_error(stringify!($function)))
        }
    };
}

parse_identifier!(family_id, AiFamilyId);
parse_identifier!(child_profile_id, AiChildProfileId);
parse_identifier!(device_id, AiDeviceId);
parse_identifier!(request_id, AiRequestId);
parse_identifier!(work_item_id, AiWorkItemId);
parse_identifier!(timestamp, AiTimestamp);

fn identity_fixture() -> Result<AiSchemaIdentity, Box<dyn Error>> {
    let family = family_id("family-1")?;
    let subject = ocentra_ai_contracts::ai_contracts::identity::AiSubjectIdentity::new(
        family.clone(),
        Some(child_profile_id("child-1")?),
        Some(device_id("device-1")?),
    )
    .map_err(test_error)?;
    AiSchemaIdentity::new(
        AiSchemaVersion::current(),
        family,
        request_id("request-1")?,
        subject,
    )
    .map_err(test_error)
}

fn serialized_enum<T: Serialize>(value: T) -> Result<String, Box<dyn Error>> {
    to_value(value)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| test_error("AI enum did not serialize as a string"))
}

#[test]
fn work_request_serializes_with_current_schema_and_canonical_field_names() -> TestResult {
    let request = AiWorkRequest::new(
        identity_fixture()?,
        work_item_id("work-1")?,
        AiWorkKind::Classification,
        timestamp("2026-08-28T09:00:00Z")?,
        Some(timestamp("2026-08-28T09:05:00Z")?),
        AiRetryPolicy::new(2, Some(1_000)).map_err(test_error)?,
        None,
    )
    .map_err(test_error)?;

    assert_eq!(
        to_value(request)?,
        json!({
            "identity": {
                "schemaVersion": AI_CONTRACT_SCHEMA_VERSION,
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
            },
            "prompt": null,
            "runtime": null
        })
    );
    Ok(())
}

#[test]
fn authority_and_custody_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(
        serialized_enum(AiAuthorityBoundary::EvidenceOnly)?,
        "evidence-only"
    );
    assert_eq!(
        serialized_enum(AiAuthorityBoundary::DeterministicPolicyRequired)?,
        "deterministic-policy-required"
    );
    assert_eq!(
        serialized_enum(AiAuthorityBoundary::ManualReviewRequired)?,
        "manual-review-required"
    );

    assert_eq!(
        serialized_enum(AiCustodyState::ChildLocalEncrypted)?,
        "child-local-encrypted"
    );
    assert_eq!(
        serialized_enum(AiCustodyState::ParentLocalEncrypted)?,
        "parent-local-encrypted"
    );
    assert_eq!(
        serialized_enum(AiCustodyState::ParentAuthorizedRedacted)?,
        "parent-authorized-redacted"
    );
    assert_eq!(
        serialized_enum(AiCustodyState::EphemeralLocal)?,
        "ephemeral-local"
    );
    assert_eq!(serialized_enum(AiCustodyState::Deleted)?, "deleted");
    assert_eq!(serialized_enum(AiCustodyState::Unavailable)?, "unavailable");

    assert_eq!(serialized_enum(AiRetentionState::Active)?, "active");
    assert_eq!(serialized_enum(AiRetentionState::Expired)?, "expired");
    assert_eq!(serialized_enum(AiRetentionState::Tombstoned)?, "tombstoned");
    assert_eq!(serialized_enum(AiRetentionState::Deleted)?, "deleted");
    assert_eq!(
        serialized_enum(AiRetentionState::ManualRequired)?,
        "manual-required"
    );

    assert_eq!(
        serialized_enum(AiRedactionState::NotApplicable)?,
        "not-applicable"
    );
    assert_eq!(serialized_enum(AiRedactionState::Redacted)?, "redacted");
    assert_eq!(
        serialized_enum(AiRedactionState::FullyRedacted)?,
        "fully-redacted"
    );
    assert_eq!(
        serialized_enum(AiRedactionState::RejectedPrivatePayload)?,
        "rejected-private-payload"
    );

    assert_eq!(serialized_enum(AiDegradedState::None)?, "none");
    assert_eq!(
        serialized_enum(AiDegradedState::MissingEvidence)?,
        "missing-evidence"
    );
    assert_eq!(
        serialized_enum(AiDegradedState::InvalidOutput)?,
        "invalid-output"
    );
    assert_eq!(serialized_enum(AiDegradedState::Timeout)?, "timeout");
    assert_eq!(
        serialized_enum(AiDegradedState::ModelUnavailable)?,
        "model-unavailable"
    );
    assert_eq!(
        serialized_enum(AiDegradedState::ProviderUnavailable)?,
        "provider-unavailable"
    );
    assert_eq!(
        serialized_enum(AiDegradedState::CustodyUnavailable)?,
        "custody-unavailable"
    );
    assert_eq!(
        serialized_enum(AiDegradedState::ManualRequired)?,
        "manual-required"
    );

    assert_eq!(serialized_enum(AiDurabilityState::Durable)?, "durable");
    assert_eq!(
        serialized_enum(AiDurabilityState::AppendPending)?,
        "append-pending"
    );
    assert_eq!(
        serialized_enum(AiDurabilityState::ReplayOnly)?,
        "replay-only"
    );
    assert_eq!(
        serialized_enum(AiDurabilityState::NotDurable)?,
        "not-durable"
    );
    assert_eq!(
        serialized_enum(AiDurabilityState::ManualRequired)?,
        "manual-required"
    );
    assert_eq!(serialized_enum(AiValidationState::Accepted)?, "accepted");
    assert_eq!(serialized_enum(AiValidationState::Rejected)?, "rejected");
    assert_eq!(
        serialized_enum(AiValidationState::ManualRequired)?,
        "manual-required"
    );
    Ok(())
}

#[test]
fn evidence_and_reference_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(serialized_enum(AiEvidenceKind::Browser)?, "browser");
    assert_eq!(serialized_enum(AiEvidenceKind::App)?, "app");
    assert_eq!(serialized_enum(AiEvidenceKind::Game)?, "game");
    assert_eq!(serialized_enum(AiEvidenceKind::Network)?, "network");
    assert_eq!(
        serialized_enum(AiEvidenceKind::ScreenSummary)?,
        "screen-summary"
    );
    assert_eq!(serialized_enum(AiEvidenceKind::Activity)?, "activity");
    assert_eq!(serialized_enum(AiEvidenceKind::ParentRule)?, "parent-rule");
    assert_eq!(serialized_enum(AiEvidenceKind::Audit)?, "audit");
    assert_eq!(
        serialized_enum(AiProvenanceKind::DirectObservation)?,
        "direct-observation"
    );
    assert_eq!(
        serialized_enum(AiProvenanceKind::DerivedFromEvidence)?,
        "derived-from-evidence"
    );
    assert_eq!(
        serialized_enum(AiProvenanceKind::DerivedFromResult)?,
        "derived-from-result"
    );
    assert_eq!(
        serialized_enum(AiProvenanceKind::ParentAuthoredRule)?,
        "parent-authored-rule"
    );
    assert_eq!(serialized_enum(AiContextBuildState::Ready)?, "ready");
    assert_eq!(serialized_enum(AiContextBuildState::Partial)?, "partial");
    assert_eq!(serialized_enum(AiContextBuildState::Rejected)?, "rejected");
    assert_eq!(
        serialized_enum(AiContextBuildState::ManualRequired)?,
        "manual-required"
    );
    assert_eq!(
        serialized_enum(AiReferenceValidationState::Validated)?,
        "validated"
    );
    assert_eq!(
        serialized_enum(AiReferenceValidationState::MissingSource)?,
        "missing-source"
    );
    assert_eq!(
        serialized_enum(AiReferenceValidationState::CustodyBlocked)?,
        "custody-blocked"
    );
    assert_eq!(serialized_enum(AiReferenceValidationState::Stale)?, "stale");
    assert_eq!(
        serialized_enum(AiReferenceValidationState::Rejected)?,
        "rejected"
    );
    Ok(())
}

#[test]
fn memory_graph_and_journal_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(
        serialized_enum(AiMemoryReferenceKind::RecentActivity)?,
        "recent-activity"
    );
    assert_eq!(
        serialized_enum(AiMemoryReferenceKind::EvidenceMemory)?,
        "evidence-memory"
    );
    assert_eq!(
        serialized_enum(AiMemoryReferenceKind::SemanticMemory)?,
        "semantic-memory"
    );
    assert_eq!(
        serialized_enum(AiMemoryReferenceKind::PolicyMemory)?,
        "policy-memory"
    );
    assert_eq!(serialized_enum(AiGraphNodeKind::Evidence)?, "evidence");
    assert_eq!(serialized_enum(AiGraphNodeKind::Activity)?, "activity");
    assert_eq!(serialized_enum(AiGraphNodeKind::Result)?, "result");
    assert_eq!(serialized_enum(AiGraphNodeKind::Memory)?, "memory");
    assert_eq!(serialized_enum(AiGraphNodeKind::PolicyRule)?, "policy-rule");
    assert_eq!(serialized_enum(AiGraphEdgeKind::Supports)?, "supports");
    assert_eq!(
        serialized_enum(AiGraphEdgeKind::DerivedFrom)?,
        "derived-from"
    );
    assert_eq!(serialized_enum(AiGraphEdgeKind::RelatedTo)?, "related-to");
    assert_eq!(serialized_enum(AiGraphEdgeKind::GovernedBy)?, "governed-by");

    assert_eq!(
        serialized_enum(AiJournalEntryKind::WorkLifecycle)?,
        "work-lifecycle"
    );
    assert_eq!(
        serialized_enum(AiJournalEntryKind::ContextBuilt)?,
        "context-built"
    );
    assert_eq!(
        serialized_enum(AiJournalEntryKind::ResultValidated)?,
        "result-validated"
    );
    assert_eq!(
        serialized_enum(AiJournalEntryKind::ExplanationPublished)?,
        "explanation-published"
    );
    assert_eq!(
        serialized_enum(AiJournalEntryKind::RemoteAssistant)?,
        "remote-assistant"
    );
    assert_eq!(
        serialized_enum(AiJournalPayloadKind::WorkItem)?,
        "work-item"
    );
    assert_eq!(
        serialized_enum(AiJournalPayloadKind::EvidenceContext)?,
        "evidence-context"
    );
    assert_eq!(serialized_enum(AiJournalPayloadKind::Result)?, "result");
    assert_eq!(
        serialized_enum(AiJournalPayloadKind::Explanation)?,
        "explanation"
    );
    assert_eq!(
        serialized_enum(AiJournalPayloadKind::RemoteAssistant)?,
        "remote-assistant"
    );
    Ok(())
}

#[test]
fn result_and_output_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(serialized_enum(AiResultKind::Observation)?, "observation");
    assert_eq!(
        serialized_enum(AiResultKind::Classification)?,
        "classification"
    );
    assert_eq!(serialized_enum(AiResultKind::Summary)?, "summary");
    assert_eq!(serialized_enum(AiResultKind::Explanation)?, "explanation");
    assert_eq!(serialized_enum(AiResultKind::NoClaim)?, "no-claim");
    assert_eq!(
        serialized_enum(AiOutputValidationState::SchemaValid)?,
        "schema-valid"
    );
    assert_eq!(
        serialized_enum(AiOutputValidationState::SchemaInvalid)?,
        "schema-invalid"
    );
    assert_eq!(
        serialized_enum(AiOutputValidationState::EvidenceMissing)?,
        "evidence-missing"
    );
    assert_eq!(
        serialized_enum(AiOutputValidationState::ConfidenceInvalid)?,
        "confidence-invalid"
    );
    assert_eq!(
        serialized_enum(AiOutputValidationState::PolicyHandoffRequired)?,
        "policy-handoff-required"
    );
    assert_eq!(
        serialized_enum(AiOutputValidationState::ManualRequired)?,
        "manual-required"
    );
    Ok(())
}

#[test]
fn explanation_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(
        serialized_enum(AiExplanationSurface::ParentReadModel)?,
        "parent-read-model"
    );
    assert_eq!(
        serialized_enum(AiExplanationSurface::ChildSafetyInternal)?,
        "child-safety-internal"
    );
    assert_eq!(
        serialized_enum(AiExplanationSurface::AuditRecord)?,
        "audit-record"
    );
    assert_eq!(serialized_enum(AiExplanationState::Ready)?, "ready");
    assert_eq!(serialized_enum(AiExplanationState::Degraded)?, "degraded");
    assert_eq!(
        serialized_enum(AiExplanationState::Unavailable)?,
        "unavailable"
    );
    assert_eq!(
        serialized_enum(AiExplanationState::ManualRequired)?,
        "manual-required"
    );
    Ok(())
}

#[test]
fn actor_and_work_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(serialized_enum(AiActorRole::Parent)?, "parent");
    assert_eq!(serialized_enum(AiActorRole::ChildAgent)?, "child-agent");
    assert_eq!(serialized_enum(AiActorRole::LocalRuntime)?, "local-runtime");
    assert_eq!(
        serialized_enum(AiActorRole::ParentAssistant)?,
        "parent-assistant"
    );
    assert_eq!(
        serialized_enum(AiActorRole::RemoteAssistant)?,
        "remote-assistant"
    );
    assert_eq!(serialized_enum(AiActorRole::System)?, "system");
    assert_eq!(serialized_enum(AiWorkKind::ContextBuild)?, "context-build");
    assert_eq!(
        serialized_enum(AiWorkKind::Classification)?,
        "classification"
    );
    assert_eq!(serialized_enum(AiWorkKind::Explanation)?, "explanation");
    assert_eq!(
        serialized_enum(AiWorkKind::MemoryDerivation)?,
        "memory-derivation"
    );
    assert_eq!(
        serialized_enum(AiWorkKind::GraphDerivation)?,
        "graph-derivation"
    );
    assert_eq!(
        serialized_enum(AiWorkKind::ParentAssistant)?,
        "parent-assistant"
    );
    assert_eq!(
        serialized_enum(AiWorkKind::RemoteAssistant)?,
        "remote-assistant"
    );
    assert_eq!(serialized_enum(AiWorkState::Queued)?, "queued");
    assert_eq!(serialized_enum(AiWorkState::Claimed)?, "claimed");
    assert_eq!(serialized_enum(AiWorkState::Running)?, "running");
    assert_eq!(serialized_enum(AiWorkState::Succeeded)?, "succeeded");
    assert_eq!(serialized_enum(AiWorkState::Failed)?, "failed");
    assert_eq!(serialized_enum(AiWorkState::Cancelled)?, "cancelled");
    assert_eq!(serialized_enum(AiWorkState::TimedOut)?, "timed-out");
    assert_eq!(
        serialized_enum(AiWorkState::ManualRequired)?,
        "manual-required"
    );
    Ok(())
}

#[test]
fn remote_assistant_enums_serialize_to_generated_kebab_case_literals() -> TestResult {
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::Disabled)?,
        "disabled"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::AwaitingParentAuthorization)?,
        "awaiting-parent-authorization"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::Authorized)?,
        "authorized"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::Submitted)?,
        "submitted"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::Succeeded)?,
        "succeeded"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::Degraded)?,
        "degraded"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantState::ManualRequired)?,
        "manual-required"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantSafetyBoundary::ParentReportOnly)?,
        "parent-report-only"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath)?,
        "outside-child-safety-blocking-path"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantRedactionPolicy::ReferencesOnly)?,
        "references-only"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantRedactionPolicy::RedactedSummaries)?,
        "redacted-summaries"
    );
    assert_eq!(
        serialized_enum(AiRemoteAssistantRedactionPolicy::NoChildPayload)?,
        "no-child-payload"
    );
    Ok(())
}

#[test]
fn work_state_transition_contract_is_fail_closed() {
    assert!(AiWorkState::Queued.can_transition_from(None));
    assert!(!AiWorkState::Running.can_transition_from(None));
    assert!(AiWorkState::Running.can_transition_from(Some(AiWorkState::Claimed)));
    assert!(!AiWorkState::Succeeded.can_transition_from(Some(AiWorkState::Queued)));
    assert!(AiWorkState::Succeeded.is_terminal());
    assert!(!AiWorkState::Queued.is_terminal());
}

#[test]
fn submitted_remote_wire_request_keeps_only_reference_metadata() -> TestResult {
    let request: AiRemoteAssistantWireRequest = serde_json::from_value(json!({
        "schemaVersion": AI_CONTRACT_SCHEMA_VERSION,
        "requestId": "remote-request-1",
        "familyId": "family-1",
        "authorizationReferenceId": "authorization-1",
        "prompt": {
            "templateId": "template-1",
            "version": "prompt-v1",
            "task": "Summarize the redacted evidence references"
        },
        "requestedAt": "2026-08-28T09:00:00Z",
        "state": "submitted"
    }))?;

    assert_eq!(request.family_id().as_str(), "family-1");
    assert_eq!(request.request_id().as_str(), "remote-request-1");
    assert_eq!(
        request.authorization_reference_id().as_str(),
        "authorization-1"
    );
    assert_eq!(request.wire_requested_at().as_str(), "2026-08-28T09:00:00Z");
    Ok(())
}

#[test]
fn generated_typescript_source_is_rust_versioned_and_declares_owner_boundaries() {
    let generated = ocentra_ai_contracts::ai_contracts_ts::ai_contracts_typescript();
    assert_eq!(
        generated.lines().next(),
        Some("// Rust schema version: ai-contracts-v1")
    );
    let schema_declarations = generated
        .lines()
        .filter(|line| line.starts_with("export const AiContractSchemaVersion = "))
        .collect::<Vec<_>>();
    assert_eq!(
        schema_declarations,
        vec!["export const AiContractSchemaVersion = \"ai-contracts-v1\" as const;"]
    );
    let required_interfaces = generated
        .lines()
        .filter_map(|line| line.strip_prefix("export interface "))
        .filter_map(|line| line.strip_suffix(" {"))
        .filter(|name| {
            matches!(
                *name,
                "AiEvidenceContext" | "AiResult" | "AiRemoteAssistantWireRequest"
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        required_interfaces,
        vec![
            "AiEvidenceContext",
            "AiResult",
            "AiRemoteAssistantWireRequest"
        ]
    );
    let authority_boundary_fields = generated
        .lines()
        .filter_map(|line| line.strip_prefix("  authorityBoundary: "))
        .collect::<Vec<_>>();
    assert_eq!(
        authority_boundary_fields,
        vec![
            "AiAuthorityBoundary;",
            "AiAuthorityBoundary;",
            "AiAuthorityBoundary;",
            "AiAuthorityBoundary;",
            "AiAuthorityBoundary;",
        ]
    );
    let digest_fields = generated
        .lines()
        .filter_map(|line| line.strip_prefix("  digest: "))
        .collect::<Vec<_>>();
    assert_eq!(digest_fields, vec!["AiDigest;", "AiDigest;"]);
}
