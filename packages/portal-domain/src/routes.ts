import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import {
  PortalConnectionState as SharedPortalConnectionState,
  PortalConnectionStateSchema as SharedPortalConnectionStateSchema,
  PortalDevToolUrlSchema as SharedPortalDevToolUrlSchema,
  PortalRoute as SharedPortalRoute,
  PortalRouteHashPrefix,
  PortalRouteHashQuerySeparator,
  PortalRouteLiteral as SharedPortalRouteLiteral,
  PortalRouteSchema as SharedPortalRouteSchema,
  type PortalConnectionState as SharedPortalConnectionStateValue,
  type PortalDevToolUrl as SharedPortalDevToolUrlValue,
  type PortalRoute as SharedPortalRouteValue,
} from '@ocentra-parent/schema-domain/portal-contracts';

export const PortalRouteLiteral = SharedPortalRouteLiteral;
export const PortalRouteSchema = SharedPortalRouteSchema;
export type PortalRoute = SharedPortalRouteValue;
export const PortalRoute = SharedPortalRoute;
export { PortalRouteHashPrefix, PortalRouteHashQuerySeparator };

export type PortalRouteHashPath = `${typeof PortalRouteHashPrefix}${PortalRoute}`;
export type PortalRouteHashQueryPath =
  `${typeof PortalRouteHashPrefix}${PortalRoute}${typeof PortalRouteHashQuerySeparator}${string}`;

export function portalRouteHashPath(route: PortalRoute): PortalRouteHashPath {
  return `${PortalRouteHashPrefix}${route}`;
}

export function portalRouteHashPathWithQuery(route: PortalRoute, query: string): PortalRouteHashQueryPath {
  return `${PortalRouteHashPrefix}${route}${PortalRouteHashQuerySeparator}${query}`;
}

export function portalRouteFromHashPath(routeHash: string): PortalRoute | null {
  const normalizedHash = routeHash.replace(/^#\/?/u, '');
  const route = normalizedHash.split(PortalRouteHashQuerySeparator)[0] ?? '';
  const parsedRoute = PortalRouteSchema.safeParse(route);
  if (!parsedRoute.success) {
    return null;
  }
  return PortalRoutes.some((portalRoute) => portalRoute === parsedRoute.data) ? parsedRoute.data : null;
}

export const PortalRoutes = [
  PortalRoute.Overview,
  PortalRoute.Assistant,
  PortalRoute.Start,
  PortalRoute.Activity,
  PortalRoute.Browser,
  PortalRoute.BrowserSettings,
  PortalRoute.Policy,
  PortalRoute.PolicyApps,
  PortalRoute.PolicyGames,
  PortalRoute.PolicyScreen,
  PortalRoute.PolicyNetwork,
  PortalRoute.PolicyTracking,
  PortalRoute.PolicyRemoteScreen,
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
  PortalRoute.Logs,
] as const;

export const PortalNetworkEvidenceDrawerRoutes = [PortalRoute.Activity, PortalRoute.NetworkActivity] as const;
export const PortalAppGameParentSurfaceRoutes = [PortalRoute.AppGameSessions] as const;
export const PortalAiRuntimeRoutes = [PortalRoute.AiRuntime] as const;
export const PortalBrowserParentSurfaceRoutes = [PortalRoute.Browser] as const;
export const PortalScreenSettingsRoutes = [PortalRoute.SettingsRules] as const;
export const PortalScreenSummaryRoutes = [PortalRoute.ScreenAnalysis] as const;
export const PortalTrackingStatusRoutes = [PortalRoute.PolicyTracking] as const;

export function isPortalAiRuntimeRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalAiRuntimeRoutes);
}

export function isPortalAppGameParentSurfaceRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalAppGameParentSurfaceRoutes);
}

export function isPortalBrowserParentSurfaceRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalBrowserParentSurfaceRoutes);
}

export function isPortalNetworkEvidenceDrawerRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalNetworkEvidenceDrawerRoutes);
}

export function isPortalScreenSettingsRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalScreenSettingsRoutes);
}

export function isPortalScreenSummaryRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalScreenSummaryRoutes);
}

export function isPortalTrackingStatusRoute(route: PortalRoute): boolean {
  return routeMatches(route, PortalTrackingStatusRoutes);
}

function routeMatches(route: PortalRoute, routes: readonly PortalRoute[]): boolean {
  return routes.some((candidate) => candidate === route);
}

export const PortalDevToolWindow = {
  FrameTunerHeight: 900,
  FrameTunerHash: portalRouteHashPath(PortalRoute.FrameTuner),
  FrameTunerLabel: 'portal-app-layout',
  FrameTunerWidth: 1280,
  PopupFeatures: 'popup=yes,width=1280,height=900,resizable=yes,scrollbars=yes',
  TauriErrorEvent: 'tauri://error',
  TauriInternalKey: '__TAURI_INTERNALS__',
} as const;

export const PortalConnectionStateSchema = SharedPortalConnectionStateSchema;
export type PortalConnectionState = SharedPortalConnectionStateValue;
export const PortalConnectionState = SharedPortalConnectionState;

export const PortalDevToolUrlSchema = SharedPortalDevToolUrlSchema;
export type PortalDevToolUrl = SharedPortalDevToolUrlValue;

export function portalDevToolUrl(origin: string, pathname: string, route: PortalRoute): PortalDevToolUrl {
  return PortalDevToolUrlSchema.parse(`${origin}${pathname}${portalRouteHashPath(route)}`);
}

export const PortalRouteGroup = {
  Monitor: resolvePortalDevText(PortalDevTextToken.NavGroupMonitor),
  Guide: resolvePortalDevText(PortalDevTextToken.NavGroupGuide),
  Operate: resolvePortalDevText(PortalDevTextToken.NavGroupOperate),
  DevTools: resolvePortalDevText(PortalDevTextToken.NavGroupDevTools),
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
    PortalRoute.PolicyApps,
    PortalDevTextToken.AppGameSessions,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PolicyGames,
    PortalDevTextToken.AppGameSessions,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PolicyScreen,
    PortalDevTextToken.ScreenAnalysis,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PolicyNetwork,
    PortalDevTextToken.NetworkFlow,
    PortalDevTextToken.NoNetworkFlow,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PolicyTracking,
    PortalDevTextToken.DeviceInventory,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.PolicyRemoteScreen,
    PortalDevTextToken.RemoteScreen,
    PortalDevTextToken.ProductSurfacePending,
    PortalRouteGroup.Operate
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
    PortalRouteGroup.DevTools
  ),
  routeDescriptor(
    PortalRoute.Commands,
    PortalDevTextToken.Commands,
    PortalDevTextToken.CommandsDescription,
    PortalRouteGroup.DevTools
  ),
  routeDescriptor(
    PortalRoute.Events,
    PortalDevTextToken.Events,
    PortalDevTextToken.EventsDescription,
    PortalRouteGroup.DevTools
  ),
  routeDescriptor(
    PortalRoute.Logs,
    PortalDevTextToken.Logs,
    PortalDevTextToken.LogsDescription,
    PortalRouteGroup.DevTools
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

