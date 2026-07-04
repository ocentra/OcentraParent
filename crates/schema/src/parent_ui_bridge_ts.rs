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

const PARENT_DEV_BRIDGE_URL_ENV_KEY: &str = "VITE_PARENT_DEV_BRIDGE_URL";
const PARENT_AGENT_PROTOCOL_BRIDGE_TYPES_TOKEN: &str = "__PARENT_AGENT_PROTOCOL_BRIDGE_TYPES__";
const PARENT_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN: &str = "__PARENT_ACTIVITY_MEMORY_GRAPH_TYPES__";
const GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN: &str =
    "__GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES__";
const PARENT_TAURI_INTERNAL_WINDOW_KEY: &str = "__TAURI_INTERNALS__";
const PARENT_TYPEOF_UNDEFINED: &str = "undefined";
const GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_TOKEN: &str =
    "__GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS__";
const GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL_TOKEN: &str =
    "__GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL__";

const GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_JSON: &str = r#"{
  "CommandId": "tracking-retention-settings-write-command",
  "SettingsKindRetentionWindow": "retention-window-setting",
  "WriterIntentRef": "tracking-retention-settings-write-retention-window",
  "ReadModelProofRefs": [
    "output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json",
    "output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json"
  ],
  "MutationProofRef": "output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json",
  "LocalServiceStateSnapshotRef": "agent-service-local-retention-settings-state",
  "DurableSettingsStoreRef": "agent-service-local-retention-settings-durable-json",
  "WriteStateAccepted": "service-write-command-accepted",
  "WriteStateRejected": "service-write-command-rejected",
  "AcceptedAt": "2026-06-06T19:50:00Z"
}"#;

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

const ACTIVITY_MEMORY_GRAPH_TYPESCRIPT_TEMPLATE: &str = r#"export type __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus =
  | 'usable'
  | 'degraded'
  | 'stale'
  | 'rejected';

export const __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus = {
  Usable: 'usable',
  Degraded: 'degraded',
  Stale: 'stale',
  Rejected: 'rejected',
} as const;

export type __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind =
  | 'child-profile'
  | 'device'
  | 'browser-url'
  | 'domain'
  | 'video'
  | 'app'
  | 'game'
  | 'activity-session';

export const __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind = {
  ChildProfile: 'child-profile',
  Device: 'device',
  BrowserUrl: 'browser-url',
  Domain: 'domain',
  Video: 'video',
  App: 'app',
  Game: 'game',
  ActivitySession: 'activity-session',
} as const;

export type __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind =
  | 'visited'
  | 'watched'
  | 'played'
  | 'active-during'
  | 'performed-by-child'
  | 'derived-from-evidence';

export const __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind = {
  Visited: 'visited',
  Watched: 'watched',
  Played: 'played',
  ActiveDuring: 'active-during',
  PerformedByChild: 'performed-by-child',
  DerivedFromEvidence: 'derived-from-evidence',
} as const;

export type __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind =
  | 'visited-urls'
  | 'played-games'
  | 'watched-videos'
  | 'activity-by-time-range'
  | 'explain-evidence';

export const __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind = {
  VisitedUrls: 'visited-urls',
  PlayedGames: 'played-games',
  WatchedVideos: 'watched-videos',
  ActivityByTimeRange: 'activity-by-time-range',
  ExplainEvidence: 'explain-evidence',
} as const;

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEvidenceReferenceSnapshot {
  readonly evidenceReferenceId: string;
  readonly kind: string;
  readonly observedAt: string;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphParentActionReferenceSnapshot {
  readonly actionReferenceId: string;
  readonly actor: {
    readonly actorId: string;
    readonly role: string;
  };
  readonly policyVersion: string;
  readonly createdAt: string;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDeviceReferenceSnapshot {
  readonly deviceId: string;
  readonly childProfileId: string | null;
  readonly label: string;
  readonly platform: string;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfileReferenceSnapshot {
  readonly childProfileId: string;
  readonly displayName: string;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot {
  readonly entryStatus: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus;
  readonly sourceEvidenceReferences: readonly __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEvidenceReferenceSnapshot[];
  readonly sourcePolicyVersion: string | null;
  readonly sourceParentActionReferences: readonly __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphParentActionReferenceSnapshot[];
  readonly generatedAt: string;
  readonly expiresAt: string | null;
  readonly confidence: number;
  readonly derivedIndexVersion: string;
  readonly degradedReasons: readonly string[];
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTimeRangeSnapshot {
  readonly observedFrom: string;
  readonly observedUntil: string;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot {
  readonly graphId: string;
  readonly nodeId: string;
  readonly nodeKind: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind;
  readonly label: string;
  readonly childProfile: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDeviceReferenceSnapshot | null;
  readonly trace: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeSnapshot {
  readonly graphId: string;
  readonly edgeId: string;
  readonly edgeKind: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind;
  readonly fromNodeId: string;
  readonly toNodeId: string;
  readonly observedFrom: string;
  readonly observedUntil: string | null;
  readonly durationMs: number | null;
  readonly trace: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQuerySnapshot {
  readonly queryId: string;
  readonly queryKind: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind;
  readonly childProfile: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDeviceReferenceSnapshot;
  readonly timeRange: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTimeRangeSnapshot;
  readonly asOf: string;
  readonly limit: number;
}

export interface __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custody: string;
  readonly capabilityStatus: string;
  readonly query: __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQuerySnapshot;
  readonly readAt: string;
  readonly nodes: readonly __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot[];
  readonly edges: readonly __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeSnapshot[];
  readonly returnedNodeCount: number;
  readonly returnedEdgeCount: number;
  readonly omittedEdgeCount: number;
  readonly degradedReasons: readonly string[];
}

export type __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeId =
  __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot['nodeId'];

export function decode__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDigest(
  digest: string
): __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot | null {
  try {
    return decode__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot(JSON.parse(digest) as unknown);
  } catch {
    return null;
  }
}

export function decode__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot(
  value: unknown
): __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot | null {
  return is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot(value) ? value : null;
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshot {
  if (!is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value)) {
    return false;
  }
  const nodes = value['nodes'];
  const edges = value['edges'];
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshotMetadata(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshotCollections(
      value,
      nodes,
      edges
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshotMetadata(
  value: Record<string, unknown>
): boolean {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value['schemaVersion']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['generatedAt']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['custody']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['capabilityStatus']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQuerySnapshot(value['query']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['readAt']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphReadModelSnapshotCollections(
  value: Record<string, unknown>,
  nodes: unknown,
  edges: unknown
): boolean {
  return (
    Array.isArray(nodes) &&
    nodes.every(is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot) &&
    Array.isArray(edges) &&
    edges.every(is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeSnapshot) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value['returnedNodeCount']) &&
    value['returnedNodeCount'] === nodes.length &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value['returnedEdgeCount']) &&
    value['returnedEdgeCount'] === edges.length &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value['omittedEdgeCount'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQuerySnapshot(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQuerySnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['queryId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind(value['queryKind']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDevice(value['device']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTimeRange(value['timeRange']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['asOf']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value['limit'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['graphId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['nodeId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind(value['nodeKind']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['label']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableDevice(value['device']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeSnapshot(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['graphId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['edgeId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind(value['edgeKind']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['fromNodeId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['toNodeId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['observedFrom']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableString(value['observedUntil']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableCount(value['durationMs']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshot {
  if (!is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value)) {
    return false;
  }
  const evidenceRefs = value['sourceEvidenceReferences'];
  const parentActionRefs = value['sourceParentActionReferences'];
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotMetadata(
      value,
      evidenceRefs,
      parentActionRefs
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotMetadata(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotReferencesValid(
      value,
      evidenceRefs,
      parentActionRefs
    ) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotFieldsValid(value)
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotReferencesValid(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus(value['entryStatus']) &&
    Array.isArray(evidenceRefs) &&
    evidenceRefs.every(is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEvidenceReference) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableString(value['sourcePolicyVersion']) &&
    Array.isArray(parentActionRefs) &&
    parentActionRefs.every(is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphParentActionReference) &&
    (evidenceRefs.length > 0 || value['sourcePolicyVersion'] !== null || parentActionRefs.length > 0)
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTraceSnapshotFieldsValid(
  value: Record<string, unknown>
): boolean {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['generatedAt']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableString(value['expiresAt']) &&
    typeof value['confidence'] === 'number' &&
    value['confidence'] >= 0 &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['derivedIndexVersion']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEvidenceReference(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEvidenceReferenceSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['evidenceReferenceId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['kind']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['observedAt'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphParentActionReference(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphParentActionReferenceSnapshot {
  if (!is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value)) {
    return false;
  }
  const actor = value['actor'];
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['actionReferenceId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(actor) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(actor['actorId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(actor['role']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['policyVersion']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['createdAt'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDevice(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDeviceReferenceSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['deviceId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableString(value['childProfileId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['label']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['platform'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfile(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfileReferenceSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['childProfileId']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['displayName'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTimeRange(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphTimeRangeSnapshot {
  return (
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['observedFrom']) &&
    is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value['observedUntil'])
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableChildProfile(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfileReferenceSnapshot | null {
  return value === null || is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphChildProfile(value);
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableDevice(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDeviceReferenceSnapshot | null {
  return value === null || is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphDevice(value);
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableString(value: unknown): value is string | null {
  return value === null || is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value);
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNullableCount(value: unknown): value is number | null {
  return value === null || is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value);
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind {
  return (
    typeof value === 'string' &&
    Object.values(__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind).includes(
      value as __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNodeKind
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind {
  return (
    typeof value === 'string' &&
    Object.values(__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind).includes(
      value as __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEdgeKind
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus {
  return (
    typeof value === 'string' &&
    Object.values(__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus).includes(
      value as __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphEntryStatus
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind(
  value: unknown
): value is __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind {
  return (
    typeof value === 'string' &&
    Object.values(__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind).includes(
      value as __ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphQueryKind
    )
  );
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphNonNegativeInteger(value: unknown): value is number {
  return typeof value === 'number' && Number.isInteger(value) && value >= 0;
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphStringArray(value: unknown): value is readonly string[] {
  return Array.isArray(value) && value.every(is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString);
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function is__ACTIVITY_MEMORY_GRAPH_PREFIX__ActivityMemoryGraphRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}"#;

const PARENT_UI_BRIDGE_TYPESCRIPT_TEMPLATE: &str = r#"/* generated from crates/schema/src/parent_ui_bridge.rs */

export type ParentRouteId =
  | 'overview'
  | 'assistant'
  | 'start'
  | 'activity'
  | 'browser'
  | 'browser-settings'
  | 'policy'
  | 'policy-apps'
  | 'policy-games'
  | 'policy-screen'
  | 'policy-network'
  | 'policy-tracking'
  | 'policy-remote-screen'
  | 'rule-management'
  | 'schedules'
  | 'approvals'
  | 'enforcement'
  | 'privacy-design'
  | 'memory'
  | 'memory-settings'
  | 'ai-guide'
  | 'ai-runtime'
  | 'api-providers'
  | 'reports-guide'
  | 'screen-analysis'
  | 'app-game-sessions'
  | 'network-activity'
  | 'devices'
  | 'lan-pairing'
  | 'capability-status'
  | 'notifications'
  | 'notification-channels'
  | 'drive-connections'
  | 'export-retention'
  | 'remote-access'
  | 'report-compiler'
  | 'audit-history'
  | 'subscription'
  | 'entitlements'
  | 'platforms-install'
  | 'install-updates'
  | 'diagnostics'
  | 'proof-panels'
  | 'settings-rules'
  | 'app-layout'
  | 'frame-tuner'
  | 'commands'
  | 'events'
  | 'logs';

export const ParentRoute = {
  Overview: 'overview',
  Assistant: 'assistant',
  Start: 'start',
  Activity: 'activity',
  Browser: 'browser',
  BrowserSettings: 'browser-settings',
  Policy: 'policy',
  PolicyApps: 'policy-apps',
  PolicyGames: 'policy-games',
  PolicyScreen: 'policy-screen',
  PolicyNetwork: 'policy-network',
  PolicyTracking: 'policy-tracking',
  PolicyRemoteScreen: 'policy-remote-screen',
  RuleManagement: 'rule-management',
  Schedules: 'schedules',
  Approvals: 'approvals',
  Enforcement: 'enforcement',
  PrivacyDesign: 'privacy-design',
  Memory: 'memory',
  MemorySettings: 'memory-settings',
  AiGuide: 'ai-guide',
  AiRuntime: 'ai-runtime',
  ApiProviders: 'api-providers',
  ReportsGuide: 'reports-guide',
  ScreenAnalysis: 'screen-analysis',
  AppGameSessions: 'app-game-sessions',
  NetworkActivity: 'network-activity',
  Devices: 'devices',
  LanPairing: 'lan-pairing',
  CapabilityStatus: 'capability-status',
  Notifications: 'notifications',
  NotificationChannels: 'notification-channels',
  DriveConnections: 'drive-connections',
  ExportRetention: 'export-retention',
  RemoteAccess: 'remote-access',
  ReportCompiler: 'report-compiler',
  AuditHistory: 'audit-history',
  Subscription: 'subscription',
  Entitlements: 'entitlements',
  PlatformsInstall: 'platforms-install',
  InstallUpdates: 'install-updates',
  Diagnostics: 'diagnostics',
  ProofPanels: 'proof-panels',
  SettingsRules: 'settings-rules',
  AppLayout: 'app-layout',
  FrameTuner: 'frame-tuner',
  Commands: 'commands',
  Events: 'events',
  Logs: 'logs',
} as const;

export const ParentRoutes: readonly ParentRouteId[] = [
  ParentRoute.Overview,
  ParentRoute.Assistant,
  ParentRoute.Start,
  ParentRoute.Activity,
  ParentRoute.Browser,
  ParentRoute.BrowserSettings,
  ParentRoute.Policy,
  ParentRoute.PolicyApps,
  ParentRoute.PolicyGames,
  ParentRoute.PolicyScreen,
  ParentRoute.PolicyNetwork,
  ParentRoute.PolicyTracking,
  ParentRoute.PolicyRemoteScreen,
  ParentRoute.RuleManagement,
  ParentRoute.Schedules,
  ParentRoute.Approvals,
  ParentRoute.Enforcement,
  ParentRoute.PrivacyDesign,
  ParentRoute.Memory,
  ParentRoute.MemorySettings,
  ParentRoute.AiGuide,
  ParentRoute.AiRuntime,
  ParentRoute.ApiProviders,
  ParentRoute.ReportsGuide,
  ParentRoute.ScreenAnalysis,
  ParentRoute.AppGameSessions,
  ParentRoute.NetworkActivity,
  ParentRoute.Devices,
  ParentRoute.LanPairing,
  ParentRoute.CapabilityStatus,
  ParentRoute.Notifications,
  ParentRoute.NotificationChannels,
  ParentRoute.DriveConnections,
  ParentRoute.ExportRetention,
  ParentRoute.RemoteAccess,
  ParentRoute.ReportCompiler,
  ParentRoute.AuditHistory,
  ParentRoute.Subscription,
  ParentRoute.Entitlements,
  ParentRoute.PlatformsInstall,
  ParentRoute.InstallUpdates,
  ParentRoute.Diagnostics,
  ParentRoute.ProofPanels,
  ParentRoute.SettingsRules,
  ParentRoute.AppLayout,
  ParentRoute.FrameTuner,
  ParentRoute.Commands,
  ParentRoute.Events,
  ParentRoute.Logs,
] as const;

export const ParentRouteTitle: Readonly<Record<ParentRouteId, string>> = {
  [ParentRoute.Overview]: 'Overview',
  [ParentRoute.Assistant]: 'Assistant',
  [ParentRoute.Start]: 'Start',
  [ParentRoute.Activity]: 'Activity',
  [ParentRoute.Browser]: 'Browser',
  [ParentRoute.BrowserSettings]: 'Browser settings',
  [ParentRoute.Policy]: 'Policy',
  [ParentRoute.PolicyApps]: 'Policy apps',
  [ParentRoute.PolicyGames]: 'Policy games',
  [ParentRoute.PolicyScreen]: 'Policy screen',
  [ParentRoute.PolicyNetwork]: 'Policy network',
  [ParentRoute.PolicyTracking]: 'Policy tracking',
  [ParentRoute.PolicyRemoteScreen]: 'Policy remote screen',
  [ParentRoute.RuleManagement]: 'Rule management',
  [ParentRoute.Schedules]: 'Schedules',
  [ParentRoute.Approvals]: 'Approvals',
  [ParentRoute.Enforcement]: 'Enforcement',
  [ParentRoute.PrivacyDesign]: 'Privacy design',
  [ParentRoute.Memory]: 'Memory',
  [ParentRoute.MemorySettings]: 'Memory settings',
  [ParentRoute.AiGuide]: 'AI guide',
  [ParentRoute.AiRuntime]: 'AI runtime',
  [ParentRoute.ApiProviders]: 'API providers',
  [ParentRoute.ReportsGuide]: 'Reports guide',
  [ParentRoute.ScreenAnalysis]: 'Screen analysis',
  [ParentRoute.AppGameSessions]: 'App game sessions',
  [ParentRoute.NetworkActivity]: 'Network activity',
  [ParentRoute.Devices]: 'Devices',
  [ParentRoute.LanPairing]: 'LAN pairing',
  [ParentRoute.CapabilityStatus]: 'Capability status',
  [ParentRoute.Notifications]: 'Notifications',
  [ParentRoute.NotificationChannels]: 'Notification channels',
  [ParentRoute.DriveConnections]: 'Drive connections',
  [ParentRoute.ExportRetention]: 'Export retention',
  [ParentRoute.RemoteAccess]: 'Remote access',
  [ParentRoute.ReportCompiler]: 'Report compiler',
  [ParentRoute.AuditHistory]: 'Audit history',
  [ParentRoute.Subscription]: 'Subscription',
  [ParentRoute.Entitlements]: 'Entitlements',
  [ParentRoute.PlatformsInstall]: 'Platforms install',
  [ParentRoute.InstallUpdates]: 'Install updates',
  [ParentRoute.Diagnostics]: 'Diagnostics',
  [ParentRoute.ProofPanels]: 'Proof panels',
  [ParentRoute.SettingsRules]: 'Settings rules',
  [ParentRoute.AppLayout]: 'App layout',
  [ParentRoute.FrameTuner]: 'Frame tuner',
  [ParentRoute.Commands]: 'Commands',
  [ParentRoute.Events]: 'Events',
  [ParentRoute.Logs]: 'Logs',
} as const;

export const ParentDevDiagnosticRoutes: readonly ParentRouteId[] = [
  ParentRoute.Diagnostics,
  ParentRoute.ProofPanels,
  ParentRoute.AppLayout,
  ParentRoute.FrameTuner,
  ParentRoute.Commands,
  ParentRoute.Events,
  ParentRoute.Logs,
] as const;

export const ParentNetworkEvidenceDrawerRoutes: readonly ParentRouteId[] = [
  ParentRoute.Activity,
  ParentRoute.NetworkActivity,
] as const;

export const ParentInlineNetworkEvidenceDrawerRoutes: readonly ParentRouteId[] = [
  ParentRoute.Activity,
] as const;

export const ParentAppGameParentSurfaceRoutes: readonly ParentRouteId[] = [
  ParentRoute.AppGameSessions,
] as const;

export const ParentAiRuntimeRoutes: readonly ParentRouteId[] = [
  ParentRoute.AiRuntime,
] as const;

export const ParentBrowserParentSurfaceRoutes: readonly ParentRouteId[] = [
  ParentRoute.ProofPanels,
] as const;

export const ParentPolicyPreviewRoutes: readonly ParentRouteId[] = [
  ParentRoute.RuleManagement,
  ParentRoute.Schedules,
  ParentRoute.Approvals,
  ParentRoute.Enforcement,
] as const;

export const ParentScreenSettingsRoutes: readonly ParentRouteId[] = [
  ParentRoute.SettingsRules,
] as const;

export const ParentScreenSummaryRoutes: readonly ParentRouteId[] = [
  ParentRoute.ScreenAnalysis,
] as const;

export const ParentSetupFirstRunRoutes: readonly ParentRouteId[] = [
  ParentRoute.Start,
] as const;

export const ParentTrackingStatusRoutes: readonly ParentRouteId[] = [
  ParentRoute.PolicyTracking,
] as const;

export function isParentAiRuntimeRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentAiRuntimeRoutes);
}

export function isParentAppGameParentSurfaceRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentAppGameParentSurfaceRoutes);
}

export function isParentBrowserParentSurfaceRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentBrowserParentSurfaceRoutes);
}

export function isParentNetworkEvidenceDrawerRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentNetworkEvidenceDrawerRoutes);
}

export function isParentInlineNetworkEvidenceDrawerRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentInlineNetworkEvidenceDrawerRoutes);
}

export function isParentPolicyPreviewRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentPolicyPreviewRoutes);
}

export function isParentScreenSettingsRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentScreenSettingsRoutes);
}

export function isParentScreenSummaryRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentScreenSummaryRoutes);
}

export function isParentSetupFirstRunRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentSetupFirstRunRoutes);
}

export function isParentTrackingStatusRoute(route: ParentRouteId): boolean {
  return parentRouteMatches(route, ParentTrackingStatusRoutes);
}

function parentRouteMatches(route: ParentRouteId, routes: readonly ParentRouteId[]): boolean {
  return routes.some((candidate) => candidate === route);
}

export type ParentRouteGroupId = 'monitor' | 'guide' | 'operate' | 'dev-tools';

export const ParentRouteGroup = {
  Monitor: 'monitor',
  Guide: 'guide',
  Operate: 'operate',
  DevTools: 'dev-tools',
} as const;

export const ParentSidebarRouteGroups: readonly ParentRouteGroupId[] = [
  ParentRouteGroup.Monitor,
  ParentRouteGroup.Guide,
  ParentRouteGroup.Operate,
] as const;

export type ParentRouteMetadataEntry = {
  readonly route: ParentRouteId;
  readonly group: ParentRouteGroupId;
  readonly sidebar: boolean;
};

export const ParentRouteMetadata: Readonly<Record<ParentRouteId, ParentRouteMetadataEntry>> = {
  [ParentRoute.Overview]: { route: ParentRoute.Overview, group: ParentRouteGroup.Monitor, sidebar: true },
  [ParentRoute.Assistant]: { route: ParentRoute.Assistant, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.Start]: { route: ParentRoute.Start, group: ParentRouteGroup.Monitor, sidebar: true },
  [ParentRoute.Activity]: { route: ParentRoute.Activity, group: ParentRouteGroup.Monitor, sidebar: true },
  [ParentRoute.Browser]: { route: ParentRoute.Browser, group: ParentRouteGroup.Monitor, sidebar: true },
  [ParentRoute.BrowserSettings]: { route: ParentRoute.BrowserSettings, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Policy]: { route: ParentRoute.Policy, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.PolicyApps]: { route: ParentRoute.PolicyApps, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PolicyGames]: { route: ParentRoute.PolicyGames, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PolicyScreen]: { route: ParentRoute.PolicyScreen, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PolicyNetwork]: { route: ParentRoute.PolicyNetwork, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PolicyTracking]: { route: ParentRoute.PolicyTracking, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PolicyRemoteScreen]: { route: ParentRoute.PolicyRemoteScreen, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.RuleManagement]: { route: ParentRoute.RuleManagement, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Schedules]: { route: ParentRoute.Schedules, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Approvals]: { route: ParentRoute.Approvals, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Enforcement]: { route: ParentRoute.Enforcement, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PrivacyDesign]: { route: ParentRoute.PrivacyDesign, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.Memory]: { route: ParentRoute.Memory, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.MemorySettings]: { route: ParentRoute.MemorySettings, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.AiGuide]: { route: ParentRoute.AiGuide, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.AiRuntime]: { route: ParentRoute.AiRuntime, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.ApiProviders]: { route: ParentRoute.ApiProviders, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.ReportsGuide]: { route: ParentRoute.ReportsGuide, group: ParentRouteGroup.Guide, sidebar: true },
  [ParentRoute.ScreenAnalysis]: { route: ParentRoute.ScreenAnalysis, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.AppGameSessions]: { route: ParentRoute.AppGameSessions, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.NetworkActivity]: { route: ParentRoute.NetworkActivity, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Devices]: { route: ParentRoute.Devices, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.LanPairing]: { route: ParentRoute.LanPairing, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.CapabilityStatus]: { route: ParentRoute.CapabilityStatus, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Notifications]: { route: ParentRoute.Notifications, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.NotificationChannels]: { route: ParentRoute.NotificationChannels, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.DriveConnections]: { route: ParentRoute.DriveConnections, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.ExportRetention]: { route: ParentRoute.ExportRetention, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.RemoteAccess]: { route: ParentRoute.RemoteAccess, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.ReportCompiler]: { route: ParentRoute.ReportCompiler, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.AuditHistory]: { route: ParentRoute.AuditHistory, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Subscription]: { route: ParentRoute.Subscription, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Entitlements]: { route: ParentRoute.Entitlements, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.PlatformsInstall]: { route: ParentRoute.PlatformsInstall, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.InstallUpdates]: { route: ParentRoute.InstallUpdates, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.Diagnostics]: { route: ParentRoute.Diagnostics, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.ProofPanels]: { route: ParentRoute.ProofPanels, group: ParentRouteGroup.DevTools, sidebar: true },
  [ParentRoute.SettingsRules]: { route: ParentRoute.SettingsRules, group: ParentRouteGroup.Operate, sidebar: true },
  [ParentRoute.AppLayout]: { route: ParentRoute.AppLayout, group: ParentRouteGroup.DevTools, sidebar: false },
  [ParentRoute.FrameTuner]: { route: ParentRoute.FrameTuner, group: ParentRouteGroup.DevTools, sidebar: false },
  [ParentRoute.Commands]: { route: ParentRoute.Commands, group: ParentRouteGroup.DevTools, sidebar: true },
  [ParentRoute.Events]: { route: ParentRoute.Events, group: ParentRouteGroup.DevTools, sidebar: true },
  [ParentRoute.Logs]: { route: ParentRoute.Logs, group: ParentRouteGroup.DevTools, sidebar: true },
} as const;

export const ParentSidebarRoutes: readonly ParentRouteId[] = ParentRoutes.filter(
  (route) => ParentRouteMetadata[route].sidebar
);

export type ParentBridgeConnectionState = 'disconnected' | 'connecting' | 'connected' | 'error';

export const ParentBridgeConnectionState = {
  Disconnected: 'disconnected',
  Connecting: 'connecting',
  Connected: 'connected',
  Error: 'error',
} as const;

export type ParentRouteDataSource = 'host-bridge' | 'rust-read-model' | 'dev-diagnostics' | 'unavailable';

export const ParentRouteDataSource = {
  HostBridge: 'host-bridge',
  RustReadModel: 'rust-read-model',
  DevDiagnostics: 'dev-diagnostics',
  Unavailable: 'unavailable',
} as const;

export type ParentPortalTone = 'cyan' | 'gold' | 'purple' | 'red' | 'muted';

export const ParentPortalTone = {
  Cyan: 'cyan',
  Gold: 'gold',
  Purple: 'purple',
  Red: 'red',
  Muted: 'muted',
} as const;

export type ParentPortalParentAccessState =
  | 'active-controller'
  | 'observer-only'
  | 'unauthenticated'
  | 'proof-missing';

export const ParentPortalParentAccessState = {
  ActiveController: 'active-controller',
  ObserverOnly: 'observer-only',
  Unauthenticated: 'unauthenticated',
  ProofMissing: 'proof-missing',
} as const;

export type ParentUnknownRecord = Record<string, unknown>;
export type ParentUiActionPayloadValue = string | number | boolean | null;
export type ParentUiActionPayload = Record<string, ParentUiActionPayloadValue>;

__PARENT_AGENT_PROTOCOL_BRIDGE_TYPES__

__PARENT_ACTIVITY_MEMORY_GRAPH_TYPES__

export const ParentBridgeCommand = {
  LoadRoute: '__PARENT_BRIDGE_COMMAND_LOAD_ROUTE__',
  Dispatch: '__PARENT_BRIDGE_COMMAND_DISPATCH__',
  Subscribe: '__PARENT_BRIDGE_COMMAND_SUBSCRIBE__',
  Unsubscribe: '__PARENT_BRIDGE_COMMAND_UNSUBSCRIBE__',
} as const;

export type ParentBridgeCommandName = (typeof ParentBridgeCommand)[keyof typeof ParentBridgeCommand];

export const ParentDevBridgeRoute = {
  LoadRoute: '__PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE__',
  Dispatch: '__PARENT_DEV_BRIDGE_ROUTE_DISPATCH__',
} as const;

export type ParentDevBridgeRouteName = (typeof ParentDevBridgeRoute)[keyof typeof ParentDevBridgeRoute];

export const ParentUiActionPayloadField = {
  ScreenSettingsRequest: 'screenSettingsRequest',
  ScreenSettingsResponse: 'screenSettingsResponse',
  ScreenSettingsUpdateKind: 'screenSettingsUpdateKind',
} as const;

export const ParentScreenSettingsCommandRuntime = { SchemaVersion: __PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION__, RequestIdPrefix: '__PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX__' } as const;

export const ParentScreenSettingsUpdateKind = { Get: '__PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET__', Replace: '__PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE__' } as const;

export type ParentScreenSettingsUpdateKind =
  (typeof ParentScreenSettingsUpdateKind)[keyof typeof ParentScreenSettingsUpdateKind];
export type ParentScreenSettingsServiceRequestId =
  `${typeof ParentScreenSettingsCommandRuntime.RequestIdPrefix}${number}`;

export const ParentHostBridgeRuntime = {
  SchemaVersion: __PARENT_UI_BRIDGE_SCHEMA_VERSION__,
  DevRouteSubscriptionPollMs: __PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS__,
  RouteHashPrefix: '__PARENT_ROUTE_HASH_PREFIX__',
  RouteHashQuerySeparator: '__PARENT_ROUTE_HASH_QUERY_SEPARATOR__',
  RouteSubscriptionEventPrefix: '__PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX__',
  UrlPathSeparator: '/',
  PostMethod: 'POST',
  JsonContentTypeHeader: 'content-type',
  JsonContentType: 'application/json',
  StringType: 'string',
  TypeofUndefined: '__PARENT_TYPEOF_UNDEFINED__',
  DevBridgeUrlEnvKey: '__PARENT_DEV_BRIDGE_URL_ENV_KEY__',
  TauriCoreModule: '@tauri-apps/api/core',
  TauriEventModule: '@tauri-apps/api/event',
  TauriInternalWindowKey: '__PARENT_TAURI_INTERNAL_WINDOW_KEY__',
  EmptyText: '',
  AgentEndpointPending: 'host-bridge://pending',
  AgentEndpointDevWeb: 'host-bridge://dev-web',
  SeasonLabelLocal: 'LOCAL',
  RouteCapabilityAvailable: 'available',
  RouteCapabilityUnavailable: 'unavailable',
  ParentAccessProofMissing: 'proof-missing',
  HouseholdUnavailable: 'unavailable',
  ChildDeviceUnavailable: 'unavailable',
  UiBridgeCardId: 'ui-bridge',
  ProductRuntimeCardId: 'product-runtime',
  RouteCapabilityCardId: 'route-capability',
  UiBridgeLabel: 'UI bridge',
  ProductRuntimeLabel: 'Product runtime',
  RouteCapabilityLabel: 'Route capability',
  UiBridgeConnected: 'connected',
  ManualRequired: 'manual-required',
  BridgeConnectedDetail: 'The TSX shell is running without a Tauri host.',
  LaunchDesktopDetail: 'Launch the desktop app to load Rust-owned route snapshots.',
  DiagnosticsChromeOnlyDetail: 'Diagnostics chrome only.',
  NoProductReadModelDetail: 'No product read model is attached.',
  NoLiveSnapshotDetail: 'No live parent-route snapshot is currently available.',
  HostBridgeEventId: 'host-bridge-event',
  HostBridgePeerId: 'host-bridge',
  PortalRole: 'portal',
  InfoSeverity: 'info',
  PrimaryAreaBridge: 'Bridge',
  PrimaryAreaRuntime: 'Runtime',
  PrimaryAreaRoute: 'Route',
} as const;

export type ParentRouteHashPath = `${typeof ParentHostBridgeRuntime.RouteHashPrefix}${ParentRouteId}`;
export type ParentRouteHashQueryPath =
  `${typeof ParentHostBridgeRuntime.RouteHashPrefix}${ParentRouteId}${typeof ParentHostBridgeRuntime.RouteHashQuerySeparator}${string}`;

export function parentRouteHashPath(route: ParentRouteId): ParentRouteHashPath {
  return `${ParentHostBridgeRuntime.RouteHashPrefix}${route}`;
}

export function parentRouteHashPathWithQuery(
  route: ParentRouteId,
  query: string
): ParentRouteHashQueryPath {
  return `${ParentHostBridgeRuntime.RouteHashPrefix}${route}${ParentHostBridgeRuntime.RouteHashQuerySeparator}${query}`;
}

export function parentRouteFromHashPath(routeHash: string): ParentRouteId | null {
  const hashWithoutPrefix = routeHash.startsWith(ParentHostBridgeRuntime.RouteHashPrefix)
    ? routeHash.slice(ParentHostBridgeRuntime.RouteHashPrefix.length)
    : routeHash;
  const normalizedHash = hashWithoutPrefix.startsWith(ParentHostBridgeRuntime.UrlPathSeparator)
    ? hashWithoutPrefix.slice(ParentHostBridgeRuntime.UrlPathSeparator.length)
    : hashWithoutPrefix;
  const route =
    normalizedHash.split(ParentHostBridgeRuntime.RouteHashQuerySeparator)[0] ??
    ParentHostBridgeRuntime.EmptyText;
  return isParentRoute(route) ? route : null;
}

export function isParentRoute(value: string): value is ParentRouteId {
  return ParentRoutes.some((route) => route === value);
}

export function parentRouteSubscriptionEventName(
  subscriptionId: string
): string {
  return `${ParentHostBridgeRuntime.RouteSubscriptionEventPrefix}${subscriptionId}`;
}

export function parentDevBridgeHttpError(
  route: ParentDevBridgeRouteName,
  status: number
): string {
  return `parent dev bridge ${route} failed with ${status}`;
}

export function parentDevBridgeDispatchUnavailableMessage(
  parentDevBridgeUrl: string
): string {
  return `Dev web bridge could not reach ${parentDevBridgeUrl}. Launch the desktop app to load Rust-owned route snapshots.`;
}

export function parentDevBridgeUnavailableDetail(parentDevBridgeUrl: string): string {
  return `The Rust-owned dev bridge at ${parentDevBridgeUrl} is unavailable.`;
}

export function presentationOnlyDevWebHostBridgeMessage(): string {
  return 'Dev web host bridge is presentation-only. Launch the desktop app for product data and actions.';
}

export type ParentRouteSubscriptionId = Parameters<typeof parentRouteSubscriptionEventName>[0];
export type ParentRouteSubscriptionEventName = ReturnType<typeof parentRouteSubscriptionEventName>;
export type ParentDevBridgeUrl = Parameters<typeof parentDevBridgeDispatchUnavailableMessage>[0];

export interface ParentRouteContext {
  readonly selectedChildDeviceId?: string | null;
}

export interface ParentPortalRowSnapshot {
  readonly label: string;
  readonly order: number;
  readonly signalScore: number;
  readonly readyCount: number;
  readonly gapCount: number;
  readonly primaryArea: string;
  readonly trend: string;
  readonly tone: ParentPortalTone;
}

export interface ParentPortalShellStatusCardSnapshot {
  readonly id: string;
  readonly label: string;
  readonly value: string;
  readonly detail: string;
  readonly tone: ParentPortalTone;
}

export interface ParentPortalShellStatusSnapshot {
  readonly routeLabel: string;
  readonly parentAccessState: ParentPortalParentAccessState;
  readonly globalConnectionState: string;
  readonly routeCapabilityState: string;
  readonly dataSourceLabel: string;
  readonly cards: readonly ParentPortalShellStatusCardSnapshot[];
}

export interface ParentCommandResultDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentCommandResultProjectionSnapshot {
  readonly projectionKind: string;
  readonly details: readonly ParentCommandResultDetailSnapshot[];
}

export interface ParentRouteEventSnapshot {
  readonly event?: string | null;
  readonly eventId?: string | null;
  readonly correlationId?: string | null;
  readonly sentAt?: string | null;
  readonly sourcePeerId?: string | null;
  readonly sourceRole?: 'portal' | 'agent-service' | 'cloud-relay' | null;
  readonly targetPeerId?: string | null;
  readonly targetRole?: 'portal' | 'agent-service' | 'cloud-relay' | null;
  readonly severity?: string | null;
  readonly payload?: ParentUnknownRecord | null;
  readonly snapshot?: ParentUnknownRecord | null;
  readonly commandResultProjection?: ParentCommandResultProjectionSnapshot | null;
}

export interface ParentLanAddDeviceScanSummarySnapshot {
  readonly schemaVersion: number;
  readonly sourceLabels: readonly string[];
  readonly scannedDeviceCount: number;
  readonly agentDeviceCount: number;
  readonly passiveDeviceCount: number;
  readonly infrastructureDeviceCount: number;
  readonly unsupportedDeviceCount: number;
}

export interface ParentLanPairingDeviceRefSnapshot {
  readonly deviceId: string;
  readonly childProfileId?: string | null;
  readonly label: string;
  readonly platform: string;
  readonly ipAddress?: string | null;
  readonly macAddress?: string | null;
  readonly hostname?: string | null;
  readonly networkInterface?: string | null;
  readonly agentStatus?: string | null;
}

export interface ParentLanServiceIdentityProbeEvidenceSnapshot {
  readonly evidenceKind: string;
  readonly value: string;
}

export interface ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
  readonly schemaVersion: number;
  readonly discoveredAt: string;
  readonly childDevice: ParentLanPairingDeviceRefSnapshot;
  readonly agentPeerId: string;
  readonly routeId: string;
  readonly networkMode: string;
  readonly reachability: string;
  readonly addressRef: string;
  readonly discoveryStatus: string;
  readonly discoveryState: string;
  readonly evidenceSources: readonly string[];
  readonly serviceIdentityProbeEvidence: readonly ParentLanServiceIdentityProbeEvidenceSnapshot[];
  readonly hintSources: readonly string[];
}

export interface ParentLanBrowserAddDevicePairingRequestSnapshot {
  readonly schemaVersion: number;
  readonly challengeId: string;
  readonly childDeviceId: string;
  readonly parentDeviceId: string;
  readonly routeId: string;
  readonly origin: string;
  readonly pairingState: string;
  readonly rejectionReason?: string | null;
  readonly issuedAt: string;
  readonly expiresAt: string;
}

export interface ParentLanDiscoveryEvidenceRecordSnapshot {
  readonly schemaVersion: number;
  readonly evidenceId: string;
  readonly source: string;
  readonly evidenceKind: string;
  readonly deviceId: string;
  readonly value: string;
  readonly normalizedValue: string;
  readonly firstSeenAt: string;
  readonly lastSeenAt: string;
  readonly expiresAt?: string | null;
  readonly confidence: string;
  readonly mergeKey: string;
  readonly note?: string | null;
}

export interface ParentLanCanonicalHouseholdNetworkIdentitySnapshot {
  readonly hostname?: string | null;
  readonly ipAddresses: readonly string[];
  readonly macAddress?: string | null;
  readonly macVendor?: string | null;
  readonly networkInterfaces: readonly string[];
  readonly reachability: string;
  readonly confidence: string;
  readonly staleAt?: string | null;
  readonly offlineAt?: string | null;
  readonly evidenceRecords: readonly ParentLanDiscoveryEvidenceRecordSnapshot[];
}

export interface ParentLanChildAgentInventoryPacketSnapshot {
  readonly deviceName: string;
  readonly platform: string;
  readonly os: string;
  readonly cpuModel?: string | null;
  readonly cpuCores?: string | null;
  readonly memoryTotal?: string | null;
  readonly gpuModel?: string | null;
  readonly gpuDriver?: string | null;
  readonly gpuMemory?: string | null;
  readonly nvidiaSmi?: string | null;
  readonly networkInterfaces: readonly string[];
  readonly capabilities: readonly string[];
  readonly roleState: string;
  readonly routeState: string;
  readonly pairingTrustState: string;
}

export interface ParentLanCanonicalHouseholdDeviceSnapshot {
  readonly schemaVersion: number;
  readonly canonicalDeviceId: string;
  readonly displayName: string;
  readonly classification: string;
  readonly roleBadges: readonly string[];
  readonly enrollable: boolean;
  readonly discoveryState: string;
  readonly trustState: string;
  readonly routeId?: string | null;
  readonly routeState: string;
  readonly networkMode: string;
  readonly sourceLabels: readonly string[];
  readonly networkIdentity: ParentLanCanonicalHouseholdNetworkIdentitySnapshot;
  readonly childAgentInventory?: ParentLanChildAgentInventoryPacketSnapshot | null;
  readonly policyTargetSurfaces: readonly string[];
}

export interface ParentLanTrustedDeviceRegistryEntrySnapshot {
  readonly schemaVersion: number;
  readonly pairingId: string;
  readonly childDevice: ParentLanPairingDeviceRefSnapshot;
  readonly parentDevice: ParentLanPairingDeviceRefSnapshot;
  readonly routeId: string;
  readonly origin: string;
  readonly proofDigest: string;
  readonly trustState: string;
  readonly trustedAt: string;
  readonly expiresAt: string;
  readonly revokedAt?: string | null;
}

export interface ParentLanHouseholdDeviceDecisionSnapshot {
  readonly schemaVersion: number;
  readonly actionId: string;
  readonly actionKind: string;
  readonly canonicalDeviceId: string;
  readonly childProfileId: string | null;
  readonly displayName: string | null;
  readonly deviceKind: string | null;
  readonly parentActorId: string;
  readonly decidedAt: string;
  readonly revokedAt: string | null;
}

export interface ParentLanSignedDiscoveryRelayAdapterRowSnapshot {
  readonly schemaVersion: number;
  readonly adapter: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly sourceConfidence: string;
  readonly custodyLabel: string;
  readonly runtimeOwner: string;
  readonly evidenceLabel: string;
  readonly requiredArtifactSummary?: string | null;
}

export interface ParentLanSignedDiscoveryRelaySignedProofRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly discoveryState: string;
  readonly responseState: string;
  readonly rejectionReason?: string | null;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly evidenceLabel: string;
}

export interface ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly routeId?: string | null;
  readonly discoveryState: string;
  readonly responseState: string;
  readonly rejectionReason?: string | null;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly custodyLabel: string;
  readonly evidenceLabel: string;
}

export interface ParentLanSignedDiscoveryRelayCacheRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly decisionState: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly custodyLabel: string;
  readonly evidenceLabel: string;
}

export interface ParentLanSignedDiscoveryRelaySpineSummarySnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly adapterRows: readonly ParentLanSignedDiscoveryRelayAdapterRowSnapshot[];
  readonly signedProofRows: readonly ParentLanSignedDiscoveryRelaySignedProofRowSnapshot[];
  readonly routeSafetyRows: readonly ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot[];
  readonly relayCacheRows: readonly ParentLanSignedDiscoveryRelayCacheRowSnapshot[];
  readonly manualProofRequired: readonly string[];
  readonly notImplemented: readonly string[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
}

export interface ParentLanSelectedDeviceReadinessSnapshot {
  readonly schemaVersion: number;
  readonly selectedChildDeviceId?: string | null;
  readonly routeId?: string | null;
  readonly pairingId?: string | null;
  readonly trustState: string;
  readonly reachability: string;
  readonly readyForControl: boolean;
  readonly staleAt?: string | null;
  readonly offlineAt?: string | null;
}

export interface ParentLanDiscoveryEventRowSnapshot {
  readonly schemaVersion: number;
  readonly eventId: string;
  readonly eventKind: string;
  readonly occurredAt: string;
  readonly previousEventId?: string | null;
  readonly scanSessionId?: string | null;
  readonly affectedDeviceId?: string | null;
  readonly evidenceId?: string | null;
  readonly summary: string;
}

export interface ParentLanDiscoveryEventHistorySnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly state: string;
  readonly latestEventId?: string | null;
  readonly latestObservedAt?: string | null;
  readonly rows: readonly ParentLanDiscoveryEventRowSnapshot[];
}

export interface ParentLanDiscoverySourceMatrixWorkpackRowSnapshot {
  readonly workpackId: string;
  readonly title: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly status: string;
  readonly readModelVisible: boolean;
  readonly requiredArtifactSummary?: string | null;
}

export interface ParentLanDiscoverySourceMatrixSourceRowSnapshot {
  readonly source: string;
  readonly workpackId: string;
  readonly status: string;
  readonly authority: string;
  readonly runtimePath: string;
  readonly uiSurface: string;
  readonly canConfirmChildAgent: boolean;
  readonly canAssignChildProfile: boolean;
  readonly canControlRoute: boolean;
  readonly requiresSelectedInterface: boolean;
  readonly persistsAcrossRestart: boolean;
  readonly evidenceLabel: string;
  readonly requiredArtifactSummary?: string | null;
}

export interface ParentLanDiscoverySourceMatrixSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly workpackRows: readonly ParentLanDiscoverySourceMatrixWorkpackRowSnapshot[];
  readonly sourceRows: readonly ParentLanDiscoverySourceMatrixSourceRowSnapshot[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
}

export interface ParentLanAddDeviceReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly discoverySource: string;
  readonly addDeviceState: string;
  readonly localServiceDiscoveryState: string;
  readonly physicalHouseholdLanState: string;
  readonly cloudRelayState: string;
  readonly scanSummary: ParentLanAddDeviceScanSummarySnapshot;
  readonly discoveredDevices: readonly ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot[];
  readonly discoveryEventHistory: ParentLanDiscoveryEventHistorySnapshot;
  readonly canonicalHouseholdDevices: readonly ParentLanCanonicalHouseholdDeviceSnapshot[];
  readonly pairingRequests: readonly ParentLanBrowserAddDevicePairingRequestSnapshot[];
  readonly trustedDeviceRegistry: readonly ParentLanTrustedDeviceRegistryEntrySnapshot[];
  readonly householdDeviceDecisions: readonly ParentLanHouseholdDeviceDecisionSnapshot[];
  readonly signedDiscoveryRelaySpine?: ParentLanSignedDiscoveryRelaySpineSummarySnapshot | null;
  readonly lanDiscoverySourceMatrix?: ParentLanDiscoverySourceMatrixSnapshot | null;
  readonly trustedDeviceIds: readonly string[];
  readonly revokedDeviceIds: readonly string[];
  readonly selectedDeviceReadiness: ParentLanSelectedDeviceReadinessSnapshot;
  readonly controllerAuthority: string;
  readonly observerAuthority: string;
  readonly routeRequirementLabels: readonly string[];
  readonly auditCheckLabels: readonly string[];
  readonly honestNonClaims: readonly string[];
}

export interface ParentActivityEvidenceRefSnapshot {
  readonly evidenceId: string;
  readonly kind: string;
  readonly digest?: string | null;
  readonly uri?: string | null;
}

export interface ParentActivityNetworkEndpointSnapshot {
  readonly ip?: string | null;
  readonly port?: number | null;
}

export interface ParentActivityNetworkFlowCountersSnapshot {
  readonly connectionCount: number;
  readonly bytesSent?: number | null;
  readonly bytesReceived?: number | null;
  readonly firstSeenAt?: string | null;
  readonly lastSeenAt?: string | null;
}

export interface ParentActivityNetworkFlowObservationSnapshot {
  readonly schemaVersion: number;
  readonly eventId: string;
  readonly observedAt: string;
  readonly observer: string;
  readonly capabilityStatus: string;
  readonly adapterId: string;
  readonly protocol?: string | null;
  readonly tcpState?: string | null;
  readonly localEndpoint: ParentActivityNetworkEndpointSnapshot;
  readonly destinationEndpoint: ParentActivityNetworkEndpointSnapshot;
  readonly destinationDomain?: string | null;
  readonly domainAttributionStatus: string;
  readonly processAttributionStatus: string;
  readonly processId?: number | null;
  readonly processName?: string | null;
  readonly counters: ParentActivityNetworkFlowCountersSnapshot;
  readonly evidence: readonly ParentActivityEvidenceRefSnapshot[];
}

export interface ParentActivityNetworkFlowReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custody: string;
  readonly limit: number;
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly exportableRows: number;
  readonly capabilityStatus: string;
  readonly latestEventId?: string | null;
  readonly latestObservedAt?: string | null;
  readonly latestTombstoneEventId?: string | null;
  readonly latestTombstoneObservedAt?: string | null;
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly rows: readonly ParentActivityNetworkFlowObservationSnapshot[];
}

export interface ParentActivityTrackingReadModelCountSnapshot {
  readonly value: string;
  readonly count: number;
}

export interface ParentActivityTrackingReadModelRowSnapshot {
  readonly schemaVersion: number;
  readonly eventId: string;
  readonly observedAt: string;
  readonly deviceId: string;
  readonly platform: string;
  readonly observer: string;
  readonly kind: string;
  readonly subjectKind: string;
  readonly subjectId: string;
  readonly subjectDisplayName?: string | null;
  readonly capabilityStatus?: string | null;
  readonly queryVisibility: string;
  readonly deletedAt?: string | null;
  readonly evidenceReferenceIds: readonly string[];
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly evidence: readonly ParentActivityEvidenceRefSnapshot[];
}

export interface ParentActivityTrackingReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custodyLabel: string;
  readonly limit: number;
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly capabilityStatus: string;
  readonly latestEventId?: string | null;
  readonly latestObservedAt?: string | null;
  readonly latestActiveEventId?: string | null;
  readonly latestActiveObservedAt?: string | null;
  readonly latestTombstoneEventId?: string | null;
  readonly latestTombstoneObservedAt?: string | null;
  readonly activeKindCounts: readonly ParentActivityTrackingReadModelCountSnapshot[];
  readonly activeDeviceCounts: readonly ParentActivityTrackingReadModelCountSnapshot[];
  readonly activeCapabilityStatusCounts: readonly ParentActivityTrackingReadModelCountSnapshot[];
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly rows: readonly ParentActivityTrackingReadModelRowSnapshot[];
}

export type ParentActivityTrackingReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export interface ParentActivityTrackingReadModelResultSnapshot {
  readonly ok: boolean;
  readonly reason?: ParentActivityTrackingReadModelFailureReason | null;
  readonly value?: ParentActivityTrackingReadModelSnapshot | null;
}

export interface ParentNetworkRuntimeEventValueSnapshot {
  readonly aiAnalysisRef?: string | null;
  readonly policyDecisionRef?: string | null;
  readonly enforcementResultRef?: string | null;
}

export interface ParentNetworkEvidenceSummarySnapshot {
  readonly analyzerAlertRef?: string | null;
  readonly detectionResultRef?: string | null;
  readonly aiAuditRef?: string | null;
  readonly riskBudgetRef?: string | null;
  readonly policyDecisionRef?: string | null;
  readonly networkEvidenceGrade?: string | null;
  readonly interventionResultRef?: string | null;
}

export interface ParentNetworkRuntimeEventResultSnapshot {
  readonly ok: boolean;
  readonly reason?: string | null;
  readonly eventType?: string | null;
  readonly value?: ParentNetworkRuntimeEventValueSnapshot | null;
}

export interface ParentNetworkRuntimeEventChainStreamSnapshot {
  readonly streamedEventCount?: number | null;
  readonly events: readonly ParentNetworkRuntimeEventResultSnapshot[];
  readonly invalidEventCount: number;
}

export interface ParentPolicyPreviewReadModelSnapshot {
  readonly schemaVersion?: string | null;
  readonly generatedAt?: string | null;
  readonly custody?: string | null;
  readonly limit?: number | null;
  readonly returned: number;
  readonly capabilityStatus?: string | null;
  readonly previewId?: string | null;
  readonly latestEventId?: string | null;
  readonly latestObservedAt?: string | null;
  readonly targetId?: string | null;
  readonly targetType?: string | null;
  readonly targetValue?: string | null;
  readonly evidenceReferenceCount?: number | null;
  readonly parentRuleContextReferenceCount?: number | null;
  readonly parentRuleContextRefIds?: string | null;
  readonly decisionId?: string | null;
  readonly decisionAction?: string | null;
  readonly reasonCodes?: string | null;
  readonly ruleIds?: string | null;
  readonly localAiResultId?: string | null;
  readonly dryRun?: boolean | null;
  readonly enforcementHandoffState?: string | null;
  readonly policyPreviewSaveState?: string | null;
  readonly policyPreviewManualReviewState?: string | null;
  readonly policyPreviewTargetState?: string | null;
  readonly policyPreviewTargetExplanationCode?: string | null;
  readonly policyPreviewFindingKinds?: string | null;
  readonly policySourceStatus?: string | null;
  readonly policySourceSurface?: string | null;
  readonly policyRequestOrigin?: string | null;
  readonly policyAssistantConfirmationState?: string | null;
  readonly policyRequestStatus?: string | null;
  readonly policyApprovalId?: string | null;
  readonly policyOverrideId?: string | null;
  readonly policyReplayOfApprovalId?: string | null;
  readonly policyReviewedByActorId?: string | null;
  readonly policyReviewedByActorRole?: string | null;
  readonly policyReviewedAt?: string | null;
  readonly policyAuditReferenceId?: string | null;
  readonly networkEvidenceGrade?: string | null;
  readonly networkRequestedPolicyAction?: string | null;
  readonly networkMappedPolicyAction?: string | null;
  readonly networkPolicyMappingMode?: string | null;
  readonly networkAdapterActionAuthorized?: boolean | null;
  readonly networkEnforcementCommandAuthorized?: boolean | null;
}

export interface ParentRouteLiveActivitySnapshot {
  readonly recentSummary?: ParentUnknownRecord | null;
  readonly ingestStatus?: ParentUnknownRecord | null;
  readonly activityScreenReadModel?: ParentUnknownRecord | null;
  readonly screenSummaryPanel?: ParentScreenSummaryPanelSnapshot | null;
  readonly browserManagedEvent?: ParentRouteEventSnapshot | null;
  readonly browserManagedStatus?: ParentUnknownRecord | null;
  readonly localAiRuntimeStatusEvent?: ParentRouteEventSnapshot | null;
  readonly lanAiJobEvent?: ParentRouteEventSnapshot | null;
  readonly parentAssistantBoundaryEvent?: ParentRouteEventSnapshot | null;
  readonly activityMemoryGraphReadModel?: ParentActivityMemoryGraphReadModelSnapshot | null;
  readonly networkFlowEvent?: ParentRouteEventSnapshot | null;
  readonly networkFlowReadModel?: ParentActivityNetworkFlowReadModelSnapshot | null;
  readonly networkEvidenceSummary?: ParentNetworkEvidenceSummarySnapshot | null;
  readonly networkRuntimeEventChainStream?: ParentNetworkRuntimeEventChainStreamSnapshot | null;
  readonly lanPairingBrowserDiscoveryEvent?: ParentRouteEventSnapshot | null;
  readonly lanAddDeviceReadModel?: ParentLanAddDeviceReadModelSnapshot | null;
  readonly policyPreviewPanel?: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel?: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel?: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel?: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel?: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel?: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel?: ParentAppGameTimerParentSurfacePanelSnapshot | null;
  readonly browserInterventionEvent?: ParentRouteEventSnapshot | null;
  readonly browserInterventionReadModel?: ParentUnknownRecord | null;
  readonly activityTrackingReadModelEvent?: ParentRouteEventSnapshot | null;
  readonly activityTrackingReadModel?: ParentActivityTrackingReadModelResultSnapshot | null;
  readonly activityTrackingPanel?: ParentTrackingStatusPanelSnapshot | null;
  readonly activityTrackingRetentionSettingsWriteResult?: ParentUnknownRecord | null;
}

export interface ParentPolicyPreviewPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentPolicyPreviewPanelCardSnapshot {
  readonly title: string;
  readonly summary: string;
  readonly details: readonly ParentPolicyPreviewPanelDetailSnapshot[];
}

export interface ParentPolicyPreviewPanelSnapshot {
  readonly title: string;
  readonly body: string;
  readonly summary: string;
  readonly summaryDetails: readonly ParentPolicyPreviewPanelDetailSnapshot[];
  readonly cards: readonly ParentPolicyPreviewPanelCardSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentAppGamePanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentAppGamePanelRowSnapshot {
  readonly title: string;
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
}

export interface ParentAppGamePanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly loadState: string;
  readonly summaryDetails: readonly ParentAppGamePanelDetailSnapshot[];
  readonly rows: readonly ParentAppGamePanelRowSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentAppGameActionRowSnapshot {
  readonly title: string;
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
  readonly actionLabel?: string | null;
  readonly actionPayload?: ParentUnknownRecord | null;
}

export interface ParentAppGameAdapterDispatchPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly preflightPanel: ParentAppGamePanelSnapshot;
  readonly resultPanel: ParentAppGamePanelSnapshot;
  readonly executeActionLabel?: string | null;
}

export interface ParentAppGameTimerParentSurfacePanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly loadState: string;
  readonly summaryDetails: readonly ParentAppGamePanelDetailSnapshot[];
  readonly parentActionRows: readonly ParentAppGamePanelRowSnapshot[];
  readonly parentPreferenceSetupRows: readonly ParentAppGameActionRowSnapshot[];
  readonly rows: readonly ParentAppGamePanelRowSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentAppGameNotificationParentSurfacePanelRowSnapshot {
  readonly key: string;
  readonly title: string;
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
}

export interface ParentAppGameNotificationParentSurfacePanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly state: string;
  readonly summary: string;
  readonly productClaim: string;
  readonly metrics: readonly ParentAppGamePanelDetailSnapshot[];
  readonly rows: readonly ParentAppGameNotificationParentSurfacePanelRowSnapshot[];
  readonly emptyMessage: string;
}

export interface ParentScreenSummaryPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentScreenSummaryPanelRowSnapshot {
  readonly title: string;
  readonly details: readonly ParentScreenSummaryPanelDetailSnapshot[];
}

export interface ParentScreenSummaryPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly loadState: string;
  readonly summaryDetails: readonly ParentScreenSummaryPanelDetailSnapshot[];
  readonly rows: readonly ParentScreenSummaryPanelRowSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentTrackingStatusPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentTrackingStatusPanelCardSnapshot {
  readonly key: string;
  readonly title: string;
  readonly details: readonly ParentTrackingStatusPanelDetailSnapshot[];
}

export interface ParentTrackingStatusPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly summaryCards: readonly ParentTrackingStatusPanelCardSnapshot[];
  readonly cards: readonly ParentTrackingStatusPanelCardSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentSetupFirstRunPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentSetupFirstRunPanelCardSnapshot {
  readonly title: string;
  readonly summary: string;
  readonly details: readonly ParentSetupFirstRunPanelDetailSnapshot[];
}

export interface ParentSetupFirstRunPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly summaryCardTitle: string;
  readonly summary: string;
  readonly summaryDetails: readonly ParentSetupFirstRunPanelDetailSnapshot[];
  readonly cards: readonly ParentSetupFirstRunPanelCardSnapshot[];
  readonly productClaim: string;
}

export interface ParentBrowserPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface ParentBrowserPanelRowSnapshot {
  readonly key: string;
  readonly title: string;
  readonly details: readonly ParentBrowserPanelDetailSnapshot[];
}

export interface ParentBrowserPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly summary: string;
  readonly summaryDetails: readonly ParentBrowserPanelDetailSnapshot[];
  readonly rows: readonly ParentBrowserPanelRowSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface ParentRouteBrowserPanelsSnapshot {
  readonly browserParentExplanation?: ParentBrowserPanelSnapshot | null;
  readonly socialAuditExplanation?: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReport?: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurface?: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDelivery?: ParentBrowserPanelSnapshot | null;
  readonly socialDashboard?: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatus?: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatus?: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatus?: ParentBrowserPanelSnapshot | null;
}

export interface ParentRouteSummary {
  readonly title: string;
  readonly routeCapability: string;
  readonly parentAccess: string;
  readonly household: string;
  readonly childDevice: string;
}

export interface ParentRouteSnapshot {
  readonly schemaVersion: number;
  readonly route: ParentRouteId;
  readonly generatedAt: string;
  readonly seasonLabel: string;
  readonly lastUpdated: string;
  readonly connectionState: ParentBridgeConnectionState;
  readonly commandEnabled: boolean;
  readonly agentEndpoint: string;
  readonly dataSource: ParentRouteDataSource;
  readonly summary: ParentRouteSummary;
  readonly diagnosticPanelsEnabled: boolean;
  readonly parentPortalRows?: readonly ParentPortalRowSnapshot[] | null;
  readonly parentPortalShellStatus?: ParentPortalShellStatusSnapshot | null;
  readonly liveActivity?: ParentRouteLiveActivitySnapshot | null;
  readonly browserPanels?: ParentRouteBrowserPanelsSnapshot | null;
  readonly setupFirstRunPanel?: ParentSetupFirstRunPanelSnapshot | null;
  readonly screenSettingsServiceResponse?: ParentUnknownRecord | null;
}

export type ParentChildDeviceId = NonNullable<ParentRouteContext['selectedChildDeviceId']>;
export type ParentUiDisplayText = ParentPortalRowSnapshot['label'];
export type ParentRouteSummaryState = ParentPortalRowSnapshot['trend'];
export type ParentPortalShellStatusCardId = ParentPortalShellStatusCardSnapshot['id'];
export type ParentRouteEventId = NonNullable<ParentRouteEventSnapshot['eventId']>;
export type ParentRouteEventName = NonNullable<ParentRouteEventSnapshot['event']>;
export type ParentRouteEventSeverity = NonNullable<ParentRouteEventSnapshot['severity']>;
export type ParentRouteTimestamp = ParentRouteSnapshot['generatedAt'];
export type ParentRouteAgentEndpoint = ParentRouteSnapshot['agentEndpoint'];
export type ParentPortalDetailValue = string;
export type ParentPortalClipboardText = string;
export type ParentTrackingStatusProofArtifact = string;

function parseParentUiBridgeNonEmptyText(value: string, field: string): string {
  if (value.trim().length === 0) {
    throw new TypeError(`${field} must be non-empty`);
  }
  return value;
}

export function decodeParentPortalDetailValue(value: string): ParentPortalDetailValue {
  return parseParentUiBridgeNonEmptyText(value, 'ParentPortalDetailValue');
}

export function decodeParentPortalClipboardText(value: string): ParentPortalClipboardText {
  return parseParentUiBridgeNonEmptyText(value, 'ParentPortalClipboardText');
}

export function decodeParentTrackingStatusProofArtifact(
  value: string
): ParentTrackingStatusProofArtifact {
  return parseParentUiBridgeNonEmptyText(value, 'ParentTrackingStatusProofArtifact');
}

export type ParentUiActionKind =
  | 'refresh-route'
  | 'reconnect'
  | 'agent-command-requested'
  | 'policy-request-assistant-preview-confirm-requested'
  | 'lan-pairing-browser-discovery-scan-requested'
  | 'network-flow-read-model-refresh-requested'
  | 'tracking-retention-settings-write-requested'
  | 'screen-settings-get-requested'
  | 'screen-settings-replace-requested'
  | 'app-game-adapter-dispatch-execute-requested'
  | 'app-game-timer-parent-preference-setup-requested';

export const ParentUiActionKind = {
  RefreshRoute: 'refresh-route',
  Reconnect: 'reconnect',
  AgentCommandRequested: 'agent-command-requested',
  PolicyRequestAssistantPreviewConfirmRequested: 'policy-request-assistant-preview-confirm-requested',
  LanPairingBrowserDiscoveryScanRequested: 'lan-pairing-browser-discovery-scan-requested',
  NetworkFlowReadModelRefreshRequested: 'network-flow-read-model-refresh-requested',
  TrackingRetentionSettingsWriteRequested: 'tracking-retention-settings-write-requested',
  ScreenSettingsGetRequested: 'screen-settings-get-requested',
  ScreenSettingsReplaceRequested: 'screen-settings-replace-requested',
  AppGameAdapterDispatchExecuteRequested: 'app-game-adapter-dispatch-execute-requested',
  AppGameTimerParentPreferenceSetupRequested: 'app-game-timer-parent-preference-setup-requested',
} as const;

export type ParentScreenSettingsServiceBridgeAction =
  | typeof ParentUiActionKind.ScreenSettingsGetRequested
  | typeof ParentUiActionKind.ScreenSettingsReplaceRequested;
export type ParentScreenSettingsServiceCommandDraft = { readonly action: ParentScreenSettingsServiceBridgeAction; readonly payload: ParentUiActionPayload; readonly requestId: ParentScreenSettingsServiceRequestId };

export function parentScreenSettingsRequestId(sequence: number): ParentScreenSettingsServiceRequestId {
  return `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}${sequence}` as ParentScreenSettingsServiceRequestId;
}

export function parentScreenSettingsGetCommandDraft(sequence: number): ParentScreenSettingsServiceCommandDraft {
  const requestId = parentScreenSettingsRequestId(sequence);
  const kind = ParentScreenSettingsUpdateKind.Get;
  const request = { schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion, requestId, kind };
  return { action: ParentUiActionKind.ScreenSettingsGetRequested, payload: parentScreenSettingsCommandPayload(request, kind), requestId };
}

export function parentScreenSettingsReplaceCommandDraft(input: { readonly baseSettingVersion: number | null; readonly sequence: number; readonly setting: unknown; }): ParentScreenSettingsServiceCommandDraft {
  const requestId = parentScreenSettingsRequestId(input.sequence);
  const kind = ParentScreenSettingsUpdateKind.Replace;
  const request = { schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion, requestId, kind, baseSettingVersion: input.baseSettingVersion, setting: input.setting };
  return { action: ParentUiActionKind.ScreenSettingsReplaceRequested, payload: parentScreenSettingsCommandPayload(request, kind), requestId };
}

function parentScreenSettingsCommandPayload(request: unknown, kind: ParentScreenSettingsUpdateKind): ParentUiActionPayload {
  return { [ParentUiActionPayloadField.ScreenSettingsRequest]: JSON.stringify(request), [ParentUiActionPayloadField.ScreenSettingsUpdateKind]: kind };
}

export interface ParentUiAction {
  readonly action: ParentUiActionKind;
  readonly route: ParentRouteId;
  readonly command?: string | null;
  readonly payload: ParentUiActionPayload;
}

export interface ParentUiActionResult {
  readonly schemaVersion: number;
  readonly accepted: boolean;
  readonly connectionState: ParentBridgeConnectionState;
  readonly message: string;
  readonly snapshot: ParentRouteSnapshot | null;
  readonly events: readonly ParentRouteEventSnapshot[];
}

export type ParentUiActionCommand = NonNullable<ParentUiAction['command']>;
export type ParentUiActionResultMessage = ParentUiActionResult['message'];

export interface ParentSubscriptionEvent {
  readonly schemaVersion: number;
  readonly route: ParentRouteId;
  readonly snapshot: ParentRouteSnapshot;
  readonly events?: readonly ParentRouteEventSnapshot[] | null;
}

export interface HostBridge {
  loadRoute(route: ParentRouteId, context?: ParentRouteContext): Promise<ParentRouteSnapshot>;
  dispatch(action: ParentUiAction): Promise<ParentUiActionResult>;
  subscribe(
    route: ParentRouteId,
    context: ParentRouteContext,
    onEvent: (event: ParentSubscriptionEvent) => void
  ): Promise<() => void>;
}
"#;

const PARENT_UI_SCREEN_BRIDGE_TYPESCRIPT_TEMPLATE: &str = r#"/* generated from crates/schema/src/parent_ui_bridge.rs */

import {
  ParentScreenSettingsCommandRuntime,
  ParentScreenSettingsUpdateKind,
  type ParentUnknownRecord,
} from './parent-ui-bridge';

export const ParentScreenSettingsUpdateStatus = { Accepted: '__PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED__', Rejected: '__PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED__' } as const;
type ParentScreenSettingsUpdateKindValue = (typeof ParentScreenSettingsUpdateKind)[keyof typeof ParentScreenSettingsUpdateKind];
type ParentScreenSettingsUpdateStatus = (typeof ParentScreenSettingsUpdateStatus)[keyof typeof ParentScreenSettingsUpdateStatus];

export interface ParentScreenAnalysisParentSetting {
  readonly schemaVersion: number; readonly screenAnalysisEnabled: boolean; readonly analysisMode: string;
  readonly cadenceCaptureEnabled: boolean; readonly cadenceSeconds: number; readonly strictModeEnabled: boolean;
  readonly triggerCaptureEnabled: boolean; readonly enabledTriggers: readonly string[]; readonly allowedCaptureScope: string;
  readonly ocrTextEnabled: boolean; readonly ocrTextSnippetLimit: number; readonly redactionMode: string;
  readonly ocrTextRetentionMode: string; readonly credentialSuppressionEnabled: boolean; readonly piiRedactionEnabled: boolean;
  readonly temporaryImageTtlSeconds: number; readonly maxRetryCount: number; readonly deleteAfterSuccess: boolean;
  readonly deleteAfterExpiry: boolean; readonly retainRawImage: boolean; readonly policyUseEnabled: boolean;
  readonly changedByParentRef: string; readonly changedAt: string; readonly settingVersion: number; readonly reason: string | null;
}

interface ParentScreenEvidenceRemoteBoundarySetting {
  readonly schemaVersion: number; readonly parentSettingRef: string; readonly settingVersion: number;
  readonly rawScreenshotRetentionMode: string; readonly liveViewMode: string; readonly rawScreenshotRemoteUploadEnabled: boolean;
  readonly remoteSummaryMode: string; readonly remoteSummaryRedactedOnly: boolean; readonly parentApprovedRemoteSummary: boolean;
  readonly remoteSummaryApprovalRef: string | null; readonly remoteSummaryDestinationCustodyState: string;
  readonly changedByParentRef: string; readonly changedAt: string; readonly reason: string | null;
}

type ParentScreenEvidenceSettingsUiIntentKey =
  | 'disabledLocalSummary'
  | 'observeOnlyLocalSummary'
  | 'strictDryRunLocalSummary'
  | 'approvedRawRetentionLocalTtl';

interface ParentScreenEvidenceSettingsUiIntent {
  readonly intentKey: ParentScreenEvidenceSettingsUiIntentKey; readonly label: string; readonly detail: string;
  readonly setting: ParentScreenAnalysisParentSetting; readonly remoteBoundarySetting: ParentScreenEvidenceRemoteBoundarySetting;
}

export interface ParentScreenEvidenceSettingsUiProof {
  readonly title: string; readonly note: string; readonly intentLegend: string; readonly draftHeading: string;
  readonly draftTriggerHeading: string; readonly retentionHeading: string; readonly serviceCommandHeading: string;
  readonly serviceApplyActionLabel: string; readonly serviceRefreshActionLabel: string; readonly servicePendingStatus: string;
  readonly serviceAcceptedStatus: string; readonly serviceRejectedStatus: string; readonly serviceDisconnectedStatus: string;
  readonly serviceNoResponseStatus: string; readonly validationStatusLabel: string; readonly validationStatusValue: string;
  readonly defaultIntentKey: ParentScreenEvidenceSettingsUiIntentKey; readonly intents: readonly ParentScreenEvidenceSettingsUiIntent[];
}

export interface ParentScreenControlSettingsPortalMetric { readonly label: string; readonly value: string; readonly detail: string; }
export interface ParentScreenControlSettingsPortalGate {
  readonly label: string; readonly status: string; readonly statusText: string; readonly capabilityState: string;
  readonly runtimeOwner: string; readonly detail: string; readonly sourceDocument: string;
}

interface ParentScreenControlSettingsPortalProof {
  readonly title: string; readonly note: string; readonly metrics: readonly ParentScreenControlSettingsPortalMetric[];
  readonly gates: readonly ParentScreenControlSettingsPortalGate[];
}

interface ParentScreenOptionalVisibilityRawRetentionSetting {
  readonly sourceLabel: string; readonly custodyState: string; readonly retentionBehavior: string; readonly [key: string]: unknown;
}
interface ParentScreenOptionalVisibilityLiveViewSetting {
  readonly sourceLabel: string; readonly custodyState: string; readonly transportMode: string; readonly [key: string]: unknown;
}
interface ParentScreenOptionalVisibilityPermissionGate { readonly permissionEvidenceKind: string; readonly [key: string]: unknown; }

export interface ParentScreenOptionalVisibilityCapabilityStatus {
  readonly schemaVersion: number; readonly checkedAt: string; readonly capabilityKind: string; readonly parentSettingRef: string;
  readonly readinessState: string; readonly rawRetentionSetting: ParentScreenOptionalVisibilityRawRetentionSetting | null;
  readonly liveViewSetting: ParentScreenOptionalVisibilityLiveViewSetting | null;
  readonly liveViewPermissionGate: ParentScreenOptionalVisibilityPermissionGate | null;
  readonly runtimeProofRef: string | null; readonly deletionProofRef: string | null; readonly transportProofRef: string | null;
  readonly childDisclosureReady: boolean; readonly childDeviceCapabilityReady: boolean; readonly productModeReady: boolean;
  readonly rawFramesRetained: boolean; readonly rawRemoteUploadAllowed: boolean; readonly remoteInputAllowed: boolean; readonly reason: string;
}

interface ParentScreenOptionalVisibilityCapabilityProof {
  readonly schemaVersion: number; readonly generatedAt: string; readonly proofId: string;
  readonly rows: readonly ParentScreenOptionalVisibilityCapabilityStatus[]; readonly nonClaims: readonly string[];
}

export interface ParentScreenSettingsUpdateResponse {
  readonly schemaVersion: number; readonly requestId: string; readonly kind: ParentScreenSettingsUpdateKindValue;
  readonly status: ParentScreenSettingsUpdateStatus; readonly setting: ParentScreenAnalysisParentSetting | null;
  readonly auditEventId: string | null; readonly rejectionReason: string | null; readonly message: string | null;
}

const ParentScreenEvidenceSettingsWritableUiProofValue = __PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF__ as const satisfies ParentScreenEvidenceSettingsUiProof;
const ParentScreenControlSettingsPortalProofValue = __PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF__ as const satisfies ParentScreenControlSettingsPortalProof;
const ParentScreenOptionalVisibilityCapabilityStatusProofValue = __PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF__ as const satisfies ParentScreenOptionalVisibilityCapabilityProof;

export const ParentScreenOptionalVisibilityCapabilityProofGeneratedAt = '__PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT__' as const;

export function parentScreenEvidenceSettingsWritableUiProof(): ParentScreenEvidenceSettingsUiProof {
  return ParentScreenEvidenceSettingsWritableUiProofValue;
}

export function parentScreenControlSettingsPortalProof(): ParentScreenControlSettingsPortalProof {
  return ParentScreenControlSettingsPortalProofValue;
}

export function parentScreenOptionalVisibilityCapabilityStatusProof(
  generatedAt: typeof ParentScreenOptionalVisibilityCapabilityProofGeneratedAt
): ParentScreenOptionalVisibilityCapabilityProof {
  if (generatedAt !== ParentScreenOptionalVisibilityCapabilityProofGeneratedAt) {
    throw new TypeError('generatedAt must match the Rust-owned screen optional visibility proof timestamp');
  }
  return ParentScreenOptionalVisibilityCapabilityStatusProofValue;
}

export function decodeParentScreenSettingsUpdateResponse(value: unknown): ParentScreenSettingsUpdateResponse | null {
  return isParentScreenSettingsUpdateResponse(value) ? value : null;
}

function isParentScreenSettingsUpdateResponse(value: unknown): value is ParentScreenSettingsUpdateResponse {
  if (!isParentScreenRecord(value)) {
    return false;
  }
  return value['schemaVersion'] === ParentScreenSettingsCommandRuntime.SchemaVersion &&
    isParentScreenNonEmptyString(value['requestId']) &&
    isParentScreenSettingsUpdateKind(value['kind']) &&
    isParentScreenSettingsUpdateStatus(value['status']) &&
    isParentScreenAnalysisParentSettingOrNull(value['setting']) &&
    isParentScreenNullableString(value['auditEventId']) &&
    isParentScreenNullableString(value['rejectionReason']) &&
    isParentScreenNullableString(value['message']);
}

function isParentScreenAnalysisParentSettingOrNull(value: unknown): value is ParentScreenAnalysisParentSetting | null {
  return value === null || isParentScreenAnalysisParentSetting(value);
}

function isParentScreenAnalysisParentSetting(value: unknown): value is ParentScreenAnalysisParentSetting {
  if (!isParentScreenRecord(value)) {
    return false;
  }
  return value['schemaVersion'] === ParentScreenSettingsCommandRuntime.SchemaVersion &&
    typeof value['screenAnalysisEnabled'] === 'boolean' && typeof value['analysisMode'] === 'string' &&
    typeof value['cadenceCaptureEnabled'] === 'boolean' && typeof value['cadenceSeconds'] === 'number' &&
    typeof value['strictModeEnabled'] === 'boolean' && typeof value['triggerCaptureEnabled'] === 'boolean' &&
    Array.isArray(value['enabledTriggers']) && value['enabledTriggers'].every((trigger) => typeof trigger === 'string') &&
    typeof value['allowedCaptureScope'] === 'string' && typeof value['ocrTextEnabled'] === 'boolean' &&
    typeof value['ocrTextSnippetLimit'] === 'number' && typeof value['redactionMode'] === 'string' &&
    typeof value['ocrTextRetentionMode'] === 'string' && typeof value['credentialSuppressionEnabled'] === 'boolean' &&
    typeof value['piiRedactionEnabled'] === 'boolean' && typeof value['temporaryImageTtlSeconds'] === 'number' &&
    typeof value['maxRetryCount'] === 'number' && typeof value['deleteAfterSuccess'] === 'boolean' &&
    typeof value['deleteAfterExpiry'] === 'boolean' && typeof value['retainRawImage'] === 'boolean' &&
    typeof value['policyUseEnabled'] === 'boolean' && isParentScreenNonEmptyString(value['changedByParentRef']) &&
    isParentScreenNonEmptyString(value['changedAt']) && typeof value['settingVersion'] === 'number' &&
    isParentScreenNullableString(value['reason']);
}

function isParentScreenSettingsUpdateKind(value: unknown): value is ParentScreenSettingsUpdateKindValue {
  return value === ParentScreenSettingsUpdateKind.Get || value === ParentScreenSettingsUpdateKind.Replace;
}

function isParentScreenSettingsUpdateStatus(value: unknown): value is ParentScreenSettingsUpdateStatus {
  return value === ParentScreenSettingsUpdateStatus.Accepted || value === ParentScreenSettingsUpdateStatus.Rejected;
}

function isParentScreenNullableString(value: unknown): value is string | null {
  return value === null || typeof value === 'string';
}

function isParentScreenNonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function isParentScreenRecord(value: unknown): value is ParentUnknownRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
"#;

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

const PORTAL_CONTRACTS_TYPESCRIPT_TEMPLATE: &str = r#"/* generated from crates/schema/src/parent_ui_bridge.rs */

import { Schema } from 'effect';

const brandedNonEmptyStringSchema = <Brand extends string>(brand: Brand) =>
  Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));

export const GeneratedPortalRouteLiteral = {
  Overview: 'overview',
  Assistant: 'assistant',
  Start: 'start',
  Activity: 'activity',
  Browser: 'browser',
  BrowserSettings: 'browser-settings',
  Policy: 'policy',
  PolicyApps: 'policy-apps',
  PolicyGames: 'policy-games',
  PolicyScreen: 'policy-screen',
  PolicyNetwork: 'policy-network',
  PolicyTracking: 'policy-tracking',
  PolicyRemoteScreen: 'policy-remote-screen',
  RuleManagement: 'rule-management',
  Schedules: 'schedules',
  Approvals: 'approvals',
  Enforcement: 'enforcement',
  PrivacyDesign: 'privacy-design',
  Memory: 'memory',
  MemorySettings: 'memory-settings',
  AiGuide: 'ai-guide',
  AiRuntime: 'ai-runtime',
  ApiProviders: 'api-providers',
  ReportsGuide: 'reports-guide',
  ScreenAnalysis: 'screen-analysis',
  AppGameSessions: 'app-game-sessions',
  NetworkActivity: 'network-activity',
  Devices: 'devices',
  LanPairing: 'lan-pairing',
  CapabilityStatus: 'capability-status',
  Notifications: 'notifications',
  NotificationChannels: 'notification-channels',
  DriveConnections: 'drive-connections',
  ExportRetention: 'export-retention',
  RemoteAccess: 'remote-access',
  ReportCompiler: 'report-compiler',
  AuditHistory: 'audit-history',
  Subscription: 'subscription',
  Entitlements: 'entitlements',
  PlatformsInstall: 'platforms-install',
  InstallUpdates: 'install-updates',
  Diagnostics: 'diagnostics',
  ProofPanels: 'proof-panels',
  SettingsRules: 'settings-rules',
  AppLayout: 'app-layout',
  FrameTuner: 'frame-tuner',
  Commands: 'commands',
  Events: 'events',
  Logs: 'logs',
} as const;

export type GeneratedPortalRoute =
  (typeof GeneratedPortalRouteLiteral)[keyof typeof GeneratedPortalRouteLiteral];

export const GeneratedPortalRoute = {
  Overview: GeneratedPortalRouteLiteral.Overview,
  Assistant: GeneratedPortalRouteLiteral.Assistant,
  Start: GeneratedPortalRouteLiteral.Start,
  Activity: GeneratedPortalRouteLiteral.Activity,
  Browser: GeneratedPortalRouteLiteral.Browser,
  BrowserSettings: GeneratedPortalRouteLiteral.BrowserSettings,
  Policy: GeneratedPortalRouteLiteral.Policy,
  PolicyApps: GeneratedPortalRouteLiteral.PolicyApps,
  PolicyGames: GeneratedPortalRouteLiteral.PolicyGames,
  PolicyScreen: GeneratedPortalRouteLiteral.PolicyScreen,
  PolicyNetwork: GeneratedPortalRouteLiteral.PolicyNetwork,
  PolicyTracking: GeneratedPortalRouteLiteral.PolicyTracking,
  PolicyRemoteScreen: GeneratedPortalRouteLiteral.PolicyRemoteScreen,
  RuleManagement: GeneratedPortalRouteLiteral.RuleManagement,
  Schedules: GeneratedPortalRouteLiteral.Schedules,
  Approvals: GeneratedPortalRouteLiteral.Approvals,
  Enforcement: GeneratedPortalRouteLiteral.Enforcement,
  PrivacyDesign: GeneratedPortalRouteLiteral.PrivacyDesign,
  Memory: GeneratedPortalRouteLiteral.Memory,
  MemorySettings: GeneratedPortalRouteLiteral.MemorySettings,
  AiGuide: GeneratedPortalRouteLiteral.AiGuide,
  AiRuntime: GeneratedPortalRouteLiteral.AiRuntime,
  ApiProviders: GeneratedPortalRouteLiteral.ApiProviders,
  ReportsGuide: GeneratedPortalRouteLiteral.ReportsGuide,
  ScreenAnalysis: GeneratedPortalRouteLiteral.ScreenAnalysis,
  AppGameSessions: GeneratedPortalRouteLiteral.AppGameSessions,
  NetworkActivity: GeneratedPortalRouteLiteral.NetworkActivity,
  Devices: GeneratedPortalRouteLiteral.Devices,
  LanPairing: GeneratedPortalRouteLiteral.LanPairing,
  CapabilityStatus: GeneratedPortalRouteLiteral.CapabilityStatus,
  Notifications: GeneratedPortalRouteLiteral.Notifications,
  NotificationChannels: GeneratedPortalRouteLiteral.NotificationChannels,
  DriveConnections: GeneratedPortalRouteLiteral.DriveConnections,
  ExportRetention: GeneratedPortalRouteLiteral.ExportRetention,
  RemoteAccess: GeneratedPortalRouteLiteral.RemoteAccess,
  ReportCompiler: GeneratedPortalRouteLiteral.ReportCompiler,
  AuditHistory: GeneratedPortalRouteLiteral.AuditHistory,
  Subscription: GeneratedPortalRouteLiteral.Subscription,
  Entitlements: GeneratedPortalRouteLiteral.Entitlements,
  PlatformsInstall: GeneratedPortalRouteLiteral.PlatformsInstall,
  InstallUpdates: GeneratedPortalRouteLiteral.InstallUpdates,
  Diagnostics: GeneratedPortalRouteLiteral.Diagnostics,
  ProofPanels: GeneratedPortalRouteLiteral.ProofPanels,
  SettingsRules: GeneratedPortalRouteLiteral.SettingsRules,
  AppLayout: GeneratedPortalRouteLiteral.AppLayout,
  FrameTuner: GeneratedPortalRouteLiteral.FrameTuner,
  Commands: GeneratedPortalRouteLiteral.Commands,
  Events: GeneratedPortalRouteLiteral.Events,
  Logs: GeneratedPortalRouteLiteral.Logs,
} as const;

export const GeneratedPortalRouteHashPrefix = '#/' as const;
export const GeneratedPortalRouteHashQuerySeparator = '?' as const;
export type GeneratedPortalRouteHashPath =
  `${typeof GeneratedPortalRouteHashPrefix}${GeneratedPortalRoute}`;
export type GeneratedPortalRouteHashQueryPath =
  `${typeof GeneratedPortalRouteHashPrefix}${GeneratedPortalRoute}${typeof GeneratedPortalRouteHashQuerySeparator}${string}`;

export type GeneratedPortalConnectionState =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'error';

export const GeneratedPortalConnectionState = {
  Disconnected: 'disconnected',
  Connecting: 'connecting',
  Connected: 'connected',
  Error: 'error',
} as const;

type GeneratedPortalRouteEventRole = 'portal' | 'agent-service' | 'cloud-relay';
export type GeneratedPortalRouteEventPayloadRecord = Readonly<Record<string, unknown>>;

export const GeneratedPortalActivityQuerySchemaVersion = 1 as const;

export const GeneratedPortalActivityEventKind = {
  ProcessObserved: 'activity.process.observed',
  WindowFocused: 'activity.window.focused',
  DomainObserved: 'activity.domain.observed',
  UrlObserved: 'activity.url.observed',
  VideoObserved: 'activity.video.observed',
  BrowserInterventionApplied: 'activity.browser.intervention.applied',
  EnforcementAuditRecorded: 'activity.enforcement.audit-recorded',
  DeviceIdleStateObserved: 'activity.device.idle-state-observed',
  ScreenAnalysisSummarized: 'activity.screen.analysis.summarized',
  LocationObserved: 'activity.location.observed',
  TrackingAlertEvaluated: 'activity.tracking.alert.evaluated',
  TrackingGeofenceTransitionEvaluated: 'activity.tracking.geofence-transition.evaluated',
  TrackingExpectedPlaceEvaluated: 'activity.tracking.expected-place.evaluated',
  TrackingChildCheckInResponded: 'activity.tracking.child-check-in.responded',
  TrackingParentNotificationRequested: 'activity.tracking.parent-notification.requested',
  TrackingRetentionDeleted: 'activity.tracking.retention.deleted',
  NetworkRetentionDeleted: 'activity.network.retention.deleted',
} as const;
export type GeneratedPortalActivityEventKind =
  (typeof GeneratedPortalActivityEventKind)[keyof typeof GeneratedPortalActivityEventKind];

export type GeneratedPortalActivityReadModelState =
  | 'ready'
  | 'empty'
  | 'unavailable'
  | 'offline'
  | 'stale'
  | 'permission-required'
  | 'scaffold-only';

export type GeneratedPortalActivitySurfaceReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalActivityReportDocumentSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalActivityHistoricalReportListSnapshot = GeneratedPortalRouteEventPayloadRecord;

export interface GeneratedPortalActivityIngestStatusSnapshot {
  readonly schemaVersion: number;
  readonly databaseReady: boolean;
  readonly eventsIngested: number;
  readonly eventsStored: number;
  readonly duplicateEvents: number;
  readonly lastEventId?: string | null;
}

export interface GeneratedPortalActivityRecentSummarySnapshot {
  readonly schemaVersion: number;
  readonly limit: number;
  readonly returned: number;
  readonly firstObservedAt?: string | null;
  readonly lastObservedAt?: string | null;
  readonly lastEventId?: string | null;
  readonly mostRecentKind?: string | null;
  readonly mostRecentObserver?: string | null;
  readonly mostRecentSubjectKind?: string | null;
  readonly mostRecentSubjectId?: string | null;
  readonly mostRecentSubjectName?: string | null;
}

export type GeneratedPortalBrowserEvidenceReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalBrowserInventoryReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalBrowserManagedSessionStatusSnapshot = GeneratedPortalRouteEventPayloadRecord & {
  readonly managedState?: string | null;
  readonly capabilityStatus?: string | null;
  readonly degradedReason?: string | null;
  readonly browserFamily?: string | null;
  readonly browserChannel?: string | null;
  readonly browserVersion?: string | null;
  readonly managedBrowserSessionId?: string | null;
  readonly profilePathRef?: string | null;
  readonly bridgeEndpointRef?: string | null;
  readonly custodyLabel?: string | null;
  readonly checkedAt?: string | null;
};

export type GeneratedPortalBrowserInterventionRowSnapshot = GeneratedPortalRouteEventPayloadRecord & {
  readonly browserInterventionId?: string | null;
  readonly decisionSource?: string | null;
  readonly policyDecisionId?: string | null;
  readonly interventionActionId?: string | null;
  readonly interventionAuditId?: string | null;
  readonly evidenceReferenceIds?: readonly string[];
  readonly interventionAction?: string | null;
  readonly interventionTargetType?: string | null;
  readonly interventionTargetValue?: string | null;
  readonly requestedUrl?: string | null;
  readonly processId?: number | null;
  readonly interventionMechanism?: string | null;
  readonly interventionOutcome?: string | null;
  readonly browserBoundaryState?: string | null;
  readonly exactUrlClaimState?: string | null;
  readonly unmanagedDetectionState?: string | null;
  readonly unmanagedFallbackAction?: string | null;
  readonly childDeliveryState?: string | null;
  readonly reason?: string | null;
  readonly custodyLabel?: string | null;
};

export type GeneratedPortalBrowserInterventionReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord & {
  readonly returned: number;
  readonly latestObservedAt?: string | null;
  readonly generatedAt?: string | null;
  readonly latestEventId?: string | null;
  readonly managedSessionInterventionCapability?: string | null;
  readonly unmanagedBrowserEnforcement?: string | null;
  readonly rows: readonly GeneratedPortalBrowserInterventionRowSnapshot[];
};

export type GeneratedPortalNetworkRemoteDeliveryStatusSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalNetworkLiveCaptureStatusSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalNetworkPlatformGateStatusSnapshot = GeneratedPortalRouteEventPayloadRecord;
export const GeneratedPortalNetworkRuntimeEventTypeSchema = brandedNonEmptyStringSchema(
  'GeneratedPortalNetworkRuntimeEventType'
);
export type GeneratedPortalNetworkRuntimeEventType = typeof GeneratedPortalNetworkRuntimeEventTypeSchema.Type;
export type GeneratedPortalNetworkRuntimeEventPayload = GeneratedPortalRouteEventPayloadRecord;

export interface GeneratedPortalScreenSummaryPanelDetailSnapshot {
  readonly label: string;
  readonly value: string;
}

export interface GeneratedPortalScreenSummaryPanelRowSnapshot {
  readonly title: string;
  readonly details: readonly GeneratedPortalScreenSummaryPanelDetailSnapshot[];
}

export interface GeneratedPortalScreenSummaryPanelSnapshot {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly loadState: string;
  readonly summaryDetails: readonly GeneratedPortalScreenSummaryPanelDetailSnapshot[];
  readonly rows: readonly GeneratedPortalScreenSummaryPanelRowSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
}

export interface GeneratedPortalLanAddDeviceScanSummarySnapshot {
  readonly schemaVersion: number;
  readonly sourceLabels: readonly string[];
  readonly scannedDeviceCount: number;
  readonly agentDeviceCount: number;
  readonly passiveDeviceCount: number;
  readonly infrastructureDeviceCount: number;
  readonly unsupportedDeviceCount: number;
}

interface GeneratedPortalLanPairingDeviceRefSnapshot {
  readonly deviceId: string;
  readonly childProfileId?: string | null;
  readonly label: string;
  readonly platform: string;
  readonly ipAddress?: string | null;
  readonly macAddress?: string | null;
  readonly hostname?: string | null;
  readonly networkInterface?: string | null;
  readonly agentStatus?: string | null;
}

interface GeneratedPortalLanServiceIdentityProbeEvidenceSnapshot {
  readonly evidenceKind: string;
  readonly value: string;
}

interface GeneratedPortalLanBrowserAddDeviceDiscoveryDeviceSnapshot {
  readonly schemaVersion: number;
  readonly discoveredAt: string;
  readonly childDevice: GeneratedPortalLanPairingDeviceRefSnapshot;
  readonly agentPeerId: string;
  readonly routeId: string;
  readonly networkMode: string;
  readonly reachability: string;
  readonly addressRef: string;
  readonly discoveryStatus: string;
  readonly discoveryState: string;
  readonly evidenceSources: readonly string[];
  readonly serviceIdentityProbeEvidence: readonly GeneratedPortalLanServiceIdentityProbeEvidenceSnapshot[];
  readonly hintSources: readonly string[];
}

interface GeneratedPortalLanBrowserAddDevicePairingRequestSnapshot {
  readonly schemaVersion: number;
  readonly challengeId: string;
  readonly childDeviceId: string;
  readonly parentDeviceId: string;
  readonly routeId: string;
  readonly origin: string;
  readonly pairingState: string;
  readonly rejectionReason?: string | null;
  readonly issuedAt: string;
  readonly expiresAt: string;
}

export interface GeneratedPortalLanDiscoveryEvidenceRecordSnapshot {
  readonly schemaVersion: number;
  readonly evidenceId: string;
  readonly source: string;
  readonly evidenceKind: string;
  readonly deviceId: string;
  readonly value: string;
  readonly normalizedValue: string;
  readonly firstSeenAt: string;
  readonly lastSeenAt: string;
  readonly expiresAt: string | null;
  readonly confidence: string;
  readonly mergeKey: string;
  readonly note: string | null;
}

export interface GeneratedPortalLanCanonicalHouseholdNetworkIdentitySnapshot {
  readonly hostname?: string | null;
  readonly ipAddresses: readonly string[];
  readonly macAddress?: string | null;
  readonly macVendor?: string | null;
  readonly networkInterfaces: readonly string[];
  readonly reachability: string;
  readonly confidence: string;
  readonly staleAt?: string | null;
  readonly offlineAt?: string | null;
  readonly evidenceRecords: readonly GeneratedPortalLanDiscoveryEvidenceRecordSnapshot[];
}

interface GeneratedPortalLanChildAgentInventoryPacketSnapshot {
  readonly deviceName: string;
  readonly platform: string;
  readonly os: string;
  readonly cpuModel?: string | null;
  readonly cpuCores?: string | null;
  readonly memoryTotal?: string | null;
  readonly gpuModel?: string | null;
  readonly gpuDriver?: string | null;
  readonly gpuMemory?: string | null;
  readonly nvidiaSmi?: string | null;
  readonly networkInterfaces: readonly string[];
  readonly capabilities: readonly string[];
  readonly roleState: string;
  readonly routeState: string;
  readonly pairingTrustState: string;
}

export interface GeneratedPortalLanCanonicalHouseholdDeviceSnapshot {
  readonly schemaVersion: number;
  readonly canonicalDeviceId: string;
  readonly displayName: string;
  readonly classification: string;
  readonly roleBadges: readonly string[];
  readonly enrollable: boolean;
  readonly discoveryState: string;
  readonly trustState: string;
  readonly routeId?: string | null;
  readonly routeState: string;
  readonly networkMode: string;
  readonly sourceLabels: readonly string[];
  readonly networkIdentity: GeneratedPortalLanCanonicalHouseholdNetworkIdentitySnapshot;
  readonly childAgentInventory?: GeneratedPortalLanChildAgentInventoryPacketSnapshot | null;
  readonly policyTargetSurfaces: readonly string[];
}

export interface GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot {
  readonly schemaVersion: number;
  readonly pairingId: string;
  readonly childDevice: GeneratedPortalLanPairingDeviceRefSnapshot;
  readonly parentDevice: GeneratedPortalLanPairingDeviceRefSnapshot;
  readonly routeId: string;
  readonly origin: string;
  readonly proofDigest: string;
  readonly trustState: string;
  readonly trustedAt: string;
  readonly expiresAt: string;
  readonly revokedAt: string | null;
}

export interface GeneratedPortalLanHouseholdDeviceDecisionSnapshot {
  readonly schemaVersion: number;
  readonly actionId: string;
  readonly actionKind: string;
  readonly canonicalDeviceId: string;
  readonly childProfileId: string | null;
  readonly displayName: string | null;
  readonly deviceKind: string | null;
  readonly parentActorId: string;
  readonly decidedAt: string;
  readonly revokedAt: string | null;
}

export interface GeneratedPortalLanSignedDiscoveryRelayAdapterRowSnapshot {
  readonly schemaVersion: number;
  readonly adapter: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly sourceConfidence: string;
  readonly custodyLabel: string;
  readonly runtimeOwner: string;
  readonly evidenceLabel: string;
  readonly requiredArtifactSummary?: string | null;
}

export interface GeneratedPortalLanSignedDiscoveryRelaySignedProofRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly discoveryState: string;
  readonly responseState: string;
  readonly rejectionReason?: string | null;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly evidenceLabel: string;
}

export interface GeneratedPortalLanSignedDiscoveryRelayRouteSafetyRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly routeId?: string | null;
  readonly discoveryState: string;
  readonly responseState: string;
  readonly rejectionReason?: string | null;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly custodyLabel: string;
  readonly evidenceLabel: string;
}

export interface GeneratedPortalLanSignedDiscoveryRelayCacheRowSnapshot {
  readonly schemaVersion: number;
  readonly check: string;
  readonly decisionState: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly custodyLabel: string;
  readonly evidenceLabel: string;
}

export interface GeneratedPortalLanSignedDiscoveryRelaySpineSummarySnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly adapterRows: readonly GeneratedPortalLanSignedDiscoveryRelayAdapterRowSnapshot[];
  readonly signedProofRows: readonly GeneratedPortalLanSignedDiscoveryRelaySignedProofRowSnapshot[];
  readonly routeSafetyRows: readonly GeneratedPortalLanSignedDiscoveryRelayRouteSafetyRowSnapshot[];
  readonly relayCacheRows: readonly GeneratedPortalLanSignedDiscoveryRelayCacheRowSnapshot[];
  readonly manualProofRequired: readonly string[];
  readonly notImplemented: readonly string[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
}

export interface GeneratedPortalLanSelectedDeviceReadinessSnapshot {
  readonly schemaVersion: number;
  readonly selectedChildDeviceId?: string | null;
  readonly routeId?: string | null;
  readonly pairingId?: string | null;
  readonly trustState: string;
  readonly reachability: string;
  readonly readyForControl: boolean;
  readonly staleAt?: string | null;
  readonly offlineAt?: string | null;
}

export interface GeneratedPortalLanDiscoveryEventRowSnapshot {
  readonly schemaVersion: number;
  readonly eventId: string;
  readonly eventKind: string;
  readonly occurredAt: string;
  readonly previousEventId: string | null;
  readonly scanSessionId: string | null;
  readonly affectedDeviceId: string | null;
  readonly evidenceId: string | null;
  readonly summary: string;
}

export interface GeneratedPortalLanDiscoveryEventHistorySnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly state: string;
  readonly latestEventId: string | null;
  readonly latestObservedAt: string | null;
  readonly rows: readonly GeneratedPortalLanDiscoveryEventRowSnapshot[];
}

export interface GeneratedPortalLanDiscoverySourceMatrixWorkpackRowSnapshot {
  readonly workpackId: string;
  readonly title: string;
  readonly discoveryState: string;
  readonly proofState: string;
  readonly runtimeOwner: string;
  readonly status: string;
  readonly readModelVisible: boolean;
  readonly requiredArtifactSummary?: string | null;
}

export interface GeneratedPortalLanDiscoverySourceMatrixSourceRowSnapshot {
  readonly source: string;
  readonly workpackId: string;
  readonly status: string;
  readonly authority: string;
  readonly runtimePath: string;
  readonly uiSurface: string;
  readonly canConfirmChildAgent: boolean;
  readonly canAssignChildProfile: boolean;
  readonly canControlRoute: boolean;
  readonly requiresSelectedInterface: boolean;
  readonly persistsAcrossRestart: boolean;
  readonly evidenceLabel: string;
  readonly requiredArtifactSummary?: string | null;
}

export interface GeneratedPortalLanDiscoverySourceMatrixSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly workpackRows: readonly GeneratedPortalLanDiscoverySourceMatrixWorkpackRowSnapshot[];
  readonly sourceRows: readonly GeneratedPortalLanDiscoverySourceMatrixSourceRowSnapshot[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
}

export interface GeneratedPortalLanAddDeviceReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly discoverySource: string;
  readonly addDeviceState: string;
  readonly localServiceDiscoveryState: string;
  readonly physicalHouseholdLanState: string;
  readonly cloudRelayState: string;
  readonly scanSummary: GeneratedPortalLanAddDeviceScanSummarySnapshot;
  readonly discoveredDevices: readonly GeneratedPortalLanBrowserAddDeviceDiscoveryDeviceSnapshot[];
  readonly discoveryEventHistory: GeneratedPortalLanDiscoveryEventHistorySnapshot;
  readonly canonicalHouseholdDevices: readonly GeneratedPortalLanCanonicalHouseholdDeviceSnapshot[];
  readonly pairingRequests: readonly GeneratedPortalLanBrowserAddDevicePairingRequestSnapshot[];
  readonly trustedDeviceRegistry: readonly GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot[];
  readonly householdDeviceDecisions: readonly GeneratedPortalLanHouseholdDeviceDecisionSnapshot[];
  readonly signedDiscoveryRelaySpine: GeneratedPortalLanSignedDiscoveryRelaySpineSummarySnapshot | null;
  readonly lanDiscoverySourceMatrix: GeneratedPortalLanDiscoverySourceMatrixSnapshot | null;
  readonly trustedDeviceIds: readonly string[];
  readonly revokedDeviceIds: readonly string[];
  readonly selectedDeviceReadiness: GeneratedPortalLanSelectedDeviceReadinessSnapshot;
  readonly controllerAuthority: string;
  readonly observerAuthority: string;
  readonly routeRequirementLabels: readonly string[];
  readonly auditCheckLabels: readonly string[];
  readonly honestNonClaims: readonly string[];
}

__PARENT_AGENT_PROTOCOL_BRIDGE_TYPES__

__GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES__

export type GeneratedPortalSocialAlertReportReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalSocialAlertReportParentSurfaceReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalSocialParentNotificationDeliveryReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;
export type GeneratedPortalSocialDashboardUxSnapshot = GeneratedPortalRouteEventPayloadRecord;

export const GeneratedPortalSocialReadModelPayloadField = {
  AlertReport: GeneratedPortalAgentProtocolField.BrowserSocialAlertReportReadModel,
  AlertReportParentSurface: GeneratedPortalAgentProtocolField.BrowserSocialAlertReportParentSurfaceReadModel,
  ParentNotificationDelivery: GeneratedPortalAgentProtocolField.BrowserSocialParentNotificationDeliveryReadModel,
  Dashboard: GeneratedPortalAgentProtocolField.BrowserSocialDashboardReadModel,
} as const;

export type GeneratedPortalSocialReadModelPayloadFieldName =
  (typeof GeneratedPortalSocialReadModelPayloadField)[keyof typeof GeneratedPortalSocialReadModelPayloadField];

interface GeneratedPortalActivityEvidenceRefSnapshot {
  readonly evidenceId: string;
  readonly kind: string;
  readonly digest?: string | null;
  readonly uri?: string | null;
}

interface GeneratedParentActivityTrackingReadModelCountSnapshot {
  readonly value: string;
  readonly count: number;
}

interface GeneratedParentActivityTrackingReadModelRowSnapshot {
  readonly schemaVersion: number;
  readonly eventId: string;
  readonly observedAt: string;
  readonly deviceId: string;
  readonly platform: string;
  readonly observer: string;
  readonly kind: string;
  readonly subjectKind: string;
  readonly subjectId: string;
  readonly subjectDisplayName?: string | null;
  readonly capabilityStatus?: string | null;
  readonly queryVisibility: string;
  readonly deletedAt?: string | null;
  readonly evidenceReferenceIds: readonly string[];
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly evidence: readonly GeneratedPortalActivityEvidenceRefSnapshot[];
}

interface GeneratedParentActivityTrackingReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custodyLabel: string;
  readonly limit: number;
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly capabilityStatus: string;
  readonly latestEventId?: string | null;
  readonly latestObservedAt?: string | null;
  readonly latestActiveEventId?: string | null;
  readonly latestActiveObservedAt?: string | null;
  readonly latestTombstoneEventId?: string | null;
  readonly latestTombstoneObservedAt?: string | null;
  readonly activeKindCounts: readonly GeneratedParentActivityTrackingReadModelCountSnapshot[];
  readonly activeDeviceCounts: readonly GeneratedParentActivityTrackingReadModelCountSnapshot[];
  readonly activeCapabilityStatusCounts: readonly GeneratedParentActivityTrackingReadModelCountSnapshot[];
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly rows: readonly GeneratedParentActivityTrackingReadModelRowSnapshot[];
}

const GeneratedTrackingRetentionSettingsWriteDefaults =
  __GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS__ as const;

type GeneratedTrackingRetentionSettingsWriteKind =
  | typeof GeneratedTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow
  | 'delete-after-alert-setting'
  | 'parent-export-setting'
  | 'remote-sync-disabled-setting'
  | 'remote-ai-disabled-setting';

type GeneratedTrackingRetentionSettingsWriteState =
  | typeof GeneratedTrackingRetentionSettingsWriteDefaults.WriteStateAccepted
  | typeof GeneratedTrackingRetentionSettingsWriteDefaults.WriteStateRejected;

const GeneratedTrackingDeleteAfterAlertResolutionState = {
  DeleteAfterAlertResolved: 'delete-after-alert-resolved',
  RetainAfterAlertResolved: 'retain-after-alert-resolved',
} as const;
type GeneratedTrackingDeleteAfterAlertResolutionState =
  (typeof GeneratedTrackingDeleteAfterAlertResolutionState)[keyof typeof GeneratedTrackingDeleteAfterAlertResolutionState];

const GeneratedTrackingParentExportState = {
  Prepared: 'prepared',
  NotPrepared: 'not-prepared',
} as const;
type GeneratedTrackingParentExportState =
  (typeof GeneratedTrackingParentExportState)[keyof typeof GeneratedTrackingParentExportState];

const GeneratedTrackingRemoteSyncState = { Enabled: 'enabled', Disabled: 'disabled' } as const;
type GeneratedTrackingRemoteSyncState =
  (typeof GeneratedTrackingRemoteSyncState)[keyof typeof GeneratedTrackingRemoteSyncState];

const GeneratedTrackingRemoteAiState = { Enabled: 'enabled', Disabled: 'disabled' } as const;
type GeneratedTrackingRemoteAiState =
  (typeof GeneratedTrackingRemoteAiState)[keyof typeof GeneratedTrackingRemoteAiState];

const GeneratedTrackingDurableSettingsPersistenceState = {
  Persisted: 'persisted',
  NotPersisted: 'not-persisted',
} as const;
type GeneratedTrackingDurableSettingsPersistenceState =
  (typeof GeneratedTrackingDurableSettingsPersistenceState)[keyof typeof GeneratedTrackingDurableSettingsPersistenceState];

const GeneratedTrackingExecutionClaimState = {
  Claimed: 'claimed',
  Unclaimed: 'unclaimed',
} as const;
type GeneratedTrackingExecutionClaimState =
  (typeof GeneratedTrackingExecutionClaimState)[keyof typeof GeneratedTrackingExecutionClaimState];

interface GeneratedTrackingRetentionSettingsWriteResult {
  readonly schemaVersion: number;
  readonly commandId: string;
  readonly settingsKind: GeneratedTrackingRetentionSettingsWriteKind;
  readonly writeState: GeneratedTrackingRetentionSettingsWriteState;
  readonly acceptedAt: string;
  readonly sourceWriterIntentRefs: readonly string[];
  readonly sourceReadModelProofRefs: readonly string[];
  readonly sourceMutationProofRefs: readonly string[];
  readonly appliedRetentionWindowHours: number | null;
  readonly appliedDeleteAfterAlertResolutionState: GeneratedTrackingDeleteAfterAlertResolutionState;
  readonly parentExportState: GeneratedTrackingParentExportState;
  readonly remoteSyncState: GeneratedTrackingRemoteSyncState;
  readonly remoteAiState: GeneratedTrackingRemoteAiState;
  readonly localServiceStateRevision: number | null;
  readonly localServiceStateSnapshotRef: string;
  readonly durableSettingsStoreRef: string;
  readonly durableSettingsPersistenceState: GeneratedTrackingDurableSettingsPersistenceState;
  readonly childConfigResponseState?: 'applied' | 'rejected' | null;
  readonly effectiveTrackingState?: 'enabled' | 'disabled' | 'degraded' | null;
  readonly childConfigAckState?: 'received' | 'missing';
  readonly commandTransportClaimState: GeneratedTrackingExecutionClaimState;
  readonly serviceWritePreflightClaimState: GeneratedTrackingExecutionClaimState;
  readonly serviceMutationExecutionState: GeneratedTrackingExecutionClaimState;
  readonly portalWritableUiClaimState: GeneratedTrackingExecutionClaimState;
  readonly platformRuntimeClaimState: GeneratedTrackingExecutionClaimState;
  readonly childDeviceDeliveryClaimState: GeneratedTrackingExecutionClaimState;
  readonly providerDeliveryClaimState: GeneratedTrackingExecutionClaimState;
  readonly notificationReceiptClaimState: GeneratedTrackingExecutionClaimState;
  readonly physicalDeviceClaimState: GeneratedTrackingExecutionClaimState;
  readonly authorityClaimState: GeneratedTrackingExecutionClaimState;
  readonly productClaimState: GeneratedTrackingExecutionClaimState;
}

const GeneratedTrackingNotificationParentSurfaceHistoryStatus = {
  HistoryIntentReady: 'history-intent-ready',
  ManualActionRequired: 'manual-action-required',
  ProviderUnavailable: 'provider-unavailable',
} as const;
type GeneratedTrackingNotificationParentSurfaceHistoryStatus =
  (typeof GeneratedTrackingNotificationParentSurfaceHistoryStatus)[keyof typeof GeneratedTrackingNotificationParentSurfaceHistoryStatus];

const GeneratedRequiredTrackingNotificationParentSurfaceHistoryNonClaims = [
  'no-rendered-parent-notification-ui',
  'no-parent-preference-mutation-runtime',
  'no-parent-frequency-control-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-authority-proof',
  'no-retry-worker-runtime',
  'no-production-durable-history-storage',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;
type GeneratedTrackingNotificationParentSurfaceHistoryNonClaim =
  (typeof GeneratedRequiredTrackingNotificationParentSurfaceHistoryNonClaims)[number];

interface GeneratedTrackingNotificationParentSurfaceHistoryFamily {
  readonly familyId: string;
}

interface GeneratedTrackingNotificationParentSurfaceHistoryRow {
  readonly historyRowId: string;
  readonly sourceAlertId: string;
  readonly sourceProviderNotificationRowId: string;
  readonly sourceReceiptBoundaryRowId: string;
  readonly sourcePreferencePreflightRowId: string;
  readonly status: GeneratedTrackingNotificationParentSurfaceHistoryStatus;
  readonly sourcePolicyDecisionId: string;
  readonly evidenceRefs: readonly string[];
  readonly notificationStatusRefs: readonly string[];
  readonly reasonCodeRefs: readonly string[];
  readonly providerStatusEntryRef: string;
  readonly providerAttemptRef: string;
  readonly auditRefs: readonly string[];
  readonly providerPreferenceRefs: readonly string[];
  readonly parentPreferenceRequirementRefs: readonly string[];
  readonly quietHoursRequirementRefs: readonly string[];
  readonly receiptRequirementRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly drillInRefs: readonly string[];
  readonly redactedParentSummaryRef: string;
  readonly renderedParentNotificationUiClaimed: false;
  readonly parentPreferenceMutationRuntimeClaimed: false;
  readonly providerDeliveryClaimed: false;
  readonly receiptIngestionRuntimeClaimed: false;
  readonly childDeviceDeliveryClaimed: false;
  readonly mobilePhysicalDeviceProofClaimed: false;
  readonly authorityProofClaimed: false;
}

interface GeneratedTrackingNotificationParentSurfaceHistoryReadModel {
  readonly schemaVersion: 'v0.6';
  readonly proofId: string;
  readonly generatedAt: string;
  readonly family: GeneratedTrackingNotificationParentSurfaceHistoryFamily;
  readonly sourceProviderNotificationProofRef: string;
  readonly sourceReceiptBoundaryProofRef: string;
  readonly sourcePreferencePreflightProofRef: string;
  readonly sourceContractRefs: readonly string[];
  readonly rows: readonly GeneratedTrackingNotificationParentSurfaceHistoryRow[];
  readonly historyIntentReadyCount: number;
  readonly manualActionRequiredCount: number;
  readonly providerUnavailableCount: number;
  readonly proofNonClaims: readonly GeneratedTrackingNotificationParentSurfaceHistoryNonClaim[];
  readonly renderedParentNotificationUiClaimed: false;
  readonly parentPreferenceMutationRuntimeClaimed: false;
  readonly parentFrequencyControlUiClaimed: false;
  readonly quietHoursTimerRuntimeClaimed: false;
  readonly providerDeliveryRuntimeClaimed: false;
  readonly providerReceiptIngestionRuntimeClaimed: false;
  readonly providerCredentialsClaimed: false;
  readonly cloudRoutingClaimed: false;
  readonly childDeviceDeliveryClaimed: false;
  readonly mobilePhysicalDeviceProofClaimed: false;
  readonly authorityProofClaimed: false;
  readonly retryExecutionRuntimeClaimed: false;
  readonly productionDurableHistoryStorageClaimed: false;
  readonly productionDurableOutboxStorageClaimed: false;
  readonly adapterDispatchClaimed: false;
}

const GeneratedDefaultTrackingNotificationParentSurfaceHistoryReadModel =
  __GENERATED_TRACKING_NOTIFICATION_PARENT_SURFACE_HISTORY_READ_MODEL__ as const satisfies GeneratedTrackingNotificationParentSurfaceHistoryReadModel;

type GeneratedParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: TypeError };

const GeneratedTrackingNotificationParentSurfaceHistoryReadModelSchema = {
  parse(value: unknown): GeneratedTrackingNotificationParentSurfaceHistoryReadModel {
    const parsed = decodeGeneratedTrackingNotificationParentSurfaceHistoryReadModel(value);
    if (parsed === null) {
      throw new TypeError('GeneratedTrackingNotificationParentSurfaceHistoryReadModel is invalid');
    }
    return parsed;
  },
  safeParse(value: unknown): GeneratedParseResult<GeneratedTrackingNotificationParentSurfaceHistoryReadModel> {
    const parsed = decodeGeneratedTrackingNotificationParentSurfaceHistoryReadModel(value);
    return parsed === null
      ? { success: false, error: new TypeError('GeneratedTrackingNotificationParentSurfaceHistoryReadModel is invalid') }
      : { success: true, data: parsed };
  },
} as const;

function decodeGeneratedParentActivityTrackingReadModelSnapshot(
  value: unknown
): GeneratedParentActivityTrackingReadModelSnapshot | null {
  if (!generatedRecord(value) || !Array.isArray(value['rows'])) {
    return null;
  }
  return value as unknown as GeneratedParentActivityTrackingReadModelSnapshot;
}

function decodeGeneratedTrackingRetentionSettingsWriteResult(
  value: unknown
): GeneratedTrackingRetentionSettingsWriteResult | null {
  if (!generatedRecord(value) || typeof value['commandId'] !== 'string') {
    return null;
  }
  return value as unknown as GeneratedTrackingRetentionSettingsWriteResult;
}

function decodeGeneratedTrackingNotificationParentSurfaceHistoryReadModel(
  value: unknown
): GeneratedTrackingNotificationParentSurfaceHistoryReadModel | null {
  if (!isGeneratedTrackingNotificationReadModel(value)) {
    return null;
  }
  return value;
}

function isGeneratedTrackingNotificationReadModel(
  value: unknown
): value is GeneratedTrackingNotificationParentSurfaceHistoryReadModel {
  if (!generatedRecord(value) || value['schemaVersion'] !== 'v0.6' || !generatedString(value['proofId'])) {
    return false;
  }
  const rows = value['rows'];
  return (
    generatedTrackingNotificationReadModelIdentity(value as Record<string, unknown>) &&
    generatedTrackingNotificationReadModelRows(rows) &&
    generatedNotificationClaimFlagsStayFalse(value as Record<string, unknown>) &&
    generatedNotificationCountsMatch(value as Record<string, unknown>, rows) &&
    generatedRequiredNotificationNonClaimsPresent(value['proofNonClaims'])
  );
}

function generatedTrackingNotificationReadModelIdentity(
  value: Record<string, unknown>
): boolean {
  return (
    generatedRecord(value['family']) &&
    generatedString(value['family']['familyId']) &&
    generatedString(value['generatedAt']) &&
    generatedString(value['sourceProviderNotificationProofRef']) &&
    generatedString(value['sourceReceiptBoundaryProofRef']) &&
    generatedString(value['sourcePreferencePreflightProofRef']) &&
    generatedStringArray(value['sourceContractRefs'])
  );
}

function generatedTrackingNotificationReadModelRows(
  rows: unknown
): rows is readonly GeneratedTrackingNotificationParentSurfaceHistoryRow[] {
  return Array.isArray(rows) && rows.every(isGeneratedTrackingNotificationRow);
}

function isGeneratedTrackingNotificationRow(
  value: unknown
): value is GeneratedTrackingNotificationParentSurfaceHistoryRow {
  return (
    generatedTrackingNotificationRowIdentity(value as Record<string, unknown>) &&
    generatedTrackingNotificationRowReferences(value as Record<string, unknown>) &&
    generatedNotificationRowClaimFlagsStayFalse(value as Record<string, unknown>)
  );
}

function generatedTrackingNotificationRowIdentity(
  value: Record<string, unknown>
): boolean {
  return (
    generatedRecord(value) &&
    generatedString(value['historyRowId']) &&
    generatedString(value['sourceAlertId']) &&
    generatedString(value['sourceProviderNotificationRowId']) &&
    generatedString(value['sourceReceiptBoundaryRowId']) &&
    generatedString(value['sourcePreferencePreflightRowId']) &&
    generatedNotificationStatus(value['status']) &&
    generatedString(value['sourcePolicyDecisionId']) &&
    generatedString(value['providerStatusEntryRef']) &&
    generatedString(value['providerAttemptRef']) &&
    generatedString(value['redactedParentSummaryRef'])
  );
}

function generatedTrackingNotificationRowReferences(
  value: Record<string, unknown>
): boolean {
  return (
    generatedStringArray(value['evidenceRefs']) &&
    generatedStringArray(value['notificationStatusRefs']) &&
    generatedStringArray(value['reasonCodeRefs']) &&
    generatedStringArray(value['auditRefs']) &&
    generatedStringArray(value['providerPreferenceRefs']) &&
    generatedStringArray(value['parentPreferenceRequirementRefs']) &&
    generatedStringArray(value['quietHoursRequirementRefs']) &&
    generatedStringArray(value['receiptRequirementRefs']) &&
    generatedStringArray(value['manualProofRequirements']) &&
    generatedStringArray(value['drillInRefs'])
  );
}

function generatedNotificationStatus(
  value: unknown
): value is GeneratedTrackingNotificationParentSurfaceHistoryStatus {
  return Object.values(GeneratedTrackingNotificationParentSurfaceHistoryStatus).includes(
    value as GeneratedTrackingNotificationParentSurfaceHistoryStatus
  );
}

function generatedNotificationCountsMatch(
  value: Record<string, unknown>,
  rows: readonly GeneratedTrackingNotificationParentSurfaceHistoryRow[]
): boolean {
  return value['historyIntentReadyCount'] === generatedNotificationStatusCount(rows, GeneratedTrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady) &&
    value['manualActionRequiredCount'] === generatedNotificationStatusCount(rows, GeneratedTrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired) &&
    value['providerUnavailableCount'] === generatedNotificationStatusCount(rows, GeneratedTrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable);
}

function generatedNotificationStatusCount(
  rows: readonly GeneratedTrackingNotificationParentSurfaceHistoryRow[],
  status: GeneratedTrackingNotificationParentSurfaceHistoryStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

function generatedRequiredNotificationNonClaimsPresent(value: unknown): boolean {
  return Array.isArray(value) &&
    GeneratedRequiredTrackingNotificationParentSurfaceHistoryNonClaims.every((claim) => value.includes(claim));
}

function generatedNotificationClaimFlagsStayFalse(value: Record<string, unknown>): boolean {
  return [
    value['renderedParentNotificationUiClaimed'],
    value['parentPreferenceMutationRuntimeClaimed'],
    value['parentFrequencyControlUiClaimed'],
    value['quietHoursTimerRuntimeClaimed'],
    value['providerDeliveryRuntimeClaimed'],
    value['providerReceiptIngestionRuntimeClaimed'],
    value['providerCredentialsClaimed'],
    value['cloudRoutingClaimed'],
    value['childDeviceDeliveryClaimed'],
    value['mobilePhysicalDeviceProofClaimed'],
    value['authorityProofClaimed'],
    value['retryExecutionRuntimeClaimed'],
    value['productionDurableHistoryStorageClaimed'],
    value['productionDurableOutboxStorageClaimed'],
    value['adapterDispatchClaimed'],
  ].every((claim) => claim === false);
}

function generatedNotificationRowClaimFlagsStayFalse(value: Record<string, unknown>): boolean {
  return [
    value['renderedParentNotificationUiClaimed'],
    value['parentPreferenceMutationRuntimeClaimed'],
    value['providerDeliveryClaimed'],
    value['receiptIngestionRuntimeClaimed'],
    value['childDeviceDeliveryClaimed'],
    value['mobilePhysicalDeviceProofClaimed'],
    value['authorityProofClaimed'],
  ].every((claim) => claim === false);
}

function generatedStringArray(value: unknown): value is readonly string[] {
  return Array.isArray(value) && value.every(generatedString);
}

function generatedString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function generatedRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

export const GeneratedPortalTrackingContracts = {
  ActivityTrackingReadModel: {
    decode: decodeGeneratedParentActivityTrackingReadModelSnapshot,
  },
  RetentionSettingsWrite: {
    Defaults: GeneratedTrackingRetentionSettingsWriteDefaults,
    DeleteAfterAlertResolutionState: GeneratedTrackingDeleteAfterAlertResolutionState,
    DurableSettingsPersistenceState: GeneratedTrackingDurableSettingsPersistenceState,
    ExecutionClaimState: GeneratedTrackingExecutionClaimState,
    ParentExportState: GeneratedTrackingParentExportState,
    RemoteAiState: GeneratedTrackingRemoteAiState,
    RemoteSyncState: GeneratedTrackingRemoteSyncState,
    Result: {
      decode: decodeGeneratedTrackingRetentionSettingsWriteResult,
    },
  },
  NotificationParentSurfaceHistory: {
    DefaultReadModel: GeneratedDefaultTrackingNotificationParentSurfaceHistoryReadModel,
    ReadModelSchema: GeneratedTrackingNotificationParentSurfaceHistoryReadModelSchema,
    RequiredNonClaims: GeneratedRequiredTrackingNotificationParentSurfaceHistoryNonClaims,
    Status: GeneratedTrackingNotificationParentSurfaceHistoryStatus,
    decode: decodeGeneratedTrackingNotificationParentSurfaceHistoryReadModel,
  },
} as const;

export interface GeneratedPortalRouteEventSnapshot {
  readonly event?: string | null;
  readonly eventId?: string | null;
  readonly correlationId?: string | null;
  readonly sentAt?: string | null;
  readonly sourcePeerId?: string | null;
  readonly sourceRole?: GeneratedPortalRouteEventRole | null;
  readonly targetPeerId?: string | null;
  readonly targetRole?: GeneratedPortalRouteEventRole | null;
  readonly severity?: string | null;
  readonly payload?: GeneratedPortalRouteEventPayloadRecord | null;
  readonly snapshot?: GeneratedPortalRouteEventPayloadRecord | null;
}

export const GeneratedPortalDevToolUrlSchema = brandedNonEmptyStringSchema('GeneratedPortalDevToolUrl');
export type GeneratedPortalDevToolUrl = typeof GeneratedPortalDevToolUrlSchema.Type;
export const GeneratedPortalDetailValueSchema = brandedNonEmptyStringSchema('GeneratedPortalDetailValue');
export type GeneratedPortalDetailValue = typeof GeneratedPortalDetailValueSchema.Type;
export type GeneratedPortalClipboardText = string;
export const GeneratedTrackingStatusProofArtifactSchema = brandedNonEmptyStringSchema(
  'GeneratedTrackingStatusProofArtifact'
);
export type GeneratedTrackingStatusProofArtifact = typeof GeneratedTrackingStatusProofArtifactSchema.Type;
"#;

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
    ACTIVITY_MEMORY_GRAPH_TYPESCRIPT_TEMPLATE.replace("__ACTIVITY_MEMORY_GRAPH_PREFIX__", prefix)
}

fn trim_generated_trailing_whitespace(value: String) -> String {
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

fn compact_generated_typescript(value: String) -> String {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Normal,
        SingleQuote,
        DoubleQuote,
        Backtick,
        LineComment,
        BlockComment,
    }

    let mut compacted = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    let mut mode = Mode::Normal;
    let mut pending_space = false;

    while let Some(ch) = chars.next() {
        match mode {
            Mode::Normal => match ch {
                '\'' => {
                    if pending_space {
                        compacted.push(' ');
                        pending_space = false;
                    }
                    compacted.push(ch);
                    mode = Mode::SingleQuote;
                }
                '"' => {
                    if pending_space {
                        compacted.push(' ');
                        pending_space = false;
                    }
                    compacted.push(ch);
                    mode = Mode::DoubleQuote;
                }
                '`' => {
                    if pending_space {
                        compacted.push(' ');
                        pending_space = false;
                    }
                    compacted.push(ch);
                    mode = Mode::Backtick;
                }
                '/' if chars.peek() == Some(&'/') => {
                    chars.next();
                    pending_space = true;
                    mode = Mode::LineComment;
                }
                '/' if chars.peek() == Some(&'*') => {
                    chars.next();
                    pending_space = true;
                    mode = Mode::BlockComment;
                }
                ch if ch.is_whitespace() => {
                    pending_space = true;
                }
                _ => {
                    if pending_space {
                        compacted.push(' ');
                        pending_space = false;
                    }
                    compacted.push(ch);
                }
            },
            Mode::SingleQuote => {
                compacted.push(ch);
                if ch == '\\' {
                    if let Some(next) = chars.next() {
                        compacted.push(next);
                    }
                } else if ch == '\'' {
                    mode = Mode::Normal;
                }
            }
            Mode::DoubleQuote => {
                compacted.push(ch);
                if ch == '\\' {
                    if let Some(next) = chars.next() {
                        compacted.push(next);
                    }
                } else if ch == '"' {
                    mode = Mode::Normal;
                }
            }
            Mode::Backtick => {
                compacted.push(ch);
                if ch == '\\' {
                    if let Some(next) = chars.next() {
                        compacted.push(next);
                    }
                } else if ch == '`' {
                    mode = Mode::Normal;
                }
            }
            Mode::LineComment => {
                if ch == '\n' {
                    mode = Mode::Normal;
                    pending_space = true;
                }
            }
            Mode::BlockComment => {
                if ch == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    mode = Mode::Normal;
                    pending_space = true;
                }
            }
        }
    }

    compacted.trim().to_owned()
}

pub fn parent_ui_bridge_typescript() -> String {
    trim_generated_trailing_whitespace(
        PARENT_UI_BRIDGE_TYPESCRIPT_TEMPLATE
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
        PORTAL_CONTRACTS_TYPESCRIPT_TEMPLATE
            .replace(
                PARENT_AGENT_PROTOCOL_BRIDGE_TYPES_TOKEN,
                &generated_portal_agent_protocol_bridge_typescript(),
            )
            .replace(
                GENERATED_PORTAL_ACTIVITY_MEMORY_GRAPH_TYPES_TOKEN,
                &activity_memory_graph_typescript("GeneratedPortal"),
            )
            .replace(
                GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_TOKEN,
                &parent_ui_bridge_json_literal(
                    GENERATED_TRACKING_RETENTION_SETTINGS_WRITE_DEFAULTS_JSON,
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
