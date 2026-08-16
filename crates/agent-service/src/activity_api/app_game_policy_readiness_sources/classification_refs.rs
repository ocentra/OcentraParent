use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
};

use super::{
    push_evidence, push_local_db_row_evidence, ClassificationStateRef, LocalDbRowEvidenceIdRef,
};

pub(super) fn ai_classifier_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(
            &mut evidence,
            LocalDbRowEvidenceIdRef(&row.classifier_run_id),
        );
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(evidence_ref));
        }
    }
    evidence
}

pub(super) fn category_candidate_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.inventory_rows {
        for candidate in &row.category_candidates {
            push_evidence(&mut evidence, candidate.evidence.clone());
            if let Some(catalog_ref) = &candidate.catalog_ref {
                push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(catalog_ref));
            }
        }
    }
    evidence
}

pub(super) fn category_candidate_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .inventory_rows
        .iter()
        .map(|row| row.category_candidates.len() as u64)
        .sum()
}

pub(super) fn unknown_review_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    push_unknown_review_rows(
        &mut evidence,
        model.inventory_rows.iter().map(|row| {
            (
                ClassificationStateRef(&row.classification_state),
                row.evidence.clone(),
                LocalDbRowEvidenceIdRef(&row.inventory_entry_id),
            )
        }),
    );
    push_unknown_review_rows(
        &mut evidence,
        model.running_now_rows.iter().map(|row| {
            (
                ClassificationStateRef(&row.classification_state),
                row.evidence.clone(),
                LocalDbRowEvidenceIdRef(&row.runtime_evidence_id),
            )
        }),
    );
    push_unknown_review_rows(
        &mut evidence,
        model.foreground_now_rows.iter().map(|row| {
            (
                ClassificationStateRef(&row.classification_state),
                row.evidence.clone(),
                LocalDbRowEvidenceIdRef(&row.foreground_evidence_id),
            )
        }),
    );
    push_unknown_review_rows(
        &mut evidence,
        model.launcher_rows.iter().map(|row| {
            (
                ClassificationStateRef(&row.classification_state),
                row.evidence.clone(),
                LocalDbRowEvidenceIdRef(&row.launcher_evidence_id),
            )
        }),
    );
    evidence
}

pub(super) fn unknown_review_row_count(model: &AppGameServiceReadModel) -> u64 {
    let inventory = count_unknown_review_rows(
        model
            .inventory_rows
            .iter()
            .map(|row| ClassificationStateRef(&row.classification_state)),
    );
    let runtime = count_unknown_review_rows(
        model
            .running_now_rows
            .iter()
            .map(|row| ClassificationStateRef(&row.classification_state)),
    );
    let foreground = count_unknown_review_rows(
        model
            .foreground_now_rows
            .iter()
            .map(|row| ClassificationStateRef(&row.classification_state)),
    );
    let launcher = count_unknown_review_rows(
        model
            .launcher_rows
            .iter()
            .map(|row| ClassificationStateRef(&row.classification_state)),
    );
    inventory + runtime + foreground + launcher
}

fn count_unknown_review_rows<'a>(states: impl Iterator<Item = ClassificationStateRef<'a>>) -> u64 {
    states
        .filter(|state| is_unknown_review_classification(*state))
        .count() as u64
}

fn push_unknown_review_rows<'a>(
    target: &mut Vec<ActivityEvidenceRef>,
    rows: impl Iterator<
        Item = (
            ClassificationStateRef<'a>,
            Vec<ActivityEvidenceRef>,
            LocalDbRowEvidenceIdRef<'a>,
        ),
    >,
) {
    for (classification_state, evidence, evidence_id) in rows {
        if is_unknown_review_classification(classification_state) {
            push_evidence(target, evidence);
            push_local_db_row_evidence(target, evidence_id);
        }
    }
}

fn is_unknown_review_classification(classification_state: ClassificationStateRef<'_>) -> bool {
    classification_state.0 == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
        || classification_state.0 == APP_GAME_CLASSIFICATION_POSSIBLY_GAME
        || classification_state.0 == APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
}
