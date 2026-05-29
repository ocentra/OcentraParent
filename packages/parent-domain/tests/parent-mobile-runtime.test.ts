import { describe, expect, it } from 'vitest';
import { ParentMobileRuntimeReadModelSchema } from '../src/parent-mobile-runtime';

const CheckedAt = '2026-05-28T16:05:00.000Z';

const AndroidObserverReadModel = {
  schemaVersion: 'v0.9-parent-mobile-shell',
  parentDeviceId: 'parent-mobile-android-observer',
  platform: 'android',
  packageProof: {
    platform: 'android',
    packageState: 'ci-mechanical-proof',
    launchTarget: 'ca.ocentra.parent.agent/.MainActivity',
    proofCommand: 'cmd /c npm run release:package:android',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
  },
  serviceAvailability: {
    localService: 'manual-required',
    lanService: 'degraded',
    cloudRelay: 'not-implemented',
    selectedRouteId: 'route-parent-mobile-lan-provider',
  },
  controllerProof: {
    controllerState: 'observer',
    controllerLeaseId: null,
    takeoverRequestAllowed: false,
    commandAuthorityState: 'observer-read-only',
  },
  assistantJobProof: {
    route: 'lan-ai-provider',
    jobState: 'degraded',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
    unavailableReason: 'lan-ai-provider-unavailable',
  },
  localModelExecutionState: 'disabled-by-default',
  localModelExecutionAllowed: false,
  childAgentBehaviorClaim: 'not-claimed',
  updatedAt: CheckedAt,
} as const;

const IosObserverReadModel = {
  ...AndroidObserverReadModel,
  parentDeviceId: 'parent-mobile-ios-observer',
  platform: 'ios',
  packageProof: {
    platform: 'ios',
    packageState: 'ci-mechanical-proof',
    launchTarget: 'ca.ocentra.parent.agent',
    proofCommand: 'bash scripts/release/ios/build-simulator-app.sh',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
  },
  serviceAvailability: {
    localService: 'manual-required',
    lanService: 'manual-required',
    cloudRelay: 'not-implemented',
    selectedRouteId: null,
  },
  controllerProof: {
    controllerState: 'manual-required',
    controllerLeaseId: null,
    takeoverRequestAllowed: true,
    commandAuthorityState: 'controller-takeover-manual-required',
  },
  assistantJobProof: {
    route: 'unavailable',
    jobState: 'unavailable',
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: [],
    unavailableReason: 'mobile-package-proof-required',
  },
} as const;

const SubmittedLanProviderReadModel = {
  ...AndroidObserverReadModel,
  serviceAvailability: {
    ...AndroidObserverReadModel.serviceAvailability,
    lanService: 'available',
  },
  assistantJobProof: {
    ...AndroidObserverReadModel.assistantJobProof,
    jobState: 'submitted',
    providerId: 'lan-ai-provider-family-pc',
    unavailableReason: null,
  },
} as const;

describe('parent mobile runtime read model contracts', () => {
  registerAcceptedStateTests();
  registerGuardrailTests();
});

function registerAcceptedStateTests(): void {
  it('ParentMobileRuntimeReadModelSchema: accepts Android observer scaffold with LAN assistant degraded state', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(AndroidObserverReadModel);

    expect(parsed.platform).toBe('android');
    expect(parsed.controllerProof.controllerState).toBe('observer');
    expect(parsed.localModelExecutionAllowed).toBe(false);
    expect(parsed.childAgentBehaviorClaim).toBe('not-claimed');
  });

  it('ParentMobileRuntimeReadModelSchema: accepts submitted LAN provider job with provider identity', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(SubmittedLanProviderReadModel);

    expect(parsed.assistantJobProof.jobState).toBe('submitted');
    expect(parsed.assistantJobProof.providerId).toBe('lan-ai-provider-family-pc');
    expect(parsed.assistantJobProof.unavailableReason).toBeNull();
  });

  it('ParentMobileRuntimeReadModelSchema: accepts iOS manual-required package state without cloud relay claim', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(IosObserverReadModel);

    expect(parsed.platform).toBe('ios');
    expect(parsed.serviceAvailability.cloudRelay).toBe('not-implemented');
    expect(parsed.assistantJobProof.jobState).toBe('unavailable');
  });
}

function registerGuardrailTests(): void {
  it('ParentMobileRuntimeReadModelSchema: rejects parent mobile local model execution claims', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        localModelExecutionAllowed: true,
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects child-agent behavior claims from parent mobile shell proof', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        childAgentBehaviorClaim: 'foreground-child-agent',
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects degraded LAN provider job without unavailable reason', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        assistantJobProof: {
          ...AndroidObserverReadModel.assistantJobProof,
          jobState: 'degraded',
          providerId: null,
          unavailableReason: null,
        },
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects unavailable LAN provider job with provider identity', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        assistantJobProof: {
          ...AndroidObserverReadModel.assistantJobProof,
          jobState: 'unavailable',
          providerId: 'lan-ai-provider-family-pc',
          unavailableReason: 'lan-ai-provider-unavailable',
        },
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects observer state with write authority or a controller lease', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        controllerProof: {
          controllerState: 'observer',
          controllerLeaseId: 'controller-lease-from-mobile',
          takeoverRequestAllowed: false,
          commandAuthorityState: 'active-controller-backend-proof',
        },
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: accepts active controller backend proof only with an explicit lease', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse({
      ...AndroidObserverReadModel,
      controllerProof: {
        controllerState: 'active-controller',
        controllerLeaseId: 'controller-lease-parent-mobile-proof',
        takeoverRequestAllowed: true,
        commandAuthorityState: 'active-controller-backend-proof',
      },
    });

    expect(parsed.controllerProof.controllerLeaseId).toBe('controller-lease-parent-mobile-proof');
    expect(parsed.controllerProof.commandAuthorityState).toBe('active-controller-backend-proof');
    expect(parsed.childAgentBehaviorClaim).toBe('not-claimed');
  });
}
