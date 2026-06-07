import { describe, expect, it } from 'vitest';
import { NotificationLocalOutboxAdapterProofReadModel } from '../src/notification-local-outbox-adapter-proof';
import { NotificationLocalOutboxSchedulerProofReadModel } from '../src/notification-local-outbox-scheduler-proof';
import {
  TrackingNotificationLocalOutboxReadinessReadModelSchema,
  TrackingNotificationLocalOutboxReadinessRowSchema,
  buildTrackingNotificationLocalOutboxReadinessReadModel,
} from '../src/tracking-notification-local-outbox-readiness-proof';
import { buildTrackingNotificationReceiptBoundaryReadModel } from '../src/tracking-notification-receipt-boundary-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

const Timestamp = '2026-06-07T15:03:00.000Z';
const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-07T15:00:00.000Z',
} as const;

describe('tracking notification local outbox readiness proof', () => {
  it('ties tracking receipt rows to existing local outbox and scheduler rows', () => {
    const readModel = readinessReadModel();

    expect(readModel.rows.map((row) => row.readinessState)).toEqual([
      'local-outbox-receipt-required',
      'local-outbox-manual-required',
      'local-outbox-provider-unavailable',
    ]);
    expect(readModel.receiptRequiredCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.providerUnavailableCount).toBe(1);
    expect(readModel.sourceLocalOutboxAdapterProofRef).toBe(NotificationLocalOutboxAdapterProofReadModel.readModelId);
    expect(readModel.sourceLocalOutboxSchedulerProofRef).toBe(
      NotificationLocalOutboxSchedulerProofReadModel.readModelId
    );
  });

  it('preserves tracking evidence and local artifact refs without provider or durable storage claims', () => {
    const readModel = readinessReadModel();
    const receiptRequired = readModel.rows[0];
    const manualRequired = readModel.rows[1];
    const providerUnavailable = readModel.rows[2];

    expect(receiptRequired.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(receiptRequired.receiptRequirementRefs).toContain(
      'tracking-receipt-webhook-contract-required-tracking-alert-home-arrival'
    );
    expect(receiptRequired.localOutboxStateRef).toBe('receipt-required');
    expect(receiptRequired.schedulerStateRef).toBe('receipt-required');
    expect(manualRequired.localOutboxStateRef).toBe('manual-required');
    expect(providerUnavailable.localOutboxStateRef).toBe('dead-lettered');
    expect(providerUnavailable.schedulerStateRef).toBe('dead-letter-review');
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('rejects missing artifact refs and claim upgrades', () => {
    const readModel = readinessReadModel();
    const row = readModel.rows[0];

    expect(
      TrackingNotificationLocalOutboxReadinessRowSchema.safeParse({
        ...row,
        schedulerArtifactRef: '',
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationLocalOutboxReadinessRowSchema.safeParse({
        ...row,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationLocalOutboxReadinessReadModelSchema.safeParse({
        ...readModel,
        productionDurableOutboxStorageClaimed: true,
      }).success
    ).toBe(false);
  });
});

function readinessReadModel() {
  return buildTrackingNotificationLocalOutboxReadinessReadModel(
    {
      generatedAt: Timestamp,
      proofId: 'tracking-notification-local-outbox-readiness-proof',
      sourceContractRefs: [
        'tracking-notification-receipt-boundary-proof',
        'notification-local-outbox-adapter-proof',
        'notification-local-outbox-scheduler-proof',
      ],
    },
    receiptProofReadModel()
  );
}

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingNotificationLocalOutboxReadinessReadModel>) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
  };
}

function receiptProofReadModel() {
  return buildTrackingNotificationReceiptBoundaryReadModel(
    {
      generatedAt: Timestamp,
      proofId: 'tracking-notification-receipt-boundary-proof',
      familyId: 'family-tracking-notification-local-outbox',
      sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
      sourceContractRefs: ['tracking-provider-notification-proof'],
    },
    providerProofReadModel()
  );
}

function providerProofReadModel() {
  return buildTrackingProviderNotificationProofReadModel(
    {
      generatedAt: Timestamp,
      proofId: 'tracking-provider-notification-proof',
      familyId: 'family-tracking-notification-local-outbox',
      sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
      sourceContractRefs: ['tracking-location-policy', 'notification-local-outbox-adapter-proof'],
    },
    sourceTrackingReadModel()
  );
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
