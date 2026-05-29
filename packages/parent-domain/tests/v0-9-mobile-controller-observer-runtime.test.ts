import { describe, expect, it } from 'vitest';
import { V09MobileControllerObserverRuntimeReadModelSchema } from '../src/v0-9-mobile-controller-observer-runtime';

const CheckedAt = '2026-05-29T20:35:00.000Z';

const OperationProofs = [
  {
    operation: 'observe-status',
    intentKind: 'health-query',
    responseState: 'completed',
    operationState: 'allowed-read-only',
    runtimeOwner: 'parent-mobile-shell',
    rejectionReason: null,
    proofLabel: 'parent-mobile:observer-status-read-model',
    proofRequirement: 'typed parent mobile shell can read status without control authority',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'preview-policy-draft',
    intentKind: 'rule-query',
    responseState: 'completed',
    operationState: 'allowed-read-only',
    runtimeOwner: 'parent-mobile-shell',
    rejectionReason: null,
    proofLabel: 'parent-mobile:observer-policy-preview-read-model',
    proofRequirement: 'policy preview is read-only and does not write child runtime state',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'refresh-capabilities',
    intentKind: 'lan-ai-provider-status',
    responseState: 'completed',
    operationState: 'allowed-read-only',
    runtimeOwner: 'parent-mobile-shell',
    rejectionReason: null,
    proofLabel: 'parent-mobile:capability-refresh-read-model',
    proofRequirement: 'capability refresh only updates observer readiness labels',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'request-controller-takeover',
    intentKind: 'controller-lease-takeover',
    responseState: 'rejected',
    operationState: 'manual-required-mobile-package',
    runtimeOwner: 'manual-proof',
    rejectionReason: 'takeover-denied',
    proofLabel: 'first-child-agent:controller-lease-takeover-denied',
    proofRequirement: 'real Android or iOS package/device proof before parent mobile takeover can be accepted',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'release-controller-lease',
    intentKind: 'controller-lease-release',
    responseState: 'completed',
    operationState: 'proved-local-service',
    runtimeOwner: 'agent-service',
    rejectionReason: null,
    proofLabel: 'first-child-agent:controller-lease-released',
    proofRequirement: 'backend release transition is covered by local real-service proof, not mobile authority proof',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'submit-lan-ai-job',
    intentKind: 'lan-ai-job-submit',
    responseState: 'degraded',
    operationState: 'degraded-provider',
    runtimeOwner: 'lan-ai-provider',
    rejectionReason: 'lan-ai-provider-unavailable',
    proofLabel: 'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
    proofRequirement: 'LAN AI job submission stays degraded or unavailable until a real mobile package bridge exists',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'write-policy',
    intentKind: 'rule-update',
    responseState: 'rejected',
    operationState: 'rejected-observer-read-only',
    runtimeOwner: 'agent-service',
    rejectionReason: 'observer-read-only',
    proofLabel: 'first-child-agent:observer-policy-write-rejected',
    proofRequirement: 'observer mobile surface cannot write rules',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'approve-override',
    intentKind: 'approval-decision',
    responseState: 'rejected',
    operationState: 'rejected-observer-read-only',
    runtimeOwner: 'agent-service',
    rejectionReason: 'observer-read-only',
    proofLabel: 'first-child-agent:observer-approval-rejected',
    proofRequirement: 'observer mobile surface cannot approve overrides',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'pair-device',
    intentKind: 'configuration-update',
    responseState: 'rejected',
    operationState: 'rejected-observer-read-only',
    runtimeOwner: 'agent-service',
    rejectionReason: 'observer-read-only',
    proofLabel: 'first-child-agent:observer-pair-device-rejected',
    proofRequirement: 'observer mobile surface cannot pair devices',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
  {
    operation: 'revoke-device',
    intentKind: 'configuration-update',
    responseState: 'rejected',
    operationState: 'rejected-observer-read-only',
    runtimeOwner: 'agent-service',
    rejectionReason: 'observer-read-only',
    proofLabel: 'first-child-agent:observer-revoke-device-rejected',
    proofRequirement: 'observer mobile surface cannot revoke devices',
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  },
] as const;

const AndroidReadModel = {
  platform: 'android',
  parentDeviceId: 'parent-mobile-android-observer',
  role: 'observer',
  controllerState: 'observer',
  commandAuthorityState: 'observer-read-only',
  serviceState: 'degraded',
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
  operationProofs: OperationProofs,
} as const;

const IosReadModel = {
  ...AndroidReadModel,
  platform: 'ios',
  parentDeviceId: 'parent-mobile-ios-observer',
  role: 'controller-candidate',
  controllerState: 'manual-required',
  commandAuthorityState: 'controller-takeover-manual-required',
  serviceState: 'manual-required',
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

    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(missingSourceProof).success).toBe(false);
    expect(V09MobileControllerObserverRuntimeReadModelSchema.safeParse(cloudRelayClaim).success).toBe(false);
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

function withOperation(operation: (typeof OperationProofs)[number]['operation'], patch: object) {
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
