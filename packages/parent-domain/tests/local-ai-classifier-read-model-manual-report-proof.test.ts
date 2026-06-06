import { describe, expect, it } from 'vitest';
import {
  buildLocalAiClassifierReportSnapshot,
  LocalAiClassifierReportRowSchema,
  LocalAiClassifierReportSnapshotSchema,
} from '../src/local-ai-classifier-read-model-manual-report-proof';
import { runLocalAiDeterministicClassifier } from '../src/local-ai-deterministic-classifier-proof';

describe('local AI classifier read-model manual report proof', () => {
  it(
    'projects deterministic classifier rows into ready, manual-required, and unavailable report states',
    provesReportStates
  );
  it('preserves typed evidence, rule, runtime, prompt, and trace refs without authority claims', provesRefs);
  it('rejects raw retention, model execution, policy authority, enforcement, and mismatched counts', rejectsOverclaims);
});

const GeneratedAt = '2026-06-06T11:05:00.000Z';
const SourceProofRefs = ['output/ai-plan-proof/local-ai-deterministic-classifier-proof/proof-summary.json'];
const ObservedAt = '2026-06-06T07:30:00.000Z';
const Runtime = {
  runtimeReferenceId: 'local-ai-runtime-deterministic-classifier',
  providerId: 'local-provider-deterministic-classifier',
  modelId: 'deterministic-classifier-v1',
  modelReference: 'artifact:deterministic_classifier_v1',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: ObservedAt,
  unavailableReason: null,
};

function provesReportStates() {
  const snapshot = reportSnapshot();

  expect(snapshot.readyRowCount).toBe(3);
  expect(snapshot.manualRequiredRowCount).toBe(2);
  expect(snapshot.unavailableRowCount).toBe(1);
  expect(snapshot.rows.map((row) => row.reportState)).toEqual([
    'ready',
    'ready',
    'ready',
    'manual-required',
    'manual-required',
    'unavailable',
  ]);
  expect(snapshot.rows[3].manualRequiredReasons).toEqual([
    'manual:low-confidence',
    'manual:action:ask-parent',
    'manual:degraded:invalid-output',
  ]);
  expect(snapshot.rows[4].manualRequiredReasons).toEqual([
    'manual:missing-evidence',
    'manual:action:unknown',
    'manual:degraded:invalid-output',
    'manual:unknown:missing-evidence',
  ]);
  expect(snapshot.rows[5].manualRequiredReasons).toEqual([
    'manual:runtime-unavailable',
    'manual:action:ask-parent',
    'manual:degraded:provider-unavailable',
    'manual:unknown:model-unavailable',
  ]);
}

function provesRefs() {
  const snapshot = reportSnapshot();
  const readyVideo = snapshot.rows[0];
  const blockedProcess = snapshot.rows[2];

  expect(readyVideo.action).toBe('warn');
  expect(blockedProcess.action).toBe('block');
  expect(readyVideo.evidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual(['evidence:video']);
  expect(readyVideo.parentRuleReferences).toEqual(['policy-rule:screen-video-warn']);
  expect(readyVideo.runtimeReferenceId).toBe(Runtime.runtimeReferenceId);
  expect(readyVideo.providerId).toBe(Runtime.providerId);
  expect(readyVideo.modelId).toBe(Runtime.modelId);
  expect(readyVideo.promptVersion).toBe('prompt:deterministic-classifier:v1');
  expect(readyVideo.classifierTraceRefs).toEqual([
    'local-ai-deterministic-classifier:local-ai-eval:deterministic-classifier:video',
  ]);
  expect(snapshot.rows.every((row) => row.reportOnly)).toBe(true);
  expect(snapshot.rows.every((row) => row.dryRun)).toBe(true);
  expect(snapshot.rows.some((row) => row.modelExecuted)).toBe(false);
  expect(snapshot.rows.some((row) => row.rawEvidenceRetained)).toBe(false);
  expect(snapshot.rows.some((row) => row.rawModelOutputRetained)).toBe(false);
  expect(snapshot.rows.some((row) => row.remoteApiClaimed)).toBe(false);
  expect(snapshot.rows.some((row) => row.policyAuthorityClaimed)).toBe(false);
  expect(snapshot.rows.some((row) => row.enforcementClaimed)).toBe(false);
}

function rejectsOverclaims() {
  const snapshot = reportSnapshot();
  const readyRow = snapshot.rows[0];
  const manualRow = snapshot.rows[3];

  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, reportOnly: false }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, dryRun: false }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, modelExecuted: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, rawEvidenceRetained: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, rawModelOutputRetained: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, remoteApiClaimed: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, policyAuthorityClaimed: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...readyRow, enforcementClaimed: true }).success).toBe(false);
  expect(LocalAiClassifierReportRowSchema.safeParse({ ...manualRow, manualRequiredReasons: [] }).success).toBe(false);
  expect(LocalAiClassifierReportSnapshotSchema.safeParse({ ...snapshot, manualRequiredRowCount: 99 }).success).toBe(
    false
  );
}

function reportSnapshot() {
  return buildLocalAiClassifierReportSnapshot({
    generatedAt: GeneratedAt,
    snapshotId: 'local-ai-classifier-report:deterministic-lane',
    sourceProofRefs: SourceProofRefs,
    classifierResults: [
      classifierResult('video'),
      classifierResult('app'),
      classifierResult('process'),
      classifierResult('network'),
      missingEvidenceResult(),
      runtimeUnavailableResult(),
    ],
  });
}

function classifierResult(contextKind: string) {
  return runLocalAiDeterministicClassifier(classifierInput(contextKind));
}

function missingEvidenceResult() {
  const input = classifierInput('page');
  return runLocalAiDeterministicClassifier({
    ...input,
    evaluationInput: {
      ...input.evaluationInput,
      evidenceReferences: [],
    },
  });
}

function runtimeUnavailableResult() {
  return runLocalAiDeterministicClassifier({
    ...classifierInput('video'),
    modelRuntime: {
      ...Runtime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-deterministic-classifier-unavailable',
    },
  });
}

function classifierInput(contextKind: string) {
  const evidence = evidenceReference(contextKind);
  return {
    schemaVersion: 'v0.6',
    classifierRunId: `local-ai-deterministic-classifier:${contextKind}`,
    rawEvidenceRetained: false,
    modelRuntime: Runtime,
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: `local-ai-eval:deterministic-classifier:${contextKind}`,
      childProfile: {
        childProfileId: 'child:maya',
        displayName: 'Maya',
      },
      device: {
        deviceId: 'device:maya-windows',
        childProfileId: 'child:maya',
        label: 'Maya Windows laptop',
        platform: 'windows',
      },
      currentObservation: {
        contextKind,
        evidence,
      },
      evidenceReferences: [evidence],
      parentRuleReferences: ['policy-rule:screen-video-warn'],
      recentActivityWindow: [evidence],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: Runtime.providerId,
        modelId: Runtime.modelId,
        promptVersion: 'prompt:deterministic-classifier:v1',
      },
    },
  };
}

function evidenceReference(contextKind: string) {
  return {
    evidenceReferenceId: `evidence:${contextKind}`,
    kind: 'query-store-summary',
    observedAt: ObservedAt,
  };
}
