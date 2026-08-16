use ocentra_eventing::expect_value::ExpectValue;
use serde_json::{json, Value};

use crate::{
    constants, BrowserBoundaryState, BrowserChannel, BrowserCustodyLabel,
    BrowserExactUrlClaimState, BrowserFamily, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionDeliveryState, BrowserInterventionMechanism, BrowserInterventionOutcome,
    BrowserInterventionReadModel, BrowserInterventionRow, BrowserInterventionTargetType,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
    BrowserUnmanagedFallbackActionState, BROWSER_INTERVENTION_SCHEMA_VERSION,
};

#[test]
fn browser_intervention_read_model_serializes_decision_source_and_enforcement_state() {
    let serialized = serialized_read_model(managed_intervention_read_model());

    assert_eq!(
        serialized["managedSessionInterventionCapability"],
        json!(constants::browser::INTERVENTION_CAPABILITY_READY)
    );
    assert_eq!(
        serialized["unmanagedBrowserEnforcement"],
        json!(constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL)
    );
    assert_eq!(
        serialized["unmanagedFallbackAction"],
        json!(constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_MANUAL_REQUIRED)
    );
    assert_eq!(
        serialized["rows"][0]["decisionSource"],
        json!(constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE)
    );
    assert_managed_intervention_row(&serialized["rows"][0]);
}

#[test]
fn browser_intervention_read_model_serializes_unmanaged_process_without_exact_url_claim() {
    let serialized = serialized_read_model(unmanaged_intervention_read_model());

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
    assert_eq!(
        serialized["rows"][0]["childDeliveryState"],
        json!(constants::browser::INTERVENTION_DELIVERY_MANUAL_REQUIRED)
    );
    assert_eq!(
        serialized["rows"][0]["unmanagedFallbackAction"],
        json!(constants::browser::UNMANAGED_FALLBACK_ACTION_TERMINATE_PROCESS)
    );
}

fn assert_managed_intervention_row(row: &Value) {
    assert_eq!(
        row["interventionActionId"],
        json!(constants::activity_store::TEST_BROWSER_INTERVENTION_ACTION_ID)
    );
    assert_eq!(
        row["interventionAuditId"],
        json!(constants::activity_store::TEST_BROWSER_INTERVENTION_AUDIT_ID)
    );
    assert_eq!(
        row["evidenceReferenceIds"],
        json!([constants::activity_store::TEST_BROWSER_INTERVENTION_EVIDENCE_ID])
    );
    assert_eq!(
        row["interventionMechanism"],
        json!(constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH)
    );
    assert_eq!(
        row["browserBoundaryState"],
        json!(constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION)
    );
    assert_eq!(
        row["exactUrlClaimState"],
        json!(constants::browser::INTERVENTION_EXACT_URL_PROVEN)
    );
    assert_eq!(
        row["unmanagedDetectionState"],
        json!(constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE)
    );
    assert_eq!(
        row["childDeliveryState"],
        json!(constants::browser::INTERVENTION_DELIVERY_BLOCK_PAGE_RENDERED)
    );
}

fn managed_intervention_read_model() -> BrowserInterventionReadModel {
    intervention_read_model(
        BrowserInterventionCapabilityState::Ready,
        BrowserUnmanagedEnforcementState::RequiresOsAppControl,
        base_intervention_row(),
    )
}

fn unmanaged_intervention_read_model() -> BrowserInterventionReadModel {
    let mut row = base_intervention_row();
    row.managed_browser_session_id = None;
    row.profile_id = None;
    row.intervention_target_type = BrowserInterventionTargetType::BrowserProcess;
    row.intervention_target_value = constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string();
    row.requested_url = None;
    row.observed_url = None;
    row.intervention_mechanism = BrowserInterventionMechanism::OsAppControl;
    row.browser_boundary_state = BrowserBoundaryState::UnmanagedBrowserProcess;
    row.exact_url_claim_state = BrowserExactUrlClaimState::NotClaimed;
    row.unmanaged_detection_state = BrowserUnmanagedDetectionState::Terminated;
    row.unmanaged_fallback_action = BrowserUnmanagedFallbackActionState::TerminateProcess;
    row.child_delivery_state = BrowserInterventionDeliveryState::ManualRequired;
    row.reason = Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string());
    intervention_read_model(
        BrowserInterventionCapabilityState::NeedsManagedSession,
        BrowserUnmanagedEnforcementState::ReadyToBlock,
        row,
    )
}

fn intervention_read_model(
    capability: BrowserInterventionCapabilityState,
    unmanaged_enforcement: BrowserUnmanagedEnforcementState,
    row: BrowserInterventionRow,
) -> BrowserInterventionReadModel {
    BrowserInterventionReadModel {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: 5,
        returned: 1,
        latest_event_id: Some(
            constants::activity_store::TEST_BROWSER_INTERVENTION_EVENT_ID.to_string(),
        ),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        managed_session_intervention_capability: capability,
        unmanaged_browser_enforcement: unmanaged_enforcement,
        unmanaged_fallback_action: BrowserUnmanagedFallbackActionState::OsBlockManualRequired,
        rows: vec![row],
    }
}

fn base_intervention_row() -> BrowserInterventionRow {
    BrowserInterventionRow {
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
        intervention_action_id: Some(
            constants::activity_store::TEST_BROWSER_INTERVENTION_ACTION_ID.to_string(),
        ),
        intervention_audit_id: Some(
            constants::activity_store::TEST_BROWSER_INTERVENTION_AUDIT_ID.to_string(),
        ),
        evidence_reference_ids: vec![
            constants::activity_store::TEST_BROWSER_INTERVENTION_EVIDENCE_ID.to_string(),
        ],
        policy_decision_id: Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string()),
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
        unmanaged_fallback_action: BrowserUnmanagedFallbackActionState::Unavailable,
        child_delivery_state: BrowserInterventionDeliveryState::BlockPageRendered,
        reason: Some(constants::activity_store::TEST_BROWSER_INTERVENTION_REASON.to_string()),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn serialized_read_model(read_model: BrowserInterventionReadModel) -> Value {
    serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
}
