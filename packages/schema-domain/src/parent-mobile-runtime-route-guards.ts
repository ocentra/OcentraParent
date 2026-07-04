import type {
  ParentMobileRuntimeReadModelCandidate,
  ParentMobileServiceRouteKind,
  ParentMobileServiceRouteStatus,
} from './parent-mobile-runtime';
import { expectedParentMobileRouteState, ParentMobileStaticRouteExpectations } from './parent-mobile-runtime-route-state';
import { expectedParentMobileRouteStatusReason } from './parent-mobile-runtime-route-reason';

const RequiredParentMobileRouteKinds = [
  'local-service',
  'lan-service',
  'cloud-relay',
  'parent-cache',
  'parent-owned-storage',
] as const satisfies ReadonlyArray<ParentMobileServiceRouteKind>;

type ParentMobileStaticRouteKind = keyof typeof ParentMobileStaticRouteExpectations;

export function parentMobileRouteStatusesAreConsistent(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']
): boolean {
  const byKind = new Map(serviceAvailability.routeStatuses.map((route) => [route.routeKind, route] as const));
  const selectedRoutes = serviceAvailability.routeStatuses.filter((route) => route.selectedRouteId !== null);

  return (
    byKind.size === serviceAvailability.routeStatuses.length &&
    RequiredParentMobileRouteKinds.every((kind) => byKind.has(kind)) &&
    (serviceAvailability.selectedRouteId === null
      ? selectedRoutes.length === 0
      : selectedRoutes.length === 1 && selectedRoutes[0]?.selectedRouteId === serviceAvailability.selectedRouteId) &&
    RequiredParentMobileRouteKinds.every((kind) =>
      parentMobileRouteStatusMatchesAvailability(serviceAvailability, byKind.get(kind))
    )
  );
}

function parentMobileRouteStatusMatchesAvailability(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability'],
  routeStatus: ParentMobileServiceRouteStatus | undefined
): boolean {
  return (
    routeStatus !== undefined &&
    routeStatus.state === expectedParentMobileRouteState(serviceAvailability, routeStatus.routeKind) &&
    routeStatus.statusReason === expectedParentMobileRouteStatusReason(routeStatus) &&
    parentMobileRouteStatusCustodyMatches(routeStatus)
  );
}

function parentMobileRouteStatusCustodyMatches(routeStatus: ParentMobileServiceRouteStatus): boolean {
  const staticExpectation =
    ParentMobileStaticRouteExpectations[routeStatus.routeKind as ParentMobileStaticRouteKind] ?? null;

  return routeStatus.state === 'unavailable'
    ? routeStatus.custody === 'unavailable' && routeStatus.selectedRouteId === null
    : staticExpectation === null
      ? routeStatus.custody === routeStatus.routeKind
      : routeStatus.state === staticExpectation.state &&
        routeStatus.custody === staticExpectation.custody &&
        routeStatus.selectedRouteId === null;
}
