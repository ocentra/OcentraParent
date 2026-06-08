use ocentra_network_evidence::{
    prove_network_end_to_end_pipeline, NetworkActionResultState, NetworkCascadeSignalStrength,
    NetworkCascadeSourceKind, NetworkCrossSliceEvidenceSource, NetworkDnsAdapterCapabilityState,
    NetworkEndToEndPipelineInput, NetworkEndToEndPipelineProof, NetworkEndToEndPipelineRefs,
    NetworkEndToEndUnsupportedClaims, NetworkEvidenceGrade, NetworkEvidencePolicyAction,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
};

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

fn product_path_input_from_row(
    row: &ActivityNetworkFlowObservation,
) -> Option<NetworkEndToEndPipelineInput> {
    let target_domain = row.destination_domain.clone()?;
    let evidence_ref = evidence_ref_for_row(row);
    let available =
        row.capability_status == constants::activity_capture::CAPABILITY_STATUS_AVAILABLE;

    Some(NetworkEndToEndPipelineInput {
        refs: product_path_refs(row, target_domain),
        sources: vec![NetworkCrossSliceEvidenceSource {
            source_kind: source_kind_for_row(row),
            signal_strength: signal_strength_for_row(row, available),
            evidence_grade: evidence_grade_for_row(available),
            evidence_ref,
            exact_url_available: false,
            decrypted_payload_available: false,
            policy_action_authority: false,
            adapter_action_authority: false,
        }],
        requested_policy_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_state: adapter_capability_state(available),
        adapter_dry_run: false,
        local_ai_enabled: available,
        model_runtime_available: available,
        queue_available: available,
        unsupported_claims: NetworkEndToEndUnsupportedClaims {
            raw_network_payload_claimed: false,
            decrypted_payload_claimed: false,
            page_content_claimed: false,
            exact_url_claimed: false,
            ai_policy_authority_claimed: false,
            ui_policy_authority_claimed: false,
            network_adapter_authority_claimed: false,
            enforcement_command_claimed: false,
        },
    })
}

fn product_path_refs(
    row: &ActivityNetworkFlowObservation,
    target_domain: String,
) -> NetworkEndToEndPipelineRefs {
    let phase = phase_refs(row);
    let ai = ai_refs(row);
    let policy = policy_refs(row);
    let adapter = adapter_refs(row);

    NetworkEndToEndPipelineRefs {
        trigger_ref: phase.trigger_ref,
        capture_ref: phase.capture_ref,
        ingest_ref: phase.ingest_ref,
        typed_event_ref: phase.typed_event_ref,
        summary_refs: vec![phase.summary_ref],
        analyzer_alert_refs: vec![phase.analyzer_alert_ref],
        queue_job_ref: ai.queue_job_ref,
        queue_ref: constants::network_flow::PRODUCT_PATH_QUEUE_REF.to_owned(),
        model_runtime_ref: constants::network_flow::PRODUCT_PATH_MODEL_RUNTIME_REF.to_owned(),
        ai_detection_ref: ai.ai_detection_ref,
        ai_fixture_ref: ai.ai_fixture_ref,
        ai_evaluation_run_ref: ai.ai_evaluation_run_ref,
        ai_fixture_set_ref: constants::network_flow::PRODUCT_PATH_AI_FIXTURE_SET_REF.to_owned(),
        ai_model_card_ref: constants::network_flow::PRODUCT_PATH_AI_MODEL_CARD_REF.to_owned(),
        ai_model_version_ref: constants::network_flow::PRODUCT_PATH_AI_MODEL_VERSION_REF.to_owned(),
        ai_baseline_ref: constants::network_flow::PRODUCT_PATH_AI_BASELINE_REF.to_owned(),
        ai_audit_report_ref: ai.ai_audit_report_ref,
        ai_narrative_template_ref: constants::network_flow::PRODUCT_PATH_NARRATIVE_TEMPLATE_REF
            .to_owned(),
        policy_context_ref: policy.policy_context_ref,
        policy_decision_ref: policy.policy_decision_ref,
        parent_rule_ref: constants::network_flow::PRODUCT_PATH_PARENT_RULE_REF.to_owned(),
        risk_evaluation_ref: policy.risk_evaluation_ref,
        child_profile_ref: constants::network_flow::PRODUCT_PATH_CHILD_PROFILE_REF.to_owned(),
        risk_budget_ref: constants::network_flow::PRODUCT_PATH_RISK_BUDGET_REF.to_owned(),
        cascade_ref: policy.cascade_ref,
        household_policy_ref: constants::network_flow::PRODUCT_PATH_HOUSEHOLD_POLICY_REF.to_owned(),
        dns_adapter_plan_ref: adapter.dns_adapter_plan_ref,
        action_result_ref: adapter.action_result_ref,
        target_domain,
        adapter_authorization_ref: Some(adapter.adapter_authorization_ref),
        adapter_capability_proof_ref: Some(adapter.adapter_capability_proof_ref),
        apply_artifact_ref: Some(adapter.apply_artifact_ref),
        result_artifact_ref: Some(adapter.result_artifact_ref),
        rollback_artifact_ref: Some(adapter.rollback_artifact_ref),
        audit_event_ref: adapter.audit_event_ref,
        portal_read_model_ref: adapter.portal_read_model_ref,
        retention_ref: adapter.retention_ref,
        deletion_ref: adapter.deletion_ref,
        export_ref: adapter.export_ref,
        tombstone_ref: adapter.tombstone_ref,
    }
}

struct ProductPathPhaseRefs {
    trigger_ref: String,
    capture_ref: String,
    ingest_ref: String,
    typed_event_ref: String,
    summary_ref: String,
    analyzer_alert_ref: String,
}

struct ProductPathAiRefs {
    queue_job_ref: String,
    ai_detection_ref: String,
    ai_fixture_ref: String,
    ai_evaluation_run_ref: String,
    ai_audit_report_ref: String,
}

struct ProductPathPolicyRefs {
    policy_context_ref: String,
    policy_decision_ref: String,
    risk_evaluation_ref: String,
    cascade_ref: String,
}

struct ProductPathAdapterRefs {
    dns_adapter_plan_ref: String,
    action_result_ref: String,
    adapter_authorization_ref: String,
    adapter_capability_proof_ref: String,
    apply_artifact_ref: String,
    result_artifact_ref: String,
    rollback_artifact_ref: String,
    audit_event_ref: String,
    portal_read_model_ref: String,
    retention_ref: String,
    deletion_ref: String,
    export_ref: String,
    tombstone_ref: String,
}

fn phase_refs(row: &ActivityNetworkFlowObservation) -> ProductPathPhaseRefs {
    ProductPathPhaseRefs {
        trigger_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_TRIGGER_REF_PREFIX,
            row,
        ),
        capture_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_CAPTURE_REF_PREFIX,
            row,
        ),
        ingest_ref: row_ref(constants::network_flow::PRODUCT_PATH_INGEST_REF_PREFIX, row),
        typed_event_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_TYPED_EVENT_REF_PREFIX,
            row,
        ),
        summary_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_SUMMARY_REF_PREFIX,
            row,
        ),
        analyzer_alert_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX,
            row,
        ),
    }
}

fn ai_refs(row: &ActivityNetworkFlowObservation) -> ProductPathAiRefs {
    ProductPathAiRefs {
        queue_job_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_QUEUE_JOB_REF_PREFIX,
            row,
        ),
        ai_detection_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX,
            row,
        ),
        ai_fixture_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_AI_FIXTURE_REF_PREFIX,
            row,
        ),
        ai_evaluation_run_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_AI_EVALUATION_RUN_REF_PREFIX,
            row,
        ),
        ai_audit_report_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_AI_AUDIT_REF_PREFIX,
            row,
        ),
    }
}

fn policy_refs(row: &ActivityNetworkFlowObservation) -> ProductPathPolicyRefs {
    ProductPathPolicyRefs {
        policy_context_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_POLICY_CONTEXT_REF_PREFIX,
            row,
        ),
        policy_decision_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_POLICY_DECISION_REF_PREFIX,
            row,
        ),
        risk_evaluation_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_RISK_EVALUATION_REF_PREFIX,
            row,
        ),
        cascade_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_CASCADE_REF_PREFIX,
            row,
        ),
    }
}

fn adapter_refs(row: &ActivityNetworkFlowObservation) -> ProductPathAdapterRefs {
    ProductPathAdapterRefs {
        dns_adapter_plan_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_DNS_ADAPTER_PLAN_REF_PREFIX,
            row,
        ),
        action_result_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ACTION_RESULT_REF_PREFIX,
            row,
        ),
        adapter_authorization_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ADAPTER_AUTHORIZATION_REF_PREFIX,
            row,
        ),
        adapter_capability_proof_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ADAPTER_CAPABILITY_REF_PREFIX,
            row,
        ),
        apply_artifact_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ADAPTER_APPLY_REF_PREFIX,
            row,
        ),
        result_artifact_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ADAPTER_RESULT_REF_PREFIX,
            row,
        ),
        rollback_artifact_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_ROLLBACK_REF_PREFIX,
            row,
        ),
        audit_event_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_AUDIT_EVENT_REF_PREFIX,
            row,
        ),
        portal_read_model_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_PORTAL_READ_MODEL_REF_PREFIX,
            row,
        ),
        retention_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_RETENTION_REF_PREFIX,
            row,
        ),
        deletion_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_DELETION_REF_PREFIX,
            row,
        ),
        export_ref: row_ref(constants::network_flow::PRODUCT_PATH_EXPORT_REF_PREFIX, row),
        tombstone_ref: row_ref(
            constants::network_flow::PRODUCT_PATH_TOMBSTONE_REF_PREFIX,
            row,
        ),
    }
}

fn evidence_ref_for_row(row: &ActivityNetworkFlowObservation) -> String {
    row.evidence
        .first()
        .map(|evidence| evidence.evidence_id.clone())
        .unwrap_or_else(|| row.event_id.clone())
}

fn source_kind_for_row(row: &ActivityNetworkFlowObservation) -> NetworkCascadeSourceKind {
    if row.process_name.is_some() {
        NetworkCascadeSourceKind::ProcessAppCorrelation
    } else {
        NetworkCascadeSourceKind::DomainCategory
    }
}

fn signal_strength_for_row(
    row: &ActivityNetworkFlowObservation,
    available: bool,
) -> NetworkCascadeSignalStrength {
    if !available {
        return NetworkCascadeSignalStrength::Unavailable;
    }
    if row.process_name.is_some() {
        NetworkCascadeSignalStrength::WeakHint
    } else {
        NetworkCascadeSignalStrength::Candidate
    }
}

fn evidence_grade_for_row(available: bool) -> NetworkEvidenceGrade {
    if available {
        NetworkEvidenceGrade::B
    } else {
        NetworkEvidenceGrade::D
    }
}

fn adapter_capability_state(available: bool) -> NetworkDnsAdapterCapabilityState {
    if available {
        NetworkDnsAdapterCapabilityState::Supported
    } else {
        NetworkDnsAdapterCapabilityState::Unavailable
    }
}

fn row_ref(prefix: &str, row: &ActivityNetworkFlowObservation) -> String {
    let mut value = String::from(prefix);
    value.push_str(&evidence_ref_for_row(row));
    value
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}
