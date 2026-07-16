use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_boundary_read_model::AppGameBoundaryReadModelRow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BoundaryKindText(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EvidenceIdText(pub(super) String);

pub(super) fn push_boundary_row(
    rows: &mut Vec<AppGameBoundaryReadModelRow>,
    boundary_kind: BoundaryKindText,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
) {
    if row_count == 0 {
        return;
    }
    rows.push(AppGameBoundaryReadModelRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: boundary_kind.0.to_string(),
        boundary_kind: boundary_kind.0.to_string(),
        row_count,
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    });
}

pub(super) fn push_local_db_row_evidence(
    target: &mut Vec<ActivityEvidenceRef>,
    evidence_id: EvidenceIdText,
) {
    if evidence_id.0.is_empty() {
        return;
    }
    push_evidence(
        target,
        vec![ActivityEvidenceRef {
            evidence_id: evidence_id.0,
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}

pub(super) fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
    for evidence in rows {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence);
    }
}
