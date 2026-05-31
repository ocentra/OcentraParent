import { describe, expect, it } from 'vitest';
import {
  V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema,
  V09HouseholdDiscoveryMobileControllerRouteCheckSchema,
  V09HouseholdDiscoveryMobileControllerSourceProofSchema,
} from '../src/v0-9-household-discovery-mobile-controller-product-proof';

const checkedAt = '2026-05-30T23:58:00.000Z';
const routeId = 'route-v0-9-household-mobile-product-proof';

const readModel = {
  schemaVersion: 'v0-9-household-discovery-mobile-controller-product-proof',
  checkedAt,
  sourceProofs: [
    proofInput('v0-9-production-discovery-household-proof'),
    proofInput('v0-9-production-lan-mobile-controller-proof'),
    proofInput('v0-9-mobile-controller-discovery-runtime-proof'),
    proofInput('v0-9-mobile-controller-observer-runtime-proof'),
    proofInput('parent-mobile-controller-observer-handoff-proof'),
  ],
  productionDiscoveryStates: ['discovered', 'pending', 'paired', 'revoked', 'stale', 'offline', 'unavailable'],
  routeChecks: [
    routeCheck('paired-route-accepted', 'paired', 'paired', 'online', null, 'ci-mechanical-proof'),
    routeCheck('failed-unpaired-rejected', 'unavailable', 'unpaired', 'online', 'anonymous', 'ci-mechanical-proof'),
    routeCheck('wrong-origin-rejected', 'unavailable', 'paired', 'online', 'wrong-origin', 'ci-mechanical-proof'),
    routeCheck('wrong-device-rejected', 'unavailable', 'paired', 'online', 'wrong-device', 'ci-mechanical-proof'),
    routeCheck('replay-rejected', 'unavailable', 'paired', 'online', 'replayed', 'ci-mechanical-proof'),
    routeCheck('revoked-pairing-rejected', 'revoked', 'revoked', 'online', 'revoked', 'ci-mechanical-proof'),
    routeCheck('stale-source-rejected', 'stale', 'paired', 'stale', 'stale', 'ci-mechanical-proof'),
    routeCheck('offline-device-rejected', 'offline', 'paired', 'offline', 'offline', 'ci-mechanical-proof'),
    routeCheck('unavailable-route-rejected', 'unavailable', 'paired', 'online', 'unsupported-route', 'unavailable'),
  ],
  mobileRoutes: [
    mobileRoute('android', routeId, 'paired', 'online', 'observer', 'observer-read-only', 'degraded'),
    mobileRoute(
      'ios',
      null,
      'unavailable',
      'offline',
      'manual-required',
      'controller-takeover-manual-required',
      'manual-required'
    ),
  ],
  observerOperations: [
    operation('observe-status', 'allowed-read-only', null, 'ci-mechanical-proof'),
    operation('request-controller-takeover', 'manual-required-mobile-package', 'takeover-denied', 'manual-required'),
    operation('write-policy', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
    operation('pair-device', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
    operation('revoke-device', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
  ],
  controllerTransitions: [
    transition('takeover', 'manual-required-mobile-package', 'takeover-denied'),
    transition('release', 'proved-local-service', null),
    transition('renew', 'proved-local-service', null),
    transition('degraded-provider', 'degraded', 'lan-ai-provider-unavailable'),
    transition('failed-unpaired', 'rejected', 'anonymous'),
  ],
  manualProofBoundary: {
    physicalHouseholdLan: 'manual-required',
    parentMobileWriteAuthority: 'manual-required',
    cloudRelayImplementation: 'not-implemented',
    cloudRelayDecision: 'manual-decision-required',
    mobileBackgroundBehavior: 'manual-required',
    physicalDeviceChecklist: [
      'two physical household devices on the same LAN',
      'router or firewall reachability evidence',
      'allowed-origin artifact from the physical controller',
      'real Android or iOS package controller takeover artifact',
      'revocation followed by rejected control artifact',
    ],
  },
  claimsProved: ['local service route and mobile observer states are mechanically composed'],
  claimsNotProved: [
    'physical household LAN readiness',
    'real parent mobile write authority',
    'cloud relay routing or authentication',
  ],
} as const;

describe('V0.9 household discovery mobile controller product proof contracts', () => {
  it('accepts an aggregate proof that keeps physical household and mobile authority manual-required', () => {
    const parsed = V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.parse(readModel);

    expect(parsed.sourceProofs.map((proof) => proof.source)).toEqual([
      'v0-9-production-discovery-household-proof',
      'v0-9-production-lan-mobile-controller-proof',
      'v0-9-mobile-controller-discovery-runtime-proof',
      'v0-9-mobile-controller-observer-runtime-proof',
      'parent-mobile-controller-observer-handoff-proof',
    ]);
    expect(parsed.productionDiscoveryStates).toEqual([
      'discovered',
      'pending',
      'paired',
      'revoked',
      'stale',
      'offline',
      'unavailable',
    ]);
    expect(parsed.mobileRoutes.map((route) => route.commandAuthorityState)).toEqual([
      'observer-read-only',
      'controller-takeover-manual-required',
    ]);
    expect(parsed.manualProofBoundary.physicalHouseholdLan).toBe('manual-required');
    expect(parsed.manualProofBoundary.cloudRelayImplementation).toBe('not-implemented');
  });

  it('rejects route, mobile, and cloud relay overclaims', () => {
    expect(
      V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.safeParse({
        ...readModel,
        routeChecks: readModel.routeChecks.filter((entry) => entry.check !== 'wrong-origin-rejected'),
      }).success
    ).toBe(false);
    expect(
      V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.safeParse({
        ...readModel,
        mobileRoutes: readModel.mobileRoutes.map((route) =>
          route.platform === 'android'
            ? {
                ...route,
                controllerState: 'active-controller',
                commandAuthorityState: 'active-controller-backend-proof',
              }
            : route
        ),
      }).success
    ).toBe(false);
    expect(
      V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.safeParse({
        ...readModel,
        manualProofBoundary: {
          ...readModel.manualProofBoundary,
          cloudRelayImplementation: 'ci-mechanical-proof',
        },
      }).success
    ).toBe(false);
  });

  it('keeps source proof and route check vocabularies explicit', () => {
    expect(
      V09HouseholdDiscoveryMobileControllerSourceProofSchema.parse('parent-mobile-controller-observer-handoff-proof')
    ).toBe('parent-mobile-controller-observer-handoff-proof');
    expect(V09HouseholdDiscoveryMobileControllerRouteCheckSchema.parse('replay-rejected')).toBe('replay-rejected');
    expect(
      V09HouseholdDiscoveryMobileControllerRouteCheckSchema.safeParse('physical-household-route-accepted').success
    ).toBe(false);
  });
});

function proofInput(source: unknown) {
  return {
    source,
    path: `test-results/${source}/proof.json`,
    command: `node scripts/test/${source}.mjs`,
  };
}

function routeCheck(
  check: unknown,
  discoveryState: unknown,
  trustState: unknown,
  reachability: unknown,
  rejectionReason: unknown,
  proofState: unknown
) {
  return {
    check,
    routeId,
    discoveryState,
    trustState,
    reachability,
    rejectionReason,
    proofState,
    proofLabel: `${String(check)} proof`,
  };
}

function mobileRoute(
  platform: unknown,
  mobileRouteId: unknown,
  discoveryState: unknown,
  reachability: unknown,
  controllerState: unknown,
  commandAuthorityState: unknown,
  serviceState: unknown
) {
  return {
    platform,
    routeId: mobileRouteId,
    discoveryState,
    reachability,
    controllerState,
    commandAuthorityState,
    serviceState,
    proofState: platform === 'android' ? 'ci-mechanical-proof' : 'manual-required',
    proofLabel: `${String(platform)} parent mobile route proof`,
  };
}

function operation(operationName: unknown, operationState: unknown, rejectionReason: unknown, proofState: unknown) {
  return {
    operation: operationName,
    operationState,
    rejectionReason,
    proofState,
    proofLabel: `${String(operationName)} operation proof`,
  };
}

function transition(transitionName: unknown, state: unknown, rejectionReason: unknown) {
  return {
    transition: transitionName,
    state,
    rejectionReason,
    proofLabel: `${String(transitionName)} transition proof`,
  };
}
