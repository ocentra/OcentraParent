import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../src/local-ai-context-builder';
import { LocalAiDegradedState } from '../src/local-ai-primitives';
import { proveLocalAiTextLlmAdapterBoundary } from '../src/local-ai-text-llm-adapter-boundary-proof';
import { parseLocalAiTextOutput } from '../src/local-ai-text-output-parser-proof';
import { buildLocalAiTextParserReadModelSnapshot } from '../src/local-ai-text-parser-read-model-proof';
import { buildLocalAiTextParserPolicyHandoffProof } from '../src/local-ai-text-parser-policy-handoff-proof';

describe('local AI stored-evidence parser policy integration', () => {
  it('feeds stored screen evidence through parser, read-model, and dry-run policy handoff', provesIntegrationPath);
});

const observedAt = '2026-06-06T15:20:00.000Z';
const childProfile = { childProfileId: 'child:maya', displayName: 'Maya' };
const device = {
  deviceId: 'device:maya-windows',
  childProfileId: 'child:maya',
  label: 'Maya Windows laptop',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
  kind: 'query-store-summary',
  observedAt,
};
const readyRuntime = {
  runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
  providerId: 'local-provider-llama-cli',
  modelId: 'gemma-4-e2b-it-q4-k-m',
  modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: observedAt,
  unavailableReason: null,
};

function storedContextInput() {
  return {
    contextId: 'context:stored-screen-wiki-ocr',
    request: {
      schemaVersion: 'v0.6',
      requestId: 'local-ai-eval:stored-screen-wiki-ocr',
      requestedAt: observedAt,
      childProfile,
      device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [
        {
          parentRuleRefId: 'parent-rule-context:screen-video-warn',
          policyVersion: 'policy-v1',
          family: { familyId: 'family:maya' },
          childProfile,
          device,
          rule: {
            ruleId: 'policy-rule:screen-video-warn',
            target: {
              targetId: 'target:screen-video',
              targetType: 'category',
              targetValue: 'screen-video',
            },
            action: 'warn',
            scheduleId: null,
            priority: 20,
            reasonCode: 'parent-rule:screen-video-warn',
            createdBy: { actorId: 'parent:alex', role: 'parent' },
            enabled: true,
            effectiveFrom: null,
            effectiveUntil: null,
          },
          targetEvidenceRefs: ['screen-ref-wiki-ocr'],
          custody: 'parent-device-cache',
          updatedAt: observedAt,
          expiresAt: null,
        },
      ],
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: ['child-device-query-store'],
      promptVersion: 'prompt:screen-safety:v1',
    },
    evidenceReferences: [
      {
        evidenceRefId: 'screen-ref-wiki-ocr',
        evidence: sourceEvidence,
        evidenceKind: 'screen-summary',
        sourceSchemaVersion: 'v0.6',
        observedAt,
        ingestedAt: observedAt,
        freshUntil: null,
        sourceId: 'source:screen-winrt-ocr-worker',
        adapterId: 'adapter:winrt-ocr',
        device,
        childProfile,
        custody: 'child-device-query-store',
        retentionState: 'deleted-source',
        confidence: 0.72,
        confidenceKind: 'classifier',
        capabilityStatus: 'available',
        degradedReasons: [],
        unknownReasons: [],
        sourceEvidenceReferences: [sourceEvidence],
      },
    ],
    runtimeReferences: [readyRuntime],
    memoryReferences: [],
    graphReferences: [],
  };
}

function adapterInputFromContext(contextResult: ReturnType<typeof buildLocalAiEvidenceContext>) {
  const context = requireContext(contextResult);

  return {
    schemaVersion: 'v0.6',
    adapterRequestId: 'local-ai-text-adapter:stored-screen-wiki-ocr',
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    localAdapterAvailable: true,
    manualProofRequired: false,
    modelRuntime: readyRuntime,
    promptVersion: context.promptVersion,
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: context.requestId,
      childProfile,
      device,
      currentObservation: {
        observationReferenceId: 'observation:stored-screen-wiki-ocr',
        contextKind: 'page',
        evidence: sourceEvidence,
      },
      evidenceReferences: contextResult.auditEvidenceReferences,
      parentRuleReferences: context.parentRuleReferences,
      recentActivityWindow: contextResult.auditEvidenceReferences,
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: readyRuntime.providerId,
        modelId: readyRuntime.modelId,
        promptVersion: context.promptVersion,
      },
    },
  };
}

function provesIntegrationPath() {
  const contextResult = buildLocalAiEvidenceContext(storedContextInput());
  const parserProof = parseLocalAiTextOutput(parserInputFromContext(contextResult));
  const readModel = buildReadModel(parserProof);
  const policyHandoff = buildPolicyHandoff(readModel.rows);

  assertStoredContext(contextResult);
  expect(parserProof.state).toBe('parsed-local-result');
  expect(parserProof.rawModelOutputRetained).toBe(false);
  expect(readModel.readyRowCount).toBe(1);
  expect(readModel.rows[0]?.evidenceReferences).toEqual(contextResult.auditEvidenceReferences);
  assertPolicyHandoffRow(policyHandoff.rows[0]);
}

function parserInputFromContext(contextResult: ReturnType<typeof buildLocalAiEvidenceContext>) {
  return {
    schemaVersion: 'v0.6',
    parserRunId: 'local-ai-text-parser:stored-screen-wiki-ocr',
    adapterProof: proveLocalAiTextLlmAdapterBoundary(adapterInputFromContext(contextResult)),
    candidateOutput: candidateOutputFromContext(contextResult),
    rawModelOutputRetained: false,
  };
}

function buildReadModel(parserProof: ReturnType<typeof parseLocalAiTextOutput>) {
  return buildLocalAiTextParserReadModelSnapshot({
    generatedAt: observedAt,
    snapshotId: 'local-ai-text-parser-read-model:snapshot:stored-evidence-integration',
    sourceProofRefs: ['proof:stored-evidence-parser-integration'],
    parserProofs: [parserProof],
  });
}

function buildPolicyHandoff(readModelRows: readonly unknown[]) {
  return buildLocalAiTextParserPolicyHandoffProof({
    generatedAt: observedAt,
    proofId: 'local-ai-text-parser-policy-handoff:stored-evidence-integration',
    sourceProofRefs: ['proof:stored-evidence-parser-integration'],
    readModelRows,
  });
}

function assertStoredContext(contextResult: ReturnType<typeof buildLocalAiEvidenceContext>) {
  const context = requireContext(contextResult);
  const selectedScreenEvidence = context.evidenceReferences.find(
    (reference) => reference.evidenceRefId === 'screen-ref-wiki-ocr'
  );

  expect(contextResult.state).toBe('ready');
  expect(context.screenSummaryRefs).toEqual(['screen-ref-wiki-ocr']);
  expect(context.parentRuleReferences).toEqual(['policy-rule:screen-video-warn']);
  expect(selectedScreenEvidence?.retentionState).toBe('deleted-source');
  expect(selectedScreenEvidence?.custody).toBe('child-device-query-store');
}

function assertPolicyHandoffRow(handoffRow: ReturnType<typeof buildPolicyHandoff>['rows'][number] | undefined) {
  if (!handoffRow?.policyDecision) {
    throw new Error('expected policy handoff row');
  }

  expect(handoffRow.handoffState).toBe('policy-dry-run-ready');
  expect(handoffRow.policyDecision.action).toBe('warn');
  expect(handoffRow.policyDecision.dryRun).toBe(true);
  expect(handoffRow.policyDecision.enforcementHandoffState).toBe('disabled');
  expect(handoffRow.modelExecuted).toBe(false);
  expect(handoffRow.rawModelOutputRetained).toBe(false);
  expect(handoffRow.remoteApiClaimed).toBe(false);
  expect(handoffRow.policyAuthorityClaimed).toBe(false);
  expect(handoffRow.enforcementClaimed).toBe(false);
}

function candidateOutputFromContext(contextResult: ReturnType<typeof buildLocalAiEvidenceContext>) {
  const context = requireContext(contextResult);

  return {
    schemaVersion: 'v0.6',
    resultId: 'local-ai-text-result:stored-screen-wiki-ocr',
    requestId: context.requestId,
    action: 'warn',
    confidence: 0.72,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['local-ai-text:stored-screen-video-risk'],
    explanationReference: 'local-ai-text-explanation:stored-screen-wiki-ocr',
    evidenceReferences: contextResult.auditEvidenceReferences,
    parentRuleReferences: context.parentRuleReferences,
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: readyRuntime,
    promptVersion: context.promptVersion,
    expiresAt: null,
  };
}

function requireContext(contextResult: ReturnType<typeof buildLocalAiEvidenceContext>) {
  if (!contextResult.context) {
    throw new Error('expected stored evidence context');
  }
  return contextResult.context;
}
