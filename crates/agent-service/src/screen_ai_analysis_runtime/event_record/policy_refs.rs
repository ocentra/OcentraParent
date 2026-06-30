use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use super::ScreenAiAnalysisEventRecord;

pub(super) struct ScreenAiServicePolicyRefs {
    pub(super) policy_decision_ref: Option<String>,
    pub(super) policy_action: Option<String>,
    pub(super) policy_reason_codes: Vec<String>,
    pub(super) parent_rule_refs: Vec<String>,
    pub(super) parent_explanation_refs: Vec<String>,
    pub(super) explanation_reasons: Vec<String>,
    pub(super) deletion_reasons: Vec<String>,
}

pub(super) fn service_policy_refs(
    queue_job_id: &str,
    policy_eligible: bool,
) -> ScreenAiServicePolicyRefs {
    if !policy_eligible {
        return empty_policy_refs();
    }
    ScreenAiServicePolicyRefs {
        policy_decision_ref: Some(prefixed_id(
            constants::screen_flow::SCREEN_SERVICE_POLICY_DECISION_ID_PREFIX,
            queue_job_id,
        )),
        policy_action: Some(constants::screen_flow::SCREEN_SERVICE_POLICY_ACTION_ALLOW.to_string()),
        policy_reason_codes: vec![
            constants::screen_flow::SCREEN_SERVICE_POLICY_REASON_CODE.to_string()
        ],
        parent_rule_refs: vec![constants::screen_flow::SCREEN_SERVICE_PARENT_RULE_REF.to_string()],
        parent_explanation_refs: vec![prefixed_id(
            constants::screen_flow::SCREEN_SERVICE_PARENT_EXPLANATION_REF_PREFIX,
            queue_job_id,
        )],
        explanation_reasons: vec![
            constants::screen_flow::SCREEN_SERVICE_EXPLANATION_REASON.to_string()
        ],
        deletion_reasons: vec![constants::screen_flow::SCREEN_SERVICE_DELETION_REASON.to_string()],
    }
}

pub(super) fn screen_analysis_policy_fields(
    record: &ScreenAiAnalysisEventRecord,
) -> Vec<(&'static str, LogFieldValue)> {
    [
        optional_string_field(
            constants::field::POLICY_DECISION_ID,
            record.policy_decision_ref.clone(),
        ),
        optional_string_field(
            constants::field::POLICY_ACTION,
            record.policy_action.clone(),
        ),
        optional_string_list_field(
            constants::field::POLICY_REASON_CODES,
            &record.policy_reason_codes,
        ),
        optional_string_list_field(constants::field::POLICY_RULE_IDS, &record.parent_rule_refs),
        optional_string_list_field(
            constants::field::SCREEN_PARENT_EXPLANATION_REFS,
            &record.parent_explanation_refs,
        ),
        optional_string_list_field(
            constants::field::SCREEN_EXPLANATION_REASONS,
            &record.explanation_reasons,
        ),
        optional_string_list_field(
            constants::field::SCREEN_DELETION_REASONS,
            &record.deletion_reasons,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn empty_policy_refs() -> ScreenAiServicePolicyRefs {
    ScreenAiServicePolicyRefs {
        policy_decision_ref: None,
        policy_action: None,
        policy_reason_codes: Vec::new(),
        parent_rule_refs: Vec::new(),
        parent_explanation_refs: Vec::new(),
        explanation_reasons: Vec::new(),
        deletion_reasons: Vec::new(),
    }
}

fn optional_string_field(
    key: &'static str,
    value: Option<String>,
) -> Option<(&'static str, LogFieldValue)> {
    value.map(|value| string_field(key, value))
}

fn optional_string_list_field(
    key: &'static str,
    values: &[String],
) -> Option<(&'static str, LogFieldValue)> {
    if values.is_empty() {
        None
    } else {
        Some(string_field(
            key,
            values.join(&constants::delimiter::LIST.to_string()),
        ))
    }
}

fn string_field(key: &'static str, value: impl Into<String>) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.into()))
}

fn prefixed_id(prefix: &str, value: &str) -> String {
    let mut id = String::from(prefix);
    id.push_str(value);
    id
}
