import { describe, expect, it } from 'vitest';

import {
  type DeleteExecutorRow,
  DeleteExecutorReadModelSchema,
  DeleteExecutorRequiredDataClasses,
  DeleteExecutorRowSchema,
  summarizeDeleteExecutorStatuses,
  summarizeDeleteExecutorTargets,
} from '@ocentra-parent/schema-domain/delete-executor-proof';
import { DeleteExecutorReadModel } from '@ocentra-parent/schema-domain/delete-executor-read-model';

describe('delete executor logging proof contract', () => {
  it('covers delete executor targets and manual runtime statuses without execution claims', () => {
    const readModel = DeleteExecutorReadModelSchema.parse(DeleteExecutorReadModel);

    expect(readModel.readModelId).toBe('production-support-delete-executor-proof');
    expect(summarizeDeleteExecutorTargets(readModel.rows)).toEqual({
      'local-export-output': 2,
      'support-backend-payload': 1,
      'status-backend-payload': 1,
      'public-runtime-payload': 1,
      'legal-disclosure-payload': 1,
    });
    expect(summarizeDeleteExecutorStatuses(readModel.rows)).toEqual({
      'source-contract-ready': 0,
      'delete-request-recorded': 1,
      'executor-manual-required': 2,
      'executor-unavailable': 1,
      'blocked-before-runtime': 2,
    });

    for (const row of readModel.rows) {
      expectRowEvidenceRefs(row);
      expectRowHasNoExecutionClaims(row);
    }
  });

  it('rejects execution overclaims and missing custody proof refs', () => {
    const localRequest = rowFor('local-output-delete-request-recorded');

    for (const invalidRow of [
      { ...localRequest, rowId: 'invalid-real-delete', realDeleteExecuted: true },
      { ...localRequest, rowId: 'invalid-durable-queue', durableQueueExecuted: true },
      { ...localRequest, rowId: 'invalid-payload-delete', payloadDeletionExecuted: true },
      { ...localRequest, rowId: 'invalid-provider-execution', providerExecutionOccurred: true },
      { ...localRequest, rowId: 'invalid-public-runtime', publicRuntimeExecuted: true },
      { ...localRequest, rowId: 'invalid-legal-execution', legalExecutionOccurred: true },
      { ...localRequest, rowId: 'invalid-backend-upload', backendUploadExecuted: true },
      { ...localRequest, rowId: 'invalid-sla', productionSlaClaimed: true },
      { ...localRequest, rowId: 'invalid-child-custody', childActivityCustodyClaimed: true },
      { ...localRequest, rowId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
      { ...localRequest, rowId: 'invalid-raw-child', containsRawChildActivity: true },
      { ...localRequest, rowId: 'invalid-support-payload', containsRawSupportBundlePayload: true },
      { ...localRequest, rowId: 'invalid-provider-secret', containsProviderSecrets: true },
      { ...localRequest, rowId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
      { ...localRequest, rowId: 'invalid-missing-custody', custodyRefs: [] },
      { ...localRequest, rowId: 'invalid-duplicate-data-class', disclosedDataClasses: ['source-proof-ref'] },
    ]) {
      expect(() => DeleteExecutorRowSchema.parse(invalidRow)).toThrow();
    }
  });
});

function rowFor(rowId: string) {
  const row = DeleteExecutorReadModel.rows.find((candidate) => candidate.rowId === rowId);
  if (row === undefined) {
    throw new Error(`Missing delete executor row: ${rowId}`);
  }
  return row;
}

function expectRowEvidenceRefs(row: DeleteExecutorRow): void {
  expect(row.disclosedDataClasses).toEqual([...DeleteExecutorRequiredDataClasses]);
  expect(row.deleteRequestRefs).toHaveLength(1);
  expect(row.authorizationRefs).toHaveLength(1);
  expect(row.redactionAuditRefs).toHaveLength(1);
  expect(row.custodyRefs).toHaveLength(1);
  expect(row.sourceProofRefs.length).toBeGreaterThan(0);
  expect(row.manualProofRequirements.length).toBeGreaterThan(0);
}

function expectRowHasNoExecutionClaims(row: DeleteExecutorRow): void {
  expect(row.realDeleteExecuted).toBe(false);
  expect(row.durableQueueExecuted).toBe(false);
  expect(row.payloadDeletionExecuted).toBe(false);
  expect(row.providerExecutionOccurred).toBe(false);
  expect(row.publicRuntimeExecuted).toBe(false);
  expect(row.legalExecutionOccurred).toBe(false);
  expect(row.backendUploadExecuted).toBe(false);
  expect(row.productionSlaClaimed).toBe(false);
  expect(row.childActivityCustodyClaimed).toBe(false);
  expect(row.ocentraHostedFamilyDataDefault).toBe(false);
  expect(row.containsRawChildActivity).toBe(false);
  expect(row.containsRawSupportBundlePayload).toBe(false);
  expect(row.containsProviderSecrets).toBe(false);
  expect(row.containsRemoteSupportTranscripts).toBe(false);
}
