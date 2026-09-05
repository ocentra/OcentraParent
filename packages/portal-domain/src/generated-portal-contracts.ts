/* generated from crates/schema/src/parent_ui_bridge.rs */

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

export const GeneratedPortalAgentProtocolRuntime = { SchemaVersion: 1, MessageIdPrefix: "cmd-" } as const; export type GeneratedPortalAgentProtocolPayloadValue = string | number | boolean | null; export type GeneratedPortalAgentProtocolPayload = Readonly<Record<string, GeneratedPortalAgentProtocolPayloadValue>>; export const GeneratedPortalAgentPeerRole = { Portal: "portal", AgentService: "agent-service", CloudRelay: "cloud-relay" } as const; export type GeneratedPortalAgentPeerRole = (typeof GeneratedPortalAgentPeerRole)[keyof typeof GeneratedPortalAgentPeerRole]; export const GeneratedPortalAgentRoute = { Localhost: "localhost", LocalNetwork: "local-network", CloudRelay: "cloud-relay" } as const; export type GeneratedPortalAgentRoute = (typeof GeneratedPortalAgentRoute)[keyof typeof GeneratedPortalAgentRoute]; export interface GeneratedPortalAgentPeer { readonly peerId: string; readonly role: GeneratedPortalAgentPeerRole; } export interface GeneratedPortalAgentMessageTarget { readonly deviceId: string; readonly platform: string; readonly route: GeneratedPortalAgentRoute; } export const GeneratedPortalAgentPeerDefaults = { PortalDev: {"peerId":"portal-dev","role":"portal"} } as const; export const GeneratedPortalAgentTargetDefaults = { LocalhostWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"localhost"}, LocalNetworkWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"local-network"} } as const; export interface GeneratedPortalAgentCommandEnvelope { readonly schemaVersion: number; readonly messageId: string; readonly sentAt: string; readonly source: GeneratedPortalAgentPeer; readonly target: GeneratedPortalAgentMessageTarget; readonly command: GeneratedPortalAgentCommandName; readonly payload: GeneratedPortalAgentProtocolPayload; } export function decodeGeneratedPortalAgentCommandEnvelope(value: unknown): GeneratedPortalAgentCommandEnvelope { const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty string`); } return fieldValue; }; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => { const fieldValue = record[field]; if (typeof fieldValue !== 'number') { throw new TypeError(`${field} must be a number`); } return fieldValue; }; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => { const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return schemaVersion; }; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => { const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned protocol literal`); } return fieldValue as T; }; const readPeer = (candidate: unknown): GeneratedPortalAgentPeer => { if (!isRecord(candidate)) { throw new TypeError('peer must be an object'); } return { peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values(GeneratedPortalAgentPeerRole)) }; }; const readTarget = (candidate: unknown): GeneratedPortalAgentMessageTarget => { if (!isRecord(candidate)) { throw new TypeError('target must be an object'); } return { deviceId: readString(candidate, 'deviceId'), platform: readString(candidate, 'platform'), route: readLiteral(candidate, 'route', Object.values(GeneratedPortalAgentRoute)) }; }; const readPayload = (candidate: unknown): GeneratedPortalAgentProtocolPayload => { if (!isRecord(candidate)) { throw new TypeError('payload must be an object'); } for (const payloadValue of Object.values(candidate)) { if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') { throw new TypeError('payload values must be primitive protocol values'); } } return candidate as GeneratedPortalAgentProtocolPayload; }; if (!isRecord(value)) { throw new TypeError('command envelope must be an object'); } return { schemaVersion: readSchemaVersion(value), messageId: readString(value, 'messageId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readTarget(value['target']), command: readLiteral(value, 'command', Object.values(GeneratedPortalAgentCommand)), payload: readPayload(value['payload']) }; } export const GeneratedPortalAgentProtocolLogLevel = { Trace: "trace", Debug: "debug", Info: "info", Warn: "warn", Error: "error" } as const; export type GeneratedPortalAgentProtocolLogLevel = (typeof GeneratedPortalAgentProtocolLogLevel)[keyof typeof GeneratedPortalAgentProtocolLogLevel]; export interface GeneratedPortalAgentEventEnvelope { readonly schemaVersion: number; readonly eventId: string; readonly correlationId: string; readonly sentAt: string; readonly source: GeneratedPortalAgentPeer; readonly target: GeneratedPortalAgentPeer; readonly event: GeneratedPortalAgentEventName; readonly severity: GeneratedPortalAgentProtocolLogLevel; readonly payload: GeneratedPortalAgentProtocolPayload; readonly snapshot: unknown | null; } export function decodeGeneratedPortalAgentEventEnvelope(value: unknown): GeneratedPortalAgentEventEnvelope { const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty string`); } return fieldValue; }; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => { const fieldValue = record[field]; if (typeof fieldValue !== 'number') { throw new TypeError(`${field} must be a number`); } return fieldValue; }; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => { const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return schemaVersion; }; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => { const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned protocol literal`); } return fieldValue as T; }; const readPeer = (candidate: unknown): GeneratedPortalAgentPeer => { if (!isRecord(candidate)) { throw new TypeError('peer must be an object'); } return { peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values(GeneratedPortalAgentPeerRole)) }; }; const readPayload = (candidate: unknown): GeneratedPortalAgentProtocolPayload => { if (!isRecord(candidate)) { throw new TypeError('payload must be an object'); } for (const payloadValue of Object.values(candidate)) { if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') { throw new TypeError('payload values must be primitive protocol values'); } } return candidate as GeneratedPortalAgentProtocolPayload; }; if (!isRecord(value)) { throw new TypeError('event envelope must be an object'); } return { schemaVersion: readSchemaVersion(value), eventId: readString(value, 'eventId'), correlationId: readString(value, 'correlationId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readPeer(value['target']), event: readLiteral(value, 'event', Object.values(GeneratedPortalAgentEvent)), severity: readLiteral(value, 'severity', Object.values(GeneratedPortalAgentProtocolLogLevel)), payload: readPayload(value['payload']), snapshot: value['snapshot'] ?? null }; } function decodeNonEmptyProtocolString(value: unknown, label: string): string { if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${label} must be a non-empty Rust-owned protocol string`); } return value; } export function decodeGeneratedPortalAgentMessageId(value: unknown): string { return decodeNonEmptyProtocolString(value, 'messageId'); } export function decodeGeneratedPortalAgentTimestamp(value: unknown): string { return decodeNonEmptyProtocolString(value, 'timestamp'); } export function decodeGeneratedPortalSerializedAgentMessage(value: unknown): string { return decodeNonEmptyProtocolString(value, 'serializedMessage'); } export function isGeneratedPortalAgentProtocolLogText(value: unknown): value is string { return typeof value === 'string'; } export const GeneratedPortalAgentProtocolField = { ActivityDigest: "activityDigest", ActivityFamilySources: "activityFamilySources", ActivityReadModel: "activityReadModel", ActivityReadModelKind: "activityReadModelKind", ActivityReportDocument: "activityReportDocument", ActivityReportFrequency: "activityReportFrequency", ActivityReportId: "activityReportId", ActivityReports: "activityReports", ActivitySurfaceState: "activitySurfaceState", ActivityTrackingRetentionSettingsWriteResult: "trackingRetentionSettingsWriteResult", ClaimBoundary: "claimBoundary", DeviceId: "deviceId", EventRef: "eventRef", EventType: "eventType", FamilyId: "familyId", Origin: "origin", Payload: "payload", StartedAt: "startedAt", StaleAt: "staleAt", BrowserSocialAlertReportReadModel: "browserSocialAlertReportReadModel", BrowserSocialAlertReportParentSurfaceReadModel: "browserSocialAlertReportParentSurfaceReadModel", BrowserSocialDashboardReadModel: "browserSocialDashboardReadModel", BrowserSocialParentNotificationDeliveryReadModel: "browserSocialParentNotificationDeliveryReadModel", BrowserRuntimeActionIntentAdapterExecutions: "browserRuntimeActionIntentAdapterExecutions", BrowserRuntimeActionIntentCandidates: "browserRuntimeActionIntentCandidates", BrowserRuntimeActionIntentChildAcceptedEventRefs: "browserRuntimeActionIntentChildAcceptedEventRefs", BrowserRuntimeActionIntentChildAcceptedRows: "browserRuntimeActionIntentChildAcceptedRows", BrowserRuntimeActionIntentChildCommandRefs: "browserRuntimeActionIntentChildCommandRefs", BrowserRuntimeActionIntentChildInterventionExecutions: "browserRuntimeActionIntentChildInterventionExecutions", BrowserRuntimeActionIntentDispatchAttempts: "browserRuntimeActionIntentDispatchAttempts", BrowserRuntimeActionIntentEnforcementExecutions: "browserRuntimeActionIntentEnforcementExecutions", BrowserRuntimeActionIntentHandoffCandidates: "browserRuntimeActionIntentHandoffCandidates", BrowserRuntimeActionIntentHandoffOutboxRefs: "browserRuntimeActionIntentHandoffOutboxRefs", BrowserRuntimeActionIntentHandoffRefs: "browserRuntimeActionIntentHandoffRefs", BrowserRuntimeActionIntentParentReadModelRefs: "browserRuntimeActionIntentParentReadModelRefs", BrowserRuntimeEventChainStream: "browserRuntimeEventChainStream", BrowserRuntimeExactUrlRows: "browserRuntimeExactUrlRows", BrowserRuntimeFailedRows: "browserRuntimeFailedRows", BrowserRuntimeInterventionCommandEvents: "browserRuntimeInterventionCommandEvents", BrowserRuntimeManualRequiredRows: "browserRuntimeManualRequiredRows", BrowserRuntimeObservedRows: "browserRuntimeObservedRows", BrowserRuntimeReadModelProjectionEvents: "browserRuntimeReadModelProjectionEvents", BrowserRuntimeSocialProviderAttemptRefs: "browserRuntimeSocialProviderAttemptRefs", BrowserRuntimeSocialProviderDispatchRequiredRows: "browserRuntimeSocialProviderDispatchRequiredRows", BrowserRuntimeSocialProviderDurableResultRefs: "browserRuntimeSocialProviderDurableResultRefs", BrowserRuntimeSocialProviderDurableRows: "browserRuntimeSocialProviderDurableRows", BrowserRuntimeSocialProviderDurableStoreRefs: "browserRuntimeSocialProviderDurableStoreRefs", BrowserRuntimeSocialProviderManualReceiptRequiredRows: "browserRuntimeSocialProviderManualReceiptRequiredRows", BrowserRuntimeSocialProviderReadModelRefs: "browserRuntimeSocialProviderReadModelRefs", BrowserRuntimeSocialProviderReceiptBoundaryRows: "browserRuntimeSocialProviderReceiptBoundaryRows", BrowserRuntimeSocialProviderReceiptProofRefs: "browserRuntimeSocialProviderReceiptProofRefs", BrowserRuntimeSocialProviderSupportStatusRefs: "browserRuntimeSocialProviderSupportStatusRefs", BrowserRuntimeStreamedEvents: "browserRuntimeStreamedEvents", LanAiJobId: "lanAiJobId", LanAiJobState: "lanAiJobState", LanAiJobStatus: "lanAiJobStatus", LanAiProviderCustodyLabel: "lanAiProviderCustodyLabel", LanAiProviderRoutingState: "lanAiProviderRoutingState", LanControllerLeaseExpiresAt: "controllerLeaseExpiresAt", LanControllerLeaseId: "controllerLeaseId", LanControllerLeaseIssuedAt: "controllerLeaseIssuedAt", LanCanonicalDeviceId: "canonicalDeviceId", LanChildDeviceId: "childDeviceId", LanControllerDeviceId: "controllerDeviceId", LanHouseholdActionId: "householdActionId", LanHouseholdActionKind: "householdActionKind", LanHouseholdActionChildProfileId: "childProfileId", LanHouseholdActionDisplayName: "displayName", LanHouseholdActionDeviceKind: "deviceKind", LanHouseholdActionRevokedAt: "revokedAt", LanIntentId: "intentId", LanIntentKind: "intentKind", LanPairingId: "pairingId", LanParentAuthority: "parentAuthority", LanParentActorId: "parentActorId", LanParentDeviceId: "parentDeviceId", LanProofDigest: "proofDigest", LanRouteId: "routeId", LoadState: "loadState", LocalAiAdapterReadinessState: "readinessState", LocalAiCapabilityFlags: "capabilityFlags", LocalAiDegradedState: "degradedState", LocalAiExecutionState: "executionState", LocalAiModelId: "modelId", LocalAiPrivacyMode: "privacyMode", LocalAiProviderId: "providerId", LocalAiProviderSource: "providerSource", LocalAiResourceClass: "resourceClass", LocalAiRuntimeReferenceId: "runtimeReferenceId", LocalAiUnavailableReason: "unavailableReason", Message: "message", NetworkAndroidVpnServiceGateStatus: "networkAndroidVpnServiceGateStatus", NetworkAppleNetworkExtensionGateStatus: "networkAppleNetworkExtensionGateStatus", NetworkLinuxNftablesLabStatus: "networkLinuxNftablesLabStatus", NetworkLiveCaptureStatus: "networkLiveCaptureStatus", NetworkRuntimeDeadLetters: "networkRuntimeDeadLetters", NetworkRuntimeDeliveredRows: "networkRuntimeDeliveredRows", NetworkRuntimeEnforcementCommandEvents: "networkRuntimeEnforcementCommandEvents", NetworkRuntimeEventChainStream: "networkRuntimeEventChainStream", NetworkRuntimeFailedRows: "networkRuntimeFailedRows", NetworkRuntimeManualRequiredRows: "networkRuntimeManualRequiredRows", NetworkRuntimeObservedRows: "networkRuntimeObservedRows", NetworkRuntimePublishReports: "networkRuntimePublishReports", NetworkRuntimeStoredEvents: "networkRuntimeStoredEvents", NetworkRuntimeStreamedEvents: "networkRuntimeStreamedEvents", NetworkRemoteDeliveryStatus: "networkRemoteDeliveryStatus", NetworkWindowsFirewallLabStatus: "networkWindowsFirewallLabStatus", NetworkWindowsWfpGateStatus: "networkWindowsWfpGateStatus", Online: "online", ParentAssistantAnswerText: "parentAssistantAnswerText", ParentAssistantAnswerState: "parentAssistantAnswerState", ParentAssistantApiAuthorizationState: "parentAssistantApiAuthorizationState", ParentAssistantApiCustodyLabel: "parentAssistantApiCustodyLabel", ParentAssistantApiDeletionState: "parentAssistantApiDeletionState", ParentAssistantApiProviderBoundary: "parentAssistantApiProviderBoundary", ParentAssistantApiRetentionState: "parentAssistantApiRetentionState", ParentAssistantCitationCount: "parentAssistantCitationCount", ParentAssistantEvidenceSummary: "parentAssistantEvidenceSummary", ParentAssistantProviderRoute: "parentAssistantProviderRoute", ParentAssistantRequestId: "parentAssistantRequestId", ParentAssistantQuickActionId: "quickActionId", ParentAssistantPromptTemplateId: "promptTemplateId", ParentAssistantStarterCategory: "starterCategory", ParentAssistantInputText: "inputText", ParentAssistantInputSource: "inputSource", RangeEnd: "rangeEnd", RangeStart: "rangeStart", Reason: "reason", RequestedAt: "requestedAt", Returned: "returned", ScopeKind: "scopeKind", Transport: "transport" } as const; export type GeneratedPortalAgentProtocolFieldName = (typeof GeneratedPortalAgentProtocolField)[keyof typeof GeneratedPortalAgentProtocolField]; export const GeneratedPortalAgentBrowserRuntimeEventType = { EvidenceObserved: "browser.evidence.observed", EvidenceJournaled: "browser.evidence.journaled", AiAnalysisRequested: "browser.ai.analysis.requested", AiAnalysisCompleted: "browser.ai.analysis.completed", PolicyEvaluationRequested: "browser.policy.evaluation.requested", PolicyDecisionCompleted: "browser.policy.decision.completed", InterventionCommandIssued: "browser.intervention.command.issued", InterventionResultObserved: "browser.intervention.result.observed", AuditEntryCommitted: "browser.audit.entry.committed", ReadModelProjected: "browser.read-model.projected" } as const; export type GeneratedPortalAgentBrowserRuntimeEventType = (typeof GeneratedPortalAgentBrowserRuntimeEventType)[keyof typeof GeneratedPortalAgentBrowserRuntimeEventType]; export const GeneratedPortalAgentBrowserRuntimePhase = { EvidenceObserved: "EvidenceObserved", EvidenceJournaled: "EvidenceJournaled", AiAnalysisRequested: "AiAnalysisRequested", AiAnalysisCompleted: "AiAnalysisCompleted", PolicyEvaluationRequested: "PolicyEvaluationRequested", PolicyDecisionCompleted: "PolicyDecisionCompleted", InterventionCommandIssued: "InterventionCommandIssued", InterventionResultObserved: "InterventionResultObserved", AuditEntryCommitted: "AuditEntryCommitted", ReadModelProjected: "ReadModelProjected" } as const; export type GeneratedPortalAgentBrowserRuntimePhase = (typeof GeneratedPortalAgentBrowserRuntimePhase)[keyof typeof GeneratedPortalAgentBrowserRuntimePhase]; export const GeneratedPortalAgentBrowserRuntimeCapabilityStatus = { Available: "available", TabListOnly: "tab-list-only", UnsupportedBrowser: "unsupported-browser", UnmanagedBrowser: "unmanaged-browser", ManagedProfileMissing: "managed-profile-missing", BridgeMissing: "bridge-missing", PermissionLimited: "permission-limited", Stale: "stale", AdapterError: "adapter-error", DisabledByParent: "disabled-by-parent" } as const; export type GeneratedPortalAgentBrowserRuntimeCapabilityStatus = (typeof GeneratedPortalAgentBrowserRuntimeCapabilityStatus)[keyof typeof GeneratedPortalAgentBrowserRuntimeCapabilityStatus]; export const GeneratedPortalAgentBrowserRuntimeCustodyLabel = { ChildDeviceLocal: "child-device-local", LocalNetworkChildAgent: "local-network-child-agent", ParentCache: "parent-cache", ParentOwnedExport: "parent-owned-export", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentBrowserRuntimeCustodyLabel = (typeof GeneratedPortalAgentBrowserRuntimeCustodyLabel)[keyof typeof GeneratedPortalAgentBrowserRuntimeCustodyLabel]; export const GeneratedPortalAgentBrowserRuntimeQueryVisibility = { LiveLocal: "live-local", LiveLan: "live-lan", ParentCache: "parent-cache", ParentOwnedExport: "parent-owned-export", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentBrowserRuntimeQueryVisibility = (typeof GeneratedPortalAgentBrowserRuntimeQueryVisibility)[keyof typeof GeneratedPortalAgentBrowserRuntimeQueryVisibility];
export type GeneratedPortalAgentBrowserRuntimeEventPayload = { readonly phase: GeneratedPortalAgentBrowserRuntimePhase; readonly sourceRef: string; readonly evidenceRef: string; readonly capabilityStatus: GeneratedPortalAgentBrowserRuntimeCapabilityStatus; readonly custodyLabel: GeneratedPortalAgentBrowserRuntimeCustodyLabel; readonly queryVisibility: GeneratedPortalAgentBrowserRuntimeQueryVisibility; readonly degradedReason: string | null; readonly journalRef: string | null; readonly aiRequestRef: string | null; readonly aiAnalysisRef: string | null; readonly policyEvaluationRef: string | null; readonly policyDecisionRef: string | null; readonly policyPreviewId: string | null; readonly assistantActionIntentId: string | null; readonly interventionCommandRef: string | null; readonly interventionResultRef: string | null; readonly auditEntryRef: string | null; readonly readModelRef: string | null; readonly previousPhaseRef: string | null; readonly exactUrlClaimed: boolean; readonly aiAuthority: false; readonly policyAuthority: boolean; readonly dryRun: boolean; readonly adapterDispatchClaimed: boolean; readonly interventionCommandAllowed: boolean; readonly observedAt: string; };
export type GeneratedPortalAgentBrowserRuntimeEventChainEntry = { readonly eventType: GeneratedPortalAgentBrowserRuntimeEventType; readonly eventRef: string; readonly payload: GeneratedPortalAgentBrowserRuntimeEventPayload; };
export type GeneratedPortalAgentBrowserRuntimeEventChainStream = { readonly observedRows: number; readonly streamedEvents: number; readonly failedRows: number; readonly exactUrlRows: number; readonly manualRequiredRows: number; readonly interventionCommandEvents: number; readonly readModelProjectionEvents: number; readonly actionIntentCandidates: number; readonly actionIntentHandoffCandidates: number; readonly actionIntentHandoffOutboxRefs: readonly string[]; readonly actionIntentHandoffRefs: readonly string[]; readonly actionIntentChildAcceptedRows: number; readonly actionIntentChildCommandRefs: readonly string[]; readonly actionIntentChildAcceptedEventRefs: readonly string[]; readonly actionIntentParentReadModelRefs: readonly string[]; readonly actionIntentDispatchAttempts: 0; readonly actionIntentAdapterExecutions: 0; readonly actionIntentChildInterventionExecutions: 0; readonly actionIntentEnforcementExecutions: 0; readonly socialProviderReceiptBoundaryRows: number; readonly socialProviderDispatchRequiredRows: number; readonly socialProviderManualReceiptRequiredRows: number; readonly socialProviderAttemptRefs: readonly string[]; readonly socialProviderReceiptProofRefs: readonly string[]; readonly socialProviderDurableRows: number; readonly socialProviderDurableResultRefs: readonly string[]; readonly socialProviderDurableStoreRefs: readonly string[]; readonly socialProviderReadModelRefs: readonly string[]; readonly socialProviderSupportStatusRefs: readonly string[]; readonly entries: readonly GeneratedPortalAgentBrowserRuntimeEventChainEntry[]; };
export type GeneratedPortalAgentBrowserRuntimeActionIntentCandidate = { readonly eventRef: string; readonly policyPreviewId: string; readonly assistantActionIntentId: string; readonly sourceRef: string; readonly evidenceRef: string; readonly observedAt: string; };
const GeneratedPortalAgentBrowserRuntimePhaseEventType = { [GeneratedPortalAgentBrowserRuntimePhase.EvidenceObserved]: GeneratedPortalAgentBrowserRuntimeEventType.EvidenceObserved, [GeneratedPortalAgentBrowserRuntimePhase.EvidenceJournaled]: GeneratedPortalAgentBrowserRuntimeEventType.EvidenceJournaled, [GeneratedPortalAgentBrowserRuntimePhase.AiAnalysisRequested]: GeneratedPortalAgentBrowserRuntimeEventType.AiAnalysisRequested, [GeneratedPortalAgentBrowserRuntimePhase.AiAnalysisCompleted]: GeneratedPortalAgentBrowserRuntimeEventType.AiAnalysisCompleted, [GeneratedPortalAgentBrowserRuntimePhase.PolicyEvaluationRequested]: GeneratedPortalAgentBrowserRuntimeEventType.PolicyEvaluationRequested, [GeneratedPortalAgentBrowserRuntimePhase.PolicyDecisionCompleted]: GeneratedPortalAgentBrowserRuntimeEventType.PolicyDecisionCompleted, [GeneratedPortalAgentBrowserRuntimePhase.InterventionCommandIssued]: GeneratedPortalAgentBrowserRuntimeEventType.InterventionCommandIssued, [GeneratedPortalAgentBrowserRuntimePhase.InterventionResultObserved]: GeneratedPortalAgentBrowserRuntimeEventType.InterventionResultObserved, [GeneratedPortalAgentBrowserRuntimePhase.AuditEntryCommitted]: GeneratedPortalAgentBrowserRuntimeEventType.AuditEntryCommitted, [GeneratedPortalAgentBrowserRuntimePhase.ReadModelProjected]: GeneratedPortalAgentBrowserRuntimeEventType.ReadModelProjected } as const;
function __GeneratedPortalAgentBrowserRuntimeIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __GeneratedPortalAgentBrowserRuntimeReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string`); } return value; }
function __GeneratedPortalAgentBrowserRuntimeReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string or null`); } return value; }
function __GeneratedPortalAgentBrowserRuntimeReadNumber(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value)) { throw new TypeError(`${field} must be a finite browser runtime number`); } return value; }
function __GeneratedPortalAgentBrowserRuntimeReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a browser runtime boolean`); } return value; }
function __GeneratedPortalAgentBrowserRuntimeReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __GeneratedPortalAgentBrowserRuntimeReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __GeneratedPortalAgentBrowserRuntimeReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __GeneratedPortalAgentBrowserRuntimeReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned browser runtime literal`); } return value as T; }
function __GeneratedPortalAgentBrowserRuntimeReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a browser runtime string array`); } return value as readonly string[]; }
function __GeneratedPortalAgentBrowserRuntimePayloadIsHonest(payload: GeneratedPortalAgentBrowserRuntimeEventPayload): boolean { if (!__GeneratedPortalAgentBrowserRuntimeContextSupportsExactUrl(payload) && payload.exactUrlClaimed) { return false; } if (!__GeneratedPortalAgentBrowserRuntimeUnavailableContextHasReason(payload)) { return false; } if (!payload.exactUrlClaimed && payload.interventionCommandAllowed) { return false; } if (!__GeneratedPortalAgentBrowserRuntimeDryRunHasNoDispatch(payload)) { return false; } if (payload.adapterDispatchClaimed && !payload.interventionCommandAllowed) { return false; } if (!payload.interventionCommandAllowed) { return payload.interventionCommandRef === null && payload.interventionResultRef === null; } return payload.interventionCommandRef !== null && payload.adapterDispatchClaimed; }
function __GeneratedPortalAgentBrowserRuntimeContextSupportsExactUrl(payload: GeneratedPortalAgentBrowserRuntimeEventPayload): boolean { const capabilityAllowsExactUrl = payload.capabilityStatus === GeneratedPortalAgentBrowserRuntimeCapabilityStatus.Available || payload.capabilityStatus === GeneratedPortalAgentBrowserRuntimeCapabilityStatus.TabListOnly; const queryAllowsExactUrl = payload.queryVisibility === GeneratedPortalAgentBrowserRuntimeQueryVisibility.LiveLocal || payload.queryVisibility === GeneratedPortalAgentBrowserRuntimeQueryVisibility.LiveLan; return capabilityAllowsExactUrl && queryAllowsExactUrl && payload.custodyLabel !== GeneratedPortalAgentBrowserRuntimeCustodyLabel.Unavailable; }
function __GeneratedPortalAgentBrowserRuntimeUnavailableContextHasReason(payload: GeneratedPortalAgentBrowserRuntimeEventPayload): boolean { if (payload.queryVisibility !== GeneratedPortalAgentBrowserRuntimeQueryVisibility.Unavailable && payload.capabilityStatus !== GeneratedPortalAgentBrowserRuntimeCapabilityStatus.BridgeMissing && payload.capabilityStatus !== GeneratedPortalAgentBrowserRuntimeCapabilityStatus.Stale && payload.capabilityStatus !== GeneratedPortalAgentBrowserRuntimeCapabilityStatus.AdapterError) { return true; } return payload.degradedReason !== null; }
function __GeneratedPortalAgentBrowserRuntimeDryRunHasNoDispatch(payload: GeneratedPortalAgentBrowserRuntimeEventPayload): boolean { if (!payload.dryRun) { return true; } return !payload.adapterDispatchClaimed && !payload.interventionCommandAllowed && payload.interventionCommandRef === null && payload.interventionResultRef === null; }
function __GeneratedPortalAgentBrowserRuntimeActionIntentCandidatesFromEntries(entries: readonly GeneratedPortalAgentBrowserRuntimeEventChainEntry[]): GeneratedPortalAgentBrowserRuntimeActionIntentCandidate[] { return entries.flatMap((entry) => { const payload = entry.payload; if (payload.phase !== GeneratedPortalAgentBrowserRuntimePhase.PolicyDecisionCompleted || !payload.dryRun || !payload.policyAuthority || payload.policyPreviewId === null || payload.assistantActionIntentId === null) { return []; } return [{ eventRef: entry.eventRef, policyPreviewId: payload.policyPreviewId, assistantActionIntentId: payload.assistantActionIntentId, sourceRef: payload.sourceRef, evidenceRef: payload.evidenceRef, observedAt: payload.observedAt }]; }); }
function __GeneratedPortalAgentBrowserRuntimeActionIntentChildStatusIsHonest(stream: GeneratedPortalAgentBrowserRuntimeEventChainStream): boolean { return stream.actionIntentChildCommandRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentChildAcceptedEventRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentParentReadModelRefs.length === stream.actionIntentChildAcceptedRows; }
function __GeneratedPortalAgentBrowserRuntimeSocialProviderReceiptRefsAreEmpty(stream: GeneratedPortalAgentBrowserRuntimeEventChainStream): boolean { return stream.socialProviderAttemptRefs.length === 0 && stream.socialProviderReceiptProofRefs.length === 0 && stream.socialProviderDurableRows === 0 && stream.socialProviderDurableResultRefs.length === 0 && stream.socialProviderDurableStoreRefs.length === 0 && stream.socialProviderReadModelRefs.length === 0 && stream.socialProviderSupportStatusRefs.length === 0; }
function __GeneratedPortalAgentBrowserRuntimeSocialProviderReceiptStateIsHonest(stream: GeneratedPortalAgentBrowserRuntimeEventChainStream): boolean { if (stream.socialProviderReceiptBoundaryRows !== stream.socialProviderDispatchRequiredRows + stream.socialProviderManualReceiptRequiredRows) { return false; } if (stream.socialProviderDispatchRequiredRows === 0) { return __GeneratedPortalAgentBrowserRuntimeSocialProviderReceiptRefsAreEmpty(stream); } return stream.socialProviderAttemptRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderReceiptProofRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableRows === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableResultRefs.length === stream.socialProviderDurableRows && stream.socialProviderDurableStoreRefs.length === stream.socialProviderDurableRows && stream.socialProviderReadModelRefs.length === stream.socialProviderDurableRows && stream.socialProviderSupportStatusRefs.length === stream.socialProviderDurableRows; }
function __GeneratedPortalAgentBrowserRuntimeStreamIsHonest(stream: GeneratedPortalAgentBrowserRuntimeEventChainStream): boolean { return stream.streamedEvents === stream.entries.length && stream.actionIntentCandidates >= __GeneratedPortalAgentBrowserRuntimeActionIntentCandidatesFromEntries(stream.entries).length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffOutboxRefs.length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffRefs.length && stream.actionIntentHandoffOutboxRefs.length === stream.actionIntentHandoffRefs.length && __GeneratedPortalAgentBrowserRuntimeActionIntentChildStatusIsHonest(stream) && __GeneratedPortalAgentBrowserRuntimeSocialProviderReceiptStateIsHonest(stream); }
export function decodeGeneratedPortalAgentBrowserRuntimeEventPayload(value: unknown): GeneratedPortalAgentBrowserRuntimeEventPayload { if (!__GeneratedPortalAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime payload must be an object'); } const payload: GeneratedPortalAgentBrowserRuntimeEventPayload = { phase: __GeneratedPortalAgentBrowserRuntimeReadLiteral(value, 'phase', Object.values(GeneratedPortalAgentBrowserRuntimePhase)), sourceRef: __GeneratedPortalAgentBrowserRuntimeReadString(value, 'sourceRef'), evidenceRef: __GeneratedPortalAgentBrowserRuntimeReadString(value, 'evidenceRef'), capabilityStatus: __GeneratedPortalAgentBrowserRuntimeReadLiteral(value, 'capabilityStatus', Object.values(GeneratedPortalAgentBrowserRuntimeCapabilityStatus)), custodyLabel: __GeneratedPortalAgentBrowserRuntimeReadLiteral(value, 'custodyLabel', Object.values(GeneratedPortalAgentBrowserRuntimeCustodyLabel)), queryVisibility: __GeneratedPortalAgentBrowserRuntimeReadLiteral(value, 'queryVisibility', Object.values(GeneratedPortalAgentBrowserRuntimeQueryVisibility)), degradedReason: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'degradedReason'), journalRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'journalRef'), aiRequestRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'aiRequestRef'), aiAnalysisRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'aiAnalysisRef'), policyEvaluationRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'policyEvaluationRef'), policyDecisionRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'policyDecisionRef'), policyPreviewId: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'policyPreviewId'), assistantActionIntentId: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'assistantActionIntentId'), interventionCommandRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'interventionCommandRef'), interventionResultRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'interventionResultRef'), auditEntryRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'auditEntryRef'), readModelRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'readModelRef'), previousPhaseRef: __GeneratedPortalAgentBrowserRuntimeReadNullableString(value, 'previousPhaseRef'), exactUrlClaimed: __GeneratedPortalAgentBrowserRuntimeReadBoolean(value, 'exactUrlClaimed'), aiAuthority: __GeneratedPortalAgentBrowserRuntimeReadRequiredBoolean(value, 'aiAuthority', false), policyAuthority: __GeneratedPortalAgentBrowserRuntimeReadBoolean(value, 'policyAuthority'), dryRun: __GeneratedPortalAgentBrowserRuntimeReadBoolean(value, 'dryRun'), adapterDispatchClaimed: __GeneratedPortalAgentBrowserRuntimeReadBoolean(value, 'adapterDispatchClaimed'), interventionCommandAllowed: __GeneratedPortalAgentBrowserRuntimeReadBoolean(value, 'interventionCommandAllowed'), observedAt: __GeneratedPortalAgentBrowserRuntimeReadString(value, 'observedAt') }; if (!__GeneratedPortalAgentBrowserRuntimePayloadIsHonest(payload)) { throw new TypeError('browser runtime payload violates Rust-owned claim boundaries'); } return payload; }
export function decodeGeneratedPortalAgentBrowserRuntimeEventChainEntry(value: unknown): GeneratedPortalAgentBrowserRuntimeEventChainEntry { if (!__GeneratedPortalAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime entry must be an object'); } const entry: GeneratedPortalAgentBrowserRuntimeEventChainEntry = { eventType: __GeneratedPortalAgentBrowserRuntimeReadLiteral(value, 'eventType', Object.values(GeneratedPortalAgentBrowserRuntimeEventType)), eventRef: __GeneratedPortalAgentBrowserRuntimeReadString(value, 'eventRef'), payload: decodeGeneratedPortalAgentBrowserRuntimeEventPayload(value['payload']) }; if (GeneratedPortalAgentBrowserRuntimePhaseEventType[entry.payload.phase] !== entry.eventType) { throw new TypeError('browser runtime event type must match payload phase'); } return entry; }
export function decodeGeneratedPortalAgentBrowserRuntimeEventChainStream(value: unknown): GeneratedPortalAgentBrowserRuntimeEventChainStream { if (!__GeneratedPortalAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime stream must be an object'); } const entriesValue = value['entries']; if (!Array.isArray(entriesValue)) { throw new TypeError('entries must be a browser runtime array'); } const stream: GeneratedPortalAgentBrowserRuntimeEventChainStream = { observedRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'observedRows'), streamedEvents: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'streamedEvents'), failedRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'failedRows'), exactUrlRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'exactUrlRows'), manualRequiredRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'manualRequiredRows'), interventionCommandEvents: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'interventionCommandEvents'), readModelProjectionEvents: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'readModelProjectionEvents'), actionIntentCandidates: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'actionIntentCandidates'), actionIntentHandoffCandidates: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'actionIntentHandoffCandidates'), actionIntentHandoffOutboxRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'actionIntentHandoffOutboxRefs'), actionIntentHandoffRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'actionIntentHandoffRefs'), actionIntentChildAcceptedRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'actionIntentChildAcceptedRows'), actionIntentChildCommandRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'actionIntentChildCommandRefs'), actionIntentChildAcceptedEventRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'actionIntentChildAcceptedEventRefs'), actionIntentParentReadModelRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'actionIntentParentReadModelRefs'), actionIntentDispatchAttempts: __GeneratedPortalAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentDispatchAttempts', 0), actionIntentAdapterExecutions: __GeneratedPortalAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentAdapterExecutions', 0), actionIntentChildInterventionExecutions: __GeneratedPortalAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentChildInterventionExecutions', 0), actionIntentEnforcementExecutions: __GeneratedPortalAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentEnforcementExecutions', 0), socialProviderReceiptBoundaryRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'socialProviderReceiptBoundaryRows'), socialProviderDispatchRequiredRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'socialProviderDispatchRequiredRows'), socialProviderManualReceiptRequiredRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'socialProviderManualReceiptRequiredRows'), socialProviderAttemptRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderAttemptRefs'), socialProviderReceiptProofRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderReceiptProofRefs'), socialProviderDurableRows: __GeneratedPortalAgentBrowserRuntimeReadNumber(value, 'socialProviderDurableRows'), socialProviderDurableResultRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderDurableResultRefs'), socialProviderDurableStoreRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderDurableStoreRefs'), socialProviderReadModelRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderReadModelRefs'), socialProviderSupportStatusRefs: __GeneratedPortalAgentBrowserRuntimeReadStringArray(value, 'socialProviderSupportStatusRefs'), entries: entriesValue.map((entry) => decodeGeneratedPortalAgentBrowserRuntimeEventChainEntry(entry)) }; if (!__GeneratedPortalAgentBrowserRuntimeStreamIsHonest(stream)) { throw new TypeError('browser runtime stream violates Rust-owned claim boundaries'); } return stream; }
function __GeneratedPortalAgentBrowserRuntimeReadRequiredNumber<T extends number>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __GeneratedPortalAgentBrowserRuntimeReadNumber(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
export const GeneratedPortalAgentBrowserRuntimeEventPayloadSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentBrowserRuntimeEventPayload } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentBrowserRuntimeEventPayload(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentBrowserRuntimeEventChainEntrySchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentBrowserRuntimeEventChainEntry } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentBrowserRuntimeEventChainEntry(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentBrowserRuntimeEventChainStreamSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentBrowserRuntimeEventChainStream } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentBrowserRuntimeEventChainStream(value) }; } catch { return { success: false }; } } } as const;
 export const GeneratedPortalAgentNetworkRuntimeEventType = { NetworkFlowObserved: "network.flow.observed", NetworkDomainObserved: "network.domain.observed", NetworkActivityClassified: "network.activity.classified", AiAnalysisRequested: "ai.analysis.requested", AiAnalysisCompleted: "ai.analysis.completed", PolicyEvaluationRequested: "policy.evaluation.requested", PolicyDecisionCompleted: "policy.decision.completed", EnforcementCommandIssued: "enforcement.command.issued", EnforcementResultObserved: "enforcement.result.observed", AuditEntryCommitted: "audit.entry.committed", PortalReadModelUpdated: "portal.read_model.updated" } as const; export type GeneratedPortalAgentNetworkRuntimeEventType = (typeof GeneratedPortalAgentNetworkRuntimeEventType)[keyof typeof GeneratedPortalAgentNetworkRuntimeEventType]; export const GeneratedPortalAgentNetworkEvidenceGrade = { A: "A", B: "B", C: "C", D: "D" } as const; export type GeneratedPortalAgentNetworkEvidenceGrade = (typeof GeneratedPortalAgentNetworkEvidenceGrade)[keyof typeof GeneratedPortalAgentNetworkEvidenceGrade]; export const GeneratedPortalAgentNetworkDomainAttributionKind = { DnsAnswer: "dns-answer", SniVisible: "sni-visible", HttpHost: "http-host", ReverseLookup: "reverse-lookup", IpOnly: "ip-only", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkDomainAttributionKind = (typeof GeneratedPortalAgentNetworkDomainAttributionKind)[keyof typeof GeneratedPortalAgentNetworkDomainAttributionKind]; export const GeneratedPortalAgentNetworkRuntimeActivityKind = { SocialCandidate: "social-candidate", VideoCandidate: "video-candidate", GameCandidate: "game-candidate", VpnProxyTunnelCandidate: "vpn-proxy-tunnel-candidate", Unknown: "unknown" } as const; export type GeneratedPortalAgentNetworkRuntimeActivityKind = (typeof GeneratedPortalAgentNetworkRuntimeActivityKind)[keyof typeof GeneratedPortalAgentNetworkRuntimeActivityKind]; export const GeneratedPortalAgentNetworkAiAdvisoryState = { Requested: "requested", Completed: "completed", ManualReviewRequired: "manual-review-required", ProviderUnavailable: "provider-unavailable" } as const; export type GeneratedPortalAgentNetworkAiAdvisoryState = (typeof GeneratedPortalAgentNetworkAiAdvisoryState)[keyof typeof GeneratedPortalAgentNetworkAiAdvisoryState]; export const GeneratedPortalAgentNetworkPolicyDecisionAction = { Observe: "observe", Warn: "warn", AskParent: "ask-parent", Limit: "limit", Block: "block", ManualReview: "manual-review", Unknown: "unknown" } as const; export type GeneratedPortalAgentNetworkPolicyDecisionAction = (typeof GeneratedPortalAgentNetworkPolicyDecisionAction)[keyof typeof GeneratedPortalAgentNetworkPolicyDecisionAction]; export const GeneratedPortalAgentNetworkEnforcementMode = { DryRun: "dry-run", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkEnforcementMode = (typeof GeneratedPortalAgentNetworkEnforcementMode)[keyof typeof GeneratedPortalAgentNetworkEnforcementMode]; export const GeneratedPortalAgentNetworkEnforcementResultStatus = { DryRun: "dry-run", ManualRequired: "manual-required", Unavailable: "unavailable", Rejected: "rejected" } as const; export type GeneratedPortalAgentNetworkEnforcementResultStatus = (typeof GeneratedPortalAgentNetworkEnforcementResultStatus)[keyof typeof GeneratedPortalAgentNetworkEnforcementResultStatus]; export const GeneratedPortalAgentNetworkAuditOutcome = { Committed: "committed", Failed: "failed" } as const; export type GeneratedPortalAgentNetworkAuditOutcome = (typeof GeneratedPortalAgentNetworkAuditOutcome)[keyof typeof GeneratedPortalAgentNetworkAuditOutcome]; export const GeneratedPortalAgentNetworkPortalUpdateKind = { NetworkReadModel: "network-read-model", CapabilityState: "capability-state", ManualRequiredState: "manual-required-state" } as const; export type GeneratedPortalAgentNetworkPortalUpdateKind = (typeof GeneratedPortalAgentNetworkPortalUpdateKind)[keyof typeof GeneratedPortalAgentNetworkPortalUpdateKind];
export type GeneratedPortalAgentNetworkClaimBoundary = { readonly exactUrlAvailable: boolean; readonly decryptedHttpsPayloadAvailable: boolean; readonly messageContentAvailable: boolean; readonly searchQueryAvailable: boolean; readonly adapterActionExecuted: boolean; };
export type GeneratedPortalAgentNetworkFlowObservedEvent = { readonly schemaVersion: number; readonly flowEventRef: string; readonly observedAt: string; readonly deviceRef: string; readonly flowEvidenceRef: string; readonly custody: string; readonly evidenceGrade: GeneratedPortalAgentNetworkEvidenceGrade; readonly claimBoundary: GeneratedPortalAgentNetworkClaimBoundary; };
export type GeneratedPortalAgentNetworkDomainObservedEvent = { readonly schemaVersion: number; readonly domainEventRef: string; readonly previousEventRef: string; readonly flowEvidenceRef: string; readonly domainEvidenceRef: string; readonly attribution: GeneratedPortalAgentNetworkDomainAttributionKind; readonly evidenceGrade: GeneratedPortalAgentNetworkEvidenceGrade; readonly uncertaintyCodes: readonly string[]; readonly claimBoundary: GeneratedPortalAgentNetworkClaimBoundary; };
export type GeneratedPortalAgentNetworkActivityClassifiedEvent = { readonly schemaVersion: number; readonly classificationEventRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly activityKind: GeneratedPortalAgentNetworkRuntimeActivityKind; readonly confidence: number; readonly evidenceGrade: GeneratedPortalAgentNetworkEvidenceGrade; readonly uncertaintyCodes: readonly string[]; };
export type GeneratedPortalAgentNetworkAiAnalysisRequestedEvent = { readonly schemaVersion: number; readonly aiRequestRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly promptTemplateRef: string; readonly custody: string; readonly rawPacketPayloadIncluded: false; };
export type GeneratedPortalAgentNetworkAiAnalysisCompletedEvent = { readonly schemaVersion: number; readonly aiAnalysisRef: string; readonly aiRequestRef: string; readonly previousEventRef: string; readonly advisoryState: GeneratedPortalAgentNetworkAiAdvisoryState; readonly evidenceRefs: readonly string[]; readonly unsupportedClaims: readonly string[]; };
export type GeneratedPortalAgentNetworkPolicyEvaluationRequestedEvent = { readonly schemaVersion: number; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly aiAnalysisRef: string | null; readonly parentRuleRefs: readonly string[]; readonly dryRun: boolean; };
export type GeneratedPortalAgentNetworkPolicyDecisionCompletedEvent = { readonly schemaVersion: number; readonly policyDecisionRef: string; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly decisionAction: GeneratedPortalAgentNetworkPolicyDecisionAction; readonly evidenceRefs: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly adapterCapabilityRequired: boolean; };
export type GeneratedPortalAgentNetworkEnforcementCommandIssuedEvent = { readonly schemaVersion: number; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly adapterCapabilityRef: string; readonly enforcementMode: GeneratedPortalAgentNetworkEnforcementMode; readonly evidenceRefs: readonly string[]; readonly rollbackRef: string | null; };
export type GeneratedPortalAgentNetworkEnforcementResultObservedEvent = { readonly schemaVersion: number; readonly enforcementResultRef: string; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly resultStatus: GeneratedPortalAgentNetworkEnforcementResultStatus; readonly adapterActionExecuted: false; readonly rollbackRef: string | null; readonly unavailableReasonCode: string | null; };
export type GeneratedPortalAgentNetworkAuditEntryCommittedEvent = { readonly schemaVersion: number; readonly auditEntryRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly enforcementCommandRef: string | null; readonly enforcementResultRef: string | null; readonly evidenceRefs: readonly string[]; readonly auditOutcome: GeneratedPortalAgentNetworkAuditOutcome; };
export type GeneratedPortalAgentNetworkPortalReadModelUpdatedEvent = { readonly schemaVersion: number; readonly readModelRef: string; readonly previousEventRef: string; readonly auditEntryRef: string; readonly updateKind: GeneratedPortalAgentNetworkPortalUpdateKind; readonly visibleManualRequired: boolean; readonly visibleUnavailable: boolean; };
export type GeneratedPortalAgentNetworkRuntimeEventPayload = GeneratedPortalAgentNetworkFlowObservedEvent | GeneratedPortalAgentNetworkDomainObservedEvent | GeneratedPortalAgentNetworkActivityClassifiedEvent | GeneratedPortalAgentNetworkAiAnalysisRequestedEvent | GeneratedPortalAgentNetworkAiAnalysisCompletedEvent | GeneratedPortalAgentNetworkPolicyEvaluationRequestedEvent | GeneratedPortalAgentNetworkPolicyDecisionCompletedEvent | GeneratedPortalAgentNetworkEnforcementCommandIssuedEvent | GeneratedPortalAgentNetworkEnforcementResultObservedEvent | GeneratedPortalAgentNetworkAuditEntryCommittedEvent | GeneratedPortalAgentNetworkPortalReadModelUpdatedEvent;
function __GeneratedPortalAgentNetworkRuntimeIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __GeneratedPortalAgentNetworkRuntimeReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__GeneratedPortalAgentNetworkRuntimeIsRecord(value)) { throw new TypeError(`${label} must be a network runtime object`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string or null`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network runtime boolean`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { const value = record['schemaVersion']; if (value !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return GeneratedPortalAgentProtocolRuntime.SchemaVersion; }
function __GeneratedPortalAgentNetworkRuntimeReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __GeneratedPortalAgentNetworkRuntimeReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network runtime literal`); } return value as T; }
function __GeneratedPortalAgentNetworkRuntimeReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a network runtime string array`); } return value as readonly string[]; }
function __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = __GeneratedPortalAgentNetworkRuntimeReadStringArray(record, field); if (value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string array`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be a network runtime confidence from 0 to 1`); } return value; }
function __GeneratedPortalAgentNetworkRuntimeReadClaimBoundary(value: unknown): GeneratedPortalAgentNetworkClaimBoundary { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'claimBoundary'); const boundary = { exactUrlAvailable: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'exactUrlAvailable'), decryptedHttpsPayloadAvailable: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'decryptedHttpsPayloadAvailable'), messageContentAvailable: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'messageContentAvailable'), searchQueryAvailable: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'searchQueryAvailable'), adapterActionExecuted: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'adapterActionExecuted') }; if (boundary.exactUrlAvailable || boundary.decryptedHttpsPayloadAvailable || boundary.messageContentAvailable || boundary.searchQueryAvailable || boundary.adapterActionExecuted) { throw new TypeError('network runtime claim boundary cannot claim unsupported content or adapter action'); } return boundary; }
function __GeneratedPortalAgentNetworkRuntimeDecodeFlowObserved(value: unknown): GeneratedPortalAgentNetworkFlowObservedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network flow observed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), flowEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'flowEventRef'), observedAt: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'observedAt'), deviceRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'deviceRef'), flowEvidenceRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'flowEvidenceRef'), custody: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'custody'), evidenceGrade: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(GeneratedPortalAgentNetworkEvidenceGrade)), claimBoundary: __GeneratedPortalAgentNetworkRuntimeReadClaimBoundary(record['claimBoundary']) }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeDomainObserved(value: unknown): GeneratedPortalAgentNetworkDomainObservedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network domain observed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), domainEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'domainEventRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), flowEvidenceRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'flowEvidenceRef'), domainEvidenceRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'domainEvidenceRef'), attribution: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'attribution', Object.values(GeneratedPortalAgentNetworkDomainAttributionKind)), evidenceGrade: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(GeneratedPortalAgentNetworkEvidenceGrade)), uncertaintyCodes: __GeneratedPortalAgentNetworkRuntimeReadStringArray(record, 'uncertaintyCodes'), claimBoundary: __GeneratedPortalAgentNetworkRuntimeReadClaimBoundary(record['claimBoundary']) }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeActivityClassified(value: unknown): GeneratedPortalAgentNetworkActivityClassifiedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network activity classified payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), classificationEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'classificationEventRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), activityKind: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'activityKind', Object.values(GeneratedPortalAgentNetworkRuntimeActivityKind)), confidence: __GeneratedPortalAgentNetworkRuntimeReadConfidence(record, 'confidence'), evidenceGrade: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(GeneratedPortalAgentNetworkEvidenceGrade)), uncertaintyCodes: __GeneratedPortalAgentNetworkRuntimeReadStringArray(record, 'uncertaintyCodes') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeAiAnalysisRequested(value: unknown): GeneratedPortalAgentNetworkAiAnalysisRequestedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network AI analysis requested payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), aiRequestRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'aiRequestRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), promptTemplateRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'promptTemplateRef'), custody: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'custody'), rawPacketPayloadIncluded: __GeneratedPortalAgentNetworkRuntimeReadRequiredBoolean(record, 'rawPacketPayloadIncluded', false) }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeAiAnalysisCompleted(value: unknown): GeneratedPortalAgentNetworkAiAnalysisCompletedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network AI analysis completed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), aiAnalysisRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'aiAnalysisRef'), aiRequestRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'aiRequestRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), advisoryState: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'advisoryState', Object.values(GeneratedPortalAgentNetworkAiAdvisoryState)), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), unsupportedClaims: __GeneratedPortalAgentNetworkRuntimeReadStringArray(record, 'unsupportedClaims') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodePolicyEvaluationRequested(value: unknown): GeneratedPortalAgentNetworkPolicyEvaluationRequestedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network policy evaluation requested payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), policyEvaluationRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'policyEvaluationRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), aiAnalysisRef: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'aiAnalysisRef'), parentRuleRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'parentRuleRefs'), dryRun: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'dryRun') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodePolicyDecisionCompleted(value: unknown): GeneratedPortalAgentNetworkPolicyDecisionCompletedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network policy decision completed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), policyDecisionRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), policyEvaluationRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'policyEvaluationRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), decisionAction: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'decisionAction', Object.values(GeneratedPortalAgentNetworkPolicyDecisionAction)), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), parentRuleRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'parentRuleRefs'), adapterCapabilityRequired: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'adapterCapabilityRequired') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeEnforcementCommandIssued(value: unknown): GeneratedPortalAgentNetworkEnforcementCommandIssuedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network enforcement command issued payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), enforcementCommandRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'enforcementCommandRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), policyDecisionRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), adapterCapabilityRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'adapterCapabilityRef'), enforcementMode: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'enforcementMode', Object.values(GeneratedPortalAgentNetworkEnforcementMode)), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), rollbackRef: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'rollbackRef') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeEnforcementResultObserved(value: unknown): GeneratedPortalAgentNetworkEnforcementResultObservedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network enforcement result observed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), enforcementResultRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'enforcementResultRef'), enforcementCommandRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'enforcementCommandRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), resultStatus: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'resultStatus', Object.values(GeneratedPortalAgentNetworkEnforcementResultStatus)), adapterActionExecuted: __GeneratedPortalAgentNetworkRuntimeReadRequiredBoolean(record, 'adapterActionExecuted', false), rollbackRef: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'rollbackRef'), unavailableReasonCode: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'unavailableReasonCode') }; }
function __GeneratedPortalAgentNetworkRuntimeDecodeAuditEntryCommitted(value: unknown): GeneratedPortalAgentNetworkAuditEntryCommittedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network audit entry committed payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), auditEntryRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'auditEntryRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), policyDecisionRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), enforcementCommandRef: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'enforcementCommandRef'), enforcementResultRef: __GeneratedPortalAgentNetworkRuntimeReadNullableString(record, 'enforcementResultRef'), evidenceRefs: __GeneratedPortalAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), auditOutcome: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'auditOutcome', Object.values(GeneratedPortalAgentNetworkAuditOutcome)) }; }
function __GeneratedPortalAgentNetworkRuntimeDecodePortalReadModelUpdated(value: unknown): GeneratedPortalAgentNetworkPortalReadModelUpdatedEvent { const record = __GeneratedPortalAgentNetworkRuntimeReadRecord(value, 'network portal read model updated payload'); return { schemaVersion: __GeneratedPortalAgentNetworkRuntimeReadSchemaVersion(record), readModelRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'readModelRef'), previousEventRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'previousEventRef'), auditEntryRef: __GeneratedPortalAgentNetworkRuntimeReadString(record, 'auditEntryRef'), updateKind: __GeneratedPortalAgentNetworkRuntimeReadLiteral(record, 'updateKind', Object.values(GeneratedPortalAgentNetworkPortalUpdateKind)), visibleManualRequired: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'visibleManualRequired'), visibleUnavailable: __GeneratedPortalAgentNetworkRuntimeReadBoolean(record, 'visibleUnavailable') }; }
export function decodeGeneratedPortalAgentNetworkRuntimeEventPayload(eventType: GeneratedPortalAgentNetworkRuntimeEventType, value: unknown): GeneratedPortalAgentNetworkRuntimeEventPayload { switch (eventType) { case GeneratedPortalAgentNetworkRuntimeEventType.NetworkFlowObserved: return __GeneratedPortalAgentNetworkRuntimeDecodeFlowObserved(value); case GeneratedPortalAgentNetworkRuntimeEventType.NetworkDomainObserved: return __GeneratedPortalAgentNetworkRuntimeDecodeDomainObserved(value); case GeneratedPortalAgentNetworkRuntimeEventType.NetworkActivityClassified: return __GeneratedPortalAgentNetworkRuntimeDecodeActivityClassified(value); case GeneratedPortalAgentNetworkRuntimeEventType.AiAnalysisRequested: return __GeneratedPortalAgentNetworkRuntimeDecodeAiAnalysisRequested(value); case GeneratedPortalAgentNetworkRuntimeEventType.AiAnalysisCompleted: return __GeneratedPortalAgentNetworkRuntimeDecodeAiAnalysisCompleted(value); case GeneratedPortalAgentNetworkRuntimeEventType.PolicyEvaluationRequested: return __GeneratedPortalAgentNetworkRuntimeDecodePolicyEvaluationRequested(value); case GeneratedPortalAgentNetworkRuntimeEventType.PolicyDecisionCompleted: return __GeneratedPortalAgentNetworkRuntimeDecodePolicyDecisionCompleted(value); case GeneratedPortalAgentNetworkRuntimeEventType.EnforcementCommandIssued: return __GeneratedPortalAgentNetworkRuntimeDecodeEnforcementCommandIssued(value); case GeneratedPortalAgentNetworkRuntimeEventType.EnforcementResultObserved: return __GeneratedPortalAgentNetworkRuntimeDecodeEnforcementResultObserved(value); case GeneratedPortalAgentNetworkRuntimeEventType.AuditEntryCommitted: return __GeneratedPortalAgentNetworkRuntimeDecodeAuditEntryCommitted(value); case GeneratedPortalAgentNetworkRuntimeEventType.PortalReadModelUpdated: return __GeneratedPortalAgentNetworkRuntimeDecodePortalReadModelUpdated(value); } }
export const GeneratedPortalAgentNetworkRuntimeEventTypeSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkRuntimeEventType } | { readonly success: false } { if (typeof value === 'string' && (Object.values(GeneratedPortalAgentNetworkRuntimeEventType) as readonly string[]).includes(value)) { return { success: true, data: value as GeneratedPortalAgentNetworkRuntimeEventType }; } return { success: false }; } } as const;
 export const GeneratedPortalAgentNetworkRemoteDeliveryStatusRefs = { StatusRef: "network.remote-delivery.external-cross-process-transport-status.10t", EventChainJournalRef: "network.remote-delivery.event-chain-journal.10c", ReceiptLedgerRef: "network.remote-delivery.event-chain.receipt-ledger.10d", LocalReceiptAckRef: "network.remote-delivery.event-chain.local-receipt-ack.10d", DurableEnvelopeRef: "network.remote-delivery.durable-envelope.10e", DurableStoreRef: "network.remote-delivery.durable-envelope-store.10e", DurableReplayRef: "network.remote-delivery.durable-envelope-replay.10e", DurableDeleteExportRef: "network.remote-delivery.durable-envelope-delete-export.10e", DurableSupportStatusRef: "network.remote-delivery.durable-envelope-support-status.10e", OutboxRef: "network.remote-delivery.outbox.10g", OutboxHandoffRef: "network.remote-delivery.outbox-handoff.10g", OutboxReplayRef: "network.remote-delivery.outbox-replay.10g", OutboxSupportStatusRef: "network.remote-delivery.outbox-support-status.10g", TransportDispatchStateRef: "network.remote-delivery.transport-dispatch-state.10k", BlockedDispatchRef: "network.remote-delivery.dispatch-blocked-manual-required.10k", FutureTransportSeamRef: "network.remote-delivery.future-transport-seam.10k", FixtureTransportRef: "network.remote-delivery.fixture-transport.10l", FixtureDispatchAttemptRef: "network.remote-delivery.fixture-dispatch-attempt.10l", FixtureAckRef: "network.remote-delivery.fixture-ack.10l", DeleteExportPropagationRef: "network.remote-delivery.delete-export-propagation-readiness.10m", RemoteDeleteReadinessRef: "network.remote-delivery.remote-delete-readiness.10m", RemoteExportReadinessRef: "network.remote-delivery.remote-export-readiness.10m", ProviderRouteRef: "network.remote-delivery.provider-route.10p", ChildDeviceRouteRef: "network.remote-delivery.child-device-route.10p", ProviderDeliveryReadinessRef: "network.remote-delivery.provider-readiness.10p", ChildDeviceDeliveryReadinessRef: "network.remote-delivery.child-device-readiness.10p", CrossProcessCustodyStatusRef: "network.remote-delivery.cross-process-custody-status.10q", CrossProcessReplayReadinessRef: "network.remote-delivery.cross-process-replay-readiness.10q", RemoteRetentionReadinessRef: "network.remote-delivery.remote-retention-readiness.10q", RemoteDeleteCustodyReadinessRef: "network.remote-delivery.remote-delete-custody-readiness.10q", RemoteExportCustodyReadinessRef: "network.remote-delivery.remote-export-custody-readiness.10q", CrossProcessReplayRef: "network.remote-delivery.cross-process-replay.10r", CrossProcessReplayStoreRef: "network.remote-delivery.cross-process-replay-store.10r", CrossProcessReplayCursorRef: "network.remote-delivery.cross-process-replay-cursor.10r", ExternalCrossProcessTransportRef: "network.remote-delivery.external-cross-process-transport.10t", ExternalCrossProcessTransportEnvelopeRef: "network.remote-delivery.external-cross-process-transport-envelope.10t", ExternalCrossProcessTransportAckRef: "network.remote-delivery.external-cross-process-transport-ack.10t" } as const; export const GeneratedPortalAgentNetworkLiveCaptureStatusRefs = { StatusRef: "network.live-capture.status.13a", Row13StatusRef: "network.live-capture.proof-gate.13", ExecutionStatusRef: "network.live-capture.execution-status.13b", RawStorageStatusRef: "network.live-capture.raw-storage-custody.03a", WindowsProofRef: "network.live-capture.windows-npcap.13", ManualProofRef: "network.live-capture.manual-required.13", LinuxProofRef: "network.live-capture.linux-libpcap.13", MacosProofRef: "network.live-capture.macos-bpf-libpcap.13", InterfaceRef: "network.live-capture.interface.13", DriverRef: "network.live-capture.driver-proof.13", PermissionRef: "network.live-capture.permission-proof.13", BoundedCaptureRef: "network.live-capture.bounded-capture.13", CleanStopRef: "network.live-capture.clean-stop.13", QuotaRef: "network.live-capture.quota-rotation.13", RetentionRef: "network.live-capture.retention-delete-export.13", CustodyRef: "network.live-capture.custody.13", PrivateTrafficExclusionRef: "network.live-capture.private-traffic-exclusion.13", WindowsExecutionRef: "network.live-capture.execution.windows-npcap.13b", ManualExecutionRef: "network.live-capture.execution.manual-required.13b", LinuxExecutionRef: "network.live-capture.execution.linux-libpcap.13b", MacosExecutionRef: "network.live-capture.execution.macos-bpf-libpcap.13b", DriverInvocationRef: "network.live-capture.driver-invocation.13b", InterfaceObservationRef: "network.live-capture.interface-observation.13b", ExecutionPermissionRef: "network.live-capture.permission.13b", BoundedWindowRef: "network.live-capture.bounded-window.13b", ExecutionCleanStopRef: "network.live-capture.clean-stop.13b", ExecutionCustodyRef: "network.live-capture.custody.13b", ExecutionRetentionRef: "network.live-capture.retention-delete-export.13b", MetadataSanitizationRef: "network.live-capture.metadata-sanitization.13b", ExecutionPrivateTrafficExclusionRef: "network.live-capture.private-traffic-exclusion.13b", RawManifestRef: "network.raw-capture.manifest.03a", RawStorageLocationRef: "network.raw-capture.storage-location.03a", RawEncryptionRef: "network.raw-capture.encryption-at-rest.03a", RawQuotaRef: "network.raw-capture.quota-rotation.03a", RawRetentionRef: "network.raw-capture.retention-policy.03a", RawDeleteExportRef: "network.raw-capture.delete-export.03a", RawCustodyChainRef: "network.raw-capture.custody-chain.03a", RawPrivateTrafficExclusionRef: "network.raw-capture.private-traffic-exclusion.03a" } as const; export const GeneratedPortalAgentNetworkLinuxNftablesLabStatusRefs = { StatusRef: "network.linux-nftables.lab-status.42a", LabRef: "network.linux-nftables.lab-execution.42a", LinuxAdapterGateRef: "network.linux-adapter.gate.42a", PolicyDecisionRef: "network.policy-decision.linux.42a", ParentRuleRef: "network.parent-rule.linux.42a", EvidenceRef: "network.evidence.linux.42a", DistroRef: "network.linux.distro.42a", KernelRef: "network.linux.kernel.42a", TableName: "ocentra_parent_lab_row42a", ChainName: "ocentra_parent_lab_chain_row42a", TargetRemoteAddress: "203.0.113.253", CreateTableCommandRef: "network.linux-nftables.command.create-table.42a", CreateChainCommandRef: "network.linux-nftables.command.create-chain.42a", AddRuleCommandRef: "network.linux-nftables.command.add-rule.42a", VerifyRuleCommandRef: "network.linux-nftables.command.verify-rule-present.42a", DeleteTableCommandRef: "network.linux-nftables.command.delete-table.42a", VerifyRemovedCommandRef: "network.linux-nftables.command.verify-table-removed.42a", CreateTableOutputSha256: "sha256:network-linux-nftables-create-table-42a", CreateChainOutputSha256: "sha256:network-linux-nftables-create-chain-42a", AddRuleOutputSha256: "sha256:network-linux-nftables-add-rule-42a", VerifyRuleOutputSha256: "sha256:network-linux-nftables-verify-rule-present-42a", DeleteTableOutputSha256: "sha256:network-linux-nftables-delete-table-42a", VerifyRemovedOutputSha256: "sha256:network-linux-nftables-verify-table-removed-42a" } as const; export const GeneratedPortalAgentNetworkWindowsFirewallLabStatusRefs = { StatusRef: "network.windows-firewall.lab-status.38a", LabRef: "network.windows-firewall.lab-execution.38a", FirewallAdapterPlanRef: "network.windows-firewall.adapter-plan.38a", PolicyDecisionRef: "network.policy-decision.windows-firewall.38a", ParentRuleRef: "network.parent-rule.windows-firewall.38a", EvidenceRef: "network.evidence.windows-firewall.38a", WindowsOsScopeRef: "network.windows-firewall.os-scope.38a", TargetRef: "network.windows-firewall.target.remote-address.38a", FirewallRuleRef: "network.windows-firewall.rule.38a", RuleName: "OcentraParentNetworkLab-row38a", TargetRemoteAddress: "203.0.113.254", ApplyRuleCommandRef: "network.windows-firewall.command.apply-rule.38a", VerifyPresentCommandRef: "network.windows-firewall.command.verify-rule-present.38a", RollbackRuleCommandRef: "network.windows-firewall.command.rollback-rule.38a", VerifyRemovedCommandRef: "network.windows-firewall.command.verify-rule-removed.38a", ApplyRuleOutputSha256: "sha256:network-windows-firewall-apply-rule-38a", VerifyPresentOutputSha256: "sha256:network-windows-firewall-verify-rule-present-38a", RollbackRuleOutputSha256: "sha256:network-windows-firewall-rollback-rule-38a", VerifyRemovedOutputSha256: "sha256:network-windows-firewall-verify-rule-removed-38a" } as const; export const GeneratedPortalAgentNetworkWindowsWfpGateStatusRefs = { StatusRef: "network.windows-wfp.gate-status.39", WfpGateRef: "network.windows-wfp.gate.39", PolicyDecisionRef: "network.policy-decision.windows-wfp.39", ParentRuleRef: "network.parent-rule.windows-wfp.39", EvidenceRef: "network.evidence.windows-wfp.39", LocalAiResultRef: "network.local-ai.windows-wfp.39", TargetRef: "network.windows-wfp.target.39", WfpProviderRef: "network.windows-wfp.provider.39", WfpLayerRef: "network.windows-wfp.layer.39", AdministratorPermissionProofRef: "network.windows-wfp.admin-permission-proof.39", DriverSigningProofRef: "network.windows-wfp.driver-signing-proof.39", DriverPackageProofRef: "network.windows-wfp.driver-package-proof.39", ProviderRegistrationPlanRef: "network.windows-wfp.provider-registration-plan.39", LayerCapabilityMatrixRef: "network.windows-wfp.layer-capability-matrix.39", RollbackPlanRef: "network.windows-wfp.rollback-plan.39", LabResultArtifactRef: "network.windows-wfp.lab-result-artifact.39", AuditEventRef: "network.windows-wfp.audit-event.39" } as const; export const GeneratedPortalAgentNetworkAndroidVpnServiceGateStatusRefs = { StatusRef: "network.android-vpn-service.gate-status.40", AndroidVpnServiceGateRef: "network.android-vpn-service.gate.40", PolicyDecisionRef: "network.policy-decision.android-vpn-service.40", ParentRuleRef: "network.parent-rule.android-vpn-service.40", EvidenceRef: "network.evidence.android-vpn-service.40", LocalAiResultRef: "network.local-ai.android-vpn-service.40", PackageRef: "network.android-vpn-service.package.40", VpnServiceRef: "network.android-vpn-service.service.40", VpnServiceDeclarationRef: "network.android-vpn-service.declaration.40", UserConsentProofRef: "network.android-vpn-service.user-consent-proof.40", PhysicalDeviceProofRef: "network.android-vpn-service.physical-device-proof.40", PackageIdentityProofRef: "network.android-vpn-service.package-identity-proof.40", VirtualInterfaceProofRef: "network.android-vpn-service.virtual-interface-proof.40", TrafficObservationProofRef: "network.android-vpn-service.traffic-observation-proof.40", RollbackPlanRef: "network.android-vpn-service.rollback-plan.40", AuditEventRef: "network.android-vpn-service.audit-event.40", DeviceOwnerProofRef: "network.android-vpn-service.device-owner-proof.40" } as const; export const GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatusRefs = { StatusRef: "network.apple-network-extension.gate-status.41", AppleNetworkExtensionGateRef: "network.apple-network-extension.gate.41", PolicyDecisionRef: "network.policy-decision.apple-network-extension.41", ParentRuleRef: "network.parent-rule.apple-network-extension.41", EvidenceRef: "network.evidence.apple-network-extension.41", LocalAiResultRef: "network.local-ai.apple-network-extension.41", BundleRef: "network.apple-network-extension.bundle.41", NetworkExtensionRef: "network.apple-network-extension.extension.41", DeveloperTeamProofRef: "network.apple-network-extension.developer-team-proof.41", EntitlementApprovalProofRef: "network.apple-network-extension.entitlement-approval-proof.41", ProvisioningProfileProofRef: "network.apple-network-extension.provisioning-profile-proof.41", SigningProofRef: "network.apple-network-extension.signing-proof.41", DeviceOrTestFlightProofRef: "network.apple-network-extension.device-or-testflight-proof.41", NetworkExtensionDeclarationRef: "network.apple-network-extension.declaration.41", ExtensionConfigurationProofRef: "network.apple-network-extension.configuration-proof.41", RollbackPlanRef: "network.apple-network-extension.rollback-plan.41", AuditEventRef: "network.apple-network-extension.audit-event.41", SupervisionOrMdmProofRef: "network.apple-network-extension.supervision-or-mdm-proof.41" } as const; export const GeneratedPortalAgentNetworkRemoteDeliveryStatusState = { FixtureRequirementsRecordedButNotImplemented: "fixture-requirements-recorded-but-not-implemented", ManualRequired: "manual-required" } as const; export type GeneratedPortalAgentNetworkRemoteDeliveryStatusState = (typeof GeneratedPortalAgentNetworkRemoteDeliveryStatusState)[keyof typeof GeneratedPortalAgentNetworkRemoteDeliveryStatusState]; export const GeneratedPortalAgentNetworkRemoteDeliveryTransportDispatchState = { ManualRequiredBlocked: "manual-required-blocked" } as const; export type GeneratedPortalAgentNetworkRemoteDeliveryTransportDispatchState = (typeof GeneratedPortalAgentNetworkRemoteDeliveryTransportDispatchState)[keyof typeof GeneratedPortalAgentNetworkRemoteDeliveryTransportDispatchState]; export const GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState = { ManualRequiredUnavailable: "manual-required-unavailable" } as const; export type GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState = (typeof GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState)[keyof typeof GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState]; export const GeneratedPortalAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState = { ManualRequiredUnavailable: "manual-required-unavailable" } as const; export type GeneratedPortalAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState = (typeof GeneratedPortalAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState)[keyof typeof GeneratedPortalAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState]; export const GeneratedPortalAgentNetworkRemoteDeliveryExternalCrossProcessTransportState = { DeterministicEnvelopeAckRecorded: "deterministic-envelope-ack-recorded" } as const; export type GeneratedPortalAgentNetworkRemoteDeliveryExternalCrossProcessTransportState = (typeof GeneratedPortalAgentNetworkRemoteDeliveryExternalCrossProcessTransportState)[keyof typeof GeneratedPortalAgentNetworkRemoteDeliveryExternalCrossProcessTransportState]; export const GeneratedPortalAgentNetworkLiveCapturePlatform = { WindowsNpcap: "windows-npcap", LinuxLibpcap: "linux-libpcap", MacosBpfLibpcap: "macos-bpf-libpcap" } as const; export type GeneratedPortalAgentNetworkLiveCapturePlatform = (typeof GeneratedPortalAgentNetworkLiveCapturePlatform)[keyof typeof GeneratedPortalAgentNetworkLiveCapturePlatform]; export const GeneratedPortalAgentNetworkLiveCaptureProofState = { ProofReady: "proof-ready", ManualRequired: "manual-required", Unavailable: "unavailable", Degraded: "degraded" } as const; export type GeneratedPortalAgentNetworkLiveCaptureProofState = (typeof GeneratedPortalAgentNetworkLiveCaptureProofState)[keyof typeof GeneratedPortalAgentNetworkLiveCaptureProofState]; export const GeneratedPortalAgentNetworkRawCaptureStorageState = { CustodyReady: "custody-ready", ManualRequired: "manual-required", Unavailable: "unavailable", Degraded: "degraded" } as const; export type GeneratedPortalAgentNetworkRawCaptureStorageState = (typeof GeneratedPortalAgentNetworkRawCaptureStorageState)[keyof typeof GeneratedPortalAgentNetworkRawCaptureStorageState]; export const GeneratedPortalAgentNetworkLiveCaptureExecutionState = { ManualRequired: "manual-required", BoundedExecuted: "bounded-executed", Unavailable: "unavailable", Degraded: "degraded" } as const; export type GeneratedPortalAgentNetworkLiveCaptureExecutionState = (typeof GeneratedPortalAgentNetworkLiveCaptureExecutionState)[keyof typeof GeneratedPortalAgentNetworkLiveCaptureExecutionState]; export const GeneratedPortalAgentNetworkLinuxNftablesLabState = { ManualRequired: "manual-required", ExecutedAndRolledBack: "executed-and-rolled-back", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkLinuxNftablesLabState = (typeof GeneratedPortalAgentNetworkLinuxNftablesLabState)[keyof typeof GeneratedPortalAgentNetworkLinuxNftablesLabState]; export const GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind = { CreateTable: "create-table", CreateChain: "create-chain", AddRule: "add-rule", VerifyRulePresent: "verify-rule-present", DeleteTable: "delete-table", VerifyTableRemoved: "verify-table-removed" } as const; export type GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind = (typeof GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind)[keyof typeof GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind]; export const GeneratedPortalAgentNetworkWindowsFirewallLabState = { ManualRequired: "manual-required", ExecutedAndRolledBack: "executed-and-rolled-back", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkWindowsFirewallLabState = (typeof GeneratedPortalAgentNetworkWindowsFirewallLabState)[keyof typeof GeneratedPortalAgentNetworkWindowsFirewallLabState]; export const GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind = { ApplyRule: "apply-rule", VerifyRulePresent: "verify-rule-present", RollbackRule: "rollback-rule", VerifyRuleRemoved: "verify-rule-removed" } as const; export type GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind = (typeof GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind)[keyof typeof GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind]; export const GeneratedPortalAgentNetworkWindowsWfpGateState = { ManualRequired: "manual-required", ResearchOnly: "research-only", Unavailable: "unavailable", LabProofReady: "lab-proof-ready" } as const; export type GeneratedPortalAgentNetworkWindowsWfpGateState = (typeof GeneratedPortalAgentNetworkWindowsWfpGateState)[keyof typeof GeneratedPortalAgentNetworkWindowsWfpGateState]; export const GeneratedPortalAgentNetworkWindowsWfpCapabilityState = { ManualRequired: "manual-required", LabReady: "lab-ready", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkWindowsWfpCapabilityState = (typeof GeneratedPortalAgentNetworkWindowsWfpCapabilityState)[keyof typeof GeneratedPortalAgentNetworkWindowsWfpCapabilityState]; export const GeneratedPortalAgentNetworkAndroidVpnServiceGateState = { ManualRequired: "manual-required", ResearchOnly: "research-only", Unavailable: "unavailable", PhysicalDeviceProofReady: "physical-device-proof-ready" } as const; export type GeneratedPortalAgentNetworkAndroidVpnServiceGateState = (typeof GeneratedPortalAgentNetworkAndroidVpnServiceGateState)[keyof typeof GeneratedPortalAgentNetworkAndroidVpnServiceGateState]; export const GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState = { PhysicalDeviceReady: "physical-device-ready", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState = (typeof GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState)[keyof typeof GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState]; export const GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact = { VpnServiceDeclaration: "vpn-service-declaration", UserConsentProof: "user-consent-proof", PhysicalDeviceProof: "physical-device-proof", PackageIdentityProof: "package-identity-proof", VirtualInterfaceProof: "virtual-interface-proof", TrafficObservationProof: "traffic-observation-proof", RollbackPlan: "rollback-plan", AuditEvent: "audit-event", DeviceOwnerProof: "device-owner-proof" } as const; export type GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact = (typeof GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact)[keyof typeof GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact]; export const GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason = { ResearchOnlyRequested: "research-only-requested", CapabilityManualRequired: "capability-manual-required", CapabilityUnavailable: "capability-unavailable", EvidenceGradeBelowProofThreshold: "evidence-grade-below-proof-threshold", PolicyNotVpnServiceApproved: "policy-not-vpn-service-approved", MissingRequiredArtifact: "missing-required-artifact" } as const; export type GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason = (typeof GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason)[keyof typeof GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason]; export const GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform = { MacOs: "mac-os", Ios: "ios" } as const; export type GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform = (typeof GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform)[keyof typeof GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform]; export const GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState = { AppleDeviceReady: "apple-device-ready", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState = (typeof GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState)[keyof typeof GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState]; export const GeneratedPortalAgentNetworkAppleNetworkExtensionGateState = { ResearchOnly: "research-only", ManualRequired: "manual-required", Unavailable: "unavailable", AppleEntitlementProofReady: "apple-entitlement-proof-ready" } as const; export type GeneratedPortalAgentNetworkAppleNetworkExtensionGateState = (typeof GeneratedPortalAgentNetworkAppleNetworkExtensionGateState)[keyof typeof GeneratedPortalAgentNetworkAppleNetworkExtensionGateState]; export const GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact = { DeveloperTeamProof: "developer-team-proof", EntitlementApprovalProof: "entitlement-approval-proof", ProvisioningProfileProof: "provisioning-profile-proof", SigningProof: "signing-proof", DeviceOrTestflightProof: "device-or-testflight-proof", NetworkExtensionDeclaration: "network-extension-declaration", ExtensionConfigurationProof: "extension-configuration-proof", RollbackPlan: "rollback-plan", AuditEvent: "audit-event", SupervisionOrMdmProof: "supervision-or-mdm-proof" } as const; export type GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact = (typeof GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact)[keyof typeof GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact]; export const GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason = { ResearchOnlyRequested: "research-only-requested", CapabilityManualRequired: "capability-manual-required", CapabilityUnavailable: "capability-unavailable", EvidenceGradeBelowProofThreshold: "evidence-grade-below-proof-threshold", PolicyNotNetworkExtensionApproved: "policy-not-network-extension-approved", MissingRequiredArtifact: "missing-required-artifact" } as const; export type GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason = (typeof GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason)[keyof typeof GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason];
export type GeneratedPortalAgentNetworkRemoteDeliveryStatus = Readonly<Record<string, unknown>>;
export type GeneratedPortalAgentNetworkLiveCaptureStatusRow = Readonly<Record<string, unknown>>;
export type GeneratedPortalAgentNetworkLiveCaptureStatus = Readonly<Record<string, unknown>> & { readonly rows: readonly GeneratedPortalAgentNetworkLiveCaptureStatusRow[] };
export type GeneratedPortalAgentNetworkLinuxNftablesLabCommandRow = Readonly<Record<string, unknown>> & { readonly kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind };
export type GeneratedPortalAgentNetworkLinuxNftablesLabStatus = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly GeneratedPortalAgentNetworkLinuxNftablesLabCommandRow[] };
export type GeneratedPortalAgentNetworkWindowsFirewallLabCommandRow = Readonly<Record<string, unknown>> & { readonly kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind };
export type GeneratedPortalAgentNetworkWindowsFirewallLabStatus = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly GeneratedPortalAgentNetworkWindowsFirewallLabCommandRow[] };
export type GeneratedPortalAgentNetworkWindowsWfpGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly string[]; readonly missingRequiredArtifacts: readonly string[]; readonly wfpLabProofReady: boolean; readonly enforcementCommandPublished: false };
export type GeneratedPortalAgentNetworkAndroidVpnServiceGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason[]; readonly missingRequiredArtifacts: readonly GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact[]; readonly gateState: GeneratedPortalAgentNetworkAndroidVpnServiceGateState; readonly physicalDeviceProofReady: boolean; readonly enforcementCommandPublished: false };
export type GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason[]; readonly missingRequiredArtifacts: readonly GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact[]; readonly platform: GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform; readonly gateState: GeneratedPortalAgentNetworkAppleNetworkExtensionGateState; readonly appleEntitlementProofReady: boolean; readonly enforcementCommandPublished: false };
const __GeneratedPortalAgentNetworkStatusRemoteStringFields = ['statusRef','custodyProofRef','publisherAuthRef','subscriberAuthRef','encryptionRef','retentionPolicyRef','replayPlanRef','deletionPlanRef','offsetPolicyRef','dedupePolicyRef','transportConfigRef','relayIdentityRef','relayPolicyRef','eventChainJournalRef','receiptLedgerRef','localReceiptAckRef','durableEnvelopeRef','durableStoreRef','durableReplayRef','durableDeleteExportRef','durableSupportStatusRef','outboxRef','outboxHandoffRef','outboxReplayRef','outboxSupportStatusRef','transportDispatchStateRef','blockedDispatchRef','futureTransportSeamRef','fixtureTransportRef','fixtureDispatchAttemptRef','fixtureAckRef','deleteExportPropagationRef','remoteDeleteReadinessRef','remoteExportReadinessRef','providerRouteRef','childDeviceRouteRef','providerDeliveryReadinessRef','childDeviceDeliveryReadinessRef','crossProcessCustodyStatusRef','crossProcessReplayReadinessRef','remoteRetentionReadinessRef','remoteDeleteCustodyReadinessRef','remoteExportCustodyReadinessRef','crossProcessReplayRef','crossProcessReplayStoreRef','crossProcessReplayCursorRef','externalCrossProcessTransportRef','externalCrossProcessTransportEnvelopeRef','externalCrossProcessTransportAckRef'] as const;
const __GeneratedPortalAgentNetworkStatusRemoteCountFields = ['brokerMissingArtifactCount','familyHubMissingArtifactCount','acceptedEventTypeCount','droppedEventDeadLetterCount','durableEnvelopeMissingArtifactCount','outboxCandidateCount','sourceOutboxCandidateCount','preparedNotDispatchedCount','blockedDispatchRecordCount','fixtureSourceOutboxCandidateCount','fixtureDispatchAttemptCount','fixtureRemoteAckCount','deleteExportReadinessRecordCount','remoteDeleteReadyCount','remoteExportReadyCount','providerDeliveryReadinessRecordCount','childDeviceDeliveryReadinessRecordCount','crossProcessReplayReadinessRecordCount','remoteRetentionReadinessRecordCount','remoteDeleteCustodyReadinessRecordCount','remoteExportCustodyReadinessRecordCount','crossProcessReplayRecordCount','crossProcessReplayStoreWriteCount','crossProcessReplayCursorNextSequence','externalCrossProcessTransportRecordCount','externalCrossProcessTransportEnvelopeCount','externalCrossProcessTransportAckCount'] as const;
const __GeneratedPortalAgentNetworkStatusRemoteZeroFields = ['providerDeliveryArtifactCount','childDeviceDeliveryArtifactCount','crossProcessReplayArtifactCount','remoteRetentionArtifactCount','remoteDeleteCustodyArtifactCount','remoteExportCustodyArtifactCount','dispatchReadyCandidateCount','dispatchAttemptCount','remoteAckCount','sequenceGapCount','eventIdMismatchCount','eventTypeMismatchCount','correlationMismatchCount','enforcementCommandEventCount','adapterActionExecutedCount','rawPcapAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','videoContentAvailableCount','privateMessageContentAvailableCount','searchQueryAvailableCount'] as const;
const __GeneratedPortalAgentNetworkStatusRemoteBooleanFields = ['localIdempotencyQueueProved','queuedDuplicateRejected','completedDuplicateRejected','durableEnvelopeReady','blockedDispatchRecordsMatchOutboxCandidates','fixtureRecordsMatchOutboxCandidates','deleteExportRecordsMatchFixtureAcks','providerDeliveryRecordsMatchFixtureAcks','childDeviceDeliveryRecordsMatchFixtureAcks','crossProcessCustodyRecordsMatchProviderChildReadiness','crossProcessReplayRecordsMatchDurableEnvelopes','crossProcessReplayRecordsMatchCustodyReadiness','externalCrossProcessTransportRecordsMatchReplayRecords','externalCrossProcessTransportAckRecordsMatchEnvelopes','duplicateDurableEnvelopeRejected','outboxCandidatesMatchDurableEnvelopes','outboxCandidatesMatchReceipts'] as const;
const __GeneratedPortalAgentNetworkStatusRemoteFalseFields = ['brokerDeliveryImplemented','familyHubDeliveryImplemented','remoteDeliveryAckImplemented','providerDeliveryImplemented','childDeviceDeliveryImplemented','remoteDeleteExportPropagationImplemented','productReadyRemoteDelivery','policyAuthority','sideEffectAuthority','hostFilteringClaimed'] as const;
const __GeneratedPortalAgentNetworkStatusRemoteTrueFields = ['crossProcessReplayImplemented','externalCrossProcessTransportImplemented'] as const;
const __GeneratedPortalAgentNetworkStatusLiveStatusStringFields = ['statusRef','row13StatusRef','executionStatusRef','rawStorageStatusRef'] as const;
const __GeneratedPortalAgentNetworkStatusLiveStatusCountFields = ['platformRowCount','proofReadyCount','manualRequiredCount','unavailableCount','degradedCount','requiredArtifactCount','missingArtifactCount','storageCustodyReadyCount','storageManualRequiredCount','storageUnavailableCount','storageDegradedCount','storageMissingArtifactCount','boundedExecutedCount','executionManualRequiredCount','executionUnavailableCount','executionDegradedCount','executionMissingArtifactCount','metadataSnapshotExecutedCount','capturedPacketCount','captureReadyCount','rawArtifactStorageAuthorizedCount','driverInvokedCount','liveCaptureExecutedCount'] as const;
const __GeneratedPortalAgentNetworkStatusLiveStatusZeroFields = ['rawArtifactCreatedCount','remoteUploadEnabledCount','rawPcapWithoutCustodyAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','privateMessageAvailableCount','searchQueryAvailableCount','policyAuthorityCount','adapterAuthorityCount','enforcementCommandEventCount','netstatMetadataSubstitutionCount','hostFilteringClaimCount'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowStringFields = ['captureProofRef','storageProofRef'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowNullableStringFields = ['interfaceRef','driverProofRef','permissionProofRef','boundedCaptureRef','cleanStopRef','quotaRotationRef','retentionDeleteExportRef','custodyRef','privateTrafficExclusionRef','rawArtifactManifestRef','storageLocationRef','encryptionAtRestRef','storageQuotaRotationRef','retentionPolicyRef','storageDeleteExportRef','custodyChainRef','storagePrivateTrafficExclusionRef','executionRef','driverInvocationRef','interfaceObservationRef','executionPermissionRef','boundedWindowRef','executionCleanStopRef','executionCustodyRef','executionRetentionDeleteExportRef','metadataOnlySanitizationRef','executionPrivateTrafficExclusionRef'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowCountFields = ['executionMissingArtifactCount','capturedPacketCount','missingArtifactCount','storageMissingArtifactCount'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowBooleanFields = ['metadataSnapshotExecuted','captureReady','rawArtifactStorageAuthorized','driverInvoked','liveCaptureExecuted'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowFalseFields = ['rawArtifactCreated','remoteUploadEnabled','rawPcapWithoutCustodyAvailable','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','privateMessageAvailable','searchQueryAvailable','policyAuthority','adapterAuthority','netstatMetadataSubstitutedForLiveCapture','hostFilteringClaimed'] as const;
const __GeneratedPortalAgentNetworkStatusLiveRowZeroFields = ['enforcementCommandsPublished'] as const;
const __GeneratedPortalAgentNetworkStatusLinuxNftablesStringFields = ['statusRef','labRef','linuxAdapterGateRef','policyDecisionRef','parentRuleRef','distroRef','kernelRef','tableName','chainName','targetRemoteAddress'] as const;
const __GeneratedPortalAgentNetworkStatusLinuxNftablesBooleanFields = ['wslHostObserved','rootPermissionObserved','nftToolObserved','tableCreateObserved','chainCreateObserved','ruleAddObserved','verifyPresentObserved','rollbackObserved','verifyRemovedObserved','labPacketFilterRuleExecuted','rollbackVerified'] as const;
const __GeneratedPortalAgentNetworkStatusLinuxNftablesFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','genericLinuxSupportClaimed','serviceManagerInstallClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __GeneratedPortalAgentNetworkStatusLinuxNftablesObservedFlags = [{ field: 'tableCreateObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.CreateTable },{ field: 'chainCreateObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.CreateChain },{ field: 'ruleAddObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.AddRule },{ field: 'verifyPresentObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.VerifyRulePresent },{ field: 'rollbackObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.DeleteTable },{ field: 'verifyRemovedObserved', kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.VerifyTableRemoved }] as const;
const __GeneratedPortalAgentNetworkStatusLinuxNftablesExpectedOutcomes = [{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.CreateTable, table: true, chain: false, rule: false },{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.CreateChain, table: true, chain: true, rule: false },{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.AddRule, table: true, chain: true, rule: true },{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.VerifyRulePresent, table: true, chain: true, rule: true },{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.DeleteTable, table: false, chain: false, rule: false },{ kind: GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind.VerifyTableRemoved, table: false, chain: false, rule: false }] as const;
const __GeneratedPortalAgentNetworkStatusWindowsFirewallStringFields = ['statusRef','labRef','firewallAdapterPlanRef','policyDecisionRef','parentRuleRef','windowsOsScopeRef','targetRef','firewallRuleRef','ruleName','targetRemoteAddress'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsFirewallBooleanFields = ['windowsHostObserved','administratorPermissionObserved','applyCommandObserved','verifyPresentObserved','rollbackCommandObserved','verifyRemovedObserved','labFirewallMutationExecuted','rollbackVerified','adapterApplyAuthorized'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsFirewallFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','hostFirewallMutationClaimed','netshCommandInvoked','powershellCommandInvoked','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsFirewallObservedFlags = [{ field: 'applyCommandObserved', kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.ApplyRule },{ field: 'verifyPresentObserved', kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.VerifyRulePresent },{ field: 'rollbackCommandObserved', kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.RollbackRule },{ field: 'verifyRemovedObserved', kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.VerifyRuleRemoved }] as const;
const __GeneratedPortalAgentNetworkStatusWindowsFirewallExpectedOutcomes = [{ kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.ApplyRule, rulePresentAfterCommand: true },{ kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.VerifyRulePresent, rulePresentAfterCommand: true },{ kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.RollbackRule, rulePresentAfterCommand: false },{ kind: GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind.VerifyRuleRemoved, rulePresentAfterCommand: false }] as const;
const __GeneratedPortalAgentNetworkStatusWindowsWfpStringFields = ['statusRef','wfpGateRef','policyDecisionRef','parentRuleRef','targetRef','wfpProviderRef','wfpLayerRef'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsWfpNullableStringFields = ['localAiResultRef','administratorPermissionProofRef','driverSigningProofRef','driverPackageProofRef','providerRegistrationPlanRef','layerCapabilityMatrixRef','rollbackPlanRef','labResultArtifactRef','auditEventRef'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsWfpStringArrayFields = ['evidenceRefs','boundaryReasons','missingRequiredArtifacts'] as const;
const __GeneratedPortalAgentNetworkStatusWindowsWfpFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','liveDriverInstallClaimed','calloutRegistrationClaimed','packetBlockClaimed','kernelPayloadInspectionClaimed','commandInvocationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __GeneratedPortalAgentNetworkStatusAndroidVpnStringFields = ['statusRef','androidVpnServiceGateRef','policyDecisionRef','parentRuleRef','packageRef','vpnServiceRef'] as const;
const __GeneratedPortalAgentNetworkStatusAndroidVpnNullableStringFields = ['localAiResultRef','vpnServiceDeclarationRef','userConsentProofRef','physicalDeviceProofRef','packageIdentityProofRef','virtualInterfaceProofRef','trafficObservationProofRef','rollbackPlanRef','auditEventRef','deviceOwnerProofRef'] as const;
const __GeneratedPortalAgentNetworkStatusAndroidVpnBooleanFields = ['deviceOwnerRequired','physicalDeviceProofReady','deviceOwnerAuthorityProved'] as const;
const __GeneratedPortalAgentNetworkStatusAndroidVpnFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','emulatorOnlyProductSupportClaimed','liveVpnTunnelClaimed','packetBlockClaimed','appPackageCorrelationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionStringFields = ['statusRef','appleNetworkExtensionGateRef','policyDecisionRef','parentRuleRef','bundleRef','networkExtensionRef'] as const;
const __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionNullableStringFields = ['localAiResultRef','developerTeamProofRef','entitlementApprovalProofRef','provisioningProfileProofRef','signingProofRef','deviceOrTestFlightProofRef','networkExtensionDeclarationRef','extensionConfigurationProofRef','rollbackPlanRef','auditEventRef','supervisionOrMdmProofRef'] as const;
const __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionBooleanFields = ['supervisionRequired','appleEntitlementProofReady','supervisionAuthorityProved'] as const;
const __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','simulatorOnlyProductSupportClaimed','liveNetworkExtensionClaimed','packetBlockClaimed','appLevelControlClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
function __GeneratedPortalAgentNetworkStatusIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __GeneratedPortalAgentNetworkStatusReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__GeneratedPortalAgentNetworkStatusIsRecord(value)) { throw new TypeError(`${label} must be a network status object`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string or null`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadCount(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative integer`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value)) { throw new TypeError(`${field} must be an integer`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadRequiredCount(record: Readonly<Record<string, unknown>>, field: string, expected: number): number { const value = __GeneratedPortalAgentNetworkStatusReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __GeneratedPortalAgentNetworkStatusReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network status boolean`); } return value; }
function __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record: Readonly<Record<string, unknown>>, field: string, expected: boolean): boolean { const value = __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __GeneratedPortalAgentNetworkStatusReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __GeneratedPortalAgentNetworkStatusReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network status literal`); } return value as T; }
function __GeneratedPortalAgentNetworkStatusReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a network status string array`); } values.forEach((value) => { if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} entries must be non-empty network status strings`); } }); return values; }
function __GeneratedPortalAgentNetworkStatusReadLiteralArray<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): readonly T[] { return __GeneratedPortalAgentNetworkStatusReadStringArray(record, field).map((value) => { if (!allowed.includes(value as T)) { throw new TypeError(`${field} entries must be Rust-owned network status literals`); } return value as T; }); }
function __GeneratedPortalAgentNetworkStatusReadRecordArray(record: Readonly<Record<string, unknown>>, field: string, label: string): readonly Readonly<Record<string, unknown>>[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a ${label} array`); } return values.map((value) => __GeneratedPortalAgentNetworkStatusReadRecord(value, label)); }
function __GeneratedPortalAgentNetworkStatusRequireCountMatches(record: Readonly<Record<string, unknown>>, field: string, expected: number): void { const value = __GeneratedPortalAgentNetworkStatusReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must match command evidence length`); } }
function __GeneratedPortalAgentNetworkStatusRequireUniqueRowsByKind<T extends Readonly<Record<string, unknown>> & { readonly kind: string }>(rows: readonly T[], label: string): ReadonlyMap<string, T> { const byKind = new Map(rows.map((row) => [row.kind, row] as const)); if (byKind.size !== rows.length) { throw new TypeError(`${label} command evidence must use unique command kinds`); } return byKind; }
function __GeneratedPortalAgentNetworkStatusRequireObservedFlags(record: Readonly<Record<string, unknown>>, byKind: ReadonlyMap<string, Readonly<Record<string, unknown>>>, flags: readonly { readonly field: string; readonly kind: string }[], label: string): void { flags.forEach(({ field, kind }) => { if (__GeneratedPortalAgentNetworkStatusReadBoolean(record, field) !== byKind.has(kind)) { throw new TypeError(`${label} observed flags must match command evidence`); } }); }
function __GeneratedPortalAgentNetworkStatusRequireLinuxNftablesOutcomes(byKind: ReadonlyMap<string, GeneratedPortalAgentNetworkLinuxNftablesLabCommandRow>): void { __GeneratedPortalAgentNetworkStatusLinuxNftablesExpectedOutcomes.forEach(({ kind, table, chain, rule }) => { const row = byKind.get(kind); if (row === undefined || row['tablePresentAfterCommand'] !== table || row['chainPresentAfterCommand'] !== chain || row['rulePresentAfterCommand'] !== rule) { throw new TypeError('Linux nftables command evidence must match bounded apply and rollback outcomes'); } }); }
function __GeneratedPortalAgentNetworkStatusRequireWindowsFirewallOutcomes(byKind: ReadonlyMap<string, GeneratedPortalAgentNetworkWindowsFirewallLabCommandRow>): void { __GeneratedPortalAgentNetworkStatusWindowsFirewallExpectedOutcomes.forEach(({ kind, rulePresentAfterCommand }) => { const row = byKind.get(kind); if (row === undefined || row['rulePresentAfterCommand'] !== rulePresentAfterCommand) { throw new TypeError('Windows firewall command evidence must match bounded apply and rollback outcomes'); } }); }
function __GeneratedPortalAgentNetworkStatusGateProofReadyIsValid(capabilityReady: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityReady && proofReady && boundaryReasons.length === 0 && missingRequiredArtifacts.length === 0; }
function __GeneratedPortalAgentNetworkStatusGateManualRequiredIsValid(capabilityManualRequired: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityManualRequired || boundaryReasons.length > 0 || missingRequiredArtifacts.length > 0 || !proofReady; }
function __GeneratedPortalAgentNetworkStatusRequireGateConsistency(label: string, proofReadyGate: boolean, proofReadyValid: boolean, manualRequiredGate: boolean, manualRequiredValid: boolean): void { if (proofReadyGate && !proofReadyValid) { throw new TypeError(`${label} proof-ready status must preserve bounded proof invariants`); } if (manualRequiredGate && !manualRequiredValid) { throw new TypeError(`${label} manual-required status must preserve bounded blockers`); } }
export function decodeGeneratedPortalAgentNetworkRemoteDeliveryStatus(value: unknown): GeneratedPortalAgentNetworkRemoteDeliveryStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network remote delivery status'); for (const field of __GeneratedPortalAgentNetworkStatusRemoteStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusRemoteCountFields) { __GeneratedPortalAgentNetworkStatusReadCount(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusRemoteZeroFields) { __GeneratedPortalAgentNetworkStatusReadRequiredCount(record, field, 0); } for (const field of __GeneratedPortalAgentNetworkStatusRemoteBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusRemoteFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } for (const field of __GeneratedPortalAgentNetworkStatusRemoteTrueFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, true); } __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'brokerStatus', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryStatusState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'familyHubStatus', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryStatusState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'transportDispatchState', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryTransportDispatchState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'providerDeliveryReadinessState', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'childDeviceDeliveryReadinessState', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryProviderChildReadinessState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'crossProcessCustodyReadinessState', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'externalCrossProcessTransportState', Object.values(GeneratedPortalAgentNetworkRemoteDeliveryExternalCrossProcessTransportState)); return record as GeneratedPortalAgentNetworkRemoteDeliveryStatus; }
export function decodeGeneratedPortalAgentNetworkLiveCaptureStatusRow(value: unknown): GeneratedPortalAgentNetworkLiveCaptureStatusRow { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network live capture status row'); for (const field of __GeneratedPortalAgentNetworkStatusLiveRowStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveRowNullableStringFields) { __GeneratedPortalAgentNetworkStatusReadNullableString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveRowCountFields) { __GeneratedPortalAgentNetworkStatusReadCount(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveRowBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveRowFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } for (const field of __GeneratedPortalAgentNetworkStatusLiveRowZeroFields) { __GeneratedPortalAgentNetworkStatusReadRequiredCount(record, field, 0); } __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'platform', Object.values(GeneratedPortalAgentNetworkLiveCapturePlatform)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'proofState', Object.values(GeneratedPortalAgentNetworkLiveCaptureProofState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'storageState', Object.values(GeneratedPortalAgentNetworkRawCaptureStorageState)); __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'executionState', Object.values(GeneratedPortalAgentNetworkLiveCaptureExecutionState)); return record as GeneratedPortalAgentNetworkLiveCaptureStatusRow; }
export function decodeGeneratedPortalAgentNetworkLiveCaptureStatus(value: unknown): GeneratedPortalAgentNetworkLiveCaptureStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network live capture status'); for (const field of __GeneratedPortalAgentNetworkStatusLiveStatusStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveStatusCountFields) { __GeneratedPortalAgentNetworkStatusReadCount(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLiveStatusZeroFields) { __GeneratedPortalAgentNetworkStatusReadRequiredCount(record, field, 0); } const rows = record['rows']; if (!Array.isArray(rows)) { throw new TypeError('rows must be a network live capture status row array'); } rows.forEach((row) => decodeGeneratedPortalAgentNetworkLiveCaptureStatusRow(row)); return record as GeneratedPortalAgentNetworkLiveCaptureStatus; }
export function decodeGeneratedPortalAgentNetworkLinuxNftablesLabCommandRow(value: unknown): GeneratedPortalAgentNetworkLinuxNftablesLabCommandRow { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Linux nftables lab command row'); const kind = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'kind', Object.values(GeneratedPortalAgentNetworkLinuxNftablesLabCommandKind)); __GeneratedPortalAgentNetworkStatusReadString(record, 'commandRef'); __GeneratedPortalAgentNetworkStatusReadInteger(record, 'exitStatus'); __GeneratedPortalAgentNetworkStatusReadString(record, 'outputSha256'); __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'tablePresentAfterCommand'); __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'chainPresentAfterCommand'); __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as GeneratedPortalAgentNetworkLinuxNftablesLabCommandRow; }
export function decodeGeneratedPortalAgentNetworkLinuxNftablesLabStatus(value: unknown): GeneratedPortalAgentNetworkLinuxNftablesLabStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Linux nftables lab status'); for (const field of __GeneratedPortalAgentNetworkStatusLinuxNftablesStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); for (const field of __GeneratedPortalAgentNetworkStatusLinuxNftablesBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusLinuxNftablesFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } const state = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'state', Object.values(GeneratedPortalAgentNetworkLinuxNftablesLabState)); const commandEvidence = __GeneratedPortalAgentNetworkStatusReadRecordArray(record, 'commandEvidence', 'network Linux nftables lab command row').map((row) => decodeGeneratedPortalAgentNetworkLinuxNftablesLabCommandRow(row)); __GeneratedPortalAgentNetworkStatusRequireCountMatches(record, 'commandCount', commandEvidence.length); __GeneratedPortalAgentNetworkStatusRequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === GeneratedPortalAgentNetworkLinuxNftablesLabState.ExecutedAndRolledBack) { const byKind = __GeneratedPortalAgentNetworkStatusRequireUniqueRowsByKind(commandEvidence, 'Linux nftables lab'); __GeneratedPortalAgentNetworkStatusRequireObservedFlags(record, byKind, __GeneratedPortalAgentNetworkStatusLinuxNftablesObservedFlags, 'Linux nftables lab'); __GeneratedPortalAgentNetworkStatusRequireLinuxNftablesOutcomes(byKind); } return { ...record, commandEvidence } as GeneratedPortalAgentNetworkLinuxNftablesLabStatus; }
export function decodeGeneratedPortalAgentNetworkWindowsFirewallLabCommandRow(value: unknown): GeneratedPortalAgentNetworkWindowsFirewallLabCommandRow { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Windows firewall lab command row'); const kind = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'kind', Object.values(GeneratedPortalAgentNetworkWindowsFirewallLabCommandKind)); __GeneratedPortalAgentNetworkStatusReadString(record, 'commandRef'); __GeneratedPortalAgentNetworkStatusReadInteger(record, 'exitStatus'); __GeneratedPortalAgentNetworkStatusReadString(record, 'outputSha256'); __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as GeneratedPortalAgentNetworkWindowsFirewallLabCommandRow; }
export function decodeGeneratedPortalAgentNetworkWindowsFirewallLabStatus(value: unknown): GeneratedPortalAgentNetworkWindowsFirewallLabStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Windows firewall lab status'); for (const field of __GeneratedPortalAgentNetworkStatusWindowsFirewallStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); for (const field of __GeneratedPortalAgentNetworkStatusWindowsFirewallBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusWindowsFirewallFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } const state = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'state', Object.values(GeneratedPortalAgentNetworkWindowsFirewallLabState)); const commandEvidence = __GeneratedPortalAgentNetworkStatusReadRecordArray(record, 'commandEvidence', 'network Windows firewall lab command row').map((row) => decodeGeneratedPortalAgentNetworkWindowsFirewallLabCommandRow(row)); __GeneratedPortalAgentNetworkStatusRequireCountMatches(record, 'commandCount', commandEvidence.length); __GeneratedPortalAgentNetworkStatusRequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === GeneratedPortalAgentNetworkWindowsFirewallLabState.ExecutedAndRolledBack) { const byKind = __GeneratedPortalAgentNetworkStatusRequireUniqueRowsByKind(commandEvidence, 'Windows firewall lab'); __GeneratedPortalAgentNetworkStatusRequireObservedFlags(record, byKind, __GeneratedPortalAgentNetworkStatusWindowsFirewallObservedFlags, 'Windows firewall lab'); __GeneratedPortalAgentNetworkStatusRequireWindowsFirewallOutcomes(byKind); } return { ...record, commandEvidence } as GeneratedPortalAgentNetworkWindowsFirewallLabStatus; }
export function decodeGeneratedPortalAgentNetworkWindowsWfpGateStatus(value: unknown): GeneratedPortalAgentNetworkWindowsWfpGateStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Windows WFP gate status'); for (const field of __GeneratedPortalAgentNetworkStatusWindowsWfpStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusWindowsWfpNullableStringFields) { __GeneratedPortalAgentNetworkStatusReadNullableString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusWindowsWfpStringArrayFields) { __GeneratedPortalAgentNetworkStatusReadStringArray(record, field); } const capabilityState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(GeneratedPortalAgentNetworkWindowsWfpCapabilityState)); const gateState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(GeneratedPortalAgentNetworkWindowsWfpGateState)); const boundaryReasons = __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'boundaryReasons'); const missingRequiredArtifacts = __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'missingRequiredArtifacts'); const wfpLabProofReady = __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'wfpLabProofReady'); for (const field of __GeneratedPortalAgentNetworkStatusWindowsWfpFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } __GeneratedPortalAgentNetworkStatusRequireGateConsistency('Windows WFP', gateState === GeneratedPortalAgentNetworkWindowsWfpGateState.LabProofReady, __GeneratedPortalAgentNetworkStatusGateProofReadyIsValid(capabilityState === GeneratedPortalAgentNetworkWindowsWfpCapabilityState.LabReady, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts), gateState === GeneratedPortalAgentNetworkWindowsWfpGateState.ManualRequired, __GeneratedPortalAgentNetworkStatusGateManualRequiredIsValid(capabilityState === GeneratedPortalAgentNetworkWindowsWfpCapabilityState.ManualRequired, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, wfpLabProofReady, enforcementCommandPublished: false } as GeneratedPortalAgentNetworkWindowsWfpGateStatus; }
export function decodeGeneratedPortalAgentNetworkAndroidVpnServiceGateStatus(value: unknown): GeneratedPortalAgentNetworkAndroidVpnServiceGateStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Android VpnService gate status'); for (const field of __GeneratedPortalAgentNetworkStatusAndroidVpnStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusAndroidVpnNullableStringFields) { __GeneratedPortalAgentNetworkStatusReadNullableString(record, field); } __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); const capabilityState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState)); const gateState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(GeneratedPortalAgentNetworkAndroidVpnServiceGateState)); const boundaryReasons = __GeneratedPortalAgentNetworkStatusReadLiteralArray(record, 'boundaryReasons', Object.values(GeneratedPortalAgentNetworkAndroidVpnServiceBoundaryReason)); const missingRequiredArtifacts = __GeneratedPortalAgentNetworkStatusReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(GeneratedPortalAgentNetworkAndroidVpnServiceRequiredArtifact)); for (const field of __GeneratedPortalAgentNetworkStatusAndroidVpnBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } const physicalDeviceProofReady = __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'physicalDeviceProofReady'); for (const field of __GeneratedPortalAgentNetworkStatusAndroidVpnFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } __GeneratedPortalAgentNetworkStatusRequireGateConsistency('Android VpnService', gateState === GeneratedPortalAgentNetworkAndroidVpnServiceGateState.PhysicalDeviceProofReady, __GeneratedPortalAgentNetworkStatusGateProofReadyIsValid(capabilityState === GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState.PhysicalDeviceReady, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts), gateState === GeneratedPortalAgentNetworkAndroidVpnServiceGateState.ManualRequired, __GeneratedPortalAgentNetworkStatusGateManualRequiredIsValid(capabilityState === GeneratedPortalAgentNetworkAndroidVpnServiceCapabilityState.ManualRequired, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, gateState, physicalDeviceProofReady, enforcementCommandPublished: false } as GeneratedPortalAgentNetworkAndroidVpnServiceGateStatus; }
export function decodeGeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus(value: unknown): GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus { const record = __GeneratedPortalAgentNetworkStatusReadRecord(value, 'network Apple Network Extension gate status'); for (const field of __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionStringFields) { __GeneratedPortalAgentNetworkStatusReadString(record, field); } for (const field of __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionNullableStringFields) { __GeneratedPortalAgentNetworkStatusReadNullableString(record, field); } __GeneratedPortalAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); const platform = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'platform', Object.values(GeneratedPortalAgentNetworkAppleNetworkExtensionPlatform)); const capabilityState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState)); const gateState = __GeneratedPortalAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(GeneratedPortalAgentNetworkAppleNetworkExtensionGateState)); const boundaryReasons = __GeneratedPortalAgentNetworkStatusReadLiteralArray(record, 'boundaryReasons', Object.values(GeneratedPortalAgentNetworkAppleNetworkExtensionBoundaryReason)); const missingRequiredArtifacts = __GeneratedPortalAgentNetworkStatusReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(GeneratedPortalAgentNetworkAppleNetworkExtensionRequiredArtifact)); for (const field of __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionBooleanFields) { __GeneratedPortalAgentNetworkStatusReadBoolean(record, field); } const appleEntitlementProofReady = __GeneratedPortalAgentNetworkStatusReadBoolean(record, 'appleEntitlementProofReady'); for (const field of __GeneratedPortalAgentNetworkStatusAppleNetworkExtensionFalseFields) { __GeneratedPortalAgentNetworkStatusReadRequiredBoolean(record, field, false); } __GeneratedPortalAgentNetworkStatusRequireGateConsistency('Apple Network Extension', gateState === GeneratedPortalAgentNetworkAppleNetworkExtensionGateState.AppleEntitlementProofReady, __GeneratedPortalAgentNetworkStatusGateProofReadyIsValid(capabilityState === GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState.AppleDeviceReady, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts), gateState === GeneratedPortalAgentNetworkAppleNetworkExtensionGateState.ManualRequired, __GeneratedPortalAgentNetworkStatusGateManualRequiredIsValid(capabilityState === GeneratedPortalAgentNetworkAppleNetworkExtensionCapabilityState.ManualRequired, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, platform, gateState, appleEntitlementProofReady, enforcementCommandPublished: false } as GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus; }
export const GeneratedPortalAgentNetworkRemoteDeliveryStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkRemoteDeliveryStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkRemoteDeliveryStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkLiveCaptureStatusRowSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkLiveCaptureStatusRow } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkLiveCaptureStatusRow(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkLiveCaptureStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkLiveCaptureStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkLiveCaptureStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkLinuxNftablesLabStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkLinuxNftablesLabStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkLinuxNftablesLabStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkWindowsFirewallLabStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkWindowsFirewallLabStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkWindowsFirewallLabStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkWindowsWfpGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkWindowsWfpGateStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkWindowsWfpGateStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkAndroidVpnServiceGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkAndroidVpnServiceGateStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkAndroidVpnServiceGateStatus(value) }; } catch { return { success: false }; } } } as const;
export const GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentNetworkAppleNetworkExtensionGateStatus(value) }; } catch { return { success: false }; } } } as const;
 export const GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults = { CommandId: "tracking-retention-settings-write-command", SettingsKindRetentionWindow: "retention-window-setting", WriterIntentRef: "tracking-retention-settings-write-retention-window", ReadModelProofRefs: ["output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json", "output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json"], MutationProofRef: "output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json", LocalServiceStateSnapshotRef: "agent-service-local-retention-settings-state", DurableSettingsStoreRef: "agent-service-local-retention-settings-durable-json", WriteStateAccepted: "service-write-command-accepted", WriteStateRejected: "service-write-command-rejected", AcceptedAt: "2026-06-06T19:50:00Z" } as const; export const GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState = { DeleteAfterAlertResolved: "delete-after-alert-resolved", RetainAfterAlertResolved: "retain-after-alert-resolved" } as const; export type GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState = (typeof GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState)[keyof typeof GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState]; export const GeneratedPortalAgentTrackingParentExportState = { Prepared: "prepared", NotPrepared: "not-prepared" } as const; export type GeneratedPortalAgentTrackingParentExportState = (typeof GeneratedPortalAgentTrackingParentExportState)[keyof typeof GeneratedPortalAgentTrackingParentExportState]; export const GeneratedPortalAgentTrackingRemoteSyncState = { Enabled: "enabled", Disabled: "disabled" } as const; export type GeneratedPortalAgentTrackingRemoteSyncState = (typeof GeneratedPortalAgentTrackingRemoteSyncState)[keyof typeof GeneratedPortalAgentTrackingRemoteSyncState]; export const GeneratedPortalAgentTrackingRemoteAiState = { Enabled: "enabled", Disabled: "disabled" } as const; export type GeneratedPortalAgentTrackingRemoteAiState = (typeof GeneratedPortalAgentTrackingRemoteAiState)[keyof typeof GeneratedPortalAgentTrackingRemoteAiState]; export const GeneratedPortalAgentTrackingDurableSettingsPersistenceState = { Persisted: "persisted", NotPersisted: "not-persisted" } as const; export type GeneratedPortalAgentTrackingDurableSettingsPersistenceState = (typeof GeneratedPortalAgentTrackingDurableSettingsPersistenceState)[keyof typeof GeneratedPortalAgentTrackingDurableSettingsPersistenceState]; export const GeneratedPortalAgentTrackingConfigAckState = { Received: "received", Missing: "missing" } as const; export type GeneratedPortalAgentTrackingConfigAckState = (typeof GeneratedPortalAgentTrackingConfigAckState)[keyof typeof GeneratedPortalAgentTrackingConfigAckState]; export const GeneratedPortalAgentTrackingExecutionClaimState = { Claimed: "claimed", Unclaimed: "unclaimed" } as const; export type GeneratedPortalAgentTrackingExecutionClaimState = (typeof GeneratedPortalAgentTrackingExecutionClaimState)[keyof typeof GeneratedPortalAgentTrackingExecutionClaimState]; export const GeneratedPortalAgentTrackingConfigUpdateResponseState = { Applied: "applied", Rejected: "rejected" } as const; export type GeneratedPortalAgentTrackingConfigUpdateResponseState = (typeof GeneratedPortalAgentTrackingConfigUpdateResponseState)[keyof typeof GeneratedPortalAgentTrackingConfigUpdateResponseState]; export const GeneratedPortalAgentTrackingEffectiveState = { Enabled: "enabled", Disabled: "disabled", Degraded: "degraded" } as const; export type GeneratedPortalAgentTrackingEffectiveState = (typeof GeneratedPortalAgentTrackingEffectiveState)[keyof typeof GeneratedPortalAgentTrackingEffectiveState];
export type GeneratedPortalAgentTrackingRetentionSettingsWriteResult = { readonly schemaVersion: number; readonly commandId: string; readonly settingsKind: string; readonly writeState: string; readonly acceptedAt: string; readonly sourceWriterIntentRefs: readonly string[]; readonly sourceReadModelProofRefs: readonly string[]; readonly sourceMutationProofRefs: readonly string[]; readonly appliedRetentionWindowHours: number | null; readonly appliedDeleteAfterAlertResolutionState: GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState; readonly parentExportState: GeneratedPortalAgentTrackingParentExportState; readonly remoteSyncState: typeof GeneratedPortalAgentTrackingRemoteSyncState.Disabled; readonly remoteAiState: typeof GeneratedPortalAgentTrackingRemoteAiState.Disabled; readonly localServiceStateRevision: number | null; readonly localServiceStateSnapshotRef: string; readonly durableSettingsStoreRef: string; readonly durableSettingsPersistenceState: GeneratedPortalAgentTrackingDurableSettingsPersistenceState; readonly childConfigResponseState?: GeneratedPortalAgentTrackingConfigUpdateResponseState | null; readonly effectiveTrackingState?: GeneratedPortalAgentTrackingEffectiveState | null; readonly childConfigAckState: GeneratedPortalAgentTrackingConfigAckState; readonly commandTransportClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Claimed; readonly serviceWritePreflightClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Claimed; readonly serviceMutationExecutionState: GeneratedPortalAgentTrackingExecutionClaimState; readonly portalWritableUiClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly platformRuntimeClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly childDeviceDeliveryClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly providerDeliveryClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly notificationReceiptClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly physicalDeviceClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly authorityClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; readonly productClaimState: typeof GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed; };
type GeneratedPortalAgentTrackingRetentionSettingsWriteResultOptionals = { childConfigResponseState?: GeneratedPortalAgentTrackingConfigUpdateResponseState | null; effectiveTrackingState?: GeneratedPortalAgentTrackingEffectiveState | null };
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultIsRecord(candidate: unknown): candidate is Readonly<Record<string, unknown>> { return typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(record: Readonly<Record<string, unknown>>, field: string): string { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty tracking retention string`); } return fieldValue; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { if (record['schemaVersion'] !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return GeneratedPortalAgentProtocolRuntime.SchemaVersion; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadNullableNumber(record: Readonly<Record<string, unknown>>, field: string): number | null { const fieldValue = record[field]; if (fieldValue === null) { return null; } if (typeof fieldValue !== 'number' || !Number.isInteger(fieldValue) || fieldValue <= 0) { throw new TypeError(`${field} must be a positive integer or null`); } return fieldValue; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const fieldValue = record[field]; if (!Array.isArray(fieldValue) || fieldValue.length === 0 || fieldValue.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a non-empty string array`); } return fieldValue as readonly string[]; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const fieldValue = decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const fieldValue = decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(record, field); if (fieldValue !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T | null | undefined { const fieldValue = record[field]; if (fieldValue === undefined) { return undefined; } if (fieldValue === null) { return null; } if (typeof fieldValue !== 'string' || !allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadAckState(record: Readonly<Record<string, unknown>>): GeneratedPortalAgentTrackingConfigAckState { if (record['childConfigAckState'] === undefined) { return GeneratedPortalAgentTrackingConfigAckState.Missing; } return decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(record, 'childConfigAckState', Object.values(GeneratedPortalAgentTrackingConfigAckState)); }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultAttachOptionals(result: GeneratedPortalAgentTrackingRetentionSettingsWriteResult, childConfigResponseState: GeneratedPortalAgentTrackingConfigUpdateResponseState | null | undefined, effectiveTrackingState: GeneratedPortalAgentTrackingEffectiveState | null | undefined): GeneratedPortalAgentTrackingRetentionSettingsWriteResult { const resultWithOptionals = result as GeneratedPortalAgentTrackingRetentionSettingsWriteResult & GeneratedPortalAgentTrackingRetentionSettingsWriteResultOptionals; if (childConfigResponseState !== undefined) { resultWithOptionals.childConfigResponseState = childConfigResponseState; } if (effectiveTrackingState !== undefined) { resultWithOptionals.effectiveTrackingState = effectiveTrackingState; } return resultWithOptionals; }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultRequireAcceptedInvariants(result: GeneratedPortalAgentTrackingRetentionSettingsWriteResult): void { if (result.writeState !== GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted) { return; } if (result.commandTransportClaimState !== GeneratedPortalAgentTrackingExecutionClaimState.Claimed) { throw new TypeError('accepted tracking write result must prove command transport'); } if (result.serviceMutationExecutionState !== GeneratedPortalAgentTrackingExecutionClaimState.Claimed) { throw new TypeError('accepted tracking write result must execute local mutation'); } if (result.localServiceStateRevision === null) { throw new TypeError('accepted tracking write result must include local service revision'); } if (result.durableSettingsPersistenceState !== GeneratedPortalAgentTrackingDurableSettingsPersistenceState.Persisted) { throw new TypeError('accepted tracking write result must persist durable settings'); } }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultRequireRetentionWindowInvariant(result: GeneratedPortalAgentTrackingRetentionSettingsWriteResult): void { if (result.settingsKind === GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow && result.appliedRetentionWindowHours === null) { throw new TypeError('retention-window write result must include applied retention window'); } }
function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultFinalize(result: GeneratedPortalAgentTrackingRetentionSettingsWriteResult, childConfigResponseState: GeneratedPortalAgentTrackingConfigUpdateResponseState | null | undefined, effectiveTrackingState: GeneratedPortalAgentTrackingEffectiveState | null | undefined): GeneratedPortalAgentTrackingRetentionSettingsWriteResult { const resultWithOptionals = decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultAttachOptionals(result, childConfigResponseState, effectiveTrackingState); decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultRequireAcceptedInvariants(resultWithOptionals); decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultRequireRetentionWindowInvariant(resultWithOptionals); return resultWithOptionals; }
export function decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResult(value: unknown): GeneratedPortalAgentTrackingRetentionSettingsWriteResult { if (!decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultIsRecord(value)) { throw new TypeError('tracking retention write result must be an object'); } const childConfigResponseState = decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral(value, 'childConfigResponseState', Object.values(GeneratedPortalAgentTrackingConfigUpdateResponseState)); const effectiveTrackingState = decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral(value, 'effectiveTrackingState', Object.values(GeneratedPortalAgentTrackingEffectiveState)); const result: GeneratedPortalAgentTrackingRetentionSettingsWriteResult = { schemaVersion: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadSchemaVersion(value), commandId: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(value, 'commandId'), settingsKind: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'settingsKind', GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow), writeState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'writeState', [GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted, GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults.WriteStateRejected] as const), acceptedAt: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(value, 'acceptedAt'), sourceWriterIntentRefs: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceWriterIntentRefs'), sourceReadModelProofRefs: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceReadModelProofRefs'), sourceMutationProofRefs: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceMutationProofRefs'), appliedRetentionWindowHours: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadNullableNumber(value, 'appliedRetentionWindowHours'), appliedDeleteAfterAlertResolutionState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'appliedDeleteAfterAlertResolutionState', Object.values(GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState)), parentExportState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'parentExportState', Object.values(GeneratedPortalAgentTrackingParentExportState)), remoteSyncState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'remoteSyncState', GeneratedPortalAgentTrackingRemoteSyncState.Disabled), remoteAiState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'remoteAiState', GeneratedPortalAgentTrackingRemoteAiState.Disabled), localServiceStateRevision: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadNullableNumber(value, 'localServiceStateRevision'), localServiceStateSnapshotRef: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(value, 'localServiceStateSnapshotRef'), durableSettingsStoreRef: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadString(value, 'durableSettingsStoreRef'), durableSettingsPersistenceState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'durableSettingsPersistenceState', Object.values(GeneratedPortalAgentTrackingDurableSettingsPersistenceState)), childConfigAckState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadAckState(value), commandTransportClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'commandTransportClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Claimed), serviceWritePreflightClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'serviceWritePreflightClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Claimed), serviceMutationExecutionState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'serviceMutationExecutionState', Object.values(GeneratedPortalAgentTrackingExecutionClaimState)), portalWritableUiClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'portalWritableUiClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), platformRuntimeClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'platformRuntimeClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), childDeviceDeliveryClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'childDeviceDeliveryClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), providerDeliveryClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'providerDeliveryClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), notificationReceiptClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'notificationReceiptClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), physicalDeviceClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'physicalDeviceClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), authorityClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'authorityClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed), productClaimState: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'productClaimState', GeneratedPortalAgentTrackingExecutionClaimState.Unclaimed) }; return decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResultFinalize(result, childConfigResponseState, effectiveTrackingState); }
export const GeneratedPortalAgentTrackingRetentionSettingsWriteResultSchema = { safeParse(value: unknown): { readonly success: true; readonly data: GeneratedPortalAgentTrackingRetentionSettingsWriteResult } | { readonly success: false } { try { return { success: true, data: decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResult(value) }; } catch { return { success: false }; } } } as const;
 export const GeneratedPortalAgentProtocolDelimiter = { List: ",", EventIdSuffix: "-" } as const; export type GeneratedPortalAgentProtocolDelimiter = (typeof GeneratedPortalAgentProtocolDelimiter)[keyof typeof GeneratedPortalAgentProtocolDelimiter]; export const GeneratedPortalAgentCommand = { HealthCheck: "agent.health.check", LogSnapshotGet: "agent.log.snapshot.get", DevEcho: "agent.dev.echo", WatchStatusGet: "agent.watch.status.get", ActivityIngestStatusGet: "agent.activity.ingest.status.get", ActivityRecentSummaryGet: "agent.activity.recent.summary.get", ActivityMemoryGraphGet: "agent.activity.memory-graph.get", ActivityReportDailyGenerate: "agent.activity.report.daily.generate", ActivityReportWeeklyGenerate: "agent.activity.report.weekly.generate", ActivityReportMonthlyGenerate: "agent.activity.report.monthly.generate", ActivityReportSave: "agent.activity.report.save", ActivityReportHistoryList: "agent.activity.report.history.list", ActivityScreenReadModelGet: "agent.activity.screen.read-model.get", ActivityAppUseReadModelGet: "agent.activity.app-use.read-model.get", ActivityBrowserReadModelGet: "agent.activity.browser.read-model.get", ActivityGamesReadModelGet: "agent.activity.games.read-model.get", ActivityAppGameBoundaryReadModelGet: "agent.activity.app-game.boundary.read-model.get", ActivityAppGamePolicyReadinessReadModelGet: "agent.activity.app-game.policy-readiness.read-model.get", ActivityAppGameNotificationReadinessReadModelGet: "agent.activity.app-game.notification-readiness.read-model.get", ActivityAppGameAdapterExecutionReadinessReadModelGet: "agent.activity.app-game.adapter-execution-readiness.read-model.get", ActivityAppGamePlatformProofStatusReadModelGet: "agent.activity.app-game.platform-proof-status.read-model.get", ActivityAppGameChildRuntimeTransportReceiptReadModelGet: "agent.activity.app-game.child-runtime-transport-receipt.read-model.get", ActivityAppGameAdapterDispatchPreflightReadModelGet: "agent.activity.app-game.adapter-dispatch-preflight.read-model.get", ActivityAppGameAdapterDispatchResultReadModelGet: "agent.activity.app-game.adapter-dispatch-result.read-model.get", ActivityAppGameAdapterDispatchExecute: "agent.activity.app-game.adapter-dispatch.execute", ActivityAppGameTimerParentSurfaceReadModelGet: "agent.activity.app-game.timer-parent-surface.read-model.get", ActivityAppGameTimerParentPreferenceSetupRequest: "agent.activity.app-game.timer-parent-surface.parent-preference-setup.request", BrowserSocialDashboardReadModelGet: "agent.browser.social-dashboard.read-model.get", BrowserSocialAuditExplanationReadModelGet: "agent.browser.social-audit-explanation.read-model.get", BrowserSocialAlertReportReadModelGet: "agent.browser.social-alert-report.read-model.get", BrowserSocialAlertReportParentSurfaceReadModelGet: "agent.browser.social-alert-report.parent-surface.read-model.get", BrowserSocialParentNotificationDeliveryReadModelGet: "agent.browser.social-parent-notification-delivery.read-model.get", BrowserSocialSourceCustodyMutationApply: "agent.browser.social-source-custody.mutation.apply", ActivityNetworkReadModelGet: "agent.activity.network.read-model.get", ActivityTrackingRetentionSettingsWrite: "agent.activity.tracking.retention-settings.write", ParentRuntimeIntentIngressPublish: "agent.parent-runtime.intent-ingress.publish", BrowserEvidenceRecentGet: "agent.browser.evidence.recent.get", BrowserManagedBridgePoll: "agent.browser.managed.bridge.poll", BrowserInventoryReadModelGet: "agent.browser.inventory.read-model.get", BrowserInterventionReadModelGet: "agent.browser.intervention.read-model.get", BrowserRuntimeEventChainStreamGet: "agent.browser.runtime.event-chain.stream.get", NetworkFlowReadModelGet: "agent.network.flow.read-model.get", LanPairingStatusGet: "agent.lan-pairing.status.get", NetworkRuntimeEventChainStreamGet: "agent.network.runtime.event-chain.stream.get", LanRuntimeEventChainStreamGet: "agent.lan.runtime.event-chain.stream.get", NetworkRemoteDeliveryStatusGet: "agent.network.remote-delivery.status.get", NetworkLiveCaptureStatusGet: "agent.network.live-capture.status.get", NetworkLinuxNftablesLabStatusGet: "agent.network.linux-nftables-lab.status.get", NetworkWindowsFirewallLabStatusGet: "agent.network.windows-firewall-lab.status.get", NetworkWindowsWfpGateStatusGet: "agent.network.windows-wfp-gate.status.get", NetworkAndroidVpnServiceGateStatusGet: "agent.network.android-vpn-service-gate.status.get", NetworkAppleNetworkExtensionGateStatusGet: "agent.network.apple-network-extension-gate.status.get", ActivityTrackingReadModelGet: "agent.activity.tracking.read-model.get", LocalAiRuntimeStatusGet: "agent.local-ai.runtime.status.get", LocalAiChatGenerate: "agent.local-ai.chat.generate", ParentAssistantAnswerGenerate: "agent.parent-assistant.answer.generate", PolicyPreviewReadModelGet: "agent.policy.preview.read-model.get", PolicyRequestAssistantPreviewConfirm: "agent.policy.request.assistant-preview.confirm", PolicyRequestParentResolutionResolve: "agent.policy.request.parent-resolution.resolve", BrowserPolicyGet: "agent.browser-policy.get", BrowserPolicyPreview: "agent.browser-policy.preview", BrowserPolicyPatch: "agent.browser-policy.patch", BrowserPolicyReplace: "agent.browser-policy.replace", BrowserPolicyRollback: "agent.browser-policy.rollback", ScreenSettingsGet: "agent.screen-settings.get", ScreenSettingsReplace: "agent.screen-settings.replace", EnforcementExecute: "agent.enforcement.execute", EnforcementTimerRecover: "agent.enforcement.timer.recover", EnforcementTimerExpire: "agent.enforcement.timer.expire", EnforcementOverrideCancel: "agent.enforcement.override.cancel", EnforcementProductControlSpineGet: "agent.enforcement.product-control-spine.get", EnforcementPolicyDispatchGet: "agent.enforcement.policy-dispatch.get", EnforcementBroadAdapterProofGet: "agent.enforcement.broad-adapter-proof.get", EnforcementSupportedAdapterRuntimeProofGet: "agent.enforcement.supported-adapter-runtime-proof.get", ParentAssistantThreadList: "agent.parent-assistant.thread.list", ParentAssistantThreadCreate: "agent.parent-assistant.thread.create", ParentAssistantThreadOpen: "agent.parent-assistant.thread.open", ParentAssistantThreadArchive: "agent.parent-assistant.thread.archive", ParentAssistantMessageSend: "agent.parent-assistant.message.send", ParentAssistantRunCancel: "agent.parent-assistant.run.cancel", ParentAssistantQuickActionStart: "agent.parent-assistant.quick-action.start", ParentAssistantActionPreview: "agent.parent-assistant.action.preview", ParentAssistantActionConfirm: "agent.parent-assistant.action.confirm", ParentAssistantProviderStatusGet: "agent.parent-assistant.provider.status.get", LanPairingProofSubmit: "agent.lan-pairing.proof.submit", LanPairingRouteSelect: "agent.lan-pairing.route.select", LanPairingRouteRevoke: "agent.lan-pairing.route.revoke", LanPairingBrowserDiscoveryScan: "agent.lan-pairing.browser-discovery.scan", LanPairingAddDeviceRequest: "agent.lan-pairing.add-device.request", LanPairingSignedChildAgentObserve: "agent.lan-pairing.signed-child-agent.observe", LanPairingControllerLeaseRenew: "agent.lan-pairing.controller-lease.renew", LanPairingControllerLeaseRelease: "agent.lan-pairing.controller-lease.release", LanPairingControllerLeaseTakeover: "agent.lan-pairing.controller-lease.takeover", LanAiProviderStatusGet: "agent.lan-ai.provider.status.get", LanAiJobSubmit: "agent.lan-ai.job.submit" } as const; export type GeneratedPortalAgentCommandName = (typeof GeneratedPortalAgentCommand)[keyof typeof GeneratedPortalAgentCommand]; export const GeneratedPortalAgentEvent = { ConnectionReady: "agent.connection.ready", CommandRejected: "agent.command.rejected", HealthReported: "agent.health.reported", LogSnapshotReported: "agent.log.snapshot.reported", DevEchoed: "agent.dev.echoed", WatchStatusReported: "agent.watch.status.reported", ActivityIngestStatusReported: "agent.activity.ingest.status.reported", ActivityRecentSummaryReported: "agent.activity.recent.summary.reported", ActivityMemoryGraphReported: "agent.activity.memory-graph.reported", ActivityReportGenerated: "agent.activity.report.generated", ActivityReportSaved: "agent.activity.report.saved", ActivityReportHistoryReported: "agent.activity.report.history.reported", ActivityScreenReadModelReported: "agent.activity.screen.read-model.reported", ActivityAppUseReadModelReported: "agent.activity.app-use.read-model.reported", ActivityBrowserReadModelReported: "agent.activity.browser.read-model.reported", ActivityGamesReadModelReported: "agent.activity.games.read-model.reported", ActivityAppGameBoundaryReadModelReported: "agent.activity.app-game.boundary.read-model.reported", ActivityAppGameNotificationReadinessReadModelReported: "agent.activity.app-game.notification-readiness.read-model.reported", ActivityAppGameAdapterExecutionReadinessReadModelReported: "agent.activity.app-game.adapter-execution-readiness.read-model.reported", ActivityAppGamePlatformProofStatusReadModelReported: "agent.activity.app-game.platform-proof-status.read-model.reported", ActivityAppGameChildRuntimeTransportReceiptReadModelReported: "agent.activity.app-game.child-runtime-transport-receipt.read-model.reported", ActivityAppGameAdapterDispatchPreflightReadModelReported: "agent.activity.app-game.adapter-dispatch-preflight.read-model.reported", ActivityAppGameAdapterDispatchResultReadModelReported: "agent.activity.app-game.adapter-dispatch-result.read-model.reported", ActivityAppGameAdapterDispatchExecuted: "agent.activity.app-game.adapter-dispatch.executed", ActivityAppGameTimerParentSurfaceReadModelReported: "agent.activity.app-game.timer-parent-surface.read-model.reported", ActivityAppGameTimerParentPreferenceSetupRequested: "agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested", BrowserSocialDashboardReadModelReported: "agent.browser.social-dashboard.read-model.reported", BrowserSocialAuditExplanationReadModelReported: "agent.browser.social-audit-explanation.read-model.reported", BrowserSocialAlertReportReadModelReported: "agent.browser.social-alert-report.read-model.reported", BrowserSocialAlertReportParentSurfaceReadModelReported: "agent.browser.social-alert-report.parent-surface.read-model.reported", BrowserSocialParentNotificationDeliveryReadModelReported: "agent.browser.social-parent-notification-delivery.read-model.reported", BrowserSocialSourceCustodyMutationApplied: "agent.browser.social-source-custody.mutation.applied", ActivityNetworkReadModelReported: "agent.activity.network.read-model.reported", BrowserEvidenceRecentReported: "agent.browser.evidence.recent.reported", BrowserManagedStatusReported: "agent.browser.managed.status.reported", BrowserInventoryReadModelReported: "agent.browser.inventory.read-model.reported", BrowserInterventionReadModelReported: "agent.browser.intervention.read-model.reported", BrowserRuntimeEventChainStreamReported: "agent.browser.runtime.event-chain.stream.reported", NetworkFlowReadModelReported: "agent.network.flow.read-model.reported", NetworkRuntimeEventChainStreamReported: "agent.network.runtime.event-chain.stream.reported", LanRuntimeEventChainStreamReported: "agent.lan.runtime.event-chain.stream.reported", NetworkRemoteDeliveryStatusReported: "agent.network.remote-delivery.status.reported", NetworkLiveCaptureStatusReported: "agent.network.live-capture.status.reported", NetworkLinuxNftablesLabStatusReported: "agent.network.linux-nftables-lab.status.reported", NetworkWindowsFirewallLabStatusReported: "agent.network.windows-firewall-lab.status.reported", NetworkWindowsWfpGateStatusReported: "agent.network.windows-wfp-gate.status.reported", NetworkAndroidVpnServiceGateStatusReported: "agent.network.android-vpn-service-gate.status.reported", NetworkAppleNetworkExtensionGateStatusReported: "agent.network.apple-network-extension-gate.status.reported", ActivityTrackingReadModelReported: "agent.activity.tracking.read-model.reported", ActivityTrackingRetentionSettingsWriteReported: "agent.activity.tracking.retention-settings.write.reported", ParentRuntimeIntentIngressReported: "agent.parent-runtime.intent-ingress.reported", LocalAiRuntimeStatusReported: "agent.local-ai.runtime.status.reported", LocalAiChatGenerationReported: "agent.local-ai.chat.generation.reported", PolicyPreviewReadModelReported: "agent.policy.preview.read-model.reported", PolicyRequestAssistantPreviewConfirmReported: "agent.policy.request.assistant-preview.confirm.reported", PolicyRequestParentResolutionResolved: "agent.policy.request.parent-resolution.resolved", BrowserPolicyReported: "agent.browser-policy.reported", BrowserPolicyPreviewed: "agent.browser-policy.previewed", BrowserPolicyPatchAccepted: "agent.browser-policy.patch.accepted", BrowserPolicyPatchRejected: "agent.browser-policy.patch.rejected", BrowserPolicyReplaceAccepted: "agent.browser-policy.replace.accepted", BrowserPolicyReplaceRejected: "agent.browser-policy.replace.rejected", BrowserPolicyRollbackAccepted: "agent.browser-policy.rollback.accepted", BrowserPolicyRollbackRejected: "agent.browser-policy.rollback.rejected", ScreenSettingsReported: "agent.screen-settings.reported", ScreenSettingsReplaceAccepted: "agent.screen-settings.replace.accepted", ScreenSettingsReplaceRejected: "agent.screen-settings.replace.rejected", EnforcementAuditReported: "agent.enforcement.audit.reported", EnforcementTimerReported: "agent.enforcement.timer.reported", EnforcementProductControlSpineReported: "agent.enforcement.product-control-spine.reported", EnforcementPolicyDispatchReported: "agent.enforcement.policy-dispatch.reported", EnforcementBroadAdapterProofReported: "agent.enforcement.broad-adapter-proof.reported", EnforcementSupportedAdapterRuntimeProofReported: "agent.enforcement.supported-adapter-runtime-proof.reported", ActivityAppGamePolicyReadinessReadModelReported: "agent.activity.app-game.policy-readiness.read-model.reported", ParentAssistantAnswerReported: "agent.parent-assistant.answer.reported", ParentAssistantThreadUpdated: "agent.parent-assistant.thread.updated", ParentAssistantMessageAccepted: "agent.parent-assistant.message.accepted", ParentAssistantRunStarted: "agent.parent-assistant.run.started", ParentAssistantMessageDelta: "agent.parent-assistant.message.delta", ParentAssistantMessageCompleted: "agent.parent-assistant.message.completed", ParentAssistantActionPreviewed: "agent.parent-assistant.action.previewed", ParentAssistantActionConfirmed: "agent.parent-assistant.action.confirmed", ParentAssistantProviderDegraded: "agent.parent-assistant.provider.degraded", ParentAssistantErrorReported: "agent.parent-assistant.error.reported", LanPairingStatusReported: "agent.lan-pairing.status.reported", LanPairingBrowserDiscoveryReported: "agent.lan-pairing.browser-discovery.reported", LanPairingAddDeviceReported: "agent.lan-pairing.add-device.reported", LanPairingSignedChildAgentReported: "agent.lan-pairing.signed-child-agent.reported", LanPairingAuditReported: "agent.lan-pairing.audit.reported", LanAiJobReported: "agent.lan-ai.job.reported" } as const; export type GeneratedPortalAgentEventName = (typeof GeneratedPortalAgentEvent)[keyof typeof GeneratedPortalAgentEvent]; export const GeneratedPortalAgentActivitySurfaceSchemaVersion = 1 as const; export const GeneratedPortalAgentActivitySurfaceScopeKind = { Family: "family", Device: "device" } as const; export type GeneratedPortalAgentActivitySurfaceScopeKind = (typeof GeneratedPortalAgentActivitySurfaceScopeKind)[keyof typeof GeneratedPortalAgentActivitySurfaceScopeKind]; export const GeneratedPortalAgentActivityReportFrequency = { Daily: "daily", Weekly: "weekly", Monthly: "monthly" } as const; export type GeneratedPortalAgentActivityReportFrequency = (typeof GeneratedPortalAgentActivityReportFrequency)[keyof typeof GeneratedPortalAgentActivityReportFrequency]; export const GeneratedPortalAgentActivityReportSectionKind = { Summary: "summary", Screen: "screen", AppUse: "app-use", Browser: "browser", Games: "games", Network: "network" } as const; export type GeneratedPortalAgentActivityReportSectionKind = (typeof GeneratedPortalAgentActivityReportSectionKind)[keyof typeof GeneratedPortalAgentActivityReportSectionKind]; export const GeneratedPortalAgentActivityReadModelState = { Ready: "ready", Empty: "empty", Unavailable: "unavailable", Offline: "offline", Stale: "stale", PermissionRequired: "permission-required", ScaffoldOnly: "scaffold-only" } as const; export type GeneratedPortalAgentActivityReadModelState = (typeof GeneratedPortalAgentActivityReadModelState)[keyof typeof GeneratedPortalAgentActivityReadModelState]; export const GeneratedPortalAgentActivityReportSourceReachabilityState = { Reachable: "reachable", Unreachable: "unreachable", Offline: "offline", Error: "error" } as const; export type GeneratedPortalAgentActivityReportSourceReachabilityState = (typeof GeneratedPortalAgentActivityReportSourceReachabilityState)[keyof typeof GeneratedPortalAgentActivityReportSourceReachabilityState]; export const GeneratedPortalAgentActivitySavedReportState = { Draft: "draft", Saved: "saved", StorageUnavailable: "storage-unavailable", Degraded: "degraded", ScaffoldOnly: "scaffold-only" } as const; export type GeneratedPortalAgentActivitySavedReportState = (typeof GeneratedPortalAgentActivitySavedReportState)[keyof typeof GeneratedPortalAgentActivitySavedReportState]; export const GeneratedPortalAgentActivityReportCustodyLabel = { ChildDeviceLocalSummary: "child-device-local-summary", ParentDeviceLocalReportJson: "parent-device-local-report-json", ParentDeviceLocalHistory: "parent-device-local-history" } as const; export type GeneratedPortalAgentActivityReportCustodyLabel = (typeof GeneratedPortalAgentActivityReportCustodyLabel)[keyof typeof GeneratedPortalAgentActivityReportCustodyLabel]; export const GeneratedPortalAgentActivityReportSourceLabel = { ActivityQueryStoreSummary: "activity-query-store-summary", FamilyFanoutSourceState: "family-fanout-source-state", SavedReportJson: "saved-report-json", SavedReportHistory: "saved-report-history" } as const; export type GeneratedPortalAgentActivityReportSourceLabel = (typeof GeneratedPortalAgentActivityReportSourceLabel)[keyof typeof GeneratedPortalAgentActivityReportSourceLabel]; export const GeneratedPortalAgentActivityEvidenceKind = { JournalEntry: "journal-entry", Screenshot: "screenshot", StorageObject: "storage-object", LocalDbRow: "local-db-row" } as const; export type GeneratedPortalAgentActivityEvidenceKind = (typeof GeneratedPortalAgentActivityEvidenceKind)[keyof typeof GeneratedPortalAgentActivityEvidenceKind];
export const GeneratedPortalAgentActivitySurfaceReadModelKindName = { Screen: GeneratedPortalAgentActivityReportSectionKind.Screen, AppUse: GeneratedPortalAgentActivityReportSectionKind.AppUse, Browser: GeneratedPortalAgentActivityReportSectionKind.Browser, Games: GeneratedPortalAgentActivityReportSectionKind.Games, Network: GeneratedPortalAgentActivityReportSectionKind.Network } as const;
export type GeneratedPortalAgentActivitySurfaceReadModelKind = (typeof GeneratedPortalAgentActivitySurfaceReadModelKindName)[keyof typeof GeneratedPortalAgentActivitySurfaceReadModelKindName];
export type GeneratedPortalAgentActivitySurfaceSchemaParser<T> = { readonly parse: (input: unknown) => T; readonly safeParse: (input: unknown) => { readonly success: true; readonly data: T } | { readonly success: false } };
export type GeneratedPortalAgentActivityEvidenceRef = { readonly evidenceId: string; readonly kind: GeneratedPortalAgentActivityEvidenceKind; readonly digest: string | null; readonly uri: string | null };
export type GeneratedPortalAgentActivitySurfaceScope = { readonly scopeKind: GeneratedPortalAgentActivitySurfaceScopeKind; readonly familyId: string | null; readonly deviceId: string | null };
export type GeneratedPortalAgentActivitySurfaceRequest = { readonly schemaVersion: typeof GeneratedPortalAgentActivitySurfaceSchemaVersion; readonly scope: GeneratedPortalAgentActivitySurfaceScope; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string };
export type GeneratedPortalAgentActivityReportSourceState = { readonly deviceId: string; readonly reachabilityState: GeneratedPortalAgentActivityReportSourceReachabilityState; readonly state: GeneratedPortalAgentActivityReadModelState; readonly reason: string | null; readonly lastUpdatedAt: string | null; readonly custodyLabel: GeneratedPortalAgentActivityReportCustodyLabel; readonly sourceLabel: GeneratedPortalAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type GeneratedPortalAgentActivityReportSection = { readonly sectionKind: GeneratedPortalAgentActivityReportSectionKind; readonly title: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly summary: string; readonly itemCount: number; readonly evidence: readonly GeneratedPortalAgentActivityEvidenceRef[] };
export type GeneratedPortalAgentActivitySavedReportMetadata = { readonly reportId: string; readonly fileName: string; readonly savedState: GeneratedPortalAgentActivitySavedReportState; readonly savedAt: string | null; readonly storageReason: string | null; readonly custodyLabel: GeneratedPortalAgentActivityReportCustodyLabel; readonly sourceLabel: GeneratedPortalAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type GeneratedPortalAgentActivityReportSourceStateSummary = { readonly totalSources: number; readonly readySources: number; readonly offlineSources: number; readonly staleSources: number; readonly unavailableSources: number; readonly unreachableSources: number; readonly errorSources: number };
export type GeneratedPortalAgentActivityReportDocument = { readonly schemaVersion: typeof GeneratedPortalAgentActivitySurfaceSchemaVersion; readonly reportId: string; readonly frequency: GeneratedPortalAgentActivityReportFrequency; readonly scope: GeneratedPortalAgentActivitySurfaceScope; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string; readonly generatedAt: string; readonly savedMetadata: GeneratedPortalAgentActivitySavedReportMetadata | null; readonly sourceStates: readonly GeneratedPortalAgentActivityReportSourceState[]; readonly sections: readonly GeneratedPortalAgentActivityReportSection[] };
export type GeneratedPortalAgentActivityHistoricalReportListItem = { readonly schemaVersion: typeof GeneratedPortalAgentActivitySurfaceSchemaVersion; readonly reportId: string; readonly fileName: string; readonly reportDate: string; readonly rangeStart: string; readonly rangeEnd: string; readonly summary: string; readonly savedState: GeneratedPortalAgentActivitySavedReportState; readonly savedAt: string | null; readonly sourceStateSummary: GeneratedPortalAgentActivityReportSourceStateSummary; readonly parsedReport: GeneratedPortalAgentActivityReportDocument; readonly custodyLabel: GeneratedPortalAgentActivityReportCustodyLabel; readonly sourceLabel: GeneratedPortalAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type GeneratedPortalAgentActivityHistoricalReportList = { readonly schemaVersion: typeof GeneratedPortalAgentActivitySurfaceSchemaVersion; readonly request: GeneratedPortalAgentActivitySurfaceRequest; readonly state: GeneratedPortalAgentActivityReadModelState; readonly storageState: GeneratedPortalAgentActivitySavedReportState; readonly storageReason: string | null; readonly reports: readonly GeneratedPortalAgentActivityHistoricalReportListItem[] };
export type GeneratedPortalAgentActivityAppGameSourceStatusRow = { readonly sourceKind: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly rowCount: number; readonly lastObservedAt: string | null; readonly capabilityStatus: string; readonly evidence: readonly GeneratedPortalAgentActivityEvidenceRef[] };
export type GeneratedPortalAgentActivityTabReadModel<Row> = { readonly schemaVersion: typeof GeneratedPortalAgentActivitySurfaceSchemaVersion; readonly request: GeneratedPortalAgentActivitySurfaceRequest; readonly state: GeneratedPortalAgentActivityReadModelState; readonly generatedAt: string; readonly summary: string; readonly rows: readonly Row[] };
export type GeneratedPortalAgentActivityScreenReadModelRow = { readonly rowId: string; readonly label: string; readonly deviceId: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly totalMs: number; readonly foregroundMs: number; readonly backgroundMs: number; readonly captureReason: string; readonly captureScope: string; readonly capabilityStatus: string; readonly queueJobId: string; readonly modelRuntimeRef: string; readonly modelId: string; readonly providerKind: string; readonly promptOrTemplateVersion: string; readonly primaryCategory: string | null; readonly confidence: number; readonly imageDeletionState: string; readonly rawImageRetained: boolean; readonly policyEligible: boolean; readonly imageDigest: string; readonly custodyState: string; readonly evidence: readonly GeneratedPortalAgentActivityEvidenceRef[]; readonly policyDecisionRef: string | null; readonly policyAction: string | null; readonly policyReasonCodes: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly localModelRuntimeRefs: readonly string[]; readonly parentExplanationRefs: readonly string[]; readonly explanationReasons: readonly string[]; readonly deletionReasons: readonly string[]; readonly ocrTextSnippets: readonly string[]; readonly redactionNotes: readonly string[] };
export type GeneratedPortalAgentActivityAppUseReadModelRow = { readonly rowId: string; readonly appName: string; readonly deviceId: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly launchCount: number; readonly inventoryRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly GeneratedPortalAgentActivityAppGameSourceStatusRow[]; readonly evidence: readonly GeneratedPortalAgentActivityEvidenceRef[] };
export type GeneratedPortalAgentActivityBrowserReadModelRow = { readonly rowId: string; readonly domainLabel: string; readonly deviceId: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly visitCount: number; readonly totalMs: number; readonly evidenceDigest: string | null };
export type GeneratedPortalAgentActivityGamesReadModelRow = { readonly rowId: string; readonly displayName: string; readonly deviceId: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly sessionCount: number; readonly launcherRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly GeneratedPortalAgentActivityAppGameSourceStatusRow[]; readonly evidence: readonly GeneratedPortalAgentActivityEvidenceRef[] };
export type GeneratedPortalAgentActivityNetworkReadModelRow = { readonly rowId: string; readonly destinationLabel: string; readonly deviceId: string; readonly state: GeneratedPortalAgentActivityReadModelState; readonly connectionCount: number; readonly totalBytes: number; readonly evidenceDigest: string | null };
export type GeneratedPortalAgentActivityScreenReadModel = GeneratedPortalAgentActivityTabReadModel<GeneratedPortalAgentActivityScreenReadModelRow>;
export type GeneratedPortalAgentActivityAppUseReadModel = GeneratedPortalAgentActivityTabReadModel<GeneratedPortalAgentActivityAppUseReadModelRow>;
export type GeneratedPortalAgentActivityBrowserReadModel = GeneratedPortalAgentActivityTabReadModel<GeneratedPortalAgentActivityBrowserReadModelRow>;
export type GeneratedPortalAgentActivityGamesReadModel = GeneratedPortalAgentActivityTabReadModel<GeneratedPortalAgentActivityGamesReadModelRow>;
export type GeneratedPortalAgentActivityNetworkReadModel = GeneratedPortalAgentActivityTabReadModel<GeneratedPortalAgentActivityNetworkReadModelRow>;
export type GeneratedPortalAgentActivitySurfaceReadModel = GeneratedPortalAgentActivityScreenReadModel | GeneratedPortalAgentActivityAppUseReadModel | GeneratedPortalAgentActivityBrowserReadModel | GeneratedPortalAgentActivityGamesReadModel | GeneratedPortalAgentActivityNetworkReadModel;
function __GeneratedPortalAgentActivitySurfaceSchema<T>(decoder: (value: unknown) => T): GeneratedPortalAgentActivitySurfaceSchemaParser<T> {
  const parse = decoder;
  const safeParse = (
    value: unknown
  ): { readonly success: true; readonly data: T } | { readonly success: false } => {
    try {
      return { success: true, data: decoder(value) };
    } catch {
      return { success: false };
    }
  };
  return { parse, safeParse } as const;
}
function __GeneratedPortalAgentActivitySurfaceIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __GeneratedPortalAgentActivitySurfaceReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__GeneratedPortalAgentActivitySurfaceIsRecord(value)) { throw new TypeError(`${label} must be an activity surface object`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadOptionalNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === undefined || value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be an activity surface boolean`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadOptionalFalse(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (value === undefined) { return false; } if (value !== false) { throw new TypeError(`${field} must be false for activity surface redaction/custody boundary`); } return false; }
function __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative activity surface integer`); } return value; }
function __GeneratedPortalAgentActivitySurfaceReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be an activity surface confidence from 0 to 1`); } return value; }

function __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record: Readonly<Record<string, unknown>>): typeof GeneratedPortalAgentActivitySurfaceSchemaVersion { if (record['schemaVersion'] !== GeneratedPortalAgentActivitySurfaceSchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned activity surface schema version'); } return GeneratedPortalAgentActivitySurfaceSchemaVersion; }
function __GeneratedPortalAgentActivitySurfaceReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __GeneratedPortalAgentActivitySurfaceReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[], fallback: T): T { const value = record[field]; if (value === undefined) { return fallback; } if (typeof value !== 'string' || !allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __GeneratedPortalAgentActivitySurfaceReadArray<T>(record: Readonly<Record<string, unknown>>, field: string, decoder: (value: unknown) => T): readonly T[] { const value = record[field]; if (!Array.isArray(value)) { throw new TypeError(`${field} must be an activity surface array`); } return value.map(decoder); }
function __GeneratedPortalAgentActivitySurfaceReadStringArrayValue(value: unknown, field: string): readonly string[] { if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be an activity surface string array`); } return value as readonly string[]; }
function __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (value === undefined) { return []; } return __GeneratedPortalAgentActivitySurfaceReadStringArrayValue(value, field); }
function __GeneratedPortalAgentActivitySurfaceDecodeEvidenceRef(value: unknown): GeneratedPortalAgentActivityEvidenceRef { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity evidence ref'); return { evidenceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'evidenceId'), kind: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'kind', Object.values(GeneratedPortalAgentActivityEvidenceKind)), digest: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'digest'), uri: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'uri') }; }
function __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record: Readonly<Record<string, unknown>>, field: string): readonly GeneratedPortalAgentActivityEvidenceRef[] { return __GeneratedPortalAgentActivitySurfaceReadArray(record, field, __GeneratedPortalAgentActivitySurfaceDecodeEvidenceRef); }
function __GeneratedPortalAgentActivitySurfaceDecodeScope(value: unknown): GeneratedPortalAgentActivitySurfaceScope { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity surface scope'); const scope = { scopeKind: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'scopeKind', Object.values(GeneratedPortalAgentActivitySurfaceScopeKind)), familyId: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'familyId'), deviceId: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'deviceId') }; if (scope.scopeKind === GeneratedPortalAgentActivitySurfaceScopeKind.Family && (scope.familyId === null || scope.deviceId !== null)) { throw new TypeError('family activity scope must include familyId only'); } if (scope.scopeKind === GeneratedPortalAgentActivitySurfaceScopeKind.Device && (scope.familyId !== null || scope.deviceId === null)) { throw new TypeError('device activity scope must include deviceId only'); } return scope; }

function __GeneratedPortalAgentActivitySurfaceDecodeRequest(value: unknown): GeneratedPortalAgentActivitySurfaceRequest { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity surface request'); return { schemaVersion: __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record), scope: __GeneratedPortalAgentActivitySurfaceDecodeScope(record['scope']), requestedAt: __GeneratedPortalAgentActivitySurfaceReadString(record, 'requestedAt'), rangeStart: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeEnd') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeSourceState(value: unknown): GeneratedPortalAgentActivityReportSourceState { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity report source state'); return { deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), reachabilityState: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'reachabilityState', Object.values(GeneratedPortalAgentActivityReportSourceReachabilityState)), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), reason: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'reason'), lastUpdatedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'lastUpdatedAt'), custodyLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(GeneratedPortalAgentActivityReportCustodyLabel), GeneratedPortalAgentActivityReportCustodyLabel.ChildDeviceLocalSummary), sourceLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(GeneratedPortalAgentActivityReportSourceLabel), GeneratedPortalAgentActivityReportSourceLabel.ActivityQueryStoreSummary), rawChildEvidenceIncluded: __GeneratedPortalAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeSection(value: unknown): GeneratedPortalAgentActivityReportSection { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity report section'); return { sectionKind: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'sectionKind', Object.values(GeneratedPortalAgentActivityReportSectionKind)), title: __GeneratedPortalAgentActivitySurfaceReadString(record, 'title'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), summary: __GeneratedPortalAgentActivitySurfaceReadString(record, 'summary'), itemCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'itemCount'), evidence: __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeSavedMetadata(value: unknown): GeneratedPortalAgentActivitySavedReportMetadata { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity saved report metadata'); return { reportId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'reportId'), fileName: __GeneratedPortalAgentActivitySurfaceReadString(record, 'fileName'), savedState: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'savedState', Object.values(GeneratedPortalAgentActivitySavedReportState)), savedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'savedAt'), storageReason: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'storageReason'), custodyLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(GeneratedPortalAgentActivityReportCustodyLabel), GeneratedPortalAgentActivityReportCustodyLabel.ParentDeviceLocalReportJson), sourceLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(GeneratedPortalAgentActivityReportSourceLabel), GeneratedPortalAgentActivityReportSourceLabel.SavedReportJson), rawChildEvidenceIncluded: __GeneratedPortalAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeSourceStateSummary(value: unknown): GeneratedPortalAgentActivityReportSourceStateSummary { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity report source state summary'); return { totalSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalSources'), readySources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'readySources'), offlineSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'offlineSources'), staleSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'staleSources'), unavailableSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'unavailableSources'), unreachableSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'unreachableSources'), errorSources: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'errorSources') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeReportDocument(value: unknown): GeneratedPortalAgentActivityReportDocument { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity report document'); const savedMetadata = record['savedMetadata']; return { schemaVersion: __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record), reportId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'reportId'), frequency: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'frequency', Object.values(GeneratedPortalAgentActivityReportFrequency)), scope: __GeneratedPortalAgentActivitySurfaceDecodeScope(record['scope']), requestedAt: __GeneratedPortalAgentActivitySurfaceReadString(record, 'requestedAt'), rangeStart: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeEnd'), generatedAt: __GeneratedPortalAgentActivitySurfaceReadString(record, 'generatedAt'), savedMetadata: savedMetadata === null ? null : __GeneratedPortalAgentActivitySurfaceDecodeSavedMetadata(savedMetadata), sourceStates: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'sourceStates', __GeneratedPortalAgentActivitySurfaceDecodeSourceState), sections: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'sections', __GeneratedPortalAgentActivitySurfaceDecodeSection) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeHistoryItem(value: unknown): GeneratedPortalAgentActivityHistoricalReportListItem { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity historical report list item'); return { schemaVersion: __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record), reportId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'reportId'), fileName: __GeneratedPortalAgentActivitySurfaceReadString(record, 'fileName'), reportDate: __GeneratedPortalAgentActivitySurfaceReadString(record, 'reportDate'), rangeStart: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rangeEnd'), summary: __GeneratedPortalAgentActivitySurfaceReadString(record, 'summary'), savedState: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'savedState', Object.values(GeneratedPortalAgentActivitySavedReportState)), savedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'savedAt'), sourceStateSummary: __GeneratedPortalAgentActivitySurfaceDecodeSourceStateSummary(record['sourceStateSummary']), parsedReport: __GeneratedPortalAgentActivitySurfaceDecodeReportDocument(record['parsedReport']), custodyLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(GeneratedPortalAgentActivityReportCustodyLabel), GeneratedPortalAgentActivityReportCustodyLabel.ParentDeviceLocalHistory), sourceLabel: __GeneratedPortalAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(GeneratedPortalAgentActivityReportSourceLabel), GeneratedPortalAgentActivityReportSourceLabel.SavedReportHistory), rawChildEvidenceIncluded: __GeneratedPortalAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeHistoricalReportList(value: unknown): GeneratedPortalAgentActivityHistoricalReportList { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity historical report list'); return { schemaVersion: __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record), request: __GeneratedPortalAgentActivitySurfaceDecodeRequest(record['request']), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), storageState: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'storageState', Object.values(GeneratedPortalAgentActivitySavedReportState)), storageReason: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'storageReason'), reports: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'reports', __GeneratedPortalAgentActivitySurfaceDecodeHistoryItem) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeSourceStatusRow(value: unknown): GeneratedPortalAgentActivityAppGameSourceStatusRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity app/game source status row'); return { sourceKind: __GeneratedPortalAgentActivitySurfaceReadString(record, 'sourceKind'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), rowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'rowCount'), lastObservedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), capabilityStatus: __GeneratedPortalAgentActivitySurfaceReadString(record, 'capabilityStatus'), evidence: __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }

function __GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record: Readonly<Record<string, unknown>>) { return { schemaVersion: __GeneratedPortalAgentActivitySurfaceReadSchemaVersion(record), request: __GeneratedPortalAgentActivitySurfaceDecodeRequest(record['request']), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), generatedAt: __GeneratedPortalAgentActivitySurfaceReadString(record, 'generatedAt'), summary: __GeneratedPortalAgentActivitySurfaceReadString(record, 'summary') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeScreenRow(value: unknown): GeneratedPortalAgentActivityScreenReadModelRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity screen read-model row'); return { rowId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rowId'), label: __GeneratedPortalAgentActivitySurfaceReadString(record, 'label'), deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), totalMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), foregroundMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundMs'), backgroundMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'backgroundMs'), captureReason: __GeneratedPortalAgentActivitySurfaceReadString(record, 'captureReason'), captureScope: __GeneratedPortalAgentActivitySurfaceReadString(record, 'captureScope'), capabilityStatus: __GeneratedPortalAgentActivitySurfaceReadString(record, 'capabilityStatus'), queueJobId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'queueJobId'), modelRuntimeRef: __GeneratedPortalAgentActivitySurfaceReadString(record, 'modelRuntimeRef'), modelId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'modelId'), providerKind: __GeneratedPortalAgentActivitySurfaceReadString(record, 'providerKind'), promptOrTemplateVersion: __GeneratedPortalAgentActivitySurfaceReadString(record, 'promptOrTemplateVersion'), primaryCategory: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'primaryCategory'), confidence: __GeneratedPortalAgentActivitySurfaceReadConfidence(record, 'confidence'), imageDeletionState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'imageDeletionState'), rawImageRetained: __GeneratedPortalAgentActivitySurfaceReadBoolean(record, 'rawImageRetained'), policyEligible: __GeneratedPortalAgentActivitySurfaceReadBoolean(record, 'policyEligible'), imageDigest: __GeneratedPortalAgentActivitySurfaceReadString(record, 'imageDigest'), custodyState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'custodyState'), evidence: __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record, 'evidence'), policyDecisionRef: __GeneratedPortalAgentActivitySurfaceReadOptionalNullableString(record, 'policyDecisionRef'), policyAction: __GeneratedPortalAgentActivitySurfaceReadOptionalNullableString(record, 'policyAction'), policyReasonCodes: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'policyReasonCodes'), parentRuleRefs: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'parentRuleRefs'), localModelRuntimeRefs: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'localModelRuntimeRefs'), parentExplanationRefs: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'parentExplanationRefs'), explanationReasons: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'explanationReasons'), deletionReasons: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'deletionReasons'), ocrTextSnippets: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'ocrTextSnippets'), redactionNotes: __GeneratedPortalAgentActivitySurfaceReadOptionalStringArray(record, 'redactionNotes') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeAppUseRow(value: unknown): GeneratedPortalAgentActivityAppUseReadModelRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity app-use read-model row'); return { rowId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rowId'), appName: __GeneratedPortalAgentActivitySurfaceReadString(record, 'appName'), deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), productKind: __GeneratedPortalAgentActivitySurfaceReadString(record, 'productKind'), classificationState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'classificationState'), inventoryState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'inventoryState'), runtimeState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'runtimeState'), foregroundState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'foregroundState'), capabilityStatus: __GeneratedPortalAgentActivitySurfaceReadString(record, 'capabilityStatus'), lastObservedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), totalMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), launchCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'launchCount'), inventoryRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'inventoryRowCount'), runningRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'sourceStatusRows', __GeneratedPortalAgentActivitySurfaceDecodeSourceStatusRow), evidence: __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeBrowserRow(value: unknown): GeneratedPortalAgentActivityBrowserReadModelRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity browser read-model row'); return { rowId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rowId'), domainLabel: __GeneratedPortalAgentActivitySurfaceReadString(record, 'domainLabel'), deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), visitCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'visitCount'), totalMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), evidenceDigest: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'evidenceDigest') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeGamesRow(value: unknown): GeneratedPortalAgentActivityGamesReadModelRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity games read-model row'); return { rowId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rowId'), displayName: __GeneratedPortalAgentActivitySurfaceReadString(record, 'displayName'), deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), productKind: __GeneratedPortalAgentActivitySurfaceReadString(record, 'productKind'), classificationState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'classificationState'), inventoryState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'inventoryState'), runtimeState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'runtimeState'), foregroundState: __GeneratedPortalAgentActivitySurfaceReadString(record, 'foregroundState'), capabilityStatus: __GeneratedPortalAgentActivitySurfaceReadString(record, 'capabilityStatus'), lastObservedAt: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), totalMs: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), sessionCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'sessionCount'), launcherRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'launcherRowCount'), runningRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'sourceStatusRows', __GeneratedPortalAgentActivitySurfaceDecodeSourceStatusRow), evidence: __GeneratedPortalAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeNetworkRow(value: unknown): GeneratedPortalAgentActivityNetworkReadModelRow { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity network read-model row'); return { rowId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'rowId'), destinationLabel: __GeneratedPortalAgentActivitySurfaceReadString(record, 'destinationLabel'), deviceId: __GeneratedPortalAgentActivitySurfaceReadString(record, 'deviceId'), state: __GeneratedPortalAgentActivitySurfaceReadLiteral(record, 'state', Object.values(GeneratedPortalAgentActivityReadModelState)), connectionCount: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'connectionCount'), totalBytes: __GeneratedPortalAgentActivitySurfaceReadNonNegativeInteger(record, 'totalBytes'), evidenceDigest: __GeneratedPortalAgentActivitySurfaceReadNullableString(record, 'evidenceDigest') }; }
function __GeneratedPortalAgentActivitySurfaceDecodeScreenReadModel(value: unknown): GeneratedPortalAgentActivityScreenReadModel { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity screen read-model'); return { ...__GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record), rows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'rows', __GeneratedPortalAgentActivitySurfaceDecodeScreenRow) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeAppUseReadModel(value: unknown): GeneratedPortalAgentActivityAppUseReadModel { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity app-use read-model'); return { ...__GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record), rows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'rows', __GeneratedPortalAgentActivitySurfaceDecodeAppUseRow) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeBrowserReadModel(value: unknown): GeneratedPortalAgentActivityBrowserReadModel { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity browser read-model'); return { ...__GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record), rows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'rows', __GeneratedPortalAgentActivitySurfaceDecodeBrowserRow) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeGamesReadModel(value: unknown): GeneratedPortalAgentActivityGamesReadModel { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity games read-model'); return { ...__GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record), rows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'rows', __GeneratedPortalAgentActivitySurfaceDecodeGamesRow) }; }
function __GeneratedPortalAgentActivitySurfaceDecodeNetworkReadModel(value: unknown): GeneratedPortalAgentActivityNetworkReadModel { const record = __GeneratedPortalAgentActivitySurfaceReadRecord(value, 'activity network read-model'); return { ...__GeneratedPortalAgentActivitySurfaceDecodeReadModelBase(record), rows: __GeneratedPortalAgentActivitySurfaceReadArray(record, 'rows', __GeneratedPortalAgentActivitySurfaceDecodeNetworkRow) }; }
export const GeneratedPortalAgentActivityReadModelStateSchema = __GeneratedPortalAgentActivitySurfaceSchema((value: unknown): GeneratedPortalAgentActivityReadModelState => { if (typeof value !== 'string' || !(Object.values(GeneratedPortalAgentActivityReadModelState) as readonly string[]).includes(value)) { throw new TypeError('activity read-model state is not Rust-owned'); } return value as GeneratedPortalAgentActivityReadModelState; });
export const GeneratedPortalAgentActivitySurfaceRequestSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeRequest);
export const GeneratedPortalAgentActivityReportDocumentSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeReportDocument);
export const GeneratedPortalAgentActivityHistoricalReportListSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeHistoricalReportList);
export const GeneratedPortalAgentActivityScreenReadModelSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeScreenReadModel);
export const GeneratedPortalAgentActivityAppUseReadModelSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeAppUseReadModel);
export const GeneratedPortalAgentActivityBrowserReadModelSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeBrowserReadModel);
export const GeneratedPortalAgentActivityGamesReadModelSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeGamesReadModel);
export const GeneratedPortalAgentActivityNetworkReadModelSchema = __GeneratedPortalAgentActivitySurfaceSchema(__GeneratedPortalAgentActivitySurfaceDecodeNetworkReadModel);

export const GeneratedPortalAgentActivitySurfaceAdapterOperationId = { GetDailyReport: "getDailyReport", GetWeeklyReport: "getWeeklyReport", GetMonthlyReport: "getMonthlyReport", SaveActivityReport: "saveActivityReport", ListHistoricalReports: "listHistoricalReports", GetScreenActivity: "getScreenActivity", GetAppUseActivity: "getAppUseActivity", GetBrowserActivity: "getBrowserActivity", GetGamesActivity: "getGamesActivity", GetNetworkActivity: "getNetworkActivity" } as const;
export const GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder = { ReportGenerate: "createActivityReportGenerateCommand", ReportSave: "createActivityReportSaveCommand", ReportHistory: "createActivityReportHistoryCommand", ReadModel: "createActivityReadModelCommand" } as const;
export const GeneratedPortalAgentActivitySurfaceAdapterEventParser = { ReportDocument: "parseActivityReportDocumentEvent", ReportHistory: "parseActivityReportHistoryEvent", ReadModel: "parseActivityReadModelEvent" } as const;
export type GeneratedPortalAgentActivitySurfaceAdapterFailureReason = "wrong-event" | "missing-json-field" | "invalid-json" | "invalid-payload";
export type GeneratedPortalAgentActivitySurfaceAdapterResponseKind = "report-document" | "report-history" | "tab-read-model";
export type GeneratedPortalAgentActivitySurfaceAdapterOperation = { readonly operationId: (typeof GeneratedPortalAgentActivitySurfaceAdapterOperationId)[keyof typeof GeneratedPortalAgentActivitySurfaceAdapterOperationId]; readonly command: GeneratedPortalAgentCommandName; readonly successEvent: GeneratedPortalAgentEventName; readonly payloadField: GeneratedPortalAgentProtocolFieldName; readonly commandBuilder: (typeof GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder)[keyof typeof GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder]; readonly eventParser: (typeof GeneratedPortalAgentActivitySurfaceAdapterEventParser)[keyof typeof GeneratedPortalAgentActivitySurfaceAdapterEventParser]; readonly responseKind: GeneratedPortalAgentActivitySurfaceAdapterResponseKind; readonly readModelKind: GeneratedPortalAgentActivitySurfaceReadModelKind | null; readonly productDataOwner: "rust-service-read-model"; readonly uiConsumer: "c-owned-activity-ui"; readonly viteDataOwner: false; readonly supportsFamilyScope: boolean; readonly supportsDeviceScope: boolean; readonly failureState: "unavailable"; readonly failureReasons: readonly GeneratedPortalAgentActivitySurfaceAdapterFailureReason[]; readonly unavailableState: "unavailable" };
function GeneratedPortalAgentActivitySurfaceAdapterOperation(operationId: GeneratedPortalAgentActivitySurfaceAdapterOperation["operationId"], command: GeneratedPortalAgentCommandName, successEvent: GeneratedPortalAgentEventName, payloadField: GeneratedPortalAgentProtocolFieldName, responseKind: GeneratedPortalAgentActivitySurfaceAdapterResponseKind, readModelKind: GeneratedPortalAgentActivitySurfaceReadModelKind | null): GeneratedPortalAgentActivitySurfaceAdapterOperation { const commandBuilder = operationId === GeneratedPortalAgentActivitySurfaceAdapterOperationId.SaveActivityReport ? GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder.ReportSave : operationId === GeneratedPortalAgentActivitySurfaceAdapterOperationId.ListHistoricalReports ? GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder.ReportHistory : readModelKind === null ? GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder.ReportGenerate : GeneratedPortalAgentActivitySurfaceAdapterCommandBuilder.ReadModel; const eventParser = responseKind === "report-history" ? GeneratedPortalAgentActivitySurfaceAdapterEventParser.ReportHistory : responseKind === "tab-read-model" ? GeneratedPortalAgentActivitySurfaceAdapterEventParser.ReadModel : GeneratedPortalAgentActivitySurfaceAdapterEventParser.ReportDocument; return { operationId, command, successEvent, payloadField, commandBuilder, eventParser, responseKind, readModelKind, productDataOwner: "rust-service-read-model", uiConsumer: "c-owned-activity-ui", viteDataOwner: false, supportsFamilyScope: true, supportsDeviceScope: true, failureState: "unavailable", failureReasons: ["wrong-event", "missing-json-field", "invalid-json", "invalid-payload"], unavailableState: "unavailable" }; }
export const GeneratedPortalAgentActivitySurfaceAdapterOperationManifest = [GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetDailyReport, GeneratedPortalAgentCommand.ActivityReportDailyGenerate, GeneratedPortalAgentEvent.ActivityReportGenerated, GeneratedPortalAgentProtocolField.ActivityReportDocument, "report-document", null), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetWeeklyReport, GeneratedPortalAgentCommand.ActivityReportWeeklyGenerate, GeneratedPortalAgentEvent.ActivityReportGenerated, GeneratedPortalAgentProtocolField.ActivityReportDocument, "report-document", null), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetMonthlyReport, GeneratedPortalAgentCommand.ActivityReportMonthlyGenerate, GeneratedPortalAgentEvent.ActivityReportGenerated, GeneratedPortalAgentProtocolField.ActivityReportDocument, "report-document", null), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.SaveActivityReport, GeneratedPortalAgentCommand.ActivityReportSave, GeneratedPortalAgentEvent.ActivityReportSaved, GeneratedPortalAgentProtocolField.ActivityReportDocument, "report-document", null), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.ListHistoricalReports, GeneratedPortalAgentCommand.ActivityReportHistoryList, GeneratedPortalAgentEvent.ActivityReportHistoryReported, GeneratedPortalAgentProtocolField.ActivityReports, "report-history", null), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetScreenActivity, GeneratedPortalAgentCommand.ActivityScreenReadModelGet, GeneratedPortalAgentEvent.ActivityScreenReadModelReported, GeneratedPortalAgentProtocolField.ActivityReadModel, "tab-read-model", GeneratedPortalAgentActivitySurfaceReadModelKindName.Screen), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetAppUseActivity, GeneratedPortalAgentCommand.ActivityAppUseReadModelGet, GeneratedPortalAgentEvent.ActivityAppUseReadModelReported, GeneratedPortalAgentProtocolField.ActivityReadModel, "tab-read-model", GeneratedPortalAgentActivitySurfaceReadModelKindName.AppUse), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetBrowserActivity, GeneratedPortalAgentCommand.ActivityBrowserReadModelGet, GeneratedPortalAgentEvent.ActivityBrowserReadModelReported, GeneratedPortalAgentProtocolField.ActivityReadModel, "tab-read-model", GeneratedPortalAgentActivitySurfaceReadModelKindName.Browser), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetGamesActivity, GeneratedPortalAgentCommand.ActivityGamesReadModelGet, GeneratedPortalAgentEvent.ActivityGamesReadModelReported, GeneratedPortalAgentProtocolField.ActivityReadModel, "tab-read-model", GeneratedPortalAgentActivitySurfaceReadModelKindName.Games), GeneratedPortalAgentActivitySurfaceAdapterOperation(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetNetworkActivity, GeneratedPortalAgentCommand.ActivityNetworkReadModelGet, GeneratedPortalAgentEvent.ActivityNetworkReadModelReported, GeneratedPortalAgentProtocolField.ActivityReadModel, "tab-read-model", GeneratedPortalAgentActivitySurfaceReadModelKindName.Network)] as const satisfies readonly GeneratedPortalAgentActivitySurfaceAdapterOperation[];
 export const GeneratedPortalAgentLanHouseholdActionKind = { Assign: "assign", Rename: "rename", Ignore: "ignore", Restore: "restore", Trust: "trust" } as const; export type GeneratedPortalAgentLanHouseholdActionKind = (typeof GeneratedPortalAgentLanHouseholdActionKind)[keyof typeof GeneratedPortalAgentLanHouseholdActionKind]; export const GeneratedPortalAgentLanIntentKind = { ConfigurationUpdate: "configuration-update" } as const; export type GeneratedPortalAgentLanIntentKind = (typeof GeneratedPortalAgentLanIntentKind)[keyof typeof GeneratedPortalAgentLanIntentKind]; export const GeneratedPortalAgentLanParentAuthority = { ActiveController: "active-controller" } as const; export type GeneratedPortalAgentLanParentAuthority = (typeof GeneratedPortalAgentLanParentAuthority)[keyof typeof GeneratedPortalAgentLanParentAuthority]; export const GeneratedPortalAgentLanDiscoveryEventKind = { InterfaceChanged: "interface-changed", ScanStarted: "scan-started", ScanFinished: "scan-finished", EvidenceFound: "evidence-found", DeviceFound: "device-found", DeviceUpdated: "device-updated", DeviceOnline: "device-online", DeviceOffline: "device-offline", AgentDiscovered: "agent-discovered", AgentConfirmed: "agent-confirmed", UnknownDetected: "unknown-detected" } as const; export type GeneratedPortalAgentLanDiscoveryEventKind = (typeof GeneratedPortalAgentLanDiscoveryEventKind)[keyof typeof GeneratedPortalAgentLanDiscoveryEventKind]; export const GeneratedPortalAgentLanHouseholdDeviceKindValues = ["mobile","desktop","laptop","tablet","router","unknown"] as const; export type GeneratedPortalAgentLanHouseholdDeviceKind = (typeof GeneratedPortalAgentLanHouseholdDeviceKindValues)[number]; export const GeneratedPortalAgentLanHouseholdActionDeviceKindField = GeneratedPortalAgentProtocolField.LanHouseholdActionDeviceKind;

export type GeneratedPortalActivityMemoryGraphEntryStatus =
  | 'usable'
  | 'degraded'
  | 'stale'
  | 'rejected';

export const GeneratedPortalActivityMemoryGraphEntryStatus = {
  Usable: 'usable',
  Degraded: 'degraded',
  Stale: 'stale',
  Rejected: 'rejected',
} as const;

export type GeneratedPortalActivityMemoryGraphNodeKind =
  | 'child-profile'
  | 'device'
  | 'browser-url'
  | 'domain'
  | 'video'
  | 'app'
  | 'game'
  | 'activity-session';

export const GeneratedPortalActivityMemoryGraphNodeKind = {
  ChildProfile: 'child-profile',
  Device: 'device',
  BrowserUrl: 'browser-url',
  Domain: 'domain',
  Video: 'video',
  App: 'app',
  Game: 'game',
  ActivitySession: 'activity-session',
} as const;

export type GeneratedPortalActivityMemoryGraphEdgeKind =
  | 'visited'
  | 'watched'
  | 'played'
  | 'active-during'
  | 'performed-by-child'
  | 'derived-from-evidence';

export const GeneratedPortalActivityMemoryGraphEdgeKind = {
  Visited: 'visited',
  Watched: 'watched',
  Played: 'played',
  ActiveDuring: 'active-during',
  PerformedByChild: 'performed-by-child',
  DerivedFromEvidence: 'derived-from-evidence',
} as const;

export type GeneratedPortalActivityMemoryGraphQueryKind =
  | 'visited-urls'
  | 'played-games'
  | 'watched-videos'
  | 'activity-by-time-range'
  | 'explain-evidence';

export const GeneratedPortalActivityMemoryGraphQueryKind = {
  VisitedUrls: 'visited-urls',
  PlayedGames: 'played-games',
  WatchedVideos: 'watched-videos',
  ActivityByTimeRange: 'activity-by-time-range',
  ExplainEvidence: 'explain-evidence',
} as const;

export interface GeneratedPortalActivityMemoryGraphEvidenceReferenceSnapshot {
  readonly evidenceReferenceId: string;
  readonly kind: string;
  readonly observedAt: string;
}

export interface GeneratedPortalActivityMemoryGraphParentActionReferenceSnapshot {
  readonly actionReferenceId: string;
  readonly actor: {
    readonly actorId: string;
    readonly role: string;
  };
  readonly policyVersion: string;
  readonly createdAt: string;
}

export interface GeneratedPortalActivityMemoryGraphDeviceReferenceSnapshot {
  readonly deviceId: string;
  readonly childProfileId: string | null;
  readonly label: string;
  readonly platform: string;
}

export interface GeneratedPortalActivityMemoryGraphChildProfileReferenceSnapshot {
  readonly childProfileId: string;
  readonly displayName: string;
}

export interface GeneratedPortalActivityMemoryGraphTraceSnapshot {
  readonly entryStatus: GeneratedPortalActivityMemoryGraphEntryStatus;
  readonly sourceEvidenceReferences: readonly GeneratedPortalActivityMemoryGraphEvidenceReferenceSnapshot[];
  readonly sourcePolicyVersion: string | null;
  readonly sourceParentActionReferences: readonly GeneratedPortalActivityMemoryGraphParentActionReferenceSnapshot[];
  readonly generatedAt: string;
  readonly expiresAt: string | null;
  readonly confidence: number;
  readonly derivedIndexVersion: string;
  readonly degradedReasons: readonly string[];
}

export interface GeneratedPortalActivityMemoryGraphTimeRangeSnapshot {
  readonly observedFrom: string;
  readonly observedUntil: string;
}

export interface GeneratedPortalActivityMemoryGraphNodeSnapshot {
  readonly graphId: string;
  readonly nodeId: string;
  readonly nodeKind: GeneratedPortalActivityMemoryGraphNodeKind;
  readonly label: string;
  readonly childProfile: GeneratedPortalActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: GeneratedPortalActivityMemoryGraphDeviceReferenceSnapshot | null;
  readonly trace: GeneratedPortalActivityMemoryGraphTraceSnapshot;
}

export interface GeneratedPortalActivityMemoryGraphEdgeSnapshot {
  readonly graphId: string;
  readonly edgeId: string;
  readonly edgeKind: GeneratedPortalActivityMemoryGraphEdgeKind;
  readonly fromNodeId: string;
  readonly toNodeId: string;
  readonly observedFrom: string;
  readonly observedUntil: string | null;
  readonly durationMs: number | null;
  readonly trace: GeneratedPortalActivityMemoryGraphTraceSnapshot;
}

export interface GeneratedPortalActivityMemoryGraphQuerySnapshot {
  readonly queryId: string;
  readonly queryKind: GeneratedPortalActivityMemoryGraphQueryKind;
  readonly childProfile: GeneratedPortalActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: GeneratedPortalActivityMemoryGraphDeviceReferenceSnapshot;
  readonly timeRange: GeneratedPortalActivityMemoryGraphTimeRangeSnapshot;
  readonly asOf: string;
  readonly limit: number;
}

export interface GeneratedPortalActivityMemoryGraphReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custody: string;
  readonly capabilityStatus: string;
  readonly query: GeneratedPortalActivityMemoryGraphQuerySnapshot;
  readonly readAt: string;
  readonly nodes: readonly GeneratedPortalActivityMemoryGraphNodeSnapshot[];
  readonly edges: readonly GeneratedPortalActivityMemoryGraphEdgeSnapshot[];
  readonly returnedNodeCount: number;
  readonly returnedEdgeCount: number;
  readonly omittedEdgeCount: number;
  readonly degradedReasons: readonly string[];
}

export type GeneratedPortalActivityMemoryGraphNodeId =
  GeneratedPortalActivityMemoryGraphNodeSnapshot['nodeId'];

export function decodeGeneratedPortalActivityMemoryGraphDigest(
  digest: string
): GeneratedPortalActivityMemoryGraphReadModelSnapshot | null {
  try {
    return decodeGeneratedPortalActivityMemoryGraphReadModelSnapshot(JSON.parse(digest) as unknown);
  } catch {
    return null;
  }
}

export function decodeGeneratedPortalActivityMemoryGraphReadModelSnapshot(
  value: unknown
): GeneratedPortalActivityMemoryGraphReadModelSnapshot | null {
  return isGeneratedPortalActivityMemoryGraphReadModelSnapshot(value) ? value : null;
}

function isGeneratedPortalActivityMemoryGraphReadModelSnapshot(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphReadModelSnapshot {
  if (!isGeneratedPortalActivityMemoryGraphRecord(value)) {
    return false;
  }
  const nodes = value['nodes'];
  const edges = value['edges'];
  return (
    isGeneratedPortalActivityMemoryGraphReadModelSnapshotMetadata(value) &&
    isGeneratedPortalActivityMemoryGraphReadModelSnapshotCollections(
      value,
      nodes,
      edges
    )
  );
}

function isGeneratedPortalActivityMemoryGraphReadModelSnapshotMetadata(
  value: Record<string, unknown>
): boolean {
  return (
    isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value['schemaVersion']) &&
    isGeneratedPortalActivityMemoryGraphString(value['generatedAt']) &&
    isGeneratedPortalActivityMemoryGraphString(value['custody']) &&
    isGeneratedPortalActivityMemoryGraphString(value['capabilityStatus']) &&
    isGeneratedPortalActivityMemoryGraphQuerySnapshot(value['query']) &&
    isGeneratedPortalActivityMemoryGraphString(value['readAt']) &&
    isGeneratedPortalActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function isGeneratedPortalActivityMemoryGraphReadModelSnapshotCollections(
  value: Record<string, unknown>,
  nodes: unknown,
  edges: unknown
): boolean {
  return (
    Array.isArray(nodes) &&
    nodes.every(isGeneratedPortalActivityMemoryGraphNodeSnapshot) &&
    Array.isArray(edges) &&
    edges.every(isGeneratedPortalActivityMemoryGraphEdgeSnapshot) &&
    isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value['returnedNodeCount']) &&
    value['returnedNodeCount'] === nodes.length &&
    isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value['returnedEdgeCount']) &&
    value['returnedEdgeCount'] === edges.length &&
    isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value['omittedEdgeCount'])
  );
}

function isGeneratedPortalActivityMemoryGraphQuerySnapshot(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphQuerySnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['queryId']) &&
    isGeneratedPortalActivityMemoryGraphQueryKind(value['queryKind']) &&
    isGeneratedPortalActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    isGeneratedPortalActivityMemoryGraphDevice(value['device']) &&
    isGeneratedPortalActivityMemoryGraphTimeRange(value['timeRange']) &&
    isGeneratedPortalActivityMemoryGraphString(value['asOf']) &&
    isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value['limit'])
  );
}

function isGeneratedPortalActivityMemoryGraphNodeSnapshot(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphNodeSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['graphId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['nodeId']) &&
    isGeneratedPortalActivityMemoryGraphNodeKind(value['nodeKind']) &&
    isGeneratedPortalActivityMemoryGraphString(value['label']) &&
    isGeneratedPortalActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    isGeneratedPortalActivityMemoryGraphNullableDevice(value['device']) &&
    isGeneratedPortalActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function isGeneratedPortalActivityMemoryGraphEdgeSnapshot(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphEdgeSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['graphId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['edgeId']) &&
    isGeneratedPortalActivityMemoryGraphEdgeKind(value['edgeKind']) &&
    isGeneratedPortalActivityMemoryGraphString(value['fromNodeId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['toNodeId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['observedFrom']) &&
    isGeneratedPortalActivityMemoryGraphNullableString(value['observedUntil']) &&
    isGeneratedPortalActivityMemoryGraphNullableCount(value['durationMs']) &&
    isGeneratedPortalActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function isGeneratedPortalActivityMemoryGraphTraceSnapshot(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphTraceSnapshot {
  if (!isGeneratedPortalActivityMemoryGraphRecord(value)) {
    return false;
  }
  const evidenceRefs = value['sourceEvidenceReferences'];
  const parentActionRefs = value['sourceParentActionReferences'];
  return (
    isGeneratedPortalActivityMemoryGraphTraceSnapshotMetadata(
      value,
      evidenceRefs,
      parentActionRefs
    )
  );
}

function isGeneratedPortalActivityMemoryGraphTraceSnapshotMetadata(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    isGeneratedPortalActivityMemoryGraphTraceSnapshotReferencesValid(
      value,
      evidenceRefs,
      parentActionRefs
    ) &&
    isGeneratedPortalActivityMemoryGraphTraceSnapshotFieldsValid(value)
  );
}

function isGeneratedPortalActivityMemoryGraphTraceSnapshotReferencesValid(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    isGeneratedPortalActivityMemoryGraphEntryStatus(value['entryStatus']) &&
    Array.isArray(evidenceRefs) &&
    evidenceRefs.every(isGeneratedPortalActivityMemoryGraphEvidenceReference) &&
    isGeneratedPortalActivityMemoryGraphNullableString(value['sourcePolicyVersion']) &&
    Array.isArray(parentActionRefs) &&
    parentActionRefs.every(isGeneratedPortalActivityMemoryGraphParentActionReference) &&
    (evidenceRefs.length > 0 || value['sourcePolicyVersion'] !== null || parentActionRefs.length > 0)
  );
}

function isGeneratedPortalActivityMemoryGraphTraceSnapshotFieldsValid(
  value: Record<string, unknown>
): boolean {
  return (
    isGeneratedPortalActivityMemoryGraphString(value['generatedAt']) &&
    isGeneratedPortalActivityMemoryGraphNullableString(value['expiresAt']) &&
    typeof value['confidence'] === 'number' &&
    value['confidence'] >= 0 &&
    isGeneratedPortalActivityMemoryGraphString(value['derivedIndexVersion']) &&
    isGeneratedPortalActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function isGeneratedPortalActivityMemoryGraphEvidenceReference(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphEvidenceReferenceSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['evidenceReferenceId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['kind']) &&
    isGeneratedPortalActivityMemoryGraphString(value['observedAt'])
  );
}

function isGeneratedPortalActivityMemoryGraphParentActionReference(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphParentActionReferenceSnapshot {
  if (!isGeneratedPortalActivityMemoryGraphRecord(value)) {
    return false;
  }
  const actor = value['actor'];
  return (
    isGeneratedPortalActivityMemoryGraphString(value['actionReferenceId']) &&
    isGeneratedPortalActivityMemoryGraphRecord(actor) &&
    isGeneratedPortalActivityMemoryGraphString(actor['actorId']) &&
    isGeneratedPortalActivityMemoryGraphString(actor['role']) &&
    isGeneratedPortalActivityMemoryGraphString(value['policyVersion']) &&
    isGeneratedPortalActivityMemoryGraphString(value['createdAt'])
  );
}

function isGeneratedPortalActivityMemoryGraphDevice(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphDeviceReferenceSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['deviceId']) &&
    isGeneratedPortalActivityMemoryGraphNullableString(value['childProfileId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['label']) &&
    isGeneratedPortalActivityMemoryGraphString(value['platform'])
  );
}

function isGeneratedPortalActivityMemoryGraphChildProfile(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphChildProfileReferenceSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['childProfileId']) &&
    isGeneratedPortalActivityMemoryGraphString(value['displayName'])
  );
}

function isGeneratedPortalActivityMemoryGraphTimeRange(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphTimeRangeSnapshot {
  return (
    isGeneratedPortalActivityMemoryGraphRecord(value) &&
    isGeneratedPortalActivityMemoryGraphString(value['observedFrom']) &&
    isGeneratedPortalActivityMemoryGraphString(value['observedUntil'])
  );
}

function isGeneratedPortalActivityMemoryGraphNullableChildProfile(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphChildProfileReferenceSnapshot | null {
  return value === null || isGeneratedPortalActivityMemoryGraphChildProfile(value);
}

function isGeneratedPortalActivityMemoryGraphNullableDevice(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphDeviceReferenceSnapshot | null {
  return value === null || isGeneratedPortalActivityMemoryGraphDevice(value);
}

function isGeneratedPortalActivityMemoryGraphNullableString(value: unknown): value is string | null {
  return value === null || isGeneratedPortalActivityMemoryGraphString(value);
}

function isGeneratedPortalActivityMemoryGraphNullableCount(value: unknown): value is number | null {
  return value === null || isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value);
}

function isGeneratedPortalActivityMemoryGraphNodeKind(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphNodeKind {
  return (
    typeof value === 'string' &&
    Object.values(GeneratedPortalActivityMemoryGraphNodeKind).includes(
      value as GeneratedPortalActivityMemoryGraphNodeKind
    )
  );
}

function isGeneratedPortalActivityMemoryGraphEdgeKind(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphEdgeKind {
  return (
    typeof value === 'string' &&
    Object.values(GeneratedPortalActivityMemoryGraphEdgeKind).includes(
      value as GeneratedPortalActivityMemoryGraphEdgeKind
    )
  );
}

function isGeneratedPortalActivityMemoryGraphEntryStatus(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphEntryStatus {
  return (
    typeof value === 'string' &&
    Object.values(GeneratedPortalActivityMemoryGraphEntryStatus).includes(
      value as GeneratedPortalActivityMemoryGraphEntryStatus
    )
  );
}

function isGeneratedPortalActivityMemoryGraphQueryKind(
  value: unknown
): value is GeneratedPortalActivityMemoryGraphQueryKind {
  return (
    typeof value === 'string' &&
    Object.values(GeneratedPortalActivityMemoryGraphQueryKind).includes(
      value as GeneratedPortalActivityMemoryGraphQueryKind
    )
  );
}

function isGeneratedPortalActivityMemoryGraphNonNegativeInteger(value: unknown): value is number {
  return typeof value === 'number' && Number.isInteger(value) && value >= 0;
}

function isGeneratedPortalActivityMemoryGraphStringArray(value: unknown): value is readonly string[] {
  return Array.isArray(value) && value.every(isGeneratedPortalActivityMemoryGraphString);
}

function isGeneratedPortalActivityMemoryGraphString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function isGeneratedPortalActivityMemoryGraphRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

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
  {"AcceptedAt":"2026-06-06T19:50:00Z","CommandId":"tracking-retention-settings-write-command","DurableSettingsStoreRef":"agent-service-local-retention-settings-durable-json","LocalServiceStateSnapshotRef":"agent-service-local-retention-settings-state","MutationProofRef":"output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json","ReadModelProofRefs":["output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json","output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json"],"SettingsKindRetentionWindow":"retention-window-setting","WriteStateAccepted":"service-write-command-accepted","WriteStateRejected":"service-write-command-rejected","WriterIntentRef":"tracking-retention-settings-write-retention-window"} as const;

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
  {"adapterDispatchClaimed":false,"authorityProofClaimed":false,"childDeviceDeliveryClaimed":false,"cloudRoutingClaimed":false,"family":{"familyId":"family-tracking-notification-history"},"generatedAt":"2026-06-06T16:16:00.000Z","historyIntentReadyCount":1,"manualActionRequiredCount":1,"mobilePhysicalDeviceProofClaimed":false,"parentFrequencyControlUiClaimed":false,"parentPreferenceMutationRuntimeClaimed":false,"productionDurableHistoryStorageClaimed":false,"productionDurableOutboxStorageClaimed":false,"proofId":"tracking-notification-parent-surface-history-proof","proofNonClaims":["no-rendered-parent-notification-ui","no-parent-preference-mutation-runtime","no-parent-frequency-control-ui","no-quiet-hours-timer-runtime","no-provider-delivery-execution","no-provider-receipt-ingestion-runtime","no-provider-credentials","no-cloud-routing","no-child-device-delivery","no-mobile-physical-device-proof","no-authority-proof","no-retry-worker-runtime","no-production-durable-history-storage","no-production-durable-outbox-storage","no-adapter-dispatch"],"providerCredentialsClaimed":false,"providerDeliveryRuntimeClaimed":false,"providerReceiptIngestionRuntimeClaimed":false,"providerUnavailableCount":1,"quietHoursTimerRuntimeClaimed":false,"renderedParentNotificationUiClaimed":false,"retryExecutionRuntimeClaimed":false,"rows":[{"auditRefs":["tracking-provider-notification-audit-tracking-alert-home-arrival"],"authorityProofClaimed":false,"childDeviceDeliveryClaimed":false,"drillInRefs":["tracking-notification-history-drill-in-tracking-alert-home-arrival"],"evidenceRefs":["location-evidence-geofence-entry"],"historyRowId":"tracking-notification-history-tracking-alert-home-arrival","manualProofRequirements":["provider-delivery-runtime-required","receipt-webhook-runtime-required"],"mobilePhysicalDeviceProofClaimed":false,"notificationStatusRefs":["tracking-notification-intent-home-arrival"],"parentPreferenceMutationRuntimeClaimed":false,"parentPreferenceRequirementRefs":["parent-notification-preference-required-home-arrival"],"providerAttemptRef":"tracking-provider-attempt-home-arrival","providerDeliveryClaimed":false,"providerPreferenceRefs":["tracking-parent-provider-preference-home-arrival"],"providerStatusEntryRef":"tracking-provider-status-entry-home-arrival","quietHoursRequirementRefs":["tracking-quiet-hours-policy-required-tracking-alert-home-arrival"],"reasonCodeRefs":["home-arrival-notification"],"receiptIngestionRuntimeClaimed":false,"receiptRequirementRefs":["receipt-ingestion-required-home-arrival"],"redactedParentSummaryRef":"tracking-notification-redacted-summary-tracking-alert-home-arrival","renderedParentNotificationUiClaimed":false,"sourceAlertId":"tracking-alert-home-arrival","sourcePolicyDecisionId":"tracking-decision-home-arrival","sourcePreferencePreflightRowId":"tracking-notification-preference-preflight-tracking-alert-home-arrival","sourceProviderNotificationRowId":"tracking-provider-notification-tracking-alert-home-arrival","sourceReceiptBoundaryRowId":"tracking-notification-receipt-tracking-alert-home-arrival","status":"history-intent-ready"},{"auditRefs":["tracking-provider-notification-audit-tracking-alert-left-expected-place"],"authorityProofClaimed":false,"childDeviceDeliveryClaimed":false,"drillInRefs":["tracking-notification-history-drill-in-tracking-alert-left-expected-place"],"evidenceRefs":["location-evidence-geofence-entry"],"historyRowId":"tracking-notification-history-tracking-alert-left-expected-place","manualProofRequirements":["manual-provider-review-required","quiet-hours-runtime-required"],"mobilePhysicalDeviceProofClaimed":false,"notificationStatusRefs":["tracking-notification-intent-left-school"],"parentPreferenceMutationRuntimeClaimed":false,"parentPreferenceRequirementRefs":["tracking-parent-notification-preference-required-tracking-alert-left-school"],"providerAttemptRef":"tracking-provider-attempt-left-school","providerDeliveryClaimed":false,"providerPreferenceRefs":["tracking-parent-provider-preference-left-school"],"providerStatusEntryRef":"tracking-provider-status-entry-left-school","quietHoursRequirementRefs":["quiet-hours-requirement-left-school"],"reasonCodeRefs":["left-expected-place"],"receiptIngestionRuntimeClaimed":false,"receiptRequirementRefs":["manual-receipt-required-left-school"],"redactedParentSummaryRef":"tracking-notification-redacted-summary-tracking-alert-left-expected-place","renderedParentNotificationUiClaimed":false,"sourceAlertId":"tracking-alert-left-expected-place","sourcePolicyDecisionId":"tracking-decision-left-expected-place","sourcePreferencePreflightRowId":"tracking-notification-preference-preflight-tracking-alert-left-expected-place","sourceProviderNotificationRowId":"tracking-provider-notification-tracking-alert-left-expected-place","sourceReceiptBoundaryRowId":"tracking-notification-receipt-tracking-alert-left-expected-place","status":"manual-action-required"},{"auditRefs":["tracking-provider-notification-audit-tracking-alert-provider-unavailable"],"authorityProofClaimed":false,"childDeviceDeliveryClaimed":false,"drillInRefs":["tracking-notification-history-drill-in-tracking-alert-provider-unavailable"],"evidenceRefs":["location-evidence-geofence-entry"],"historyRowId":"tracking-notification-history-tracking-alert-provider-unavailable","manualProofRequirements":["provider-adapter-unavailable","manual-parent-history-review-required"],"mobilePhysicalDeviceProofClaimed":false,"notificationStatusRefs":["tracking-notification-intent-provider-unavailable"],"parentPreferenceMutationRuntimeClaimed":false,"parentPreferenceRequirementRefs":["source-unavailable-preference-required"],"providerAttemptRef":"tracking-provider-attempt-unavailable","providerDeliveryClaimed":false,"providerPreferenceRefs":["tracking-parent-provider-preference-provider-unavailable"],"providerStatusEntryRef":"tracking-provider-status-entry-provider-unavailable","quietHoursRequirementRefs":[],"reasonCodeRefs":["provider-unavailable"],"receiptIngestionRuntimeClaimed":false,"receiptRequirementRefs":["provider-receipt-unavailable"],"redactedParentSummaryRef":"tracking-notification-redacted-summary-tracking-alert-provider-unavailable","renderedParentNotificationUiClaimed":false,"sourceAlertId":"tracking-alert-provider-unavailable","sourcePolicyDecisionId":"tracking-decision-provider-unavailable","sourcePreferencePreflightRowId":"tracking-notification-preference-preflight-tracking-alert-provider-unavailable","sourceProviderNotificationRowId":"tracking-provider-notification-tracking-alert-provider-unavailable","sourceReceiptBoundaryRowId":"tracking-notification-receipt-tracking-alert-provider-unavailable","status":"provider-unavailable"}],"schemaVersion":"v0.6","sourceContractRefs":["tracking-provider-notification-proof","tracking-notification-receipt-boundary-proof","tracking-notification-preference-preflight-proof","notifications-expectations","location-geofence-device-status"],"sourcePreferencePreflightProofRef":"tracking-notification-preference-preflight-proof-for-parent-surface-history","sourceProviderNotificationProofRef":"tracking-provider-notification-proof-for-parent-surface-history","sourceReceiptBoundaryProofRef":"tracking-notification-receipt-boundary-proof-for-parent-surface-history"} as const satisfies GeneratedTrackingNotificationParentSurfaceHistoryReadModel;

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