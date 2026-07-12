import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  PortalRoute,
  PortalRouteHashPrefix,
  PortalRouteHashQuerySeparator,
  PortalRouteSchema as SharedPortalRouteSchema,
  type PortalRoute as PortalRouteValue,
} from './portal-contract-adapter';
import {
  PortalDevToolUrlSchema as SharedPortalDevToolUrlSchema,
  type PortalDevToolUrl,
} from './portal-contract-text-contracts';
import { generatedPortalRouteFromHashPath } from './portal-route-state.generated';

export type PortalRouteHashPath = `${typeof PortalRouteHashPrefix}${PortalRouteValue}`;
export type PortalRouteHashQueryPath =
  `${typeof PortalRouteHashPrefix}${PortalRouteValue}${typeof PortalRouteHashQuerySeparator}${string}`;

export function portalRouteHashPath(route: PortalRouteValue): PortalRouteHashPath {
  return `${PortalRouteHashPrefix}${route}`;
}

export function portalRouteHashPathWithQuery(route: PortalRouteValue, query: string): PortalRouteHashQueryPath {
  return `${PortalRouteHashPrefix}${route}${PortalRouteHashQuerySeparator}${query}`;
}

export function portalRouteFromHashPath(routeHash: string): PortalRouteValue | null {
  const route = generatedPortalRouteFromHashPath(routeHash);
  const parsedRoute = SharedPortalRouteSchema.safeParse(route);
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
  PortalRoute.ProofPanels,
  PortalRoute.SettingsRules,
  PortalRoute.AppLayout,
  PortalRoute.FrameTuner,
  PortalRoute.Commands,
  PortalRoute.Events,
  PortalRoute.Logs,
] as const;

export const PortalNetworkEvidenceDrawerRoutes = [PortalRoute.Activity, PortalRoute.NetworkActivity] as const;
export const PortalInlineNetworkEvidenceDrawerRoutes = [PortalRoute.Activity] as const;
export const PortalAppGameParentSurfaceRoutes = [PortalRoute.AppGameSessions] as const;
export const PortalAiRuntimeRoutes = [PortalRoute.AiRuntime] as const;
export const PortalBrowserParentSurfaceRoutes = [PortalRoute.Browser] as const;
export const PortalDeveloperRoutes = [PortalRoute.Commands, PortalRoute.Events, PortalRoute.Logs] as const;
export const PortalDeveloperCommandRoutes = [PortalRoute.Commands] as const;
export const PortalDeveloperEventRoutes = [PortalRoute.Events] as const;
export const PortalDeveloperLogRoutes = [PortalRoute.Logs] as const;
export const PortalPolicyPreviewRoutes = [
  PortalRoute.RuleManagement,
  PortalRoute.Schedules,
  PortalRoute.Approvals,
  PortalRoute.Enforcement,
] as const;
export const PortalScreenSettingsRoutes = [PortalRoute.SettingsRules] as const;
export const PortalScreenSummaryRoutes = [PortalRoute.ScreenAnalysis] as const;
export const PortalSetupFirstRunRoutes = [PortalRoute.Start] as const;
export const PortalTrackingStatusRoutes = [PortalRoute.PolicyTracking] as const;

export function isPortalAiRuntimeRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalAiRuntimeRoutes);
}

export function isPortalAppGameParentSurfaceRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalAppGameParentSurfaceRoutes);
}

export function isPortalBrowserParentSurfaceRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalBrowserParentSurfaceRoutes);
}

export function isPortalDeveloperRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalDeveloperRoutes);
}

export function isPortalDeveloperCommandRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalDeveloperCommandRoutes);
}

export function isPortalDeveloperEventRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalDeveloperEventRoutes);
}

export function isPortalDeveloperLogRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalDeveloperLogRoutes);
}

export function isPortalNetworkEvidenceDrawerRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalNetworkEvidenceDrawerRoutes);
}

export function isPortalInlineNetworkEvidenceDrawerRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalInlineNetworkEvidenceDrawerRoutes);
}

export function isPortalPolicyPreviewRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalPolicyPreviewRoutes);
}

export function isPortalScreenSettingsRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalScreenSettingsRoutes);
}

export function isPortalScreenSummaryRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalScreenSummaryRoutes);
}

export function isPortalSetupFirstRunRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalSetupFirstRunRoutes);
}

export function isPortalTrackingStatusRoute(route: PortalRouteValue): boolean {
  return routeMatches(route, PortalTrackingStatusRoutes);
}

function routeMatches(route: PortalRouteValue, routes: readonly PortalRouteValue[]): boolean {
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

export function portalDevToolUrl(origin: string, pathname: string, route: PortalRouteValue): PortalDevToolUrl {
  return SharedPortalDevToolUrlSchema.parse(`${origin}${pathname}${portalRouteHashPath(route)}`);
}

export const PortalRouteGroup = {
  Monitor: resolvePortalDevText(PortalDevTextToken.NavGroupMonitor),
  Guide: resolvePortalDevText(PortalDevTextToken.NavGroupGuide),
  Operate: resolvePortalDevText(PortalDevTextToken.NavGroupOperate),
  DevTools: resolvePortalDevText(PortalDevTextToken.NavGroupDevTools),
} as const;

export type PortalRouteGroupValue = (typeof PortalRouteGroup)[keyof typeof PortalRouteGroup];

export type PortalRouteDescriptor = {
  readonly route: PortalRouteValue;
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
    PortalRoute.ProofPanels,
    PortalDevTextToken.ProofPanels,
    PortalDevTextToken.ProofPanelsDescription,
    PortalRouteGroup.DevTools
  ),
  routeDescriptor(
    PortalRoute.SettingsRules,
    PortalDevTextToken.SettingsRules,
    PortalDevTextToken.SettingsRulesDescription,
    PortalRouteGroup.Operate
  ),
  routeDescriptor(
    PortalRoute.AppLayout,
    PortalDevTextToken.FrameTuner,
    PortalDevTextToken.FrameTunerDescription,
    PortalRouteGroup.DevTools
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
  (descriptor) => descriptor.route !== PortalRoute.AppLayout && descriptor.route !== PortalRoute.FrameTuner
);

function routeDescriptor(
  route: PortalRouteValue,
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
