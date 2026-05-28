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

describe('parent mobile runtime read model contracts', () => {
  it('ParentMobileRuntimeReadModelSchema: accepts Android observer scaffold with LAN assistant degraded state', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(AndroidObserverReadModel);

    expect(parsed.platform).toBe('android');
    expect(parsed.controllerProof.controllerState).toBe('observer');
    expect(parsed.localModelExecutionAllowed).toBe(false);
    expect(parsed.childAgentBehaviorClaim).toBe('not-claimed');
  });

  it('ParentMobileRuntimeReadModelSchema: accepts iOS manual-required package state without cloud relay claim', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(IosObserverReadModel);

    expect(parsed.platform).toBe('ios');
    expect(parsed.serviceAvailability.cloudRelay).toBe('not-implemented');
    expect(parsed.assistantJobProof.jobState).toBe('unavailable');
  });

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
});
