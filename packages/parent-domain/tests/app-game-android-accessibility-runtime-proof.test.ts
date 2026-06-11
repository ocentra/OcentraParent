import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidAccessibilityRuntimeProofSchema,
  createAppGameAndroidAccessibilityRuntimeProof,
  summarizeAppGameAndroidAccessibilityRuntimeProof,
} from '../src/app-game-android-accessibility-runtime-proof';

describe('app-game Android Accessibility runtime proof', () => {
  recordsDeclaredServiceWithoutPromotingOverlayExecution();
  allowsBoundServiceOnlyAsRuntimeSampleEvidence();
  rejectsRawRowsAndDispatchClaims();
});

function recordsDeclaredServiceWithoutPromotingOverlayExecution() {
  it('records the package-declared service while keeping overlay runtime blocked before enablement', () => {
    const proof = createAppGameAndroidAccessibilityRuntimeProof({
      declarationState: 'accessibility-service-declared',
      runtimeState: 'accessibility-runtime-waiting-for-enablement',
      eventSampleState: 'accessibility-event-sample-waiting-for-enablement',
      manifestServiceDeclared: true,
      serviceConfigDeclared: true,
      uiRuntimeStateObserved: true,
      settingsStateObserved: true,
      checkedAt: '2026-06-08T21:10:00.000Z',
    });
    const summary = summarizeAppGameAndroidAccessibilityRuntimeProof(proof);

    expect(summary.declarationState).toBe('accessibility-service-declared');
    expect(summary.runtimeState).toBe('accessibility-runtime-waiting-for-enablement');
    expect(summary.eventSampleState).toBe('accessibility-event-sample-waiting-for-enablement');
    expect(summary.manifestServiceDeclared).toBe(true);
    expect(summary.serviceConfigDeclared).toBe(true);
    expect(proof.canDispatchOverlayAdapter).toBe(false);
    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'android-accessibility-service-enable-proof-not-attached',
        'android-accessibility-event-sample-not-observed',
        'android-accessibility-overlay-runtime-not-proved',
        'android-child-device-delivery-not-proved',
        'android-platform-enforcement-not-proved',
      ])
    );
  });
}

function allowsBoundServiceOnlyAsRuntimeSampleEvidence() {
  it('records bound service sample state without claiming overlay execution or child delivery', () => {
    const proof = createAppGameAndroidAccessibilityRuntimeProof({
      declarationState: 'accessibility-service-declared',
      runtimeState: 'accessibility-runtime-bound',
      eventSampleState: 'accessibility-event-sample-observed',
      manifestServiceDeclared: true,
      serviceConfigDeclared: true,
      uiRuntimeStateObserved: true,
      settingsStateObserved: true,
      checkedAt: '2026-06-08T21:10:00.000Z',
    });

    expect(proof.openGaps).not.toContain('android-accessibility-service-enable-proof-not-attached');
    expect(proof.openGaps).not.toContain('android-accessibility-event-sample-not-observed');
    expect(proof.openGaps).toContain('android-accessibility-overlay-runtime-not-proved');
    expect(proof.overlayRuntimeClaimed).toBe(false);
    expect(proof.childDeviceDeliveryClaimed).toBe(false);
    expect(proof.platformEnforcementClaimed).toBe(false);
  });
}

function rejectsRawRowsAndDispatchClaims() {
  it('rejects raw Accessibility event/service data, overlay runtime, adapter dispatch, and enforcement claims', () => {
    const proof = createAppGameAndroidAccessibilityRuntimeProof({
      declarationState: 'accessibility-service-declared',
      runtimeState: 'accessibility-runtime-waiting-for-enablement',
      eventSampleState: 'accessibility-event-sample-waiting-for-enablement',
      manifestServiceDeclared: true,
      serviceConfigDeclared: true,
      uiRuntimeStateObserved: true,
      settingsStateObserved: true,
      checkedAt: '2026-06-08T21:10:00.000Z',
    });

    expect(
      AppGameAndroidAccessibilityRuntimeProofSchema.safeParse({
        ...proof,
        rawAccessibilityEventRowsStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAccessibilityRuntimeProofSchema.safeParse({
        ...proof,
        canDispatchOverlayAdapter: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAccessibilityRuntimeProofSchema.safeParse({
        ...proof,
        overlayRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
}
