import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import {
  ParentRoute,
  ParentRouteGroup,
  ParentRouteMetadata,
  ParentSidebarRoutes,
  type ParentRouteGroupId,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

type PortalRouteTextToken = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

type ParentRoutePresentationText = {
  readonly label: PortalRouteTextToken;
  readonly description: PortalRouteTextToken;
};

export type PortalRouteDescriptor = {
  readonly route: ParentRouteId;
  readonly label: PortalDisplayText;
  readonly description: PortalDisplayText;
  readonly group: ParentRouteGroupId;
};

const ParentRouteGroupLabels: Readonly<Record<ParentRouteGroupId, PortalDisplayText>> = {
  [ParentRouteGroup.Monitor]: resolvePortalDevText(PortalDevTextToken.NavGroupMonitor),
  [ParentRouteGroup.Guide]: resolvePortalDevText(PortalDevTextToken.NavGroupGuide),
  [ParentRouteGroup.Operate]: resolvePortalDevText(PortalDevTextToken.NavGroupOperate),
  [ParentRouteGroup.DevTools]: resolvePortalDevText(PortalDevTextToken.NavGroupDevTools),
};

const ParentRoutePresentationTextTokens: Readonly<Record<ParentRouteId, ParentRoutePresentationText>> = {
  [ParentRoute.Overview]: {
    label: PortalDevTextToken.Overview,
    description: PortalDevTextToken.OverviewDescription,
  },
  [ParentRoute.Assistant]: {
    label: PortalDevTextToken.AiRuntime,
    description: PortalDevTextToken.AiRuntimeBody,
  },
  [ParentRoute.Start]: {
    label: PortalDevTextToken.ParentPortal,
    description: PortalDevTextToken.ParentPortalDescription,
  },
  [ParentRoute.Activity]: {
    label: PortalDevTextToken.Activity,
    description: PortalDevTextToken.ActivityDescription,
  },
  [ParentRoute.Browser]: {
    label: PortalDevTextToken.Browser,
    description: PortalDevTextToken.BrowserDescription,
  },
  [ParentRoute.BrowserSettings]: {
    label: PortalDevTextToken.BrowserControls,
    description: PortalDevTextToken.BrowserBlockBody,
  },
  [ParentRoute.Policy]: {
    label: PortalDevTextToken.Policy,
    description: PortalDevTextToken.PolicyDescription,
  },
  [ParentRoute.PolicyApps]: {
    label: PortalDevTextToken.AppGameSessions,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.PolicyGames]: {
    label: PortalDevTextToken.AppGameSessions,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.PolicyScreen]: {
    label: PortalDevTextToken.ScreenAnalysis,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.PolicyNetwork]: {
    label: PortalDevTextToken.NetworkFlow,
    description: PortalDevTextToken.NoNetworkFlow,
  },
  [ParentRoute.PolicyTracking]: {
    label: PortalDevTextToken.DeviceInventory,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.PolicyRemoteScreen]: {
    label: PortalDevTextToken.RemoteScreen,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.RuleManagement]: {
    label: PortalDevTextToken.RuleBuilder,
    description: PortalDevTextToken.RuleBuilderBody,
  },
  [ParentRoute.Schedules]: {
    label: PortalDevTextToken.SchedulesBudgets,
    description: PortalDevTextToken.SchedulesBudgetsBody,
  },
  [ParentRoute.Approvals]: {
    label: PortalDevTextToken.Approvals,
    description: PortalDevTextToken.ApprovalsBody,
  },
  [ParentRoute.Enforcement]: {
    label: PortalDevTextToken.PolicyModeActive,
    description: PortalDevTextToken.PolicyPreviewNoEnforcement,
  },
  [ParentRoute.PrivacyDesign]: {
    label: PortalDevTextToken.DataCustodyTitle,
    description: PortalDevTextToken.DataCustodyBody,
  },
  [ParentRoute.Memory]: {
    label: PortalDevTextToken.Memory,
    description: PortalDevTextToken.MemoryDescription,
  },
  [ParentRoute.MemorySettings]: {
    label: PortalDevTextToken.Memory,
    description: PortalDevTextToken.MemoryBody,
  },
  [ParentRoute.AiGuide]: {
    label: PortalDevTextToken.AiRuntime,
    description: PortalDevTextToken.AiRuntimeDescription,
  },
  [ParentRoute.AiRuntime]: {
    label: PortalDevTextToken.AiRuntime,
    description: PortalDevTextToken.AiRuntimeBody,
  },
  [ParentRoute.ApiProviders]: {
    label: PortalDevTextToken.AiRuntime,
    description: PortalDevTextToken.AiRuntimeBody,
  },
  [ParentRoute.ReportsGuide]: {
    label: PortalDevTextToken.Activity,
    description: PortalDevTextToken.ActivityDescription,
  },
  [ParentRoute.ScreenAnalysis]: {
    label: PortalDevTextToken.ScreenAnalysis,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.AppGameSessions]: {
    label: PortalDevTextToken.AppGameSessions,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.NetworkActivity]: {
    label: PortalDevTextToken.NetworkFlow,
    description: PortalDevTextToken.NoNetworkFlow,
  },
  [ParentRoute.Devices]: {
    label: PortalDevTextToken.Devices,
    description: PortalDevTextToken.DevicesDescription,
  },
  [ParentRoute.LanPairing]: {
    label: PortalDevTextToken.Pairing,
    description: PortalDevTextToken.PairingBody,
  },
  [ParentRoute.CapabilityStatus]: {
    label: PortalDevTextToken.Diagnostics,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.Notifications]: {
    label: PortalDevTextToken.Notifications,
    description: PortalDevTextToken.NotificationsBody,
  },
  [ParentRoute.NotificationChannels]: {
    label: PortalDevTextToken.Notifications,
    description: PortalDevTextToken.NotificationsBody,
  },
  [ParentRoute.DriveConnections]: {
    label: PortalDevTextToken.DriveConnectionsTitle,
    description: PortalDevTextToken.DriveConnectionsBody,
  },
  [ParentRoute.ExportRetention]: {
    label: PortalDevTextToken.ExportSync,
    description: PortalDevTextToken.DriveConnectionsBody,
  },
  [ParentRoute.RemoteAccess]: {
    label: PortalDevTextToken.DataCustodyTitle,
    description: PortalDevTextToken.DataCustodyBody,
  },
  [ParentRoute.ReportCompiler]: {
    label: PortalDevTextToken.Activity,
    description: PortalDevTextToken.ActivityDescription,
  },
  [ParentRoute.AuditHistory]: {
    label: PortalDevTextToken.Events,
    description: PortalDevTextToken.EventsDescription,
  },
  [ParentRoute.Subscription]: {
    label: PortalDevTextToken.BillingEntitlements,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.Entitlements]: {
    label: PortalDevTextToken.BillingEntitlements,
    description: PortalDevTextToken.ProductSurfacePending,
  },
  [ParentRoute.PlatformsInstall]: {
    label: PortalDevTextToken.DesktopApp,
    description: PortalDevTextToken.DesktopAppBody,
  },
  [ParentRoute.InstallUpdates]: {
    label: PortalDevTextToken.DesktopApp,
    description: PortalDevTextToken.DesktopAppBody,
  },
  [ParentRoute.Diagnostics]: {
    label: PortalDevTextToken.Diagnostics,
    description: PortalDevTextToken.DiagnosticsDescription,
  },
  [ParentRoute.ProofPanels]: {
    label: PortalDevTextToken.ProofPanels,
    description: PortalDevTextToken.ProofPanelsDescription,
  },
  [ParentRoute.SettingsRules]: {
    label: PortalDevTextToken.SettingsRules,
    description: PortalDevTextToken.SettingsRulesDescription,
  },
  [ParentRoute.AppLayout]: {
    label: PortalDevTextToken.FrameTuner,
    description: PortalDevTextToken.FrameTunerDescription,
  },
  [ParentRoute.FrameTuner]: {
    label: PortalDevTextToken.FrameTuner,
    description: PortalDevTextToken.FrameTunerDescription,
  },
  [ParentRoute.Commands]: {
    label: PortalDevTextToken.Commands,
    description: PortalDevTextToken.CommandsDescription,
  },
  [ParentRoute.Events]: {
    label: PortalDevTextToken.Events,
    description: PortalDevTextToken.EventsDescription,
  },
  [ParentRoute.Logs]: {
    label: PortalDevTextToken.Logs,
    description: PortalDevTextToken.LogsDescription,
  },
};

export const ParentPortalRouteDescriptors: readonly PortalRouteDescriptor[] =
  ParentSidebarRoutes.map(parentPortalRouteDescriptor);

export function routeDescriptor(route: ParentRouteId): PortalRouteDescriptor {
  return parentPortalRouteDescriptor(route);
}

export function parentRouteGroupLabel(group: ParentRouteGroupId): PortalDisplayText {
  return ParentRouteGroupLabels[group];
}

function parentPortalRouteDescriptor(route: ParentRouteId): PortalRouteDescriptor {
  const text = ParentRoutePresentationTextTokens[route];
  return {
    route,
    label: resolvePortalDevText(text.label),
    description: resolvePortalDevText(text.description),
    group: ParentRouteMetadata[route].group,
  };
}
