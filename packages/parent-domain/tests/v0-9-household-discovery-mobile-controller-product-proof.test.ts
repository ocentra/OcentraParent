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
    ...['android', 'ios'].flatMap((platform) => [
      operation(platform, 'observe-status', 'allowed-read-only', null, 'ci-mechanical-proof'),
      operation(platform, 'preview-policy-draft', 'allowed-read-only', null, 'ci-mechanical-proof'),
      operation(platform, 'refresh-capabilities', 'allowed-read-only', null, 'ci-mechanical-proof'),
      operation(
        platform,
        'request-controller-takeover',
        'manual-required-mobile-package',
        'takeover-denied',
        'manual-required'
      ),
      operation(platform, 'release-controller-lease', 'proved-local-service', null, 'ci-mechanical-proof'),
      operation(platform, 'submit-lan-ai-job', 'degraded-provider', 'lan-ai-provider-unavailable', 'degraded'),
      operation(platform, 'write-policy', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
      operation(
        platform,
        'approve-override',
        'rejected-observer-read-only',
        'observer-read-only',
        'ci-mechanical-proof'
      ),
      operation(platform, 'pair-device', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
      operation(platform, 'revoke-device', 'rejected-observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
    ]),
  ],
  controllerTransitions: [
    transition('takeover', 'manual-required-mobile-package', 'takeover-denied'),
    transition('release', 'proved-local-service', null),
    transition('renew', 'proved-local-service', null),
    transition('degraded-provider', 'degraded', 'lan-ai-provider-unavailable'),
    transition('failed-unpaired', 'rejected', 'anonymous'),
  ],
  selectedTrustedDeviceEvidence: {
    storageState: 'ci-mechanical-proof',
    securityState: 'ci-mechanical-proof',
    selectedRouteRecoveryLabels: [
      'second-child-agent:restart-restores-selected-route',
      'second-child-agent:restart-recovered-approval-accepted',
    ],
    trustedRegistryLabels: ['first-child-agent:local-json-registry', 'second-child-agent:local-json-registry'],
    selectedRouteTrustLabels: [
      'first-child-agent:selected-route-trust-state-paired',
      'second-child-agent:selected-route-trust-state-paired',
      'second-child-agent:restart-restores-selected-route-trust-state',
    ],
    selectedDeviceRejectionLabels: [
      'first-child-agent:replay-rejected',
      'second-child-agent:replay-rejected',
      'first-child-agent:stale-control-rejected',
      'second-child-agent:stale-control-rejected',
      'first-child-agent:missing-controller-lease-rejected',
      'second-child-agent:missing-controller-lease-rejected',
      'first-child-agent:route-revoked',
      'first-child-agent:revoked-control-rejected',
    ],
    wrongDeviceRejectionLabel: 'wrong-agent-port-rejected-as-wrong-device',
    proofLabel: 'trusted-device-selected-device-storage-security-proof',
  },
  auditProofCustody: {
    proofState: 'ci-mechanical-proof',
    physicalDeviceProofState: 'manual-required',
    routeAuditLabels: [
      'paired-route-accepted:ci-mechanical-proof',
      'failed-unpaired-rejected:ci-mechanical-proof',
      'wrong-origin-rejected:ci-mechanical-proof',
      'wrong-device-rejected:ci-mechanical-proof',
      'replay-rejected:ci-mechanical-proof',
      'revoked-pairing-rejected:ci-mechanical-proof',
      'stale-source-rejected:ci-mechanical-proof',
      'offline-device-rejected:ci-mechanical-proof',
      'unavailable-route-rejected:unavailable',
    ],
    observerAuditLabels: [
      ...['android', 'ios'].flatMap((platform) => [
        `${platform}:parent-mobile:observer-status-read-model`,
        `${platform}:parent-mobile:observer-policy-preview-read-model`,
        `${platform}:parent-mobile:capability-refresh-read-model`,
        `${platform}:first-child-agent:controller-lease-takeover-denied`,
        `${platform}:first-child-agent:controller-lease-released`,
        `${platform}:parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable`,
        `${platform}:first-child-agent:observer-policy-write-rejected`,
        `${platform}:first-child-agent:observer-approval-rejected`,
        `${platform}:first-child-agent:observer-pair-device-rejected`,
        `${platform}:first-child-agent:observer-revoke-device-rejected`,
      ]),
    ],
    manualBoundaryLabels: [
      'two physical household devices on the same LAN',
      'router or firewall reachability evidence',
      'allowed-origin artifact from the physical controller',
      'real Android or iOS package controller takeover artifact',
      'revocation followed by rejected control artifact',
    ],
    proofLabel: 'aggregate-route-observer-audit-proof-custody',
  },
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

    expectAcceptedReadModel(parsed);
  });

  it('rejects route, mobile, and cloud relay overclaims', () => {
    expectRejectedReadModelVariants();
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

function expectAcceptedReadModel(
  parsed: ReturnType<typeof V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.parse>
) {
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
  expect(parsed.observerOperations).toHaveLength(20);
  expect(parsed.selectedTrustedDeviceEvidence.storageState).toBe('ci-mechanical-proof');
  expect(parsed.auditProofCustody.physicalDeviceProofState).toBe('manual-required');
  expect(parsed.manualProofBoundary.physicalHouseholdLan).toBe('manual-required');
  expect(parsed.manualProofBoundary.cloudRelayImplementation).toBe('not-implemented');
}

function expectRejectedReadModelVariants() {
  expectRejectedReadModel({
    ...readModel,
    routeChecks: readModel.routeChecks.filter((entry) => entry.check !== 'wrong-origin-rejected'),
  });
  expectRejectedReadModel({
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
  });
  expectRejectedReadModel({
    ...readModel,
    routeChecks: readModel.routeChecks.map((entry) =>
      entry.check === 'revoked-pairing-rejected' ? { ...entry, rejectionReason: null } : entry
    ),
  });
  expectRejectedReadModel({
    ...readModel,
    observerOperations: readModel.observerOperations.filter(
      (entry) => entry.platform !== 'ios' || entry.operation !== 'submit-lan-ai-job'
    ),
  });
  expectRejectedReadModel({
    ...readModel,
    selectedTrustedDeviceEvidence: {
      ...readModel.selectedTrustedDeviceEvidence,
      storageState: 'manual-required',
    },
  });
  expectRejectedReadModel({
    ...readModel,
    auditProofCustody: {
      ...readModel.auditProofCustody,
      physicalDeviceProofState: 'ci-mechanical-proof',
    },
  });
  expectRejectedReadModel({
    ...readModel,
    manualProofBoundary: {
      ...readModel.manualProofBoundary,
      cloudRelayImplementation: 'ci-mechanical-proof',
    },
  });
}

function expectRejectedReadModel(candidate: unknown) {
  expect(V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema.safeParse(candidate).success).toBe(false);
}

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

function operation(
  platform: unknown,
  operationName: unknown,
  operationState: unknown,
  rejectionReason: unknown,
  proofState: unknown
) {
  return {
    platform,
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
