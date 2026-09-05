use ocentra_parent_agent_protocol::ActivityEvidenceRef;

use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;

use super::super::AppGameRiskEvidenceState;

pub(super) fn parse_evidence(
    values: &[ActivityEvidenceRef],
) -> (Vec<AppGamePolicyEvidenceRef>, Vec<String>) {
    let mut parsed = Vec::new();
    let mut invalid = Vec::new();
    for evidence in values {
        match AppGamePolicyEvidenceRef::parse(evidence.evidence_id.clone()) {
            Ok(reference) => parsed.push(reference),
            Err(_) => invalid.push(evidence.evidence_id.clone()),
        }
    }
    (parsed, invalid)
}

pub(super) fn invalid_evidence_refs(values: &[ActivityEvidenceRef]) -> Vec<String> {
    parse_evidence(values).1
}

pub(super) fn evidence_state(
    has_category_proof: bool,
    has_supporting_evidence: bool,
    has_invalid_evidence: bool,
) -> AppGameRiskEvidenceState {
    if has_invalid_evidence {
        AppGameRiskEvidenceState::Invalid
    } else if has_category_proof && has_supporting_evidence {
        AppGameRiskEvidenceState::Complete
    } else {
        AppGameRiskEvidenceState::Missing
    }
}

pub(super) fn unknown_evidence_state(
    row: &ocentra_parent_agent_protocol::app_game::AppGameInventoryEvidenceRow,
) -> AppGameRiskEvidenceState {
    let (_, invalid) = parse_evidence(&row.evidence);
    if !invalid.is_empty() {
        AppGameRiskEvidenceState::Invalid
    } else {
        AppGameRiskEvidenceState::Missing
    }
}
