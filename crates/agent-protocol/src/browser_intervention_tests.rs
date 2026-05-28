use serde_json::json;

use crate::{
    constants, BrowserBoundaryState, BrowserChannel, BrowserCustodyLabel,
    BrowserExactUrlClaimState, BrowserFamily, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionMechanism, BrowserInterventionOutcome, BrowserInterventionReadModel,
    BrowserInterventionRow, BrowserInterventionTargetType, BrowserQueryVisibilityLabel,
    BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
    BROWSER_INTERVENTION_SCHEMA_VERSION,
};

#[test]
fn browser_intervention_read_model_serializes_decision_source_and_enforcement_state() {
    let read_model = BrowserInterventionReadModel {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: 5,
        returned: 1,
        latest_event_id: Some(
            constants::activity_store::TEST_BROWSER_INTERVENTION_EVENT_ID.to_string(),
        ),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        managed_session_intervention_capability: BrowserInterventionCapabilityState::Ready,
        unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState::RequiresOsAppControl,
        rows: vec![BrowserInterventionRow {
            schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
            browser_intervention_id: constants::activity_store::TEST_BROWSER_INTERVENTION_ID
                .to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            source_id: constants::browser::INTERVENTION_SOURCE_ID_MANAGED_BROWSER.to_string(),
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            browser_family: Some(BrowserFamily::Chrome),
            browser_channel: Some(BrowserChannel::Stable),
            managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
            profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
            process_id: Some(constants::activity_store::TEST_BROWSER_PROCESS_ID),
            policy_decision_id: Some(
                constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            ),
            decision_source: BrowserInterventionDecisionSource::ParentRule,
            intervention_action: BrowserInterventionAction::Block,
            intervention_target_type: BrowserInterventionTargetType::Video,
            intervention_target_value: constants::activity_store::TEST_BROWSER_URL.to_string(),
            requested_url: Some(constants::activity_store::TEST_BROWSER_URL.to_string()),
            observed_url: Some(constants::activity_store::TEST_BROWSER_URL.to_string()),
            intervention_mechanism: BrowserInterventionMechanism::ChromiumCdpFetch,
            intervention_outcome: BrowserInterventionOutcome::Blocked,
            browser_boundary_state: BrowserBoundaryState::ManagedSession,
            exact_url_claim_state: BrowserExactUrlClaimState::ExactUrlProven,
            unmanaged_detection_state: BrowserUnmanagedDetectionState::None,
            reason: Some(constants::activity_store::TEST_BROWSER_INTERVENTION_REASON.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["managedSessionInterventionCapability"],
        json!(constants::browser::INTERVENTION_CAPABILITY_READY)
    );
    assert_eq!(
        serialized["unmanagedBrowserEnforcement"],
        json!(constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL)
    );
    assert_eq!(
        serialized["rows"][0]["decisionSource"],
        json!(constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE)
    );
    assert_eq!(
        serialized["rows"][0]["interventionMechanism"],
        json!(constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH)
    );
    assert_eq!(
        serialized["rows"][0]["browserBoundaryState"],
        json!(constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION)
    );
    assert_eq!(
        serialized["rows"][0]["exactUrlClaimState"],
        json!(constants::browser::INTERVENTION_EXACT_URL_PROVEN)
    );
    assert_eq!(
        serialized["rows"][0]["unmanagedDetectionState"],
        json!(constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE)
    );
}

#[test]
fn browser_intervention_read_model_serializes_unmanaged_process_without_exact_url_claim() {
    let read_model = BrowserInterventionReadModel {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: 5,
        returned: 1,
        latest_event_id: Some(
            constants::activity_store::TEST_BROWSER_INTERVENTION_EVENT_ID.to_string(),
        ),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        managed_session_intervention_capability:
            BrowserInterventionCapabilityState::NeedsManagedSession,
        unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState::ReadyToBlock,
        rows: vec![BrowserInterventionRow {
            schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
            browser_intervention_id: constants::activity_store::TEST_BROWSER_INTERVENTION_ID
                .to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            source_id: constants::browser::INTERVENTION_SOURCE_ID_MANAGED_BROWSER.to_string(),
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            browser_family: Some(BrowserFamily::Chrome),
            browser_channel: Some(BrowserChannel::Stable),
            managed_browser_session_id: None,
            profile_id: None,
            process_id: Some(constants::activity_store::TEST_BROWSER_PROCESS_ID),
            policy_decision_id: Some(
                constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            ),
            decision_source: BrowserInterventionDecisionSource::ParentRule,
            intervention_action: BrowserInterventionAction::Block,
            intervention_target_type: BrowserInterventionTargetType::BrowserProcess,
            intervention_target_value: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
            requested_url: None,
            observed_url: None,
            intervention_mechanism: BrowserInterventionMechanism::OsAppControl,
            intervention_outcome: BrowserInterventionOutcome::Blocked,
            browser_boundary_state: BrowserBoundaryState::UnmanagedBrowserProcess,
            exact_url_claim_state: BrowserExactUrlClaimState::NotClaimed,
            unmanaged_detection_state: BrowserUnmanagedDetectionState::Terminated,
            reason: Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string()),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["rows"][0]["browserBoundaryState"],
        json!(constants::browser::INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS)
    );
    assert_eq!(
        serialized["rows"][0]["exactUrlClaimState"],
        json!(constants::browser::INTERVENTION_EXACT_URL_NOT_CLAIMED)
    );
    assert_eq!(serialized["rows"][0]["requestedUrl"], json!(null));
    assert_eq!(serialized["rows"][0]["observedUrl"], json!(null));
}
