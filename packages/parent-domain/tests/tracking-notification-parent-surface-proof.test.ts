import { describe, expect, it } from 'vitest';
import {
  TrackingNotificationParentSurfaceReadModelSchema,
  TrackingNotificationParentSurfaceRowSchema,
  buildTrackingNotificationParentSurfaceReadModel,
} from '../src/tracking-notification-parent-surface-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

const Timestamp = '2026-06-06T07:22:00.000Z';
const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-06T07:20:00.000Z',
} as const;

const ProviderProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-provider-notification-proof',
  familyId: 'family-tracking-notification-parent-surface',
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
  sourceContractRefs: [
    'tracking-location-policy',
    'v0-8-notification-provider-status-boundary',
    'notification-local-outbox-adapter-proof',
    'location-geofence-device-status',
  ],
} as const;

const ParentSurfaceProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-notification-parent-surface-proof',
  sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
  sourceContractRefs: [
    'tracking-provider-notification-proof',
    'v0-8-notification-provider-status-boundary',
    'notifications-expectations-parent-surface',
    'location-geofence-device-status',
  ],
} as const;

describe('tracking notification parent surface proof', () => {
  it('derives parent history, manual-action, and unavailable rows from provider notification proof', () => {
    const readModel = buildTrackingNotificationParentSurfaceReadModel(
      ParentSurfaceProofOptions,
      sourceProviderProofReadModel()
    );

    expect(readModel.rows.map((row) => row.parentSurfaceStatus)).toEqual([
      'history-row-ready',
      'manual-action-required',
      'unavailable-visible',
    ]);
    expect(readModel.rows.map((row) => row.historyVisibility)).toEqual([
      'status-history-ready',
      'manual-review-only',
      'unavailable-row-visible',
    ]);
    expect(readModel.rows.map((row) => row.preferenceVisibility)).toEqual([
      'preference-setup-required',
      'preference-setup-required',
      'preference-unavailable-visible',
    ]);
    expect(readModel.historyRowReadyCount).toBe(1);
    expect(readModel.manualActionRequiredCount).toBe(1);
    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(readModel.preferenceSetupRequiredCount).toBe(2);
  });
});

describe('tracking notification parent surface proof references', () => {
  it('preserves alert evidence, policy, status, reason, drill-in, audit, and preference refs', () => {
    const readModel = buildTrackingNotificationParentSurfaceReadModel(
      ParentSurfaceProofOptions,
      sourceProviderProofReadModel()
    );
    const historyReady = readModel.rows[0];
    const manualRequired = readModel.rows[1];
    const unavailable = readModel.rows[2];

    expect(historyReady.sourceProviderProofRowRef).toBe('tracking-provider-notification-tracking-alert-home-arrival');
    expect(historyReady.sourcePolicyDecisionId).toBe('tracking-decision-home-arrival');
    expect(historyReady.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(historyReady.sourceNotificationStatusRefs).toEqual(['tracking-notification-intent-home-arrival']);
    expect(historyReady.reasonCodeRefs).toEqual(['home-arrival-notification']);
    expect(historyReady.drillInRefs).toEqual([
      'tracking-alert-home-arrival',
      'tracking-decision-home-arrival',
      'tracking-notification-intent-home-arrival',
    ]);
    expect(historyReady.auditRefs).toEqual(['tracking-provider-notification-audit-tracking-alert-home-arrival']);
    expect(manualRequired.minimalSurfacePayloadBoundary).toContain('authenticated drill-in refs only');
    expect(unavailable.parentVisibleNotificationStatusRef).toBe(
      'tracking-provider-status-unavailable-tracking-alert-provider-unavailable'
    );
  });
});

describe('tracking notification parent surface proof claim boundaries', () => {
  it('rejects missing refs and UI, mutation, delivery, receipt, authority, or device overclaims', () => {
    const readModel = buildTrackingNotificationParentSurfaceReadModel(
      ParentSurfaceProofOptions,
      sourceProviderProofReadModel()
    );
    const row = readModel.rows[0];

    expect(
      TrackingNotificationParentSurfaceRowSchema.safeParse({
        ...row,
        drillInRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationParentSurfaceRowSchema.safeParse({
        ...row,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        parentNotificationUiRendered: true,
      }).success
    ).toBe(false);
  });
});

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
