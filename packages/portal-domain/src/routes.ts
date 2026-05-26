import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const PortalRouteSchema = withParser(
  Schema.Literal(
    'overview',
    'assistant',
    'start',
    'activity',
    'browser',
    'browser-settings',
    'policy',
    'rule-management',
    'schedules',
    'approvals',
    'enforcement',
    'privacy-design',
    'memory',
    'memory-settings',
    'ai-guide',
    'ai-runtime',
    'api-providers',
    'reports-guide',
    'report-settings',
    'screen-analysis',
    'app-game-sessions',
    'network-activity',
    'devices',
    'lan-pairing',
    'capability-status',
    'notifications',
    'notification-channels',
    'drive-connections',
    'export-retention',
    'remote-access',
    'report-compiler',
    'audit-history',
    'subscription',
    'entitlements',
    'platforms-install',
    'install-updates',
    'diagnostics',
    'settings-rules',
    'app-layout',
    'frame-tuner',
    'commands',
    'events'
  )
);
export type PortalRoute = Infer<typeof PortalRouteSchema>;

export const PortalRoute = {
  Overview: PortalRouteSchema.parse('overview'),
  Assistant: PortalRouteSchema.parse('assistant'),
  Start: PortalRouteSchema.parse('start'),
  Activity: PortalRouteSchema.parse('activity'),
  Browser: PortalRouteSchema.parse('browser'),
  BrowserSettings: PortalRouteSchema.parse('browser-settings'),
  Policy: PortalRouteSchema.parse('policy'),
  RuleManagement: PortalRouteSchema.parse('rule-management'),
  Schedules: PortalRouteSchema.parse('schedules'),
  Approvals: PortalRouteSchema.parse('approvals'),
  Enforcement: PortalRouteSchema.parse('enforcement'),
  PrivacyDesign: PortalRouteSchema.parse('privacy-design'),
  Memory: PortalRouteSchema.parse('memory'),
  MemorySettings: PortalRouteSchema.parse('memory-settings'),
  AiGuide: PortalRouteSchema.parse('ai-guide'),
  AiRuntime: PortalRouteSchema.parse('ai-runtime'),
  ApiProviders: PortalRouteSchema.parse('api-providers'),
  ReportsGuide: PortalRouteSchema.parse('reports-guide'),
  ReportSettings: PortalRouteSchema.parse('report-settings'),
  ScreenAnalysis: PortalRouteSchema.parse('screen-analysis'),
  AppGameSessions: PortalRouteSchema.parse('app-game-sessions'),
  NetworkActivity: PortalRouteSchema.parse('network-activity'),
  Devices: PortalRouteSchema.parse('devices'),
  LanPairing: PortalRouteSchema.parse('lan-pairing'),
  CapabilityStatus: PortalRouteSchema.parse('capability-status'),
  Notifications: PortalRouteSchema.parse('notifications'),
  NotificationChannels: PortalRouteSchema.parse('notification-channels'),
  DriveConnections: PortalRouteSchema.parse('drive-connections'),
  ExportRetention: PortalRouteSchema.parse('export-retention'),
  RemoteAccess: PortalRouteSchema.parse('remote-access'),
  ReportCompiler: PortalRouteSchema.parse('report-compiler'),
  AuditHistory: PortalRouteSchema.parse('audit-history'),
  Subscription: PortalRouteSchema.parse('subscription'),
  Entitlements: PortalRouteSchema.parse('entitlements'),
  PlatformsInstall: PortalRouteSchema.parse('platforms-install'),
  InstallUpdates: PortalRouteSchema.parse('install-updates'),
  Diagnostics: PortalRouteSchema.parse('diagnostics'),
  SettingsRules: PortalRouteSchema.parse('settings-rules'),
  FrameTuner: PortalRouteSchema.parse('app-layout'),
  LegacyFrameTuner: PortalRouteSchema.parse('frame-tuner'),
  Commands: PortalRouteSchema.parse('commands'),
  Events: PortalRouteSchema.parse('events'),
} as const;

export const PortalRoutes = [
  PortalRoute.Overview,
  PortalRoute.Assistant,
  PortalRoute.Start,
  PortalRoute.Activity,
  PortalRoute.Browser,
  PortalRoute.BrowserSettings,
  PortalRoute.Policy,
  PortalRoute.RuleManagement,
  PortalRoute.Schedules,
  PortalRoute.Approvals,
  PortalRoute.Enforcement,
  PortalRoute.PrivacyDesign,
  PortalRoute.Memory,
  PortalRoute.MemorySettings,
  PortalRoute.AiGuide,
  PortalRoute.AiRuntime,
  PortalRoute.ApiProviders,
  PortalRoute.ReportsGuide,
  PortalRoute.ReportSettings,
  PortalRoute.ScreenAnalysis,
  PortalRoute.AppGameSessions,
  PortalRoute.NetworkActivity,
  PortalRoute.Devices,
  PortalRoute.LanPairing,
  PortalRoute.CapabilityStatus,
  PortalRoute.Notifications,
  PortalRoute.NotificationChannels,
  PortalRoute.DriveConnections,
  PortalRoute.ExportRetention,
  PortalRoute.RemoteAccess,
  PortalRoute.ReportCompiler,
  PortalRoute.AuditHistory,
  PortalRoute.Subscription,
  PortalRoute.Entitlements,
  PortalRoute.PlatformsInstall,
  PortalRoute.InstallUpdates,
  PortalRoute.Diagnostics,
  PortalRoute.SettingsRules,
  PortalRoute.FrameTuner,
  PortalRoute.Commands,
  PortalRoute.Events,
] as const;

export const PortalRouteGroup = {
  Monitor: resolvePortalDevText(PortalDevTextToken.NavGroupMonitor),
  Guide: resolvePortalDevText(PortalDevTextToken.NavGroupGuide),
  Operate: resolvePortalDevText(PortalDevTextToken.NavGroupOperate),
} as const;

export type PortalRouteGroupValue = (typeof PortalRouteGroup)[keyof typeof PortalRouteGroup];

export type PortalRouteDescriptor = {
  readonly route: PortalRoute;
  readonly label: DisplayText;
  readonly description: DisplayText;
  readonly group: PortalRouteGroupValue;
};

export const PortalRouteDescriptors: readonly PortalRouteDescriptor[] = [
  routeDescriptor(
    PortalRoute.Overview,
    PortalDevTextToken.Overview,
    PortalDevTextToken.OverviewDescription,
    PortalRouteGroup.Monitor
  ),
  routeDescriptor(
    PortalRoute.Assistant,
    PortalDevTextToken.AiRuntime,
    PortalDevTextToken.AiRuntimeBody,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.Start,
    PortalDevTextToken.ParentPortal,
    PortalDevTextToken.ParentPortalDescription,
    PortalRouteGroup.Monitor
  ),
  routeDescriptor(
    PortalRoute.Activity,
    PortalDevTextToken.Activity,
    PortalDevTextToken.ActivityDescription,
    PortalRouteGroup.Monitor
  ),
  routeDescriptor(
    PortalRoute.Browser,
    PortalDevTextToken.Browser,
    PortalDevTextToken.BrowserDescription,
    PortalRouteGroup.Monitor
  ),
  routeDescriptor(
    PortalRoute.BrowserSettings,
    PortalDevTextToken.BrowserControls,
    PortalDevTextToken.BrowserBlockBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Policy,
    PortalDevTextToken.Policy,
    PortalDevTextToken.PolicyDescription,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.RuleManagement,
    PortalDevTextToken.RuleBuilder,
    PortalDevTextToken.RuleBuilderBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Schedules,
    PortalDevTextToken.SchedulesBudgets,
    PortalDevTextToken.SchedulesBudgetsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Approvals,
    PortalDevTextToken.Approvals,
    PortalDevTextToken.ApprovalsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Enforcement,
    PortalDevTextToken.PolicyModeActive,
    PortalDevTextToken.PolicyPreviewNoEnforcement,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PrivacyDesign,
    PortalDevTextToken.DataCustodyTitle,
    PortalDevTextToken.DataCustodyBody,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.Memory,
    PortalDevTextToken.Memory,
    PortalDevTextToken.MemoryDescription,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.MemorySettings,
    PortalDevTextToken.Memory,
    PortalDevTextToken.MemoryBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.AiGuide,
    PortalDevTextToken.AiRuntime,
    PortalDevTextToken.AiRuntimeDescription,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.AiRuntime,
    PortalDevTextToken.AiRuntime,
    PortalDevTextToken.AiRuntimeBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.ApiProviders,
    PortalDevTextToken.AiRuntime,
    PortalDevTextToken.AiRuntimeBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.ReportsGuide,
    PortalDevTextToken.Activity,
    PortalDevTextToken.ActivityDescription,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.ReportSettings,
    PortalDevTextToken.Activity,
    PortalDevTextToken.ActivityDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.ScreenAnalysis,
    PortalDevTextToken.ScreenAnalysis,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.AppGameSessions,
    PortalDevTextToken.AppGameSessions,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.NetworkActivity,
    PortalDevTextToken.NetworkFlow,
    PortalDevTextToken.NoNetworkFlow,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Devices,
    PortalDevTextToken.Devices,
    PortalDevTextToken.DevicesDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.LanPairing,
    PortalDevTextToken.Pairing,
    PortalDevTextToken.PairingBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.CapabilityStatus,
    PortalDevTextToken.Diagnostics,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Notifications,
    PortalDevTextToken.Notifications,
    PortalDevTextToken.NotificationsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.NotificationChannels,
    PortalDevTextToken.Notifications,
    PortalDevTextToken.NotificationsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.DriveConnections,
    PortalDevTextToken.DriveConnectionsTitle,
    PortalDevTextToken.DriveConnectionsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.ExportRetention,
    PortalDevTextToken.ExportSync,
    PortalDevTextToken.DriveConnectionsBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.RemoteAccess,
    PortalDevTextToken.DataCustodyTitle,
    PortalDevTextToken.DataCustodyBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.ReportCompiler,
    PortalDevTextToken.Activity,
    PortalDevTextToken.ActivityDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.AuditHistory,
    PortalDevTextToken.Events,
    PortalDevTextToken.EventsDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Subscription,
    PortalDevTextToken.BillingEntitlements,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Entitlements,
    PortalDevTextToken.BillingEntitlements,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PlatformsInstall,
    PortalDevTextToken.DesktopApp,
    PortalDevTextToken.DesktopAppBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.InstallUpdates,
    PortalDevTextToken.DesktopApp,
    PortalDevTextToken.DesktopAppBody,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Diagnostics,
    PortalDevTextToken.Diagnostics,
    PortalDevTextToken.DiagnosticsDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.SettingsRules,
    PortalDevTextToken.SettingsRules,
    PortalDevTextToken.SettingsRulesDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.FrameTuner,
    PortalDevTextToken.FrameTuner,
    PortalDevTextToken.FrameTunerDescription,
    PortalRouteGroup.Operate
  ),
] as const;

export const PortalSidebarRouteDescriptors: readonly PortalRouteDescriptor[] = PortalRouteDescriptors.filter(
  (descriptor) => descriptor.route !== PortalRoute.FrameTuner
);

function routeDescriptor(
  route: PortalRoute,
  labelToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken],
  descriptionToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken],
  group: PortalRouteGroupValue
): PortalRouteDescriptor {
  return {
    route,
    label: resolvePortalDevText(labelToken),
    description: resolvePortalDevText(descriptionToken),
    group,
  };
}
