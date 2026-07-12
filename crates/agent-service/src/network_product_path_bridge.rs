use ocentra_network_evidence::{
    action_result::NetworkActionResultState,
    pipeline::{prove_network_end_to_end_pipeline, NetworkEndToEndPipelineProof},
};
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;

#[path = "network_product_path_bridge/input.rs"]
mod input;

use self::input::product_path_input_from_row;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NetworkProductPathServiceProofReport {
    pub(crate) observed_rows: usize,
    pub(crate) proved_rows: usize,
    pub(crate) skipped_rows: usize,
    pub(crate) failed_rows: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) unavailable_rows: usize,
    pub(crate) policy_decision_count: usize,
    pub(crate) action_result_count: usize,
    pub(crate) retention_record_count: usize,
    pub(crate) delete_record_count: usize,
    pub(crate) export_record_count: usize,
    pub(crate) portal_read_model_count: usize,
    pub(crate) enforcement_command_events: usize,
    pub(crate) adapter_action_executed_count: usize,
    pub(crate) ai_advisory_rows: usize,
    pub(crate) weak_or_unavailable_blocked_rows: usize,
    pub(crate) analyzer_alert_refs: Vec<String>,
    pub(crate) ai_detection_refs: Vec<String>,
    pub(crate) risk_budget_refs: Vec<String>,
    pub(crate) policy_decision_refs: Vec<String>,
    pub(crate) action_result_refs: Vec<String>,
    pub(crate) retention_refs: Vec<String>,
    pub(crate) deletion_refs: Vec<String>,
    pub(crate) export_refs: Vec<String>,
    pub(crate) portal_read_model_refs: Vec<String>,
}

pub(crate) fn prove_network_product_path_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkProductPathServiceProofReport {
    let mut report = NetworkProductPathServiceProofReport {
        observed_rows: read_model.rows.len(),
        ..NetworkProductPathServiceProofReport::default()
    };

    for row in &read_model.rows {
        let Some(input) = product_path_input_from_row(row) else {
            report.skipped_rows += 1;
            continue;
        };
        match prove_network_end_to_end_pipeline(input) {
            Ok(proof) => report.record_success(&proof),
            Err(_) => report.failed_rows += 1,
        }
    }

    report
}

impl NetworkProductPathServiceProofReport {
    fn record_success(&mut self, proof: &NetworkEndToEndPipelineProof) {
        let push_unique = |values: &mut Vec<String>, value: String| {
            if !values.contains(&value) {
                values.push(value);
            }
        };

        self.proved_rows += 1;
        if proof.action_result.result_state == NetworkActionResultState::ManualRequired {
            self.manual_required_rows += 1;
        }
        if proof.action_result.result_state == NetworkActionResultState::Unavailable {
            self.unavailable_rows += 1;
        }
        self.policy_decision_count += 1;
        self.action_result_count += 1;
        self.retention_record_count += 1;
        self.delete_record_count += 1;
        self.export_record_count += 1;
        self.portal_read_model_count += 1;
        self.enforcement_command_events += proof.enforcement_commands_published;
        self.adapter_action_executed_count += usize::from(proof.adapter_action_executed);
        self.ai_advisory_rows += usize::from(proof.ai_advisory_only);
        self.weak_or_unavailable_blocked_rows +=
            usize::from(proof.weak_or_unavailable_evidence_enforcement_blocked);
        for result in &proof.ai_detection.results {
            push_unique(&mut self.ai_detection_refs, result.detection_ref.clone());
            for analyzer_alert_ref in &result.analyzer_alert_refs {
                push_unique(&mut self.analyzer_alert_refs, analyzer_alert_ref.clone());
            }
        }
        push_unique(
            &mut self.risk_budget_refs,
            proof.risk_budget.risk_budget_ref.clone(),
        );
        push_unique(
            &mut self.policy_decision_refs,
            proof.policy_mapping.policy_decision_ref.clone(),
        );
        push_unique(
            &mut self.action_result_refs,
            proof.action_result.action_result_ref.clone(),
        );
        push_unique(
            &mut self.retention_refs,
            proof.retention_delete_export.retention_ref.clone(),
        );
        push_unique(
            &mut self.deletion_refs,
            proof.retention_delete_export.deletion_ref.clone(),
        );
        push_unique(
            &mut self.export_refs,
            proof.retention_delete_export.export_ref.clone(),
        );
        push_unique(
            &mut self.portal_read_model_refs,
            proof.retention_delete_export.portal_read_model_ref.clone(),
        );
    }
}
