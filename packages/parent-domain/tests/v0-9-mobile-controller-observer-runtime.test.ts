import { describe, expect, it } from 'vitest';
import type { ParentMobileServiceAvailabilityState } from '../src/parent-mobile-runtime';
import {
  V09MobileControllerObserverRuntimeReadModelSchema,
  type V09MobileControllerObserverOperation,
  type V09MobileControllerObserverRouteKind,
} from '../src/v0-9-mobile-controller-observer-runtime';

const CheckedAt = '2026-05-29T20:35:00.000Z';

const AndroidOperationProofs = operationProofs('degraded');
const IosOperationProofs = operationProofs('unavailable');

const AndroidReadModel = {
  platform: 'android',
  parentDeviceId: 'parent-mobile-android-observer',
  role: 'observer',
  controllerState: 'observer',
  commandAuthorityState: 'observer-read-only',
  controllerLeaseProof: {
    leaseState: 'visible-read-only',
    controllerLeaseVisible: true,
    controllerLeaseId: 'lease-parent-desktop-controller-read-only',
    proofRequirement: 'parent mobile can see the controller lease but cannot write with it',
  },
  serviceState: 'degraded',
  routeStatuses: routeStatuses('manual-required', 'degraded', 'route-parent-mobile-lan-provider'),
  packageReadiness: {
    packageState: 'ci-mechanical-proof',
    runtimeState: 'ci-mechanical-proof',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
    foregroundOrBackgroundState: 'manual-required',
    notificationState: 'manual-required',
    missingCapabilityProofs: [
      'parent-mobile-controller',
      'foreground-mobile-service',
      'notifications',
      'package-lifecycle',
      'store-distribution',
    ],
  },
  capabilities: [
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
  ],
  operationProofs: AndroidOperationProofs,
} as const;

const IosReadModel = {
  ...AndroidReadModel,
  platform: 'ios',
  parentDeviceId: 'parent-mobile-ios-observer',
  role: 'controller-candidate',
  controllerState: 'manual-required',
  commandAuthorityState: 'controller-takeover-manual-required',
  controllerLeaseProof: {
    leaseState: 'manual-required',
    controllerLeaseVisible: false,
    controllerLeaseId: null,
    proofRequirement: 'iOS controller lease visibility requires signed package and device proof',
  },
  serviceState: 'manual-required',
  routeStatuses: routeStatuses('manual-required', 'manual-required', null),
  packageReadiness: {
    packageState: 'ci-mechanical-proof',
    runtimeState: 'ci-mechanical-proof',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
    foregroundOrBackgroundState: 'manual-required',
    notificationState: 'manual-required',
    missingCapabilityProofs: [
      'parent-mobile-controller',
      'notifications',
      'background-execution',
      'signing-entitlements',
      'testflight-distribution',
      'store-distribution',
    ],
  },
  operationProofs: IosOperationProofs,
} as const;

const RuntimeReadModel = {
  schemaVersion: 'v0.9-mobile-controller-observer-runtime',
  cloudRelayState: 'not-implemented',
  mobileReadModels: [AndroidReadModel, IosReadModel],
  proofHarness: {
    sourceProofs: [
      {
        source: 'parent-mobile-shell-runtime-proof',
        path: 'test-results/parent-mobile-shell-runtime-proof/proof.json',
        command: 'node scripts/test/parent-mobile-shell-runtime-proof.mjs',
      },
      {
        source: 'v0-9-production-lan-mobile-controller-proof',
        path: 'test-results/v0-9-production-lan-mobile-controller-proof/proof.json',
        command: 'node scripts/test/v0-9-production-lan-mobile-controller-proof.mjs',
      },
      {
        source: 'v0-9-mobile-controller-discovery-runtime-proof',
        path: 'test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json',
        command: 'node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs',
      },
    ],
    outputProofPath: 'test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json',
    checkpointPath: 'docs/checkpoints/v0-9-mobile-controller-observer-runtime-proof-2026-05-29.md',
  },
  claimBoundaries: {
    parentMobileWriteAuthority: 'manual-required until real Android or iOS package/device controller proof exists',
    physicalHouseholdLan: 'manual-required until two physical devices and router/firewall artifacts exist',
    cloudRelay: 'not implemented and not counted as LAN proof',
    childAgentBehavior: 'not claimed by parent mobile observer runtime proof',
    mobileChildAgentParity:
      'not claimed; parent mobile observer runtime does not prove Android or iOS child-agent parity',
    signingStoresEntitlements: 'manual-required until signing store and entitlement artifacts exist',
    cUiOwnership: 'C UI can render the contract later but this proof does not touch UI or vendor paths',
  },
  updatedAt: CheckedAt,
} as const;

describe('V0.9 mobile controller observer runtime proof contracts', () => {
  registerAcceptedStateTests();
  registerAuthorityGuardrailTests();
  registerOperationGuardrailTests();
  registerProofHarnessGuardrailTests();
});

function registerAcceptedStateTests(): void {
  it('accepts observer read-only, manual takeover, release, and degraded provider states', () => {
    const parsed = V09MobileControllerObserverRuntimeReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.cloudRelayState).toBe('not-implemented');
    expect(parsed.mobileReadModels.map((readModel) => readModel.platform)).toEqual(['android', 'ios']);
    expect(parsed.mobileReadModels[0]?.controllerLeaseProof.leaseState).toBe('visible-read-only');
    expect(parsed.mobileReadModels[1]?.controllerLeaseProof.leaseState).toBe('manual-required');
    expect(parsed.mobileReadModels[0]?.routeStatuses.map((route) => route.routeKind)).toEqual([
      'local-service',
      'lan-service',
      'cloud-relay',
      'parent-cache',
      'parent-owned-storage',
    ]);
    expect(parsed.mobileReadModels[0]?.operationProofs.map((proof) => proof.operation)).toEqual([
      'observe-status',
      'preview-policy-draft',
      'refresh-capabilities',
      'request-controller-takeover',
      'release-controller-lease',
      'submit-lan-ai-job',
      'write-policy',
      'approve-override',
      'pair-device',
      'revoke-device',
    ]);
    expect(
      parsed.mobileReadModels.map(
        (readModel) =>
          readModel.operationProofs.find((proof) => proof.operation === 'submit-lan-ai-job')?.operationState
      )
    ).toEqual(['degraded-provider', 'unavailable']);
    expect(parsed.proofHarness.sourceProofs.map((proof) => proof.source)).toEqual([
      'parent-mobile-shell-runtime-proof',
      'v0-9-production-lan-mobile-controller-proof',
      'v0-9-mobile-controller-discovery-runtime-proof',
    ]);
  });
}

function registerAuthorityGuardrailTests(): void {
  it('rejects a parent mobile route upgraded to active write authority', () => {
    const activeControllerClaim = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) =>
        readModel.platform === 'android'
          ? {
              ...readModel,
              controllerState: 'active-controller',
              commandAuthorityState: 'active-controller-backend-proof',
            }
          : readModel
      ),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(activeControllerClaim).success).toBe(false);
  });

  it('rejects hidden controller lease write authority on an observer route', () => {
    const writableLeaseClaim = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) =>
        readModel.platform === 'android'
          ? {
              ...readModel,
              controllerLeaseProof: {
                ...readModel.controllerLeaseProof,
                leaseState: 'visible-read-only',
                controllerLeaseVisible: true,
                controllerLeaseId: null,
              },
            }
          : readModel
      ),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(writableLeaseClaim).success).toBe(false);
  });
}

function registerOperationGuardrailTests(): void {
  it('rejects observer write operations presented as completed read-only work', () => {
    const acceptedWriteClaim = withOperation('write-policy', {
      responseState: 'completed',
      operationState: 'allowed-read-only',
      runtimeOwner: 'parent-mobile-shell',
      rejectionReason: null,
    });

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(acceptedWriteClaim).success).toBe(false);
  });

  it('rejects controller takeover accepted from scaffold package evidence', () => {
    const acceptedTakeoverClaim = withOperation('request-controller-takeover', {
      responseState: 'completed',
      operationState: 'proved-local-service',
      runtimeOwner: 'agent-service',
      rejectionReason: null,
    });

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(acceptedTakeoverClaim).success).toBe(false);
  });

  it('rejects missing degraded LAN AI and controller release operation coverage', () => {
    const missingOperations = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) => ({
        ...readModel,
        operationProofs: readModel.operationProofs.filter(
          (proof) => proof.operation !== 'submit-lan-ai-job' && proof.operation !== 'release-controller-lease'
        ),
      })),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(missingOperations).success).toBe(false);
  });

  it('rejects proofs that do not cover both degraded and unavailable LAN AI provider states', () => {
    const missingUnavailableState = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) => ({
        ...readModel,
        operationProofs: readModel.operationProofs.map((proof) =>
          proof.operation === 'submit-lan-ai-job'
            ? {
                ...proof,
                responseState: 'degraded',
                operationState: 'degraded-provider',
              }
            : proof
        ),
      })),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(missingUnavailableState).success).toBe(false);
  });
}

function registerProofHarnessGuardrailTests(): void {
  it('rejects incomplete proof harnesses and cloud relay upgrades', () => {
    const missingSourceProof = {
      ...RuntimeReadModel,
      proofHarness: {
        ...RuntimeReadModel.proofHarness,
        sourceProofs: RuntimeReadModel.proofHarness.sourceProofs.filter(
          (proof) => proof.source !== 'v0-9-mobile-controller-discovery-runtime-proof'
        ),
      },
    };
    const cloudRelayClaim = {
      ...RuntimeReadModel,
      cloudRelayState: 'ci-mechanical-proof',
    };
    const cacheFreshnessClaim = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) =>
        readModel.platform === 'android'
          ? {
              ...readModel,
              routeStatuses: readModel.routeStatuses.map((route) =>
                route.routeKind === 'parent-cache' ? { ...route, state: 'available' } : route
              ),
            }
          : readModel
      ),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(missingSourceProof).success).toBe(false);
    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(cloudRelayClaim).success).toBe(false);
    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(cacheFreshnessClaim).success).toBe(false);
  });

  it('rejects package readiness without explicit manual capability gaps', () => {
    const missingPackageGaps = {
      ...RuntimeReadModel,
      mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) =>
        readModel.platform === 'ios'
          ? {
              ...readModel,
              packageReadiness: {
                ...readModel.packageReadiness,
                missingCapabilityProofs: [],
              },
            }
          : readModel
      ),
    };

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(missingPackageGaps).success).toBe(false);
  });
}

function operationProofs(aiState: 'degraded' | 'unavailable') {
  return [
    ...readOnlyOperationProofs(),
    ...controllerOperationProofs(),
    lanAiOperationProof(aiState),
    ...observerRejectedOperationProofs(),
  ] as const;
}

function readOnlyOperationProofs() {
  return [
    operationProof(
      'observe-status',
      'health-query',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:observer-status-read-model',
      'typed parent mobile shell can read status without control authority'
    ),
    operationProof(
      'preview-policy-draft',
      'rule-query',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:observer-policy-preview-read-model',
      'policy preview is read-only and does not write child runtime state'
    ),
    operationProof(
      'refresh-capabilities',
      'lan-ai-provider-status',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:capability-refresh-read-model',
      'capability refresh only updates observer readiness labels'
    ),
  ] as const;
}

function controllerOperationProofs() {
  return [
    operationProof(
      'request-controller-takeover',
      'controller-lease-takeover',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'takeover-denied',
      'first-child-agent:controller-lease-takeover-denied',
      'real Android or iOS package/device proof before parent mobile takeover can be accepted'
    ),
    operationProof(
      'release-controller-lease',
      'controller-lease-release',
      'completed',
      'proved-local-service',
      'agent-service',
      null,
      'first-child-agent:controller-lease-released',
      'backend release transition is covered by local real-service proof, not mobile authority proof'
    ),
  ] as const;
}

function observerRejectedOperationProofs() {
  return [
    operationProof(
      'write-policy',
      'rule-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-policy-write-rejected',
      'observer mobile surface cannot write rules'
    ),
    operationProof(
      'approve-override',
      'approval-decision',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-approval-rejected',
      'observer mobile surface cannot approve overrides'
    ),
    operationProof(
      'pair-device',
      'configuration-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-pair-device-rejected',
      'observer mobile surface cannot pair devices'
    ),
    operationProof(
      'revoke-device',
      'configuration-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-revoke-device-rejected',
      'observer mobile surface cannot revoke devices'
    ),
  ] as const;
}

function lanAiOperationProof(aiState: 'degraded' | 'unavailable') {
  return operationProof(
    'submit-lan-ai-job',
    'lan-ai-job-submit',
    'degraded',
    aiState === 'degraded' ? 'degraded-provider' : 'unavailable',
    'lan-ai-provider',
    'lan-ai-provider-unavailable',
    aiState === 'degraded'
      ? 'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable'
      : 'parent-mobile-observer-scaffold:controller-job-unavailable-with-provider-unavailable',
    'LAN AI job submission stays degraded or unavailable until a real mobile package bridge exists'
  );
}

function operationProof(
  operation: V09MobileControllerObserverOperation,
  intentKind: string,
  responseState: string,
  operationState: string,
  runtimeOwner: string,
  rejectionReason: string | null,
  proofLabel: string,
  proofRequirement: string
) {
  return {
    operation,
    intentKind,
    responseState,
    operationState,
    runtimeOwner,
    rejectionReason,
    proofLabel,
    proofRequirement,
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  };
}

function withOperation(operation: V09MobileControllerObserverOperation, patch: object) {
  return {
    ...RuntimeReadModel,
    mobileReadModels: RuntimeReadModel.mobileReadModels.map((readModel) => ({
      ...readModel,
      operationProofs: readModel.operationProofs.map((proof) =>
        proof.operation === operation ? { ...proof, ...patch } : proof
      ),
    })),
  };
}

function routeStatuses(
  localService: ParentMobileServiceAvailabilityState,
  lanService: ParentMobileServiceAvailabilityState,
  selectedRouteId: 'route-parent-mobile-lan-provider' | null
) {
  return [
    routeStatus('local-service', localService, null),
    routeStatus('lan-service', lanService, selectedRouteId),
    routeStatus('cloud-relay', 'not-implemented', null),
    routeStatus('parent-cache', 'stale', null),
    routeStatus('parent-owned-storage', 'offline', null),
  ] as const;
}

function routeStatus(
  routeKind: V09MobileControllerObserverRouteKind,
  state: ParentMobileServiceAvailabilityState,
  selectedRouteId: 'route-parent-mobile-lan-provider' | null
) {
  return {
    routeKind,
    state,
    selectedRouteId,
    proofRequirement: `${routeKind} status must stay explicit in the V0.9 parent mobile observer runtime`,
  };
}
