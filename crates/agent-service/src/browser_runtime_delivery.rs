use ocentra_parent_agent_core::{
    publish_browser_runtime_chain_for_input, BrowserRuntimeInput, BrowserRuntimePhase,
    BrowserRuntimeReport,
};
use ocentra_parent_agent_protocol::{
    constants, BrowserCapabilityStatus, BrowserEvidenceReadModel, BrowserQueryVisibilityLabel,
    BrowserTabEvidence,
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
    let latest_event_ref = read_model.latest_event_id.clone();
    BrowserRuntimeInput {
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
        intervention_command_ref: None,
        intervention_result_ref: None,
        audit_entry_ref: latest_event_ref.clone(),
        read_model_ref: read_model_ref(read_model, row),
        observed_at: row.observed_at.clone(),
        exact_url_claimed: row_has_exact_url_boundary(row),
        ai_authority: false,
        policy_authority: false,
        intervention_command_allowed: false,
    }
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

fn count_phase(report: &BrowserRuntimeReport, phase: BrowserRuntimePhase) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| {
            event
                .decode::<ocentra_parent_agent_core::BrowserRuntimeEventPayload>()
                .map(|envelope| envelope.payload.phase == phase)
                .unwrap_or(false)
        })
        .count()
}
