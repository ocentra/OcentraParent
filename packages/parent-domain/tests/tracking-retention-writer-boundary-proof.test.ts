import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionWriterBoundaryReadModelSchema,
  TrackingRetentionWriterBoundaryRequestSchema,
  TrackingRetentionWriterBoundaryRowSchema,
  buildTrackingRetentionWriterBoundaryReadModel,
} from '../src/tracking-retention-writer-boundary-proof';

const Timestamp = '2026-06-06T05:58:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-retention-writer-boundary-proof',
  familyId: 'family-tracking-retention-writer',
  childProfileId: 'child-profile-aarav',
  deviceId: 'device-aarav-phone',
  deviceLabel: 'Aarav phone',
  platform: 'android',
  sourceFeatureRefs: [
    'location-geofence-device-status',
    'tracking-plan-wp07-retention-and-custody-model',
    'tracking-plan-wp32-journal-sqlite-and-read-model-proof',
  ],
} as const;

describe('tracking retention writer boundary proof', () => {
  it('builds typed retention writer rows without claiming service mutation or delivery', () => {
    const readModel = buildTrackingRetentionWriterBoundaryReadModel(ProofOptions, retentionRequests());

    expect(readModel.rows.map((row) => row.request.settingKind)).toEqual([
      'retention-window',
      'delete-after-alert-resolved',
      'parent-export',
      'remote-sync',
      'remote-ai',
    ]);
    expect(readModel.rows.map((row) => row.state)).toEqual([
      'accepted-for-contract',
      'accepted-for-contract',
      'manual-service-mutation-required',
      'remote-sync-disabled',
      'remote-ai-disabled',
    ]);
    expect(readModel.acceptedForContractCount).toBe(2);
    expect(readModel.manualServiceMutationRequiredCount).toBe(1);
    expect(readModel.disabledRemoteRuntimeCount).toBe(2);
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('keeps proof, evidence, audit, validation, envelope, and read-model refs attached to every row', () => {
    const readModel = buildTrackingRetentionWriterBoundaryReadModel(ProofOptions, retentionRequests());

    expect(
      readModel.rows.map((row) => ({
        proofRefs: row.request.sourceProofRefs.length,
        evidenceRefs: row.request.evidenceRefs.length,
        auditRefs: row.request.auditRefs.length,
        validationRef: row.validationRef,
        mutationEnvelopeRef: row.mutationEnvelopeRef,
        readModelUpdateRef: row.readModelUpdateRef,
      }))
    ).toEqual([
      refSummary('retention-window'),
      refSummary('delete-after-alert-resolved'),
      refSummary('parent-export'),
      refSummary('remote-sync'),
      refSummary('remote-ai'),
    ]);
  });

  it('rejects remote runtime enablement and service mutation overclaims', () => {
    const readModel = buildTrackingRetentionWriterBoundaryReadModel(ProofOptions, retentionRequests());
    const remoteSync = readModel.rows[3];

    expect(
      TrackingRetentionWriterBoundaryRequestSchema.safeParse({
        ...remoteSync.request,
        remoteSyncEnabled: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionWriterBoundaryRowSchema.safeParse({
        ...remoteSync,
        serviceMutationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionWriterBoundaryReadModelSchema.safeParse({
        ...readModel,
        productReadyClaimed: true,
      }).success
    ).toBe(false);
  });
});

function refSummary(kind: string) {
  return {
    proofRefs: 3,
    evidenceRefs: 2,
    auditRefs: 1,
    validationRef: `tracking-retention-writer-validation-tracking-retention-writer-${kind}`,
    mutationEnvelopeRef: `tracking-retention-writer-envelope-tracking-retention-writer-${kind}`,
    readModelUpdateRef: `tracking-retention-writer-read-model-update-tracking-retention-writer-${kind}`,
  };
}

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingRetentionWriterBoundaryReadModel>) {
  return {
    serviceMutationClaimed: readModel.serviceMutationClaimed,
    platformRetentionWriterClaimed: readModel.platformRetentionWriterClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    notificationReceiptClaimed: readModel.notificationReceiptClaimed,
    remoteSyncRuntimeClaimed: readModel.remoteSyncRuntimeClaimed,
    remoteAiRuntimeClaimed: readModel.remoteAiRuntimeClaimed,
    portalSettingsUiClaimed: readModel.portalSettingsUiClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    productReadyClaimed: readModel.productReadyClaimed,
  };
}

function retentionRequests() {
  return [
    retentionRequest('retention-window', 'tracking-retention-value-7d'),
    retentionRequest('delete-after-alert-resolved', 'tracking-retention-value-delete-after-alert'),
    retentionRequest('parent-export', 'tracking-retention-value-parent-export'),
    retentionRequest('remote-sync', 'tracking-retention-value-remote-sync-disabled'),
    retentionRequest('remote-ai', 'tracking-retention-value-remote-ai-disabled'),
  ] as const;
}

function retentionRequest(settingKind: string, requestedValueRef: string) {
  return {
    requestId: `tracking-retention-writer-${settingKind}`,
    settingKind,
    requestedValueRef,
    parentActionRef: `tracking-retention-parent-action-${settingKind}`,
    sourceProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
    ],
    evidenceRefs: [`tracking-retention-evidence-${settingKind}`, `tracking-retention-read-model-${settingKind}`],
    auditRefs: [`tracking-retention-audit-${settingKind}`],
    requestedAt: Timestamp,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
  };
}
