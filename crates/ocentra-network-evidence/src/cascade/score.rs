use super::{NetworkCascadeSignalStrength, NetworkCascadeSource, NetworkCascadeSourceKind};
use crate::dns::types::NetworkEvidenceGrade;

pub(super) fn source_score(source: &NetworkCascadeSource) -> u16 {
    strength_score(source.signal_strength)
        + kind_score(source.source_kind)
        + grade_score(source.evidence_grade)
}

fn strength_score(strength: NetworkCascadeSignalStrength) -> u16 {
    const SCORES: [u16; 4] = [600, 300, 100, 0];
    SCORES[strength as usize]
}

fn kind_score(kind: NetworkCascadeSourceKind) -> u16 {
    const SCORES: [u16; 7] = [70, 60, 50, 40, 30, 20, 10];
    SCORES[kind as usize]
}

fn grade_score(grade: NetworkEvidenceGrade) -> u16 {
    const SCORES: [u16; 4] = [4, 3, 2, 1];
    SCORES[grade as usize]
}
