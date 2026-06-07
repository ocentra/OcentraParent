import { describe, expect, it } from 'vitest';
import {
  TrackingAiStoredRefConsumerRowSchema,
  buildTrackingAiStoredRefConsumerProof,
  type TrackingAiStoredRefConsumerProof,
  type TrackingAiStoredRefConsumerRow,
} from '../src/tracking-ai-stored-ref-consumer-proof';

const GeneratedAt = '2026-06-07T05:44:00.000Z';

describe('tracking AI stored-ref consumer proof', () => {
  it('builds AI report, policy, and metadata consumer rows without product overclaims', () => {
    const proof = buildTrackingAiStoredRefConsumerProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-ai-stored-ref-consumer-proof');
    expect(proof.rows.map((row) => row.consumerKind)).toEqual([
      'ai-parent-report-context',
      'ai-policy-drill-in-context',
      'ai-metadata-fallback-context',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    for (const row of proof.rows) {
      expectConsumerReadiness(row);
      expectNoProductClaims(row);
    }
  });

  it('ties consumer rows to the existing provider route matrix', () => {
    const proof = buildTrackingAiStoredRefConsumerProof(GeneratedAt);

    expect(rowFor(proof, 'ai-parent-report-context').analysisInput.providerRouteId).toBe(
      'parent-local-tracking-ai-route'
    );
    expect(rowFor(proof, 'ai-policy-drill-in-context').analysisInput.providerRouteId).toBe(
      'child-local-tracking-ai-route'
    );
    expect(rowFor(proof, 'ai-metadata-fallback-context').analysisInput.providerRouteId).toBe(
      'metadata-only-tracking-ai-route'
    );
  });

  it('rejects AI consumer rows without stored refs or route proof refs', () => {
    const row = rowFor(buildTrackingAiStoredRefConsumerProof(GeneratedAt), 'ai-parent-report-context');

    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, storedJournalRefs: [] }).success).toBe(false);
    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, storedReadModelRowRefs: [] }).success).toBe(false);
    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, aiProviderRouteProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, reportPolicyConsumerProofRefs: [] }).success).toBe(
      false
    );
  });

  it('rejects model execution and assistant policy write upgrades', () => {
    const row = rowFor(buildTrackingAiStoredRefConsumerProof(GeneratedAt), 'ai-policy-drill-in-context');

    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, modelExecutionClaimed: true }).success).toBe(false);
    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, assistantPolicyWriteClaimed: true }).success).toBe(
      false
    );
    expect(TrackingAiStoredRefConsumerRowSchema.safeParse({ ...row, assistantEnforcementClaimed: true }).success).toBe(
      false
    );
  });
});

function rowFor(
  proof: TrackingAiStoredRefConsumerProof,
  consumerKind: TrackingAiStoredRefConsumerRow['consumerKind']
): TrackingAiStoredRefConsumerRow {
  const row = proof.rows.find((entry) => entry.consumerKind === consumerKind);
  if (row === undefined) {
    throw new Error(`Missing tracking AI stored-ref consumer row: ${consumerKind}`);
  }
  return row;
}

function expectConsumerReadiness(row: TrackingAiStoredRefConsumerRow): void {
  expect(row.readinessState).toBe('stored-ref-consumer-ready');
  expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
  expect(row.currentProofTier).toBe('P2_HOSTED_CI');
  expect(row.sourceProofRefs.length).toBeGreaterThan(0);
  expect(row.aiProviderRouteProofRefs).toContain(
    'output/tracking-plan-proof/24-ai-provider-routing/18-ai-provider-routing-custody-proof.json'
  );
  expect(row.reportPolicyConsumerProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json'
  );
  expect(row.storedJournalRefs.length).toBeGreaterThan(0);
  expect(row.storedReadModelRowRefs.length).toBeGreaterThan(0);
  expect(row.evidenceReferences.length).toBeGreaterThan(0);
  expect(row.analysisInput.evidenceReferences).toEqual(row.evidenceReferences);
}

function expectNoProductClaims(row: TrackingAiStoredRefConsumerRow): void {
  expect(row.aiStoredRefConsumerClaimed).toBe(true);
  expect(row.modelExecutionClaimed).toBe(false);
  expect(row.assistantPolicyWriteClaimed).toBe(false);
  expect(row.assistantEnforcementClaimed).toBe(false);
  expect(row.childDeviceRuntimeClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
  expect(row.productionBehaviorClaimed).toBe(false);
  expect(row.productClaimReady).toBe(false);
}
