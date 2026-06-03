import { describe, expect, it } from 'vitest';
import {
  ParentOwnedLocalExportRuntimeDeleteReceiptSchema,
  ParentOwnedLocalExportRuntimeJobSchema,
  ParentOwnedLocalExportRuntimeKnownGaps,
  ParentOwnedLocalExportRuntimeOutputSchema,
  ParentOwnedLocalExportRuntimeProofReadModel,
  ParentOwnedLocalExportRuntimeProofSchema,
  ParentOwnedLocalExportRuntimeScopeSchema,
  summarizeParentOwnedLocalExportRuntimeDataClasses,
  summarizeParentOwnedLocalExportRuntimeStates,
} from '../src/parent-owned-local-export-runtime';

describe('parent-owned local export delete runtime proof contracts', () => {
  acceptsTheLocalExportDeleteRuntimeProof();
  rejectsCloudProviderPortalAndCustodyOverclaims();
  rejectsUnsafeExportScopeAndOutputRows();
  rejectsUnsafeDeleteAndRuntimeStateRows();
});

function acceptsTheLocalExportDeleteRuntimeProof(): void {
  it('covers local export queue write delete failure offline and manual states', () => {
    const proof = ParentOwnedLocalExportRuntimeProofSchema.parse(ParentOwnedLocalExportRuntimeProofReadModel);

    expect(summarizeParentOwnedLocalExportRuntimeStates(proof.jobs)).toEqual({
      'export-queued': 1,
      'export-running': 1,
      'export-written': 1,
      'delete-requested': 1,
      'delete-confirmed': 1,
      'delete-failed': 1,
      'offline-queued': 1,
      'manual-required': 1,
    });
    expect(summarizeParentOwnedLocalExportRuntimeDataClasses(proof.jobs)).toEqual({
      'encrypted-journal-segment': 8,
      'sqlite-query-row': 8,
      'parent-rule': 0,
      'approval-decision': 0,
      'device-registry-entry': 0,
      'notification-history': 0,
      'audit-event': 0,
      'generated-summary': 8,
    });
    expect(ParentOwnedLocalExportRuntimeKnownGaps).toContain(
      'Real filesystem writer, retention scheduler, delete executor, and durable audit persistence remain future work.'
    );
  });
}

function rejectsCloudProviderPortalAndCustodyOverclaims(): void {
  it('rejects runtime proofs that claim cloud provider portal Ocentra custody or child mutation', () => {
    const proof = ParentOwnedLocalExportRuntimeProofReadModel;

    for (const invalidProof of [
      { ...proof, cloudTransferRuntimeClaimed: true },
      { ...proof, connectorOAuthClaimed: true },
      { ...proof, providerApiClaimed: true },
      { ...proof, portalUiClaimed: true },
      { ...proof, ocentraHostedFamilyDataCustodyClaimed: true },
      { ...proof, remoteReportCompilerClaimed: true },
      { ...proof, childDeviceMutationClaimed: true },
      { ...proof, rawEvidenceUploadClaimed: true },
      { ...proof, nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-ocentra-family-data-custody') },
    ]) {
      expect(ParentOwnedLocalExportRuntimeProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsUnsafeExportScopeAndOutputRows(): void {
  it('rejects unparented unsafe custody export scope and output rows', () => {
    const exportWritten = jobFor('export-written');
    const scope = exportWritten.scope;
    const output = exportWritten.output;
    if (output === null) {
      throw new Error('missing export output');
    }

    for (const invalidScope of [
      { ...scope, parentAuthorized: false },
      { ...scope, rawEvidenceUploaded: true },
      { ...scope, ocentraHostedFamilyDataStored: true },
      { ...scope, destinationOwnership: 'ocentra-hosted-non-activity-metadata' },
      { ...scope, requestedDataClasses: [] },
    ]) {
      expect(ParentOwnedLocalExportRuntimeScopeSchema.safeParse(invalidScope).success).toBe(false);
    }

    for (const invalidOutput of [
      { ...output, destinationOwnership: 'ocentra-hosted-non-activity-metadata' },
      { ...output, rawEvidenceIncludedByDefault: true },
      { ...output, ocentraHostedCopyRetained: true },
      { ...output, encryptedAtRest: false },
      { ...output, sourceEvidenceRefs: [] },
    ]) {
      expect(ParentOwnedLocalExportRuntimeOutputSchema.safeParse(invalidOutput).success).toBe(false);
    }
  });
}

function rejectsUnsafeDeleteAndRuntimeStateRows(): void {
  it('rejects delete receipts and runtime rows that mutate local evidence or omit required refs', () => {
    const deleteConfirmed = jobFor('delete-confirmed');
    const deleteFailed = jobFor('delete-failed');
    if (deleteConfirmed.deleteReceipt === null || deleteFailed.deleteReceipt === null) {
      throw new Error('missing delete receipt');
    }

    expect(
      ParentOwnedLocalExportRuntimeDeleteReceiptSchema.safeParse({
        ...deleteConfirmed.deleteReceipt,
        deletedAt: null,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedLocalExportRuntimeDeleteReceiptSchema.safeParse({
        ...deleteConfirmed.deleteReceipt,
        sourceEvidenceRetained: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedLocalExportRuntimeDeleteReceiptSchema.safeParse({
        ...deleteFailed.deleteReceipt,
        failureReasonRef: null,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedLocalExportRuntimeJobSchema.safeParse({
        ...deleteFailed,
        localEvidenceMutated: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedLocalExportRuntimeJobSchema.safeParse({
        ...jobFor('export-written'),
        output: null,
      }).success
    ).toBe(false);
  });
}

function jobFor(state: 'export-written' | 'delete-confirmed' | 'delete-failed') {
  const job = ParentOwnedLocalExportRuntimeProofReadModel.jobs.find((candidate) => candidate.state === state);
  if (job === undefined) {
    throw new Error(`missing local export runtime job: ${state}`);
  }
  return job;
}
