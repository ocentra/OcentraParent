use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;

pub(super) fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = evidence_claim_refs(model);
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

pub(super) fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .evidence_claim_rows
        .iter()
        .flat_map(|row| {
            let mut refs = row.evidence.clone();
            refs.push(ActivityEvidenceRef {
                evidence_id: row.claim_id.clone(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            });
            refs
        })
        .collect()
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .identity_rows
        .iter()
        .flat_map(|row| {
            let mut refs = row.evidence.clone();
            refs.push(ActivityEvidenceRef {
                evidence_id: row.identity_id.clone(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            });
            refs
        })
        .collect()
}

pub(super) fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .approval_authority_rows
        .iter()
        .map(|row| ActivityEvidenceRef {
            evidence_id: row.authority_id.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        })
        .collect()
}

pub(super) fn platform_authority_row_refs(
    model: &AppGameServiceReadModel,
) -> Vec<ActivityEvidenceRef> {
    model
        .platform_authority_matrices
        .iter()
        .flat_map(|matrix| matrix.rows.iter())
        .map(|row| ActivityEvidenceRef {
            evidence_id: row.row_id.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        })
        .collect()
}

pub(super) fn platform_authority_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .platform_authority_matrices
        .iter()
        .map(|matrix| matrix.rows.len() as u64)
        .sum()
}

pub(super) fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
    let mut seen: BTreeSet<String> = target
        .iter()
        .map(|candidate| candidate.evidence_id.clone())
        .collect();
    target.extend(
        rows.into_iter()
            .filter(|evidence| seen.insert(evidence.evidence_id.clone())),
    );
}
