use ocentra_network_evidence::{
    bundle::NetworkCrossSliceEvidenceSource,
    cascade::{NetworkCascadeSignalStrength, NetworkCascadeSourceKind},
    dns::types::NetworkEvidenceGrade,
    dns_adapter::NetworkDnsAdapterCapabilityState,
    pipeline::{
        NetworkEndToEndPipelineInput, NetworkEndToEndPipelineRefs, NetworkEndToEndUnsupportedClaims,
    },
    policy::NetworkEvidencePolicyAction,
};
use ocentra_parent_agent_protocol::{constants, network_flow::ActivityNetworkFlowObservation};

#[derive(Clone)]
struct ProductPathTargetDomain(String);

#[derive(Clone)]
struct EvidenceRefText(String);

#[derive(Clone, Copy)]
struct EvidenceRefPrefix(&'static str);

struct ProductPathRowRefs<'a> {
    row: &'a ActivityNetworkFlowObservation,
    evidence_ref: EvidenceRefText,
}

impl<'a> ProductPathRowRefs<'a> {
    fn from_row(row: &'a ActivityNetworkFlowObservation) -> Self {
        let evidence_ref = row
            .evidence
            .first()
            .map(|evidence| evidence.evidence_id.clone())
            .unwrap_or_else(|| row.event_id.clone());
        Self {
            row,
            evidence_ref: EvidenceRefText(evidence_ref),
        }
    }

    fn build_ref(&self, prefix: EvidenceRefPrefix) -> EvidenceRefText {
        let mut value = String::from(prefix.0);
        value.push_str(&self.evidence_ref.0);
        EvidenceRefText(value)
    }
}

pub(super) fn product_path_input_from_row(
    row: &ActivityNetworkFlowObservation,
) -> Option<NetworkEndToEndPipelineInput> {
    let refs = ProductPathRowRefs::from_row(row);
    let target_domain = ProductPathTargetDomain(row.destination_domain.clone()?);
    let available =
        row.capability_status == constants::activity_capture::CAPABILITY_STATUS_AVAILABLE;

    Some(NetworkEndToEndPipelineInput {
        refs: product_path_refs(&refs, target_domain),
        sources: vec![NetworkCrossSliceEvidenceSource {
            source_kind: source_kind_for_row(refs.row),
            signal_strength: signal_strength_for_row(refs.row, available),
            evidence_grade: evidence_grade_for_row(available),
            evidence_ref: refs.evidence_ref.0.clone(),
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
    refs: &ProductPathRowRefs<'_>,
    target_domain: ProductPathTargetDomain,
) -> NetworkEndToEndPipelineRefs {
    let phase = phase_refs(refs);
    let ai = ai_refs(refs);
    let policy = policy_refs(refs);
    let adapter = adapter_refs(refs);

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
        target_domain: target_domain.0,
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

fn phase_refs(refs: &ProductPathRowRefs<'_>) -> ProductPathPhaseRefs {
    ProductPathPhaseRefs {
        trigger_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_TRIGGER_REF_PREFIX,
            ))
            .0,
        capture_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_CAPTURE_REF_PREFIX,
            ))
            .0,
        ingest_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_INGEST_REF_PREFIX,
            ))
            .0,
        typed_event_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_TYPED_EVENT_REF_PREFIX,
            ))
            .0,
        summary_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_SUMMARY_REF_PREFIX,
            ))
            .0,
        analyzer_alert_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX,
            ))
            .0,
    }
}

fn ai_refs(refs: &ProductPathRowRefs<'_>) -> ProductPathAiRefs {
    ProductPathAiRefs {
        queue_job_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_QUEUE_JOB_REF_PREFIX,
            ))
            .0,
        ai_detection_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_AI_DETECTION_REF_PREFIX,
            ))
            .0,
        ai_fixture_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_AI_FIXTURE_REF_PREFIX,
            ))
            .0,
        ai_evaluation_run_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_AI_EVALUATION_RUN_REF_PREFIX,
            ))
            .0,
        ai_audit_report_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_AI_AUDIT_REF_PREFIX,
            ))
            .0,
    }
}

fn policy_refs(refs: &ProductPathRowRefs<'_>) -> ProductPathPolicyRefs {
    ProductPathPolicyRefs {
        policy_context_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_POLICY_CONTEXT_REF_PREFIX,
            ))
            .0,
        policy_decision_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_POLICY_DECISION_REF_PREFIX,
            ))
            .0,
        risk_evaluation_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_RISK_EVALUATION_REF_PREFIX,
            ))
            .0,
        cascade_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_CASCADE_REF_PREFIX,
            ))
            .0,
    }
}

fn adapter_refs(refs: &ProductPathRowRefs<'_>) -> ProductPathAdapterRefs {
    ProductPathAdapterRefs {
        dns_adapter_plan_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_DNS_ADAPTER_PLAN_REF_PREFIX,
            ))
            .0,
        action_result_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ACTION_RESULT_REF_PREFIX,
            ))
            .0,
        adapter_authorization_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ADAPTER_AUTHORIZATION_REF_PREFIX,
            ))
            .0,
        adapter_capability_proof_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ADAPTER_CAPABILITY_REF_PREFIX,
            ))
            .0,
        apply_artifact_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ADAPTER_APPLY_REF_PREFIX,
            ))
            .0,
        result_artifact_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ADAPTER_RESULT_REF_PREFIX,
            ))
            .0,
        rollback_artifact_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_ROLLBACK_REF_PREFIX,
            ))
            .0,
        audit_event_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_AUDIT_EVENT_REF_PREFIX,
            ))
            .0,
        portal_read_model_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_PORTAL_READ_MODEL_REF_PREFIX,
            ))
            .0,
        retention_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_RETENTION_REF_PREFIX,
            ))
            .0,
        deletion_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_DELETION_REF_PREFIX,
            ))
            .0,
        export_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_EXPORT_REF_PREFIX,
            ))
            .0,
        tombstone_ref: refs
            .build_ref(EvidenceRefPrefix(
                constants::network_flow::PRODUCT_PATH_TOMBSTONE_REF_PREFIX,
            ))
            .0,
    }
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
