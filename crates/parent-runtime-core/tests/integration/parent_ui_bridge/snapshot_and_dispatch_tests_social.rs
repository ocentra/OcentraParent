use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use serde_json::{json, Value};

use super::*;

#[test]
fn browser_route_projects_typed_social_service_responses() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Browser,
        browser_projection(),
        TestContext("Browser route projects social service responses"),
    );

    assert_eq!(value["route"], "browser");
    assert_eq!(
        value["browserPanels"]["socialDashboard"]["summary"],
        "1 social dashboard rows"
    );
    assert_eq!(
        value["browserPanels"]["socialDashboard"]["rows"][0]["title"],
        "account-approval-queue"
    );
    assert_eq!(
        value["browserPanels"]["socialDashboard"]["rows"][0]["details"][0]["value"],
        "ready-for-review"
    );
    assert_eq!(
        value["browserPanels"]["socialAuditExplanation"]["summary"],
        "0 social explanation rows"
    );
    assert_eq!(
        value["browserPanels"]["socialAlertReport"]["summary"],
        "0 social alert/report rows"
    );
    assert_eq!(
        value["browserPanels"]["socialAlertReportParentSurface"]["summary"],
        "0 parent surface rows"
    );
    assert_eq!(
        value["browserPanels"]["socialParentNotificationDelivery"]["summary"],
        "0 parent notification readiness rows"
    );
}

fn browser_projection() -> Vec<ParentAgentServiceProjectionResponse> {
    vec![
        projection_response(
            AgentCommandName::AgentActivityBrowserReadModelGet,
            browser_activity_read_model_response_event(),
        ),
        response(
            AgentCommandName::AgentBrowserInventoryReadModelGet,
            AgentEventName::AgentBrowserInventoryReadModelReported,
            constants::field::BROWSER_INVENTORY_READ_MODEL_JSON,
            empty_browser_inventory(),
        ),
        response(
            AgentCommandName::AgentBrowserEvidenceRecentGet,
            AgentEventName::AgentBrowserEvidenceRecentReported,
            constants::field::BROWSER_EVIDENCE_READ_MODEL_JSON,
            empty_browser_evidence(),
        ),
        response(
            AgentCommandName::AgentBrowserManagedBridgePoll,
            AgentEventName::AgentBrowserManagedStatusReported,
            constants::field::BROWSER_MANAGED_STATUS_JSON,
            unavailable_browser_managed_status(),
        ),
        response(
            AgentCommandName::AgentBrowserInterventionReadModelGet,
            AgentEventName::AgentBrowserInterventionReadModelReported,
            constants::field::BROWSER_INTERVENTION_READ_MODEL_JSON,
            empty_browser_intervention(),
        ),
        response(
            AgentCommandName::AgentBrowserSocialDashboardReadModelGet,
            AgentEventName::AgentBrowserSocialDashboardReadModelReported,
            constants::field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
            social_dashboard(),
        ),
        response(
            AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet,
            AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported,
            constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
            empty_social_audit(),
        ),
        response(
            AgentCommandName::AgentBrowserSocialAlertReportReadModelGet,
            AgentEventName::AgentBrowserSocialAlertReportReadModelReported,
            constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
            empty_social_alerts(),
        ),
        response(
            AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet,
            AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
            constants::field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL,
            empty_social_parent_surface(),
        ),
        response(
            AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet,
            AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported,
            constants::field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
            empty_social_notification_delivery(),
        ),
    ]
}

fn response(
    command: AgentCommandName,
    event: AgentEventName,
    field: &str,
    value: Value,
) -> ParentAgentServiceProjectionResponse {
    let mut payload = BTreeMap::new();
    payload.insert(
        field.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&value),
            "Browser response fixture serializes",
        )),
    );
    projection_response(
        command,
        AgentEventEnvelope {
            schema_version: 1,
            event_id: format!("browser-response-{field}"),
            correlation_id: "browser-social-projection".to_string(),
            sent_at: "2026-09-05T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                role: AgentPeerRole::AgentService,
            },
            target: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            event,
            severity: LogLevel::Info,
            payload: payload.into(),
            snapshot: None,
        },
    )
}

fn empty_browser_inventory() -> Value {
    json!({
        "schemaVersion": 1,
        "generatedAt": "2026-09-05T00:00:00Z",
        "limit": 25,
        "returned": 0,
        "latestObservedAt": null,
        "capabilityStatus": "bridge-missing",
        "custodyLabel": "unavailable",
        "queryVisibility": "unavailable",
        "rows": []
    })
}

fn empty_browser_evidence() -> Value {
    json!({
        "schemaVersion": 1,
        "generatedAt": "2026-09-05T00:00:00Z",
        "limit": 25,
        "returned": 0,
        "latestEventId": null,
        "latestObservedAt": null,
        "capabilityStatus": "bridge-missing",
        "custodyLabel": "unavailable",
        "queryVisibility": "unavailable",
        "rows": []
    })
}

fn unavailable_browser_managed_status() -> Value {
    json!({
        "schemaVersion": 1,
        "checkedAt": "2026-09-05T00:00:00Z",
        "managedBrowserSessionId": null,
        "browserFamily": null,
        "browserChannel": null,
        "browserVersion": null,
        "profileId": null,
        "profilePathRef": null,
        "profileRootRef": null,
        "profileScopeId": null,
        "profileLifecycleState": null,
        "policyRevision": null,
        "processId": null,
        "bridgeKind": null,
        "bridgeEndpointRef": null,
        "unmanagedProcessName": null,
        "unmanagedExecutablePathRef": null,
        "unmanagedSignatureRef": null,
        "unmanagedProcessHashRef": null,
        "unmanagedProcessKind": null,
        "unmanagedDetectionConfidence": null,
        "unmanagedDetectionReason": null,
        "managedState": "bridge-disconnected",
        "capabilityStatus": "bridge-missing",
        "degradedReason": "managed browser bridge unavailable",
        "startedAt": null,
        "custodyLabel": "unavailable",
        "queryVisibility": "unavailable"
    })
}

fn empty_browser_intervention() -> Value {
    json!({
        "schemaVersion": 1,
        "generatedAt": "2026-09-05T00:00:00Z",
        "limit": 25,
        "returned": 0,
        "latestEventId": null,
        "latestObservedAt": null,
        "managedSessionInterventionCapability": "needs-managed-session",
        "unmanagedBrowserEnforcement": "unavailable",
        "unmanagedFallbackAction": "unavailable",
        "rows": []
    })
}

fn social_dashboard() -> Value {
    json!({
        "schemaVersion": "social-dashboard-ux-contract",
        "familyId": "family-ref",
        "childProfileId": "child-ref",
        "generatedAt": "2026-09-05T00:00:00Z",
        "panels": [{
            "panelId": "account-approval-queue",
            "panelKind": "account-approval-queue",
            "status": "ready-for-review",
            "primaryAction": "open-parent-approval",
            "severity": "info",
            "sortOrder": 0,
            "sourceEvidenceRefs": ["social-approval-evidence"],
            "reasons": ["parent-review-needed"],
            "renderedUiClaimed": true,
            "notificationClaimed": false,
            "runtimeDataFetchClaimed": false,
            "policyDecisionClaimed": false,
            "nativeAppControlClaimed": false,
            "connectorAuthorizationClaimed": false,
            "enforcementClaimed": false
        }],
        "claimBoundaries": {
            "renderedPortalUi": "rendered",
            "notificationDelivery": "not-claimed",
            "runtimeDataFetch": "not-claimed",
            "policyDecision": "not-claimed",
            "nativeAppControl": "not-claimed",
            "connectorAuthorization": "not-claimed",
            "enforcement": "not-claimed"
        }
    })
}

fn empty_social_audit() -> Value {
    json!({
        "schemaVersion": "social-audit-explanation-read-model",
        "snapshotId": "social-audit-snapshot",
        "familyId": "family-ref",
        "childProfileId": "child-ref",
        "capturedAt": "2026-09-05T00:00:00Z",
        "entries": [],
        "claimBoundaries": {
            "runtimeAuditStore": "not-claimed",
            "renderedExplanationUi": "rendered",
            "notificationDelivery": "not-claimed",
            "rawAccountVideoMessageContent": "excluded",
            "connectorAuthorization": "not-claimed",
            "nativeAppControl": "not-claimed",
            "finalPolicyDecision": "not-claimed",
            "enforcement": "not-claimed"
        }
    })
}

fn empty_social_alerts() -> Value {
    json!({
        "schemaVersion": "social-alert-report-read-model",
        "familyId": "family-ref",
        "childProfileId": "child-ref",
        "generatedAt": "2026-09-05T00:00:00Z",
        "intents": [],
        "providerStatusRows": [],
        "claimBoundaries": {
            "providerDelivery": "not-claimed",
            "reportDelivery": "not-claimed",
            "parentNotificationUi": "not-claimed",
            "finalPolicyDecision": "not-claimed",
            "enforcement": "not-claimed"
        }
    })
}

fn empty_social_parent_surface() -> Value {
    json!({
        "schemaVersion": "social-alert-report-parent-surface-read-model",
        "intentId": "social-parent-surface",
        "generatedAt": "2026-09-05T00:00:00Z",
        "sourceProviderStatusHandoffId": "provider-handoff",
        "sourcePreferenceStatusHandoffId": "preference-handoff",
        "rows": [],
        "manualActionRequiredCount": 0,
        "unavailableVisibleCount": 0,
        "historyVisibleCount": 0,
        "preferenceSetupRequiredCount": 0,
        "parentSurfaceNonClaims": ["no-parent-notification-ui"],
        "parentNotificationUiRendered": false,
        "parentNotificationPreferenceUiRendered": false,
        "parentFrequencyControlUiRendered": false,
        "parentNotificationHistoryUiRendered": false,
        "providerDeliveryRuntimeClaimed": false,
        "providerReceiptIngestionClaimed": false,
        "providerCredentialsClaimed": false,
        "cloudRoutingClaimed": false,
        "childDeliveryClaimed": false,
        "quietHoursTimerRuntimeClaimed": false,
        "retryExecutionRuntimeClaimed": false,
        "productionDurableOutboxStorageClaimed": false,
        "adapterDispatchClaimed": false,
        "reportDeliveryExecutionClaimed": false,
        "finalPolicyExecutionClaimed": false,
        "connectorNativeRuntimeClaimed": false,
        "enforcementClaimed": false
    })
}

fn empty_social_notification_delivery() -> Value {
    json!({
        "schemaVersion": "social-parent-notification-delivery-read-model",
        "readinessId": "social-parent-notification-readiness",
        "generatedAt": "2026-09-05T00:00:00Z",
        "sourceReportWriterProofRef": "report-writer-proof",
        "rows": [],
        "nonClaims": ["no-parent-notification-ui-delivery"],
        "parentReportStatusReadyCount": 0,
        "manualRequiredCount": 0,
        "unavailableCount": 0,
        "parentNotificationUiDeliveryClaimed": false,
        "externalRuntimeReportDeliveryClaimed": false,
        "finalPolicyExecutionClaimed": false,
        "enforcementClaimed": false
    })
}
