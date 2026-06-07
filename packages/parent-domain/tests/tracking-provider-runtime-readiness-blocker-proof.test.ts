import { describe, expect, it } from 'vitest';
import { NotificationLocalOutboxAdapterProofReadModel } from '../src/notification-local-outbox-adapter-proof';
import { NotificationLocalOutboxSchedulerProofReadModel } from '../src/notification-local-outbox-scheduler-proof';
import {
  RequiredTrackingProviderRuntimeReadinessBlockers,
  TrackingProviderRuntimeReadinessBlockerProofSchema,
  buildTrackingProviderRuntimeReadinessBlockerProof,
} from '../src/tracking-provider-runtime-readiness-blocker-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';
import { buildTrackingNotificationLocalOutboxReadinessReadModel } from '../src/tracking-notification-local-outbox-readiness-proof';
import { buildTrackingNotificationReceiptBoundaryReadModel } from '../src/tracking-notification-receipt-boundary-proof';
import { buildTrackingProviderDeliveryArtifactGateProof } from '../src/tracking-provider-delivery-artifact-gate-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../src/tracking-provider-notification-proof';

const generatedAt = '2026-06-07T20:10:00.000Z';
const evidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-07T20:05:00.000Z',
} as const;
const sourceProofRefs = [
  'test-results/tracking-provider-notification-proof/proof.json',
  'test-results/tracking-notification-receipt-boundary-proof/proof.json',
  'test-results/tracking-notification-local-outbox-readiness-proof/proof.json',
  'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
];

describe('tracking provider runtime readiness blocker proof', () => {
  it('aggregates provider, receipt, local outbox, and artifact-gate blockers', () => {
    const proof = buildProof();

    expect(proof.blockers).toHaveLength(RequiredTrackingProviderRuntimeReadinessBlockers.length);
    expect(proof.providerNotificationRows).toBeGreaterThan(0);
    expect(proof.receiptBoundaryRows).toBeGreaterThan(0);
    expect(proof.localOutboxReadinessRows).toBeGreaterThan(0);
    expect(proof.missingProviderRuntimeArtifactCount).toBeGreaterThan(0);
    expect(proof.blockers.every((row) => row.status === 'manual-required')).toBe(true);
    expect(proof.productClaims.providerDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.webhookReceiptIngestionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps every provider runtime blocker tied to source proof refs and missing artifacts', () => {
    const proof = buildProof();

    expect(proof.sourceProofRefs).toEqual(sourceProofRefs);
    for (const row of proof.blockers) {
      expect(row.sourceProofRefs).toEqual(sourceProofRefs);
      expect(row.blockingArtifactRefs).toContain('00-run-metadata.json');
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('rejects aggregate proofs that have no missing runtime artifact blocker', () => {
    const proof = buildProof();
    const invalid = TrackingProviderRuntimeReadinessBlockerProofSchema.safeParse({
      ...proof,
      missingProviderRuntimeArtifactCount: 0,
    });

    expect(invalid.success).toBe(false);
  });
});

function buildProof() {
  const sourceReadModel = sourceTrackingReadModel();
  const providerProof = buildTrackingProviderNotificationProofReadModel(
    {
      generatedAt,
      proofId: 'tracking-provider-notification-proof',
      familyId: 'family-tracking-provider-runtime-readiness',
      sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
      sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
    },
    sourceReadModel
  );
  const receiptProof = buildTrackingNotificationReceiptBoundaryReadModel(
    {
      generatedAt,
      proofId: 'tracking-notification-receipt-boundary-proof',
      familyId: 'family-tracking-provider-runtime-readiness',
      sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
      sourceContractRefs: ['tracking-provider-notification-proof', 'v0-8-notification-provider-status-boundary'],
    },
    providerProof
  );
  const localOutboxProof = buildTrackingNotificationLocalOutboxReadinessReadModel(
    {
      generatedAt,
      proofId: 'tracking-notification-local-outbox-readiness-proof',
      sourceContractRefs: [
        'tracking-notification-receipt-boundary-proof',
        'notification-local-outbox-adapter-proof',
        'notification-local-outbox-scheduler-proof',
      ],
    },
    receiptProof,
    NotificationLocalOutboxAdapterProofReadModel,
    NotificationLocalOutboxSchedulerProofReadModel
  );
  const artifactGateProof = buildTrackingProviderDeliveryArtifactGateProof(generatedAt, { presentArtifacts: [] });

  return buildTrackingProviderRuntimeReadinessBlockerProof(
    {
      generatedAt,
      proofId: 'tracking-provider-runtime-readiness-blocker-proof',
      sourceProofRefs,
    },
    providerProof,
    receiptProof,
    localOutboxProof,
    artifactGateProof
  );
}

function sourceTrackingReadModel() {
  return TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt,
    rules: [],
    decisions: [],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [
      alert({
        alertId: 'tracking-alert-home-arrival',
        severity: 'info',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-home-arrival',
        notificationStatusRefs: ['tracking-notification-intent-home-arrival'],
        reasonCodes: ['home-arrival-notification'],
      }),
      alert({
        alertId: 'tracking-alert-left-expected-place',
        severity: 'urgent',
        sensitiveDetailMode: 'authenticated-drill-in-only',
        policyDecisionId: 'tracking-decision-left-school',
        notificationStatusRefs: ['tracking-notification-intent-left-school'],
        reasonCodes: ['left-expected-place'],
      }),
      alert({
        alertId: 'tracking-alert-provider-unavailable',
        severity: 'warning',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-provider-unavailable',
        notificationStatusRefs: [],
        reasonCodes: ['provider-unavailable'],
      }),
    ],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  });
}

function alert(input: {
  readonly alertId: string;
  readonly severity: 'info' | 'watch' | 'warning' | 'urgent' | 'critical';
  readonly sensitiveDetailMode: 'minimal-provider-body' | 'authenticated-drill-in-only';
  readonly policyDecisionId: string;
  readonly notificationStatusRefs: readonly string[];
  readonly reasonCodes: readonly string[];
}) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    createdAt: generatedAt,
    evidenceReferences: [evidenceTrace],
    acknowledgementId: null,
    ...input,
  };
}
