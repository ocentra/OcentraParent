import { describe, expect, it } from 'vitest';
import {
  ParentMobileServiceBridgeRuntimeReadModelSchema,
  type ParentMobileServiceBridgeAssistantJobState,
  type ParentMobileServiceBridgeConnectionKind,
  type ParentMobileServiceBridgeOperation,
} from '../src/parent-mobile-service-bridge-runtime';

const CheckedAt = '2026-05-29T23:45:00.000Z';

const AndroidCapabilities = [
  capabilityState(
    'parent-mobile-observer',
    'scaffold',
    'typed observer read model and package launch target',
    'observer state is represented without mobile UX parity'
  ),
  capabilityState(
    'parent-mobile-controller',
    'manual-required',
    'real mobile package and device controller takeover proof',
    'no parent mobile write authority is claimed from scaffold state'
  ),
  capabilityState(
    'foreground-mobile-service',
    'manual-required',
    'Android emulator or device foreground-service and notification proof',
    'manifest declaration is not foreground behavior proof'
  ),
  capabilityState(
    'notifications',
    'manual-required',
    'Android notification permission prompt and delivery proof',
    'permission declaration is not runtime notification proof'
  ),
  capabilityState(
    'package-lifecycle',
    'manual-required',
    'Android install launch background update and uninstall proof',
    'debug package mechanics are not store or lifecycle proof'
  ),
  capabilityState(
    'store-distribution',
    'planned',
    'Google Play signing and release-track proof',
    'store distribution is not wired'
  ),
] as const;

const IosCapabilities = [
  capabilityState(
    'parent-mobile-observer',
    'scaffold',
    'typed observer read model and simulator app target',
    'observer state is represented without mobile UX parity'
  ),
  capabilityState(
    'parent-mobile-controller',
    'manual-required',
    'real signed mobile package and device controller takeover proof',
    'no parent mobile write authority is claimed from simulator scaffold'
  ),
  capabilityState(
    'foreground-mobile-service',
    'unavailable',
    'iOS has no Android-style foreground service',
    'foreground service is not an iOS parent mobile claim'
  ),
  capabilityState(
    'notifications',
    'manual-required',
    'iOS notification permission and delivery proof',
    'notification behavior requires device or simulator permission evidence'
  ),
  capabilityState(
    'background-execution',
    'manual-required',
    'iOS background mode entitlement and device behavior proof',
    'simulator app target is not background execution proof'
  ),
  capabilityState(
    'signing-entitlements',
    'manual-required',
    'Apple signing team provisioning and entitlement proof',
    'simulator build is not signing or entitlement proof'
  ),
  capabilityState(
    'testflight-distribution',
    'manual-required',
    'TestFlight build upload install and launch proof',
    'TestFlight distribution is not wired'
  ),
  capabilityState('store-distribution', 'planned', 'App Store release-track proof', 'store distribution is not wired'),
] as const;

const AndroidReadModel = {
  platform: 'android',
  parentDeviceId: 'parent-mobile-android-service-bridge',
  role: 'observer',
  controllerState: 'observer',
  commandAuthorityState: 'observer-read-only',
  connections: serviceConnections('degraded'),
  packageReadiness: {
    platform: 'android',
    packageState: 'ci-mechanical-proof',
    serviceLaunchState: 'manual-required',
    launchTarget: 'ca.ocentra.parent.agent/.MainActivity',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
    missingCapabilityProofs: [
      'parent-mobile-controller',
      'foreground-mobile-service',
      'notifications',
      'package-lifecycle',
      'store-distribution',
    ],
  },
  aiSubmission: {
    route: 'lan-ai-provider',
    jobState: 'degraded',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
    unavailableReason: 'lan-ai-provider-unavailable',
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
  },
  capabilities: AndroidCapabilities,
  operationProofs: operationProofs('degraded'),
} as const;

const IosReadModel = {
  platform: 'ios',
  parentDeviceId: 'parent-mobile-ios-service-bridge',
  role: 'controller-candidate',
  controllerState: 'manual-required',
  commandAuthorityState: 'controller-takeover-manual-required',
  connections: serviceConnections('manual-required'),
  packageReadiness: {
    platform: 'ios',
    packageState: 'ci-mechanical-proof',
    serviceLaunchState: 'manual-required',
    launchTarget: 'ca.ocentra.parent.agent',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
    missingCapabilityProofs: [
      'parent-mobile-controller',
      'notifications',
      'background-execution',
      'signing-entitlements',
      'testflight-distribution',
      'store-distribution',
    ],
  },
  aiSubmission: {
    route: 'unavailable',
    jobState: 'unavailable',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: [],
    unavailableReason: 'mobile-package-service-bridge-required',
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
  },
  capabilities: IosCapabilities,
  operationProofs: operationProofs('unavailable'),
} as const;

const RuntimeReadModel = {
  schemaVersion: 'parent-mobile-service-bridge-proof',
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
        source: 'v0-9-mobile-controller-observer-runtime-proof',
        path: 'test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json',
        command: 'node scripts/test/v0-9-mobile-controller-observer-runtime-proof.mjs',
      },
    ],
    outputProofPath: 'test-results/parent-mobile-service-bridge-proof/proof.json',
    checkpointPath: 'docs/checkpoints/parent-mobile-service-bridge-proof-2026-05-29.md',
  },
  mobileBridgeReadModels: [AndroidReadModel, IosReadModel],
  claimBoundaries: {
    parentMobileWriteAuthority: 'manual-required until real Android or iOS package/device controller proof exists',
    physicalHouseholdLan: 'manual-required until two physical devices and router/firewall artifacts exist',
    cloudRelay: 'not implemented and not counted as local, LAN, or mobile proof',
    phoneLocalModel: 'disabled by default; parent mobile does not load a phone-local model for assistant work',
    packageServiceLaunch: 'manual-required until foreground/background mobile service launch is proven on device',
    cUiOwnership: 'C UI can render this later, but this proof does not touch UI or vendor paths',
  },
  updatedAt: CheckedAt,
} as const;

describe('parent mobile service bridge runtime proof contracts', () => {
  registerAcceptedStateTests();
  registerBridgeClaimGuardrailTests();
  registerOperationGuardrailTests();
  registerPackageAndHarnessGuardrailTests();
});

function registerAcceptedStateTests(): void {
  it('accepts local/LAN service, observer authority, package gaps, and disabled phone-local model state', () => {
    const parsed = ParentMobileServiceBridgeRuntimeReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.mobileBridgeReadModels.map((readModel) => readModel.platform)).toEqual(['android', 'ios']);
    expect(parsed.mobileBridgeReadModels[0]?.connections.map((connection) => connection.connectionKind)).toEqual([
      'local-service',
      'lan-service',
      'cloud-relay',
      'mobile-package',
    ]);
    expect(parsed.mobileBridgeReadModels[0]?.aiSubmission.jobState).toBe('degraded');
    expect(parsed.mobileBridgeReadModels[1]?.aiSubmission.jobState).toBe('unavailable');
    expect(parsed.mobileBridgeReadModels[0]?.operationProofs.map((proof) => proof.operation)).toEqual([
      'service-status-read',
      'lan-route-status-read',
      'capability-refresh',
      'package-service-launch',
      'controller-takeover-request',
      'controller-release',
      'write-policy',
      'approval-decision',
      'submit-lan-ai-job',
      'submit-cloud-relay-job',
      'submit-phone-local-model-job',
    ]);
  });
}

function registerBridgeClaimGuardrailTests(): void {
  it('rejects cloud relay availability and parent mobile local-service availability claims', () => {
    const cloudRelayClaim = withConnection('android', 'cloud-relay', { state: 'available' });
    const localServiceClaim = withConnection('android', 'local-service', { state: 'available' });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(cloudRelayClaim).success).toBe(false);
    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(localServiceClaim).success).toBe(false);
  });

  it('rejects phone-local model execution and child-service provider submit claims from mobile', () => {
    const localModelClaim = withMobilePatch('android', {
      aiSubmission: {
        ...AndroidReadModel.aiSubmission,
        localModelExecutionAllowed: true,
      },
    });
    const submittedProviderClaim = withMobilePatch('android', {
      aiSubmission: {
        ...AndroidReadModel.aiSubmission,
        jobState: 'degraded',
        providerId: 'lan-ai-provider-family-pc',
      },
    });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(localModelClaim).success).toBe(false);
    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(submittedProviderClaim).success).toBe(false);
  });

  it('rejects active controller authority from the parent mobile service bridge', () => {
    const activeControllerClaim = withMobilePatch('android', {
      controllerState: 'active-controller',
      commandAuthorityState: 'active-controller-backend-proof',
    });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(activeControllerClaim).success).toBe(false);
  });
}

function registerOperationGuardrailTests(): void {
  it('rejects observer write and approval operations presented as completed work', () => {
    const acceptedWriteClaim = withOperation('android', 'write-policy', {
      responseState: 'completed',
      operationState: 'allowed-read-only',
      runtimeOwner: 'parent-mobile-shell',
      rejectionReason: null,
    });
    const acceptedApprovalClaim = withOperation('android', 'approval-decision', {
      responseState: 'completed',
      operationState: 'allowed-read-only',
      runtimeOwner: 'parent-mobile-shell',
      rejectionReason: null,
    });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(acceptedWriteClaim).success).toBe(false);
    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(acceptedApprovalClaim).success).toBe(false);
  });

  it('rejects missing LAN AI, cloud relay, or phone-local model operation coverage', () => {
    const missingOperations = withMobilePatch('android', {
      operationProofs: AndroidReadModel.operationProofs.filter(
        (proof) =>
          proof.operation !== 'submit-lan-ai-job' &&
          proof.operation !== 'submit-cloud-relay-job' &&
          proof.operation !== 'submit-phone-local-model-job'
      ),
    });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(missingOperations).success).toBe(false);
  });
}

function registerPackageAndHarnessGuardrailTests(): void {
  it('rejects package readiness without explicit mobile service launch gaps', () => {
    const packageGapClaim = withMobilePatch('ios', {
      packageReadiness: {
        ...IosReadModel.packageReadiness,
        serviceLaunchState: 'ci-mechanical-proof',
        missingCapabilityProofs: [],
      },
    });

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(packageGapClaim).success).toBe(false);
  });

  it('rejects incomplete source-proof harnesses', () => {
    const missingObserverProof = {
      ...RuntimeReadModel,
      proofHarness: {
        ...RuntimeReadModel.proofHarness,
        sourceProofs: RuntimeReadModel.proofHarness.sourceProofs.filter(
          (proof) => proof.source !== 'v0-9-mobile-controller-observer-runtime-proof'
        ),
      },
    };

    expect(ParentMobileServiceBridgeRuntimeReadModelSchema.safeParse(missingObserverProof).success).toBe(false);
  });
}

function serviceConnections(lanState: 'degraded' | 'manual-required') {
  return [
    connection('local-service', 'manual-required', 'manual-proof', null),
    connection('lan-service', lanState, 'lan-ai-provider', 'route-parent-mobile-lan-provider'),
    connection('cloud-relay', 'not-implemented', 'cloud-relay-not-implemented', null),
    connection('mobile-package', 'ci-mechanical-proof', 'parent-mobile-shell', null),
  ] as const;
}

function connection(
  connectionKind: ParentMobileServiceBridgeConnectionKind,
  state: 'manual-required' | 'degraded' | 'not-implemented' | 'ci-mechanical-proof',
  runtimeOwner: 'manual-proof' | 'lan-ai-provider' | 'cloud-relay-not-implemented' | 'parent-mobile-shell',
  selectedRouteId: 'route-parent-mobile-lan-provider' | null
) {
  return {
    connectionKind,
    state,
    runtimeOwner,
    selectedRouteId,
    proofLabel: `parent-mobile-service-bridge:${connectionKind}`,
    proofRequirement: `${connectionKind} state must stay explicit in the parent mobile service bridge`,
  };
}

function operationProofs(aiState: Extract<ParentMobileServiceBridgeAssistantJobState, 'degraded' | 'unavailable'>) {
  return [...readOnlyOperationProofs(), ...authorityOperationProofs(), ...aiAndRelayOperationProofs(aiState)] as const;
}

function readOnlyOperationProofs() {
  return [
    operationProof(
      'service-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof(
      'lan-route-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof('capability-refresh', 'completed', 'allowed-read-only', 'parent-mobile-shell', 'observer-read-only'),
  ] as const;
}

function authorityOperationProofs() {
  return [
    operationProof(
      'package-service-launch',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-package-service-launch-proof-required'
    ),
    operationProof(
      'controller-takeover-request',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-controller-takeover-device-proof-required'
    ),
    operationProof('controller-release', 'completed', 'proved-local-service', 'agent-service', 'observer-read-only'),
    operationProof(
      'write-policy',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
    operationProof(
      'approval-decision',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
  ] as const;
}

function aiAndRelayOperationProofs(
  aiState: Extract<ParentMobileServiceBridgeAssistantJobState, 'degraded' | 'unavailable'>
) {
  return [
    operationProof(
      'submit-lan-ai-job',
      aiState,
      aiState === 'degraded' ? 'degraded-provider' : 'unavailable',
      'lan-ai-provider',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      aiState === 'degraded' ? 'lan-ai-provider-degraded' : 'mobile-package-service-bridge-required'
    ),
    operationProof(
      'submit-cloud-relay-job',
      'not-implemented',
      'not-implemented',
      'cloud-relay-not-implemented',
      'observer-read-only',
      null,
      'cloud-relay-not-implemented'
    ),
    operationProof(
      'submit-phone-local-model-job',
      'rejected',
      'rejected-no-phone-local-model',
      'parent-mobile-shell',
      'observer-read-only',
      null,
      'phone-local-model-disabled-by-default'
    ),
  ] as const;
}

function operationProof(
  operation: ParentMobileServiceBridgeOperation,
  responseState: 'completed' | 'rejected' | 'degraded' | 'unavailable' | 'not-implemented',
  operationState:
    | 'allowed-read-only'
    | 'proved-local-service'
    | 'rejected-observer-read-only'
    | 'manual-required-mobile-package'
    | 'degraded-provider'
    | 'unavailable'
    | 'not-implemented'
    | 'rejected-no-phone-local-model',
  runtimeOwner:
    | 'parent-mobile-shell'
    | 'agent-service'
    | 'lan-ai-provider'
    | 'manual-proof'
    | 'cloud-relay-not-implemented',
  commandAuthorityState: 'observer-read-only' | 'controller-takeover-manual-required',
  rejectionReason: 'observer-read-only' | 'takeover-denied' | 'lan-ai-provider-unavailable' | null = null,
  unavailableReason: string | null = null
) {
  return {
    operation,
    responseState,
    operationState,
    runtimeOwner,
    commandAuthorityState,
    rejectionReason,
    unavailableReason,
    proofLabel: `parent-mobile-service-bridge:${operation}`,
    proofRequirement: `${operation} proof must not upgrade parent mobile beyond current bridge evidence`,
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  };
}

function capabilityState(capability: string, status: string, proofRequirement: string, claimBoundary: string) {
  return { capability, status, proofRequirement, claimBoundary };
}

function withConnection(platform: 'android' | 'ios', kind: ParentMobileServiceBridgeConnectionKind, patch: object) {
  return withMobilePatch(platform, {
    connections: (platform === 'android' ? AndroidReadModel.connections : IosReadModel.connections).map((connection) =>
      connection.connectionKind === kind ? { ...connection, ...patch } : connection
    ),
  });
}

function withOperation(platform: 'android' | 'ios', operation: ParentMobileServiceBridgeOperation, patch: object) {
  return withMobilePatch(platform, {
    operationProofs: (platform === 'android' ? AndroidReadModel.operationProofs : IosReadModel.operationProofs).map(
      (proof) => (proof.operation === operation ? { ...proof, ...patch } : proof)
    ),
  });
}

function withMobilePatch(platform: 'android' | 'ios', patch: object) {
  return {
    ...RuntimeReadModel,
    mobileBridgeReadModels: RuntimeReadModel.mobileBridgeReadModels.map((readModel) =>
      readModel.platform === platform ? { ...readModel, ...patch } : readModel
    ),
  };
}
