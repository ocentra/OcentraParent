use ocentra_parent_agent_protocol::activity::policy::{
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};

use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

pub(crate) fn evidence_references_from_row(
    row: &PolicyPreviewStoreRow,
) -> Vec<ParentEvidenceReference> {
    let mut references = vec![ParentEvidenceReference {
        evidence_reference_id: row.event_id.clone(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: row.observed_at.clone(),
    }];

    for evidence in &row.evidence {
        if let Some(reference) = evidence_reference_from_activity(evidence, &row.observed_at) {
            push_unique_reference(&mut references, reference);
        }
    }

    references
}

fn evidence_reference_from_activity(
    evidence: &ActivityEvidenceRef,
    observed_at: &str,
) -> Option<ParentEvidenceReference> {
    let kind = evidence_reference_kind(&evidence.kind)?;

    Some(ParentEvidenceReference {
        evidence_reference_id: evidence.evidence_id.clone(),
        kind,
        observed_at: observed_at.to_string(),
    })
}

fn evidence_reference_kind(kind: &ActivityEvidenceKind) -> Option<ParentEvidenceReferenceKind> {
    const RULES: &[(ActivityEvidenceKind, ParentEvidenceReferenceKind)] = &[
        (
            ActivityEvidenceKind::JournalEntry,
            ParentEvidenceReferenceKind::JournalEvent,
        ),
        (
            ActivityEvidenceKind::LocalDbRow,
            ParentEvidenceReferenceKind::QueryStoreSummary,
        ),
    ];

    RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == *kind).then_some(*mapped))
}

fn push_unique_reference(
    references: &mut Vec<ParentEvidenceReference>,
    reference: ParentEvidenceReference,
) {
    if !references
        .iter()
        .any(|existing| existing.evidence_reference_id == reference.evidence_reference_id)
    {
        references.push(reference);
    }
}
