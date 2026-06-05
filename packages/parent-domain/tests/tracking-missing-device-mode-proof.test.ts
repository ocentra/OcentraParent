import { describe, expect, it } from 'vitest';
import {
  TrackingMissingDeviceModeProofReadModelSchema,
  TrackingMissingDeviceModeProofRowSchema,
  buildTrackingMissingDeviceModeProofReadModel,
} from '../src/tracking-missing-device-mode-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

const Timestamp = '2026-06-05T15:18:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-missing-device-mode-proof',
  familyId: 'family-tracking-missing-device',
  childProfileId: 'child-profile-aarav',
  deviceId: 'device-aarav-phone',
  deviceLabel: 'Aarav phone',
  platform: 'android',
  sourceTrackingReadModelRef: 'tracking-location-policy-read-model-missing-device',
  sourceContractRefs: [
    'tracking-location-policy',
    'device-location-tracking-capability-guide',
    'tracking-control-settings-inventory',
    'tracking-ui-ux-requirements-guide',
    'location-geofence-device-status',
  ],
} as const;

describe('tracking missing-device mode proof', () => {
  it('builds last-known and offline missing-device rows without current-location claims', () => {
    const readModel = buildTrackingMissingDeviceModeProofReadModel(ProofOptions, sourceTrackingReadModel());

    expect(readModel.rows.map((row) => row.state)).toEqual([
      'last-known-only',
      'offline',
      'contact-requested',
      'manual-required',
    ]);
    expect(readModel.lastKnownOnlyCount).toBe(1);
    expect(readModel.offlineCount).toBe(1);
    expect(readModel.contactRequestedCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.runtimeEvidenceRefs).toHaveLength(16);
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('keeps contact, battery, connectivity, pending upload, action, and accessible UI refs prominent', () => {
    const readModel = buildTrackingMissingDeviceModeProofReadModel(ProofOptions, sourceTrackingReadModel());
    const offline = readModel.rows[1];

    expect(offline.lastKnownEvidenceRef).toBe('location-evidence-last-known-powered-off');
    expect(offline.deviceStatusEvidenceRef).toBe('device-status-powered-off');
    expect(offline.statusSnapshot.contactState).toBe('powered-off');
    expect(offline.statusSnapshot.batteryPercent).toBe(9);
    expect(offline.statusSnapshot.pendingUploadCount).toBe(1);
    expect(offline.uiState.primaryBadge).toBe('offline');
    expect(offline.uiState.secondaryBadges).toEqual(['last-known', 'battery-throttled']);
    expect(offline.uiState.evidenceDrawerRefs).toEqual([
      'location-evidence-last-known-powered-off',
      'device-status-powered-off',
      'tracking-device-battery-tracking-missing-device-powered-off',
      'device-status-powered-off',
      'tracking-device-pending-upload-tracking-missing-device-powered-off',
    ]);
    expect(offline.uiState.actionKinds).toEqual([
      'review-last-known',
      'ask-child-check-in',
      'call-child',
      'mark-found',
    ]);
    expect(offline.uiState.currentLocationCopyAllowed).toBe(false);
  });

  it('rejects rows and read models that overclaim powered-off tracking or live current location', () => {
    const readModel = buildTrackingMissingDeviceModeProofReadModel(ProofOptions, sourceTrackingReadModel());
    const offline = readModel.rows[1];

    expect(
      TrackingMissingDeviceModeProofRowSchema.safeParse({
        ...offline,
        currentLocationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingMissingDeviceModeProofRowSchema.safeParse({
        ...offline,
        poweredOffTrackingClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingMissingDeviceModeProofReadModelSchema.safeParse({
        ...readModel,
        liveTrackingRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingMissingDeviceModeProofReadModel>) {
  return {
    currentLocationRuntimeClaimed: readModel.currentLocationRuntimeClaimed,
    liveTrackingRuntimeClaimed: readModel.liveTrackingRuntimeClaimed,
    poweredOffDeviceTrackingClaimed: readModel.poweredOffDeviceTrackingClaimed,
    remoteSyncRuntimeClaimed: readModel.remoteSyncRuntimeClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    portalRuntimeUiClaimed: readModel.portalRuntimeUiClaimed,
    osLostModeApiClaimed: readModel.osLostModeApiClaimed,
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
    alerts: [],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [
      missingCase({
        caseId: 'tracking-missing-device-last-known',
        state: 'last-known-only',
        lastKnownEvidenceId: 'location-evidence-last-known-stale',
        deviceStatusEvidenceId: 'device-status-offline-last-known',
        reasonCodes: ['missing-device-last-known-only'],
      }),
      missingCase({
        caseId: 'tracking-missing-device-powered-off',
        state: 'offline',
        lastKnownEvidenceId: 'location-evidence-last-known-powered-off',
        deviceStatusEvidenceId: 'device-status-powered-off',
        reasonCodes: ['missing-device-powered-off-last-known-only'],
      }),
      missingCase({
        caseId: 'tracking-missing-device-contact-requested',
        state: 'contact-requested',
        lastKnownEvidenceId: 'location-evidence-last-known-contact-requested',
        deviceStatusEvidenceId: 'device-status-contact-action-queued',
        reasonCodes: ['missing-device-contact-action-queued'],
      }),
      missingCase({
        caseId: 'tracking-missing-device-manual-required',
        state: 'manual-required',
        lastKnownEvidenceId: 'location-evidence-last-known-manual-required',
        deviceStatusEvidenceId: 'device-status-platform-proof-required',
        reasonCodes: ['missing-device-platform-proof-required'],
      }),
    ],
    platformProofRoutes: [],
  });
}

function missingCase(input: {
  readonly caseId: string;
  readonly state: 'last-known-only' | 'offline' | 'contact-requested' | 'manual-required';
  readonly lastKnownEvidenceId: string;
  readonly deviceStatusEvidenceId: string;
  readonly reasonCodes: readonly string[];
}) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    caseId: input.caseId,
    openedAt: Timestamp,
    state: input.state,
    lastKnownEvidence: {
      evidenceReferenceId: input.lastKnownEvidenceId,
      kind: 'journal-event',
      observedAt: '2026-06-05T15:12:00.000Z',
    },
    deviceStatusEvidence: {
      evidenceReferenceId: input.deviceStatusEvidenceId,
      kind: 'query-store-summary',
      observedAt: '2026-06-05T15:13:00.000Z',
    },
    contactActionRefs: [`tracking-contact-action-${input.caseId}`],
    reasonCodes: input.reasonCodes,
  };
}
