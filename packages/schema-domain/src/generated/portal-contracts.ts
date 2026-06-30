/* generated from crates/schema/src/parent_ui_bridge.rs */

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

export type GeneratedPortalRouteEventRole = 'portal' | 'agent-service' | 'cloud-relay';
export type GeneratedPortalRouteEventPayloadRecord = Readonly<Record<string, unknown>>;

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

export type GeneratedPortalDevToolUrl = string;
export type GeneratedPortalDetailValue = string;
export type GeneratedPortalClipboardText = string;
export type GeneratedTrackingStatusProofArtifact = string;
