import { describe, expect, it } from 'vitest';
import { ParentMobileRuntimeReadModelSchema } from '../src/parent-mobile-runtime';
import {
  AndroidObserverReadModel,
  AndroidParentMobileCapabilities,
  IosObserverReadModel,
  SubmittedLanProviderReadModel,
} from './parent-mobile-runtime-fixtures';

describe('parent mobile runtime read model contracts', () => {
  registerAcceptedStateTests();
  registerModelClaimGuardrailTests();
  registerAssistantJobGuardrailTests();
  registerControllerGuardrailTests();
  registerCapabilityGuardrailTests();
});

function registerAcceptedStateTests(): void {
  it('ParentMobileRuntimeReadModelSchema: accepts Android observer scaffold with LAN assistant degraded state', () => {
    const parsed = ParentMobileRuntimeReadModelSchema.parse(AndroidObserverReadModel);

    expect(parsed.platform).toBe('android');
    expect(parsed.controllerProof.controllerState).toBe('observer');
    expect(parsed.localModelExecutionAllowed).toBe(false);
    expect(parsed.childAgentBehaviorClaim).toBe('not-claimed');
    expect(parsed.platformCapabilities.map((entry) => entry.capability)).toEqual([
      'parent-mobile-observer',
      'parent-mobile-controller',
      'foreground-mobile-service',
      'notifications',
      'package-lifecycle',
      'store-distribution',
    ]);
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
    expect(parsed.platformCapabilities.find((entry) => entry.capability === 'foreground-mobile-service')?.status).toBe(
      'unavailable'
    );
  });
}

function registerModelClaimGuardrailTests(): void {
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
}

function registerAssistantJobGuardrailTests(): void {
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
}

function registerControllerGuardrailTests(): void {
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
}

function registerCapabilityGuardrailTests(): void {
  it('ParentMobileRuntimeReadModelSchema: rejects missing Android notification capability proof', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        platformCapabilities: AndroidParentMobileCapabilities.filter((entry) => entry.capability !== 'notifications'),
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects completed controller support from mobile scaffold proof', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...AndroidObserverReadModel,
        platformCapabilities: AndroidParentMobileCapabilities.map((entry) =>
          entry.capability === 'parent-mobile-controller' ? { ...entry, status: 'implemented' } : entry
        ),
      }).success
    ).toBe(false);
  });

  it('ParentMobileRuntimeReadModelSchema: rejects Android capability proof reused for iOS', () => {
    expect(
      ParentMobileRuntimeReadModelSchema.safeParse({
        ...IosObserverReadModel,
        platformCapabilities: AndroidParentMobileCapabilities,
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
