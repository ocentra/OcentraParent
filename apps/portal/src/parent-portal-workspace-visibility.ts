import {
  isParentPolicyPreviewRoute,
  isParentScreenSummaryRoute,
  isParentScreenSettingsRoute,
  isParentTrackingStatusRoute,
  ParentRoute,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

const HIDDEN_WORKSPACE_ROUTES = new Set<ParentRouteId>([
  ParentRoute.Browser,
  ParentRoute.BrowserSettings,
  ParentRoute.CapabilityStatus,
  ParentRoute.NetworkActivity,
  ParentRoute.PolicyApps,
  ParentRoute.PolicyGames,
  ParentRoute.PolicyScreen,
  ParentRoute.PolicyRemoteScreen,
  ParentRoute.RemoteAccess,
  ParentRoute.PlatformsInstall,
  ParentRoute.InstallUpdates,
]);

export function parentPortalWorkspaceIsVisible(route: ParentRouteId): boolean {
  return (
    !HIDDEN_WORKSPACE_ROUTES.has(route) &&
    !isParentTrackingStatusRoute(route) &&
    !isParentScreenSummaryRoute(route) &&
    !isParentScreenSettingsRoute(route) &&
    !isParentPolicyPreviewRoute(route)
  );
}
