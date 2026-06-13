import { describe, expect, it } from 'vitest';
import {
  TrackingNotificationPreferencePreflightReadModelSchema,
  TrackingNotificationPreferencePreflightStatus,
  buildTrackingNotificationPreferencePreflightReadModel,
} from '../../src/tracking-notification-preference-preflight-proof';
import { buildTrackingProviderNotificationProofReadModel } from '../../src/tracking-provider-notification-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';

const Timestamp = '2026-06-06T08:02:00.000Z';
const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-geofence-entry',
  kind: 'journal-event',
  observedAt: '2026-06-06T08:00:00.000Z',
} as const;
const ProviderOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-provider-notification-proof-for-preference-preflight',
  familyId: 'family-tracking-preference-preflight',
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-preference-preflight',
  sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
} as const;
const PreferenceOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'tracking-notification-preference-preflight-proof',
  sourceContractRefs: [
    'tracking-provider-notification-proof',
    'v3-notification-rule-provider-retry-contract',
    'notification-parent-preference-boundary',
    'notification-quiet-hours-policy-boundary',
  ],
} as const;

describe('tracking notification preference preflight proof', () => {
  it('derives parent preference required rows from provider-adapter-required tracking notifications', () => {
    const readModel = buildTrackingNotificationPreferencePreflightReadModel(PreferenceOptions, providerReadModel());

    expect(readModel.parentPreferenceRequiredCount).toBe(1);
    expect(readModel.sourceManualRequiredCount).toBe(1);
    expect(readModel.sourceUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired,
      TrackingNotificationPreferencePreflightStatus.SourceManualRequired,
      TrackingNotificationPreferencePreflightStatus.SourceUnavailable,
    ]);
    expect(readModel.rows[0].parentPreferenceState).toBe('manual-setup-required');
    expect(readModel.rows[0].quietHoursDecision).toBe('manual-required');
  });

  it('preserves source evidence, policy, reason, provider attempt, and preference refs', () => {
    const [preferenceRow, manualRow, unavailableRow] = buildTrackingNotificationPreferencePreflightReadModel(
      PreferenceOptions,
      providerReadModel()
    ).rows;

    expect(preferenceRow.sourcePolicyDecisionId).toBe('tracking-decision-home-arrival');
    expect(preferenceRow.evidenceRefs).toEqual(['location-evidence-geofence-entry']);
    expect(preferenceRow.providerAttemptRef).toBe('tracking-provider-attempt-not-started-tracking-alert-home-arrival');
    expect(preferenceRow.providerPreferenceRefs).toEqual([
      'tracking-provider-notification-preference-tracking-alert-home-arrival',
    ]);
    expect(manualRow.parentPreferenceState).toBeNull();
    expect(manualRow.manualProofRequirements).toContain(
      'tracking-provider-critical-escalation-review-tracking-alert-left-expected-place'
    );
    expect(unavailableRow.quietHoursRequirementRefs).toContain(
      'tracking-provider-unavailable-tracking-alert-provider-unavailable'
    );
  });

  it('rejects parent preference UI and delivery overclaims', () => {
    const readModel = buildTrackingNotificationPreferencePreflightReadModel(PreferenceOptions, providerReadModel());

    expect(readModel.parentNotificationPreferenceUiClaimed).toBe(false);
    expect(readModel.parentNotificationHistoryUiClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(
      TrackingNotificationPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        parentNotificationPreferenceUiClaimed: true,
      }).success
    ).toBe(false);
  });
});

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
    evidenceReferences: [EvidenceTrace],
    acknowledgementId: null,
    ...input,
  };
}
