use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceKind;
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_memory_graph_rows::MemoryGraphStoreRow;

pub(crate) fn evidence_references(row: &MemoryGraphStoreRow) -> Vec<ParentEvidenceReference> {
    if row.evidence.is_empty() {
        return vec![ParentEvidenceReference {
            evidence_reference_id: row.event_id.clone(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: row.observed_at.clone(),
        }];
    }
    row.evidence
        .iter()
        .map(|evidence| ParentEvidenceReference {
            evidence_reference_id: evidence.evidence_id.clone(),
            kind: parent_evidence_kind(&evidence.kind),
            observed_at: row.observed_at.clone(),
        })
        .collect()
}

pub(crate) fn parent_evidence_kind(kind: &ActivityEvidenceKind) -> ParentEvidenceReferenceKind {
    match kind {
        ActivityEvidenceKind::JournalEntry => ParentEvidenceReferenceKind::JournalEvent,
        ActivityEvidenceKind::Screenshot => ParentEvidenceReferenceKind::ActivityEvent,
        ActivityEvidenceKind::StorageObject => ParentEvidenceReferenceKind::ActivityEvent,
        ActivityEvidenceKind::LocalDbRow => ParentEvidenceReferenceKind::ActivityEvent,
    }
}

pub(crate) fn confidence_for_row(row: &MemoryGraphStoreRow) -> f64 {
    match row.kind.as_str() {
        constants::activity_event_kind::WINDOW_FOCUSED => 0.4,
        _ => 1.0,
    }
}
