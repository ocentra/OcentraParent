use super::parent_ui_bridge::{
    PARENT_BRIDGE_COMMAND_DISPATCH, PARENT_BRIDGE_COMMAND_LOAD_ROUTE,
    PARENT_BRIDGE_COMMAND_SUBSCRIBE, PARENT_BRIDGE_COMMAND_UNSUBSCRIBE,
    PARENT_DEV_BRIDGE_ROUTE_DISPATCH, PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE, PARENT_ROUTE_HASH_PREFIX,
    PARENT_ROUTE_HASH_QUERY_SEPARATOR, PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX,
    PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS, PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF_JSON,
    PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF_JSON,
    PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT,
    PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF_JSON,
    PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION, PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX,
    PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET, PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE,
    PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED, PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED,
    PARENT_UI_BRIDGE_SCHEMA_VERSION,
};
use crate::parent_agent_protocol_bridge_ts::{
    generated_portal_agent_protocol_bridge_typescript, parent_agent_protocol_bridge_typescript,
    parent_agent_protocol_domain_contracts_typescript,
};
use crate::schema_result_or_unreachable;
use ocentra_parent_agent_protocol::constants::activity_event_kind;
use ocentra_parent_agent_protocol::constants::tracking_retention_settings_write;

const PARENT_DEV_BRIDGE_URL_ENV_KEY: &str = "VITE_PARENT_DEV_BRIDGE_URL";
const PARENT_AGENT_PROTOCOL_BRIDGE_TYPES_TOKEN: &str = "__PARENT_AGENT_PROTOCOL_BRIDGE_TYPES__";
const PARENT_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN: &str = "__PARENT_ACTIVITY_MEMORY_GRAPH_TYPES__";
const GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN: &str =
    "__GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES__";
const GENERATED_PORTAL_ACTIVITY_EVENT_KIND_TOKEN: &str = "__GENERATED_PORTAL_ACTIVITY_EVENT_KIND__";
const PARENT_TAURI_INTERNAL_WINDOW_KEY: &str = "__TAURI_INTERNALS__";
const PARENT_TYPEOF_UNDEFINED: &str = "undefined";
const GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_TOKEN: &str =
    "__GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS__";
const GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL_TOKEN: &str =
    "__GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL__";

fn generated_tracking_retention_settings_write_defaults_json() -> String {
    format!(
        r#"{{
  "CommandId": "{}",
  "SettingsKindRetentionWindow": "{}",
  "WriterIntentRef": "{}",
  "ReadModelProofRefs": [
    "{}",
    "{}"
  ],
  "MutationProofRef": "{}",
  "LocalServiceStateSnapshotRef": "{}",
  "DurableSettingsStoreRef": "{}",
  "WriteStateAccepted": "{}",
  "WriteStateRejected": "{}",
  "AcceptedAt": "{}"
}}"#,
        tracking_retention_settings_write::COMMAND_ID,
        tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW,
        tracking_retention_settings_write::WRITER_INTENT_REF,
        tracking_retention_settings_write::READ_MODEL_PROOF_REF,
        tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF,
        tracking_retention_settings_write::MUTATION_PROOF_REF,
        tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF,
        tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF,
        tracking_retention_settings_write::WRITE_STATE_ACCEPTED,
        tracking_retention_settings_write::WRITE_STATE_REJECTED,
        tracking_retention_settings_write::ACCEPTED_AT,
    )
}

fn generated_portal_activity_event_kind_typescript() -> String {
    format!(
        r#"{{
  ProcessObserved: '{}',
  WindowFocused: '{}',
  DomainObserved: '{}',
  UrlObserved: '{}',
  VideoObserved: '{}',
  BrowserInterventionApplied: '{}',
  EnforcementAuditRecorded: '{}',
  DeviceIdleStateObserved: '{}',
  ScreenAnalysisSummarized: '{}',
  LocationObserved: '{}',
  TrackingAlertEvaluated: '{}',
  TrackingGeofenceTransitionEvaluated: '{}',
  TrackingExpectedPlaceEvaluated: '{}',
  TrackingChildCheckInResponded: '{}',
  TrackingParentNotificationRequested: '{}',
  TrackingRetentionDeleted: '{}',
  NetworkRetentionDeleted: '{}',
}} as const"#,
        activity_event_kind::PROCESS_OBSERVED,
        activity_event_kind::WINDOW_FOCUSED,
        activity_event_kind::DOMAIN_OBSERVED,
        activity_event_kind::URL_OBSERVED,
        activity_event_kind::VIDEO_OBSERVED,
        activity_event_kind::BROWSER_INTERVENTION_APPLIED,
        activity_event_kind::ENFORCEMENT_AUDIT_RECORDED,
        activity_event_kind::DEVICE_IDLE_STATE_OBSERVED,
        activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED,
        activity_event_kind::LOCATION_OBSERVED,
        activity_event_kind::TRACKING_ALERT_EVALUATED,
        activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
        activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
        activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
        activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
        activity_event_kind::TRACKING_RETENTION_DELETED,
        activity_event_kind::NETWORK_RETENTION_DELETED,
    )
}

const GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL_JSON: &str = r#"{
  "schemaVersion": "v0.6",
  "proofId": "tracking-notification-parent-surface-history-proof",
  "generatedAt": "2026-06-06T16:16:00.000Z",
  "family": {
    "familyId": "family-tracking-notification-history"
  },
  "sourceProviderNotificationProofRef": "tracking-provider-notification-proof-for-parent-surface-history",
  "sourceReceiptBoundaryProofRef": "tracking-notification-receipt-boundary-proof-for-parent-surface-history",
  "sourcePreferencePreflightProofRef": "tracking-notification-preference-preflight-proof-for-parent-surface-history",
  "sourceContractRefs": [
    "tracking-provider-notification-proof",
    "tracking-notification-receipt-boundary-proof",
    "tracking-notification-preference-preflight-proof",
    "notifications-expectations",
    "location-geofence-device-status"
  ],
  "rows": [
    {
      "historyRowId": "tracking-notification-history-tracking-alert-home-arrival",
      "sourceAlertId": "tracking-alert-home-arrival",
      "sourceProviderNotificationRowId": "tracking-provider-notification-tracking-alert-home-arrival",
      "sourceReceiptBoundaryRowId": "tracking-notification-receipt-tracking-alert-home-arrival",
      "sourcePreferencePreflightRowId": "tracking-notification-preference-preflight-tracking-alert-home-arrival",
      "status": "history-intent-ready",
      "sourcePolicyDecisionId": "tracking-decision-home-arrival",
      "evidenceRefs": ["location-evidence-geofence-entry"],
      "notificationStatusRefs": ["tracking-notification-intent-home-arrival"],
      "reasonCodeRefs": ["home-arrival-notification"],
      "providerStatusEntryRef": "tracking-provider-status-entry-home-arrival",
      "providerAttemptRef": "tracking-provider-attempt-home-arrival",
      "auditRefs": ["tracking-provider-notification-audit-tracking-alert-home-arrival"],
      "providerPreferenceRefs": ["tracking-parent-provider-preference-home-arrival"],
      "parentPreferenceRequirementRefs": ["parent-notification-preference-required-home-arrival"],
      "quietHoursRequirementRefs": ["tracking-quiet-hours-policy-required-tracking-alert-home-arrival"],
      "receiptRequirementRefs": ["receipt-ingestion-required-home-arrival"],
      "manualProofRequirements": ["provider-delivery-runtime-required", "receipt-webhook-runtime-required"],
      "drillInRefs": ["tracking-notification-history-drill-in-tracking-alert-home-arrival"],
      "redactedParentSummaryRef": "tracking-notification-redacted-summary-tracking-alert-home-arrival",
      "renderedParentNotificationUiClaimed": false,
      "parentPreferenceMutationRuntimeClaimed": false,
      "providerDeliveryClaimed": false,
      "receiptIngestionRuntimeClaimed": false,
      "childDeviceDeliveryClaimed": false,
      "mobilePhysicalDeviceProofClaimed": false,
      "authorityProofClaimed": false
    },
    {
      "historyRowId": "tracking-notification-history-tracking-alert-left-expected-place",
      "sourceAlertId": "tracking-alert-left-expected-place",
      "sourceProviderNotificationRowId": "tracking-provider-notification-tracking-alert-left-expected-place",
      "sourceReceiptBoundaryRowId": "tracking-notification-receipt-tracking-alert-left-expected-place",
      "sourcePreferencePreflightRowId": "tracking-notification-preference-preflight-tracking-alert-left-expected-place",
      "status": "manual-action-required",
      "sourcePolicyDecisionId": "tracking-decision-left-expected-place",
      "evidenceRefs": ["location-evidence-geofence-entry"],
      "notificationStatusRefs": ["tracking-notification-intent-left-school"],
      "reasonCodeRefs": ["left-expected-place"],
      "providerStatusEntryRef": "tracking-provider-status-entry-left-school",
      "providerAttemptRef": "tracking-provider-attempt-left-school",
      "auditRefs": ["tracking-provider-notification-audit-tracking-alert-left-expected-place"],
      "providerPreferenceRefs": ["tracking-parent-provider-preference-left-school"],
      "parentPreferenceRequirementRefs": ["tracking-parent-notification-preference-required-tracking-alert-left-school"],
      "quietHoursRequirementRefs": ["quiet-hours-requirement-left-school"],
      "receiptRequirementRefs": ["manual-receipt-required-left-school"],
      "manualProofRequirements": ["manual-provider-review-required", "quiet-hours-runtime-required"],
      "drillInRefs": ["tracking-notification-history-drill-in-tracking-alert-left-expected-place"],
      "redactedParentSummaryRef": "tracking-notification-redacted-summary-tracking-alert-left-expected-place",
      "renderedParentNotificationUiClaimed": false,
      "parentPreferenceMutationRuntimeClaimed": false,
      "providerDeliveryClaimed": false,
      "receiptIngestionRuntimeClaimed": false,
      "childDeviceDeliveryClaimed": false,
      "mobilePhysicalDeviceProofClaimed": false,
      "authorityProofClaimed": false
    },
    {
      "historyRowId": "tracking-notification-history-tracking-alert-provider-unavailable",
      "sourceAlertId": "tracking-alert-provider-unavailable",
      "sourceProviderNotificationRowId": "tracking-provider-notification-tracking-alert-provider-unavailable",
      "sourceReceiptBoundaryRowId": "tracking-notification-receipt-tracking-alert-provider-unavailable",
      "sourcePreferencePreflightRowId": "tracking-notification-preference-preflight-tracking-alert-provider-unavailable",
      "status": "provider-unavailable",
      "sourcePolicyDecisionId": "tracking-decision-provider-unavailable",
      "evidenceRefs": ["location-evidence-geofence-entry"],
      "notificationStatusRefs": ["tracking-notification-intent-provider-unavailable"],
      "reasonCodeRefs": ["provider-unavailable"],
      "providerStatusEntryRef": "tracking-provider-status-entry-provider-unavailable",
      "providerAttemptRef": "tracking-provider-attempt-unavailable",
      "auditRefs": ["tracking-provider-notification-audit-tracking-alert-provider-unavailable"],
      "providerPreferenceRefs": ["tracking-parent-provider-preference-provider-unavailable"],
      "parentPreferenceRequirementRefs": ["source-unavailable-preference-required"],
      "quietHoursRequirementRefs": [],
      "receiptRequirementRefs": ["provider-receipt-unavailable"],
      "manualProofRequirements": ["provider-adapter-unavailable", "manual-parent-history-review-required"],
      "drillInRefs": ["tracking-notification-history-drill-in-tracking-alert-provider-unavailable"],
      "redactedParentSummaryRef": "tracking-notification-redacted-summary-tracking-alert-provider-unavailable",
      "renderedParentNotificationUiClaimed": false,
      "parentPreferenceMutationRuntimeClaimed": false,
      "providerDeliveryClaimed": false,
      "receiptIngestionRuntimeClaimed": false,
      "childDeviceDeliveryClaimed": false,
      "mobilePhysicalDeviceProofClaimed": false,
      "authorityProofClaimed": false
    }
  ],
  "historyIntentReadyCount": 1,
  "manualActionRequiredCount": 1,
  "providerUnavailableCount": 1,
  "proofNonClaims": [
    "no-rendered-parent-notification-ui",
    "no-parent-preference-mutation-runtime",
    "no-parent-frequency-control-ui",
    "no-quiet-hours-timer-runtime",
    "no-provider-delivery-execution",
    "no-provider-receipt-ingestion-runtime",
    "no-provider-credentials",
    "no-cloud-routing",
    "no-child-device-delivery",
    "no-mobile-physical-device-proof",
    "no-authority-proof",
    "no-retry-worker-runtime",
    "no-production-durable-history-storage",
    "no-production-durable-outbox-storage",
    "no-adapter-dispatch"
  ],
  "renderedParentNotificationUiClaimed": false,
  "parentPreferenceMutationRuntimeClaimed": false,
  "parentFrequencyControlUiClaimed": false,
  "quietHoursTimerRuntimeClaimed": false,
  "providerDeliveryRuntimeClaimed": false,
  "providerReceiptIngestionRuntimeClaimed": false,
  "providerCredentialsClaimed": false,
  "cloudRoutingClaimed": false,
  "childDeviceDeliveryClaimed": false,
  "mobilePhysicalDeviceProofClaimed": false,
  "authorityProofClaimed": false,
  "retryExecutionRuntimeClaimed": false,
  "productionDurableHistoryStorageClaimed": false,
  "productionDurableOutboxStorageClaimed": false,
  "adapterDispatchClaimed": false
}"#;

const ACTIVITY_MEMORY_GRAPH_TYPESCRIPT_TEMPLATE: &str =
    include_str!("parent_ui_bridge_ts_activity_memory_graph.template.txt");

fn parent_ui_bridge_typescript_template() -> String {
    assemble_template_fragments(&[
        include_str!("parent_ui_bridge_typescript.routes.template.txt"),
        include_str!("parent_ui_bridge_typescript.route-metadata.template.txt"),
        include_str!("parent_ui_bridge_typescript.runtime.template.txt"),
        include_str!("parent_ui_bridge_typescript.snapshots.template.txt"),
        include_str!("parent_ui_bridge_typescript.activity.template.txt"),
        include_str!("parent_ui_bridge_typescript.panels.template.txt"),
        include_str!("parent_ui_bridge_typescript.summary.template.txt"),
        include_str!("parent_ui_bridge_typescript.actions.template.txt"),
    ])
}

const PARENT_UI_SCREEN_BRIDGE_TYPESCRIPT_TEMPLATE: &str =
    include_str!("parent_ui_screen_bridge_typescript.template.txt");

const PARENT_BRIDGE_COMMAND_LOAD_ROUTE_TOKEN: &str = "__PARENT_BRIDGE_COMMAND_LOAD_ROUTE__";
const PARENT_BRIDGE_COMMAND_DISPATCH_TOKEN: &str = "__PARENT_BRIDGE_COMMAND_DISPATCH__";
const PARENT_BRIDGE_COMMAND_SUBSCRIBE_TOKEN: &str = "__PARENT_BRIDGE_COMMAND_SUBSCRIBE__";
const PARENT_BRIDGE_COMMAND_UNSUBSCRIBE_TOKEN: &str = "__PARENT_BRIDGE_COMMAND_UNSUBSCRIBE__";
const PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE_TOKEN: &str = "__PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE__";
const PARENT_DEV_BRIDGE_ROUTE_DISPATCH_TOKEN: &str = "__PARENT_DEV_BRIDGE_ROUTE_DISPATCH__";
const PARENT_DEV_BRIDGE_URL_ENV_KEY_TOKEN: &str = "__PARENT_DEV_BRIDGE_URL_ENV_KEY__";
const PARENT_ROUTE_HASH_PREFIX_TOKEN: &str = "__PARENT_ROUTE_HASH_PREFIX__";
const PARENT_ROUTE_HASH_QUERY_SEPARATOR_TOKEN: &str = "__PARENT_ROUTE_HASH_QUERY_SEPARATOR__";
const PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION__";
const PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX__";
const PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET__";
const PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE__";
const PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED__";
const PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED_TOKEN: &str =
    "__PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED__";
const PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF_TOKEN: &str =
    "__PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF__";
const PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF_TOKEN: &str =
    "__PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF__";
const PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT_TOKEN: &str =
    "__PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT__";
const PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF_TOKEN: &str =
    "__PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF__";
const PARENT_SUBSCRIPTION_EVENT_PREFIX_TOKEN: &str = "__PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX__";
const PARENT_SUBSCRIPTION_POLL_MS_TOKEN: &str = "__PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS__";
const PARENT_TAURI_INTERNAL_WINDOW_KEY_TOKEN: &str = "__PARENT_TAURI_INTERNAL_WINDOW_KEY__";
const PARENT_TYPEOF_UNDEFINED_TOKEN: &str = "__PARENT_TYPEOF_UNDEFINED__";
const PARENT_UI_BRIDGE_SCHEMA_VERSION_TOKEN: &str = "__PARENT_UI_BRIDGE_SCHEMA_VERSION__";

fn portal_contracts_typescript_template() -> String {
    let types = include_str!("parent_ui_bridge_ts_portal_contracts.types.template.txt")
        .strip_suffix('\n')
        .unwrap_or(include_str!(
            "parent_ui_bridge_ts_portal_contracts.types.template.txt"
        ));
    let runtime = include_str!("parent_ui_bridge_ts_portal_contracts.runtime.template.txt")
        .strip_suffix('\n')
        .unwrap_or(include_str!(
            "parent_ui_bridge_ts_portal_contracts.runtime.template.txt"
        ));
    format!("{types}\n\n{runtime}")
}

fn assemble_template_fragments(fragments: &[&str]) -> String {
    fragments
        .iter()
        .map(|fragment| fragment.strip_suffix('\n').unwrap_or(fragment))
        .collect()
}

fn parent_ui_bridge_json_literal(value: &str) -> String {
    let parsed: serde_json::Value = schema_result_or_unreachable(
        serde_json::from_str(value),
        "parent UI bridge screen JSON parses",
    );
    schema_result_or_unreachable(
        serde_json::to_string(&parsed),
        "parent UI bridge screen JSON serializes",
    )
}

fn activity_memory_graph_typescript(prefix: &str) -> String {
    ACTIVITY_MEMORY_GRAPH_TYPESCRIPT_TEMPLATE
        .strip_suffix('\n')
        .unwrap_or(ACTIVITY_MEMORY_GRAPH_TYPESCRIPT_TEMPLATE)
        .replace("__ACTIVITY_MEMORY_GRAPH_PREFIX__", prefix)
}

fn trim_generated_trailing_whitespace(value: &str) -> String {
    let mut trimmed = String::with_capacity(value.len());

    for line in value.split_inclusive('\n') {
        if let Some(content) = line.strip_suffix('\n') {
            trimmed.push_str(content.trim_end());
            trimmed.push('\n');
        } else {
            trimmed.push_str(line.trim_end());
        }
    }

    trimmed
}

pub fn parent_ui_bridge_typescript() -> String {
    trim_generated_trailing_whitespace(
        &parent_ui_bridge_typescript_template()
            .replace(
                PARENT_BRIDGE_COMMAND_LOAD_ROUTE_TOKEN,
                PARENT_BRIDGE_COMMAND_LOAD_ROUTE,
            )
            .replace(
                PARENT_BRIDGE_COMMAND_DISPATCH_TOKEN,
                PARENT_BRIDGE_COMMAND_DISPATCH,
            )
            .replace(
                PARENT_BRIDGE_COMMAND_SUBSCRIBE_TOKEN,
                PARENT_BRIDGE_COMMAND_SUBSCRIBE,
            )
            .replace(
                PARENT_BRIDGE_COMMAND_UNSUBSCRIBE_TOKEN,
                PARENT_BRIDGE_COMMAND_UNSUBSCRIBE,
            )
            .replace(
                PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE_TOKEN,
                PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE,
            )
            .replace(
                PARENT_DEV_BRIDGE_ROUTE_DISPATCH_TOKEN,
                PARENT_DEV_BRIDGE_ROUTE_DISPATCH,
            )
            .replace(
                PARENT_DEV_BRIDGE_URL_ENV_KEY_TOKEN,
                PARENT_DEV_BRIDGE_URL_ENV_KEY,
            )
            .replace(PARENT_ROUTE_HASH_PREFIX_TOKEN, PARENT_ROUTE_HASH_PREFIX)
            .replace(
                PARENT_ROUTE_HASH_QUERY_SEPARATOR_TOKEN,
                PARENT_ROUTE_HASH_QUERY_SEPARATOR,
            )
            .replace(
                PARENT_SUBSCRIPTION_EVENT_PREFIX_TOKEN,
                PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX,
            )
            .replace(
                PARENT_SUBSCRIPTION_POLL_MS_TOKEN,
                &PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS.to_string(),
            )
            .replace(
                PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION_TOKEN,
                &PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION.to_string(),
            )
            .replace(
                PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX_TOKEN,
                PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX,
            )
            .replace(
                PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET_TOKEN,
                PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET,
            )
            .replace(
                PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE_TOKEN,
                PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE,
            )
            .replace(
                PARENT_TAURI_INTERNAL_WINDOW_KEY_TOKEN,
                PARENT_TAURI_INTERNAL_WINDOW_KEY,
            )
            .replace(PARENT_TYPEOF_UNDEFINED_TOKEN, PARENT_TYPEOF_UNDEFINED)
            .replace(
                PARENT_UI_BRIDGE_SCHEMA_VERSION_TOKEN,
                &PARENT_UI_BRIDGE_SCHEMA_VERSION.to_string(),
            )
            .replace(
                PARENT_AGENT_PROTOCOL_BRIDGE_TYPES_TOKEN,
                &parent_agent_protocol_bridge_typescript(),
            )
            .replace(
                PARENT_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN,
                &activity_memory_graph_typescript("Parent"),
            ),
    )
}

pub fn agent_protocol_domain_contracts_typescript() -> String {
    parent_agent_protocol_domain_contracts_typescript()
}

pub fn parent_ui_screen_bridge_typescript() -> String {
    PARENT_UI_SCREEN_BRIDGE_TYPESCRIPT_TEMPLATE
        .replace(
            PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED_TOKEN,
            PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED,
        )
        .replace(
            PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED_TOKEN,
            PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED,
        )
        .replace(
            PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF_TOKEN,
            &parent_ui_bridge_json_literal(PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF_JSON),
        )
        .replace(
            PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF_TOKEN,
            &parent_ui_bridge_json_literal(PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF_JSON),
        )
        .replace(
            PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT_TOKEN,
            PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT,
        )
        .replace(
            PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF_TOKEN,
            &parent_ui_bridge_json_literal(
                PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF_JSON,
            ),
        )
}

pub fn portal_contracts_typescript() -> String {
    trim_generated_trailing_whitespace(
        &portal_contracts_typescript_template()
            .replace(
                PARENT_AGENT_PROTOCOL_BRIDGE_TYPES_TOKEN,
                &generated_portal_agent_protocol_bridge_typescript(),
            )
            .replace(
                GENERATED_PORTAL_ACTIVITY_EVENT_KIND_TOKEN,
                &generated_portal_activity_event_kind_typescript(),
            )
            .replace(
                GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN,
                &activity_memory_graph_typescript("GeneratedPortal"),
            )
            .replace(
                GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_TOKEN,
                &parent_ui_bridge_json_literal(
                    &generated_tracking_retention_settings_write_defaults_json(),
                ),
            )
            .replace(
                GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL_TOKEN,
                &parent_ui_bridge_json_literal(
                    GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL_JSON,
                ),
            ),
    )
}
