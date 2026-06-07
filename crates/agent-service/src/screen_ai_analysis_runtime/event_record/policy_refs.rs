use ocentra_parent_agent_protocol::{constants, LogFieldValue};

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

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::{
        ActivityCaptureCapabilityStatus, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
        SCREEN_CATEGORY_UNKNOWN, SCREEN_PROVIDER_LOCAL_VISION, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
        SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
        SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
    };

    use super::*;

    #[test]
    fn policy_eligible_service_record_carries_bridge_required_policy_refs() {
        let fields = screen_analysis_policy_fields(&record_with_policy_eligibility(true));
        let expected_policy_decision_ref = prefixed_id(
            constants::screen_flow::SCREEN_SERVICE_POLICY_DECISION_ID_PREFIX,
            constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
        );

        assert_eq!(
            string_value(&fields, constants::field::POLICY_DECISION_ID),
            Some(expected_policy_decision_ref.as_str())
        );
        assert_eq!(
            string_value(&fields, constants::field::POLICY_ACTION),
            Some(constants::screen_flow::SCREEN_SERVICE_POLICY_ACTION_ALLOW)
        );
        assert_eq!(
            string_value(&fields, constants::field::POLICY_REASON_CODES),
            Some(constants::screen_flow::SCREEN_SERVICE_POLICY_REASON_CODE)
        );
        assert_eq!(
            string_value(&fields, constants::field::POLICY_RULE_IDS),
            Some(constants::screen_flow::SCREEN_SERVICE_PARENT_RULE_REF)
        );
        assert_eq!(
            string_value(&fields, constants::field::SCREEN_DELETION_REASONS),
            Some(constants::screen_flow::SCREEN_SERVICE_DELETION_REASON)
        );
    }

    #[test]
    fn non_policy_eligible_service_record_does_not_fabricate_policy_refs() {
        let fields = screen_analysis_policy_fields(&record_with_policy_eligibility(false));

        assert_eq!(
            string_value(&fields, constants::field::POLICY_DECISION_ID),
            None
        );
        assert_eq!(string_value(&fields, constants::field::POLICY_ACTION), None);
        assert_eq!(
            string_value(&fields, constants::field::POLICY_REASON_CODES),
            None
        );
        assert_eq!(
            string_value(&fields, constants::field::POLICY_RULE_IDS),
            None
        );
        assert_eq!(
            string_value(&fields, constants::field::SCREEN_DELETION_REASONS),
            None
        );
    }

    fn record_with_policy_eligibility(policy_eligible: bool) -> ScreenAiAnalysisEventRecord {
        let policy = service_policy_refs(
            constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
            policy_eligible,
        );
        ScreenAiAnalysisEventRecord {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
            timestamp: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
            primary_category: SCREEN_CATEGORY_UNKNOWN.to_string(),
            confidence: SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
            policy_eligible,
            provider_kind: SCREEN_PROVIDER_LOCAL_VISION.to_string(),
            model_runtime_ref: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
            model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
            prompt_or_template_version: SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string(),
            capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
            capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
            capability_status: ActivityCaptureCapabilityStatus::Available
                .as_protocol_str()
                .to_string(),
            policy_decision_ref: policy.policy_decision_ref,
            policy_action: policy.policy_action,
            policy_reason_codes: policy.policy_reason_codes,
            parent_rule_refs: policy.parent_rule_refs,
            parent_explanation_refs: policy.parent_explanation_refs,
            explanation_reasons: policy.explanation_reasons,
            deletion_reasons: policy.deletion_reasons,
            ocr_text_snippets: Vec::new(),
            redaction_notes: Vec::new(),
        }
    }

    fn string_value<'a>(fields: &'a [(&'static str, LogFieldValue)], key: &str) -> Option<&'a str> {
        fields
            .iter()
            .find_map(|(field_key, value)| match (*field_key == key, value) {
                (true, LogFieldValue::String(value)) => Some(value.as_str()),
                _ => None,
            })
    }
}
