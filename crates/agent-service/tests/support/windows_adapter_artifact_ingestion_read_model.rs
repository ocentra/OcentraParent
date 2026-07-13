#![cfg(test)]

use ocentra_parent_agent_protocol::constants::{
    windows_adapter_artifact_gate as artifact_gate,
    windows_adapter_artifact_ingestion as artifact_ingestion,
};
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::windows_adapter_artifact_gate::{
    WindowsAdapterArtifactEvidence, WindowsAdapterArtifactKind,
};
use ocentra_parent_agent_protocol::windows_adapter_artifact_ingestion::{
    WindowsAdapterArtifactIngestionAcceptedRecord, WindowsAdapterArtifactIngestionProof,
    WindowsAdapterArtifactIngestionRecord, WindowsAdapterArtifactIngestionRejection,
};
use ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilitySurface;

use crate::test_text::TestText;
use crate::windows_adapter_artifact_gate_read_model::evaluate_windows_adapter_artifact_gate;

pub(crate) fn evaluate_windows_adapter_artifact_ingestion(
    generated_at: impl std::fmt::Display,
    records: &[WindowsAdapterArtifactIngestionRecord],
) -> WindowsAdapterArtifactIngestionProof {
    let generated_at = TestText::from_display(generated_at);
    let evaluated = evaluated_records(generated_at.clone(), records);
    let accepted_evidence: Vec<_> = evaluated
        .accepted_records
        .iter()
        .map(|record| record.evidence.clone())
        .collect();

    WindowsAdapterArtifactIngestionProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: artifact_ingestion::READ_MODEL_ID_V0_8.to_string(),
        generated_at: generated_at.to_string(),
        accepted_records: evaluated.accepted_records,
        rejected_records: evaluated.rejected_records,
        gate_proof: evaluate_windows_adapter_artifact_gate(
            &generated_at.to_string(),
            &accepted_evidence,
        ),
        product_claim_boundary: artifact_ingestion::CLAIM_BOUNDARY.to_string(),
    }
}

struct EvaluatedRecords {
    accepted_records: Vec<WindowsAdapterArtifactIngestionAcceptedRecord>,
    rejected_records: Vec<WindowsAdapterArtifactIngestionRejection>,
}

fn evaluated_records(
    generated_at: TestText,
    records: &[WindowsAdapterArtifactIngestionRecord],
) -> EvaluatedRecords {
    let mut accepted_records = Vec::new();
    let mut rejected_records = Vec::new();

    for record in records {
        let refusal_reasons = refusal_reasons(record);
        if refusal_reasons.is_empty() {
            accepted_records.push(accepted_record(record));
        } else {
            rejected_records.push(rejected_record(&generated_at, record, refusal_reasons));
        }
    }

    EvaluatedRecords {
        accepted_records,
        rejected_records,
    }
}

fn accepted_record(
    record: &WindowsAdapterArtifactIngestionRecord,
) -> WindowsAdapterArtifactIngestionAcceptedRecord {
    WindowsAdapterArtifactIngestionAcceptedRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: record.ingestion_record_id.clone(),
        evidence: WindowsAdapterArtifactEvidence {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            artifact_id: record.artifact_id.clone(),
            artifact_kind: record.artifact_kind,
            surface: record.surface,
            subject_ref: record.target_subject_ref.clone(),
            custody_event_id: record.custody_event_id.clone(),
            verified_at: record.collected_at.clone(),
        },
    }
}

fn rejected_record(
    generated_at: &TestText,
    record: &WindowsAdapterArtifactIngestionRecord,
    refusal_reasons: Vec<TestText>,
) -> WindowsAdapterArtifactIngestionRejection {
    WindowsAdapterArtifactIngestionRejection {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: record.ingestion_record_id.clone(),
        artifact_id: record.artifact_id.clone(),
        artifact_kind: record.artifact_kind,
        surface: record.surface,
        refusal_reasons: refusal_reasons
            .into_iter()
            .map(|reason| reason.to_string())
            .collect(),
        rejected_at: generated_at.to_string(),
    }
}

fn refusal_reasons(record: &WindowsAdapterArtifactIngestionRecord) -> Vec<TestText> {
    let unsupported_surface = unsupported_ingestion_surface(record.surface);
    let subject_mismatch = !record.target_subject_ref.is_empty()
        && !record.artifact_subject_ref.is_empty()
        && record.target_subject_ref != record.artifact_subject_ref;
    let kind_surface_mismatch = !unsupported_surface
        && !artifact_kind_matches_surface(record.artifact_kind, record.surface);

    [
        record
            .artifact_id
            .is_empty()
            .then_some(TestText::from_display(
                artifact_ingestion::REFUSAL_EMPTY_ARTIFACT_ID,
            )),
        record
            .target_subject_ref
            .is_empty()
            .then_some(TestText::from_display(
                artifact_ingestion::REFUSAL_EMPTY_TARGET_SUBJECT,
            )),
        record
            .artifact_subject_ref
            .is_empty()
            .then_some(TestText::from_display(
                artifact_ingestion::REFUSAL_EMPTY_ARTIFACT_SUBJECT,
            )),
        (!has_custody_event(record)).then_some(TestText::from_display(
            artifact_ingestion::REFUSAL_MISSING_CUSTODY_EVENT,
        )),
        subject_mismatch.then_some(TestText::from_display(
            artifact_ingestion::REFUSAL_SUBJECT_MISMATCH,
        )),
        unsupported_surface.then_some(TestText::from_display(
            artifact_ingestion::REFUSAL_UNSUPPORTED_SURFACE,
        )),
        kind_surface_mismatch.then_some(TestText::from_display(
            artifact_ingestion::REFUSAL_KIND_SURFACE_MISMATCH,
        )),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn has_custody_event(record: &WindowsAdapterArtifactIngestionRecord) -> bool {
    record
        .custody_event_id
        .as_deref()
        .is_some_and(|custody_event_id| !custody_event_id.is_empty())
}

fn unsupported_ingestion_surface(surface: WindowsAdapterCapabilitySurface) -> bool {
    matches!(
        surface,
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget
            | WindowsAdapterCapabilitySurface::UnsupportedOsTarget
    )
}

fn artifact_kind_matches_surface(
    artifact_kind: WindowsAdapterArtifactKind,
    surface: WindowsAdapterCapabilitySurface,
) -> bool {
    match surface {
        WindowsAdapterCapabilitySurface::AppTarget => app_artifact_kind(artifact_kind),
        WindowsAdapterCapabilitySurface::DomainNetworkTarget => domain_artifact_kind(artifact_kind),
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget => {
            managed_browser_artifact_kind(artifact_kind)
        }
        WindowsAdapterCapabilitySurface::RollbackAuditTarget => app_artifact_kind(artifact_kind),
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget
        | WindowsAdapterCapabilitySurface::UnsupportedOsTarget => false,
    }
}

fn app_artifact_kind(artifact_kind: WindowsAdapterArtifactKind) -> bool {
    matches!(
        artifact_kind,
        WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence
            | WindowsAdapterArtifactKind::AdapterApplyResult
            | WindowsAdapterArtifactKind::AdapterRollbackResult
            | WindowsAdapterArtifactKind::AuditCustodyEvent
    )
}

fn domain_artifact_kind(artifact_kind: WindowsAdapterArtifactKind) -> bool {
    matches!(
        artifact_kind,
        WindowsAdapterArtifactKind::NetworkDomainFilterApply
            | WindowsAdapterArtifactKind::NetworkDomainFilterRollback
            | WindowsAdapterArtifactKind::AuditCustodyEvent
    )
}

fn managed_browser_artifact_kind(artifact_kind: WindowsAdapterArtifactKind) -> bool {
    matches!(
        artifact_kind,
        WindowsAdapterArtifactKind::ManagedBrowserExactUrlEvidence
            | WindowsAdapterArtifactKind::AuditCustodyEvent
    )
}

pub(crate) fn app_ingestion_records() -> Vec<WindowsAdapterArtifactIngestionRecord> {
    vec![
        ingestion_record(
            artifact_ingestion::RECORD_ID_APP_IDENTITY,
            artifact_gate::TEST_ARTIFACT_APP_IDENTITY,
            WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
            WindowsAdapterCapabilitySurface::AppTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_APP_APPLY,
            artifact_gate::TEST_ARTIFACT_APP_APPLY,
            WindowsAdapterArtifactKind::AdapterApplyResult,
            WindowsAdapterCapabilitySurface::AppTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_APP_ROLLBACK,
            artifact_gate::TEST_ARTIFACT_APP_ROLLBACK,
            WindowsAdapterArtifactKind::AdapterRollbackResult,
            WindowsAdapterCapabilitySurface::AppTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_APP_AUDIT,
            artifact_gate::TEST_ARTIFACT_AUDIT,
            WindowsAdapterArtifactKind::AuditCustodyEvent,
            WindowsAdapterCapabilitySurface::AppTarget,
        ),
    ]
}

pub(crate) fn domain_ingestion_records() -> Vec<WindowsAdapterArtifactIngestionRecord> {
    vec![
        ingestion_record(
            artifact_ingestion::RECORD_ID_DOMAIN_APPLY,
            artifact_gate::TEST_ARTIFACT_DOMAIN_APPLY,
            WindowsAdapterArtifactKind::NetworkDomainFilterApply,
            WindowsAdapterCapabilitySurface::DomainNetworkTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_DOMAIN_ROLLBACK,
            artifact_gate::TEST_ARTIFACT_DOMAIN_ROLLBACK,
            WindowsAdapterArtifactKind::NetworkDomainFilterRollback,
            WindowsAdapterCapabilitySurface::DomainNetworkTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_DOMAIN_AUDIT,
            artifact_gate::TEST_ARTIFACT_DOMAIN_AUDIT,
            WindowsAdapterArtifactKind::AuditCustodyEvent,
            WindowsAdapterCapabilitySurface::DomainNetworkTarget,
        ),
    ]
}

pub(crate) fn managed_browser_ingestion_records() -> Vec<WindowsAdapterArtifactIngestionRecord> {
    vec![
        ingestion_record(
            artifact_ingestion::RECORD_ID_MANAGED_BROWSER_URL,
            artifact_gate::TEST_ARTIFACT_MANAGED_EXACT_URL,
            WindowsAdapterArtifactKind::ManagedBrowserExactUrlEvidence,
            WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
        ),
        ingestion_record(
            artifact_ingestion::RECORD_ID_MANAGED_BROWSER_AUDIT,
            artifact_gate::TEST_ARTIFACT_MANAGED_AUDIT,
            WindowsAdapterArtifactKind::AuditCustodyEvent,
            WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
        ),
    ]
}

fn ingestion_record(
    ingestion_record_id: impl std::fmt::Display,
    artifact_id: impl std::fmt::Display,
    artifact_kind: WindowsAdapterArtifactKind,
    surface: WindowsAdapterCapabilitySurface,
) -> WindowsAdapterArtifactIngestionRecord {
    WindowsAdapterArtifactIngestionRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: ingestion_record_id.to_string(),
        artifact_id: artifact_id.to_string(),
        artifact_kind,
        surface,
        target_subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        artifact_subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        custody_event_id: Some(artifact_gate::TEST_CUSTODY_EVENT_ID.to_string()),
        collected_at: artifact_ingestion::TEST_INGESTED_AT.to_string(),
    }
}
