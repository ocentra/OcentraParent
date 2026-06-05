import { describe, expect, it } from 'vitest';
import {
  TrackingProviderNotificationProofReadModelSchema,
  TrackingProviderNotificationProofRowSchema,
  buildTrackingProviderNotificationProofReadModel,
} from '../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

const Timestamp = '2026-06-05T10:46:00.000Z';
const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-05T10:40:00.000Z',
} as const;

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-provider-notification-proof',
  familyId: 'family-tracking-provider-notification',
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
  sourceContractRefs: [
    'tracking-location-policy',
    'v0-8-notification-provider-status-boundary',
    'notification-local-outbox-adapter-proof',
    'location-geofence-device-status',
  ],
} as const;

describe('tracking provider notification proof', () => {
  it('maps tracking alerts into provider-boundary status evidence without delivery claims', () => {
    const readModel = buildTrackingProviderNotificationProofReadModel(ProofOptions, sourceTrackingReadModel());

    expect(readModel.rows.map((row) => row.providerStatusKind)).toEqual([
      'provider-adapter-required',
      'manual-required',
      'unavailable',
    ]);
    expect(readModel.providerAdapterRequiredCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)).toEqual([
      'manual-required',
      'manual-required',
      'unavailable',
    ]);
    expect(readModel.providerStatusBoundaryCoverageRefs).toEqual([
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ]);
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('preserves tracking evidence, policy, reason, notification status, and minimal payload boundaries', () => {
    const readModel = buildTrackingProviderNotificationProofReadModel(ProofOptions, sourceTrackingReadModel());
    const adapterRequired = readModel.rows[0];
    const urgent = readModel.rows[1];
    const unavailable = readModel.rows[2];

    expect(adapterRequired.sourcePolicyDecisionId).toBe('tracking-decision-home-arrival');
    expect(adapterRequired.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(adapterRequired.reasonCodeRefs).toEqual(['home-arrival-notification']);
    expect(adapterRequired.notificationStatusRefs).toEqual(['tracking-notification-intent-home-arrival']);
    expect(adapterRequired.providerStatusBoundaryEntry.minimalPayloadBoundary).toContain('minimal parent alert copy');
    expect(urgent.providerStatusBoundaryEntry.minimalPayloadBoundary).toContain('authenticated drill-in only');
    expect(urgent.providerStatusBoundaryEntry.manualProofRequirements).toContain(
      'tracking-provider-critical-escalation-review-tracking-alert-left-expected-place'
    );
    expect(unavailable.providerStatusBoundaryEntry.providerStatus).toBe('unavailable');
    expect(unavailable.providerStatusBoundaryEntry.notificationStatusRef).toBe(
      'tracking-provider-status-unavailable-tracking-alert-provider-unavailable'
    );
  });

  it('rejects rows that drop evidence or upgrade provider delivery claims', () => {
    const readModel = buildTrackingProviderNotificationProofReadModel(ProofOptions, sourceTrackingReadModel());
    const row = readModel.rows[0];

    expect(
      TrackingProviderNotificationProofRowSchema.safeParse({
        ...row,
        evidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingProviderNotificationProofReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingProviderNotificationProofRowSchema.safeParse({
        ...row,
        providerStatusBoundaryEntry: {
          ...row.providerStatusBoundaryEntry,
          providerDeliveryObserved: true,
        },
      }).success
    ).toBe(false);
  });
});

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingProviderNotificationProofReadModel>) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  };
}

function sourceTrackingReadModel() {
  return TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: Timestamp,
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
    createdAt: Timestamp,
    evidenceReferences: [EvidenceTrace],
    acknowledgementId: null,
    ...input,
  };
}
