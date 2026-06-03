import { describe, expect, it } from 'vitest';
import {
  BrowserGameRuntimeSignalDetectionSchema,
  BrowserGameRuntimeSignalRowSchema,
  type BrowserGameRuntimeSignalDetection,
  type BrowserGameRuntimeSignalRow,
} from '../src/browser-game-runtime-signal-detector';

describe('browser-game runtime signal detector contracts', () => {
  it('accepts shape-only runtime signal rows for canvas, gamepad, fullscreen, and cloud streaming', acceptsSignals);
  it('accepts candidate, manual-required, and unavailable signal states', acceptsFallbackRows);
  it('accepts detection bundles without claiming instrumentation', acceptsDetections);
  it(
    'rejects raw runtime data, instrumentation, AI, policy, cloud-frame, native, and enforcement claims',
    rejectsClaims
  );
  it('rejects inconsistent signal purposes and dishonest detection upgrades', rejectsInconsistentRows);
});

function acceptsSignals() {
  expect(BrowserGameRuntimeSignalRowSchema.safeParse(runtimeSignal()).success).toBe(true);
  expect(
    BrowserGameRuntimeSignalRowSchema.safeParse(
      runtimeSignal({
        signalId: 'runtime-signal-gamepad',
        signalKind: 'gamepad-api-shape',
        signalFingerprint: 'runtime-signal-fingerprint-gamepad',
        reasonCodes: ['runtime-shape-present', 'gamepad-shape-present', 'managed-browser-proof-required'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameRuntimeSignalRowSchema.safeParse(
      runtimeSignal({
        signalId: 'runtime-signal-fullscreen',
        signalKind: 'fullscreen-request-shape',
        signalFingerprint: 'runtime-signal-fingerprint-fullscreen',
        reasonCodes: ['runtime-shape-present', 'fullscreen-shape-present', 'managed-browser-proof-required'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameRuntimeSignalRowSchema.safeParse(
      runtimeSignal({
        signalId: 'runtime-signal-cloud',
        signalKind: 'cloud-streaming-shape',
        signalFingerprint: 'runtime-signal-fingerprint-cloud',
        reasonCodes: ['runtime-shape-present', 'cloud-streaming-shape-present', 'managed-browser-proof-required'],
        cloudSessionCandidate: true,
      })
    ).success
  ).toBe(true);
}

function acceptsFallbackRows() {
  expect(
    BrowserGameRuntimeSignalRowSchema.safeParse(
      runtimeSignal({
        signalId: 'runtime-signal-candidate',
        status: 'candidate-shape',
        confidence: 'medium',
      })
    ).success
  ).toBe(true);
  expect(BrowserGameRuntimeSignalRowSchema.safeParse(manualRuntimeSignal()).success).toBe(true);
  expect(
    BrowserGameRuntimeSignalRowSchema.safeParse(
      manualRuntimeSignal({
        signalId: 'runtime-signal-unavailable',
        status: 'unavailable',
        sourceKind: 'unavailable',
        reasonCodes: ['unavailable'],
      })
    ).success
  ).toBe(true);
}

function acceptsDetections() {
  expect(BrowserGameRuntimeSignalDetectionSchema.safeParse(runtimeDetection()).success).toBe(true);
  expect(
    BrowserGameRuntimeSignalDetectionSchema.safeParse(
      runtimeDetection({
        status: 'manual-required',
        confidence: 'low',
        signals: [manualRuntimeSignal()],
      })
    ).success
  ).toBe(true);
}

function rejectsClaims() {
  const invalidClaims = [
    { rawDomStored: true },
    { rawCanvasFrameStored: true },
    { rawStreamFrameStored: true },
    { rawAudioStored: true },
    { rawGamepadInputStored: true },
    { browserInstrumentationClaimed: true },
    { runtimeDetectionExecutedClaimed: true },
    { aiClassificationClaimed: true },
    { policyDecisionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidClaims) {
    expect(BrowserGameRuntimeSignalRowSchema.safeParse(runtimeSignal(invalid)).success).toBe(false);
    expect(BrowserGameRuntimeSignalDetectionSchema.safeParse(runtimeDetection(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentRows() {
  const invalidSignals = [
    { signalKind: 'unknown' },
    { sourceKind: 'unavailable' },
    { managedBrowserProofRequired: false },
    { reasonCodes: ['runtime-shape-present'] },
    { cloudSessionCandidate: true, signalKind: 'canvas-present-shape' },
    { childLaunchCandidate: true, signalKind: 'unknown' },
    { status: 'manual-required', confidence: 'high', signalKind: 'unknown' },
  ];

  for (const invalid of invalidSignals) {
    expect(BrowserGameRuntimeSignalRowSchema.safeParse(runtimeSignal(invalid)).success).toBe(false);
  }

  expect(
    BrowserGameRuntimeSignalDetectionSchema.safeParse(
      runtimeDetection({
        signals: [manualRuntimeSignal()],
      })
    ).success
  ).toBe(false);
  expect(
    BrowserGameRuntimeSignalDetectionSchema.safeParse(
      runtimeDetection({
        status: 'manual-required',
        confidence: 'high',
        signals: [manualRuntimeSignal()],
      })
    ).success
  ).toBe(false);
}

function runtimeSignal(overrides = {}): BrowserGameRuntimeSignalRow {
  return {
    signalId: 'runtime-signal-canvas',
    signalKind: 'canvas-present-shape',
    signalFingerprint: 'runtime-signal-fingerprint-canvas',
    sourceKind: 'managed-browser-signal-ref',
    sourceEvidenceRefs: ['runtime-signal-evidence-canvas'],
    confidence: 'high',
    status: 'detected-shape',
    reasonCodes: ['runtime-shape-present', 'canvas-shape-present', 'managed-browser-proof-required'],
    managedBrowserProofRequired: true,
    childLaunchCandidate: true,
    cloudSessionCandidate: false,
    rawDomStored: false,
    rawCanvasFrameStored: false,
    rawStreamFrameStored: false,
    rawAudioStored: false,
    rawGamepadInputStored: false,
    browserInstrumentationClaimed: false,
    runtimeDetectionExecutedClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualRuntimeSignal(overrides = {}): BrowserGameRuntimeSignalRow {
  return runtimeSignal({
    signalId: 'runtime-signal-manual',
    signalKind: 'unknown',
    signalFingerprint: 'runtime-signal-fingerprint-manual-required',
    sourceKind: 'manual-review-ref',
    confidence: 'low',
    status: 'manual-required',
    reasonCodes: ['manual-required'],
    managedBrowserProofRequired: true,
    childLaunchCandidate: false,
    ...overrides,
  });
}

function runtimeDetection(overrides = {}): BrowserGameRuntimeSignalDetection {
  return {
    schemaVersion: 'browser-game-runtime-signal-detector-contract',
    detectionId: 'runtime-signal-detection-reviewed',
    detectedAt: '2026-06-03T11:56:00.000Z',
    sourceEvidenceRefs: ['runtime-signal-detection-evidence'],
    signals: [runtimeSignal()],
    confidence: 'high',
    status: 'detected-shape',
    rawDomStored: false,
    rawCanvasFrameStored: false,
    rawStreamFrameStored: false,
    rawAudioStored: false,
    rawGamepadInputStored: false,
    browserInstrumentationClaimed: false,
    runtimeDetectionExecutedClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
