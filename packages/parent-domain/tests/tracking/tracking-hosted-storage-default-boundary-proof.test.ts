import { describe, expect, it } from 'vitest';
import {
  TrackingHostedStorageDefaultRowSchema,
  buildTrackingHostedStorageDefaultBoundaryProof,
  type TrackingHostedStorageDefaultKind,
  type TrackingHostedStorageDefaultProof,
  type TrackingHostedStorageDefaultRow,
} from '../../src/tracking-hosted-storage-default-boundary-proof';

const GeneratedAt = '2026-06-07T06:12:00.000Z';

describe('tracking hosted storage default boundary proof', () => {
  it('builds local-first storage boundary rows from tracking journal and read-model refs', () => {
    const proof = buildTrackingHostedStorageDefaultBoundaryProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-hosted-storage-default-boundary-proof');
    expect(proof.rows.map((row) => row.boundaryKind)).toEqual([
      'journal-local-default',
      'sqlite-read-model-local-default',
      'parent-export-local-default',
      'ai-context-stored-ref-local-default',
      'remote-sync-disabled-default',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.ocentraHostedStorageDefault).toBe(false);
    for (const row of proof.rows) {
      expectLocalStorageBoundaryRow(row);
    }
  });

  it('keeps parent export and AI context consumers tied to stronger stored-ref custody gates', () => {
    const proof = buildTrackingHostedStorageDefaultBoundaryProof(GeneratedAt);

    expect(rowFor(proof, 'parent-export-local-default').defaultCustody).toBe('parent-owned-export');
    expect(rowFor(proof, 'parent-export-local-default').parentOwnedExportRequired).toBe(true);
    expect(rowFor(proof, 'ai-context-stored-ref-local-default').storedRefConsumerRequired).toBe(true);
    expect(rowFor(proof, 'ai-context-stored-ref-local-default').aiConsumerProofRefs).toContain(
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json'
    );
    expect(rowFor(proof, 'remote-sync-disabled-default').defaultCustody).toBe('remote-disabled');
  });

  it('rejects rows that omit proof refs, evidence refs, or the required local custody boundary', () => {
    const proof = buildTrackingHostedStorageDefaultBoundaryProof(GeneratedAt);
    const journalRow = rowFor(proof, 'journal-local-default');

    expect(TrackingHostedStorageDefaultRowSchema.safeParse({ ...journalRow, sourceProofRefs: [] }).success).toBe(false);
    expect(TrackingHostedStorageDefaultRowSchema.safeParse({ ...journalRow, journalProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingHostedStorageDefaultRowSchema.safeParse({ ...journalRow, readModelProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingHostedStorageDefaultRowSchema.safeParse({ ...journalRow, evidenceReferences: [] }).success).toBe(
      false
    );
  });

  it('rejects export, AI, and remote-sync rows that weaken the non-hosted default claim', () => {
    const proof = buildTrackingHostedStorageDefaultBoundaryProof(GeneratedAt);

    expect(
      TrackingHostedStorageDefaultRowSchema.safeParse({
        ...rowFor(proof, 'parent-export-local-default'),
        parentOwnedExportRequired: false,
      }).success
    ).toBe(false);
    expect(
      TrackingHostedStorageDefaultRowSchema.safeParse({
        ...rowFor(proof, 'ai-context-stored-ref-local-default'),
        aiConsumerProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingHostedStorageDefaultRowSchema.safeParse({
        ...rowFor(proof, 'remote-sync-disabled-default'),
        defaultCustody: 'parent-device-local',
      }).success
    ).toBe(false);
  });
});

function expectLocalStorageBoundaryRow(row: TrackingHostedStorageDefaultRow): void {
  expect(row.boundaryState).toBe('hosted-storage-not-default');
  expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
  expect(row.currentProofTier).toBe('P2_HOSTED_CI');
  expect(row.sourceProofRefs.length).toBeGreaterThan(0);
  expect(row.journalProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/10-journal-sqlite-proof.json'
  );
  expect(row.readModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
  );
  expect(row.evidenceReferences.length).toBeGreaterThan(0);
  expect(row.ocentraHostedStorageDefault).toBe(false);
  expect(row.rawLocationRemoteUploadEnabled).toBe(false);
  expect(row.sqliteSnapshotRemoteUploadEnabled).toBe(false);
  expect(row.remoteSyncEnabled).toBe(false);
  expect(row.remoteAiEnabled).toBe(false);
  expectNoProductClaims(row);
}

function expectNoProductClaims(row: TrackingHostedStorageDefaultRow): void {
  expect(row.hostedStorageBoundaryClaimed).toBe(true);
  expect(row.portalUiClaimed).toBe(false);
  expect(row.serviceMutationClaimed).toBe(false);
  expect(row.platformRuntimeClaimed).toBe(false);
  expect(row.childDeviceDeliveryClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
  expect(row.productionBehaviorClaimed).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function rowFor(
  proof: TrackingHostedStorageDefaultProof,
  boundaryKind: TrackingHostedStorageDefaultKind
): TrackingHostedStorageDefaultRow {
  const row = proof.rows.find((entry) => entry.boundaryKind === boundaryKind);
  if (row === undefined) {
    throw new Error(`Missing tracking hosted storage default row: ${boundaryKind}`);
  }
  return row;
}
