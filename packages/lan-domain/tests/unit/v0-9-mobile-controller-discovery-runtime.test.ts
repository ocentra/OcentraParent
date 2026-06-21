import { describe, expect, it } from 'vitest';
import { V09MobileControllerDiscoveryRuntimeReadModelSchema } from '@ocentra-parent/schema-domain/v0-9-mobile-controller-discovery-runtime';

const RuntimeReadModel = {
  schemaVersion: 'v0.9-mobile-controller-discovery-runtime',
  lanSchemaVersion: 'v0.9',
  householdDiscovery: {
    localServiceState: 'ci-mechanical-proof',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'not-implemented',
    discoveryStatesCovered: ['discovered', 'pending', 'paired', 'revoked', 'stale', 'offline', 'unavailable'],
    evidenceReferenceIds: ['evidence-v0-9-production-discovery-proof'],
  },
  mobileRouteReadModels: [
    {
      platform: 'android',
      parentDeviceId: 'parent-mobile-android-observer',
      routeId: 'route-parent-mobile-lan-provider',
      routeSource: 'local-real-service-proof',
      discoveryState: 'paired',
      reachability: 'online',
      controllerState: 'observer',
      commandAuthorityState: 'observer-read-only',
      serviceState: 'degraded',
      packageState: 'ci-mechanical-proof',
      proofLabels: ['parent-mobile.controller-observer-boundaries'],
    },
    {
      platform: 'ios',
      parentDeviceId: 'parent-mobile-ios-observer',
      routeId: null,
      routeSource: 'manual-mobile-package-required',
      discoveryState: 'unavailable',
      reachability: 'offline',
      controllerState: 'manual-required',
      commandAuthorityState: 'controller-takeover-manual-required',
      serviceState: 'manual-required',
      packageState: 'ci-mechanical-proof',
      proofLabels: ['parent-mobile.controller-observer-boundaries'],
    },
  ],
  controllerTransitions: [
    {
      transition: 'takeover',
      state: 'manual-required-mobile-package',
      rejectionReason: 'takeover-denied',
      proofLabel: 'first-child-agent:controller-lease-takeover-denied',
    },
    {
      transition: 'release',
      state: 'proved-local-service',
      rejectionReason: null,
      proofLabel: 'first-child-agent:controller-lease-released',
    },
    {
      transition: 'renew',
      state: 'proved-local-service',
      rejectionReason: null,
      proofLabel: 'first-child-agent:controller-lease-renewed',
    },
    {
      transition: 'degraded-provider',
      state: 'degraded',
      rejectionReason: 'lan-ai-provider-unavailable',
      proofLabel: 'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
    },
    {
      transition: 'failed-unpaired',
      state: 'rejected',
      rejectionReason: 'anonymous',
      proofLabel: 'first-child-agent:anonymous-rejected',
    },
  ],
  failedUnpairedBehavior: {
    reason: 'anonymous',
    proofLabel: 'first-child-agent:anonymous-rejected',
  },
  staleOfflineBehavior: [
    {
      reason: 'stale',
      proofLabel: 'rust-service:selected-device-stale-control-rejected',
    },
    {
      reason: 'offline',
      proofLabel: 'rust-service:selected-device-offline-control-rejected',
    },
  ],
  claimBoundaries: {
    physicalHouseholdLan: 'manual-required until two physical household devices and router/firewall artifacts exist',
    parentMobileWriteAuthority: 'manual-required until Android or iOS package/device controller proof exists',
    cloudRelay: 'not implemented and not counted as LAN proof',
    mobileChildAgentBehavior: 'not claimed by parent mobile controller proof',
    storesSigningEntitlements: 'manual-required until signing store and entitlement artifacts exist',
  },
  updatedAt: '2026-05-29T17:55:00.000Z',
} as const;

describe('V0.9 mobile controller discovery runtime proof contracts', () => {
  it('accepts a runtime proof that keeps mobile controller and household LAN claims honest', () => {
    const parsed = V09MobileControllerDiscoveryRuntimeReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.householdDiscovery.physicalHouseholdLanState).toBe('manual-required');
    expect(parsed.householdDiscovery.cloudRelayState).toBe('not-implemented');
    expect(parsed.mobileRouteReadModels.map((route) => route.platform)).toEqual(['android', 'ios']);
    expect(parsed.mobileRouteReadModels[0]?.commandAuthorityState).toBe('observer-read-only');
    expect(parsed.mobileRouteReadModels[1]?.commandAuthorityState).toBe('controller-takeover-manual-required');
  });

  it('rejects a mobile route that upgrades parent mobile to active write authority', () => {
    const dishonest = {
      ...RuntimeReadModel,
      mobileRouteReadModels: RuntimeReadModel.mobileRouteReadModels.map((route) =>
        route.platform === 'android'
          ? { ...route, controllerState: 'active-controller', commandAuthorityState: 'active-controller-backend-proof' }
          : route
      ),
    };

    expect(V09MobileControllerDiscoveryRuntimeReadModelSchema.safeParse(dishonest).success).toBe(false);
  });

  it('rejects household discovery proofs that omit stale/offline/manual-gated states', () => {
    const missingDiscoveryState = {
      ...RuntimeReadModel,
      householdDiscovery: {
        ...RuntimeReadModel.householdDiscovery,
        discoveryStatesCovered: ['discovered', 'pending', 'paired', 'revoked'],
      },
    };

    expect(V09MobileControllerDiscoveryRuntimeReadModelSchema.safeParse(missingDiscoveryState).success).toBe(false);
  });

  it('rejects a cloud relay or physical household LAN upgrade without real artifacts', () => {
    const cloudRelayClaim = {
      ...RuntimeReadModel,
      householdDiscovery: {
        ...RuntimeReadModel.householdDiscovery,
        cloudRelayState: 'ci-mechanical-proof',
      },
    };
    const physicalHouseholdClaim = {
      ...RuntimeReadModel,
      householdDiscovery: {
        ...RuntimeReadModel.householdDiscovery,
        physicalHouseholdLanState: 'ci-mechanical-proof',
      },
    };

    expect(V09MobileControllerDiscoveryRuntimeReadModelSchema.safeParse(cloudRelayClaim).success).toBe(false);
    expect(V09MobileControllerDiscoveryRuntimeReadModelSchema.safeParse(physicalHouseholdClaim).success).toBe(false);
  });
});
