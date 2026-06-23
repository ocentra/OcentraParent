use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking_acknowledgement_id_from_violation_id;
use ocentra_parent_agent_protocol::tracking_ai_request_id_from_evidence_ref;
use ocentra_parent_agent_protocol::tracking_alert_evaluation_id_from_violation_id;
use ocentra_parent_agent_protocol::tracking_check_in_id_from_observation_id;
use ocentra_parent_agent_protocol::tracking_evaluation_id_from_observation_id;
use ocentra_parent_agent_protocol::tracking_evidence_ref_from_observation_id;
use ocentra_parent_agent_protocol::tracking_missing_device_evaluation_id_from_child_device_id;
use ocentra_parent_agent_protocol::tracking_notification_id_from_violation_id;
use ocentra_parent_agent_protocol::tracking_parent_defined_place_id_from_evidence_ref;
use ocentra_parent_agent_protocol::tracking_temporary_live_session_id_from_child_device_id;
use ocentra_parent_agent_protocol::tracking_transition_id_from_observation_id;
use ocentra_parent_agent_protocol::tracking_violation_id_from_ai_request_and_rule_ref;
use ocentra_parent_agent_protocol::tracking_violation_id_from_evaluation_and_rule_ref;
use ocentra_parent_agent_protocol::TrackingChildDeviceId;
use ocentra_parent_agent_protocol::TrackingObservationId;
use ocentra_parent_agent_protocol::TrackingPolicyRuleRef;

#[test]
fn tracking_derived_identifiers_use_source_refs_and_protocol_prefixes() {
    let observation_id = TrackingObservationId::parse("tracking-observation-42")
        .expect("tracking observation fixture parses");
    let child_device_id = TrackingChildDeviceId::parse(
        constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
    )
    .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID);
    let policy_rule_ref = TrackingPolicyRuleRef::parse(
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
    )
    .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);

    let evidence_ref = tracking_evidence_ref_from_observation_id(&observation_id);
    let ai_request_id = tracking_ai_request_id_from_evidence_ref(&evidence_ref);
    let transition_id = tracking_transition_id_from_observation_id(&observation_id);
    let evaluation_id = tracking_evaluation_id_from_observation_id(&observation_id);
    let check_in_id = tracking_check_in_id_from_observation_id(&observation_id);
    let ai_violation_id =
        tracking_violation_id_from_ai_request_and_rule_ref(&ai_request_id, &policy_rule_ref);
    let evaluation_violation_id =
        tracking_violation_id_from_evaluation_and_rule_ref(&evaluation_id, &policy_rule_ref);
    let notification_id = tracking_notification_id_from_violation_id(&ai_violation_id);
    let acknowledgement_id = tracking_acknowledgement_id_from_violation_id(&ai_violation_id);
    let alert_evaluation_id = tracking_alert_evaluation_id_from_violation_id(&ai_violation_id);
    let temporary_live_session_id =
        tracking_temporary_live_session_id_from_child_device_id(&child_device_id);
    let missing_device_evaluation_id =
        tracking_missing_device_evaluation_id_from_child_device_id(&child_device_id);
    let parent_defined_place_id =
        tracking_parent_defined_place_id_from_evidence_ref(&evidence_ref);

    assert_eq!(
        evidence_ref.as_str(),
        format!(
            "{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE
        )
    );
    assert_eq!(
        ai_request_id.as_str(),
        format!(
            "{}:{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE
        )
    );
    assert_eq!(
        transition_id.as_str(),
        format!(
            "{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE
        )
    );
    assert_eq!(
        evaluation_id.as_str(),
        format!(
            "{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE
        )
    );
    assert_eq!(
        check_in_id.as_str(),
        format!(
            "{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE
        )
    );
    assert_eq!(
        ai_violation_id.as_str(),
        format!(
            "{}:{}:{}:tracking-observation-42:{}",
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
        )
    );
    assert_eq!(
        evaluation_violation_id.as_str(),
        format!(
            "{}:{}:tracking-observation-42:{}",
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
        )
    );
    assert_eq!(
        notification_id.as_str(),
        format!(
            "{}:{}:{}:{}:tracking-observation-42:{}",
            constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
        )
    );
    assert_eq!(
        acknowledgement_id.as_str(),
        format!(
            "{}:{}:{}:{}:tracking-observation-42:{}",
            constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
        )
    );
    assert_eq!(
        alert_evaluation_id.as_str(),
        format!(
            "{}:{}:{}:{}:tracking-observation-42:{}",
            constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
        )
    );
    assert_eq!(
        temporary_live_session_id.as_str(),
        "tracking.temporary-live.session:child-device-default"
    );
    assert_eq!(
        missing_device_evaluation_id.as_str(),
        "tracking.missing-device.evaluation:child-device-default"
    );
    assert_eq!(
        parent_defined_place_id.as_str(),
        format!(
            "{}:{}:tracking-observation-42",
            constants::tracking_runtime::TRACKING_PARENT_DEFINED_PLACE_ID_PREFIX,
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE
        )
    );
}
