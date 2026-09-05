use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use super::{ScreenAiAnalysisEventRecord, ScreenAnalysisFieldEntry};

pub(super) struct ScreenAiServicePolicyRefs {
    pub(super) policy_decision_ref: Option<String>,
    pub(super) policy_action: Option<String>,
    pub(super) policy_reason_codes: Vec<String>,
    pub(super) parent_rule_refs: Vec<String>,
    pub(super) parent_explanation_refs: Vec<String>,
    pub(super) explanation_reasons: Vec<String>,
    pub(super) deletion_reasons: Vec<String>,
}

pub(super) fn service_policy_refs() -> ScreenAiServicePolicyRefs {
    // AI output is a candidate only. This runtime has no trusted policy resolver,
    // so it must not mint decision, action, parent-rule, or explanation authority.
    // The caller remains on the non-enforcing/degraded path until policy-control
    // supplies a verified decision and its retained references.
    empty_policy_refs()
}

pub(super) fn screen_analysis_policy_fields(
    record: &ScreenAiAnalysisEventRecord,
) -> Vec<ScreenAnalysisFieldEntry> {
    let join_values = |values: &[String]| values.join(&constants::delimiter::LIST.to_string());
    let mut fields = Vec::new();
    if let Some(value) = &record.policy_decision_ref {
        fields.push(ScreenAnalysisFieldEntry {
            key: constants::field::POLICY_DECISION_ID,
            value: LogFieldValue::String(value.clone()),
        });
    }
    if let Some(value) = &record.policy_action {
        fields.push(ScreenAnalysisFieldEntry {
            key: constants::field::POLICY_ACTION,
            value: LogFieldValue::String(value.clone()),
        });
    }
    for (key, values) in [
        (
            constants::field::POLICY_REASON_CODES,
            &record.policy_reason_codes,
        ),
        (constants::field::POLICY_RULE_IDS, &record.parent_rule_refs),
        (
            constants::field::SCREEN_PARENT_EXPLANATION_REFS,
            &record.parent_explanation_refs,
        ),
        (
            constants::field::SCREEN_EXPLANATION_REASONS,
            &record.explanation_reasons,
        ),
        (
            constants::field::SCREEN_DELETION_REASONS,
            &record.deletion_reasons,
        ),
    ] {
        if !values.is_empty() {
            fields.push(ScreenAnalysisFieldEntry {
                key,
                value: LogFieldValue::String(join_values(values)),
            });
        }
    }
    fields
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
