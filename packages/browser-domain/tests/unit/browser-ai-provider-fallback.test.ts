import { describe, expect, it } from 'vitest';
import { BrowserAiProviderFallbackDecisionSchema } from '@ocentra-parent/schema-domain/browser-ai-provider-fallback-schemas';
import {
  missingLocalProviderRoute,
  providerFallbackDecision,
  selectedFamilyHubRoute,
  selectedLocalProviderRoute,
  selectedRemoteRoute,
  unavailableLocalProviderRoute,
} from './browser-ai-provider-fallback.fixtures';

describe('browser AI provider degraded fallback contract', () => {
  it('accepts a visible local provider selection without policy authority', expectLocalProviderFallback);
  it('accepts family hub selection only after the local route is degraded', expectFamilyHubFallback);
  it('accepts remote selection only with explicit parent approval and local safety fallback', expectRemoteFallback);
  it('accepts metadata-only and no-AI fallbacks without selecting a runtime', expectNoRuntimeFallbacks);
  it('rejects hidden fallback, authority claims, and unsafe remote fallback claims', expectFallbackAuthorityRejections);
  it('rejects selected provider decisions that do not match the selected route', expectRouteMismatchRejections);
  it(
    'rejects remote fallback while a local or family-hub route is already selected',
    expectRemoteSelectionOrderRejections
  );
});

function expectLocalProviderFallback() {
  const localRoute = selectedLocalProviderRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'child-device-local-ai',
      selectedRuntimeRef: localRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['local-selected'],
      localProviderRoute: localRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.selectedProviderKind).toBe('child-device-local-ai');
    expect(parsed.data.analysisResultClaimed).toBe(false);
    expect(parsed.data.policyDecisionClaimed).toBe(false);
  }
}

function expectFamilyHubFallback() {
  const familyRoute = selectedFamilyHubRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'family-ai-hub',
      selectedRuntimeRef: familyRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['model-missing', 'family-hub-selected'],
      localProviderRoute: familyRoute.sourceLocalProviderRoute,
      familyHubRoute: familyRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.localProviderRoute.executionState).toBe('manual-required');
    expect(parsed.data.familyHubRoute?.executionState).toBe('selected');
  }
}

function expectRemoteFallback() {
  const remoteRoute = selectedRemoteRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'parent-approved-remote-ai',
      selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['provider-unavailable', 'remote-selected'],
      localProviderRoute: missingLocalProviderRoute(),
      remoteRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.remoteRoute?.parentExplicitRemoteApproval).toBe(true);
    expect(parsed.data.remoteRoute?.localSafetyFallbackAvailable).toBe(true);
  }
}

function expectNoRuntimeFallbacks() {
  const metadataOnly = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'metadata-only',
      selectedRuntimeRef: null,
      fallbackAction: 'metadata-only-review',
      fallbackReasons: ['metadata-only', 'metadata-degraded'],
      localProviderRoute: missingLocalProviderRoute(),
    })
  );
  const noAi = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'no-ai',
      selectedRuntimeRef: null,
      fallbackAction: 'parent-review',
      fallbackReasons: ['provider-unavailable', 'parent-review'],
      localProviderRoute: unavailableLocalProviderRoute(),
    })
  );

  expect(metadataOnly.success).toBe(true);
  expect(noAi.success).toBe(true);
}

function expectFallbackAuthorityRejections() {
  for (const invalid of invalidFallbackAuthorityClaims()) {
    const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(invalid);

    expect(parsed.success).toBe(false);
  }
}

function expectRouteMismatchRejections() {
  const localRoute = selectedLocalProviderRoute();
  const familyRoute = selectedFamilyHubRoute();
  const inconsistentFamily = providerFallbackDecision({
    selectedProviderKind: 'family-ai-hub',
    selectedRuntimeRef: familyRoute.selectedRuntimeRef,
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['family-hub-selected'],
    localProviderRoute: localRoute,
    familyHubRoute: familyRoute,
  });
  const runtimeMismatch = providerFallbackDecision({
    selectedProviderKind: 'child-device-local-ai',
    selectedRuntimeRef: 'different-runtime-ref',
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['local-selected'],
    localProviderRoute: localRoute,
  });

  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(inconsistentFamily).success).toBe(false);
  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(runtimeMismatch).success).toBe(false);
}

function invalidFallbackAuthorityClaims() {
  const valid = providerFallbackDecision({
    selectedProviderKind: 'metadata-only',
    selectedRuntimeRef: null,
    fallbackAction: 'metadata-only-review',
    fallbackReasons: ['metadata-only'],
    localProviderRoute: missingLocalProviderRoute(),
  });
  return [
    { ...valid, parentFallbackVisible: false },
    { ...valid, childFallbackVisible: false },
    { ...valid, analysisResultClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, localSafetyPreserved: false },
    { ...valid, remoteDefaultForBlocking: true },
    { ...valid, remoteOutageDisablesLocalSafety: true },
  ];
}

function expectRemoteSelectionOrderRejections() {
  const localRoute = selectedLocalProviderRoute();
  const familyRoute = selectedFamilyHubRoute();
  const remoteRoute = selectedRemoteRoute();

  const selectedRemoteWithSelectedLocal = providerFallbackDecision({
    selectedProviderKind: 'parent-approved-remote-ai',
    selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['remote-selected'],
    localProviderRoute: localRoute,
    remoteRoute,
  });
  const selectedRemoteWithSelectedFamily = providerFallbackDecision({
    selectedProviderKind: 'parent-approved-remote-ai',
    selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['remote-selected'],
    localProviderRoute: familyRoute.sourceLocalProviderRoute,
    familyHubRoute: familyRoute,
    remoteRoute,
  });

  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(selectedRemoteWithSelectedLocal).success).toBe(false);
  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(selectedRemoteWithSelectedFamily).success).toBe(false);
}
