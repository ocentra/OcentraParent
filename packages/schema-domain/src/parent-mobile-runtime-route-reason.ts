import type {
  ParentMobileServiceAvailabilityState,
  ParentMobileServiceRouteKind,
  ParentMobileServiceRouteStatus,
} from './parent-mobile-runtime';

const ParentMobileLiveRouteReasonByState = {
  available: 'available',
  degraded: 'degraded',
  unavailable: 'unavailable',
  'manual-required': 'proof-required',
} as const satisfies Record<ParentMobileServiceAvailabilityState, string>;

const ParentMobileParentCacheRouteReasonByState = {
  unavailable: 'parent-cache-unavailable',
  stale: 'parent-cache-stale',
} as const satisfies Record<'unavailable' | 'stale', string>;

const ParentMobileParentOwnedStorageRouteReasonByState = {
  unavailable: 'parent-owned-storage-unavailable',
  offline: 'parent-owned-storage-offline',
} as const satisfies Record<'unavailable' | 'offline', string>;

const ParentMobileRouteStatusReasonByKind = {
  'local-service': (state: ParentMobileServiceAvailabilityState) => `local-service-${ParentMobileLiveRouteReasonByState[state]}`,
  'lan-service': (state: ParentMobileServiceAvailabilityState) => `lan-service-${ParentMobileLiveRouteReasonByState[state]}`,
  'cloud-relay': () => 'cloud-relay-not-implemented',
  'parent-cache': (state: ParentMobileServiceAvailabilityState) =>
    ParentMobileParentCacheRouteReasonByState[state as 'unavailable' | 'stale'],
  'parent-owned-storage': (state: ParentMobileServiceAvailabilityState) =>
    ParentMobileParentOwnedStorageRouteReasonByState[state as 'unavailable' | 'offline'],
} as const satisfies Record<
  ParentMobileServiceRouteKind,
  (state: ParentMobileServiceAvailabilityState) => string
>;

export function expectedParentMobileRouteStatusReason(routeStatus: ParentMobileServiceRouteStatus): string {
  return ParentMobileRouteStatusReasonByKind[routeStatus.routeKind](routeStatus.state);
}
