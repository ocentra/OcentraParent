import type {
  ParentMobileRuntimeReadModelCandidate,
  ParentMobileServiceAvailabilityState,
  ParentMobileServiceRouteCustody,
  ParentMobileServiceRouteKind,
} from './parent-mobile-runtime';

const ParentMobileRouteStateAccessors = {
  'local-service': (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) =>
    serviceAvailability.localService,
  'lan-service': (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) =>
    serviceAvailability.lanService,
  'cloud-relay': (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) =>
    serviceAvailability.cloudRelay,
  'parent-cache': (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) =>
    serviceAvailability.parentCache,
  'parent-owned-storage': (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) =>
    serviceAvailability.parentOwnedStorage,
} as const satisfies Record<
  ParentMobileServiceRouteKind,
  (serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']) => ParentMobileServiceAvailabilityState
>;

export const ParentMobileStaticRouteExpectations = {
  'cloud-relay': {
    state: 'not-implemented',
    custody: 'unavailable',
  },
  'parent-cache': {
    state: 'stale',
    custody: 'parent-cache',
  },
  'parent-owned-storage': {
    state: 'offline',
    custody: 'parent-owned-storage',
  },
} as const satisfies Record<
  'cloud-relay' | 'parent-cache' | 'parent-owned-storage',
  {
    state: ParentMobileServiceAvailabilityState;
    custody: ParentMobileServiceRouteCustody;
  }
>;

export function expectedParentMobileRouteState(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability'],
  routeKind: ParentMobileServiceRouteKind
): ParentMobileServiceAvailabilityState {
  return ParentMobileRouteStateAccessors[routeKind](serviceAvailability);
}
