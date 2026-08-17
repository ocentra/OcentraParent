/* generated from crates/schema/src/parent_ui_bridge.rs */

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
] as const;export function isParentAiRuntimeRoute(route: ParentRouteId): boolean {
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
);export type ParentBridgeConnectionState = 'disconnected' | 'connecting' | 'connected' | 'error';

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

export const ParentAgentProtocolRuntime = { SchemaVersion: 1, MessageIdPrefix: "cmd-" } as const; export type ParentAgentProtocolPayloadValue = string | number | boolean | null; export type ParentAgentProtocolPayload = Readonly<Record<string, ParentAgentProtocolPayloadValue>>; export const ParentAgentPeerRole = { Portal: "portal", AgentService: "agent-service", CloudRelay: "cloud-relay" } as const; export type ParentAgentPeerRole = (typeof ParentAgentPeerRole)[keyof typeof ParentAgentPeerRole]; export const ParentAgentRoute = { Localhost: "localhost", LocalNetwork: "local-network", CloudRelay: "cloud-relay" } as const; export type ParentAgentRoute = (typeof ParentAgentRoute)[keyof typeof ParentAgentRoute]; export interface ParentAgentPeer { readonly peerId: string; readonly role: ParentAgentPeerRole; } export interface ParentAgentMessageTarget { readonly deviceId: string; readonly platform: string; readonly route: ParentAgentRoute; } export const ParentAgentPeerDefaults = { PortalDev: {"peerId":"portal-dev","role":"portal"} } as const; export const ParentAgentTargetDefaults = { LocalhostWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"localhost"}, LocalNetworkWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"local-network"} } as const; export interface ParentAgentCommandEnvelope { readonly schemaVersion: number; readonly messageId: string; readonly sentAt: string; readonly source: ParentAgentPeer; readonly target: ParentAgentMessageTarget; readonly command: ParentAgentCommandName; readonly payload: ParentAgentProtocolPayload; } export function decodeParentAgentCommandEnvelope(value: unknown): ParentAgentCommandEnvelope { const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty string`); } return fieldValue; }; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => { const fieldValue = record[field]; if (typeof fieldValue !== 'number') { throw new TypeError(`${field} must be a number`); } return fieldValue; }; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => { const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== ParentAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return schemaVersion; }; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => { const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned protocol literal`); } return fieldValue as T; }; const readPeer = (candidate: unknown): ParentAgentPeer => { if (!isRecord(candidate)) { throw new TypeError('peer must be an object'); } return { peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values(ParentAgentPeerRole)) }; }; const readTarget = (candidate: unknown): ParentAgentMessageTarget => { if (!isRecord(candidate)) { throw new TypeError('target must be an object'); } return { deviceId: readString(candidate, 'deviceId'), platform: readString(candidate, 'platform'), route: readLiteral(candidate, 'route', Object.values(ParentAgentRoute)) }; }; const readPayload = (candidate: unknown): ParentAgentProtocolPayload => { if (!isRecord(candidate)) { throw new TypeError('payload must be an object'); } for (const payloadValue of Object.values(candidate)) { if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') { throw new TypeError('payload values must be primitive protocol values'); } } return candidate as ParentAgentProtocolPayload; }; if (!isRecord(value)) { throw new TypeError('command envelope must be an object'); } return { schemaVersion: readSchemaVersion(value), messageId: readString(value, 'messageId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readTarget(value['target']), command: readLiteral(value, 'command', Object.values(ParentAgentCommand)), payload: readPayload(value['payload']) }; } export const ParentAgentProtocolLogLevel = { Trace: "trace", Debug: "debug", Info: "info", Warn: "warn", Error: "error" } as const; export type ParentAgentProtocolLogLevel = (typeof ParentAgentProtocolLogLevel)[keyof typeof ParentAgentProtocolLogLevel]; export interface ParentAgentEventEnvelope { readonly schemaVersion: number; readonly eventId: string; readonly correlationId: string; readonly sentAt: string; readonly source: ParentAgentPeer; readonly target: ParentAgentPeer; readonly event: ParentAgentEventName; readonly severity: ParentAgentProtocolLogLevel; readonly payload: ParentAgentProtocolPayload; readonly snapshot: unknown | null; } export function decodeParentAgentEventEnvelope(value: unknown): ParentAgentEventEnvelope { const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty string`); } return fieldValue; }; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => { const fieldValue = record[field]; if (typeof fieldValue !== 'number') { throw new TypeError(`${field} must be a number`); } return fieldValue; }; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => { const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== ParentAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return schemaVersion; }; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => { const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned protocol literal`); } return fieldValue as T; }; const readPeer = (candidate: unknown): ParentAgentPeer => { if (!isRecord(candidate)) { throw new TypeError('peer must be an object'); } return { peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values(ParentAgentPeerRole)) }; }; const readPayload = (candidate: unknown): ParentAgentProtocolPayload => { if (!isRecord(candidate)) { throw new TypeError('payload must be an object'); } for (const payloadValue of Object.values(candidate)) { if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') { throw new TypeError('payload values must be primitive protocol values'); } } return candidate as ParentAgentProtocolPayload; }; if (!isRecord(value)) { throw new TypeError('event envelope must be an object'); } return { schemaVersion: readSchemaVersion(value), eventId: readString(value, 'eventId'), correlationId: readString(value, 'correlationId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readPeer(value['target']), event: readLiteral(value, 'event', Object.values(ParentAgentEvent)), severity: readLiteral(value, 'severity', Object.values(ParentAgentProtocolLogLevel)), payload: readPayload(value['payload']), snapshot: value['snapshot'] ?? null }; } function decodeNonEmptyProtocolString(value: unknown, label: string): string { if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${label} must be a non-empty Rust-owned protocol string`); } return value; } export function decodeParentAgentMessageId(value: unknown): string { return decodeNonEmptyProtocolString(value, 'messageId'); } export function decodeParentAgentTimestamp(value: unknown): string { return decodeNonEmptyProtocolString(value, 'timestamp'); } export function decodeParentSerializedAgentMessage(value: unknown): string { return decodeNonEmptyProtocolString(value, 'serializedMessage'); } export function isParentAgentProtocolLogText(value: unknown): value is string { return typeof value === 'string'; } export const ParentAgentProtocolField = { ActivityDigest: "activityDigest", ActivityFamilySources: "activityFamilySources", ActivityReadModel: "activityReadModel", ActivityReadModelKind: "activityReadModelKind", ActivityReportDocument: "activityReportDocument", ActivityReportFrequency: "activityReportFrequency", ActivityReportId: "activityReportId", ActivityReports: "activityReports", ActivitySurfaceState: "activitySurfaceState", ActivityTrackingRetentionSettingsWriteResult: "trackingRetentionSettingsWriteResult", ClaimBoundary: "claimBoundary", DeviceId: "deviceId", EventRef: "eventRef", EventType: "eventType", FamilyId: "familyId", Origin: "origin", Payload: "payload", StartedAt: "startedAt", StaleAt: "staleAt", BrowserSocialAlertReportReadModel: "browserSocialAlertReportReadModel", BrowserSocialAlertReportParentSurfaceReadModel: "browserSocialAlertReportParentSurfaceReadModel", BrowserSocialDashboardReadModel: "browserSocialDashboardReadModel", BrowserSocialParentNotificationDeliveryReadModel: "browserSocialParentNotificationDeliveryReadModel", BrowserRuntimeActionIntentAdapterExecutions: "browserRuntimeActionIntentAdapterExecutions", BrowserRuntimeActionIntentCandidates: "browserRuntimeActionIntentCandidates", BrowserRuntimeActionIntentChildAcceptedEventRefs: "browserRuntimeActionIntentChildAcceptedEventRefs", BrowserRuntimeActionIntentChildAcceptedRows: "browserRuntimeActionIntentChildAcceptedRows", BrowserRuntimeActionIntentChildCommandRefs: "browserRuntimeActionIntentChildCommandRefs", BrowserRuntimeActionIntentChildInterventionExecutions: "browserRuntimeActionIntentChildInterventionExecutions", BrowserRuntimeActionIntentDispatchAttempts: "browserRuntimeActionIntentDispatchAttempts", BrowserRuntimeActionIntentEnforcementExecutions: "browserRuntimeActionIntentEnforcementExecutions", BrowserRuntimeActionIntentHandoffCandidates: "browserRuntimeActionIntentHandoffCandidates", BrowserRuntimeActionIntentHandoffOutboxRefs: "browserRuntimeActionIntentHandoffOutboxRefs", BrowserRuntimeActionIntentHandoffRefs: "browserRuntimeActionIntentHandoffRefs", BrowserRuntimeActionIntentParentReadModelRefs: "browserRuntimeActionIntentParentReadModelRefs", BrowserRuntimeEventChainStream: "browserRuntimeEventChainStream", BrowserRuntimeExactUrlRows: "browserRuntimeExactUrlRows", BrowserRuntimeFailedRows: "browserRuntimeFailedRows", BrowserRuntimeInterventionCommandEvents: "browserRuntimeInterventionCommandEvents", BrowserRuntimeManualRequiredRows: "browserRuntimeManualRequiredRows", BrowserRuntimeObservedRows: "browserRuntimeObservedRows", BrowserRuntimeReadModelProjectionEvents: "browserRuntimeReadModelProjectionEvents", BrowserRuntimeSocialProviderAttemptRefs: "browserRuntimeSocialProviderAttemptRefs", BrowserRuntimeSocialProviderDispatchRequiredRows: "browserRuntimeSocialProviderDispatchRequiredRows", BrowserRuntimeSocialProviderDurableResultRefs: "browserRuntimeSocialProviderDurableResultRefs", BrowserRuntimeSocialProviderDurableRows: "browserRuntimeSocialProviderDurableRows", BrowserRuntimeSocialProviderDurableStoreRefs: "browserRuntimeSocialProviderDurableStoreRefs", BrowserRuntimeSocialProviderManualReceiptRequiredRows: "browserRuntimeSocialProviderManualReceiptRequiredRows", BrowserRuntimeSocialProviderReadModelRefs: "browserRuntimeSocialProviderReadModelRefs", BrowserRuntimeSocialProviderReceiptBoundaryRows: "browserRuntimeSocialProviderReceiptBoundaryRows", BrowserRuntimeSocialProviderReceiptProofRefs: "browserRuntimeSocialProviderReceiptProofRefs", BrowserRuntimeSocialProviderSupportStatusRefs: "browserRuntimeSocialProviderSupportStatusRefs", BrowserRuntimeStreamedEvents: "browserRuntimeStreamedEvents", LanAiJobId: "lanAiJobId", LanAiJobState: "lanAiJobState", LanAiJobStatus: "lanAiJobStatus", LanAiProviderCustodyLabel: "lanAiProviderCustodyLabel", LanAiProviderRoutingState: "lanAiProviderRoutingState", LanControllerLeaseExpiresAt: "controllerLeaseExpiresAt", LanControllerLeaseId: "controllerLeaseId", LanControllerLeaseIssuedAt: "controllerLeaseIssuedAt", LanCanonicalDeviceId: "canonicalDeviceId", LanChildDeviceId: "childDeviceId", LanControllerDeviceId: "controllerDeviceId", LanHouseholdActionId: "householdActionId", LanHouseholdActionKind: "householdActionKind", LanHouseholdActionChildProfileId: "childProfileId", LanHouseholdActionDisplayName: "displayName", LanHouseholdActionDeviceKind: "deviceKind", LanHouseholdActionRevokedAt: "revokedAt", LanIntentId: "intentId", LanIntentKind: "intentKind", LanPairingId: "pairingId", LanParentAuthority: "parentAuthority", LanParentActorId: "parentActorId", LanParentDeviceId: "parentDeviceId", LanProofDigest: "proofDigest", LanRouteId: "routeId", LoadState: "loadState", LocalAiAdapterReadinessState: "readinessState", LocalAiCapabilityFlags: "capabilityFlags", LocalAiDegradedState: "degradedState", LocalAiExecutionState: "executionState", LocalAiModelId: "modelId", LocalAiPrivacyMode: "privacyMode", LocalAiProviderId: "providerId", LocalAiProviderSource: "providerSource", LocalAiResourceClass: "resourceClass", LocalAiRuntimeReferenceId: "runtimeReferenceId", LocalAiUnavailableReason: "unavailableReason", Message: "message", NetworkAndroidVpnServiceGateStatus: "networkAndroidVpnServiceGateStatus", NetworkAppleNetworkExtensionGateStatus: "networkAppleNetworkExtensionGateStatus", NetworkLinuxNftablesLabStatus: "networkLinuxNftablesLabStatus", NetworkLiveCaptureStatus: "networkLiveCaptureStatus", NetworkRuntimeDeadLetters: "networkRuntimeDeadLetters", NetworkRuntimeDeliveredRows: "networkRuntimeDeliveredRows", NetworkRuntimeEnforcementCommandEvents: "networkRuntimeEnforcementCommandEvents", NetworkRuntimeEventChainStream: "networkRuntimeEventChainStream", NetworkRuntimeFailedRows: "networkRuntimeFailedRows", NetworkRuntimeManualRequiredRows: "networkRuntimeManualRequiredRows", NetworkRuntimeObservedRows: "networkRuntimeObservedRows", NetworkRuntimePublishReports: "networkRuntimePublishReports", NetworkRuntimeStoredEvents: "networkRuntimeStoredEvents", NetworkRuntimeStreamedEvents: "networkRuntimeStreamedEvents", NetworkRemoteDeliveryStatus: "networkRemoteDeliveryStatus", NetworkWindowsFirewallLabStatus: "networkWindowsFirewallLabStatus", NetworkWindowsWfpGateStatus: "networkWindowsWfpGateStatus", Online: "online", ParentAssistantAnswerState: "parentAssistantAnswerState", ParentAssistantApiAuthorizationState: "parentAssistantApiAuthorizationState", ParentAssistantApiCustodyLabel: "parentAssistantApiCustodyLabel", ParentAssistantApiDeletionState: "parentAssistantApiDeletionState", ParentAssistantApiProviderBoundary: "parentAssistantApiProviderBoundary", ParentAssistantApiRetentionState: "parentAssistantApiRetentionState", ParentAssistantCitationCount: "parentAssistantCitationCount", ParentAssistantEvidenceSummary: "parentAssistantEvidenceSummary", ParentAssistantProviderRoute: "parentAssistantProviderRoute", ParentAssistantRequestId: "parentAssistantRequestId", ParentAssistantQuickActionId: "quickActionId", ParentAssistantPromptTemplateId: "promptTemplateId", ParentAssistantStarterCategory: "starterCategory", ParentAssistantInputText: "inputText", ParentAssistantInputSource: "inputSource", RangeEnd: "rangeEnd", RangeStart: "rangeStart", Reason: "reason", RequestedAt: "requestedAt", Returned: "returned", ScopeKind: "scopeKind", Transport: "transport" } as const; export type ParentAgentProtocolFieldName = (typeof ParentAgentProtocolField)[keyof typeof ParentAgentProtocolField]; export const ParentAgentBrowserRuntimeEventType = { EvidenceObserved: "browser.evidence.observed", EvidenceJournaled: "browser.evidence.journaled", AiAnalysisRequested: "browser.ai.analysis.requested", AiAnalysisCompleted: "browser.ai.analysis.completed", PolicyEvaluationRequested: "browser.policy.evaluation.requested", PolicyDecisionCompleted: "browser.policy.decision.completed", InterventionCommandIssued: "browser.intervention.command.issued", InterventionResultObserved: "browser.intervention.result.observed", AuditEntryCommitted: "browser.audit.entry.committed", ReadModelProjected: "browser.read-model.projected" } as const; export type ParentAgentBrowserRuntimeEventType = (typeof ParentAgentBrowserRuntimeEventType)[keyof typeof ParentAgentBrowserRuntimeEventType]; export const ParentAgentBrowserRuntimePhase = { EvidenceObserved: "EvidenceObserved", EvidenceJournaled: "EvidenceJournaled", AiAnalysisRequested: "AiAnalysisRequested", AiAnalysisCompleted: "AiAnalysisCompleted", PolicyEvaluationRequested: "PolicyEvaluationRequested", PolicyDecisionCompleted: "PolicyDecisionCompleted", InterventionCommandIssued: "InterventionCommandIssued", InterventionResultObserved: "InterventionResultObserved", AuditEntryCommitted: "AuditEntryCommitted", ReadModelProjected: "ReadModelProjected" } as const; export type ParentAgentBrowserRuntimePhase = (typeof ParentAgentBrowserRuntimePhase)[keyof typeof ParentAgentBrowserRuntimePhase]; export const ParentAgentBrowserRuntimeCapabilityStatus = { Available: "available", TabListOnly: "tab-list-only", UnsupportedBrowser: "unsupported-browser", UnmanagedBrowser: "unmanaged-browser", ManagedProfileMissing: "managed-profile-missing", BridgeMissing: "bridge-missing", PermissionLimited: "permission-limited", Stale: "stale", AdapterError: "adapter-error", DisabledByParent: "disabled-by-parent" } as const; export type ParentAgentBrowserRuntimeCapabilityStatus = (typeof ParentAgentBrowserRuntimeCapabilityStatus)[keyof typeof ParentAgentBrowserRuntimeCapabilityStatus]; export const ParentAgentBrowserRuntimeCustodyLabel = { ChildDeviceLocal: "child-device-local", LocalNetworkChildAgent: "local-network-child-agent", ParentCache: "parent-cache", ParentOwnedExport: "parent-owned-export", Unavailable: "unavailable" } as const; export type ParentAgentBrowserRuntimeCustodyLabel = (typeof ParentAgentBrowserRuntimeCustodyLabel)[keyof typeof ParentAgentBrowserRuntimeCustodyLabel]; export const ParentAgentBrowserRuntimeQueryVisibility = { LiveLocal: "live-local", LiveLan: "live-lan", ParentCache: "parent-cache", ParentOwnedExport: "parent-owned-export", Unavailable: "unavailable" } as const; export type ParentAgentBrowserRuntimeQueryVisibility = (typeof ParentAgentBrowserRuntimeQueryVisibility)[keyof typeof ParentAgentBrowserRuntimeQueryVisibility];
export type ParentAgentBrowserRuntimeEventPayload = { readonly phase: ParentAgentBrowserRuntimePhase; readonly sourceRef: string; readonly evidenceRef: string; readonly capabilityStatus: ParentAgentBrowserRuntimeCapabilityStatus; readonly custodyLabel: ParentAgentBrowserRuntimeCustodyLabel; readonly queryVisibility: ParentAgentBrowserRuntimeQueryVisibility; readonly degradedReason: string | null; readonly journalRef: string | null; readonly aiRequestRef: string | null; readonly aiAnalysisRef: string | null; readonly policyEvaluationRef: string | null; readonly policyDecisionRef: string | null; readonly policyPreviewId: string | null; readonly assistantActionIntentId: string | null; readonly interventionCommandRef: string | null; readonly interventionResultRef: string | null; readonly auditEntryRef: string | null; readonly readModelRef: string | null; readonly previousPhaseRef: string | null; readonly exactUrlClaimed: boolean; readonly aiAuthority: false; readonly policyAuthority: boolean; readonly dryRun: boolean; readonly adapterDispatchClaimed: boolean; readonly interventionCommandAllowed: boolean; readonly observedAt: string; };
export type ParentAgentBrowserRuntimeEventChainEntry = { readonly eventType: ParentAgentBrowserRuntimeEventType; readonly eventRef: string; readonly payload: ParentAgentBrowserRuntimeEventPayload; };
export type ParentAgentBrowserRuntimeEventChainStream = { readonly observedRows: number; readonly streamedEvents: number; readonly failedRows: number; readonly exactUrlRows: number; readonly manualRequiredRows: number; readonly interventionCommandEvents: number; readonly readModelProjectionEvents: number; readonly actionIntentCandidates: number; readonly actionIntentHandoffCandidates: number; readonly actionIntentHandoffOutboxRefs: readonly string[]; readonly actionIntentHandoffRefs: readonly string[]; readonly actionIntentChildAcceptedRows: number; readonly actionIntentChildCommandRefs: readonly string[]; readonly actionIntentChildAcceptedEventRefs: readonly string[]; readonly actionIntentParentReadModelRefs: readonly string[]; readonly actionIntentDispatchAttempts: 0; readonly actionIntentAdapterExecutions: 0; readonly actionIntentChildInterventionExecutions: 0; readonly actionIntentEnforcementExecutions: 0; readonly socialProviderReceiptBoundaryRows: number; readonly socialProviderDispatchRequiredRows: number; readonly socialProviderManualReceiptRequiredRows: number; readonly socialProviderAttemptRefs: readonly string[]; readonly socialProviderReceiptProofRefs: readonly string[]; readonly socialProviderDurableRows: number; readonly socialProviderDurableResultRefs: readonly string[]; readonly socialProviderDurableStoreRefs: readonly string[]; readonly socialProviderReadModelRefs: readonly string[]; readonly socialProviderSupportStatusRefs: readonly string[]; readonly entries: readonly ParentAgentBrowserRuntimeEventChainEntry[]; };
export type ParentAgentBrowserRuntimeActionIntentCandidate = { readonly eventRef: string; readonly policyPreviewId: string; readonly assistantActionIntentId: string; readonly sourceRef: string; readonly evidenceRef: string; readonly observedAt: string; };
const ParentAgentBrowserRuntimePhaseEventType = { [ParentAgentBrowserRuntimePhase.EvidenceObserved]: ParentAgentBrowserRuntimeEventType.EvidenceObserved, [ParentAgentBrowserRuntimePhase.EvidenceJournaled]: ParentAgentBrowserRuntimeEventType.EvidenceJournaled, [ParentAgentBrowserRuntimePhase.AiAnalysisRequested]: ParentAgentBrowserRuntimeEventType.AiAnalysisRequested, [ParentAgentBrowserRuntimePhase.AiAnalysisCompleted]: ParentAgentBrowserRuntimeEventType.AiAnalysisCompleted, [ParentAgentBrowserRuntimePhase.PolicyEvaluationRequested]: ParentAgentBrowserRuntimeEventType.PolicyEvaluationRequested, [ParentAgentBrowserRuntimePhase.PolicyDecisionCompleted]: ParentAgentBrowserRuntimeEventType.PolicyDecisionCompleted, [ParentAgentBrowserRuntimePhase.InterventionCommandIssued]: ParentAgentBrowserRuntimeEventType.InterventionCommandIssued, [ParentAgentBrowserRuntimePhase.InterventionResultObserved]: ParentAgentBrowserRuntimeEventType.InterventionResultObserved, [ParentAgentBrowserRuntimePhase.AuditEntryCommitted]: ParentAgentBrowserRuntimeEventType.AuditEntryCommitted, [ParentAgentBrowserRuntimePhase.ReadModelProjected]: ParentAgentBrowserRuntimeEventType.ReadModelProjected } as const;
function __ParentAgentBrowserRuntimeIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __ParentAgentBrowserRuntimeReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string`); } return value; }
function __ParentAgentBrowserRuntimeReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string or null`); } return value; }
function __ParentAgentBrowserRuntimeReadNumber(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value)) { throw new TypeError(`${field} must be a finite browser runtime number`); } return value; }
function __ParentAgentBrowserRuntimeReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a browser runtime boolean`); } return value; }
function __ParentAgentBrowserRuntimeReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __ParentAgentBrowserRuntimeReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __ParentAgentBrowserRuntimeReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __ParentAgentBrowserRuntimeReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned browser runtime literal`); } return value as T; }
function __ParentAgentBrowserRuntimeReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a browser runtime string array`); } return value as readonly string[]; }
function __ParentAgentBrowserRuntimePayloadIsHonest(payload: ParentAgentBrowserRuntimeEventPayload): boolean { if (!__ParentAgentBrowserRuntimeContextSupportsExactUrl(payload) && payload.exactUrlClaimed) { return false; } if (!__ParentAgentBrowserRuntimeUnavailableContextHasReason(payload)) { return false; } if (!payload.exactUrlClaimed && payload.interventionCommandAllowed) { return false; } if (!__ParentAgentBrowserRuntimeDryRunHasNoDispatch(payload)) { return false; } if (payload.adapterDispatchClaimed && !payload.interventionCommandAllowed) { return false; } if (!payload.interventionCommandAllowed) { return payload.interventionCommandRef === null && payload.interventionResultRef === null; } return payload.interventionCommandRef !== null && payload.adapterDispatchClaimed; }
function __ParentAgentBrowserRuntimeContextSupportsExactUrl(payload: ParentAgentBrowserRuntimeEventPayload): boolean { const capabilityAllowsExactUrl = payload.capabilityStatus === ParentAgentBrowserRuntimeCapabilityStatus.Available || payload.capabilityStatus === ParentAgentBrowserRuntimeCapabilityStatus.TabListOnly; const queryAllowsExactUrl = payload.queryVisibility === ParentAgentBrowserRuntimeQueryVisibility.LiveLocal || payload.queryVisibility === ParentAgentBrowserRuntimeQueryVisibility.LiveLan; return capabilityAllowsExactUrl && queryAllowsExactUrl && payload.custodyLabel !== ParentAgentBrowserRuntimeCustodyLabel.Unavailable; }
function __ParentAgentBrowserRuntimeUnavailableContextHasReason(payload: ParentAgentBrowserRuntimeEventPayload): boolean { if (payload.queryVisibility !== ParentAgentBrowserRuntimeQueryVisibility.Unavailable && payload.capabilityStatus !== ParentAgentBrowserRuntimeCapabilityStatus.BridgeMissing && payload.capabilityStatus !== ParentAgentBrowserRuntimeCapabilityStatus.Stale && payload.capabilityStatus !== ParentAgentBrowserRuntimeCapabilityStatus.AdapterError) { return true; } return payload.degradedReason !== null; }
function __ParentAgentBrowserRuntimeDryRunHasNoDispatch(payload: ParentAgentBrowserRuntimeEventPayload): boolean { if (!payload.dryRun) { return true; } return !payload.adapterDispatchClaimed && !payload.interventionCommandAllowed && payload.interventionCommandRef === null && payload.interventionResultRef === null; }
function __ParentAgentBrowserRuntimeActionIntentCandidatesFromEntries(entries: readonly ParentAgentBrowserRuntimeEventChainEntry[]): ParentAgentBrowserRuntimeActionIntentCandidate[] { return entries.flatMap((entry) => { const payload = entry.payload; if (payload.phase !== ParentAgentBrowserRuntimePhase.PolicyDecisionCompleted || !payload.dryRun || !payload.policyAuthority || payload.policyPreviewId === null || payload.assistantActionIntentId === null) { return []; } return [{ eventRef: entry.eventRef, policyPreviewId: payload.policyPreviewId, assistantActionIntentId: payload.assistantActionIntentId, sourceRef: payload.sourceRef, evidenceRef: payload.evidenceRef, observedAt: payload.observedAt }]; }); }
function __ParentAgentBrowserRuntimeActionIntentChildStatusIsHonest(stream: ParentAgentBrowserRuntimeEventChainStream): boolean { return stream.actionIntentChildCommandRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentChildAcceptedEventRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentParentReadModelRefs.length === stream.actionIntentChildAcceptedRows; }
function __ParentAgentBrowserRuntimeSocialProviderReceiptRefsAreEmpty(stream: ParentAgentBrowserRuntimeEventChainStream): boolean { return stream.socialProviderAttemptRefs.length === 0 && stream.socialProviderReceiptProofRefs.length === 0 && stream.socialProviderDurableRows === 0 && stream.socialProviderDurableResultRefs.length === 0 && stream.socialProviderDurableStoreRefs.length === 0 && stream.socialProviderReadModelRefs.length === 0 && stream.socialProviderSupportStatusRefs.length === 0; }
function __ParentAgentBrowserRuntimeSocialProviderReceiptStateIsHonest(stream: ParentAgentBrowserRuntimeEventChainStream): boolean { if (stream.socialProviderReceiptBoundaryRows !== stream.socialProviderDispatchRequiredRows + stream.socialProviderManualReceiptRequiredRows) { return false; } if (stream.socialProviderDispatchRequiredRows === 0) { return __ParentAgentBrowserRuntimeSocialProviderReceiptRefsAreEmpty(stream); } return stream.socialProviderAttemptRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderReceiptProofRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableRows === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableResultRefs.length === stream.socialProviderDurableRows && stream.socialProviderDurableStoreRefs.length === stream.socialProviderDurableRows && stream.socialProviderReadModelRefs.length === stream.socialProviderDurableRows && stream.socialProviderSupportStatusRefs.length === stream.socialProviderDurableRows; }
function __ParentAgentBrowserRuntimeStreamIsHonest(stream: ParentAgentBrowserRuntimeEventChainStream): boolean { return stream.streamedEvents === stream.entries.length && stream.actionIntentCandidates >= __ParentAgentBrowserRuntimeActionIntentCandidatesFromEntries(stream.entries).length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffOutboxRefs.length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffRefs.length && stream.actionIntentHandoffOutboxRefs.length === stream.actionIntentHandoffRefs.length && __ParentAgentBrowserRuntimeActionIntentChildStatusIsHonest(stream) && __ParentAgentBrowserRuntimeSocialProviderReceiptStateIsHonest(stream); }
export function decodeParentAgentBrowserRuntimeEventPayload(value: unknown): ParentAgentBrowserRuntimeEventPayload { if (!__ParentAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime payload must be an object'); } const payload: ParentAgentBrowserRuntimeEventPayload = { phase: __ParentAgentBrowserRuntimeReadLiteral(value, 'phase', Object.values(ParentAgentBrowserRuntimePhase)), sourceRef: __ParentAgentBrowserRuntimeReadString(value, 'sourceRef'), evidenceRef: __ParentAgentBrowserRuntimeReadString(value, 'evidenceRef'), capabilityStatus: __ParentAgentBrowserRuntimeReadLiteral(value, 'capabilityStatus', Object.values(ParentAgentBrowserRuntimeCapabilityStatus)), custodyLabel: __ParentAgentBrowserRuntimeReadLiteral(value, 'custodyLabel', Object.values(ParentAgentBrowserRuntimeCustodyLabel)), queryVisibility: __ParentAgentBrowserRuntimeReadLiteral(value, 'queryVisibility', Object.values(ParentAgentBrowserRuntimeQueryVisibility)), degradedReason: __ParentAgentBrowserRuntimeReadNullableString(value, 'degradedReason'), journalRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'journalRef'), aiRequestRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'aiRequestRef'), aiAnalysisRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'aiAnalysisRef'), policyEvaluationRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'policyEvaluationRef'), policyDecisionRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'policyDecisionRef'), policyPreviewId: __ParentAgentBrowserRuntimeReadNullableString(value, 'policyPreviewId'), assistantActionIntentId: __ParentAgentBrowserRuntimeReadNullableString(value, 'assistantActionIntentId'), interventionCommandRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'interventionCommandRef'), interventionResultRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'interventionResultRef'), auditEntryRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'auditEntryRef'), readModelRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'readModelRef'), previousPhaseRef: __ParentAgentBrowserRuntimeReadNullableString(value, 'previousPhaseRef'), exactUrlClaimed: __ParentAgentBrowserRuntimeReadBoolean(value, 'exactUrlClaimed'), aiAuthority: __ParentAgentBrowserRuntimeReadRequiredBoolean(value, 'aiAuthority', false), policyAuthority: __ParentAgentBrowserRuntimeReadBoolean(value, 'policyAuthority'), dryRun: __ParentAgentBrowserRuntimeReadBoolean(value, 'dryRun'), adapterDispatchClaimed: __ParentAgentBrowserRuntimeReadBoolean(value, 'adapterDispatchClaimed'), interventionCommandAllowed: __ParentAgentBrowserRuntimeReadBoolean(value, 'interventionCommandAllowed'), observedAt: __ParentAgentBrowserRuntimeReadString(value, 'observedAt') }; if (!__ParentAgentBrowserRuntimePayloadIsHonest(payload)) { throw new TypeError('browser runtime payload violates Rust-owned claim boundaries'); } return payload; }
export function decodeParentAgentBrowserRuntimeEventChainEntry(value: unknown): ParentAgentBrowserRuntimeEventChainEntry { if (!__ParentAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime entry must be an object'); } const entry: ParentAgentBrowserRuntimeEventChainEntry = { eventType: __ParentAgentBrowserRuntimeReadLiteral(value, 'eventType', Object.values(ParentAgentBrowserRuntimeEventType)), eventRef: __ParentAgentBrowserRuntimeReadString(value, 'eventRef'), payload: decodeParentAgentBrowserRuntimeEventPayload(value['payload']) }; if (ParentAgentBrowserRuntimePhaseEventType[entry.payload.phase] !== entry.eventType) { throw new TypeError('browser runtime event type must match payload phase'); } return entry; }
export function decodeParentAgentBrowserRuntimeEventChainStream(value: unknown): ParentAgentBrowserRuntimeEventChainStream { if (!__ParentAgentBrowserRuntimeIsRecord(value)) { throw new TypeError('browser runtime stream must be an object'); } const entriesValue = value['entries']; if (!Array.isArray(entriesValue)) { throw new TypeError('entries must be a browser runtime array'); } const stream: ParentAgentBrowserRuntimeEventChainStream = { observedRows: __ParentAgentBrowserRuntimeReadNumber(value, 'observedRows'), streamedEvents: __ParentAgentBrowserRuntimeReadNumber(value, 'streamedEvents'), failedRows: __ParentAgentBrowserRuntimeReadNumber(value, 'failedRows'), exactUrlRows: __ParentAgentBrowserRuntimeReadNumber(value, 'exactUrlRows'), manualRequiredRows: __ParentAgentBrowserRuntimeReadNumber(value, 'manualRequiredRows'), interventionCommandEvents: __ParentAgentBrowserRuntimeReadNumber(value, 'interventionCommandEvents'), readModelProjectionEvents: __ParentAgentBrowserRuntimeReadNumber(value, 'readModelProjectionEvents'), actionIntentCandidates: __ParentAgentBrowserRuntimeReadNumber(value, 'actionIntentCandidates'), actionIntentHandoffCandidates: __ParentAgentBrowserRuntimeReadNumber(value, 'actionIntentHandoffCandidates'), actionIntentHandoffOutboxRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'actionIntentHandoffOutboxRefs'), actionIntentHandoffRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'actionIntentHandoffRefs'), actionIntentChildAcceptedRows: __ParentAgentBrowserRuntimeReadNumber(value, 'actionIntentChildAcceptedRows'), actionIntentChildCommandRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'actionIntentChildCommandRefs'), actionIntentChildAcceptedEventRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'actionIntentChildAcceptedEventRefs'), actionIntentParentReadModelRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'actionIntentParentReadModelRefs'), actionIntentDispatchAttempts: __ParentAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentDispatchAttempts', 0), actionIntentAdapterExecutions: __ParentAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentAdapterExecutions', 0), actionIntentChildInterventionExecutions: __ParentAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentChildInterventionExecutions', 0), actionIntentEnforcementExecutions: __ParentAgentBrowserRuntimeReadRequiredNumber(value, 'actionIntentEnforcementExecutions', 0), socialProviderReceiptBoundaryRows: __ParentAgentBrowserRuntimeReadNumber(value, 'socialProviderReceiptBoundaryRows'), socialProviderDispatchRequiredRows: __ParentAgentBrowserRuntimeReadNumber(value, 'socialProviderDispatchRequiredRows'), socialProviderManualReceiptRequiredRows: __ParentAgentBrowserRuntimeReadNumber(value, 'socialProviderManualReceiptRequiredRows'), socialProviderAttemptRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderAttemptRefs'), socialProviderReceiptProofRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderReceiptProofRefs'), socialProviderDurableRows: __ParentAgentBrowserRuntimeReadNumber(value, 'socialProviderDurableRows'), socialProviderDurableResultRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderDurableResultRefs'), socialProviderDurableStoreRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderDurableStoreRefs'), socialProviderReadModelRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderReadModelRefs'), socialProviderSupportStatusRefs: __ParentAgentBrowserRuntimeReadStringArray(value, 'socialProviderSupportStatusRefs'), entries: entriesValue.map((entry) => decodeParentAgentBrowserRuntimeEventChainEntry(entry)) }; if (!__ParentAgentBrowserRuntimeStreamIsHonest(stream)) { throw new TypeError('browser runtime stream violates Rust-owned claim boundaries'); } return stream; }
function __ParentAgentBrowserRuntimeReadRequiredNumber<T extends number>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __ParentAgentBrowserRuntimeReadNumber(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
export const ParentAgentBrowserRuntimeEventPayloadSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentBrowserRuntimeEventPayload } | { readonly success: false } { try { return { success: true, data: decodeParentAgentBrowserRuntimeEventPayload(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentBrowserRuntimeEventChainEntrySchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentBrowserRuntimeEventChainEntry } | { readonly success: false } { try { return { success: true, data: decodeParentAgentBrowserRuntimeEventChainEntry(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentBrowserRuntimeEventChainStreamSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentBrowserRuntimeEventChainStream } | { readonly success: false } { try { return { success: true, data: decodeParentAgentBrowserRuntimeEventChainStream(value) }; } catch { return { success: false }; } } } as const;
 export const ParentAgentNetworkRuntimeEventType = { NetworkFlowObserved: "network.flow.observed", NetworkDomainObserved: "network.domain.observed", NetworkActivityClassified: "network.activity.classified", AiAnalysisRequested: "ai.analysis.requested", AiAnalysisCompleted: "ai.analysis.completed", PolicyEvaluationRequested: "policy.evaluation.requested", PolicyDecisionCompleted: "policy.decision.completed", EnforcementCommandIssued: "enforcement.command.issued", EnforcementResultObserved: "enforcement.result.observed", AuditEntryCommitted: "audit.entry.committed", PortalReadModelUpdated: "portal.read_model.updated" } as const; export type ParentAgentNetworkRuntimeEventType = (typeof ParentAgentNetworkRuntimeEventType)[keyof typeof ParentAgentNetworkRuntimeEventType]; export const ParentAgentNetworkEvidenceGrade = { A: "A", B: "B", C: "C", D: "D" } as const; export type ParentAgentNetworkEvidenceGrade = (typeof ParentAgentNetworkEvidenceGrade)[keyof typeof ParentAgentNetworkEvidenceGrade]; export const ParentAgentNetworkDomainAttributionKind = { DnsAnswer: "dns-answer", SniVisible: "sni-visible", HttpHost: "http-host", ReverseLookup: "reverse-lookup", IpOnly: "ip-only", Unavailable: "unavailable" } as const; export type ParentAgentNetworkDomainAttributionKind = (typeof ParentAgentNetworkDomainAttributionKind)[keyof typeof ParentAgentNetworkDomainAttributionKind]; export const ParentAgentNetworkRuntimeActivityKind = { SocialCandidate: "social-candidate", VideoCandidate: "video-candidate", GameCandidate: "game-candidate", VpnProxyTunnelCandidate: "vpn-proxy-tunnel-candidate", Unknown: "unknown" } as const; export type ParentAgentNetworkRuntimeActivityKind = (typeof ParentAgentNetworkRuntimeActivityKind)[keyof typeof ParentAgentNetworkRuntimeActivityKind]; export const ParentAgentNetworkAiAdvisoryState = { Requested: "requested", Completed: "completed", ManualReviewRequired: "manual-review-required", ProviderUnavailable: "provider-unavailable" } as const; export type ParentAgentNetworkAiAdvisoryState = (typeof ParentAgentNetworkAiAdvisoryState)[keyof typeof ParentAgentNetworkAiAdvisoryState]; export const ParentAgentNetworkPolicyDecisionAction = { Observe: "observe", Warn: "warn", AskParent: "ask-parent", Limit: "limit", Block: "block", ManualReview: "manual-review", Unknown: "unknown" } as const; export type ParentAgentNetworkPolicyDecisionAction = (typeof ParentAgentNetworkPolicyDecisionAction)[keyof typeof ParentAgentNetworkPolicyDecisionAction]; export const ParentAgentNetworkEnforcementMode = { DryRun: "dry-run", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type ParentAgentNetworkEnforcementMode = (typeof ParentAgentNetworkEnforcementMode)[keyof typeof ParentAgentNetworkEnforcementMode]; export const ParentAgentNetworkEnforcementResultStatus = { DryRun: "dry-run", ManualRequired: "manual-required", Unavailable: "unavailable", Rejected: "rejected" } as const; export type ParentAgentNetworkEnforcementResultStatus = (typeof ParentAgentNetworkEnforcementResultStatus)[keyof typeof ParentAgentNetworkEnforcementResultStatus]; export const ParentAgentNetworkAuditOutcome = { Committed: "committed", Failed: "failed" } as const; export type ParentAgentNetworkAuditOutcome = (typeof ParentAgentNetworkAuditOutcome)[keyof typeof ParentAgentNetworkAuditOutcome]; export const ParentAgentNetworkPortalUpdateKind = { NetworkReadModel: "network-read-model", CapabilityState: "capability-state", ManualRequiredState: "manual-required-state" } as const; export type ParentAgentNetworkPortalUpdateKind = (typeof ParentAgentNetworkPortalUpdateKind)[keyof typeof ParentAgentNetworkPortalUpdateKind];
export type ParentAgentNetworkClaimBoundary = { readonly exactUrlAvailable: boolean; readonly decryptedHttpsPayloadAvailable: boolean; readonly messageContentAvailable: boolean; readonly searchQueryAvailable: boolean; readonly adapterActionExecuted: boolean; };
export type ParentAgentNetworkFlowObservedEvent = { readonly schemaVersion: number; readonly flowEventRef: string; readonly observedAt: string; readonly deviceRef: string; readonly flowEvidenceRef: string; readonly custody: string; readonly evidenceGrade: ParentAgentNetworkEvidenceGrade; readonly claimBoundary: ParentAgentNetworkClaimBoundary; };
export type ParentAgentNetworkDomainObservedEvent = { readonly schemaVersion: number; readonly domainEventRef: string; readonly previousEventRef: string; readonly flowEvidenceRef: string; readonly domainEvidenceRef: string; readonly attribution: ParentAgentNetworkDomainAttributionKind; readonly evidenceGrade: ParentAgentNetworkEvidenceGrade; readonly uncertaintyCodes: readonly string[]; readonly claimBoundary: ParentAgentNetworkClaimBoundary; };
export type ParentAgentNetworkActivityClassifiedEvent = { readonly schemaVersion: number; readonly classificationEventRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly activityKind: ParentAgentNetworkRuntimeActivityKind; readonly confidence: number; readonly evidenceGrade: ParentAgentNetworkEvidenceGrade; readonly uncertaintyCodes: readonly string[]; };
export type ParentAgentNetworkAiAnalysisRequestedEvent = { readonly schemaVersion: number; readonly aiRequestRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly promptTemplateRef: string; readonly custody: string; readonly rawPacketPayloadIncluded: false; };
export type ParentAgentNetworkAiAnalysisCompletedEvent = { readonly schemaVersion: number; readonly aiAnalysisRef: string; readonly aiRequestRef: string; readonly previousEventRef: string; readonly advisoryState: ParentAgentNetworkAiAdvisoryState; readonly evidenceRefs: readonly string[]; readonly unsupportedClaims: readonly string[]; };
export type ParentAgentNetworkPolicyEvaluationRequestedEvent = { readonly schemaVersion: number; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly aiAnalysisRef: string | null; readonly parentRuleRefs: readonly string[]; readonly dryRun: boolean; };
export type ParentAgentNetworkPolicyDecisionCompletedEvent = { readonly schemaVersion: number; readonly policyDecisionRef: string; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly decisionAction: ParentAgentNetworkPolicyDecisionAction; readonly evidenceRefs: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly adapterCapabilityRequired: boolean; };
export type ParentAgentNetworkEnforcementCommandIssuedEvent = { readonly schemaVersion: number; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly adapterCapabilityRef: string; readonly enforcementMode: ParentAgentNetworkEnforcementMode; readonly evidenceRefs: readonly string[]; readonly rollbackRef: string | null; };
export type ParentAgentNetworkEnforcementResultObservedEvent = { readonly schemaVersion: number; readonly enforcementResultRef: string; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly resultStatus: ParentAgentNetworkEnforcementResultStatus; readonly adapterActionExecuted: false; readonly rollbackRef: string | null; readonly unavailableReasonCode: string | null; };
export type ParentAgentNetworkAuditEntryCommittedEvent = { readonly schemaVersion: number; readonly auditEntryRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly enforcementCommandRef: string | null; readonly enforcementResultRef: string | null; readonly evidenceRefs: readonly string[]; readonly auditOutcome: ParentAgentNetworkAuditOutcome; };
export type ParentAgentNetworkPortalReadModelUpdatedEvent = { readonly schemaVersion: number; readonly readModelRef: string; readonly previousEventRef: string; readonly auditEntryRef: string; readonly updateKind: ParentAgentNetworkPortalUpdateKind; readonly visibleManualRequired: boolean; readonly visibleUnavailable: boolean; };
export type ParentAgentNetworkRuntimeEventPayload = ParentAgentNetworkFlowObservedEvent | ParentAgentNetworkDomainObservedEvent | ParentAgentNetworkActivityClassifiedEvent | ParentAgentNetworkAiAnalysisRequestedEvent | ParentAgentNetworkAiAnalysisCompletedEvent | ParentAgentNetworkPolicyEvaluationRequestedEvent | ParentAgentNetworkPolicyDecisionCompletedEvent | ParentAgentNetworkEnforcementCommandIssuedEvent | ParentAgentNetworkEnforcementResultObservedEvent | ParentAgentNetworkAuditEntryCommittedEvent | ParentAgentNetworkPortalReadModelUpdatedEvent;
function __ParentAgentNetworkRuntimeIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __ParentAgentNetworkRuntimeReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__ParentAgentNetworkRuntimeIsRecord(value)) { throw new TypeError(`${label} must be a network runtime object`); } return value; }
function __ParentAgentNetworkRuntimeReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string`); } return value; }
function __ParentAgentNetworkRuntimeReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string or null`); } return value; }
function __ParentAgentNetworkRuntimeReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network runtime boolean`); } return value; }
function __ParentAgentNetworkRuntimeReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __ParentAgentNetworkRuntimeReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __ParentAgentNetworkRuntimeReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { const value = record['schemaVersion']; if (value !== ParentAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return ParentAgentProtocolRuntime.SchemaVersion; }
function __ParentAgentNetworkRuntimeReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __ParentAgentNetworkRuntimeReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network runtime literal`); } return value as T; }
function __ParentAgentNetworkRuntimeReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a network runtime string array`); } return value as readonly string[]; }
function __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = __ParentAgentNetworkRuntimeReadStringArray(record, field); if (value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string array`); } return value; }
function __ParentAgentNetworkRuntimeReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be a network runtime confidence from 0 to 1`); } return value; }
function __ParentAgentNetworkRuntimeReadClaimBoundary(value: unknown): ParentAgentNetworkClaimBoundary { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'claimBoundary'); const boundary = { exactUrlAvailable: __ParentAgentNetworkRuntimeReadBoolean(record, 'exactUrlAvailable'), decryptedHttpsPayloadAvailable: __ParentAgentNetworkRuntimeReadBoolean(record, 'decryptedHttpsPayloadAvailable'), messageContentAvailable: __ParentAgentNetworkRuntimeReadBoolean(record, 'messageContentAvailable'), searchQueryAvailable: __ParentAgentNetworkRuntimeReadBoolean(record, 'searchQueryAvailable'), adapterActionExecuted: __ParentAgentNetworkRuntimeReadBoolean(record, 'adapterActionExecuted') }; if (boundary.exactUrlAvailable || boundary.decryptedHttpsPayloadAvailable || boundary.messageContentAvailable || boundary.searchQueryAvailable || boundary.adapterActionExecuted) { throw new TypeError('network runtime claim boundary cannot claim unsupported content or adapter action'); } return boundary; }
function __ParentAgentNetworkRuntimeDecodeFlowObserved(value: unknown): ParentAgentNetworkFlowObservedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network flow observed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), flowEventRef: __ParentAgentNetworkRuntimeReadString(record, 'flowEventRef'), observedAt: __ParentAgentNetworkRuntimeReadString(record, 'observedAt'), deviceRef: __ParentAgentNetworkRuntimeReadString(record, 'deviceRef'), flowEvidenceRef: __ParentAgentNetworkRuntimeReadString(record, 'flowEvidenceRef'), custody: __ParentAgentNetworkRuntimeReadString(record, 'custody'), evidenceGrade: __ParentAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(ParentAgentNetworkEvidenceGrade)), claimBoundary: __ParentAgentNetworkRuntimeReadClaimBoundary(record['claimBoundary']) }; }
function __ParentAgentNetworkRuntimeDecodeDomainObserved(value: unknown): ParentAgentNetworkDomainObservedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network domain observed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), domainEventRef: __ParentAgentNetworkRuntimeReadString(record, 'domainEventRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), flowEvidenceRef: __ParentAgentNetworkRuntimeReadString(record, 'flowEvidenceRef'), domainEvidenceRef: __ParentAgentNetworkRuntimeReadString(record, 'domainEvidenceRef'), attribution: __ParentAgentNetworkRuntimeReadLiteral(record, 'attribution', Object.values(ParentAgentNetworkDomainAttributionKind)), evidenceGrade: __ParentAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(ParentAgentNetworkEvidenceGrade)), uncertaintyCodes: __ParentAgentNetworkRuntimeReadStringArray(record, 'uncertaintyCodes'), claimBoundary: __ParentAgentNetworkRuntimeReadClaimBoundary(record['claimBoundary']) }; }
function __ParentAgentNetworkRuntimeDecodeActivityClassified(value: unknown): ParentAgentNetworkActivityClassifiedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network activity classified payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), classificationEventRef: __ParentAgentNetworkRuntimeReadString(record, 'classificationEventRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), activityKind: __ParentAgentNetworkRuntimeReadLiteral(record, 'activityKind', Object.values(ParentAgentNetworkRuntimeActivityKind)), confidence: __ParentAgentNetworkRuntimeReadConfidence(record, 'confidence'), evidenceGrade: __ParentAgentNetworkRuntimeReadLiteral(record, 'evidenceGrade', Object.values(ParentAgentNetworkEvidenceGrade)), uncertaintyCodes: __ParentAgentNetworkRuntimeReadStringArray(record, 'uncertaintyCodes') }; }
function __ParentAgentNetworkRuntimeDecodeAiAnalysisRequested(value: unknown): ParentAgentNetworkAiAnalysisRequestedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network AI analysis requested payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), aiRequestRef: __ParentAgentNetworkRuntimeReadString(record, 'aiRequestRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), promptTemplateRef: __ParentAgentNetworkRuntimeReadString(record, 'promptTemplateRef'), custody: __ParentAgentNetworkRuntimeReadString(record, 'custody'), rawPacketPayloadIncluded: __ParentAgentNetworkRuntimeReadRequiredBoolean(record, 'rawPacketPayloadIncluded', false) }; }
function __ParentAgentNetworkRuntimeDecodeAiAnalysisCompleted(value: unknown): ParentAgentNetworkAiAnalysisCompletedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network AI analysis completed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), aiAnalysisRef: __ParentAgentNetworkRuntimeReadString(record, 'aiAnalysisRef'), aiRequestRef: __ParentAgentNetworkRuntimeReadString(record, 'aiRequestRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), advisoryState: __ParentAgentNetworkRuntimeReadLiteral(record, 'advisoryState', Object.values(ParentAgentNetworkAiAdvisoryState)), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), unsupportedClaims: __ParentAgentNetworkRuntimeReadStringArray(record, 'unsupportedClaims') }; }
function __ParentAgentNetworkRuntimeDecodePolicyEvaluationRequested(value: unknown): ParentAgentNetworkPolicyEvaluationRequestedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network policy evaluation requested payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), policyEvaluationRef: __ParentAgentNetworkRuntimeReadString(record, 'policyEvaluationRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), aiAnalysisRef: __ParentAgentNetworkRuntimeReadNullableString(record, 'aiAnalysisRef'), parentRuleRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'parentRuleRefs'), dryRun: __ParentAgentNetworkRuntimeReadBoolean(record, 'dryRun') }; }
function __ParentAgentNetworkRuntimeDecodePolicyDecisionCompleted(value: unknown): ParentAgentNetworkPolicyDecisionCompletedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network policy decision completed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), policyDecisionRef: __ParentAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), policyEvaluationRef: __ParentAgentNetworkRuntimeReadString(record, 'policyEvaluationRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), decisionAction: __ParentAgentNetworkRuntimeReadLiteral(record, 'decisionAction', Object.values(ParentAgentNetworkPolicyDecisionAction)), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), parentRuleRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'parentRuleRefs'), adapterCapabilityRequired: __ParentAgentNetworkRuntimeReadBoolean(record, 'adapterCapabilityRequired') }; }
function __ParentAgentNetworkRuntimeDecodeEnforcementCommandIssued(value: unknown): ParentAgentNetworkEnforcementCommandIssuedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network enforcement command issued payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), enforcementCommandRef: __ParentAgentNetworkRuntimeReadString(record, 'enforcementCommandRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), policyDecisionRef: __ParentAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), adapterCapabilityRef: __ParentAgentNetworkRuntimeReadString(record, 'adapterCapabilityRef'), enforcementMode: __ParentAgentNetworkRuntimeReadLiteral(record, 'enforcementMode', Object.values(ParentAgentNetworkEnforcementMode)), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), rollbackRef: __ParentAgentNetworkRuntimeReadNullableString(record, 'rollbackRef') }; }
function __ParentAgentNetworkRuntimeDecodeEnforcementResultObserved(value: unknown): ParentAgentNetworkEnforcementResultObservedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network enforcement result observed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), enforcementResultRef: __ParentAgentNetworkRuntimeReadString(record, 'enforcementResultRef'), enforcementCommandRef: __ParentAgentNetworkRuntimeReadString(record, 'enforcementCommandRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), resultStatus: __ParentAgentNetworkRuntimeReadLiteral(record, 'resultStatus', Object.values(ParentAgentNetworkEnforcementResultStatus)), adapterActionExecuted: __ParentAgentNetworkRuntimeReadRequiredBoolean(record, 'adapterActionExecuted', false), rollbackRef: __ParentAgentNetworkRuntimeReadNullableString(record, 'rollbackRef'), unavailableReasonCode: __ParentAgentNetworkRuntimeReadNullableString(record, 'unavailableReasonCode') }; }
function __ParentAgentNetworkRuntimeDecodeAuditEntryCommitted(value: unknown): ParentAgentNetworkAuditEntryCommittedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network audit entry committed payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), auditEntryRef: __ParentAgentNetworkRuntimeReadString(record, 'auditEntryRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), policyDecisionRef: __ParentAgentNetworkRuntimeReadString(record, 'policyDecisionRef'), enforcementCommandRef: __ParentAgentNetworkRuntimeReadNullableString(record, 'enforcementCommandRef'), enforcementResultRef: __ParentAgentNetworkRuntimeReadNullableString(record, 'enforcementResultRef'), evidenceRefs: __ParentAgentNetworkRuntimeReadNonEmptyStringArray(record, 'evidenceRefs'), auditOutcome: __ParentAgentNetworkRuntimeReadLiteral(record, 'auditOutcome', Object.values(ParentAgentNetworkAuditOutcome)) }; }
function __ParentAgentNetworkRuntimeDecodePortalReadModelUpdated(value: unknown): ParentAgentNetworkPortalReadModelUpdatedEvent { const record = __ParentAgentNetworkRuntimeReadRecord(value, 'network portal read model updated payload'); return { schemaVersion: __ParentAgentNetworkRuntimeReadSchemaVersion(record), readModelRef: __ParentAgentNetworkRuntimeReadString(record, 'readModelRef'), previousEventRef: __ParentAgentNetworkRuntimeReadString(record, 'previousEventRef'), auditEntryRef: __ParentAgentNetworkRuntimeReadString(record, 'auditEntryRef'), updateKind: __ParentAgentNetworkRuntimeReadLiteral(record, 'updateKind', Object.values(ParentAgentNetworkPortalUpdateKind)), visibleManualRequired: __ParentAgentNetworkRuntimeReadBoolean(record, 'visibleManualRequired'), visibleUnavailable: __ParentAgentNetworkRuntimeReadBoolean(record, 'visibleUnavailable') }; }
export function decodeParentAgentNetworkRuntimeEventPayload(eventType: ParentAgentNetworkRuntimeEventType, value: unknown): ParentAgentNetworkRuntimeEventPayload { switch (eventType) { case ParentAgentNetworkRuntimeEventType.NetworkFlowObserved: return __ParentAgentNetworkRuntimeDecodeFlowObserved(value); case ParentAgentNetworkRuntimeEventType.NetworkDomainObserved: return __ParentAgentNetworkRuntimeDecodeDomainObserved(value); case ParentAgentNetworkRuntimeEventType.NetworkActivityClassified: return __ParentAgentNetworkRuntimeDecodeActivityClassified(value); case ParentAgentNetworkRuntimeEventType.AiAnalysisRequested: return __ParentAgentNetworkRuntimeDecodeAiAnalysisRequested(value); case ParentAgentNetworkRuntimeEventType.AiAnalysisCompleted: return __ParentAgentNetworkRuntimeDecodeAiAnalysisCompleted(value); case ParentAgentNetworkRuntimeEventType.PolicyEvaluationRequested: return __ParentAgentNetworkRuntimeDecodePolicyEvaluationRequested(value); case ParentAgentNetworkRuntimeEventType.PolicyDecisionCompleted: return __ParentAgentNetworkRuntimeDecodePolicyDecisionCompleted(value); case ParentAgentNetworkRuntimeEventType.EnforcementCommandIssued: return __ParentAgentNetworkRuntimeDecodeEnforcementCommandIssued(value); case ParentAgentNetworkRuntimeEventType.EnforcementResultObserved: return __ParentAgentNetworkRuntimeDecodeEnforcementResultObserved(value); case ParentAgentNetworkRuntimeEventType.AuditEntryCommitted: return __ParentAgentNetworkRuntimeDecodeAuditEntryCommitted(value); case ParentAgentNetworkRuntimeEventType.PortalReadModelUpdated: return __ParentAgentNetworkRuntimeDecodePortalReadModelUpdated(value); } }
export const ParentAgentNetworkRuntimeEventTypeSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkRuntimeEventType } | { readonly success: false } { if (typeof value === 'string' && (Object.values(ParentAgentNetworkRuntimeEventType) as readonly string[]).includes(value)) { return { success: true, data: value as ParentAgentNetworkRuntimeEventType }; } return { success: false }; } } as const;
 export const ParentAgentNetworkRemoteDeliveryStatusRefs = { StatusRef: "network.remote-delivery.external-cross-process-transport-status.10t", EventChainJournalRef: "network.remote-delivery.event-chain-journal.10c", ReceiptLedgerRef: "network.remote-delivery.event-chain.receipt-ledger.10d", LocalReceiptAckRef: "network.remote-delivery.event-chain.local-receipt-ack.10d", DurableEnvelopeRef: "network.remote-delivery.durable-envelope.10e", DurableStoreRef: "network.remote-delivery.durable-envelope-store.10e", DurableReplayRef: "network.remote-delivery.durable-envelope-replay.10e", DurableDeleteExportRef: "network.remote-delivery.durable-envelope-delete-export.10e", DurableSupportStatusRef: "network.remote-delivery.durable-envelope-support-status.10e", OutboxRef: "network.remote-delivery.outbox.10g", OutboxHandoffRef: "network.remote-delivery.outbox-handoff.10g", OutboxReplayRef: "network.remote-delivery.outbox-replay.10g", OutboxSupportStatusRef: "network.remote-delivery.outbox-support-status.10g", TransportDispatchStateRef: "network.remote-delivery.transport-dispatch-state.10k", BlockedDispatchRef: "network.remote-delivery.dispatch-blocked-manual-required.10k", FutureTransportSeamRef: "network.remote-delivery.future-transport-seam.10k", FixtureTransportRef: "network.remote-delivery.fixture-transport.10l", FixtureDispatchAttemptRef: "network.remote-delivery.fixture-dispatch-attempt.10l", FixtureAckRef: "network.remote-delivery.fixture-ack.10l", DeleteExportPropagationRef: "network.remote-delivery.delete-export-propagation-readiness.10m", RemoteDeleteReadinessRef: "network.remote-delivery.remote-delete-readiness.10m", RemoteExportReadinessRef: "network.remote-delivery.remote-export-readiness.10m", ProviderRouteRef: "network.remote-delivery.provider-route.10p", ChildDeviceRouteRef: "network.remote-delivery.child-device-route.10p", ProviderDeliveryReadinessRef: "network.remote-delivery.provider-readiness.10p", ChildDeviceDeliveryReadinessRef: "network.remote-delivery.child-device-readiness.10p", CrossProcessCustodyStatusRef: "network.remote-delivery.cross-process-custody-status.10q", CrossProcessReplayReadinessRef: "network.remote-delivery.cross-process-replay-readiness.10q", RemoteRetentionReadinessRef: "network.remote-delivery.remote-retention-readiness.10q", RemoteDeleteCustodyReadinessRef: "network.remote-delivery.remote-delete-custody-readiness.10q", RemoteExportCustodyReadinessRef: "network.remote-delivery.remote-export-custody-readiness.10q", CrossProcessReplayRef: "network.remote-delivery.cross-process-replay.10r", CrossProcessReplayStoreRef: "network.remote-delivery.cross-process-replay-store.10r", CrossProcessReplayCursorRef: "network.remote-delivery.cross-process-replay-cursor.10r", ExternalCrossProcessTransportRef: "network.remote-delivery.external-cross-process-transport.10t", ExternalCrossProcessTransportEnvelopeRef: "network.remote-delivery.external-cross-process-transport-envelope.10t", ExternalCrossProcessTransportAckRef: "network.remote-delivery.external-cross-process-transport-ack.10t" } as const; export const ParentAgentNetworkLiveCaptureStatusRefs = { StatusRef: "network.live-capture.status.13a", Row13StatusRef: "network.live-capture.proof-gate.13", ExecutionStatusRef: "network.live-capture.execution-status.13b", RawStorageStatusRef: "network.live-capture.raw-storage-custody.03a", WindowsProofRef: "network.live-capture.windows-npcap.13", ManualProofRef: "network.live-capture.manual-required.13", LinuxProofRef: "network.live-capture.linux-libpcap.13", MacosProofRef: "network.live-capture.macos-bpf-libpcap.13", InterfaceRef: "network.live-capture.interface.13", DriverRef: "network.live-capture.driver-proof.13", PermissionRef: "network.live-capture.permission-proof.13", BoundedCaptureRef: "network.live-capture.bounded-capture.13", CleanStopRef: "network.live-capture.clean-stop.13", QuotaRef: "network.live-capture.quota-rotation.13", RetentionRef: "network.live-capture.retention-delete-export.13", CustodyRef: "network.live-capture.custody.13", PrivateTrafficExclusionRef: "network.live-capture.private-traffic-exclusion.13", WindowsExecutionRef: "network.live-capture.execution.windows-npcap.13b", ManualExecutionRef: "network.live-capture.execution.manual-required.13b", LinuxExecutionRef: "network.live-capture.execution.linux-libpcap.13b", MacosExecutionRef: "network.live-capture.execution.macos-bpf-libpcap.13b", DriverInvocationRef: "network.live-capture.driver-invocation.13b", InterfaceObservationRef: "network.live-capture.interface-observation.13b", ExecutionPermissionRef: "network.live-capture.permission.13b", BoundedWindowRef: "network.live-capture.bounded-window.13b", ExecutionCleanStopRef: "network.live-capture.clean-stop.13b", ExecutionCustodyRef: "network.live-capture.custody.13b", ExecutionRetentionRef: "network.live-capture.retention-delete-export.13b", MetadataSanitizationRef: "network.live-capture.metadata-sanitization.13b", ExecutionPrivateTrafficExclusionRef: "network.live-capture.private-traffic-exclusion.13b", RawManifestRef: "network.raw-capture.manifest.03a", RawStorageLocationRef: "network.raw-capture.storage-location.03a", RawEncryptionRef: "network.raw-capture.encryption-at-rest.03a", RawQuotaRef: "network.raw-capture.quota-rotation.03a", RawRetentionRef: "network.raw-capture.retention-policy.03a", RawDeleteExportRef: "network.raw-capture.delete-export.03a", RawCustodyChainRef: "network.raw-capture.custody-chain.03a", RawPrivateTrafficExclusionRef: "network.raw-capture.private-traffic-exclusion.03a" } as const; export const ParentAgentNetworkLinuxNftablesLabStatusRefs = { StatusRef: "network.linux-nftables.lab-status.42a", LabRef: "network.linux-nftables.lab-execution.42a", LinuxAdapterGateRef: "network.linux-adapter.gate.42a", PolicyDecisionRef: "network.policy-decision.linux.42a", ParentRuleRef: "network.parent-rule.linux.42a", EvidenceRef: "network.evidence.linux.42a", DistroRef: "network.linux.distro.42a", KernelRef: "network.linux.kernel.42a", TableName: "ocentra_parent_lab_row42a", ChainName: "ocentra_parent_lab_chain_row42a", TargetRemoteAddress: "203.0.113.253", CreateTableCommandRef: "network.linux-nftables.command.create-table.42a", CreateChainCommandRef: "network.linux-nftables.command.create-chain.42a", AddRuleCommandRef: "network.linux-nftables.command.add-rule.42a", VerifyRuleCommandRef: "network.linux-nftables.command.verify-rule-present.42a", DeleteTableCommandRef: "network.linux-nftables.command.delete-table.42a", VerifyRemovedCommandRef: "network.linux-nftables.command.verify-table-removed.42a", CreateTableOutputSha256: "sha256:network-linux-nftables-create-table-42a", CreateChainOutputSha256: "sha256:network-linux-nftables-create-chain-42a", AddRuleOutputSha256: "sha256:network-linux-nftables-add-rule-42a", VerifyRuleOutputSha256: "sha256:network-linux-nftables-verify-rule-present-42a", DeleteTableOutputSha256: "sha256:network-linux-nftables-delete-table-42a", VerifyRemovedOutputSha256: "sha256:network-linux-nftables-verify-table-removed-42a" } as const; export const ParentAgentNetworkWindowsFirewallLabStatusRefs = { StatusRef: "network.windows-firewall.lab-status.38a", LabRef: "network.windows-firewall.lab-execution.38a", FirewallAdapterPlanRef: "network.windows-firewall.adapter-plan.38a", PolicyDecisionRef: "network.policy-decision.windows-firewall.38a", ParentRuleRef: "network.parent-rule.windows-firewall.38a", EvidenceRef: "network.evidence.windows-firewall.38a", WindowsOsScopeRef: "network.windows-firewall.os-scope.38a", TargetRef: "network.windows-firewall.target.remote-address.38a", FirewallRuleRef: "network.windows-firewall.rule.38a", RuleName: "OcentraParentNetworkLab-row38a", TargetRemoteAddress: "203.0.113.254", ApplyRuleCommandRef: "network.windows-firewall.command.apply-rule.38a", VerifyPresentCommandRef: "network.windows-firewall.command.verify-rule-present.38a", RollbackRuleCommandRef: "network.windows-firewall.command.rollback-rule.38a", VerifyRemovedCommandRef: "network.windows-firewall.command.verify-rule-removed.38a", ApplyRuleOutputSha256: "sha256:network-windows-firewall-apply-rule-38a", VerifyPresentOutputSha256: "sha256:network-windows-firewall-verify-rule-present-38a", RollbackRuleOutputSha256: "sha256:network-windows-firewall-rollback-rule-38a", VerifyRemovedOutputSha256: "sha256:network-windows-firewall-verify-rule-removed-38a" } as const; export const ParentAgentNetworkWindowsWfpGateStatusRefs = { StatusRef: "network.windows-wfp.gate-status.39", WfpGateRef: "network.windows-wfp.gate.39", PolicyDecisionRef: "network.policy-decision.windows-wfp.39", ParentRuleRef: "network.parent-rule.windows-wfp.39", EvidenceRef: "network.evidence.windows-wfp.39", LocalAiResultRef: "network.local-ai.windows-wfp.39", TargetRef: "network.windows-wfp.target.39", WfpProviderRef: "network.windows-wfp.provider.39", WfpLayerRef: "network.windows-wfp.layer.39", AdministratorPermissionProofRef: "network.windows-wfp.admin-permission-proof.39", DriverSigningProofRef: "network.windows-wfp.driver-signing-proof.39", DriverPackageProofRef: "network.windows-wfp.driver-package-proof.39", ProviderRegistrationPlanRef: "network.windows-wfp.provider-registration-plan.39", LayerCapabilityMatrixRef: "network.windows-wfp.layer-capability-matrix.39", RollbackPlanRef: "network.windows-wfp.rollback-plan.39", LabResultArtifactRef: "network.windows-wfp.lab-result-artifact.39", AuditEventRef: "network.windows-wfp.audit-event.39" } as const; export const ParentAgentNetworkAndroidVpnServiceGateStatusRefs = { StatusRef: "network.android-vpn-service.gate-status.40", AndroidVpnServiceGateRef: "network.android-vpn-service.gate.40", PolicyDecisionRef: "network.policy-decision.android-vpn-service.40", ParentRuleRef: "network.parent-rule.android-vpn-service.40", EvidenceRef: "network.evidence.android-vpn-service.40", LocalAiResultRef: "network.local-ai.android-vpn-service.40", PackageRef: "network.android-vpn-service.package.40", VpnServiceRef: "network.android-vpn-service.service.40", VpnServiceDeclarationRef: "network.android-vpn-service.declaration.40", UserConsentProofRef: "network.android-vpn-service.user-consent-proof.40", PhysicalDeviceProofRef: "network.android-vpn-service.physical-device-proof.40", PackageIdentityProofRef: "network.android-vpn-service.package-identity-proof.40", VirtualInterfaceProofRef: "network.android-vpn-service.virtual-interface-proof.40", TrafficObservationProofRef: "network.android-vpn-service.traffic-observation-proof.40", RollbackPlanRef: "network.android-vpn-service.rollback-plan.40", AuditEventRef: "network.android-vpn-service.audit-event.40", DeviceOwnerProofRef: "network.android-vpn-service.device-owner-proof.40" } as const; export const ParentAgentNetworkAppleNetworkExtensionGateStatusRefs = { StatusRef: "network.apple-network-extension.gate-status.41", AppleNetworkExtensionGateRef: "network.apple-network-extension.gate.41", PolicyDecisionRef: "network.policy-decision.apple-network-extension.41", ParentRuleRef: "network.parent-rule.apple-network-extension.41", EvidenceRef: "network.evidence.apple-network-extension.41", LocalAiResultRef: "network.local-ai.apple-network-extension.41", BundleRef: "network.apple-network-extension.bundle.41", NetworkExtensionRef: "network.apple-network-extension.extension.41", DeveloperTeamProofRef: "network.apple-network-extension.developer-team-proof.41", EntitlementApprovalProofRef: "network.apple-network-extension.entitlement-approval-proof.41", ProvisioningProfileProofRef: "network.apple-network-extension.provisioning-profile-proof.41", SigningProofRef: "network.apple-network-extension.signing-proof.41", DeviceOrTestFlightProofRef: "network.apple-network-extension.device-or-testflight-proof.41", NetworkExtensionDeclarationRef: "network.apple-network-extension.declaration.41", ExtensionConfigurationProofRef: "network.apple-network-extension.configuration-proof.41", RollbackPlanRef: "network.apple-network-extension.rollback-plan.41", AuditEventRef: "network.apple-network-extension.audit-event.41", SupervisionOrMdmProofRef: "network.apple-network-extension.supervision-or-mdm-proof.41" } as const; export const ParentAgentNetworkRemoteDeliveryStatusState = { FixtureRequirementsRecordedButNotImplemented: "fixture-requirements-recorded-but-not-implemented", ManualRequired: "manual-required" } as const; export type ParentAgentNetworkRemoteDeliveryStatusState = (typeof ParentAgentNetworkRemoteDeliveryStatusState)[keyof typeof ParentAgentNetworkRemoteDeliveryStatusState]; export const ParentAgentNetworkRemoteDeliveryTransportDispatchState = { ManualRequiredBlocked: "manual-required-blocked" } as const; export type ParentAgentNetworkRemoteDeliveryTransportDispatchState = (typeof ParentAgentNetworkRemoteDeliveryTransportDispatchState)[keyof typeof ParentAgentNetworkRemoteDeliveryTransportDispatchState]; export const ParentAgentNetworkRemoteDeliveryProviderChildReadinessState = { ManualRequiredUnavailable: "manual-required-unavailable" } as const; export type ParentAgentNetworkRemoteDeliveryProviderChildReadinessState = (typeof ParentAgentNetworkRemoteDeliveryProviderChildReadinessState)[keyof typeof ParentAgentNetworkRemoteDeliveryProviderChildReadinessState]; export const ParentAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState = { ManualRequiredUnavailable: "manual-required-unavailable" } as const; export type ParentAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState = (typeof ParentAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState)[keyof typeof ParentAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState]; export const ParentAgentNetworkRemoteDeliveryExternalCrossProcessTransportState = { DeterministicEnvelopeAckRecorded: "deterministic-envelope-ack-recorded" } as const; export type ParentAgentNetworkRemoteDeliveryExternalCrossProcessTransportState = (typeof ParentAgentNetworkRemoteDeliveryExternalCrossProcessTransportState)[keyof typeof ParentAgentNetworkRemoteDeliveryExternalCrossProcessTransportState]; export const ParentAgentNetworkLiveCapturePlatform = { WindowsNpcap: "windows-npcap", LinuxLibpcap: "linux-libpcap", MacosBpfLibpcap: "macos-bpf-libpcap" } as const; export type ParentAgentNetworkLiveCapturePlatform = (typeof ParentAgentNetworkLiveCapturePlatform)[keyof typeof ParentAgentNetworkLiveCapturePlatform]; export const ParentAgentNetworkLiveCaptureProofState = { ProofReady: "proof-ready", ManualRequired: "manual-required", Unavailable: "unavailable", Degraded: "degraded" } as const; export type ParentAgentNetworkLiveCaptureProofState = (typeof ParentAgentNetworkLiveCaptureProofState)[keyof typeof ParentAgentNetworkLiveCaptureProofState]; export const ParentAgentNetworkRawCaptureStorageState = { CustodyReady: "custody-ready", ManualRequired: "manual-required", Unavailable: "unavailable", Degraded: "degraded" } as const; export type ParentAgentNetworkRawCaptureStorageState = (typeof ParentAgentNetworkRawCaptureStorageState)[keyof typeof ParentAgentNetworkRawCaptureStorageState]; export const ParentAgentNetworkLiveCaptureExecutionState = { ManualRequired: "manual-required", BoundedExecuted: "bounded-executed", Unavailable: "unavailable", Degraded: "degraded" } as const; export type ParentAgentNetworkLiveCaptureExecutionState = (typeof ParentAgentNetworkLiveCaptureExecutionState)[keyof typeof ParentAgentNetworkLiveCaptureExecutionState]; export const ParentAgentNetworkLinuxNftablesLabState = { ManualRequired: "manual-required", ExecutedAndRolledBack: "executed-and-rolled-back", Unavailable: "unavailable" } as const; export type ParentAgentNetworkLinuxNftablesLabState = (typeof ParentAgentNetworkLinuxNftablesLabState)[keyof typeof ParentAgentNetworkLinuxNftablesLabState]; export const ParentAgentNetworkLinuxNftablesLabCommandKind = { CreateTable: "create-table", CreateChain: "create-chain", AddRule: "add-rule", VerifyRulePresent: "verify-rule-present", DeleteTable: "delete-table", VerifyTableRemoved: "verify-table-removed" } as const; export type ParentAgentNetworkLinuxNftablesLabCommandKind = (typeof ParentAgentNetworkLinuxNftablesLabCommandKind)[keyof typeof ParentAgentNetworkLinuxNftablesLabCommandKind]; export const ParentAgentNetworkWindowsFirewallLabState = { ManualRequired: "manual-required", ExecutedAndRolledBack: "executed-and-rolled-back", Unavailable: "unavailable" } as const; export type ParentAgentNetworkWindowsFirewallLabState = (typeof ParentAgentNetworkWindowsFirewallLabState)[keyof typeof ParentAgentNetworkWindowsFirewallLabState]; export const ParentAgentNetworkWindowsFirewallLabCommandKind = { ApplyRule: "apply-rule", VerifyRulePresent: "verify-rule-present", RollbackRule: "rollback-rule", VerifyRuleRemoved: "verify-rule-removed" } as const; export type ParentAgentNetworkWindowsFirewallLabCommandKind = (typeof ParentAgentNetworkWindowsFirewallLabCommandKind)[keyof typeof ParentAgentNetworkWindowsFirewallLabCommandKind]; export const ParentAgentNetworkWindowsWfpGateState = { ManualRequired: "manual-required", ResearchOnly: "research-only", Unavailable: "unavailable", LabProofReady: "lab-proof-ready" } as const; export type ParentAgentNetworkWindowsWfpGateState = (typeof ParentAgentNetworkWindowsWfpGateState)[keyof typeof ParentAgentNetworkWindowsWfpGateState]; export const ParentAgentNetworkWindowsWfpCapabilityState = { ManualRequired: "manual-required", LabReady: "lab-ready", Unavailable: "unavailable" } as const; export type ParentAgentNetworkWindowsWfpCapabilityState = (typeof ParentAgentNetworkWindowsWfpCapabilityState)[keyof typeof ParentAgentNetworkWindowsWfpCapabilityState]; export const ParentAgentNetworkAndroidVpnServiceGateState = { ManualRequired: "manual-required", ResearchOnly: "research-only", Unavailable: "unavailable", PhysicalDeviceProofReady: "physical-device-proof-ready" } as const; export type ParentAgentNetworkAndroidVpnServiceGateState = (typeof ParentAgentNetworkAndroidVpnServiceGateState)[keyof typeof ParentAgentNetworkAndroidVpnServiceGateState]; export const ParentAgentNetworkAndroidVpnServiceCapabilityState = { PhysicalDeviceReady: "physical-device-ready", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type ParentAgentNetworkAndroidVpnServiceCapabilityState = (typeof ParentAgentNetworkAndroidVpnServiceCapabilityState)[keyof typeof ParentAgentNetworkAndroidVpnServiceCapabilityState]; export const ParentAgentNetworkAndroidVpnServiceRequiredArtifact = { VpnServiceDeclaration: "vpn-service-declaration", UserConsentProof: "user-consent-proof", PhysicalDeviceProof: "physical-device-proof", PackageIdentityProof: "package-identity-proof", VirtualInterfaceProof: "virtual-interface-proof", TrafficObservationProof: "traffic-observation-proof", RollbackPlan: "rollback-plan", AuditEvent: "audit-event", DeviceOwnerProof: "device-owner-proof" } as const; export type ParentAgentNetworkAndroidVpnServiceRequiredArtifact = (typeof ParentAgentNetworkAndroidVpnServiceRequiredArtifact)[keyof typeof ParentAgentNetworkAndroidVpnServiceRequiredArtifact]; export const ParentAgentNetworkAndroidVpnServiceBoundaryReason = { ResearchOnlyRequested: "research-only-requested", CapabilityManualRequired: "capability-manual-required", CapabilityUnavailable: "capability-unavailable", EvidenceGradeBelowProofThreshold: "evidence-grade-below-proof-threshold", PolicyNotVpnServiceApproved: "policy-not-vpn-service-approved", MissingRequiredArtifact: "missing-required-artifact" } as const; export type ParentAgentNetworkAndroidVpnServiceBoundaryReason = (typeof ParentAgentNetworkAndroidVpnServiceBoundaryReason)[keyof typeof ParentAgentNetworkAndroidVpnServiceBoundaryReason]; export const ParentAgentNetworkAppleNetworkExtensionPlatform = { MacOs: "mac-os", Ios: "ios" } as const; export type ParentAgentNetworkAppleNetworkExtensionPlatform = (typeof ParentAgentNetworkAppleNetworkExtensionPlatform)[keyof typeof ParentAgentNetworkAppleNetworkExtensionPlatform]; export const ParentAgentNetworkAppleNetworkExtensionCapabilityState = { AppleDeviceReady: "apple-device-ready", ManualRequired: "manual-required", Unavailable: "unavailable" } as const; export type ParentAgentNetworkAppleNetworkExtensionCapabilityState = (typeof ParentAgentNetworkAppleNetworkExtensionCapabilityState)[keyof typeof ParentAgentNetworkAppleNetworkExtensionCapabilityState]; export const ParentAgentNetworkAppleNetworkExtensionGateState = { ResearchOnly: "research-only", ManualRequired: "manual-required", Unavailable: "unavailable", AppleEntitlementProofReady: "apple-entitlement-proof-ready" } as const; export type ParentAgentNetworkAppleNetworkExtensionGateState = (typeof ParentAgentNetworkAppleNetworkExtensionGateState)[keyof typeof ParentAgentNetworkAppleNetworkExtensionGateState]; export const ParentAgentNetworkAppleNetworkExtensionRequiredArtifact = { DeveloperTeamProof: "developer-team-proof", EntitlementApprovalProof: "entitlement-approval-proof", ProvisioningProfileProof: "provisioning-profile-proof", SigningProof: "signing-proof", DeviceOrTestflightProof: "device-or-testflight-proof", NetworkExtensionDeclaration: "network-extension-declaration", ExtensionConfigurationProof: "extension-configuration-proof", RollbackPlan: "rollback-plan", AuditEvent: "audit-event", SupervisionOrMdmProof: "supervision-or-mdm-proof" } as const; export type ParentAgentNetworkAppleNetworkExtensionRequiredArtifact = (typeof ParentAgentNetworkAppleNetworkExtensionRequiredArtifact)[keyof typeof ParentAgentNetworkAppleNetworkExtensionRequiredArtifact]; export const ParentAgentNetworkAppleNetworkExtensionBoundaryReason = { ResearchOnlyRequested: "research-only-requested", CapabilityManualRequired: "capability-manual-required", CapabilityUnavailable: "capability-unavailable", EvidenceGradeBelowProofThreshold: "evidence-grade-below-proof-threshold", PolicyNotNetworkExtensionApproved: "policy-not-network-extension-approved", MissingRequiredArtifact: "missing-required-artifact" } as const; export type ParentAgentNetworkAppleNetworkExtensionBoundaryReason = (typeof ParentAgentNetworkAppleNetworkExtensionBoundaryReason)[keyof typeof ParentAgentNetworkAppleNetworkExtensionBoundaryReason];
export type ParentAgentNetworkRemoteDeliveryStatus = Readonly<Record<string, unknown>>;
export type ParentAgentNetworkLiveCaptureStatusRow = Readonly<Record<string, unknown>>;
export type ParentAgentNetworkLiveCaptureStatus = Readonly<Record<string, unknown>> & { readonly rows: readonly ParentAgentNetworkLiveCaptureStatusRow[] };
export type ParentAgentNetworkLinuxNftablesLabCommandRow = Readonly<Record<string, unknown>> & { readonly kind: ParentAgentNetworkLinuxNftablesLabCommandKind };
export type ParentAgentNetworkLinuxNftablesLabStatus = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly ParentAgentNetworkLinuxNftablesLabCommandRow[] };
export type ParentAgentNetworkWindowsFirewallLabCommandRow = Readonly<Record<string, unknown>> & { readonly kind: ParentAgentNetworkWindowsFirewallLabCommandKind };
export type ParentAgentNetworkWindowsFirewallLabStatus = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly ParentAgentNetworkWindowsFirewallLabCommandRow[] };
export type ParentAgentNetworkWindowsWfpGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly string[]; readonly missingRequiredArtifacts: readonly string[]; readonly wfpLabProofReady: boolean; readonly enforcementCommandPublished: false };
export type ParentAgentNetworkAndroidVpnServiceGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly ParentAgentNetworkAndroidVpnServiceBoundaryReason[]; readonly missingRequiredArtifacts: readonly ParentAgentNetworkAndroidVpnServiceRequiredArtifact[]; readonly gateState: ParentAgentNetworkAndroidVpnServiceGateState; readonly physicalDeviceProofReady: boolean; readonly enforcementCommandPublished: false };
export type ParentAgentNetworkAppleNetworkExtensionGateStatus = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly ParentAgentNetworkAppleNetworkExtensionBoundaryReason[]; readonly missingRequiredArtifacts: readonly ParentAgentNetworkAppleNetworkExtensionRequiredArtifact[]; readonly platform: ParentAgentNetworkAppleNetworkExtensionPlatform; readonly gateState: ParentAgentNetworkAppleNetworkExtensionGateState; readonly appleEntitlementProofReady: boolean; readonly enforcementCommandPublished: false };
const __ParentAgentNetworkStatusRemoteStringFields = ['statusRef','custodyProofRef','publisherAuthRef','subscriberAuthRef','encryptionRef','retentionPolicyRef','replayPlanRef','deletionPlanRef','offsetPolicyRef','dedupePolicyRef','transportConfigRef','relayIdentityRef','relayPolicyRef','eventChainJournalRef','receiptLedgerRef','localReceiptAckRef','durableEnvelopeRef','durableStoreRef','durableReplayRef','durableDeleteExportRef','durableSupportStatusRef','outboxRef','outboxHandoffRef','outboxReplayRef','outboxSupportStatusRef','transportDispatchStateRef','blockedDispatchRef','futureTransportSeamRef','fixtureTransportRef','fixtureDispatchAttemptRef','fixtureAckRef','deleteExportPropagationRef','remoteDeleteReadinessRef','remoteExportReadinessRef','providerRouteRef','childDeviceRouteRef','providerDeliveryReadinessRef','childDeviceDeliveryReadinessRef','crossProcessCustodyStatusRef','crossProcessReplayReadinessRef','remoteRetentionReadinessRef','remoteDeleteCustodyReadinessRef','remoteExportCustodyReadinessRef','crossProcessReplayRef','crossProcessReplayStoreRef','crossProcessReplayCursorRef','externalCrossProcessTransportRef','externalCrossProcessTransportEnvelopeRef','externalCrossProcessTransportAckRef'] as const;
const __ParentAgentNetworkStatusRemoteCountFields = ['brokerMissingArtifactCount','familyHubMissingArtifactCount','acceptedEventTypeCount','droppedEventDeadLetterCount','durableEnvelopeMissingArtifactCount','outboxCandidateCount','sourceOutboxCandidateCount','preparedNotDispatchedCount','blockedDispatchRecordCount','fixtureSourceOutboxCandidateCount','fixtureDispatchAttemptCount','fixtureRemoteAckCount','deleteExportReadinessRecordCount','remoteDeleteReadyCount','remoteExportReadyCount','providerDeliveryReadinessRecordCount','childDeviceDeliveryReadinessRecordCount','crossProcessReplayReadinessRecordCount','remoteRetentionReadinessRecordCount','remoteDeleteCustodyReadinessRecordCount','remoteExportCustodyReadinessRecordCount','crossProcessReplayRecordCount','crossProcessReplayStoreWriteCount','crossProcessReplayCursorNextSequence','externalCrossProcessTransportRecordCount','externalCrossProcessTransportEnvelopeCount','externalCrossProcessTransportAckCount'] as const;
const __ParentAgentNetworkStatusRemoteZeroFields = ['providerDeliveryArtifactCount','childDeviceDeliveryArtifactCount','crossProcessReplayArtifactCount','remoteRetentionArtifactCount','remoteDeleteCustodyArtifactCount','remoteExportCustodyArtifactCount','dispatchReadyCandidateCount','dispatchAttemptCount','remoteAckCount','sequenceGapCount','eventIdMismatchCount','eventTypeMismatchCount','correlationMismatchCount','enforcementCommandEventCount','adapterActionExecutedCount','rawPcapAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','videoContentAvailableCount','privateMessageContentAvailableCount','searchQueryAvailableCount'] as const;
const __ParentAgentNetworkStatusRemoteBooleanFields = ['localIdempotencyQueueProved','queuedDuplicateRejected','completedDuplicateRejected','durableEnvelopeReady','blockedDispatchRecordsMatchOutboxCandidates','fixtureRecordsMatchOutboxCandidates','deleteExportRecordsMatchFixtureAcks','providerDeliveryRecordsMatchFixtureAcks','childDeviceDeliveryRecordsMatchFixtureAcks','crossProcessCustodyRecordsMatchProviderChildReadiness','crossProcessReplayRecordsMatchDurableEnvelopes','crossProcessReplayRecordsMatchCustodyReadiness','externalCrossProcessTransportRecordsMatchReplayRecords','externalCrossProcessTransportAckRecordsMatchEnvelopes','duplicateDurableEnvelopeRejected','outboxCandidatesMatchDurableEnvelopes','outboxCandidatesMatchReceipts'] as const;
const __ParentAgentNetworkStatusRemoteFalseFields = ['brokerDeliveryImplemented','familyHubDeliveryImplemented','remoteDeliveryAckImplemented','providerDeliveryImplemented','childDeviceDeliveryImplemented','remoteDeleteExportPropagationImplemented','productReadyRemoteDelivery','policyAuthority','sideEffectAuthority','hostFilteringClaimed'] as const;
const __ParentAgentNetworkStatusRemoteTrueFields = ['crossProcessReplayImplemented','externalCrossProcessTransportImplemented'] as const;
const __ParentAgentNetworkStatusLiveStatusStringFields = ['statusRef','row13StatusRef','executionStatusRef','rawStorageStatusRef'] as const;
const __ParentAgentNetworkStatusLiveStatusCountFields = ['platformRowCount','proofReadyCount','manualRequiredCount','unavailableCount','degradedCount','requiredArtifactCount','missingArtifactCount','storageCustodyReadyCount','storageManualRequiredCount','storageUnavailableCount','storageDegradedCount','storageMissingArtifactCount','boundedExecutedCount','executionManualRequiredCount','executionUnavailableCount','executionDegradedCount','executionMissingArtifactCount','metadataSnapshotExecutedCount','capturedPacketCount','captureReadyCount','rawArtifactStorageAuthorizedCount','driverInvokedCount','liveCaptureExecutedCount'] as const;
const __ParentAgentNetworkStatusLiveStatusZeroFields = ['rawArtifactCreatedCount','remoteUploadEnabledCount','rawPcapWithoutCustodyAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','privateMessageAvailableCount','searchQueryAvailableCount','policyAuthorityCount','adapterAuthorityCount','enforcementCommandEventCount','netstatMetadataSubstitutionCount','hostFilteringClaimCount'] as const;
const __ParentAgentNetworkStatusLiveRowStringFields = ['captureProofRef','storageProofRef'] as const;
const __ParentAgentNetworkStatusLiveRowNullableStringFields = ['interfaceRef','driverProofRef','permissionProofRef','boundedCaptureRef','cleanStopRef','quotaRotationRef','retentionDeleteExportRef','custodyRef','privateTrafficExclusionRef','rawArtifactManifestRef','storageLocationRef','encryptionAtRestRef','storageQuotaRotationRef','retentionPolicyRef','storageDeleteExportRef','custodyChainRef','storagePrivateTrafficExclusionRef','executionRef','driverInvocationRef','interfaceObservationRef','executionPermissionRef','boundedWindowRef','executionCleanStopRef','executionCustodyRef','executionRetentionDeleteExportRef','metadataOnlySanitizationRef','executionPrivateTrafficExclusionRef'] as const;
const __ParentAgentNetworkStatusLiveRowCountFields = ['executionMissingArtifactCount','capturedPacketCount','missingArtifactCount','storageMissingArtifactCount'] as const;
const __ParentAgentNetworkStatusLiveRowBooleanFields = ['metadataSnapshotExecuted','captureReady','rawArtifactStorageAuthorized','driverInvoked','liveCaptureExecuted'] as const;
const __ParentAgentNetworkStatusLiveRowFalseFields = ['rawArtifactCreated','remoteUploadEnabled','rawPcapWithoutCustodyAvailable','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','privateMessageAvailable','searchQueryAvailable','policyAuthority','adapterAuthority','netstatMetadataSubstitutedForLiveCapture','hostFilteringClaimed'] as const;
const __ParentAgentNetworkStatusLiveRowZeroFields = ['enforcementCommandsPublished'] as const;
const __ParentAgentNetworkStatusLinuxNftablesStringFields = ['statusRef','labRef','linuxAdapterGateRef','policyDecisionRef','parentRuleRef','distroRef','kernelRef','tableName','chainName','targetRemoteAddress'] as const;
const __ParentAgentNetworkStatusLinuxNftablesBooleanFields = ['wslHostObserved','rootPermissionObserved','nftToolObserved','tableCreateObserved','chainCreateObserved','ruleAddObserved','verifyPresentObserved','rollbackObserved','verifyRemovedObserved','labPacketFilterRuleExecuted','rollbackVerified'] as const;
const __ParentAgentNetworkStatusLinuxNftablesFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','genericLinuxSupportClaimed','serviceManagerInstallClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __ParentAgentNetworkStatusLinuxNftablesObservedFlags = [{ field: 'tableCreateObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.CreateTable },{ field: 'chainCreateObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.CreateChain },{ field: 'ruleAddObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.AddRule },{ field: 'verifyPresentObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.VerifyRulePresent },{ field: 'rollbackObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.DeleteTable },{ field: 'verifyRemovedObserved', kind: ParentAgentNetworkLinuxNftablesLabCommandKind.VerifyTableRemoved }] as const;
const __ParentAgentNetworkStatusLinuxNftablesExpectedOutcomes = [{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.CreateTable, table: true, chain: false, rule: false },{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.CreateChain, table: true, chain: true, rule: false },{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.AddRule, table: true, chain: true, rule: true },{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.VerifyRulePresent, table: true, chain: true, rule: true },{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.DeleteTable, table: false, chain: false, rule: false },{ kind: ParentAgentNetworkLinuxNftablesLabCommandKind.VerifyTableRemoved, table: false, chain: false, rule: false }] as const;
const __ParentAgentNetworkStatusWindowsFirewallStringFields = ['statusRef','labRef','firewallAdapterPlanRef','policyDecisionRef','parentRuleRef','windowsOsScopeRef','targetRef','firewallRuleRef','ruleName','targetRemoteAddress'] as const;
const __ParentAgentNetworkStatusWindowsFirewallBooleanFields = ['windowsHostObserved','administratorPermissionObserved','applyCommandObserved','verifyPresentObserved','rollbackCommandObserved','verifyRemovedObserved','labFirewallMutationExecuted','rollbackVerified','adapterApplyAuthorized'] as const;
const __ParentAgentNetworkStatusWindowsFirewallFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','hostFirewallMutationClaimed','netshCommandInvoked','powershellCommandInvoked','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __ParentAgentNetworkStatusWindowsFirewallObservedFlags = [{ field: 'applyCommandObserved', kind: ParentAgentNetworkWindowsFirewallLabCommandKind.ApplyRule },{ field: 'verifyPresentObserved', kind: ParentAgentNetworkWindowsFirewallLabCommandKind.VerifyRulePresent },{ field: 'rollbackCommandObserved', kind: ParentAgentNetworkWindowsFirewallLabCommandKind.RollbackRule },{ field: 'verifyRemovedObserved', kind: ParentAgentNetworkWindowsFirewallLabCommandKind.VerifyRuleRemoved }] as const;
const __ParentAgentNetworkStatusWindowsFirewallExpectedOutcomes = [{ kind: ParentAgentNetworkWindowsFirewallLabCommandKind.ApplyRule, rulePresentAfterCommand: true },{ kind: ParentAgentNetworkWindowsFirewallLabCommandKind.VerifyRulePresent, rulePresentAfterCommand: true },{ kind: ParentAgentNetworkWindowsFirewallLabCommandKind.RollbackRule, rulePresentAfterCommand: false },{ kind: ParentAgentNetworkWindowsFirewallLabCommandKind.VerifyRuleRemoved, rulePresentAfterCommand: false }] as const;
const __ParentAgentNetworkStatusWindowsWfpStringFields = ['statusRef','wfpGateRef','policyDecisionRef','parentRuleRef','targetRef','wfpProviderRef','wfpLayerRef'] as const;
const __ParentAgentNetworkStatusWindowsWfpNullableStringFields = ['localAiResultRef','administratorPermissionProofRef','driverSigningProofRef','driverPackageProofRef','providerRegistrationPlanRef','layerCapabilityMatrixRef','rollbackPlanRef','labResultArtifactRef','auditEventRef'] as const;
const __ParentAgentNetworkStatusWindowsWfpStringArrayFields = ['evidenceRefs','boundaryReasons','missingRequiredArtifacts'] as const;
const __ParentAgentNetworkStatusWindowsWfpFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','liveDriverInstallClaimed','calloutRegistrationClaimed','packetBlockClaimed','kernelPayloadInspectionClaimed','commandInvocationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __ParentAgentNetworkStatusAndroidVpnStringFields = ['statusRef','androidVpnServiceGateRef','policyDecisionRef','parentRuleRef','packageRef','vpnServiceRef'] as const;
const __ParentAgentNetworkStatusAndroidVpnNullableStringFields = ['localAiResultRef','vpnServiceDeclarationRef','userConsentProofRef','physicalDeviceProofRef','packageIdentityProofRef','virtualInterfaceProofRef','trafficObservationProofRef','rollbackPlanRef','auditEventRef','deviceOwnerProofRef'] as const;
const __ParentAgentNetworkStatusAndroidVpnBooleanFields = ['deviceOwnerRequired','physicalDeviceProofReady','deviceOwnerAuthorityProved'] as const;
const __ParentAgentNetworkStatusAndroidVpnFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','emulatorOnlyProductSupportClaimed','liveVpnTunnelClaimed','packetBlockClaimed','appPackageCorrelationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __ParentAgentNetworkStatusAppleNetworkExtensionStringFields = ['statusRef','appleNetworkExtensionGateRef','policyDecisionRef','parentRuleRef','bundleRef','networkExtensionRef'] as const;
const __ParentAgentNetworkStatusAppleNetworkExtensionNullableStringFields = ['localAiResultRef','developerTeamProofRef','entitlementApprovalProofRef','provisioningProfileProofRef','signingProofRef','deviceOrTestFlightProofRef','networkExtensionDeclarationRef','extensionConfigurationProofRef','rollbackPlanRef','auditEventRef','supervisionOrMdmProofRef'] as const;
const __ParentAgentNetworkStatusAppleNetworkExtensionBooleanFields = ['supervisionRequired','appleEntitlementProofReady','supervisionAuthorityProved'] as const;
const __ParentAgentNetworkStatusAppleNetworkExtensionFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','simulatorOnlyProductSupportClaimed','liveNetworkExtensionClaimed','packetBlockClaimed','appLevelControlClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
function __ParentAgentNetworkStatusIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __ParentAgentNetworkStatusReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__ParentAgentNetworkStatusIsRecord(value)) { throw new TypeError(`${label} must be a network status object`); } return value; }
function __ParentAgentNetworkStatusReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string`); } return value; }
function __ParentAgentNetworkStatusReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string or null`); } return value; }
function __ParentAgentNetworkStatusReadCount(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative integer`); } return value; }
function __ParentAgentNetworkStatusReadInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value)) { throw new TypeError(`${field} must be an integer`); } return value; }
function __ParentAgentNetworkStatusReadRequiredCount(record: Readonly<Record<string, unknown>>, field: string, expected: number): number { const value = __ParentAgentNetworkStatusReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __ParentAgentNetworkStatusReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network status boolean`); } return value; }
function __ParentAgentNetworkStatusReadRequiredBoolean(record: Readonly<Record<string, unknown>>, field: string, expected: boolean): boolean { const value = __ParentAgentNetworkStatusReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __ParentAgentNetworkStatusReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __ParentAgentNetworkStatusReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network status literal`); } return value as T; }
function __ParentAgentNetworkStatusReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a network status string array`); } values.forEach((value) => { if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} entries must be non-empty network status strings`); } }); return values; }
function __ParentAgentNetworkStatusReadLiteralArray<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): readonly T[] { return __ParentAgentNetworkStatusReadStringArray(record, field).map((value) => { if (!allowed.includes(value as T)) { throw new TypeError(`${field} entries must be Rust-owned network status literals`); } return value as T; }); }
function __ParentAgentNetworkStatusReadRecordArray(record: Readonly<Record<string, unknown>>, field: string, label: string): readonly Readonly<Record<string, unknown>>[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a ${label} array`); } return values.map((value) => __ParentAgentNetworkStatusReadRecord(value, label)); }
function __ParentAgentNetworkStatusRequireCountMatches(record: Readonly<Record<string, unknown>>, field: string, expected: number): void { const value = __ParentAgentNetworkStatusReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must match command evidence length`); } }
function __ParentAgentNetworkStatusRequireUniqueRowsByKind<T extends Readonly<Record<string, unknown>> & { readonly kind: string }>(rows: readonly T[], label: string): ReadonlyMap<string, T> { const byKind = new Map(rows.map((row) => [row.kind, row] as const)); if (byKind.size !== rows.length) { throw new TypeError(`${label} command evidence must use unique command kinds`); } return byKind; }
function __ParentAgentNetworkStatusRequireObservedFlags(record: Readonly<Record<string, unknown>>, byKind: ReadonlyMap<string, Readonly<Record<string, unknown>>>, flags: readonly { readonly field: string; readonly kind: string }[], label: string): void { flags.forEach(({ field, kind }) => { if (__ParentAgentNetworkStatusReadBoolean(record, field) !== byKind.has(kind)) { throw new TypeError(`${label} observed flags must match command evidence`); } }); }
function __ParentAgentNetworkStatusRequireLinuxNftablesOutcomes(byKind: ReadonlyMap<string, ParentAgentNetworkLinuxNftablesLabCommandRow>): void { __ParentAgentNetworkStatusLinuxNftablesExpectedOutcomes.forEach(({ kind, table, chain, rule }) => { const row = byKind.get(kind); if (row === undefined || row['tablePresentAfterCommand'] !== table || row['chainPresentAfterCommand'] !== chain || row['rulePresentAfterCommand'] !== rule) { throw new TypeError('Linux nftables command evidence must match bounded apply and rollback outcomes'); } }); }
function __ParentAgentNetworkStatusRequireWindowsFirewallOutcomes(byKind: ReadonlyMap<string, ParentAgentNetworkWindowsFirewallLabCommandRow>): void { __ParentAgentNetworkStatusWindowsFirewallExpectedOutcomes.forEach(({ kind, rulePresentAfterCommand }) => { const row = byKind.get(kind); if (row === undefined || row['rulePresentAfterCommand'] !== rulePresentAfterCommand) { throw new TypeError('Windows firewall command evidence must match bounded apply and rollback outcomes'); } }); }
function __ParentAgentNetworkStatusGateProofReadyIsValid(capabilityReady: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityReady && proofReady && boundaryReasons.length === 0 && missingRequiredArtifacts.length === 0; }
function __ParentAgentNetworkStatusGateManualRequiredIsValid(capabilityManualRequired: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityManualRequired || boundaryReasons.length > 0 || missingRequiredArtifacts.length > 0 || !proofReady; }
function __ParentAgentNetworkStatusRequireGateConsistency(label: string, proofReadyGate: boolean, proofReadyValid: boolean, manualRequiredGate: boolean, manualRequiredValid: boolean): void { if (proofReadyGate && !proofReadyValid) { throw new TypeError(`${label} proof-ready status must preserve bounded proof invariants`); } if (manualRequiredGate && !manualRequiredValid) { throw new TypeError(`${label} manual-required status must preserve bounded blockers`); } }
export function decodeParentAgentNetworkRemoteDeliveryStatus(value: unknown): ParentAgentNetworkRemoteDeliveryStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network remote delivery status'); for (const field of __ParentAgentNetworkStatusRemoteStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusRemoteCountFields) { __ParentAgentNetworkStatusReadCount(record, field); } for (const field of __ParentAgentNetworkStatusRemoteZeroFields) { __ParentAgentNetworkStatusReadRequiredCount(record, field, 0); } for (const field of __ParentAgentNetworkStatusRemoteBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } for (const field of __ParentAgentNetworkStatusRemoteFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } for (const field of __ParentAgentNetworkStatusRemoteTrueFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, true); } __ParentAgentNetworkStatusReadLiteral(record, 'brokerStatus', Object.values(ParentAgentNetworkRemoteDeliveryStatusState)); __ParentAgentNetworkStatusReadLiteral(record, 'familyHubStatus', Object.values(ParentAgentNetworkRemoteDeliveryStatusState)); __ParentAgentNetworkStatusReadLiteral(record, 'transportDispatchState', Object.values(ParentAgentNetworkRemoteDeliveryTransportDispatchState)); __ParentAgentNetworkStatusReadLiteral(record, 'providerDeliveryReadinessState', Object.values(ParentAgentNetworkRemoteDeliveryProviderChildReadinessState)); __ParentAgentNetworkStatusReadLiteral(record, 'childDeviceDeliveryReadinessState', Object.values(ParentAgentNetworkRemoteDeliveryProviderChildReadinessState)); __ParentAgentNetworkStatusReadLiteral(record, 'crossProcessCustodyReadinessState', Object.values(ParentAgentNetworkRemoteDeliveryCrossProcessCustodyReadinessState)); __ParentAgentNetworkStatusReadLiteral(record, 'externalCrossProcessTransportState', Object.values(ParentAgentNetworkRemoteDeliveryExternalCrossProcessTransportState)); return record as ParentAgentNetworkRemoteDeliveryStatus; }
export function decodeParentAgentNetworkLiveCaptureStatusRow(value: unknown): ParentAgentNetworkLiveCaptureStatusRow { const record = __ParentAgentNetworkStatusReadRecord(value, 'network live capture status row'); for (const field of __ParentAgentNetworkStatusLiveRowStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusLiveRowNullableStringFields) { __ParentAgentNetworkStatusReadNullableString(record, field); } for (const field of __ParentAgentNetworkStatusLiveRowCountFields) { __ParentAgentNetworkStatusReadCount(record, field); } for (const field of __ParentAgentNetworkStatusLiveRowBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } for (const field of __ParentAgentNetworkStatusLiveRowFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } for (const field of __ParentAgentNetworkStatusLiveRowZeroFields) { __ParentAgentNetworkStatusReadRequiredCount(record, field, 0); } __ParentAgentNetworkStatusReadLiteral(record, 'platform', Object.values(ParentAgentNetworkLiveCapturePlatform)); __ParentAgentNetworkStatusReadLiteral(record, 'proofState', Object.values(ParentAgentNetworkLiveCaptureProofState)); __ParentAgentNetworkStatusReadLiteral(record, 'storageState', Object.values(ParentAgentNetworkRawCaptureStorageState)); __ParentAgentNetworkStatusReadLiteral(record, 'executionState', Object.values(ParentAgentNetworkLiveCaptureExecutionState)); return record as ParentAgentNetworkLiveCaptureStatusRow; }
export function decodeParentAgentNetworkLiveCaptureStatus(value: unknown): ParentAgentNetworkLiveCaptureStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network live capture status'); for (const field of __ParentAgentNetworkStatusLiveStatusStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusLiveStatusCountFields) { __ParentAgentNetworkStatusReadCount(record, field); } for (const field of __ParentAgentNetworkStatusLiveStatusZeroFields) { __ParentAgentNetworkStatusReadRequiredCount(record, field, 0); } const rows = record['rows']; if (!Array.isArray(rows)) { throw new TypeError('rows must be a network live capture status row array'); } rows.forEach((row) => decodeParentAgentNetworkLiveCaptureStatusRow(row)); return record as ParentAgentNetworkLiveCaptureStatus; }
export function decodeParentAgentNetworkLinuxNftablesLabCommandRow(value: unknown): ParentAgentNetworkLinuxNftablesLabCommandRow { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Linux nftables lab command row'); const kind = __ParentAgentNetworkStatusReadLiteral(record, 'kind', Object.values(ParentAgentNetworkLinuxNftablesLabCommandKind)); __ParentAgentNetworkStatusReadString(record, 'commandRef'); __ParentAgentNetworkStatusReadInteger(record, 'exitStatus'); __ParentAgentNetworkStatusReadString(record, 'outputSha256'); __ParentAgentNetworkStatusReadBoolean(record, 'tablePresentAfterCommand'); __ParentAgentNetworkStatusReadBoolean(record, 'chainPresentAfterCommand'); __ParentAgentNetworkStatusReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as ParentAgentNetworkLinuxNftablesLabCommandRow; }
export function decodeParentAgentNetworkLinuxNftablesLabStatus(value: unknown): ParentAgentNetworkLinuxNftablesLabStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Linux nftables lab status'); for (const field of __ParentAgentNetworkStatusLinuxNftablesStringFields) { __ParentAgentNetworkStatusReadString(record, field); } __ParentAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); for (const field of __ParentAgentNetworkStatusLinuxNftablesBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } for (const field of __ParentAgentNetworkStatusLinuxNftablesFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } const state = __ParentAgentNetworkStatusReadLiteral(record, 'state', Object.values(ParentAgentNetworkLinuxNftablesLabState)); const commandEvidence = __ParentAgentNetworkStatusReadRecordArray(record, 'commandEvidence', 'network Linux nftables lab command row').map((row) => decodeParentAgentNetworkLinuxNftablesLabCommandRow(row)); __ParentAgentNetworkStatusRequireCountMatches(record, 'commandCount', commandEvidence.length); __ParentAgentNetworkStatusRequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === ParentAgentNetworkLinuxNftablesLabState.ExecutedAndRolledBack) { const byKind = __ParentAgentNetworkStatusRequireUniqueRowsByKind(commandEvidence, 'Linux nftables lab'); __ParentAgentNetworkStatusRequireObservedFlags(record, byKind, __ParentAgentNetworkStatusLinuxNftablesObservedFlags, 'Linux nftables lab'); __ParentAgentNetworkStatusRequireLinuxNftablesOutcomes(byKind); } return { ...record, commandEvidence } as ParentAgentNetworkLinuxNftablesLabStatus; }
export function decodeParentAgentNetworkWindowsFirewallLabCommandRow(value: unknown): ParentAgentNetworkWindowsFirewallLabCommandRow { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Windows firewall lab command row'); const kind = __ParentAgentNetworkStatusReadLiteral(record, 'kind', Object.values(ParentAgentNetworkWindowsFirewallLabCommandKind)); __ParentAgentNetworkStatusReadString(record, 'commandRef'); __ParentAgentNetworkStatusReadInteger(record, 'exitStatus'); __ParentAgentNetworkStatusReadString(record, 'outputSha256'); __ParentAgentNetworkStatusReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as ParentAgentNetworkWindowsFirewallLabCommandRow; }
export function decodeParentAgentNetworkWindowsFirewallLabStatus(value: unknown): ParentAgentNetworkWindowsFirewallLabStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Windows firewall lab status'); for (const field of __ParentAgentNetworkStatusWindowsFirewallStringFields) { __ParentAgentNetworkStatusReadString(record, field); } __ParentAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); for (const field of __ParentAgentNetworkStatusWindowsFirewallBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } for (const field of __ParentAgentNetworkStatusWindowsFirewallFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } const state = __ParentAgentNetworkStatusReadLiteral(record, 'state', Object.values(ParentAgentNetworkWindowsFirewallLabState)); const commandEvidence = __ParentAgentNetworkStatusReadRecordArray(record, 'commandEvidence', 'network Windows firewall lab command row').map((row) => decodeParentAgentNetworkWindowsFirewallLabCommandRow(row)); __ParentAgentNetworkStatusRequireCountMatches(record, 'commandCount', commandEvidence.length); __ParentAgentNetworkStatusRequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === ParentAgentNetworkWindowsFirewallLabState.ExecutedAndRolledBack) { const byKind = __ParentAgentNetworkStatusRequireUniqueRowsByKind(commandEvidence, 'Windows firewall lab'); __ParentAgentNetworkStatusRequireObservedFlags(record, byKind, __ParentAgentNetworkStatusWindowsFirewallObservedFlags, 'Windows firewall lab'); __ParentAgentNetworkStatusRequireWindowsFirewallOutcomes(byKind); } return { ...record, commandEvidence } as ParentAgentNetworkWindowsFirewallLabStatus; }
export function decodeParentAgentNetworkWindowsWfpGateStatus(value: unknown): ParentAgentNetworkWindowsWfpGateStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Windows WFP gate status'); for (const field of __ParentAgentNetworkStatusWindowsWfpStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusWindowsWfpNullableStringFields) { __ParentAgentNetworkStatusReadNullableString(record, field); } for (const field of __ParentAgentNetworkStatusWindowsWfpStringArrayFields) { __ParentAgentNetworkStatusReadStringArray(record, field); } const capabilityState = __ParentAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(ParentAgentNetworkWindowsWfpCapabilityState)); const gateState = __ParentAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(ParentAgentNetworkWindowsWfpGateState)); const boundaryReasons = __ParentAgentNetworkStatusReadStringArray(record, 'boundaryReasons'); const missingRequiredArtifacts = __ParentAgentNetworkStatusReadStringArray(record, 'missingRequiredArtifacts'); const wfpLabProofReady = __ParentAgentNetworkStatusReadBoolean(record, 'wfpLabProofReady'); for (const field of __ParentAgentNetworkStatusWindowsWfpFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } __ParentAgentNetworkStatusRequireGateConsistency('Windows WFP', gateState === ParentAgentNetworkWindowsWfpGateState.LabProofReady, __ParentAgentNetworkStatusGateProofReadyIsValid(capabilityState === ParentAgentNetworkWindowsWfpCapabilityState.LabReady, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts), gateState === ParentAgentNetworkWindowsWfpGateState.ManualRequired, __ParentAgentNetworkStatusGateManualRequiredIsValid(capabilityState === ParentAgentNetworkWindowsWfpCapabilityState.ManualRequired, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, wfpLabProofReady, enforcementCommandPublished: false } as ParentAgentNetworkWindowsWfpGateStatus; }
export function decodeParentAgentNetworkAndroidVpnServiceGateStatus(value: unknown): ParentAgentNetworkAndroidVpnServiceGateStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Android VpnService gate status'); for (const field of __ParentAgentNetworkStatusAndroidVpnStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusAndroidVpnNullableStringFields) { __ParentAgentNetworkStatusReadNullableString(record, field); } __ParentAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); const capabilityState = __ParentAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(ParentAgentNetworkAndroidVpnServiceCapabilityState)); const gateState = __ParentAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(ParentAgentNetworkAndroidVpnServiceGateState)); const boundaryReasons = __ParentAgentNetworkStatusReadLiteralArray(record, 'boundaryReasons', Object.values(ParentAgentNetworkAndroidVpnServiceBoundaryReason)); const missingRequiredArtifacts = __ParentAgentNetworkStatusReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(ParentAgentNetworkAndroidVpnServiceRequiredArtifact)); for (const field of __ParentAgentNetworkStatusAndroidVpnBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } const physicalDeviceProofReady = __ParentAgentNetworkStatusReadBoolean(record, 'physicalDeviceProofReady'); for (const field of __ParentAgentNetworkStatusAndroidVpnFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } __ParentAgentNetworkStatusRequireGateConsistency('Android VpnService', gateState === ParentAgentNetworkAndroidVpnServiceGateState.PhysicalDeviceProofReady, __ParentAgentNetworkStatusGateProofReadyIsValid(capabilityState === ParentAgentNetworkAndroidVpnServiceCapabilityState.PhysicalDeviceReady, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts), gateState === ParentAgentNetworkAndroidVpnServiceGateState.ManualRequired, __ParentAgentNetworkStatusGateManualRequiredIsValid(capabilityState === ParentAgentNetworkAndroidVpnServiceCapabilityState.ManualRequired, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, gateState, physicalDeviceProofReady, enforcementCommandPublished: false } as ParentAgentNetworkAndroidVpnServiceGateStatus; }
export function decodeParentAgentNetworkAppleNetworkExtensionGateStatus(value: unknown): ParentAgentNetworkAppleNetworkExtensionGateStatus { const record = __ParentAgentNetworkStatusReadRecord(value, 'network Apple Network Extension gate status'); for (const field of __ParentAgentNetworkStatusAppleNetworkExtensionStringFields) { __ParentAgentNetworkStatusReadString(record, field); } for (const field of __ParentAgentNetworkStatusAppleNetworkExtensionNullableStringFields) { __ParentAgentNetworkStatusReadNullableString(record, field); } __ParentAgentNetworkStatusReadStringArray(record, 'evidenceRefs'); const platform = __ParentAgentNetworkStatusReadLiteral(record, 'platform', Object.values(ParentAgentNetworkAppleNetworkExtensionPlatform)); const capabilityState = __ParentAgentNetworkStatusReadLiteral(record, 'capabilityState', Object.values(ParentAgentNetworkAppleNetworkExtensionCapabilityState)); const gateState = __ParentAgentNetworkStatusReadLiteral(record, 'gateState', Object.values(ParentAgentNetworkAppleNetworkExtensionGateState)); const boundaryReasons = __ParentAgentNetworkStatusReadLiteralArray(record, 'boundaryReasons', Object.values(ParentAgentNetworkAppleNetworkExtensionBoundaryReason)); const missingRequiredArtifacts = __ParentAgentNetworkStatusReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(ParentAgentNetworkAppleNetworkExtensionRequiredArtifact)); for (const field of __ParentAgentNetworkStatusAppleNetworkExtensionBooleanFields) { __ParentAgentNetworkStatusReadBoolean(record, field); } const appleEntitlementProofReady = __ParentAgentNetworkStatusReadBoolean(record, 'appleEntitlementProofReady'); for (const field of __ParentAgentNetworkStatusAppleNetworkExtensionFalseFields) { __ParentAgentNetworkStatusReadRequiredBoolean(record, field, false); } __ParentAgentNetworkStatusRequireGateConsistency('Apple Network Extension', gateState === ParentAgentNetworkAppleNetworkExtensionGateState.AppleEntitlementProofReady, __ParentAgentNetworkStatusGateProofReadyIsValid(capabilityState === ParentAgentNetworkAppleNetworkExtensionCapabilityState.AppleDeviceReady, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts), gateState === ParentAgentNetworkAppleNetworkExtensionGateState.ManualRequired, __ParentAgentNetworkStatusGateManualRequiredIsValid(capabilityState === ParentAgentNetworkAppleNetworkExtensionCapabilityState.ManualRequired, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, platform, gateState, appleEntitlementProofReady, enforcementCommandPublished: false } as ParentAgentNetworkAppleNetworkExtensionGateStatus; }
export const ParentAgentNetworkRemoteDeliveryStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkRemoteDeliveryStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkRemoteDeliveryStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkLiveCaptureStatusRowSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkLiveCaptureStatusRow } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkLiveCaptureStatusRow(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkLiveCaptureStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkLiveCaptureStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkLiveCaptureStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkLinuxNftablesLabStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkLinuxNftablesLabStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkLinuxNftablesLabStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkWindowsFirewallLabStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkWindowsFirewallLabStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkWindowsFirewallLabStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkWindowsWfpGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkWindowsWfpGateStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkWindowsWfpGateStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkAndroidVpnServiceGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkAndroidVpnServiceGateStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkAndroidVpnServiceGateStatus(value) }; } catch { return { success: false }; } } } as const;
export const ParentAgentNetworkAppleNetworkExtensionGateStatusSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentNetworkAppleNetworkExtensionGateStatus } | { readonly success: false } { try { return { success: true, data: decodeParentAgentNetworkAppleNetworkExtensionGateStatus(value) }; } catch { return { success: false }; } } } as const;
 export const ParentAgentTrackingRetentionSettingsWriteDefaults = { CommandId: "tracking-retention-settings-write-command", SettingsKindRetentionWindow: "retention-window-setting", WriterIntentRef: "tracking-retention-settings-write-retention-window", ReadModelProofRefs: ["output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json", "output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json"], MutationProofRef: "output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json", LocalServiceStateSnapshotRef: "agent-service-local-retention-settings-state", DurableSettingsStoreRef: "agent-service-local-retention-settings-durable-json", WriteStateAccepted: "service-write-command-accepted", WriteStateRejected: "service-write-command-rejected", AcceptedAt: "2026-06-06T19:50:00Z" } as const; export const ParentAgentTrackingDeleteAfterAlertResolutionState = { DeleteAfterAlertResolved: "delete-after-alert-resolved", RetainAfterAlertResolved: "retain-after-alert-resolved" } as const; export type ParentAgentTrackingDeleteAfterAlertResolutionState = (typeof ParentAgentTrackingDeleteAfterAlertResolutionState)[keyof typeof ParentAgentTrackingDeleteAfterAlertResolutionState]; export const ParentAgentTrackingParentExportState = { Prepared: "prepared", NotPrepared: "not-prepared" } as const; export type ParentAgentTrackingParentExportState = (typeof ParentAgentTrackingParentExportState)[keyof typeof ParentAgentTrackingParentExportState]; export const ParentAgentTrackingRemoteSyncState = { Enabled: "enabled", Disabled: "disabled" } as const; export type ParentAgentTrackingRemoteSyncState = (typeof ParentAgentTrackingRemoteSyncState)[keyof typeof ParentAgentTrackingRemoteSyncState]; export const ParentAgentTrackingRemoteAiState = { Enabled: "enabled", Disabled: "disabled" } as const; export type ParentAgentTrackingRemoteAiState = (typeof ParentAgentTrackingRemoteAiState)[keyof typeof ParentAgentTrackingRemoteAiState]; export const ParentAgentTrackingDurableSettingsPersistenceState = { Persisted: "persisted", NotPersisted: "not-persisted" } as const; export type ParentAgentTrackingDurableSettingsPersistenceState = (typeof ParentAgentTrackingDurableSettingsPersistenceState)[keyof typeof ParentAgentTrackingDurableSettingsPersistenceState]; export const ParentAgentTrackingConfigAckState = { Received: "received", Missing: "missing" } as const; export type ParentAgentTrackingConfigAckState = (typeof ParentAgentTrackingConfigAckState)[keyof typeof ParentAgentTrackingConfigAckState]; export const ParentAgentTrackingExecutionClaimState = { Claimed: "claimed", Unclaimed: "unclaimed" } as const; export type ParentAgentTrackingExecutionClaimState = (typeof ParentAgentTrackingExecutionClaimState)[keyof typeof ParentAgentTrackingExecutionClaimState]; export const ParentAgentTrackingConfigUpdateResponseState = { Applied: "applied", Rejected: "rejected" } as const; export type ParentAgentTrackingConfigUpdateResponseState = (typeof ParentAgentTrackingConfigUpdateResponseState)[keyof typeof ParentAgentTrackingConfigUpdateResponseState]; export const ParentAgentTrackingEffectiveState = { Enabled: "enabled", Disabled: "disabled", Degraded: "degraded" } as const; export type ParentAgentTrackingEffectiveState = (typeof ParentAgentTrackingEffectiveState)[keyof typeof ParentAgentTrackingEffectiveState];
export type ParentAgentTrackingRetentionSettingsWriteResult = { readonly schemaVersion: number; readonly commandId: string; readonly settingsKind: string; readonly writeState: string; readonly acceptedAt: string; readonly sourceWriterIntentRefs: readonly string[]; readonly sourceReadModelProofRefs: readonly string[]; readonly sourceMutationProofRefs: readonly string[]; readonly appliedRetentionWindowHours: number | null; readonly appliedDeleteAfterAlertResolutionState: ParentAgentTrackingDeleteAfterAlertResolutionState; readonly parentExportState: ParentAgentTrackingParentExportState; readonly remoteSyncState: typeof ParentAgentTrackingRemoteSyncState.Disabled; readonly remoteAiState: typeof ParentAgentTrackingRemoteAiState.Disabled; readonly localServiceStateRevision: number | null; readonly localServiceStateSnapshotRef: string; readonly durableSettingsStoreRef: string; readonly durableSettingsPersistenceState: ParentAgentTrackingDurableSettingsPersistenceState; readonly childConfigResponseState?: ParentAgentTrackingConfigUpdateResponseState | null; readonly effectiveTrackingState?: ParentAgentTrackingEffectiveState | null; readonly childConfigAckState: ParentAgentTrackingConfigAckState; readonly commandTransportClaimState: typeof ParentAgentTrackingExecutionClaimState.Claimed; readonly serviceWritePreflightClaimState: typeof ParentAgentTrackingExecutionClaimState.Claimed; readonly serviceMutationExecutionState: ParentAgentTrackingExecutionClaimState; readonly portalWritableUiClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly platformRuntimeClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly childDeviceDeliveryClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly providerDeliveryClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly notificationReceiptClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly physicalDeviceClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly authorityClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; readonly productClaimState: typeof ParentAgentTrackingExecutionClaimState.Unclaimed; };
type ParentAgentTrackingRetentionSettingsWriteResultOptionals = { childConfigResponseState?: ParentAgentTrackingConfigUpdateResponseState | null; effectiveTrackingState?: ParentAgentTrackingEffectiveState | null };
function decodeParentAgentTrackingRetentionSettingsWriteResultIsRecord(candidate: unknown): candidate is Readonly<Record<string, unknown>> { return typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadString(record: Readonly<Record<string, unknown>>, field: string): string { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty tracking retention string`); } return fieldValue; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { if (record['schemaVersion'] !== ParentAgentProtocolRuntime.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return ParentAgentProtocolRuntime.SchemaVersion; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadNullableNumber(record: Readonly<Record<string, unknown>>, field: string): number | null { const fieldValue = record[field]; if (fieldValue === null) { return null; } if (typeof fieldValue !== 'number' || !Number.isInteger(fieldValue) || fieldValue <= 0) { throw new TypeError(`${field} must be a positive integer or null`); } return fieldValue; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const fieldValue = record[field]; if (!Array.isArray(fieldValue) || fieldValue.length === 0 || fieldValue.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a non-empty string array`); } return fieldValue as readonly string[]; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const fieldValue = decodeParentAgentTrackingRetentionSettingsWriteResultReadString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const fieldValue = decodeParentAgentTrackingRetentionSettingsWriteResultReadString(record, field); if (fieldValue !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T | null | undefined { const fieldValue = record[field]; if (fieldValue === undefined) { return undefined; } if (fieldValue === null) { return null; } if (typeof fieldValue !== 'string' || !allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function decodeParentAgentTrackingRetentionSettingsWriteResultReadAckState(record: Readonly<Record<string, unknown>>): ParentAgentTrackingConfigAckState { if (record['childConfigAckState'] === undefined) { return ParentAgentTrackingConfigAckState.Missing; } return decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(record, 'childConfigAckState', Object.values(ParentAgentTrackingConfigAckState)); }
function decodeParentAgentTrackingRetentionSettingsWriteResultAttachOptionals(result: ParentAgentTrackingRetentionSettingsWriteResult, childConfigResponseState: ParentAgentTrackingConfigUpdateResponseState | null | undefined, effectiveTrackingState: ParentAgentTrackingEffectiveState | null | undefined): ParentAgentTrackingRetentionSettingsWriteResult { const resultWithOptionals = result as ParentAgentTrackingRetentionSettingsWriteResult & ParentAgentTrackingRetentionSettingsWriteResultOptionals; if (childConfigResponseState !== undefined) { resultWithOptionals.childConfigResponseState = childConfigResponseState; } if (effectiveTrackingState !== undefined) { resultWithOptionals.effectiveTrackingState = effectiveTrackingState; } return resultWithOptionals; }
function decodeParentAgentTrackingRetentionSettingsWriteResultRequireAcceptedInvariants(result: ParentAgentTrackingRetentionSettingsWriteResult): void { if (result.writeState !== ParentAgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted) { return; } if (result.commandTransportClaimState !== ParentAgentTrackingExecutionClaimState.Claimed) { throw new TypeError('accepted tracking write result must prove command transport'); } if (result.serviceMutationExecutionState !== ParentAgentTrackingExecutionClaimState.Claimed) { throw new TypeError('accepted tracking write result must execute local mutation'); } if (result.localServiceStateRevision === null) { throw new TypeError('accepted tracking write result must include local service revision'); } if (result.durableSettingsPersistenceState !== ParentAgentTrackingDurableSettingsPersistenceState.Persisted) { throw new TypeError('accepted tracking write result must persist durable settings'); } }
function decodeParentAgentTrackingRetentionSettingsWriteResultRequireRetentionWindowInvariant(result: ParentAgentTrackingRetentionSettingsWriteResult): void { if (result.settingsKind === ParentAgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow && result.appliedRetentionWindowHours === null) { throw new TypeError('retention-window write result must include applied retention window'); } }
function decodeParentAgentTrackingRetentionSettingsWriteResultFinalize(result: ParentAgentTrackingRetentionSettingsWriteResult, childConfigResponseState: ParentAgentTrackingConfigUpdateResponseState | null | undefined, effectiveTrackingState: ParentAgentTrackingEffectiveState | null | undefined): ParentAgentTrackingRetentionSettingsWriteResult { const resultWithOptionals = decodeParentAgentTrackingRetentionSettingsWriteResultAttachOptionals(result, childConfigResponseState, effectiveTrackingState); decodeParentAgentTrackingRetentionSettingsWriteResultRequireAcceptedInvariants(resultWithOptionals); decodeParentAgentTrackingRetentionSettingsWriteResultRequireRetentionWindowInvariant(resultWithOptionals); return resultWithOptionals; }
export function decodeParentAgentTrackingRetentionSettingsWriteResult(value: unknown): ParentAgentTrackingRetentionSettingsWriteResult { if (!decodeParentAgentTrackingRetentionSettingsWriteResultIsRecord(value)) { throw new TypeError('tracking retention write result must be an object'); } const childConfigResponseState = decodeParentAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral(value, 'childConfigResponseState', Object.values(ParentAgentTrackingConfigUpdateResponseState)); const effectiveTrackingState = decodeParentAgentTrackingRetentionSettingsWriteResultReadOptionalNullableLiteral(value, 'effectiveTrackingState', Object.values(ParentAgentTrackingEffectiveState)); const result: ParentAgentTrackingRetentionSettingsWriteResult = { schemaVersion: decodeParentAgentTrackingRetentionSettingsWriteResultReadSchemaVersion(value), commandId: decodeParentAgentTrackingRetentionSettingsWriteResultReadString(value, 'commandId'), settingsKind: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'settingsKind', ParentAgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow), writeState: decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'writeState', [ParentAgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted, ParentAgentTrackingRetentionSettingsWriteDefaults.WriteStateRejected] as const), acceptedAt: decodeParentAgentTrackingRetentionSettingsWriteResultReadString(value, 'acceptedAt'), sourceWriterIntentRefs: decodeParentAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceWriterIntentRefs'), sourceReadModelProofRefs: decodeParentAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceReadModelProofRefs'), sourceMutationProofRefs: decodeParentAgentTrackingRetentionSettingsWriteResultReadStringArray(value, 'sourceMutationProofRefs'), appliedRetentionWindowHours: decodeParentAgentTrackingRetentionSettingsWriteResultReadNullableNumber(value, 'appliedRetentionWindowHours'), appliedDeleteAfterAlertResolutionState: decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'appliedDeleteAfterAlertResolutionState', Object.values(ParentAgentTrackingDeleteAfterAlertResolutionState)), parentExportState: decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'parentExportState', Object.values(ParentAgentTrackingParentExportState)), remoteSyncState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'remoteSyncState', ParentAgentTrackingRemoteSyncState.Disabled), remoteAiState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'remoteAiState', ParentAgentTrackingRemoteAiState.Disabled), localServiceStateRevision: decodeParentAgentTrackingRetentionSettingsWriteResultReadNullableNumber(value, 'localServiceStateRevision'), localServiceStateSnapshotRef: decodeParentAgentTrackingRetentionSettingsWriteResultReadString(value, 'localServiceStateSnapshotRef'), durableSettingsStoreRef: decodeParentAgentTrackingRetentionSettingsWriteResultReadString(value, 'durableSettingsStoreRef'), durableSettingsPersistenceState: decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'durableSettingsPersistenceState', Object.values(ParentAgentTrackingDurableSettingsPersistenceState)), childConfigAckState: decodeParentAgentTrackingRetentionSettingsWriteResultReadAckState(value), commandTransportClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'commandTransportClaimState', ParentAgentTrackingExecutionClaimState.Claimed), serviceWritePreflightClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'serviceWritePreflightClaimState', ParentAgentTrackingExecutionClaimState.Claimed), serviceMutationExecutionState: decodeParentAgentTrackingRetentionSettingsWriteResultReadLiteral(value, 'serviceMutationExecutionState', Object.values(ParentAgentTrackingExecutionClaimState)), portalWritableUiClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'portalWritableUiClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), platformRuntimeClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'platformRuntimeClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), childDeviceDeliveryClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'childDeviceDeliveryClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), providerDeliveryClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'providerDeliveryClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), notificationReceiptClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'notificationReceiptClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), physicalDeviceClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'physicalDeviceClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), authorityClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'authorityClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed), productClaimState: decodeParentAgentTrackingRetentionSettingsWriteResultReadRequiredLiteral(value, 'productClaimState', ParentAgentTrackingExecutionClaimState.Unclaimed) }; return decodeParentAgentTrackingRetentionSettingsWriteResultFinalize(result, childConfigResponseState, effectiveTrackingState); }
export const ParentAgentTrackingRetentionSettingsWriteResultSchema = { safeParse(value: unknown): { readonly success: true; readonly data: ParentAgentTrackingRetentionSettingsWriteResult } | { readonly success: false } { try { return { success: true, data: decodeParentAgentTrackingRetentionSettingsWriteResult(value) }; } catch { return { success: false }; } } } as const;
 export const ParentAgentProtocolDelimiter = { List: ",", EventIdSuffix: "-" } as const; export type ParentAgentProtocolDelimiter = (typeof ParentAgentProtocolDelimiter)[keyof typeof ParentAgentProtocolDelimiter]; export const ParentAgentCommand = { HealthCheck: "agent.health.check", LogSnapshotGet: "agent.log.snapshot.get", DevEcho: "agent.dev.echo", WatchStatusGet: "agent.watch.status.get", ActivityIngestStatusGet: "agent.activity.ingest.status.get", ActivityRecentSummaryGet: "agent.activity.recent.summary.get", ActivityMemoryGraphGet: "agent.activity.memory-graph.get", ActivityReportDailyGenerate: "agent.activity.report.daily.generate", ActivityReportWeeklyGenerate: "agent.activity.report.weekly.generate", ActivityReportMonthlyGenerate: "agent.activity.report.monthly.generate", ActivityReportSave: "agent.activity.report.save", ActivityReportHistoryList: "agent.activity.report.history.list", ActivityScreenReadModelGet: "agent.activity.screen.read-model.get", ActivityAppUseReadModelGet: "agent.activity.app-use.read-model.get", ActivityBrowserReadModelGet: "agent.activity.browser.read-model.get", ActivityGamesReadModelGet: "agent.activity.games.read-model.get", ActivityAppGameBoundaryReadModelGet: "agent.activity.app-game.boundary.read-model.get", ActivityAppGamePolicyReadinessReadModelGet: "agent.activity.app-game.policy-readiness.read-model.get", ActivityAppGameNotificationReadinessReadModelGet: "agent.activity.app-game.notification-readiness.read-model.get", ActivityAppGameAdapterExecutionReadinessReadModelGet: "agent.activity.app-game.adapter-execution-readiness.read-model.get", ActivityAppGamePlatformProofStatusReadModelGet: "agent.activity.app-game.platform-proof-status.read-model.get", ActivityAppGameChildRuntimeTransportReceiptReadModelGet: "agent.activity.app-game.child-runtime-transport-receipt.read-model.get", ActivityAppGameAdapterDispatchPreflightReadModelGet: "agent.activity.app-game.adapter-dispatch-preflight.read-model.get", ActivityAppGameAdapterDispatchResultReadModelGet: "agent.activity.app-game.adapter-dispatch-result.read-model.get", ActivityAppGameAdapterDispatchExecute: "agent.activity.app-game.adapter-dispatch.execute", ActivityAppGameTimerParentSurfaceReadModelGet: "agent.activity.app-game.timer-parent-surface.read-model.get", ActivityAppGameTimerParentPreferenceSetupRequest: "agent.activity.app-game.timer-parent-surface.parent-preference-setup.request", BrowserSocialDashboardReadModelGet: "agent.browser.social-dashboard.read-model.get", BrowserSocialAuditExplanationReadModelGet: "agent.browser.social-audit-explanation.read-model.get", BrowserSocialAlertReportReadModelGet: "agent.browser.social-alert-report.read-model.get", BrowserSocialAlertReportParentSurfaceReadModelGet: "agent.browser.social-alert-report.parent-surface.read-model.get", BrowserSocialParentNotificationDeliveryReadModelGet: "agent.browser.social-parent-notification-delivery.read-model.get", BrowserSocialSourceCustodyMutationApply: "agent.browser.social-source-custody.mutation.apply", ActivityNetworkReadModelGet: "agent.activity.network.read-model.get", ActivityTrackingRetentionSettingsWrite: "agent.activity.tracking.retention-settings.write", BrowserEvidenceRecentGet: "agent.browser.evidence.recent.get", BrowserManagedBridgePoll: "agent.browser.managed.bridge.poll", BrowserInventoryReadModelGet: "agent.browser.inventory.read-model.get", BrowserInterventionReadModelGet: "agent.browser.intervention.read-model.get", BrowserRuntimeEventChainStreamGet: "agent.browser.runtime.event-chain.stream.get", NetworkFlowReadModelGet: "agent.network.flow.read-model.get", LanPairingStatusGet: "agent.lan-pairing.status.get", NetworkRuntimeEventChainStreamGet: "agent.network.runtime.event-chain.stream.get", LanRuntimeEventChainStreamGet: "agent.lan.runtime.event-chain.stream.get", NetworkRemoteDeliveryStatusGet: "agent.network.remote-delivery.status.get", NetworkLiveCaptureStatusGet: "agent.network.live-capture.status.get", NetworkLinuxNftablesLabStatusGet: "agent.network.linux-nftables-lab.status.get", NetworkWindowsFirewallLabStatusGet: "agent.network.windows-firewall-lab.status.get", NetworkWindowsWfpGateStatusGet: "agent.network.windows-wfp-gate.status.get", NetworkAndroidVpnServiceGateStatusGet: "agent.network.android-vpn-service-gate.status.get", NetworkAppleNetworkExtensionGateStatusGet: "agent.network.apple-network-extension-gate.status.get", ActivityTrackingReadModelGet: "agent.activity.tracking.read-model.get", LocalAiRuntimeStatusGet: "agent.local-ai.runtime.status.get", LocalAiChatGenerate: "agent.local-ai.chat.generate", ParentAssistantAnswerGenerate: "agent.parent-assistant.answer.generate", PolicyPreviewReadModelGet: "agent.policy.preview.read-model.get", PolicyRequestAssistantPreviewConfirm: "agent.policy.request.assistant-preview.confirm", PolicyRequestParentResolutionResolve: "agent.policy.request.parent-resolution.resolve", BrowserPolicyGet: "agent.browser-policy.get", BrowserPolicyPreview: "agent.browser-policy.preview", BrowserPolicyPatch: "agent.browser-policy.patch", BrowserPolicyReplace: "agent.browser-policy.replace", BrowserPolicyRollback: "agent.browser-policy.rollback", ScreenSettingsGet: "agent.screen-settings.get", ScreenSettingsReplace: "agent.screen-settings.replace", EnforcementExecute: "agent.enforcement.execute", EnforcementTimerRecover: "agent.enforcement.timer.recover", EnforcementTimerExpire: "agent.enforcement.timer.expire", EnforcementOverrideCancel: "agent.enforcement.override.cancel", EnforcementProductControlSpineGet: "agent.enforcement.product-control-spine.get", EnforcementPolicyDispatchGet: "agent.enforcement.policy-dispatch.get", EnforcementBroadAdapterProofGet: "agent.enforcement.broad-adapter-proof.get", EnforcementSupportedAdapterRuntimeProofGet: "agent.enforcement.supported-adapter-runtime-proof.get", ParentAssistantThreadList: "agent.parent-assistant.thread.list", ParentAssistantThreadCreate: "agent.parent-assistant.thread.create", ParentAssistantThreadOpen: "agent.parent-assistant.thread.open", ParentAssistantThreadArchive: "agent.parent-assistant.thread.archive", ParentAssistantMessageSend: "agent.parent-assistant.message.send", ParentAssistantRunCancel: "agent.parent-assistant.run.cancel", ParentAssistantQuickActionStart: "agent.parent-assistant.quick-action.start", ParentAssistantActionPreview: "agent.parent-assistant.action.preview", ParentAssistantActionConfirm: "agent.parent-assistant.action.confirm", ParentAssistantProviderStatusGet: "agent.parent-assistant.provider.status.get", LanPairingProofSubmit: "agent.lan-pairing.proof.submit", LanPairingRouteSelect: "agent.lan-pairing.route.select", LanPairingRouteRevoke: "agent.lan-pairing.route.revoke", LanPairingBrowserDiscoveryScan: "agent.lan-pairing.browser-discovery.scan", LanPairingAddDeviceRequest: "agent.lan-pairing.add-device.request", LanPairingSignedChildAgentObserve: "agent.lan-pairing.signed-child-agent.observe", LanPairingControllerLeaseRenew: "agent.lan-pairing.controller-lease.renew", LanPairingControllerLeaseRelease: "agent.lan-pairing.controller-lease.release", LanPairingControllerLeaseTakeover: "agent.lan-pairing.controller-lease.takeover", LanAiProviderStatusGet: "agent.lan-ai.provider.status.get", LanAiJobSubmit: "agent.lan-ai.job.submit" } as const; export type ParentAgentCommandName = (typeof ParentAgentCommand)[keyof typeof ParentAgentCommand]; export const ParentAgentEvent = { ConnectionReady: "agent.connection.ready", CommandRejected: "agent.command.rejected", HealthReported: "agent.health.reported", LogSnapshotReported: "agent.log.snapshot.reported", DevEchoed: "agent.dev.echoed", WatchStatusReported: "agent.watch.status.reported", ActivityIngestStatusReported: "agent.activity.ingest.status.reported", ActivityRecentSummaryReported: "agent.activity.recent.summary.reported", ActivityMemoryGraphReported: "agent.activity.memory-graph.reported", ActivityReportGenerated: "agent.activity.report.generated", ActivityReportSaved: "agent.activity.report.saved", ActivityReportHistoryReported: "agent.activity.report.history.reported", ActivityScreenReadModelReported: "agent.activity.screen.read-model.reported", ActivityAppUseReadModelReported: "agent.activity.app-use.read-model.reported", ActivityBrowserReadModelReported: "agent.activity.browser.read-model.reported", ActivityGamesReadModelReported: "agent.activity.games.read-model.reported", ActivityAppGameBoundaryReadModelReported: "agent.activity.app-game.boundary.read-model.reported", ActivityAppGameNotificationReadinessReadModelReported: "agent.activity.app-game.notification-readiness.read-model.reported", ActivityAppGameAdapterExecutionReadinessReadModelReported: "agent.activity.app-game.adapter-execution-readiness.read-model.reported", ActivityAppGamePlatformProofStatusReadModelReported: "agent.activity.app-game.platform-proof-status.read-model.reported", ActivityAppGameChildRuntimeTransportReceiptReadModelReported: "agent.activity.app-game.child-runtime-transport-receipt.read-model.reported", ActivityAppGameAdapterDispatchPreflightReadModelReported: "agent.activity.app-game.adapter-dispatch-preflight.read-model.reported", ActivityAppGameAdapterDispatchResultReadModelReported: "agent.activity.app-game.adapter-dispatch-result.read-model.reported", ActivityAppGameAdapterDispatchExecuted: "agent.activity.app-game.adapter-dispatch.executed", ActivityAppGameTimerParentSurfaceReadModelReported: "agent.activity.app-game.timer-parent-surface.read-model.reported", ActivityAppGameTimerParentPreferenceSetupRequested: "agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested", BrowserSocialDashboardReadModelReported: "agent.browser.social-dashboard.read-model.reported", BrowserSocialAuditExplanationReadModelReported: "agent.browser.social-audit-explanation.read-model.reported", BrowserSocialAlertReportReadModelReported: "agent.browser.social-alert-report.read-model.reported", BrowserSocialAlertReportParentSurfaceReadModelReported: "agent.browser.social-alert-report.parent-surface.read-model.reported", BrowserSocialParentNotificationDeliveryReadModelReported: "agent.browser.social-parent-notification-delivery.read-model.reported", BrowserSocialSourceCustodyMutationApplied: "agent.browser.social-source-custody.mutation.applied", ActivityNetworkReadModelReported: "agent.activity.network.read-model.reported", BrowserEvidenceRecentReported: "agent.browser.evidence.recent.reported", BrowserManagedStatusReported: "agent.browser.managed.status.reported", BrowserInventoryReadModelReported: "agent.browser.inventory.read-model.reported", BrowserInterventionReadModelReported: "agent.browser.intervention.read-model.reported", BrowserRuntimeEventChainStreamReported: "agent.browser.runtime.event-chain.stream.reported", NetworkFlowReadModelReported: "agent.network.flow.read-model.reported", NetworkRuntimeEventChainStreamReported: "agent.network.runtime.event-chain.stream.reported", LanRuntimeEventChainStreamReported: "agent.lan.runtime.event-chain.stream.reported", NetworkRemoteDeliveryStatusReported: "agent.network.remote-delivery.status.reported", NetworkLiveCaptureStatusReported: "agent.network.live-capture.status.reported", NetworkLinuxNftablesLabStatusReported: "agent.network.linux-nftables-lab.status.reported", NetworkWindowsFirewallLabStatusReported: "agent.network.windows-firewall-lab.status.reported", NetworkWindowsWfpGateStatusReported: "agent.network.windows-wfp-gate.status.reported", NetworkAndroidVpnServiceGateStatusReported: "agent.network.android-vpn-service-gate.status.reported", NetworkAppleNetworkExtensionGateStatusReported: "agent.network.apple-network-extension-gate.status.reported", ActivityTrackingReadModelReported: "agent.activity.tracking.read-model.reported", ActivityTrackingRetentionSettingsWriteReported: "agent.activity.tracking.retention-settings.write.reported", LocalAiRuntimeStatusReported: "agent.local-ai.runtime.status.reported", LocalAiChatGenerationReported: "agent.local-ai.chat.generation.reported", PolicyPreviewReadModelReported: "agent.policy.preview.read-model.reported", PolicyRequestAssistantPreviewConfirmReported: "agent.policy.request.assistant-preview.confirm.reported", PolicyRequestParentResolutionResolved: "agent.policy.request.parent-resolution.resolved", BrowserPolicyReported: "agent.browser-policy.reported", BrowserPolicyPreviewed: "agent.browser-policy.previewed", BrowserPolicyPatchAccepted: "agent.browser-policy.patch.accepted", BrowserPolicyPatchRejected: "agent.browser-policy.patch.rejected", BrowserPolicyReplaceAccepted: "agent.browser-policy.replace.accepted", BrowserPolicyReplaceRejected: "agent.browser-policy.replace.rejected", BrowserPolicyRollbackAccepted: "agent.browser-policy.rollback.accepted", BrowserPolicyRollbackRejected: "agent.browser-policy.rollback.rejected", ScreenSettingsReported: "agent.screen-settings.reported", ScreenSettingsReplaceAccepted: "agent.screen-settings.replace.accepted", ScreenSettingsReplaceRejected: "agent.screen-settings.replace.rejected", EnforcementAuditReported: "agent.enforcement.audit.reported", EnforcementTimerReported: "agent.enforcement.timer.reported", EnforcementProductControlSpineReported: "agent.enforcement.product-control-spine.reported", EnforcementPolicyDispatchReported: "agent.enforcement.policy-dispatch.reported", EnforcementBroadAdapterProofReported: "agent.enforcement.broad-adapter-proof.reported", EnforcementSupportedAdapterRuntimeProofReported: "agent.enforcement.supported-adapter-runtime-proof.reported", ActivityAppGamePolicyReadinessReadModelReported: "agent.activity.app-game.policy-readiness.read-model.reported", ParentAssistantAnswerReported: "agent.parent-assistant.answer.reported", ParentAssistantThreadUpdated: "agent.parent-assistant.thread.updated", ParentAssistantMessageAccepted: "agent.parent-assistant.message.accepted", ParentAssistantRunStarted: "agent.parent-assistant.run.started", ParentAssistantMessageDelta: "agent.parent-assistant.message.delta", ParentAssistantMessageCompleted: "agent.parent-assistant.message.completed", ParentAssistantActionPreviewed: "agent.parent-assistant.action.previewed", ParentAssistantActionConfirmed: "agent.parent-assistant.action.confirmed", ParentAssistantProviderDegraded: "agent.parent-assistant.provider.degraded", ParentAssistantErrorReported: "agent.parent-assistant.error.reported", LanPairingStatusReported: "agent.lan-pairing.status.reported", LanPairingBrowserDiscoveryReported: "agent.lan-pairing.browser-discovery.reported", LanPairingAddDeviceReported: "agent.lan-pairing.add-device.reported", LanPairingSignedChildAgentReported: "agent.lan-pairing.signed-child-agent.reported", LanPairingAuditReported: "agent.lan-pairing.audit.reported", LanAiJobReported: "agent.lan-ai.job.reported" } as const; export type ParentAgentEventName = (typeof ParentAgentEvent)[keyof typeof ParentAgentEvent]; export const ParentAgentActivitySurfaceSchemaVersion = 1 as const; export const ParentAgentActivitySurfaceScopeKind = { Family: "family", Device: "device" } as const; export type ParentAgentActivitySurfaceScopeKind = (typeof ParentAgentActivitySurfaceScopeKind)[keyof typeof ParentAgentActivitySurfaceScopeKind]; export const ParentAgentActivityReportFrequency = { Daily: "daily", Weekly: "weekly", Monthly: "monthly" } as const; export type ParentAgentActivityReportFrequency = (typeof ParentAgentActivityReportFrequency)[keyof typeof ParentAgentActivityReportFrequency]; export const ParentAgentActivityReportSectionKind = { Summary: "summary", Screen: "screen", AppUse: "app-use", Browser: "browser", Games: "games", Network: "network" } as const; export type ParentAgentActivityReportSectionKind = (typeof ParentAgentActivityReportSectionKind)[keyof typeof ParentAgentActivityReportSectionKind]; export const ParentAgentActivityReadModelState = { Ready: "ready", Empty: "empty", Unavailable: "unavailable", Offline: "offline", Stale: "stale", PermissionRequired: "permission-required", ScaffoldOnly: "scaffold-only" } as const; export type ParentAgentActivityReadModelState = (typeof ParentAgentActivityReadModelState)[keyof typeof ParentAgentActivityReadModelState]; export const ParentAgentActivityReportSourceReachabilityState = { Reachable: "reachable", Unreachable: "unreachable", Offline: "offline", Error: "error" } as const; export type ParentAgentActivityReportSourceReachabilityState = (typeof ParentAgentActivityReportSourceReachabilityState)[keyof typeof ParentAgentActivityReportSourceReachabilityState]; export const ParentAgentActivitySavedReportState = { Draft: "draft", Saved: "saved", StorageUnavailable: "storage-unavailable", Degraded: "degraded", ScaffoldOnly: "scaffold-only" } as const; export type ParentAgentActivitySavedReportState = (typeof ParentAgentActivitySavedReportState)[keyof typeof ParentAgentActivitySavedReportState]; export const ParentAgentActivityReportCustodyLabel = { ChildDeviceLocalSummary: "child-device-local-summary", ParentDeviceLocalReportJson: "parent-device-local-report-json", ParentDeviceLocalHistory: "parent-device-local-history" } as const; export type ParentAgentActivityReportCustodyLabel = (typeof ParentAgentActivityReportCustodyLabel)[keyof typeof ParentAgentActivityReportCustodyLabel]; export const ParentAgentActivityReportSourceLabel = { ActivityQueryStoreSummary: "activity-query-store-summary", FamilyFanoutSourceState: "family-fanout-source-state", SavedReportJson: "saved-report-json", SavedReportHistory: "saved-report-history" } as const; export type ParentAgentActivityReportSourceLabel = (typeof ParentAgentActivityReportSourceLabel)[keyof typeof ParentAgentActivityReportSourceLabel]; export const ParentAgentActivityEvidenceKind = { JournalEntry: "journal-entry", Screenshot: "screenshot", StorageObject: "storage-object", LocalDbRow: "local-db-row" } as const; export type ParentAgentActivityEvidenceKind = (typeof ParentAgentActivityEvidenceKind)[keyof typeof ParentAgentActivityEvidenceKind];
export const ParentAgentActivitySurfaceReadModelKindName = { Screen: ParentAgentActivityReportSectionKind.Screen, AppUse: ParentAgentActivityReportSectionKind.AppUse, Browser: ParentAgentActivityReportSectionKind.Browser, Games: ParentAgentActivityReportSectionKind.Games, Network: ParentAgentActivityReportSectionKind.Network } as const;
export type ParentAgentActivitySurfaceReadModelKind = (typeof ParentAgentActivitySurfaceReadModelKindName)[keyof typeof ParentAgentActivitySurfaceReadModelKindName];
export type ParentAgentActivitySurfaceSchemaParser<T> = { readonly parse: (input: unknown) => T; readonly safeParse: (input: unknown) => { readonly success: true; readonly data: T } | { readonly success: false } };
export type ParentAgentActivityEvidenceRef = { readonly evidenceId: string; readonly kind: ParentAgentActivityEvidenceKind; readonly digest: string | null; readonly uri: string | null };
export type ParentAgentActivitySurfaceScope = { readonly scopeKind: ParentAgentActivitySurfaceScopeKind; readonly familyId: string | null; readonly deviceId: string | null };
export type ParentAgentActivitySurfaceRequest = { readonly schemaVersion: typeof ParentAgentActivitySurfaceSchemaVersion; readonly scope: ParentAgentActivitySurfaceScope; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string };
export type ParentAgentActivityReportSourceState = { readonly deviceId: string; readonly reachabilityState: ParentAgentActivityReportSourceReachabilityState; readonly state: ParentAgentActivityReadModelState; readonly reason: string | null; readonly lastUpdatedAt: string | null; readonly custodyLabel: ParentAgentActivityReportCustodyLabel; readonly sourceLabel: ParentAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type ParentAgentActivityReportSection = { readonly sectionKind: ParentAgentActivityReportSectionKind; readonly title: string; readonly state: ParentAgentActivityReadModelState; readonly summary: string; readonly itemCount: number; readonly evidence: readonly ParentAgentActivityEvidenceRef[] };
export type ParentAgentActivitySavedReportMetadata = { readonly reportId: string; readonly fileName: string; readonly savedState: ParentAgentActivitySavedReportState; readonly savedAt: string | null; readonly storageReason: string | null; readonly custodyLabel: ParentAgentActivityReportCustodyLabel; readonly sourceLabel: ParentAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type ParentAgentActivityReportSourceStateSummary = { readonly totalSources: number; readonly readySources: number; readonly offlineSources: number; readonly staleSources: number; readonly unavailableSources: number; readonly unreachableSources: number; readonly errorSources: number };
export type ParentAgentActivityReportDocument = { readonly schemaVersion: typeof ParentAgentActivitySurfaceSchemaVersion; readonly reportId: string; readonly frequency: ParentAgentActivityReportFrequency; readonly scope: ParentAgentActivitySurfaceScope; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string; readonly generatedAt: string; readonly savedMetadata: ParentAgentActivitySavedReportMetadata | null; readonly sourceStates: readonly ParentAgentActivityReportSourceState[]; readonly sections: readonly ParentAgentActivityReportSection[] };
export type ParentAgentActivityHistoricalReportListItem = { readonly schemaVersion: typeof ParentAgentActivitySurfaceSchemaVersion; readonly reportId: string; readonly fileName: string; readonly reportDate: string; readonly rangeStart: string; readonly rangeEnd: string; readonly summary: string; readonly savedState: ParentAgentActivitySavedReportState; readonly savedAt: string | null; readonly sourceStateSummary: ParentAgentActivityReportSourceStateSummary; readonly parsedReport: ParentAgentActivityReportDocument; readonly custodyLabel: ParentAgentActivityReportCustodyLabel; readonly sourceLabel: ParentAgentActivityReportSourceLabel; readonly rawChildEvidenceIncluded: boolean };
export type ParentAgentActivityHistoricalReportList = { readonly schemaVersion: typeof ParentAgentActivitySurfaceSchemaVersion; readonly request: ParentAgentActivitySurfaceRequest; readonly state: ParentAgentActivityReadModelState; readonly storageState: ParentAgentActivitySavedReportState; readonly storageReason: string | null; readonly reports: readonly ParentAgentActivityHistoricalReportListItem[] };
export type ParentAgentActivityAppGameSourceStatusRow = { readonly sourceKind: string; readonly state: ParentAgentActivityReadModelState; readonly rowCount: number; readonly lastObservedAt: string | null; readonly capabilityStatus: string; readonly evidence: readonly ParentAgentActivityEvidenceRef[] };
export type ParentAgentActivityTabReadModel<Row> = { readonly schemaVersion: typeof ParentAgentActivitySurfaceSchemaVersion; readonly request: ParentAgentActivitySurfaceRequest; readonly state: ParentAgentActivityReadModelState; readonly generatedAt: string; readonly summary: string; readonly rows: readonly Row[] };
export type ParentAgentActivityScreenReadModelRow = { readonly rowId: string; readonly label: string; readonly deviceId: string; readonly state: ParentAgentActivityReadModelState; readonly totalMs: number; readonly foregroundMs: number; readonly backgroundMs: number; readonly captureReason: string; readonly captureScope: string; readonly capabilityStatus: string; readonly queueJobId: string; readonly modelRuntimeRef: string; readonly modelId: string; readonly providerKind: string; readonly promptOrTemplateVersion: string; readonly primaryCategory: string | null; readonly confidence: number; readonly imageDeletionState: string; readonly rawImageRetained: boolean; readonly policyEligible: boolean; readonly imageDigest: string; readonly custodyState: string; readonly evidence: readonly ParentAgentActivityEvidenceRef[]; readonly policyDecisionRef: string | null; readonly policyAction: string | null; readonly policyReasonCodes: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly localModelRuntimeRefs: readonly string[]; readonly parentExplanationRefs: readonly string[]; readonly explanationReasons: readonly string[]; readonly deletionReasons: readonly string[]; readonly ocrTextSnippets: readonly string[]; readonly redactionNotes: readonly string[] };
export type ParentAgentActivityAppUseReadModelRow = { readonly rowId: string; readonly appName: string; readonly deviceId: string; readonly state: ParentAgentActivityReadModelState; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly launchCount: number; readonly inventoryRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly ParentAgentActivityAppGameSourceStatusRow[]; readonly evidence: readonly ParentAgentActivityEvidenceRef[] };
export type ParentAgentActivityBrowserReadModelRow = { readonly rowId: string; readonly domainLabel: string; readonly deviceId: string; readonly state: ParentAgentActivityReadModelState; readonly visitCount: number; readonly totalMs: number; readonly evidenceDigest: string | null };
export type ParentAgentActivityGamesReadModelRow = { readonly rowId: string; readonly displayName: string; readonly deviceId: string; readonly state: ParentAgentActivityReadModelState; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly sessionCount: number; readonly launcherRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly ParentAgentActivityAppGameSourceStatusRow[]; readonly evidence: readonly ParentAgentActivityEvidenceRef[] };
export type ParentAgentActivityNetworkReadModelRow = { readonly rowId: string; readonly destinationLabel: string; readonly deviceId: string; readonly state: ParentAgentActivityReadModelState; readonly connectionCount: number; readonly totalBytes: number; readonly evidenceDigest: string | null };
export type ParentAgentActivityScreenReadModel = ParentAgentActivityTabReadModel<ParentAgentActivityScreenReadModelRow>;
export type ParentAgentActivityAppUseReadModel = ParentAgentActivityTabReadModel<ParentAgentActivityAppUseReadModelRow>;
export type ParentAgentActivityBrowserReadModel = ParentAgentActivityTabReadModel<ParentAgentActivityBrowserReadModelRow>;
export type ParentAgentActivityGamesReadModel = ParentAgentActivityTabReadModel<ParentAgentActivityGamesReadModelRow>;
export type ParentAgentActivityNetworkReadModel = ParentAgentActivityTabReadModel<ParentAgentActivityNetworkReadModelRow>;
export type ParentAgentActivitySurfaceReadModel = ParentAgentActivityScreenReadModel | ParentAgentActivityAppUseReadModel | ParentAgentActivityBrowserReadModel | ParentAgentActivityGamesReadModel | ParentAgentActivityNetworkReadModel;
function __ParentAgentActivitySurfaceSchema<T>(decoder: (value: unknown) => T): ParentAgentActivitySurfaceSchemaParser<T> {
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
function __ParentAgentActivitySurfaceIsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __ParentAgentActivitySurfaceReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__ParentAgentActivitySurfaceIsRecord(value)) { throw new TypeError(`${label} must be an activity surface object`); } return value; }
function __ParentAgentActivitySurfaceReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string`); } return value; }
function __ParentAgentActivitySurfaceReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __ParentAgentActivitySurfaceReadOptionalNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === undefined || value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __ParentAgentActivitySurfaceReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be an activity surface boolean`); } return value; }
function __ParentAgentActivitySurfaceReadOptionalFalse(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (value === undefined) { return false; } if (value !== false) { throw new TypeError(`${field} must be false for activity surface redaction/custody boundary`); } return false; }
function __ParentAgentActivitySurfaceReadNonNegativeInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative activity surface integer`); } return value; }
function __ParentAgentActivitySurfaceReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be an activity surface confidence from 0 to 1`); } return value; }

function __ParentAgentActivitySurfaceReadSchemaVersion(record: Readonly<Record<string, unknown>>): typeof ParentAgentActivitySurfaceSchemaVersion { if (record['schemaVersion'] !== ParentAgentActivitySurfaceSchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned activity surface schema version'); } return ParentAgentActivitySurfaceSchemaVersion; }
function __ParentAgentActivitySurfaceReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __ParentAgentActivitySurfaceReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __ParentAgentActivitySurfaceReadOptionalLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[], fallback: T): T { const value = record[field]; if (value === undefined) { return fallback; } if (typeof value !== 'string' || !allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __ParentAgentActivitySurfaceReadArray<T>(record: Readonly<Record<string, unknown>>, field: string, decoder: (value: unknown) => T): readonly T[] { const value = record[field]; if (!Array.isArray(value)) { throw new TypeError(`${field} must be an activity surface array`); } return value.map(decoder); }
function __ParentAgentActivitySurfaceReadStringArrayValue(value: unknown, field: string): readonly string[] { if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be an activity surface string array`); } return value as readonly string[]; }
function __ParentAgentActivitySurfaceReadOptionalStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (value === undefined) { return []; } return __ParentAgentActivitySurfaceReadStringArrayValue(value, field); }
function __ParentAgentActivitySurfaceDecodeEvidenceRef(value: unknown): ParentAgentActivityEvidenceRef { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity evidence ref'); return { evidenceId: __ParentAgentActivitySurfaceReadString(record, 'evidenceId'), kind: __ParentAgentActivitySurfaceReadLiteral(record, 'kind', Object.values(ParentAgentActivityEvidenceKind)), digest: __ParentAgentActivitySurfaceReadNullableString(record, 'digest'), uri: __ParentAgentActivitySurfaceReadNullableString(record, 'uri') }; }
function __ParentAgentActivitySurfaceReadEvidenceArray(record: Readonly<Record<string, unknown>>, field: string): readonly ParentAgentActivityEvidenceRef[] { return __ParentAgentActivitySurfaceReadArray(record, field, __ParentAgentActivitySurfaceDecodeEvidenceRef); }
function __ParentAgentActivitySurfaceDecodeScope(value: unknown): ParentAgentActivitySurfaceScope { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity surface scope'); const scope = { scopeKind: __ParentAgentActivitySurfaceReadLiteral(record, 'scopeKind', Object.values(ParentAgentActivitySurfaceScopeKind)), familyId: __ParentAgentActivitySurfaceReadNullableString(record, 'familyId'), deviceId: __ParentAgentActivitySurfaceReadNullableString(record, 'deviceId') }; if (scope.scopeKind === ParentAgentActivitySurfaceScopeKind.Family && (scope.familyId === null || scope.deviceId !== null)) { throw new TypeError('family activity scope must include familyId only'); } if (scope.scopeKind === ParentAgentActivitySurfaceScopeKind.Device && (scope.familyId !== null || scope.deviceId === null)) { throw new TypeError('device activity scope must include deviceId only'); } return scope; }

function __ParentAgentActivitySurfaceDecodeRequest(value: unknown): ParentAgentActivitySurfaceRequest { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity surface request'); return { schemaVersion: __ParentAgentActivitySurfaceReadSchemaVersion(record), scope: __ParentAgentActivitySurfaceDecodeScope(record['scope']), requestedAt: __ParentAgentActivitySurfaceReadString(record, 'requestedAt'), rangeStart: __ParentAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __ParentAgentActivitySurfaceReadString(record, 'rangeEnd') }; }
function __ParentAgentActivitySurfaceDecodeSourceState(value: unknown): ParentAgentActivityReportSourceState { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity report source state'); return { deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), reachabilityState: __ParentAgentActivitySurfaceReadLiteral(record, 'reachabilityState', Object.values(ParentAgentActivityReportSourceReachabilityState)), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), reason: __ParentAgentActivitySurfaceReadNullableString(record, 'reason'), lastUpdatedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'lastUpdatedAt'), custodyLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(ParentAgentActivityReportCustodyLabel), ParentAgentActivityReportCustodyLabel.ChildDeviceLocalSummary), sourceLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(ParentAgentActivityReportSourceLabel), ParentAgentActivityReportSourceLabel.ActivityQueryStoreSummary), rawChildEvidenceIncluded: __ParentAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __ParentAgentActivitySurfaceDecodeSection(value: unknown): ParentAgentActivityReportSection { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity report section'); return { sectionKind: __ParentAgentActivitySurfaceReadLiteral(record, 'sectionKind', Object.values(ParentAgentActivityReportSectionKind)), title: __ParentAgentActivitySurfaceReadString(record, 'title'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), summary: __ParentAgentActivitySurfaceReadString(record, 'summary'), itemCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'itemCount'), evidence: __ParentAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __ParentAgentActivitySurfaceDecodeSavedMetadata(value: unknown): ParentAgentActivitySavedReportMetadata { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity saved report metadata'); return { reportId: __ParentAgentActivitySurfaceReadString(record, 'reportId'), fileName: __ParentAgentActivitySurfaceReadString(record, 'fileName'), savedState: __ParentAgentActivitySurfaceReadLiteral(record, 'savedState', Object.values(ParentAgentActivitySavedReportState)), savedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'savedAt'), storageReason: __ParentAgentActivitySurfaceReadNullableString(record, 'storageReason'), custodyLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(ParentAgentActivityReportCustodyLabel), ParentAgentActivityReportCustodyLabel.ParentDeviceLocalReportJson), sourceLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(ParentAgentActivityReportSourceLabel), ParentAgentActivityReportSourceLabel.SavedReportJson), rawChildEvidenceIncluded: __ParentAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __ParentAgentActivitySurfaceDecodeSourceStateSummary(value: unknown): ParentAgentActivityReportSourceStateSummary { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity report source state summary'); return { totalSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalSources'), readySources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'readySources'), offlineSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'offlineSources'), staleSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'staleSources'), unavailableSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'unavailableSources'), unreachableSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'unreachableSources'), errorSources: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'errorSources') }; }
function __ParentAgentActivitySurfaceDecodeReportDocument(value: unknown): ParentAgentActivityReportDocument { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity report document'); const savedMetadata = record['savedMetadata']; return { schemaVersion: __ParentAgentActivitySurfaceReadSchemaVersion(record), reportId: __ParentAgentActivitySurfaceReadString(record, 'reportId'), frequency: __ParentAgentActivitySurfaceReadLiteral(record, 'frequency', Object.values(ParentAgentActivityReportFrequency)), scope: __ParentAgentActivitySurfaceDecodeScope(record['scope']), requestedAt: __ParentAgentActivitySurfaceReadString(record, 'requestedAt'), rangeStart: __ParentAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __ParentAgentActivitySurfaceReadString(record, 'rangeEnd'), generatedAt: __ParentAgentActivitySurfaceReadString(record, 'generatedAt'), savedMetadata: savedMetadata === null ? null : __ParentAgentActivitySurfaceDecodeSavedMetadata(savedMetadata), sourceStates: __ParentAgentActivitySurfaceReadArray(record, 'sourceStates', __ParentAgentActivitySurfaceDecodeSourceState), sections: __ParentAgentActivitySurfaceReadArray(record, 'sections', __ParentAgentActivitySurfaceDecodeSection) }; }
function __ParentAgentActivitySurfaceDecodeHistoryItem(value: unknown): ParentAgentActivityHistoricalReportListItem { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity historical report list item'); return { schemaVersion: __ParentAgentActivitySurfaceReadSchemaVersion(record), reportId: __ParentAgentActivitySurfaceReadString(record, 'reportId'), fileName: __ParentAgentActivitySurfaceReadString(record, 'fileName'), reportDate: __ParentAgentActivitySurfaceReadString(record, 'reportDate'), rangeStart: __ParentAgentActivitySurfaceReadString(record, 'rangeStart'), rangeEnd: __ParentAgentActivitySurfaceReadString(record, 'rangeEnd'), summary: __ParentAgentActivitySurfaceReadString(record, 'summary'), savedState: __ParentAgentActivitySurfaceReadLiteral(record, 'savedState', Object.values(ParentAgentActivitySavedReportState)), savedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'savedAt'), sourceStateSummary: __ParentAgentActivitySurfaceDecodeSourceStateSummary(record['sourceStateSummary']), parsedReport: __ParentAgentActivitySurfaceDecodeReportDocument(record['parsedReport']), custodyLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'custodyLabel', Object.values(ParentAgentActivityReportCustodyLabel), ParentAgentActivityReportCustodyLabel.ParentDeviceLocalHistory), sourceLabel: __ParentAgentActivitySurfaceReadOptionalLiteral(record, 'sourceLabel', Object.values(ParentAgentActivityReportSourceLabel), ParentAgentActivityReportSourceLabel.SavedReportHistory), rawChildEvidenceIncluded: __ParentAgentActivitySurfaceReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __ParentAgentActivitySurfaceDecodeHistoricalReportList(value: unknown): ParentAgentActivityHistoricalReportList { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity historical report list'); return { schemaVersion: __ParentAgentActivitySurfaceReadSchemaVersion(record), request: __ParentAgentActivitySurfaceDecodeRequest(record['request']), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), storageState: __ParentAgentActivitySurfaceReadLiteral(record, 'storageState', Object.values(ParentAgentActivitySavedReportState)), storageReason: __ParentAgentActivitySurfaceReadNullableString(record, 'storageReason'), reports: __ParentAgentActivitySurfaceReadArray(record, 'reports', __ParentAgentActivitySurfaceDecodeHistoryItem) }; }
function __ParentAgentActivitySurfaceDecodeSourceStatusRow(value: unknown): ParentAgentActivityAppGameSourceStatusRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity app/game source status row'); return { sourceKind: __ParentAgentActivitySurfaceReadString(record, 'sourceKind'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), rowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'rowCount'), lastObservedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), capabilityStatus: __ParentAgentActivitySurfaceReadString(record, 'capabilityStatus'), evidence: __ParentAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }

function __ParentAgentActivitySurfaceDecodeReadModelBase(record: Readonly<Record<string, unknown>>) { return { schemaVersion: __ParentAgentActivitySurfaceReadSchemaVersion(record), request: __ParentAgentActivitySurfaceDecodeRequest(record['request']), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), generatedAt: __ParentAgentActivitySurfaceReadString(record, 'generatedAt'), summary: __ParentAgentActivitySurfaceReadString(record, 'summary') }; }
function __ParentAgentActivitySurfaceDecodeScreenRow(value: unknown): ParentAgentActivityScreenReadModelRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity screen read-model row'); return { rowId: __ParentAgentActivitySurfaceReadString(record, 'rowId'), label: __ParentAgentActivitySurfaceReadString(record, 'label'), deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), totalMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), foregroundMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundMs'), backgroundMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'backgroundMs'), captureReason: __ParentAgentActivitySurfaceReadString(record, 'captureReason'), captureScope: __ParentAgentActivitySurfaceReadString(record, 'captureScope'), capabilityStatus: __ParentAgentActivitySurfaceReadString(record, 'capabilityStatus'), queueJobId: __ParentAgentActivitySurfaceReadString(record, 'queueJobId'), modelRuntimeRef: __ParentAgentActivitySurfaceReadString(record, 'modelRuntimeRef'), modelId: __ParentAgentActivitySurfaceReadString(record, 'modelId'), providerKind: __ParentAgentActivitySurfaceReadString(record, 'providerKind'), promptOrTemplateVersion: __ParentAgentActivitySurfaceReadString(record, 'promptOrTemplateVersion'), primaryCategory: __ParentAgentActivitySurfaceReadNullableString(record, 'primaryCategory'), confidence: __ParentAgentActivitySurfaceReadConfidence(record, 'confidence'), imageDeletionState: __ParentAgentActivitySurfaceReadString(record, 'imageDeletionState'), rawImageRetained: __ParentAgentActivitySurfaceReadBoolean(record, 'rawImageRetained'), policyEligible: __ParentAgentActivitySurfaceReadBoolean(record, 'policyEligible'), imageDigest: __ParentAgentActivitySurfaceReadString(record, 'imageDigest'), custodyState: __ParentAgentActivitySurfaceReadString(record, 'custodyState'), evidence: __ParentAgentActivitySurfaceReadEvidenceArray(record, 'evidence'), policyDecisionRef: __ParentAgentActivitySurfaceReadOptionalNullableString(record, 'policyDecisionRef'), policyAction: __ParentAgentActivitySurfaceReadOptionalNullableString(record, 'policyAction'), policyReasonCodes: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'policyReasonCodes'), parentRuleRefs: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'parentRuleRefs'), localModelRuntimeRefs: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'localModelRuntimeRefs'), parentExplanationRefs: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'parentExplanationRefs'), explanationReasons: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'explanationReasons'), deletionReasons: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'deletionReasons'), ocrTextSnippets: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'ocrTextSnippets'), redactionNotes: __ParentAgentActivitySurfaceReadOptionalStringArray(record, 'redactionNotes') }; }
function __ParentAgentActivitySurfaceDecodeAppUseRow(value: unknown): ParentAgentActivityAppUseReadModelRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity app-use read-model row'); return { rowId: __ParentAgentActivitySurfaceReadString(record, 'rowId'), appName: __ParentAgentActivitySurfaceReadString(record, 'appName'), deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), productKind: __ParentAgentActivitySurfaceReadString(record, 'productKind'), classificationState: __ParentAgentActivitySurfaceReadString(record, 'classificationState'), inventoryState: __ParentAgentActivitySurfaceReadString(record, 'inventoryState'), runtimeState: __ParentAgentActivitySurfaceReadString(record, 'runtimeState'), foregroundState: __ParentAgentActivitySurfaceReadString(record, 'foregroundState'), capabilityStatus: __ParentAgentActivitySurfaceReadString(record, 'capabilityStatus'), lastObservedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), totalMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), launchCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'launchCount'), inventoryRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'inventoryRowCount'), runningRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __ParentAgentActivitySurfaceReadArray(record, 'sourceStatusRows', __ParentAgentActivitySurfaceDecodeSourceStatusRow), evidence: __ParentAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __ParentAgentActivitySurfaceDecodeBrowserRow(value: unknown): ParentAgentActivityBrowserReadModelRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity browser read-model row'); return { rowId: __ParentAgentActivitySurfaceReadString(record, 'rowId'), domainLabel: __ParentAgentActivitySurfaceReadString(record, 'domainLabel'), deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), visitCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'visitCount'), totalMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), evidenceDigest: __ParentAgentActivitySurfaceReadNullableString(record, 'evidenceDigest') }; }
function __ParentAgentActivitySurfaceDecodeGamesRow(value: unknown): ParentAgentActivityGamesReadModelRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity games read-model row'); return { rowId: __ParentAgentActivitySurfaceReadString(record, 'rowId'), displayName: __ParentAgentActivitySurfaceReadString(record, 'displayName'), deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), productKind: __ParentAgentActivitySurfaceReadString(record, 'productKind'), classificationState: __ParentAgentActivitySurfaceReadString(record, 'classificationState'), inventoryState: __ParentAgentActivitySurfaceReadString(record, 'inventoryState'), runtimeState: __ParentAgentActivitySurfaceReadString(record, 'runtimeState'), foregroundState: __ParentAgentActivitySurfaceReadString(record, 'foregroundState'), capabilityStatus: __ParentAgentActivitySurfaceReadString(record, 'capabilityStatus'), lastObservedAt: __ParentAgentActivitySurfaceReadNullableString(record, 'lastObservedAt'), totalMs: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalMs'), sessionCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'sessionCount'), launcherRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'launcherRowCount'), runningRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __ParentAgentActivitySurfaceReadArray(record, 'sourceStatusRows', __ParentAgentActivitySurfaceDecodeSourceStatusRow), evidence: __ParentAgentActivitySurfaceReadEvidenceArray(record, 'evidence') }; }
function __ParentAgentActivitySurfaceDecodeNetworkRow(value: unknown): ParentAgentActivityNetworkReadModelRow { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity network read-model row'); return { rowId: __ParentAgentActivitySurfaceReadString(record, 'rowId'), destinationLabel: __ParentAgentActivitySurfaceReadString(record, 'destinationLabel'), deviceId: __ParentAgentActivitySurfaceReadString(record, 'deviceId'), state: __ParentAgentActivitySurfaceReadLiteral(record, 'state', Object.values(ParentAgentActivityReadModelState)), connectionCount: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'connectionCount'), totalBytes: __ParentAgentActivitySurfaceReadNonNegativeInteger(record, 'totalBytes'), evidenceDigest: __ParentAgentActivitySurfaceReadNullableString(record, 'evidenceDigest') }; }
function __ParentAgentActivitySurfaceDecodeScreenReadModel(value: unknown): ParentAgentActivityScreenReadModel { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity screen read-model'); return { ...__ParentAgentActivitySurfaceDecodeReadModelBase(record), rows: __ParentAgentActivitySurfaceReadArray(record, 'rows', __ParentAgentActivitySurfaceDecodeScreenRow) }; }
function __ParentAgentActivitySurfaceDecodeAppUseReadModel(value: unknown): ParentAgentActivityAppUseReadModel { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity app-use read-model'); return { ...__ParentAgentActivitySurfaceDecodeReadModelBase(record), rows: __ParentAgentActivitySurfaceReadArray(record, 'rows', __ParentAgentActivitySurfaceDecodeAppUseRow) }; }
function __ParentAgentActivitySurfaceDecodeBrowserReadModel(value: unknown): ParentAgentActivityBrowserReadModel { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity browser read-model'); return { ...__ParentAgentActivitySurfaceDecodeReadModelBase(record), rows: __ParentAgentActivitySurfaceReadArray(record, 'rows', __ParentAgentActivitySurfaceDecodeBrowserRow) }; }
function __ParentAgentActivitySurfaceDecodeGamesReadModel(value: unknown): ParentAgentActivityGamesReadModel { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity games read-model'); return { ...__ParentAgentActivitySurfaceDecodeReadModelBase(record), rows: __ParentAgentActivitySurfaceReadArray(record, 'rows', __ParentAgentActivitySurfaceDecodeGamesRow) }; }
function __ParentAgentActivitySurfaceDecodeNetworkReadModel(value: unknown): ParentAgentActivityNetworkReadModel { const record = __ParentAgentActivitySurfaceReadRecord(value, 'activity network read-model'); return { ...__ParentAgentActivitySurfaceDecodeReadModelBase(record), rows: __ParentAgentActivitySurfaceReadArray(record, 'rows', __ParentAgentActivitySurfaceDecodeNetworkRow) }; }
export const ParentAgentActivityReadModelStateSchema = __ParentAgentActivitySurfaceSchema((value: unknown): ParentAgentActivityReadModelState => { if (typeof value !== 'string' || !(Object.values(ParentAgentActivityReadModelState) as readonly string[]).includes(value)) { throw new TypeError('activity read-model state is not Rust-owned'); } return value as ParentAgentActivityReadModelState; });
export const ParentAgentActivitySurfaceRequestSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeRequest);
export const ParentAgentActivityReportDocumentSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeReportDocument);
export const ParentAgentActivityHistoricalReportListSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeHistoricalReportList);
export const ParentAgentActivityScreenReadModelSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeScreenReadModel);
export const ParentAgentActivityAppUseReadModelSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeAppUseReadModel);
export const ParentAgentActivityBrowserReadModelSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeBrowserReadModel);
export const ParentAgentActivityGamesReadModelSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeGamesReadModel);
export const ParentAgentActivityNetworkReadModelSchema = __ParentAgentActivitySurfaceSchema(__ParentAgentActivitySurfaceDecodeNetworkReadModel);

export const ParentAgentActivitySurfaceAdapterOperationId = { GetDailyReport: "getDailyReport", GetWeeklyReport: "getWeeklyReport", GetMonthlyReport: "getMonthlyReport", SaveActivityReport: "saveActivityReport", ListHistoricalReports: "listHistoricalReports", GetScreenActivity: "getScreenActivity", GetAppUseActivity: "getAppUseActivity", GetBrowserActivity: "getBrowserActivity", GetGamesActivity: "getGamesActivity", GetNetworkActivity: "getNetworkActivity" } as const;
export const ParentAgentActivitySurfaceAdapterCommandBuilder = { ReportGenerate: "createActivityReportGenerateCommand", ReportSave: "createActivityReportSaveCommand", ReportHistory: "createActivityReportHistoryCommand", ReadModel: "createActivityReadModelCommand" } as const;
export const ParentAgentActivitySurfaceAdapterEventParser = { ReportDocument: "parseActivityReportDocumentEvent", ReportHistory: "parseActivityReportHistoryEvent", ReadModel: "parseActivityReadModelEvent" } as const;
export type ParentAgentActivitySurfaceAdapterFailureReason = "wrong-event" | "missing-json-field" | "invalid-json" | "invalid-payload";
export type ParentAgentActivitySurfaceAdapterResponseKind = "report-document" | "report-history" | "tab-read-model";
export type ParentAgentActivitySurfaceAdapterOperation = { readonly operationId: (typeof ParentAgentActivitySurfaceAdapterOperationId)[keyof typeof ParentAgentActivitySurfaceAdapterOperationId]; readonly command: ParentAgentCommandName; readonly successEvent: ParentAgentEventName; readonly payloadField: ParentAgentProtocolFieldName; readonly commandBuilder: (typeof ParentAgentActivitySurfaceAdapterCommandBuilder)[keyof typeof ParentAgentActivitySurfaceAdapterCommandBuilder]; readonly eventParser: (typeof ParentAgentActivitySurfaceAdapterEventParser)[keyof typeof ParentAgentActivitySurfaceAdapterEventParser]; readonly responseKind: ParentAgentActivitySurfaceAdapterResponseKind; readonly readModelKind: ParentAgentActivitySurfaceReadModelKind | null; readonly productDataOwner: "rust-service-read-model"; readonly uiConsumer: "c-owned-activity-ui"; readonly viteDataOwner: false; readonly supportsFamilyScope: boolean; readonly supportsDeviceScope: boolean; readonly failureState: "unavailable"; readonly failureReasons: readonly ParentAgentActivitySurfaceAdapterFailureReason[]; readonly unavailableState: "unavailable" };
function ParentAgentActivitySurfaceAdapterOperation(operationId: ParentAgentActivitySurfaceAdapterOperation["operationId"], command: ParentAgentCommandName, successEvent: ParentAgentEventName, payloadField: ParentAgentProtocolFieldName, responseKind: ParentAgentActivitySurfaceAdapterResponseKind, readModelKind: ParentAgentActivitySurfaceReadModelKind | null): ParentAgentActivitySurfaceAdapterOperation { const commandBuilder = operationId === ParentAgentActivitySurfaceAdapterOperationId.SaveActivityReport ? ParentAgentActivitySurfaceAdapterCommandBuilder.ReportSave : operationId === ParentAgentActivitySurfaceAdapterOperationId.ListHistoricalReports ? ParentAgentActivitySurfaceAdapterCommandBuilder.ReportHistory : readModelKind === null ? ParentAgentActivitySurfaceAdapterCommandBuilder.ReportGenerate : ParentAgentActivitySurfaceAdapterCommandBuilder.ReadModel; const eventParser = responseKind === "report-history" ? ParentAgentActivitySurfaceAdapterEventParser.ReportHistory : responseKind === "tab-read-model" ? ParentAgentActivitySurfaceAdapterEventParser.ReadModel : ParentAgentActivitySurfaceAdapterEventParser.ReportDocument; return { operationId, command, successEvent, payloadField, commandBuilder, eventParser, responseKind, readModelKind, productDataOwner: "rust-service-read-model", uiConsumer: "c-owned-activity-ui", viteDataOwner: false, supportsFamilyScope: true, supportsDeviceScope: true, failureState: "unavailable", failureReasons: ["wrong-event", "missing-json-field", "invalid-json", "invalid-payload"], unavailableState: "unavailable" }; }
export const ParentAgentActivitySurfaceAdapterOperationManifest = [ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetDailyReport, ParentAgentCommand.ActivityReportDailyGenerate, ParentAgentEvent.ActivityReportGenerated, ParentAgentProtocolField.ActivityReportDocument, "report-document", null), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetWeeklyReport, ParentAgentCommand.ActivityReportWeeklyGenerate, ParentAgentEvent.ActivityReportGenerated, ParentAgentProtocolField.ActivityReportDocument, "report-document", null), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetMonthlyReport, ParentAgentCommand.ActivityReportMonthlyGenerate, ParentAgentEvent.ActivityReportGenerated, ParentAgentProtocolField.ActivityReportDocument, "report-document", null), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.SaveActivityReport, ParentAgentCommand.ActivityReportSave, ParentAgentEvent.ActivityReportSaved, ParentAgentProtocolField.ActivityReportDocument, "report-document", null), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.ListHistoricalReports, ParentAgentCommand.ActivityReportHistoryList, ParentAgentEvent.ActivityReportHistoryReported, ParentAgentProtocolField.ActivityReports, "report-history", null), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetScreenActivity, ParentAgentCommand.ActivityScreenReadModelGet, ParentAgentEvent.ActivityScreenReadModelReported, ParentAgentProtocolField.ActivityReadModel, "tab-read-model", ParentAgentActivitySurfaceReadModelKindName.Screen), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetAppUseActivity, ParentAgentCommand.ActivityAppUseReadModelGet, ParentAgentEvent.ActivityAppUseReadModelReported, ParentAgentProtocolField.ActivityReadModel, "tab-read-model", ParentAgentActivitySurfaceReadModelKindName.AppUse), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetBrowserActivity, ParentAgentCommand.ActivityBrowserReadModelGet, ParentAgentEvent.ActivityBrowserReadModelReported, ParentAgentProtocolField.ActivityReadModel, "tab-read-model", ParentAgentActivitySurfaceReadModelKindName.Browser), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetGamesActivity, ParentAgentCommand.ActivityGamesReadModelGet, ParentAgentEvent.ActivityGamesReadModelReported, ParentAgentProtocolField.ActivityReadModel, "tab-read-model", ParentAgentActivitySurfaceReadModelKindName.Games), ParentAgentActivitySurfaceAdapterOperation(ParentAgentActivitySurfaceAdapterOperationId.GetNetworkActivity, ParentAgentCommand.ActivityNetworkReadModelGet, ParentAgentEvent.ActivityNetworkReadModelReported, ParentAgentProtocolField.ActivityReadModel, "tab-read-model", ParentAgentActivitySurfaceReadModelKindName.Network)] as const satisfies readonly ParentAgentActivitySurfaceAdapterOperation[];
 export const ParentAgentLanHouseholdActionKind = { Assign: "assign", Rename: "rename", Ignore: "ignore", Restore: "restore", Trust: "trust" } as const; export type ParentAgentLanHouseholdActionKind = (typeof ParentAgentLanHouseholdActionKind)[keyof typeof ParentAgentLanHouseholdActionKind]; export const ParentAgentLanIntentKind = { ConfigurationUpdate: "configuration-update" } as const; export type ParentAgentLanIntentKind = (typeof ParentAgentLanIntentKind)[keyof typeof ParentAgentLanIntentKind]; export const ParentAgentLanParentAuthority = { ActiveController: "active-controller" } as const; export type ParentAgentLanParentAuthority = (typeof ParentAgentLanParentAuthority)[keyof typeof ParentAgentLanParentAuthority]; export const ParentAgentLanDiscoveryEventKind = { InterfaceChanged: "interface-changed", ScanStarted: "scan-started", ScanFinished: "scan-finished", EvidenceFound: "evidence-found", DeviceFound: "device-found", DeviceUpdated: "device-updated", DeviceOnline: "device-online", DeviceOffline: "device-offline", AgentDiscovered: "agent-discovered", AgentConfirmed: "agent-confirmed", UnknownDetected: "unknown-detected" } as const; export type ParentAgentLanDiscoveryEventKind = (typeof ParentAgentLanDiscoveryEventKind)[keyof typeof ParentAgentLanDiscoveryEventKind]; export const ParentAgentLanHouseholdDeviceKindValues = ["mobile","desktop","laptop","tablet","router","unknown"] as const; export type ParentAgentLanHouseholdDeviceKind = (typeof ParentAgentLanHouseholdDeviceKindValues)[number]; export const ParentAgentLanHouseholdActionDeviceKindField = ParentAgentProtocolField.LanHouseholdActionDeviceKind; export function isParentRouteEventId(value: unknown): value is string { const suffix = ParentAgentProtocolDelimiter.EventIdSuffix; return typeof value === 'string' && Object.values(ParentAgentEvent).some((eventName) => value.startsWith(`${eventName}${suffix}`)) && value.length > value.indexOf(suffix) + suffix.length; }
export function decodeParentRouteEventId(value: unknown): string { if (!isParentRouteEventId(value)) { throw new TypeError('eventId must be a Rust-owned parent route event id'); } return value; }

export type ParentActivityMemoryGraphEntryStatus =
  | 'usable'
  | 'degraded'
  | 'stale'
  | 'rejected';

export const ParentActivityMemoryGraphEntryStatus = {
  Usable: 'usable',
  Degraded: 'degraded',
  Stale: 'stale',
  Rejected: 'rejected',
} as const;

export type ParentActivityMemoryGraphNodeKind =
  | 'child-profile'
  | 'device'
  | 'browser-url'
  | 'domain'
  | 'video'
  | 'app'
  | 'game'
  | 'activity-session';

export const ParentActivityMemoryGraphNodeKind = {
  ChildProfile: 'child-profile',
  Device: 'device',
  BrowserUrl: 'browser-url',
  Domain: 'domain',
  Video: 'video',
  App: 'app',
  Game: 'game',
  ActivitySession: 'activity-session',
} as const;

export type ParentActivityMemoryGraphEdgeKind =
  | 'visited'
  | 'watched'
  | 'played'
  | 'active-during'
  | 'performed-by-child'
  | 'derived-from-evidence';

export const ParentActivityMemoryGraphEdgeKind = {
  Visited: 'visited',
  Watched: 'watched',
  Played: 'played',
  ActiveDuring: 'active-during',
  PerformedByChild: 'performed-by-child',
  DerivedFromEvidence: 'derived-from-evidence',
} as const;

export type ParentActivityMemoryGraphQueryKind =
  | 'visited-urls'
  | 'played-games'
  | 'watched-videos'
  | 'activity-by-time-range'
  | 'explain-evidence';

export const ParentActivityMemoryGraphQueryKind = {
  VisitedUrls: 'visited-urls',
  PlayedGames: 'played-games',
  WatchedVideos: 'watched-videos',
  ActivityByTimeRange: 'activity-by-time-range',
  ExplainEvidence: 'explain-evidence',
} as const;

export interface ParentActivityMemoryGraphEvidenceReferenceSnapshot {
  readonly evidenceReferenceId: string;
  readonly kind: string;
  readonly observedAt: string;
}

export interface ParentActivityMemoryGraphParentActionReferenceSnapshot {
  readonly actionReferenceId: string;
  readonly actor: {
    readonly actorId: string;
    readonly role: string;
  };
  readonly policyVersion: string;
  readonly createdAt: string;
}

export interface ParentActivityMemoryGraphDeviceReferenceSnapshot {
  readonly deviceId: string;
  readonly childProfileId: string | null;
  readonly label: string;
  readonly platform: string;
}

export interface ParentActivityMemoryGraphChildProfileReferenceSnapshot {
  readonly childProfileId: string;
  readonly displayName: string;
}

export interface ParentActivityMemoryGraphTraceSnapshot {
  readonly entryStatus: ParentActivityMemoryGraphEntryStatus;
  readonly sourceEvidenceReferences: readonly ParentActivityMemoryGraphEvidenceReferenceSnapshot[];
  readonly sourcePolicyVersion: string | null;
  readonly sourceParentActionReferences: readonly ParentActivityMemoryGraphParentActionReferenceSnapshot[];
  readonly generatedAt: string;
  readonly expiresAt: string | null;
  readonly confidence: number;
  readonly derivedIndexVersion: string;
  readonly degradedReasons: readonly string[];
}

export interface ParentActivityMemoryGraphTimeRangeSnapshot {
  readonly observedFrom: string;
  readonly observedUntil: string;
}

export interface ParentActivityMemoryGraphNodeSnapshot {
  readonly graphId: string;
  readonly nodeId: string;
  readonly nodeKind: ParentActivityMemoryGraphNodeKind;
  readonly label: string;
  readonly childProfile: ParentActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: ParentActivityMemoryGraphDeviceReferenceSnapshot | null;
  readonly trace: ParentActivityMemoryGraphTraceSnapshot;
}

export interface ParentActivityMemoryGraphEdgeSnapshot {
  readonly graphId: string;
  readonly edgeId: string;
  readonly edgeKind: ParentActivityMemoryGraphEdgeKind;
  readonly fromNodeId: string;
  readonly toNodeId: string;
  readonly observedFrom: string;
  readonly observedUntil: string | null;
  readonly durationMs: number | null;
  readonly trace: ParentActivityMemoryGraphTraceSnapshot;
}

export interface ParentActivityMemoryGraphQuerySnapshot {
  readonly queryId: string;
  readonly queryKind: ParentActivityMemoryGraphQueryKind;
  readonly childProfile: ParentActivityMemoryGraphChildProfileReferenceSnapshot | null;
  readonly device: ParentActivityMemoryGraphDeviceReferenceSnapshot;
  readonly timeRange: ParentActivityMemoryGraphTimeRangeSnapshot;
  readonly asOf: string;
  readonly limit: number;
}

export interface ParentActivityMemoryGraphReadModelSnapshot {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly custody: string;
  readonly capabilityStatus: string;
  readonly query: ParentActivityMemoryGraphQuerySnapshot;
  readonly readAt: string;
  readonly nodes: readonly ParentActivityMemoryGraphNodeSnapshot[];
  readonly edges: readonly ParentActivityMemoryGraphEdgeSnapshot[];
  readonly returnedNodeCount: number;
  readonly returnedEdgeCount: number;
  readonly omittedEdgeCount: number;
  readonly degradedReasons: readonly string[];
}

export type ParentActivityMemoryGraphNodeId =
  ParentActivityMemoryGraphNodeSnapshot['nodeId'];

export function decodeParentActivityMemoryGraphDigest(
  digest: string
): ParentActivityMemoryGraphReadModelSnapshot | null {
  try {
    return decodeParentActivityMemoryGraphReadModelSnapshot(JSON.parse(digest) as unknown);
  } catch {
    return null;
  }
}

export function decodeParentActivityMemoryGraphReadModelSnapshot(
  value: unknown
): ParentActivityMemoryGraphReadModelSnapshot | null {
  return isParentActivityMemoryGraphReadModelSnapshot(value) ? value : null;
}

function isParentActivityMemoryGraphReadModelSnapshot(
  value: unknown
): value is ParentActivityMemoryGraphReadModelSnapshot {
  if (!isParentActivityMemoryGraphRecord(value)) {
    return false;
  }
  const nodes = value['nodes'];
  const edges = value['edges'];
  return (
    isParentActivityMemoryGraphReadModelSnapshotMetadata(value) &&
    isParentActivityMemoryGraphReadModelSnapshotCollections(
      value,
      nodes,
      edges
    )
  );
}

function isParentActivityMemoryGraphReadModelSnapshotMetadata(
  value: Record<string, unknown>
): boolean {
  return (
    isParentActivityMemoryGraphNonNegativeInteger(value['schemaVersion']) &&
    isParentActivityMemoryGraphString(value['generatedAt']) &&
    isParentActivityMemoryGraphString(value['custody']) &&
    isParentActivityMemoryGraphString(value['capabilityStatus']) &&
    isParentActivityMemoryGraphQuerySnapshot(value['query']) &&
    isParentActivityMemoryGraphString(value['readAt']) &&
    isParentActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function isParentActivityMemoryGraphReadModelSnapshotCollections(
  value: Record<string, unknown>,
  nodes: unknown,
  edges: unknown
): boolean {
  return (
    Array.isArray(nodes) &&
    nodes.every(isParentActivityMemoryGraphNodeSnapshot) &&
    Array.isArray(edges) &&
    edges.every(isParentActivityMemoryGraphEdgeSnapshot) &&
    isParentActivityMemoryGraphNonNegativeInteger(value['returnedNodeCount']) &&
    value['returnedNodeCount'] === nodes.length &&
    isParentActivityMemoryGraphNonNegativeInteger(value['returnedEdgeCount']) &&
    value['returnedEdgeCount'] === edges.length &&
    isParentActivityMemoryGraphNonNegativeInteger(value['omittedEdgeCount'])
  );
}

function isParentActivityMemoryGraphQuerySnapshot(
  value: unknown
): value is ParentActivityMemoryGraphQuerySnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['queryId']) &&
    isParentActivityMemoryGraphQueryKind(value['queryKind']) &&
    isParentActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    isParentActivityMemoryGraphDevice(value['device']) &&
    isParentActivityMemoryGraphTimeRange(value['timeRange']) &&
    isParentActivityMemoryGraphString(value['asOf']) &&
    isParentActivityMemoryGraphNonNegativeInteger(value['limit'])
  );
}

function isParentActivityMemoryGraphNodeSnapshot(
  value: unknown
): value is ParentActivityMemoryGraphNodeSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['graphId']) &&
    isParentActivityMemoryGraphString(value['nodeId']) &&
    isParentActivityMemoryGraphNodeKind(value['nodeKind']) &&
    isParentActivityMemoryGraphString(value['label']) &&
    isParentActivityMemoryGraphNullableChildProfile(value['childProfile']) &&
    isParentActivityMemoryGraphNullableDevice(value['device']) &&
    isParentActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function isParentActivityMemoryGraphEdgeSnapshot(
  value: unknown
): value is ParentActivityMemoryGraphEdgeSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['graphId']) &&
    isParentActivityMemoryGraphString(value['edgeId']) &&
    isParentActivityMemoryGraphEdgeKind(value['edgeKind']) &&
    isParentActivityMemoryGraphString(value['fromNodeId']) &&
    isParentActivityMemoryGraphString(value['toNodeId']) &&
    isParentActivityMemoryGraphString(value['observedFrom']) &&
    isParentActivityMemoryGraphNullableString(value['observedUntil']) &&
    isParentActivityMemoryGraphNullableCount(value['durationMs']) &&
    isParentActivityMemoryGraphTraceSnapshot(value['trace'])
  );
}

function isParentActivityMemoryGraphTraceSnapshot(
  value: unknown
): value is ParentActivityMemoryGraphTraceSnapshot {
  if (!isParentActivityMemoryGraphRecord(value)) {
    return false;
  }
  const evidenceRefs = value['sourceEvidenceReferences'];
  const parentActionRefs = value['sourceParentActionReferences'];
  return (
    isParentActivityMemoryGraphTraceSnapshotMetadata(
      value,
      evidenceRefs,
      parentActionRefs
    )
  );
}

function isParentActivityMemoryGraphTraceSnapshotMetadata(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    isParentActivityMemoryGraphTraceSnapshotReferencesValid(
      value,
      evidenceRefs,
      parentActionRefs
    ) &&
    isParentActivityMemoryGraphTraceSnapshotFieldsValid(value)
  );
}

function isParentActivityMemoryGraphTraceSnapshotReferencesValid(
  value: Record<string, unknown>,
  evidenceRefs: unknown,
  parentActionRefs: unknown
): boolean {
  return (
    isParentActivityMemoryGraphEntryStatus(value['entryStatus']) &&
    Array.isArray(evidenceRefs) &&
    evidenceRefs.every(isParentActivityMemoryGraphEvidenceReference) &&
    isParentActivityMemoryGraphNullableString(value['sourcePolicyVersion']) &&
    Array.isArray(parentActionRefs) &&
    parentActionRefs.every(isParentActivityMemoryGraphParentActionReference) &&
    (evidenceRefs.length > 0 || value['sourcePolicyVersion'] !== null || parentActionRefs.length > 0)
  );
}

function isParentActivityMemoryGraphTraceSnapshotFieldsValid(
  value: Record<string, unknown>
): boolean {
  return (
    isParentActivityMemoryGraphString(value['generatedAt']) &&
    isParentActivityMemoryGraphNullableString(value['expiresAt']) &&
    typeof value['confidence'] === 'number' &&
    value['confidence'] >= 0 &&
    isParentActivityMemoryGraphString(value['derivedIndexVersion']) &&
    isParentActivityMemoryGraphStringArray(value['degradedReasons'])
  );
}

function isParentActivityMemoryGraphEvidenceReference(
  value: unknown
): value is ParentActivityMemoryGraphEvidenceReferenceSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['evidenceReferenceId']) &&
    isParentActivityMemoryGraphString(value['kind']) &&
    isParentActivityMemoryGraphString(value['observedAt'])
  );
}

function isParentActivityMemoryGraphParentActionReference(
  value: unknown
): value is ParentActivityMemoryGraphParentActionReferenceSnapshot {
  if (!isParentActivityMemoryGraphRecord(value)) {
    return false;
  }
  const actor = value['actor'];
  return (
    isParentActivityMemoryGraphString(value['actionReferenceId']) &&
    isParentActivityMemoryGraphRecord(actor) &&
    isParentActivityMemoryGraphString(actor['actorId']) &&
    isParentActivityMemoryGraphString(actor['role']) &&
    isParentActivityMemoryGraphString(value['policyVersion']) &&
    isParentActivityMemoryGraphString(value['createdAt'])
  );
}

function isParentActivityMemoryGraphDevice(
  value: unknown
): value is ParentActivityMemoryGraphDeviceReferenceSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['deviceId']) &&
    isParentActivityMemoryGraphNullableString(value['childProfileId']) &&
    isParentActivityMemoryGraphString(value['label']) &&
    isParentActivityMemoryGraphString(value['platform'])
  );
}

function isParentActivityMemoryGraphChildProfile(
  value: unknown
): value is ParentActivityMemoryGraphChildProfileReferenceSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['childProfileId']) &&
    isParentActivityMemoryGraphString(value['displayName'])
  );
}

function isParentActivityMemoryGraphTimeRange(
  value: unknown
): value is ParentActivityMemoryGraphTimeRangeSnapshot {
  return (
    isParentActivityMemoryGraphRecord(value) &&
    isParentActivityMemoryGraphString(value['observedFrom']) &&
    isParentActivityMemoryGraphString(value['observedUntil'])
  );
}

function isParentActivityMemoryGraphNullableChildProfile(
  value: unknown
): value is ParentActivityMemoryGraphChildProfileReferenceSnapshot | null {
  return value === null || isParentActivityMemoryGraphChildProfile(value);
}

function isParentActivityMemoryGraphNullableDevice(
  value: unknown
): value is ParentActivityMemoryGraphDeviceReferenceSnapshot | null {
  return value === null || isParentActivityMemoryGraphDevice(value);
}

function isParentActivityMemoryGraphNullableString(value: unknown): value is string | null {
  return value === null || isParentActivityMemoryGraphString(value);
}

function isParentActivityMemoryGraphNullableCount(value: unknown): value is number | null {
  return value === null || isParentActivityMemoryGraphNonNegativeInteger(value);
}

function isParentActivityMemoryGraphNodeKind(
  value: unknown
): value is ParentActivityMemoryGraphNodeKind {
  return (
    typeof value === 'string' &&
    Object.values(ParentActivityMemoryGraphNodeKind).includes(
      value as ParentActivityMemoryGraphNodeKind
    )
  );
}

function isParentActivityMemoryGraphEdgeKind(
  value: unknown
): value is ParentActivityMemoryGraphEdgeKind {
  return (
    typeof value === 'string' &&
    Object.values(ParentActivityMemoryGraphEdgeKind).includes(
      value as ParentActivityMemoryGraphEdgeKind
    )
  );
}

function isParentActivityMemoryGraphEntryStatus(
  value: unknown
): value is ParentActivityMemoryGraphEntryStatus {
  return (
    typeof value === 'string' &&
    Object.values(ParentActivityMemoryGraphEntryStatus).includes(
      value as ParentActivityMemoryGraphEntryStatus
    )
  );
}

function isParentActivityMemoryGraphQueryKind(
  value: unknown
): value is ParentActivityMemoryGraphQueryKind {
  return (
    typeof value === 'string' &&
    Object.values(ParentActivityMemoryGraphQueryKind).includes(
      value as ParentActivityMemoryGraphQueryKind
    )
  );
}

function isParentActivityMemoryGraphNonNegativeInteger(value: unknown): value is number {
  return typeof value === 'number' && Number.isInteger(value) && value >= 0;
}

function isParentActivityMemoryGraphStringArray(value: unknown): value is readonly string[] {
  return Array.isArray(value) && value.every(isParentActivityMemoryGraphString);
}

function isParentActivityMemoryGraphString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function isParentActivityMemoryGraphRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

export const ParentBridgeCommand = {
  LoadRoute: 'parent_load_route',
  Dispatch: 'parent_dispatch',
  Subscribe: 'parent_subscribe_route',
  Unsubscribe: 'parent_unsubscribe_route',
} as const;

export type ParentBridgeCommandName = (typeof ParentBridgeCommand)[keyof typeof ParentBridgeCommand];

export const ParentDevBridgeRoute = {
  LoadRoute: 'load-route',
  Dispatch: 'dispatch',
} as const;

export type ParentDevBridgeRouteName = (typeof ParentDevBridgeRoute)[keyof typeof ParentDevBridgeRoute];

export const ParentUiActionPayloadField = {
  PolicyPreviewAuthoringDraft: 'policyPreviewAuthoringDraft',
  PolicyPreviewAuthoringHandle: 'policyPreviewAuthoringHandle',
  PolicyRequestAssistantPreviewConfirmRequest: 'policyRequestAssistantPreviewConfirmRequest',
  PolicyRequestParentResolutionRequest: 'policyRequestParentResolutionRequest',
  ScreenSettingsRequest: 'screenSettingsRequest',
  ScreenSettingsResponse: 'screenSettingsResponse',
  ScreenSettingsUpdateKind: 'screenSettingsUpdateKind',
} as const;

export const ParentScreenSettingsCommandRuntime = { SchemaVersion: 1, RequestIdPrefix: 'screen-settings-request-' } as const;

export const ParentScreenSettingsUpdateKind = { Get: 'get', Replace: 'replace' } as const;

export type ParentScreenSettingsUpdateKind =
  (typeof ParentScreenSettingsUpdateKind)[keyof typeof ParentScreenSettingsUpdateKind];
export type ParentScreenSettingsServiceRequestId =
  `${typeof ParentScreenSettingsCommandRuntime.RequestIdPrefix}${number}`;

export const ParentHostBridgeRuntime = {
  SchemaVersion: 1,
  DevRouteSubscriptionPollMs: 1000,
  DevBridgeRequestTimeoutMs: 5000,
  RouteHashPrefix: '#',
  RouteHashQuerySeparator: '?',
  RouteSubscriptionEventPrefix: 'parent-route-subscription-',
  UrlPathSeparator: '/',
  PostMethod: 'POST',
  JsonContentTypeHeader: 'content-type',
  JsonContentType: 'application/json',
  StringType: 'string',
  TypeofUndefined: 'undefined',
  DevBridgeUrlEnvKey: 'VITE_PARENT_DEV_BRIDGE_URL',
  TauriCoreModule: '@tauri-apps/api/core',
  TauriEventModule: '@tauri-apps/api/event',
  TauriInternalWindowKey: '__TAURI_INTERNALS__',
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

export type ParentRouteSubscriptionId = Parameters<typeof parentRouteSubscriptionEventName>[0];export type ParentRouteSubscriptionEventName = ReturnType<typeof parentRouteSubscriptionEventName>;
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
}export interface ParentActivityEvidenceRefSnapshot {
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
  readonly aiAuditRef?: string | null;
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

export interface ParentPolicyPreviewConfirmationContext {
  readonly requestId?: string | null;
  readonly submissionKey?: string | null;
  readonly householdId?: string | null;
  readonly childProfileId?: string | null;
  readonly deviceId?: string | null;
  readonly sourceDocumentId?: string | null;
  readonly policyVersion?: number | null;
  readonly targetReferenceId?: string | null;
  readonly ruleId?: string | null;
  readonly requestedAt?: string | null;
  readonly expiresAt?: string | null;
  readonly assistantPreviewId?: string | null;
  readonly auditReferenceIds?: string | null;
  readonly actorId?: string | null;
  readonly actorRole?: string | null;
  readonly actorState?: string | null;
  readonly confirmationAuditReferenceId?: string | null;
}

export interface ParentPolicyPreviewReadModelSnapshot {  readonly schemaVersion?: string | null;
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
  readonly confirmationContext?: ParentPolicyPreviewConfirmationContext | null;
}

export interface ParentRouteLiveActivitySnapshot {
  readonly recentSummary?: ParentUnknownRecord | null;
  readonly ingestStatus?: ParentUnknownRecord | null;
  readonly activityScreenReadModel?: ParentUnknownRecord | null;
  readonly activityAppUseReadModel?: ParentUnknownRecord | null;
  readonly activityBrowserReadModel?: ParentUnknownRecord | null;
  readonly activityGamesReadModel?: ParentUnknownRecord | null;
  readonly screenSummaryPanel?: ParentScreenSummaryPanelSnapshot | null;
  readonly browserInventoryEvent?: ParentRouteEventSnapshot | null;
  readonly browserInventoryReadModel?: ParentUnknownRecord | null;
  readonly browserEvidenceEvent?: ParentRouteEventSnapshot | null;
  readonly browserEvidenceReadModel?: ParentUnknownRecord | null;
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

export interface ParentPolicyPreviewActionSnapshot {
  readonly action: ParentUiActionKind;
  readonly label: string;
  readonly payload?: ParentUiActionPayload | null;
}

export interface ParentPolicyPreviewAuthoringSnapshot {
  readonly targetValue: string;
  readonly requestedAction: string;
  readonly stageAction: ParentPolicyPreviewActionSnapshot;
  readonly confirmAction?: ParentPolicyPreviewActionSnapshot | null;
  readonly cancelAction: ParentPolicyPreviewActionSnapshot;
}

export interface ParentPolicyPreviewPanelSnapshot {
  readonly title: string;
  readonly body: string;
  readonly summary: string;
  readonly summaryDetails: readonly ParentPolicyPreviewPanelDetailSnapshot[];
  readonly cards: readonly ParentPolicyPreviewPanelCardSnapshot[];
  readonly emptyMessage: string;
  readonly productClaim: string;
  readonly authoring?: ParentPolicyPreviewAuthoringSnapshot | null;
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

export interface ParentScreenSummaryPanelDetailSnapshot {  readonly label: string;
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

export type ParentServiceHealthState = 'ready' | 'degraded' | 'unavailable';
export type ParentServiceHealthRoute = 'localhost' | 'local-network' | 'cloud-relay';
export type ParentServiceHealthTransport = 'websocket';
export type ParentServiceHealthAuthenticationState = 'unauthenticated' | 'unavailable';
export type ParentServiceHealthReason =
  | 'ready'
  | 'transport-unavailable'
  | 'route-dependency-unavailable'
  | 'response-schema-mismatch'
  | 'response-identity-mismatch'
  | 'response-payload-mismatch'
  | 'response-nonce-mismatch'
  | 'response-event-id-mismatch'
  | 'response-timestamp-missing'
  | 'response-timestamp-stale'
  | 'service-version-missing';

export interface ParentServiceHealthTraceSnapshot {
  readonly requestId?: string | null;
  readonly correlationId?: string | null;
  readonly responseEventId?: string | null;
  readonly requestSentAt?: string | null;
  readonly responseSentAt?: string | null;
}

export interface ParentServiceHealthSnapshot {
  readonly state: ParentServiceHealthState;
  readonly route?: ParentServiceHealthRoute | null;
  readonly protocolSchemaVersion?: number | null;
  readonly serviceVersion?: string | null;
  readonly transport?: ParentServiceHealthTransport | null;
  readonly authenticationState: ParentServiceHealthAuthenticationState;
  readonly reason: ParentServiceHealthReason;
  readonly trace: ParentServiceHealthTraceSnapshot;
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
  readonly serviceHealth?: ParentServiceHealthSnapshot | null;
  readonly diagnosticPanelsEnabled: boolean;
  readonly parentPortalRows?: readonly ParentPortalRowSnapshot[] | null;
  readonly parentPortalShellStatus?: ParentPortalShellStatusSnapshot | null;
  readonly liveActivity?: ParentRouteLiveActivitySnapshot | null;
  readonly browserPanels?: ParentRouteBrowserPanelsSnapshot | null;
  readonly setupFirstRunPanel?: ParentSetupFirstRunPanelSnapshot | null;
  readonly screenSettingsServiceResponse?: ParentUnknownRecord | null;
}export type ParentChildDeviceId = NonNullable<ParentRouteContext['selectedChildDeviceId']>;
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
  | 'policy-preview-authoring-draft-staged'
  | 'policy-preview-authoring-draft-cancelled'
  | 'policy-request-assistant-preview-confirm-requested'
  | 'policy-request-parent-resolution-requested'
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
  PolicyPreviewAuthoringDraftStaged: 'policy-preview-authoring-draft-staged',
  PolicyPreviewAuthoringDraftCancelled: 'policy-preview-authoring-draft-cancelled',
  PolicyRequestAssistantPreviewConfirmRequested: 'policy-request-assistant-preview-confirm-requested',
  PolicyRequestParentResolutionRequested: 'policy-request-parent-resolution-requested',
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
