import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const PortalRouteSchema = withParser(
  Schema.Literal(
    'overview',
    'leaderboard-copy',
    'activity',
    'browser',
    'policy',
    'privacy-design',
    'memory',
    'ai-runtime',
    'devices',
    'notifications',
    'drive-connections',
    'diagnostics',
    'settings-rules',
    'frame-tuner',
    'commands',
    'events'
  )
);
export type PortalRoute = Infer<typeof PortalRouteSchema>;

export const PortalRoute = {
  Overview: PortalRouteSchema.parse('overview'),
  LeaderboardCopy: PortalRouteSchema.parse('leaderboard-copy'),
  Activity: PortalRouteSchema.parse('activity'),
  Browser: PortalRouteSchema.parse('browser'),
  Policy: PortalRouteSchema.parse('policy'),
  PrivacyDesign: PortalRouteSchema.parse('privacy-design'),
  Memory: PortalRouteSchema.parse('memory'),
  AiRuntime: PortalRouteSchema.parse('ai-runtime'),
  Devices: PortalRouteSchema.parse('devices'),
  Notifications: PortalRouteSchema.parse('notifications'),
  DriveConnections: PortalRouteSchema.parse('drive-connections'),
  Diagnostics: PortalRouteSchema.parse('diagnostics'),
  SettingsRules: PortalRouteSchema.parse('settings-rules'),
  FrameTuner: PortalRouteSchema.parse('frame-tuner'),
  Commands: PortalRouteSchema.parse('commands'),
  Events: PortalRouteSchema.parse('events'),
} as const;

export const PortalRoutes = [
  PortalRoute.Overview,
  PortalRoute.LeaderboardCopy,
  PortalRoute.Activity,
  PortalRoute.Browser,
  PortalRoute.Policy,
  PortalRoute.PrivacyDesign,
  PortalRoute.Memory,
  PortalRoute.AiRuntime,
  PortalRoute.Devices,
  PortalRoute.Notifications,
  PortalRoute.DriveConnections,
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
    PortalRoute.LeaderboardCopy,
    PortalDevTextToken.LeaderboardCopy,
    PortalDevTextToken.LeaderboardCopyDescription,
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
    PortalRoute.Policy,
    PortalDevTextToken.Policy,
    PortalDevTextToken.PolicyDescription,
    PortalRouteGroup.Guide
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
    PortalRoute.AiRuntime,
    PortalDevTextToken.AiRuntime,
    PortalDevTextToken.AiRuntimeDescription,
    PortalRouteGroup.Guide
  ),
  routeDescriptor(
    PortalRoute.Devices,
    PortalDevTextToken.Devices,
    PortalDevTextToken.DevicesDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.Notifications,
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
