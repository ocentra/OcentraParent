use ocentra_parent_agent_core::browser_event_runtime::{
    publish_browser_runtime_chain_for_input, BrowserRuntimeInput, BrowserRuntimeReport,
};
use ocentra_parent_agent_protocol::{
    constants, BrowserCapabilityStatus, BrowserEvidenceReadModel, BrowserQueryVisibilityLabel,
    BrowserRuntimeEventPayload, BrowserRuntimePhase, BrowserTabEvidence, PolicyPreviewReadModel,
    PolicyPreviewReadModelRow,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct BrowserRuntimeServiceDeliveryReport {
    pub(crate) observed_rows: usize,
    pub(crate) delivered_rows: usize,
    pub(crate) failed_rows: usize,
    pub(crate) publish_reports: usize,
    pub(crate) stored_events: usize,
    pub(crate) dead_letters: usize,
    pub(crate) exact_url_rows: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) intervention_command_events: usize,
    pub(crate) read_model_projection_events: usize,
}

pub(crate) async fn deliver_browser_runtime_for_read_model(
    read_model: &BrowserEvidenceReadModel,
) -> BrowserRuntimeServiceDeliveryReport {
    let mut delivery = BrowserRuntimeServiceDeliveryReport {
        observed_rows: read_model.rows.len(),
        ..BrowserRuntimeServiceDeliveryReport::default()
    };

    for row in &read_model.rows {
        let input = browser_runtime_input_from_row(read_model, row);
        match publish_browser_runtime_chain_for_input(input).await {
            Ok(report) => delivery.record_success(row, &report),
            Err(_) => delivery.failed_rows += 1,
        }
    }

    delivery
}

pub(crate) fn browser_runtime_input_from_row(
    read_model: &BrowserEvidenceReadModel,
    row: &BrowserTabEvidence,
) -> BrowserRuntimeInput {
    browser_runtime_input_from_row_with_policy_preview(read_model, row, None)
}

pub(crate) fn browser_runtime_input_from_row_with_policy_preview(
    read_model: &BrowserEvidenceReadModel,
    row: &BrowserTabEvidence,
    policy_preview: Option<&PolicyPreviewReadModel>,
) -> BrowserRuntimeInput {
    let latest_event_ref = read_model.latest_event_id.clone();
    let matched_preview =
        policy_preview.and_then(|model| matching_policy_preview(read_model, row, model));
    let mut input = BrowserRuntimeInput {
        source_ref: row.source_id.clone(),
        evidence_ref: row.browser_evidence_id.clone(),
        capability_status: row.capability_status.as_protocol_str().to_string(),
        custody_label: row.custody_label.as_protocol_str().to_string(),
        query_visibility: row.query_visibility.as_protocol_str().to_string(),
        degraded_reason: row.degraded_reason.clone(),
        journal_ref: latest_event_ref.clone(),
        ai_request_ref: None,
        ai_analysis_ref: None,
        policy_evaluation_ref: None,
        policy_decision_ref: None,
        policy_preview_id: None,
        action_intent_id: None,
        intervention_command_ref: None,
        intervention_result_ref: None,
        audit_entry_ref: latest_event_ref.clone(),
        read_model_ref: read_model_ref(read_model, row),
        observed_at: row.observed_at.clone(),
        exact_url_claimed: row_has_exact_url_boundary(row),
        ai_authority: false,
        policy_authority: false,
        dry_run: false,
        adapter_dispatch_claimed: false,
        intervention_command_allowed: false,
    };

    if let Some(preview) = matched_preview {
        input.policy_evaluation_ref = Some(preview.source_event_id.clone());
        input.policy_decision_ref = Some(preview.decision.decision_id.clone());
        input.policy_preview_id = Some(preview.preview_id.clone());
        input.action_intent_id = Some(action_intent_id_from_policy_decision(preview));
        input.policy_authority = true;
        input.dry_run = preview.decision.dry_run;
    }

    input
}

impl BrowserRuntimeServiceDeliveryReport {
    fn record_success(&mut self, row: &BrowserTabEvidence, report: &BrowserRuntimeReport) {
        self.delivered_rows += 1;
        self.publish_reports += report.publish_reports.len();
        self.stored_events += report.stored_events.len();
        self.dead_letters += report.dead_letters.len();
        if row_has_exact_url_boundary(row) {
            self.exact_url_rows += 1;
        } else {
            self.manual_required_rows += 1;
        }
        self.intervention_command_events +=
            count_phase(report, BrowserRuntimePhase::InterventionCommandIssued);
        self.read_model_projection_events +=
            count_phase(report, BrowserRuntimePhase::ReadModelProjected);
    }
}

fn read_model_ref(
    read_model: &BrowserEvidenceReadModel,
    row: &BrowserTabEvidence,
) -> Option<String> {
    read_model
        .latest_event_id
        .clone()
        .or_else(|| Some(row.browser_evidence_id.clone()))
}

fn row_has_exact_url_boundary(row: &BrowserTabEvidence) -> bool {
    row.managed_browser_session_id
        .starts_with(constants::browser::SESSION_ID_PREFIX_MANAGED)
        && row.query_visibility == BrowserQueryVisibilityLabel::LiveLocal
        && matches!(
            row.capability_status,
            BrowserCapabilityStatus::Available | BrowserCapabilityStatus::TabListOnly
        )
}

fn matching_policy_preview<'a>(
    read_model: &BrowserEvidenceReadModel,
    row: &BrowserTabEvidence,
    policy_preview: &'a PolicyPreviewReadModel,
) -> Option<&'a PolicyPreviewReadModelRow> {
    policy_preview
        .rows
        .iter()
        .find(|preview| policy_preview_references_browser_row(read_model, row, preview))
}

fn policy_preview_references_browser_row(
    read_model: &BrowserEvidenceReadModel,
    row: &BrowserTabEvidence,
    preview: &PolicyPreviewReadModelRow,
) -> bool {
    preview.evidence_references.iter().any(|reference| {
        reference.evidence_reference_id == row.browser_evidence_id
            || read_model
                .latest_event_id
                .as_ref()
                .is_some_and(|event_id| reference.evidence_reference_id == *event_id)
    })
}

fn action_intent_id_from_policy_decision(preview: &PolicyPreviewReadModelRow) -> String {
    let mut value = String::from(constants::browser::ACTION_INTENT_ID_PREFIX);
    value.push_str(&preview.decision.decision_id);
    value
}

fn count_phase(report: &BrowserRuntimeReport, phase: BrowserRuntimePhase) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| {
            event
                .decode::<BrowserRuntimeEventPayload>()
                .map(|envelope| envelope.payload.phase == phase)
                .unwrap_or(false)
        })
        .count()
}
