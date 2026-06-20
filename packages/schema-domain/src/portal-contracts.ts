import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const PortalRouteLiteral = {
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
  SettingsRules: 'settings-rules',
  FrameTuner: 'app-layout',
  Commands: 'commands',
  Events: 'events',
  Logs: 'logs',
} as const;

export const PortalRouteSchema = withParser(
  Schema.Literal(
    ...Object.values(PortalRouteLiteral) as [
      typeof PortalRouteLiteral[keyof typeof PortalRouteLiteral],
      ...Array<typeof PortalRouteLiteral[keyof typeof PortalRouteLiteral]>
    ]
  )
);
export type PortalRoute = Infer<typeof PortalRouteSchema>;

export const PortalRoute = {
  Overview: PortalRouteSchema.parse(PortalRouteLiteral.Overview),
  Assistant: PortalRouteSchema.parse(PortalRouteLiteral.Assistant),
  Start: PortalRouteSchema.parse(PortalRouteLiteral.Start),
  Activity: PortalRouteSchema.parse(PortalRouteLiteral.Activity),
  Browser: PortalRouteSchema.parse(PortalRouteLiteral.Browser),
  BrowserSettings: PortalRouteSchema.parse(PortalRouteLiteral.BrowserSettings),
  Policy: PortalRouteSchema.parse(PortalRouteLiteral.Policy),
  PolicyApps: PortalRouteSchema.parse(PortalRouteLiteral.PolicyApps),
  PolicyGames: PortalRouteSchema.parse(PortalRouteLiteral.PolicyGames),
  PolicyScreen: PortalRouteSchema.parse(PortalRouteLiteral.PolicyScreen),
  PolicyNetwork: PortalRouteSchema.parse(PortalRouteLiteral.PolicyNetwork),
  PolicyTracking: PortalRouteSchema.parse(PortalRouteLiteral.PolicyTracking),
  PolicyRemoteScreen: PortalRouteSchema.parse(PortalRouteLiteral.PolicyRemoteScreen),
  RuleManagement: PortalRouteSchema.parse(PortalRouteLiteral.RuleManagement),
  Schedules: PortalRouteSchema.parse(PortalRouteLiteral.Schedules),
  Approvals: PortalRouteSchema.parse(PortalRouteLiteral.Approvals),
  Enforcement: PortalRouteSchema.parse(PortalRouteLiteral.Enforcement),
  PrivacyDesign: PortalRouteSchema.parse(PortalRouteLiteral.PrivacyDesign),
  Memory: PortalRouteSchema.parse(PortalRouteLiteral.Memory),
  MemorySettings: PortalRouteSchema.parse(PortalRouteLiteral.MemorySettings),
  AiGuide: PortalRouteSchema.parse(PortalRouteLiteral.AiGuide),
  AiRuntime: PortalRouteSchema.parse(PortalRouteLiteral.AiRuntime),
  ApiProviders: PortalRouteSchema.parse(PortalRouteLiteral.ApiProviders),
  ReportsGuide: PortalRouteSchema.parse(PortalRouteLiteral.ReportsGuide),
  ScreenAnalysis: PortalRouteSchema.parse(PortalRouteLiteral.ScreenAnalysis),
  AppGameSessions: PortalRouteSchema.parse(PortalRouteLiteral.AppGameSessions),
  NetworkActivity: PortalRouteSchema.parse(PortalRouteLiteral.NetworkActivity),
  Devices: PortalRouteSchema.parse(PortalRouteLiteral.Devices),
  LanPairing: PortalRouteSchema.parse(PortalRouteLiteral.LanPairing),
  CapabilityStatus: PortalRouteSchema.parse(PortalRouteLiteral.CapabilityStatus),
  Notifications: PortalRouteSchema.parse(PortalRouteLiteral.Notifications),
  NotificationChannels: PortalRouteSchema.parse(PortalRouteLiteral.NotificationChannels),
  DriveConnections: PortalRouteSchema.parse(PortalRouteLiteral.DriveConnections),
  ExportRetention: PortalRouteSchema.parse(PortalRouteLiteral.ExportRetention),
  RemoteAccess: PortalRouteSchema.parse(PortalRouteLiteral.RemoteAccess),
  ReportCompiler: PortalRouteSchema.parse(PortalRouteLiteral.ReportCompiler),
  AuditHistory: PortalRouteSchema.parse(PortalRouteLiteral.AuditHistory),
  Subscription: PortalRouteSchema.parse(PortalRouteLiteral.Subscription),
  Entitlements: PortalRouteSchema.parse(PortalRouteLiteral.Entitlements),
  PlatformsInstall: PortalRouteSchema.parse(PortalRouteLiteral.PlatformsInstall),
  InstallUpdates: PortalRouteSchema.parse(PortalRouteLiteral.InstallUpdates),
  Diagnostics: PortalRouteSchema.parse(PortalRouteLiteral.Diagnostics),
  SettingsRules: PortalRouteSchema.parse(PortalRouteLiteral.SettingsRules),
  FrameTuner: PortalRouteSchema.parse(PortalRouteLiteral.FrameTuner),
  Commands: PortalRouteSchema.parse(PortalRouteLiteral.Commands),
  Events: PortalRouteSchema.parse(PortalRouteLiteral.Events),
  Logs: PortalRouteSchema.parse(PortalRouteLiteral.Logs),
} as const;

export const PortalRouteHashPrefix = '#/' as const;
export const PortalRouteHashQuerySeparator = '?' as const;
export type PortalRouteHashPath = `${typeof PortalRouteHashPrefix}${PortalRoute}`;
export type PortalRouteHashQueryPath = `${typeof PortalRouteHashPrefix}${PortalRoute}${typeof PortalRouteHashQuerySeparator}${string}`;

export const PortalConnectionStateSchema = withParser(
  Schema.Literal('disconnected', 'connecting', 'connected', 'error')
);
export type PortalConnectionState = Infer<typeof PortalConnectionStateSchema>;
export const PortalConnectionState = {
  Disconnected: PortalConnectionStateSchema.parse('disconnected'),
  Connecting: PortalConnectionStateSchema.parse('connecting'),
  Connected: PortalConnectionStateSchema.parse('connected'),
  Error: PortalConnectionStateSchema.parse('error'),
} as const;

export const PortalDevToolUrlSchema = brandedNonEmptyStringSchema('PortalDevToolUrl');
export type PortalDevToolUrl = typeof PortalDevToolUrlSchema.Type;
