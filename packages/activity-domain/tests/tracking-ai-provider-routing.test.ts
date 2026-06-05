import { describe, expect, it } from 'vitest';
import {
  TrackingAiProviderCapabilitySchema,
  TrackingAiProviderRouteSchema,
  TrackingAiProviderRouteSchemaVersion,
  planTrackingAiProviderRoute,
} from '../src/tracking-ai-provider-routing';

describe('tracking AI provider routing contract', () => {
  it('selects child-device local AI as the default child-safety route', expectChildDeviceLocalDefault);
  it('selects the family hub only when it stays in local LAN custody', expectFamilyHubLanOnly);
  it('rejects a family hub capability that is not LAN-only or no-retention', expectUnsafeFamilyHubRejected);
  it('requires explicit parent approval before selecting a remote provider', expectRemoteRequiresParentApproval);
  it('preserves provider unavailable and unsupported task degraded states', expectUnavailableAndUnsupportedDegraded);
  it('keeps metadata-only and no-AI routes visible without selecting a runtime', expectMetadataOnlyAndNoAi);
  it('rejects routes that hide provider visibility or claim AI policy authority', expectUnsafeRouteRejected);
});

function expectChildDeviceLocalDefault() {
  const route = planTrackingAiProviderRoute(routeRequest());

  expect(route.executionState).toBe('selected');
  expect(route.providerKind).toBe('child-device-local-ai');
  expect(route.selectedRuntimeRef).toBe('tracking-local-model-runtime-child');
  expect(route.custodyLabel).toBe('child-device-local');
  expect(route.parentExplicitRemoteApproval).toBe(false);
  expect(route.remoteDefaultForBlocking).toBe(false);
  expect(route.aiCanTriggerAlertDirectly).toBe(false);
  expect(route.aiIsFinalAuthority).toBe(false);
}

function expectFamilyHubLanOnly() {
  const route = planTrackingAiProviderRoute({
    ...routeRequest(),
    routeId: 'tracking-ai-route-family-hub',
    modelRuntimePreference: 'local-preferred',
    capability: familyHubCapability(),
  });

  expect(route.executionState).toBe('selected');
  expect(route.providerKind).toBe('family-ai-hub');
  expect(route.custodyLabel).toBe('live-lan-child-agent');
  expect(route.selectedRuntimeRef).toBe('tracking-family-hub-runtime-lan');
  expect(route.remoteDefaultForBlocking).toBe(false);
}

function expectUnsafeFamilyHubRejected() {
  const parsed = TrackingAiProviderCapabilitySchema.safeParse({
    ...familyHubCapability(),
    familyHubLanOnly: false,
    noRetention: false,
  });

  expect(parsed.success).toBe(false);
}

function expectRemoteRequiresParentApproval() {
  const withoutApproval = planTrackingAiProviderRoute(remoteRouteRequest(false));
  const withApproval = planTrackingAiProviderRoute(remoteRouteRequest(true));

  expect(withoutApproval.executionState).toBe('manual-required');
  expect(withoutApproval.degradedStates).toEqual(['parent-approval-missing']);
  expect(withoutApproval.selectedRuntimeRef).toBeNull();
  expect(withApproval.executionState).toBe('selected');
  expect(withApproval.parentExplicitRemoteApproval).toBe(true);
  expect(withApproval.custodyLabel).toBe('parent-approved-cloud');
}

function expectUnavailableAndUnsupportedDegraded() {
  const unavailable = planTrackingAiProviderRoute({
    ...routeRequest(),
    routeId: 'tracking-ai-route-provider-unavailable',
    capability: unavailableChildCapability(),
  });
  const unsupportedTask = planTrackingAiProviderRoute({
    ...routeRequest(),
    routeId: 'tracking-ai-route-unsupported-task',
    requestedTask: 'parent-summary',
  });

  expect(unavailable.executionState).toBe('unavailable');
  expect(unavailable.degradedStates).toEqual(['provider-unavailable']);
  expect(unavailable.selectedRuntimeRef).toBeNull();
  expect(unsupportedTask.executionState).toBe('manual-required');
  expect(unsupportedTask.degradedStates).toEqual(['unsupported-task']);
  expect(unsupportedTask.selectedRuntimeRef).toBeNull();
}

function expectMetadataOnlyAndNoAi() {
  const metadataOnly = planTrackingAiProviderRoute(metadataOnlyRouteRequest());
  const noAi = planTrackingAiProviderRoute(noAiRouteRequest());

  expect(metadataOnly.executionState).toBe('metadata-only');
  expect(metadataOnly.metadataOnly).toBe(true);
  expect(metadataOnly.selectedRuntimeRef).toBeNull();
  expect(noAi.executionState).toBe('no-ai');
  expect(noAi.noAi).toBe(true);
  expect(noAi.selectedRuntimeRef).toBeNull();
}

function expectUnsafeRouteRejected() {
  const route = planTrackingAiProviderRoute(routeRequest());
  const parsed = TrackingAiProviderRouteSchema.safeParse({
    ...route,
    providerVisible: false,
    aiCanTriggerAlertDirectly: true,
    aiIsFinalAuthority: true,
    assistantCanWritePolicy: true,
    remoteCanOverrideStricterLocalRules: true,
  });

  expect(parsed.success).toBe(false);
}

function routeRequest() {
  return {
    routeId: 'tracking-ai-route-child-local-home-arrival',
    routedAt: '2026-06-05T06:20:00.000Z',
    requestedTask: 'location-safety',
    modelRuntimePreference: 'child-local-required',
    deviceId: 'child-device-tracking-1',
    childProfileRef: 'child-profile-tracking-1',
    policyVersionRef: 'tracking-policy-v1',
    evidenceIds: ['location-evidence-1', 'device-status-1'],
    parentRuleRefs: ['parent-rule-home-arrival'],
    capability: childLocalCapability(),
    parentExplicitRemoteApproval: false,
    reasonCodes: ['tracking-ai-route-proof'],
  } as const;
}

function remoteRouteRequest(parentExplicitRemoteApproval: boolean) {
  return {
    ...routeRequest(),
    routeId: parentExplicitRemoteApproval
      ? 'tracking-ai-route-remote-approved'
      : 'tracking-ai-route-remote-missing-approval',
    modelRuntimePreference: 'parent-approved-remote-allowed',
    capability: remoteCapability(),
    parentExplicitRemoteApproval,
  } as const;
}

function metadataOnlyRouteRequest() {
  return {
    ...routeRequest(),
    routeId: 'tracking-ai-route-metadata-only',
    modelRuntimePreference: 'metadata-only',
    capability: metadataOnlyCapability(),
  } as const;
}

function noAiRouteRequest() {
  return {
    ...routeRequest(),
    routeId: 'tracking-ai-route-no-ai',
    modelRuntimePreference: 'no-ai',
    capability: noAiCapability(),
  } as const;
}

function childLocalCapability() {
  return {
    schemaVersion: TrackingAiProviderRouteSchemaVersion,
    providerId: 'tracking-child-local-ai',
    checkedAt: '2026-06-05T06:19:00.000Z',
    providerKind: 'child-device-local-ai',
    capabilityState: 'available',
    supportedTasks: ['location-safety', 'expected-place-safety', 'nearby-place-context', 'geofence-risk'],
    modelRuntimeRef: 'tracking-local-model-runtime-child',
    custodyLabel: 'child-device-local',
    noRetention: true,
    localOnly: true,
    parentApprovedRemoteEnabled: false,
    canRunOnChildDevice: true,
    canRunOnParentDevice: false,
    familyHubLanOnly: false,
    degradedStates: [],
    unavailableReason: null,
  } as const;
}

function unavailableChildCapability() {
  return {
    ...childLocalCapability(),
    capabilityState: 'provider-unavailable',
    modelRuntimeRef: null,
    degradedStates: ['provider-unavailable'],
    unavailableReason: 'tracking-local-provider-unavailable',
  } as const;
}

function familyHubCapability() {
  return {
    ...childLocalCapability(),
    providerId: 'tracking-family-ai-hub',
    providerKind: 'family-ai-hub',
    modelRuntimeRef: 'tracking-family-hub-runtime-lan',
    custodyLabel: 'live-lan-child-agent',
    canRunOnChildDevice: false,
    canRunOnParentDevice: false,
    familyHubLanOnly: true,
  } as const;
}

function remoteCapability() {
  return {
    ...childLocalCapability(),
    providerId: 'tracking-parent-approved-remote',
    providerKind: 'parent-approved-remote-ai',
    modelRuntimeRef: 'tracking-parent-approved-remote-runtime',
    custodyLabel: 'parent-approved-cloud',
    localOnly: false,
    parentApprovedRemoteEnabled: true,
    canRunOnChildDevice: false,
    canRunOnParentDevice: false,
  } as const;
}

function metadataOnlyCapability() {
  return {
    ...childLocalCapability(),
    providerId: 'tracking-metadata-only',
    providerKind: 'metadata-only',
    capabilityState: 'manual-required',
    supportedTasks: ['location-safety'],
    modelRuntimeRef: null,
    canRunOnChildDevice: false,
    degradedStates: ['metadata-only'],
    unavailableReason: 'tracking-metadata-only-route',
  } as const;
}

function noAiCapability() {
  return {
    ...metadataOnlyCapability(),
    providerId: 'tracking-no-ai',
    providerKind: 'no-ai',
    degradedStates: ['no-ai'],
    unavailableReason: 'tracking-no-ai-route',
  } as const;
}
