import { describe, expect, it } from 'vitest';
import {
  TrackingNotificationReceiptBoundaryReadModelSchema,
  TrackingNotificationReceiptBoundaryRowSchema,
  buildTrackingNotificationReceiptBoundaryReadModel,
} from '../src/tracking-notification-receipt-boundary-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

const Timestamp = '2026-06-06T07:04:00.000Z';
const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-06T07:00:00.000Z',
} as const;

const ProviderProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-provider-notification-proof',
  familyId: 'family-tracking-notification-receipt',
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
  sourceContractRefs: [
    'tracking-location-policy',
    'v0-8-notification-provider-status-boundary',
    'notification-local-outbox-adapter-proof',
    'location-geofence-device-status',
  ],
} as const;

const ReceiptProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-notification-receipt-boundary-proof',
  familyId: 'family-tracking-notification-receipt',
  sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
  sourceContractRefs: [
    'tracking-provider-notification-proof',
    'v0-8-notification-provider-status-boundary',
    'notifications-expectations',
    'location-geofence-device-status',
  ],
} as const;

describe('tracking notification receipt boundary proof', () => {
  it('derives receipt-ingestion boundary rows from provider notification proof rows', () => {
    const readModel = buildTrackingNotificationReceiptBoundaryReadModel(
      ReceiptProofOptions,
      sourceProviderProofReadModel()
    );

    expect(readModel.rows.map((row) => row.receiptBoundaryState)).toEqual([
      'receipt-ingestion-required',
      'manual-receipt-required',
      'provider-unavailable',
    ]);
    expect(readModel.receiptIngestionRequiredCount).toBe(1);
    expect(readModel.manualReceiptRequiredCount).toBe(1);
    expect(readModel.providerUnavailableCount).toBe(1);
    expect(readModel.providerReceiptRequiredCoverageRef).toBe('notification-provider-delivered-receipt-required');
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('preserves provider proof, evidence, policy, notification status, reason, and audit refs', () => {
    const readModel = buildTrackingNotificationReceiptBoundaryReadModel(
      ReceiptProofOptions,
      sourceProviderProofReadModel()
    );
    const receiptRequired = readModel.rows[0];
    const manualRequired = readModel.rows[1];
    const unavailable = readModel.rows[2];

    expect(receiptRequired.sourceProviderProofRowRef).toBe(
      'tracking-provider-notification-tracking-alert-home-arrival'
    );
    expect(receiptRequired.sourcePolicyDecisionId).toBe('tracking-decision-home-arrival');
    expect(receiptRequired.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(receiptRequired.notificationStatusRefs).toEqual(['tracking-notification-intent-home-arrival']);
    expect(receiptRequired.reasonCodeRefs).toEqual(['home-arrival-notification']);
    expect(receiptRequired.auditRefs).toEqual(['tracking-provider-notification-audit-tracking-alert-home-arrival']);
    expect(receiptRequired.receiptIngestionProofRequirements).toContain(
      'tracking-receipt-webhook-contract-required-tracking-alert-home-arrival'
    );
    expect(manualRequired.receiptIngestionProofRequirements).toContain(
      'tracking-receipt-critical-escalation-review-tracking-alert-left-expected-place'
    );
    expect(unavailable.receiptIngestionProofRequirements).toEqual([
      'tracking-receipt-provider-unavailable-tracking-alert-provider-unavailable',
    ]);
  });

  it('rejects missing trace refs and receipt/delivery claim upgrades', () => {
    const readModel = buildTrackingNotificationReceiptBoundaryReadModel(
      ReceiptProofOptions,
      sourceProviderProofReadModel()
    );
    const row = readModel.rows[0];

    expect(
      TrackingNotificationReceiptBoundaryRowSchema.safeParse({
        ...row,
        evidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationReceiptBoundaryRowSchema.safeParse({
        ...row,
        providerReceiptRefs: ['provider-receipt-observed'],
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationReceiptBoundaryReadModelSchema.safeParse({
        ...readModel,
        webhookReceiptIngestionRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingNotificationReceiptBoundaryReadModel>) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    webhookReceiptIngestionRuntimeClaimed: readModel.webhookReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
  };
}

function sourceProviderProofReadModel() {
  return buildTrackingProviderNotificationProofReadModel(ProviderProofOptions, sourceTrackingReadModel());
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
