import { describe, expect, it } from 'vitest';
import {
  TrackingNotificationParentSurfaceHistoryReadModelSchema,
  TrackingNotificationParentSurfaceHistoryStatus,
  buildTrackingNotificationParentSurfaceHistoryReadModel,
} from '../../src/tracking-notification-parent-surface-history-proof';
import { buildTrackingNotificationPreferencePreflightReadModel } from '../../src/tracking-notification-preference-preflight-proof';
import { buildTrackingNotificationReceiptBoundaryReadModel } from '../../src/tracking-notification-receipt-boundary-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';

const Timestamp = '2026-06-06T16:16:00.000Z';
const FamilyId = 'family-tracking-notification-history';
const ProviderOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-provider-notification-proof-for-parent-surface-history',
  familyId: FamilyId,
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-parent-surface-history',
  sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
} as const;
const ReceiptOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-notification-receipt-boundary-proof-for-parent-surface-history',
  familyId: FamilyId,
  sourceProviderNotificationProofRef: ProviderOptions.proofId,
  sourceContractRefs: ['tracking-provider-notification-proof', 'notification-receipt-boundary'],
} as const;
const PreferenceOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'tracking-notification-preference-preflight-proof-for-parent-surface-history',
  sourceContractRefs: [
    'tracking-provider-notification-proof',
    'v3-notification-rule-provider-retry-contract',
    'notification-parent-preference-boundary',
    'notification-quiet-hours-policy-boundary',
  ],
} as const;
const HistoryOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-notification-parent-surface-history-proof',
  sourceContractRefs: [
    'tracking-provider-notification-proof',
    'tracking-notification-receipt-boundary-proof',
    'tracking-notification-preference-preflight-proof',
    'notifications-expectations',
    'location-geofence-device-status',
  ],
} as const;

describe('tracking notification parent-surface history proof', () => {
  it('joins provider, receipt, and preference rows into parent history intent rows', () => {
    const readModel = historyReadModel();

    expect(readModel.historyIntentReadyCount).toBe(1);
    expect(readModel.manualActionRequiredCount).toBe(1);
    expect(readModel.providerUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady,
      TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired,
      TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable,
    ]);
  });

  it('preserves evidence, policy, audit, receipt, preference, and quiet-hours refs', () => {
    const [readyRow, manualRow, unavailableRow] = historyReadModel().rows;

    expect(readyRow.sourceProviderNotificationRowId).toBe('tracking-provider-notification-tracking-alert-home-arrival');
    expect(readyRow.sourceReceiptBoundaryRowId).toBe('tracking-notification-receipt-tracking-alert-home-arrival');
    expect(readyRow.sourcePreferencePreflightRowId).toBe(
      'tracking-notification-preference-preflight-tracking-alert-home-arrival'
    );
    expect(readyRow.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(readyRow.auditRefs).toEqual(['tracking-provider-notification-audit-tracking-alert-home-arrival']);
    expect(readyRow.receiptRequirementRefs).toContain(
      'tracking-receipt-webhook-contract-required-tracking-alert-home-arrival'
    );
    expect(readyRow.parentPreferenceRequirementRefs).toContain(
      'tracking-parent-notification-preference-required-tracking-alert-home-arrival'
    );
    expect(readyRow.quietHoursRequirementRefs).toContain(
      'tracking-quiet-hours-policy-required-tracking-alert-home-arrival'
    );
    expect(manualRow.manualProofRequirements).toContain(
      'tracking-provider-critical-escalation-review-tracking-alert-left-expected-place'
    );
    expect(unavailableRow.manualProofRequirements).toContain(
      'tracking-receipt-provider-unavailable-tracking-alert-provider-unavailable'
    );
  });

  it('rejects UI, mutation, delivery, receipt runtime, device, and authority overclaims', () => {
    const readModel = historyReadModel();

    expect(readModel.renderedParentNotificationUiClaimed).toBe(false);
    expect(readModel.parentPreferenceMutationRuntimeClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionRuntimeClaimed).toBe(false);
    expect(readModel.childDeviceDeliveryClaimed).toBe(false);
    expect(readModel.mobilePhysicalDeviceProofClaimed).toBe(false);
    expect(readModel.authorityProofClaimed).toBe(false);
    expect(
      TrackingNotificationParentSurfaceHistoryReadModelSchema.safeParse({
        ...readModel,
        renderedParentNotificationUiClaimed: true,
      }).success
    ).toBe(false);
  });
});

function historyReadModel() {
  const provider = providerReadModel();
  return buildTrackingNotificationParentSurfaceHistoryReadModel(
    HistoryOptions,
    provider,
    buildTrackingNotificationReceiptBoundaryReadModel(ReceiptOptions, provider),
    buildTrackingNotificationPreferencePreflightReadModel(PreferenceOptions, provider)
  );
}

function providerReadModel() {
  return buildTrackingProviderNotificationProofReadModel(ProviderOptions, sourceTrackingReadModel());
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
    evidenceReferences: [
      {
        evidenceReferenceId: 'location-evidence-geofence-entry',
        kind: 'journal-event',
        observedAt: '2026-06-06T16:15:00.000Z',
      },
    ],
    acknowledgementId: null,
    ...input,
  };
}
