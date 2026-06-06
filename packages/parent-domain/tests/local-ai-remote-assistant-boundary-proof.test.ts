import { describe, expect, it } from 'vitest';
import {
  LocalAiRemoteAssistantBoundaryProof,
  LocalAiRemoteAssistantBoundaryProofSchema,
  RemoteAssistantRequestSchema,
  RemoteAssistantResultSchema,
} from '../src/local-ai-remote-assistant-boundary-proof';

describe('local AI remote assistant boundary proof', () => {
  it('keeps parent-authorized remote assistance outside the child safety decision path', provesReadyBoundary);
  it('keeps remote provider failures as local-only fallback with cited local policy evidence', provesFallbackBoundary);
  it('rejects child-safety, missing evidence, and raw-retention request overclaims', rejectsRequestOverclaims);
  it(
    'rejects remote answers that claim policy authority, enforcement, or local-policy override',
    rejectsResultOverclaims
  );
});

function provesReadyBoundary() {
  const proof = LocalAiRemoteAssistantBoundaryProof;

  expect(proof.readyRequest).toMatchObject({
    parentAuthorizedRemoteUse: true,
    childSafetyDecisionPath: false,
    rawPromptRetained: false,
    custodyBoundary: 'parent-authorized-report-bundle',
  });
  expect(proof.readyRequest.approvedSourceEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual(
    ['evidence:local-ai:screen-summary-parent-bundle']
  );
  expect(proof.readyResult).toMatchObject({
    executionState: 'ready-answer',
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    childSafetyDecisionPath: false,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    remoteApiAiUsed: true,
    remoteOutputAllowedToOverrideLocalPolicy: false,
  });
  expect(proof.readyResult.localPolicyDecision.action).toBe('block');
  expect(proof.readyResult.remoteSuggestedPolicyDecision?.action).toBe('allow');
  expect(proof.readyResult.citedEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
    'evidence:local-ai:screen-summary-parent-bundle',
  ]);
}

function provesFallbackBoundary() {
  const fallback = LocalAiRemoteAssistantBoundaryProof.fallbackResult;

  expect(fallback).toMatchObject({
    executionState: 'local-only-fallback',
    answerRef: null,
    failureReason: 'remote-provider-unavailable',
    remoteApiAiUsed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    childSafetyDecisionPath: false,
  });
  expect(fallback.localPolicyDecision.action).toBe('block');
  expect(fallback.citedEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
    'evidence:local-ai:screen-summary-parent-bundle',
  ]);
}

function rejectsRequestOverclaims() {
  const request = LocalAiRemoteAssistantBoundaryProof.readyRequest;

  expect(() => RemoteAssistantRequestSchema.parse({ ...request, childSafetyDecisionPath: true })).toThrow();
  expect(() => RemoteAssistantRequestSchema.parse({ ...request, parentAuthorizedRemoteUse: false })).toThrow();
  expect(() => RemoteAssistantRequestSchema.parse({ ...request, rawPromptRetained: true })).toThrow();
  expect(() => RemoteAssistantRequestSchema.parse({ ...request, approvedSourceEvidenceReferences: [] })).toThrow();
  expect(() => RemoteAssistantRequestSchema.parse({ ...request, permittedReportBundleRefs: [] })).toThrow();
}

function rejectsResultOverclaims() {
  const result = LocalAiRemoteAssistantBoundaryProof.readyResult;

  expect(() => RemoteAssistantResultSchema.parse({ ...result, policyAuthorityClaimed: true })).toThrow();
  expect(() => RemoteAssistantResultSchema.parse({ ...result, enforcementClaimed: true })).toThrow();
  expect(() => RemoteAssistantResultSchema.parse({ ...result, rawModelOutputRetained: true })).toThrow();
  expect(() =>
    RemoteAssistantResultSchema.parse({ ...result, remoteOutputAllowedToOverrideLocalPolicy: true })
  ).toThrow();
  expect(() =>
    RemoteAssistantResultSchema.parse({
      ...result,
      citedEvidenceReferences: [
        {
          evidenceReferenceId: 'evidence:local-ai:uncited-remote-answer',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T09:35:00.000Z',
        },
      ],
    })
  ).toThrow();
  expect(() =>
    LocalAiRemoteAssistantBoundaryProofSchema.parse({
      ...LocalAiRemoteAssistantBoundaryProof,
      validationSummary: { ...LocalAiRemoteAssistantBoundaryProof.validationSummary, citedEvidenceReferenceCount: 2 },
    })
  ).toThrow();
}
