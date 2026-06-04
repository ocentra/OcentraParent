import type { ParentMobileServiceAvailabilityState } from '../src/parent-mobile-runtime';

const CheckedAt = '2026-05-28T16:05:00.000Z';
type ParentMobileFixtureRouteId = 'route-parent-mobile-lan-provider' | null;

export const AndroidParentMobileCapabilities = [
  {
    capability: 'parent-mobile-observer',
    status: 'scaffold',
    proofRequirement: 'typed observer read model and package launch target',
    claimBoundary: 'observer state is represented without mobile UX parity',
  },
  {
    capability: 'parent-mobile-controller',
    status: 'manual-required',
    proofRequirement: 'real mobile package and device controller takeover proof',
    claimBoundary: 'no parent mobile write authority is claimed from scaffold state',
  },
  {
    capability: 'foreground-mobile-service',
    status: 'manual-required',
    proofRequirement: 'Android emulator or device foreground-service and notification proof',
    claimBoundary: 'manifest declaration is not foreground behavior proof',
  },
  {
    capability: 'notifications',
    status: 'manual-required',
    proofRequirement: 'Android notification permission prompt and delivery proof',
    claimBoundary: 'permission declaration is not runtime notification proof',
  },
  {
    capability: 'package-lifecycle',
    status: 'manual-required',
    proofRequirement: 'Android install launch background update and uninstall proof',
    claimBoundary: 'debug package mechanics are not store or lifecycle proof',
  },
  {
    capability: 'store-distribution',
    status: 'planned',
    proofRequirement: 'Google Play signing and release-track proof',
    claimBoundary: 'store distribution is not wired',
  },
] as const;

export const IosParentMobileCapabilities = [
  {
    capability: 'parent-mobile-observer',
    status: 'scaffold',
    proofRequirement: 'typed observer read model and simulator app target',
    claimBoundary: 'observer state is represented without mobile UX parity',
  },
  {
    capability: 'parent-mobile-controller',
    status: 'manual-required',
    proofRequirement: 'real signed mobile package and device controller takeover proof',
    claimBoundary: 'no parent mobile write authority is claimed from simulator scaffold',
  },
  {
    capability: 'foreground-mobile-service',
    status: 'unavailable',
    proofRequirement: 'iOS has no Android-style foreground service',
    claimBoundary: 'foreground service is not an iOS parent mobile claim',
  },
  {
    capability: 'notifications',
    status: 'manual-required',
    proofRequirement: 'iOS notification permission and delivery proof',
    claimBoundary: 'notification behavior requires device or simulator permission evidence',
  },
  {
    capability: 'background-execution',
    status: 'manual-required',
    proofRequirement: 'iOS background mode entitlement and device behavior proof',
    claimBoundary: 'simulator app target is not background execution proof',
  },
  {
    capability: 'signing-entitlements',
    status: 'manual-required',
    proofRequirement: 'Apple signing team provisioning and entitlement proof',
    claimBoundary: 'simulator build is not signing or entitlement proof',
  },
  {
    capability: 'testflight-distribution',
    status: 'manual-required',
    proofRequirement: 'TestFlight build upload install and launch proof',
    claimBoundary: 'TestFlight distribution is not wired',
  },
  {
    capability: 'store-distribution',
    status: 'planned',
    proofRequirement: 'App Store release-track proof',
    claimBoundary: 'store distribution is not wired',
  },
] as const;

export const AndroidObserverReadModel = {
  schemaVersion: 'v0.9-parent-mobile-shell',
  parentDeviceId: 'parent-mobile-android-observer',
  platform: 'android',
  packageProof: {
    platform: 'android',
    packageState: 'ci-mechanical-proof',
    launchTarget: 'ca.ocentra.parent.agent/.MainActivity',
    proofCommand: 'cmd /c npm run release:package:android',
    packageLifecycleState: 'manual-required',
    packageLifecycleProofRequirement: 'Android install launch background update and uninstall proof',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
  },
  serviceAvailability: serviceAvailability('manual-required', 'degraded', 'route-parent-mobile-lan-provider'),
  controllerProof: {
    controllerState: 'observer',
    controllerLeaseId: null,
    takeoverRequestAllowed: false,
    commandAuthorityState: 'observer-read-only',
    requestBoundary: 'observer-read-only',
  },
  assistantJobProof: {
    route: 'lan-ai-provider',
    jobState: 'degraded',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
    unavailableReason: 'lan-ai-provider-unavailable',
  },
  platformCapabilities: AndroidParentMobileCapabilities,
  localModelExecutionState: 'disabled-by-default',
  localModelExecutionAllowed: false,
  childAgentBehaviorClaim: 'not-claimed',
  updatedAt: CheckedAt,
} as const;

export const IosObserverReadModel = {
  ...AndroidObserverReadModel,
  parentDeviceId: 'parent-mobile-ios-observer',
  platform: 'ios',
  packageProof: {
    platform: 'ios',
    packageState: 'ci-mechanical-proof',
    launchTarget: 'ca.ocentra.parent.agent',
    proofCommand: 'bash scripts/release/ios/build-simulator-app.sh',
    packageLifecycleState: 'manual-required',
    packageLifecycleProofRequirement: 'iOS install launch background update and uninstall proof',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
  },
  serviceAvailability: serviceAvailability('manual-required', 'manual-required', null),
  controllerProof: {
    controllerState: 'manual-required',
    controllerLeaseId: null,
    takeoverRequestAllowed: true,
    commandAuthorityState: 'controller-takeover-manual-required',
    requestBoundary: 'request-first-manual-required',
  },
  assistantJobProof: {
    route: 'unavailable',
    jobState: 'unavailable',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: [],
    unavailableReason: 'mobile-package-proof-required',
  },
  platformCapabilities: IosParentMobileCapabilities,
} as const;

export const SubmittedLanProviderReadModel = {
  ...AndroidObserverReadModel,
  serviceAvailability: {
    ...AndroidObserverReadModel.serviceAvailability,
    lanService: 'available',
    routeStatuses: parentMobileRouteStatuses('manual-required', 'available', 'route-parent-mobile-lan-provider'),
  },
  assistantJobProof: {
    ...AndroidObserverReadModel.assistantJobProof,
    jobState: 'submitted',
    providerId: 'lan-ai-provider-family-pc',
    unavailableReason: null,
  },
} as const;

export const UnavailableLanProviderReadModel = {
  ...AndroidObserverReadModel,
  serviceAvailability: {
    ...AndroidObserverReadModel.serviceAvailability,
    lanService: 'unavailable',
    selectedRouteId: null,
    routeStatuses: parentMobileRouteStatuses('manual-required', 'unavailable', null),
  },
  assistantJobProof: {
    ...AndroidObserverReadModel.assistantJobProof,
    jobState: 'unavailable',
    providerId: null,
    unavailableReason: 'lan-ai-provider-unavailable',
  },
} as const;

function serviceAvailability(
  localService: ParentMobileServiceAvailabilityState,
  lanService: ParentMobileServiceAvailabilityState,
  selectedRouteId: ParentMobileFixtureRouteId
) {
  return {
    localService,
    lanService,
    cloudRelay: 'not-implemented',
    parentCache: 'stale',
    parentOwnedStorage: 'offline',
    selectedRouteId,
    routeStatuses: parentMobileRouteStatuses(localService, lanService, selectedRouteId),
  } as const;
}

function parentMobileRouteStatuses(
  localService: ParentMobileServiceAvailabilityState,
  lanService: ParentMobileServiceAvailabilityState,
  selectedRouteId: ParentMobileFixtureRouteId
) {
  return [
    routeStatus('local-service', localService, localService === 'unavailable' ? 'unavailable' : 'local-service', null),
    routeStatus(
      'lan-service',
      lanService,
      lanService === 'unavailable' ? 'unavailable' : 'lan-service',
      selectedRouteId
    ),
    routeStatus('cloud-relay', 'not-implemented', 'unavailable', null),
    routeStatus('parent-cache', 'stale', 'parent-cache', null),
    routeStatus('parent-owned-storage', 'offline', 'parent-owned-storage', null),
  ] as const;
}

function routeStatus(
  routeKind: 'local-service' | 'lan-service' | 'cloud-relay' | 'parent-cache' | 'parent-owned-storage',
  state: ParentMobileServiceAvailabilityState,
  custody: 'local-service' | 'lan-service' | 'unavailable' | 'parent-cache' | 'parent-owned-storage',
  selectedRouteId: ParentMobileFixtureRouteId
) {
  return {
    routeKind,
    state,
    custody,
    selectedRouteId,
    statusReason: routeStatusReason(routeKind, state),
    proofRequirement: `${routeKind} status must stay explicit in the parent mobile shell read model`,
  };
}

function routeStatusReason(
  routeKind: 'local-service' | 'lan-service' | 'cloud-relay' | 'parent-cache' | 'parent-owned-storage',
  state: ParentMobileServiceAvailabilityState
) {
  if (routeKind === 'cloud-relay') {
    return 'cloud-relay-not-implemented';
  }

  if (routeKind === 'parent-cache') {
    return state === 'unavailable' ? 'parent-cache-unavailable' : 'parent-cache-stale';
  }

  if (routeKind === 'parent-owned-storage') {
    return state === 'unavailable' ? 'parent-owned-storage-unavailable' : 'parent-owned-storage-offline';
  }

  if (state === 'available') {
    return `${routeKind}-available`;
  }

  if (state === 'degraded') {
    return `${routeKind}-degraded`;
  }

  if (state === 'unavailable') {
    return `${routeKind}-unavailable`;
  }

  return `${routeKind}-proof-required`;
}
