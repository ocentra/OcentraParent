import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingEscalationRuntimeReadinessBlockers,
  TrackingEscalationRuntimeReadinessBlockerProofSchema,
  buildTrackingEscalationRuntimeReadinessBlockerProof,
} from '../../src/tracking-escalation-runtime-readiness-blocker-proof';
import { buildTrackingEscalationReadinessReadModel } from '../../src/tracking-escalation-readiness-proof';
import { TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';
import { RequiredTrackingProviderRuntimeReadinessBlockers } from '../../src/tracking-provider-runtime-readiness-blocker-proof';

describe('tracking escalation runtime readiness blocker proof', () => {
  it('aggregates escalation and provider runtime blockers without claiming runtime readiness', () => {
    const proof = buildTrackingEscalationRuntimeReadinessBlockerProof(
      {
        generatedAt: '2026-06-07T20:20:00.000Z',
        proofId: 'tracking-escalation-runtime-readiness-blocker-proof',
        sourceProofRefs: [
          'test-results/tracking-escalation-readiness-proof/proof.json',
          'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
        ],
      },
      escalationReadinessFixture(),
      providerRuntimeBlockerFixture()
    );

    expect(proof.escalationReadinessRows).toBe(1);
    expect(proof.providerRuntimeBlockerRows).toBe(RequiredTrackingProviderRuntimeReadinessBlockers.length);
    expect(proof.blockers.map((row) => row.blockerId)).toEqual([...RequiredTrackingEscalationRuntimeReadinessBlockers]);
    expect(proof.blockers.every((row) => row.status === 'manual-required')).toBe(true);
    expect(Object.values(proof.productClaims).every((claim) => claim === false)).toBe(true);
  });

  it('rejects product-ready runtime overclaims', () => {
    const proof = buildTrackingEscalationRuntimeReadinessBlockerProof(
      {
        generatedAt: '2026-06-07T20:20:00.000Z',
        proofId: 'tracking-escalation-runtime-readiness-blocker-proof',
        sourceProofRefs: [
          'test-results/tracking-escalation-readiness-proof/proof.json',
          'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
        ],
      },
      escalationReadinessFixture(),
      providerRuntimeBlockerFixture()
    );
    const unsafe = TrackingEscalationRuntimeReadinessBlockerProofSchema.safeParse({
      ...proof,
      productClaims: {
        ...proof.productClaims,
        productionEscalationWorkerRuntimeClaimed: true,
      },
    });

    expect(unsafe.success).toBe(false);
  });
});

function escalationReadinessFixture() {
  return buildTrackingEscalationReadinessReadModel(
    {
      generatedAt: '2026-06-07T20:20:00.000Z',
      readinessId: 'tracking-escalation-readiness-proof',
      sourceContractRefs: ['packages/tracking-domain/src/tracking-location-policy.ts'],
    },
    {
      schemaVersion: TrackingPolicySchemaVersion,
      generatedAt: '2026-06-07T20:20:00.000Z',
      rules: [
        {
          schemaVersion: TrackingPolicySchemaVersion,
          ruleId: 'tracking-escalation-runtime-rule',
          familyId: 'family-1',
          childProfileId: 'child-1',
          deviceId: 'child-device-1',
          policyVersion: 'tracking-policy-v1',
          targetKind: 'geofence-transition',
          action: 'escalate',
          enabled: true,
          requiresFreshEvidence: true,
          requiresParentConfirmation: true,
          reasonCodes: ['tracking-escalation-runtime-rule'],
          auditRefs: ['tracking-escalation-runtime-rule-audit'],
        },
      ],
      decisions: [
        {
          schemaVersion: TrackingPolicySchemaVersion,
          decisionId: 'tracking-escalation-runtime-decision',
          decidedAt: '2026-06-07T20:19:00.000Z',
          ruleId: 'tracking-escalation-runtime-rule',
          action: 'escalate',
          dryRun: false,
          evidenceReferences: [evidenceTrace()],
          aiAnalysisId: null,
          alertIntentId: 'tracking-escalation-runtime-alert',
          reasonCodes: ['tracking-escalation-runtime-decision'],
          auditRefs: ['tracking-escalation-runtime-decision-audit'],
        },
      ],
      acknowledgements: [],
      checkInRequests: [],
      checkInResponses: [],
      aiRoutes: [],
      aiResults: [],
      alerts: [
        {
          schemaVersion: TrackingPolicySchemaVersion,
          alertId: 'tracking-escalation-runtime-alert',
          createdAt: '2026-06-07T20:19:30.000Z',
          severity: 'critical',
          policyDecisionId: 'tracking-escalation-runtime-decision',
          evidenceReferences: [evidenceTrace()],
          sensitiveDetailMode: 'authenticated-drill-in-only',
          notificationStatusRefs: ['tracking-escalation-runtime-notification-status'],
          acknowledgementId: null,
          reasonCodes: ['tracking-escalation-runtime-alert'],
        },
      ],
      escalations: [
        {
          schemaVersion: TrackingPolicySchemaVersion,
          escalationId: 'tracking-escalation-runtime-escalation',
          alertId: 'tracking-escalation-runtime-alert',
          state: 'waiting-for-parent',
          startedAt: '2026-06-07T20:19:30.000Z',
          nextActionAt: '2026-06-07T20:25:00.000Z',
          steps: ['wait-parent-acknowledgement', 'guardian-manual-review'],
          auditRefs: ['tracking-escalation-runtime-escalation-audit'],
        },
      ],
      temporaryLiveGrants: [],
      missingDeviceCases: [],
      platformProofRoutes: [],
    }
  );
}

function providerRuntimeBlockerFixture() {
  const sourceProofRefs = [
    'test-results/tracking-provider-notification-proof/proof.json',
    'test-results/tracking-notification-receipt-boundary-proof/proof.json',
    'test-results/tracking-notification-local-outbox-readiness-proof/proof.json',
    'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
  ];
  const requiredProviderRuntimeArtifactRefs = ['provider-credentials'];
  const presentProviderRuntimeArtifactRefs: string[] = [];
  const missingProviderRuntimeArtifactRefs = ['provider-credentials'];

  return {
    schemaVersion: 'v0.6',
    proofId: 'tracking-provider-runtime-readiness-blocker-proof',
    generatedAt: '2026-06-07T20:20:00.000Z',
    proofMode: 'tracking-provider-runtime-readiness-blocker-proof',
    sourceProofRefs,
    providerNotificationRows: 1,
    receiptBoundaryRows: 1,
    localOutboxReadinessRows: 1,
    requiredProviderRuntimeArtifactCount: requiredProviderRuntimeArtifactRefs.length,
    presentProviderRuntimeArtifactCount: presentProviderRuntimeArtifactRefs.length,
    missingProviderRuntimeArtifactCount: 1,
    requiredProviderRuntimeArtifactRefs,
    presentProviderRuntimeArtifactRefs,
    missingProviderRuntimeArtifactRefs,
    providerRuntimeArtifactSetComplete: false,
    blockers: RequiredTrackingProviderRuntimeReadinessBlockers.map((blockerId) => ({
      blockerId,
      status: 'manual-required',
      sourceProofRefs,
      blockingArtifactRefs: missingProviderRuntimeArtifactRefs,
      requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      productClaimReady: false,
    })),
    productClaims: {
      providerDeliveryRuntimeClaimed: false,
      webhookReceiptIngestionRuntimeClaimed: false,
      providerCredentialsClaimed: false,
      adapterDispatchClaimed: false,
      retryExecutionRuntimeClaimed: false,
      quietHoursTimerRuntimeClaimed: false,
      parentNotificationUiRuntimeClaimed: false,
      productionDurableOutboxStorageClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productClaimReady: false,
    },
  };
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-escalation-runtime-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-07T20:19:00.000Z',
  } as const;
}
