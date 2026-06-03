import { describe, expect, it } from 'vitest';
import {
  BrowserGameEducationalClassifierResultSchema,
  type BrowserGameEducationalClassifierResult,
  BrowserGameEducationalEvidenceRowSchema,
} from '../src/browser-game-educational-classifier';

describe('browser-game educational classifier contracts', () => {
  it('accepts verified educational game candidates without claiming policy authority', acceptsVerifiedCandidate);
  it(
    'accepts misleading educational claim candidates without treating platform labels as authority',
    acceptsMisleadingClaim
  );
  it('rejects raw content, policy, runtime, UI, native, cloud-frame, and enforcement claims', rejectsAuthorityClaims);
  it('rejects inconsistent educational, unknown, and degraded states', rejectsInconsistentStates);
});

function acceptsVerifiedCandidate() {
  const parsed = BrowserGameEducationalClassifierResultSchema.parse(classifierResult());

  expect(parsed.schemaVersion).toBe('browser-game-educational-classifier-contract');
  expect(parsed.outcome).toBe('educational-candidate');
  expect(parsed.recommendedGate).toBe('allow-during-homework-candidate');
  expect(parsed.finalPolicyDecisionClaimed).toBe(false);
  expect(parsed.runtimeGateExecutedClaimed).toBe(false);
  expect(parsed.enforcementClaimed).toBe(false);
}

function acceptsMisleadingClaim() {
  const parsed = BrowserGameEducationalClassifierResultSchema.parse(
    classifierResult({
      classifierResultId: 'browser-game-educational-classifier-misleading-claim',
      category: 'unknown-educational-category',
      outcome: 'misleading-educational-claim',
      confidence: 'medium',
      recommendedGate: 'block-portal-candidate',
      evidenceRows: [
        evidenceRow({
          evidenceRowId: 'browser-game-educational-evidence-self-label',
          evidenceKind: 'platform-self-label',
          confidence: 'low',
          schoolOrParentVerified: false,
          platformSelfLabelOnly: true,
        }),
        evidenceRow({
          evidenceRowId: 'browser-game-educational-evidence-domain',
          evidenceKind: 'domain-reputation',
          confidence: 'medium',
        }),
      ],
    })
  );

  expect(parsed.outcome).toBe('misleading-educational-claim');
  expect(parsed.platformLabelTreatedAsAuthority).toBe(false);
}

function rejectsAuthorityClaims() {
  const invalidRows = [
    { rawPageBodyUsed: true },
    { rawGamePayloadUsed: true },
    { rawModelTextUsed: true },
    { accountOrPurchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { policyDecisionClaimed: true },
    { enforcementClaimed: true },
  ];
  const invalidResults = [
    { rawPageBodyUsed: true },
    { rawGamePayloadUsed: true },
    { rawModelTextUsed: true },
    { platformLabelTreatedAsAuthority: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { uiRenderedClaimed: true },
    { accountOrPurchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameEducationalEvidenceRowSchema.safeParse(evidenceRow(invalid)).success).toBe(false);
  }
  for (const invalid of invalidResults) {
    expect(BrowserGameEducationalClassifierResultSchema.safeParse(classifierResult(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentStates() {
  const invalidRows = [
    { evidenceRows: [] },
    { category: 'unknown-educational-category' },
    {
      evidenceRows: [
        evidenceRow({
          evidenceKind: 'platform-self-label',
          confidence: 'low',
          schoolOrParentVerified: false,
          platformSelfLabelOnly: true,
        }),
      ],
    },
    { outcome: 'unknown-candidate', confidence: 'high', uncertaintyReasons: [] },
    {
      degradedState: 'degraded',
      confidence: 'high',
      outcome: 'unknown-candidate',
      uncertaintyReasons: ['low-confidence'],
      recommendedGate: 'ask-parent-candidate',
    },
    {
      outcome: 'manual-required',
      degradedState: 'none',
      recommendedGate: 'manual-review-candidate',
    },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameEducationalClassifierResultSchema.safeParse(classifierResult(invalid)).success).toBe(false);
  }
}

function classifierResult(overrides = {}): BrowserGameEducationalClassifierResult {
  return {
    schemaVersion: 'browser-game-educational-classifier-contract',
    classifierResultId: 'browser-game-educational-classifier-math',
    familyId: 'family-browser-game-education',
    childProfileId: 'child-browser-game-education',
    deviceId: 'device-browser-game-education',
    classifiedAt: '2026-06-03T10:15:00.000Z',
    sourceEvidenceRefs: ['browser-game-school-source-evidence', 'browser-game-homework-context-evidence'],
    evidenceRows: [
      evidenceRow({
        evidenceKind: 'school-provided-url',
        schoolOrParentVerified: true,
      }),
      evidenceRow({
        evidenceRowId: 'browser-game-educational-evidence-subject',
        evidenceKind: 'subject-metadata',
      }),
    ],
    category: 'math',
    outcome: 'educational-candidate',
    confidence: 'high',
    recommendedGate: 'allow-during-homework-candidate',
    degradedState: 'none',
    uncertaintyReasons: [],
    homeworkContextRef: 'browser-game-homework-context-evidence',
    parentAllowlistRef: null,
    schoolSourceRef: 'browser-game-school-source-evidence',
    aiAnalysisRef: 'browser-game-educational-ai-evidence',
    metadataRef: 'browser-game-educational-metadata-evidence',
    rawPageBodyUsed: false,
    rawGamePayloadUsed: false,
    rawModelTextUsed: false,
    platformLabelTreatedAsAuthority: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    accountOrPurchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function evidenceRow(overrides = {}) {
  return {
    evidenceRowId: 'browser-game-educational-evidence-school-url',
    evidenceKind: 'school-provided-url',
    evidenceRefs: ['browser-game-educational-evidence-ref'],
    confidence: 'high',
    schoolOrParentVerified: false,
    platformSelfLabelOnly: false,
    rawPageBodyUsed: false,
    rawGamePayloadUsed: false,
    rawModelTextUsed: false,
    accountOrPurchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
