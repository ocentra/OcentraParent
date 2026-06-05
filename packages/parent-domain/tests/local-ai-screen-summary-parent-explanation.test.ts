import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../src/local-ai-context-builder';
import {
  buildScreenSummaryParentExplanation,
  ScreenSummaryParentExplanationInputSchema,
  ScreenSummaryParentExplanationSchema,
} from '../src/local-ai-screen-summary-parent-explanation';

const ObservedAt = '2026-06-05T10:43:30.710Z';
const ChildProfile = { childProfileId: 'screen-summary-explanation-child', displayName: 'Sam' };
const Device = {
  deviceId: 'screen-summary-explanation-windows-device',
  childProfileId: ChildProfile.childProfileId,
  label: 'Sam Windows PC',
  platform: 'windows',
};
const SourceEvidence = {
  evidenceReferenceId: 'screen-winrt-ocr-evidence-live-wikipedia-browser-ocr',
  kind: 'journal-event',
  observedAt: ObservedAt,
};
const RuntimeStatus = {
  runtimeReferenceId: 'screen-summary-explanation-runtime',
  providerId: 'windows-winrt-ocr',
  modelId: 'windows-winrt-ocr',
  modelReference: 'windows-winrt-ocr-local-runtime',
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
} as const;
const ScreenSummaryRefId = 'screen-winrt-ocr-result-live-wikipedia-browser-ocr-screen-summary-ref';
const ParentRuleContextReference = {
  parentRuleRefId: 'screen-summary-explanation-parent-rule-context',
  policyVersion: 'screen-summary-explanation-policy-v1',
  family: { familyId: 'screen-summary-explanation-family' },
  childProfile: ChildProfile,
  device: Device,
  rule: {
    ruleId: 'screen-winrt-ocr-rule-school',
    target: { targetId: 'screen-summary-school-target', targetType: 'category', targetValue: 'school' },
    action: 'allow',
    scheduleId: null,
    priority: 10,
    reasonCode: 'screen-winrt-ocr-school',
    createdBy: { actorId: 'parent-1', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: [ScreenSummaryRefId],
  custody: 'parent-device-cache',
  updatedAt: ObservedAt,
  expiresAt: null,
} as const;

describe('local AI screen-summary parent explanation contracts', () => {
  it('builds a parent explanation from ready deleted screen-summary context and dry-run policy refs', () => {
    const explanation = buildScreenSummaryParentExplanation(explanationInput());

    expect(explanation.readiness).toBe('ready-for-parent-audit');
    expect(explanation.screenSummaryRefs).toEqual([ScreenSummaryRefId]);
    expect(explanation.auditEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
      'stored-screen-summary-explanation-ref',
    ]);
    expect(explanation.policyDecisionRef).toBe('screen-winrt-ocr-policy-live-wikipedia-browser-ocr');
    expect(explanation.policyAction).toBe('allow');
    expect(explanation.policyReasonCodes).toEqual(['screen-winrt-ocr-school']);
    expect(explanation.policyDryRun).toBe(true);
    expect(explanation.enforcementHandoffState).toBe('disabled');
    expect(explanation.custodyLabels).toEqual(['child-device-query-store']);
    expect(explanation.deletionReasons).toEqual(['screen-image-deleted']);
    expect(explanation.claimBoundaries).toEqual(noClaims());
  });

  it('rejects remote AI, raw image retention, policy authority, portal runtime, or enforcement claims', () => {
    const invalidBoundaries = [
      { rawImageRetained: true },
      { remoteAiUsed: true },
      { apiAiUsed: true },
      { policyAuthorityClaimed: true },
      { enforcementClaimed: true },
      { portalRuntimeClaimed: true },
    ];

    for (const boundary of invalidBoundaries) {
      expect(
        ScreenSummaryParentExplanationInputSchema.safeParse({
          ...explanationInput(),
          claimBoundaries: { ...noClaims(), ...boundary },
        }).success
      ).toBe(false);
    }
  });

  it('rejects hosted custody and undeleted screen-image context before explanation', () => {
    const hosted = explanationInput({
      custody: 'ocentra-hosted-non-activity',
      allowedCustody: ['ocentra-hosted-non-activity'],
    });
    const temporary = explanationInput({
      retentionState: 'temporary',
      degradedReasons: ['screen-deletion-unconfirmed'],
    });

    expect(ScreenSummaryParentExplanationInputSchema.safeParse(hosted).success).toBe(false);
    expect(ScreenSummaryParentExplanationInputSchema.safeParse(temporary).success).toBe(false);
  });

  it('rejects policy decisions that are not dry-run explanation inputs', () => {
    const enforcing = explanationInput({
      policyDecisionOverrides: { dryRun: false, enforcementHandoffState: 'handed-off' },
    });

    expect(ScreenSummaryParentExplanationInputSchema.safeParse(enforcing).success).toBe(false);
    expect(
      ScreenSummaryParentExplanationSchema.safeParse({
        ...buildScreenSummaryParentExplanation(explanationInput()),
        claimBoundaries: { ...noClaims(), enforcementClaimed: true },
      }).success
    ).toBe(false);
  });
});

function explanationInput(overrides: ExplanationInputOverrides = {}) {
  return {
    schemaVersion: 'v0.6',
    explanationId: 'screen-summary-parent-explanation-live-wikipedia-browser-ocr',
    generatedAt: ObservedAt,
    contextResult: buildLocalAiEvidenceContext(contextInput(overrides)),
    policyDecision: {
      schemaVersion: 'v0.6',
      decisionId: 'screen-winrt-ocr-policy-live-wikipedia-browser-ocr',
      action: 'allow',
      reasonCodes: ['screen-winrt-ocr-school'],
      evidenceReferences: [
        {
          evidenceReferenceId: 'screen-winrt-ocr-policy-evidence-live-wikipedia-browser-ocr',
          kind: 'activity-event',
          observedAt: ObservedAt,
        },
      ],
      ruleIds: ['screen-winrt-ocr-rule-school'],
      localAiResultId: null,
      dryRun: true,
      enforcementHandoffState: 'disabled',
      expiresAt: null,
      ...overrides.policyDecisionOverrides,
    },
    claimBoundaries: noClaims(),
  };
}

function contextInput(overrides: ExplanationInputOverrides) {
  return {
    contextId: 'screen-summary-explanation-context',
    request: {
      schemaVersion: 'v0.6',
      requestId: 'screen-summary-explanation-context-request',
      requestedAt: ObservedAt,
      childProfile: ChildProfile,
      device: Device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [ParentRuleContextReference],
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: overrides.allowedCustody ?? ['child-device-query-store'],
      promptVersion: 'screen-summary-parent-explanation-v1',
    },
    evidenceReferences: [screenSummaryEvidence(overrides)],
    runtimeReferences: [RuntimeStatus],
    memoryReferences: [],
    graphReferences: [],
  };
}

function screenSummaryEvidence(overrides: ExplanationInputOverrides) {
  return {
    evidenceRefId: ScreenSummaryRefId,
    evidence: {
      evidenceReferenceId: 'stored-screen-summary-explanation-ref',
      kind: 'query-store-summary',
      observedAt: ObservedAt,
    },
    evidenceKind: 'screen-summary',
    sourceSchemaVersion: 'v0.6',
    observedAt: ObservedAt,
    ingestedAt: ObservedAt,
    freshUntil: null,
    sourceId: 'screen-winrt-ocr-result-live-wikipedia-browser-ocr',
    adapterId: 'windows-winrt-ocr-local-runtime',
    device: Device,
    childProfile: ChildProfile,
    custody: overrides.custody ?? 'child-device-query-store',
    retentionState: overrides.retentionState ?? 'deleted-source',
    confidence: 0.88,
    confidenceKind: 'classifier',
    capabilityStatus: 'available',
    degradedReasons: overrides.degradedReasons ?? ['screen-image-deleted'],
    unknownReasons: [],
    sourceEvidenceReferences: [SourceEvidence],
  };
}

function noClaims() {
  return {
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalRuntimeClaimed: false,
  };
}

interface ExplanationInputOverrides {
  allowedCustody?: readonly ('child-device-query-store' | 'ocentra-hosted-non-activity')[];
  custody?: 'child-device-query-store' | 'ocentra-hosted-non-activity';
  degradedReasons?: readonly ('screen-image-deleted' | 'screen-deletion-unconfirmed')[];
  policyDecisionOverrides?: Record<string, unknown>;
  retentionState?: 'deleted-source' | 'temporary';
}
