mod screen_ai_policy_refs_support {
    use std::primitive::str as TestStr;
    use std::string::String as TestString;

    use ocentra_parent_agent_protocol::constants;

    #[derive(Clone, Debug)]
    pub struct ScreenAiAnalysisEventRecord {
        pub queue_job_id: String,
        pub image_digest: String,
        pub timestamp: String,
        pub summary: String,
        pub primary_category: String,
        pub confidence: f64,
        pub policy_eligible: bool,
        pub provider_kind: String,
        pub model_runtime_ref: String,
        pub model_id: String,
        pub prompt_or_template_version: String,
        pub capture_reason: String,
        pub capture_scope: String,
        pub capability_status: String,
        pub policy_decision_ref: Option<String>,
        pub policy_action: Option<String>,
        pub policy_reason_codes: Vec<String>,
        pub parent_rule_refs: Vec<String>,
        pub parent_explanation_refs: Vec<String>,
        pub explanation_reasons: Vec<String>,
        pub deletion_reasons: Vec<String>,
        pub ocr_text_snippets: Vec<String>,
        pub redaction_notes: Vec<String>,
    }

    #[path = "../../../src/screen_ai_analysis_runtime/event_record/policy_refs.rs"]
    mod policy_refs;

    #[test]
    fn policy_eligible_service_record_carries_bridge_required_policy_refs() {
        let record = record_with_policy_eligibility(true);
        assert_record_fields(&record, true);
        let fields = policy_refs::screen_analysis_policy_fields(&record);
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
        let record = record_with_policy_eligibility(false);
        assert_record_fields(&record, false);
        let fields = policy_refs::screen_analysis_policy_fields(&record);

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
        use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
        use ocentra_parent_agent_protocol::screen_evidence::{
            SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_UNKNOWN,
            SCREEN_PROVIDER_LOCAL_VISION, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
            SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
            SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
        };

        let policy = policy_refs::service_policy_refs(
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

    fn assert_record_fields(record: &ScreenAiAnalysisEventRecord, policy_eligible: bool) {
        use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
        use ocentra_parent_agent_protocol::screen_evidence::{
            SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_UNKNOWN,
            SCREEN_PROVIDER_LOCAL_VISION, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
            SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
            SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
        };

        assert_eq!(
            record.queue_job_id,
            constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
        );
        assert_eq!(
            record.image_digest,
            constants::activity_store::TEST_SCREEN_IMAGE_DIGEST
        );
        assert_eq!(
            record.timestamp,
            constants::activity_store::TEST_FIRST_OBSERVED_AT
        );
        assert_eq!(
            record.summary,
            constants::activity_store::TEST_SCREEN_SUMMARY
        );
        assert_eq!(record.primary_category, SCREEN_CATEGORY_UNKNOWN);
        assert_eq!(record.confidence, SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE);
        assert_eq!(record.provider_kind, SCREEN_PROVIDER_LOCAL_VISION);
        assert_eq!(
            record.model_runtime_ref,
            SCREEN_SERVICE_ANALYSIS_RUNTIME_REF
        );
        assert_eq!(record.model_id, SCREEN_SERVICE_ANALYSIS_MODEL_ID);
        assert_eq!(
            record.prompt_or_template_version,
            SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION
        );
        assert_eq!(
            record.capture_reason,
            constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE
        );
        assert_eq!(record.capture_scope, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW);
        assert_eq!(
            record.capability_status,
            ActivityCaptureCapabilityStatus::Available
                .as_protocol_str()
                .to_string()
        );
        assert_eq!(record.policy_eligible, policy_eligible);
        if policy_eligible {
            assert_eq!(
                record.policy_decision_ref.as_deref(),
                Some(
                    prefixed_id(
                        constants::screen_flow::SCREEN_SERVICE_POLICY_DECISION_ID_PREFIX,
                        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
                    )
                    .as_str()
                )
            );
            assert_eq!(
                record.policy_action.as_deref(),
                Some(constants::screen_flow::SCREEN_SERVICE_POLICY_ACTION_ALLOW)
            );
            assert_eq!(
                record.policy_reason_codes,
                vec![constants::screen_flow::SCREEN_SERVICE_POLICY_REASON_CODE.to_string()]
            );
            assert_eq!(
                record.parent_rule_refs,
                vec![constants::screen_flow::SCREEN_SERVICE_PARENT_RULE_REF.to_string()]
            );
            assert_eq!(
                record.parent_explanation_refs,
                vec![prefixed_id(
                    constants::screen_flow::SCREEN_SERVICE_PARENT_EXPLANATION_REF_PREFIX,
                    constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
                )]
            );
            assert_eq!(
                record.explanation_reasons,
                vec![constants::screen_flow::SCREEN_SERVICE_EXPLANATION_REASON.to_string()]
            );
            assert_eq!(
                record.deletion_reasons,
                vec![constants::screen_flow::SCREEN_SERVICE_DELETION_REASON.to_string()]
            );
        } else {
            assert!(record.policy_decision_ref.is_none());
            assert!(record.policy_action.is_none());
            assert!(record.policy_reason_codes.is_empty());
            assert!(record.parent_rule_refs.is_empty());
            assert!(record.parent_explanation_refs.is_empty());
            assert!(record.explanation_reasons.is_empty());
            assert!(record.deletion_reasons.is_empty());
        }
        assert!(record.ocr_text_snippets.is_empty());
        assert!(record.redaction_notes.is_empty());
    }

    fn string_value<'a>(
        fields: &'a [(
            &'static TestStr,
            ocentra_parent_agent_protocol::logging::LogFieldValue,
        )],
        field_name: &TestStr,
    ) -> Option<&'a TestStr> {
        fields.iter().find_map(
            |(field_key, value)| match (*field_key == field_name, value) {
                (true, ocentra_parent_agent_protocol::logging::LogFieldValue::String(value)) => {
                    Some(value.as_str())
                }
                _ => None,
            },
        )
    }

    fn prefixed_id(prefix: &TestStr, value: &TestStr) -> TestString {
        let mut id = TestString::from(prefix);
        id.push_str(value);
        id
    }
}
