import { describe, expect, it } from 'vitest';
import {
  ParentMobileControllerObserverHandoffRuntimeReadModelSchema,
  type ParentMobileControllerObserverHandoffPhase,
  type ParentMobileControllerObserverHandoffRouteState,
} from '../src/parent-mobile-controller-observer-handoff-runtime';

const CheckedAt = '2026-05-30T19:10:00.000Z';

const AndroidHandoffReadModel = {
  platform: 'android',
  parentDeviceId: 'parent-mobile-android-handoff',
  role: 'observer',
  leaseSnapshot: {
    leaseState: 'visible-read-only',
    controllerState: 'observer',
    commandAuthorityState: 'observer-read-only',
    controllerLeaseVisible: true,
    controllerDeviceId: 'parent-desktop-controller',
    handoffRequirement: 'lease visibility is read-only until mobile controller package authority is proven',
  },
  routeSnapshot: routeSnapshot(
    'selected-route-degraded',
    'lan-route-local-network',
    'paired',
    'online',
    'candidate-degraded',
    'degrade-busy-provider',
    'parent-desktop-busy-ai-provider'
  ),
  lanAiHandoff: lanAiHandoff('degraded', 'selected-route-degraded', ['activity-event-parent-mobile-handoff']),
  handoffSteps: handoffSteps('degraded'),
} as const;

const IosHandoffReadModel = {
  platform: 'ios',
  parentDeviceId: 'parent-mobile-ios-handoff',
  role: 'controller-candidate',
  leaseSnapshot: {
    leaseState: 'manual-required',
    controllerState: 'manual-required',
    commandAuthorityState: 'controller-takeover-manual-required',
    controllerLeaseVisible: false,
    controllerDeviceId: null,
    handoffRequirement: 'iOS controller lease handoff needs signed package entitlement and device proof',
  },
  routeSnapshot: routeSnapshot(
    'provider-unavailable',
    null,
    'unavailable',
    'offline',
    'manual-required',
    'require-physical-household-proof',
    null
  ),
  lanAiHandoff: lanAiHandoff('unavailable', 'provider-unavailable', []),
  handoffSteps: handoffSteps('unavailable'),
} as const;

const RuntimeReadModel = {
  schemaVersion: 'parent-mobile-controller-observer-handoff-proof',
  proofHarness: {
    sourceProofs: [
      proofInput(
        'parent-mobile-service-bridge-proof',
        'test-results/parent-mobile-service-bridge-proof/proof.json',
        'node scripts/test/parent-mobile-service-bridge-proof.mjs'
      ),
      proofInput(
        'v0-9-production-lan-mobile-controller-proof',
        'test-results/v0-9-production-lan-mobile-controller-proof/proof.json',
        'node scripts/test/v0-9-production-lan-mobile-controller-proof.mjs'
      ),
      proofInput(
        'v0-9-mobile-controller-discovery-runtime-proof',
        'test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json',
        'node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs'
      ),
      proofInput(
        'v0-9-prod-discovery-provider-selection-proof',
        'test-results/v0-9-prod-discovery-provider-selection-proof/proof.json',
        'node scripts/test/v0-9-prod-discovery-provider-selection-proof.mjs'
      ),
    ],
    outputProofPath: 'test-results/parent-mobile-controller-observer-handoff-proof/proof.json',
    checkpointPath: 'docs/checkpoints/parent-mobile-controller-observer-handoff-proof-2026-05-30.md',
  },
  handoffReadModels: [AndroidHandoffReadModel, IosHandoffReadModel],
  claimBoundaries: {
    parentMobileWriteAuthority: 'manual-required until real mobile controller package authority exists',
    mobileParity: 'not claimed by controller-observer handoff proof',
    childMobileAgentBehavior: 'not claimed; this is parent mobile shell proof only',
    androidDeviceOwner: 'not claimed; Android device-owner behavior belongs to child-agent proof',
    iosFamilyControls: 'not claimed; Family Controls requires entitlement and platform proof',
    signingStoresEntitlements: 'manual-required for app signing store and entitlement proof',
    cloudRelay: 'not implemented and not a fallback for LAN or mobile handoff proof',
    cUiOwnership: 'C UI can render this later; no UI or vendor path is changed here',
  },
  updatedAt: CheckedAt,
} as const;

describe('parent mobile controller observer handoff runtime proof contracts', () => {
  registerAcceptedHandoffTest();
  registerAuthorityGuardrailTests();
  registerRouteGuardrailTests();
  registerAiGuardrailTests();
  registerHarnessGuardrailTests();
});

function registerAcceptedHandoffTest(): void {
  it('accepts observer lease visibility, route/provider handoff, and degraded LAN AI without mobile authority claims', () => {
    const parsed = ParentMobileControllerObserverHandoffRuntimeReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.handoffReadModels.map((readModel) => readModel.platform)).toEqual(['android', 'ios']);
    expect(parsed.handoffReadModels[0]?.leaseSnapshot.leaseState).toBe('visible-read-only');
    expect(parsed.handoffReadModels[1]?.leaseSnapshot.leaseState).toBe('manual-required');
    expect(parsed.handoffReadModels[0]?.routeSnapshot.providerLifecycleState).toBe('candidate-degraded');
    expect(parsed.handoffReadModels[1]?.routeSnapshot.providerLifecycleState).toBe('manual-required');
    expect(parsed.handoffReadModels[0]?.lanAiHandoff.localModelExecutionAllowed).toBe(false);
    expect(parsed.handoffReadModels[0]?.handoffSteps.map((step) => step.phase)).toEqual([
      'observe-controller-lease',
      'observe-selected-route',
      'request-controller-takeover',
      'deny-controller-takeover',
      'degrade-controller-session',
      'release-controller-lease',
      'handoff-lan-ai-provider',
      'disable-phone-local-model',
      'refuse-cloud-relay',
    ]);
  });
}

function registerAuthorityGuardrailTests(): void {
  it('rejects active controller authority and accepted mobile takeover from the handoff surface', () => {
    const activeController = withMobilePatch('android', {
      leaseSnapshot: {
        ...AndroidHandoffReadModel.leaseSnapshot,
        leaseState: 'visible-read-only',
        controllerState: 'active-controller',
        commandAuthorityState: 'active-controller-backend-proof',
        controllerDeviceId: 'parent-mobile-android-handoff',
      },
    });
    const acceptedTakeover = withStep('ios', 'request-controller-takeover', {
      responseState: 'completed',
      handoffState: 'released',
      runtimeOwner: 'agent-service',
      rejectionReason: null,
    });

    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(activeController).success).toBe(false);
    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(acceptedTakeover).success).toBe(false);
  });
}

function registerRouteGuardrailTests(): void {
  it('rejects selected provider and cloud relay route upgrades without real proof', () => {
    const selectedProvider = withRoutePatch('android', {
      providerLifecycleState: 'candidate-selected',
      providerPolicyDecision: 'select-authorized-provider',
    });
    const cloudRelayClaim = withRoutePatch('ios', {
      routeState: 'cloud-relay-not-implemented',
      cloudRelayState: 'available',
    });

    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(selectedProvider).success).toBe(false);
    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(cloudRelayClaim).success).toBe(false);
  });
}

function registerAiGuardrailTests(): void {
  it('rejects phone-local model execution and submitted provider job claims from mobile', () => {
    const phoneModelClaim = withMobilePatch('android', {
      lanAiHandoff: {
        ...AndroidHandoffReadModel.lanAiHandoff,
        localModelExecutionAllowed: true,
      },
    });
    const submittedProviderClaim = withMobilePatch('android', {
      lanAiHandoff: {
        ...AndroidHandoffReadModel.lanAiHandoff,
        jobState: 'submitted',
        providerId: 'parent-desktop-controller-ai-provider',
        unavailableReason: null,
      },
    });

    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(phoneModelClaim).success).toBe(false);
    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(submittedProviderClaim).success).toBe(
      false
    );
  });
}

function registerHarnessGuardrailTests(): void {
  it('rejects missing handoff phases and incomplete source proof harnesses', () => {
    const missingPhase = withMobilePatch('android', {
      handoffSteps: AndroidHandoffReadModel.handoffSteps.filter((step) => step.phase !== 'refuse-cloud-relay'),
    });
    const missingProviderSelectionProof = {
      ...RuntimeReadModel,
      proofHarness: {
        ...RuntimeReadModel.proofHarness,
        sourceProofs: RuntimeReadModel.proofHarness.sourceProofs.filter(
          (proof) => proof.source !== 'v0-9-prod-discovery-provider-selection-proof'
        ),
      },
    };

    expect(ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(missingPhase).success).toBe(false);
    expect(
      ParentMobileControllerObserverHandoffRuntimeReadModelSchema.safeParse(missingProviderSelectionProof).success
    ).toBe(false);
  });
}

function proofInput(source: string, path: string, command: string) {
  return { source, path, command };
}

function routeSnapshot(
  routeState: ParentMobileControllerObserverHandoffRouteState,
  selectedRouteId: string | null,
  discoveryState: string,
  reachability: string,
  providerLifecycleState: string,
  providerPolicyDecision: string,
  providerId: string | null
) {
  return {
    routeState,
    selectedRouteId,
    discoveryState,
    reachability,
    providerLifecycleState,
    providerPolicyDecision,
    providerId,
    cloudRelayState: 'not-implemented',
    routeRequirement: `${routeState} must remain explicit and must not silently fall back to cloud relay`,
  };
}

function lanAiHandoff(
  jobState: 'degraded' | 'unavailable',
  routeState: ParentMobileControllerObserverHandoffRouteState,
  evidenceReferenceIds: readonly string[]
) {
  return {
    jobState,
    routeState,
    providerId: null,
    unavailableReason: jobState === 'degraded' ? 'lan-ai-provider-degraded' : 'mobile-provider-unavailable',
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
    evidenceReferenceIds,
  };
}

function handoffSteps(aiState: 'degraded' | 'unavailable') {
  return [
    step('observe-controller-lease', 'completed', 'observed-read-only', 'parent-mobile-shell', 'observer-read-only'),
    step('observe-selected-route', 'completed', 'observed-read-only', 'parent-mobile-shell', 'observer-read-only'),
    step(
      'request-controller-takeover',
      'rejected',
      'manual-required',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-controller-takeover-device-proof-required'
    ),
    step('deny-controller-takeover', 'rejected', 'denied', 'agent-service', 'observer-read-only', 'takeover-denied'),
    step(
      'degrade-controller-session',
      'degraded',
      'degraded',
      'agent-service',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      'lan-ai-provider-degraded'
    ),
    step('release-controller-lease', 'completed', 'released', 'agent-service', 'observer-read-only'),
    step(
      'handoff-lan-ai-provider',
      aiState,
      aiState,
      'lan-ai-provider',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      aiState === 'degraded' ? 'lan-ai-provider-degraded' : 'mobile-provider-unavailable'
    ),
    step(
      'disable-phone-local-model',
      'rejected',
      'disabled-by-default',
      'parent-mobile-shell',
      'observer-read-only',
      null,
      'phone-local-model-disabled-by-default'
    ),
    step(
      'refuse-cloud-relay',
      'not-implemented',
      'not-implemented',
      'cloud-relay-not-implemented',
      'observer-read-only',
      null,
      'cloud-relay-not-implemented'
    ),
  ] as const;
}

function step(
  phase: ParentMobileControllerObserverHandoffPhase,
  responseState: string,
  handoffState: string,
  runtimeOwner: string,
  commandAuthorityState: string,
  rejectionReason: string | null = null,
  unavailableReason: string | null = null
) {
  return {
    phase,
    responseState,
    handoffState,
    runtimeOwner,
    commandAuthorityState,
    rejectionReason,
    unavailableReason,
    proofLabel: `parent-mobile-controller-observer-handoff:${phase}`,
    proofRequirement: `${phase} must not upgrade parent mobile beyond current package and LAN evidence`,
  };
}

function withRoutePatch(platform: 'android' | 'ios', patch: object) {
  const readModel = platform === 'android' ? AndroidHandoffReadModel : IosHandoffReadModel;
  return withMobilePatch(platform, {
    routeSnapshot: {
      ...readModel.routeSnapshot,
      ...patch,
    },
  });
}

function withStep(platform: 'android' | 'ios', phase: ParentMobileControllerObserverHandoffPhase, patch: object) {
  const readModel = platform === 'android' ? AndroidHandoffReadModel : IosHandoffReadModel;
  return withMobilePatch(platform, {
    handoffSteps: readModel.handoffSteps.map((stepEntry) =>
      stepEntry.phase === phase ? { ...stepEntry, ...patch } : stepEntry
    ),
  });
}

function withMobilePatch(platform: 'android' | 'ios', patch: object) {
  return {
    ...RuntimeReadModel,
    handoffReadModels: RuntimeReadModel.handoffReadModels.map((readModel) =>
      readModel.platform === platform ? { ...readModel, ...patch } : readModel
    ),
  };
}
