import { describe, expect, it } from 'vitest';
import {
  ProductionSupportDataExportDeleteLifecycleProofSchema,
  ProductionSupportDataExportDeleteLifecycleRowSchema,
  summarizeProductionSupportDataExportDeleteLifecycleRows,
} from '../src/production-support-data-export-delete-lifecycle-proof';
import { ProductionSupportDataExportDeleteLifecycleReadModel } from '../src/production-support-data-export-delete-lifecycle-read-model';

describe('production support data export delete lifecycle proof', () => {
  it('accepts export and delete lifecycle rows without runtime or custody overclaims', () => {
    const proof = ProductionSupportDataExportDeleteLifecycleProofSchema.parse(
      ProductionSupportDataExportDeleteLifecycleReadModel
    );

    expect(summarizeProductionSupportDataExportDeleteLifecycleRows(proof.rows)).toEqual({
      'export-requested': 1,
      'export-authorized': 1,
      'export-queued': 1,
      'export-running': 1,
      'export-succeeded': 1,
      'export-failed': 1,
      'export-manual-required': 1,
      'delete-requested': 1,
      'delete-authorized': 1,
      'delete-queued': 1,
      'delete-running': 1,
      'delete-succeeded': 1,
      'delete-failed': 1,
      'delete-manual-required': 1,
    });
    expect(proof.backendUploadExecutionState).toBe('not-implemented');
    expect(proof.publicRuntimeExecutionState).toBe('not-implemented');
    expect(proof.providerExecutionState).toBe('not-implemented');
    expect(proof.productionSlaState).toBe('not-implemented');
    expect(proof.remoteSupportSessionState).toBe('not-implemented');
    expect(proof.childActivityCustodyState).toBe('not-implemented');
  });

  it('rejects backend public provider SLA remote support and child custody claims', () => {
    const exportSucceeded = requiredRow('export-succeeded');

    for (const invalidRow of [
      { ...exportSucceeded, backendUploadExecutionState: 'running' },
      { ...exportSucceeded, publicRuntimeExecutionState: 'running' },
      { ...exportSucceeded, providerExecutionState: 'running' },
      { ...exportSucceeded, productionSlaState: 'succeeded' },
      { ...exportSucceeded, remoteSupportSessionState: 'running' },
      { ...exportSucceeded, childActivityCustodyState: 'succeeded' },
      { ...exportSucceeded, supportSafeDataClasses: [...exportSucceeded.supportSafeDataClasses, 'raw-child-activity'] },
      {
        ...exportSucceeded,
        forbiddenDataClasses: exportSucceeded.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'backend-upload-payload'
        ),
      },
    ]) {
      expect(ProductionSupportDataExportDeleteLifecycleRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });

  it('rejects incomplete lifecycle coverage or missing non-claims', () => {
    expect(
      ProductionSupportDataExportDeleteLifecycleProofSchema.safeParse({
        ...ProductionSupportDataExportDeleteLifecycleReadModel,
        rows: ProductionSupportDataExportDeleteLifecycleReadModel.rows.filter(
          (row) => row.surface !== 'delete-manual-required'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportDataExportDeleteLifecycleProofSchema.safeParse({
        ...ProductionSupportDataExportDeleteLifecycleReadModel,
        nonClaims: ProductionSupportDataExportDeleteLifecycleReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-child-activity-custody'
        ),
      }).success
    ).toBe(false);
  });
});

function requiredRow(surface: string) {
  const row = ProductionSupportDataExportDeleteLifecycleReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing data export/delete lifecycle row: ${surface}`);
  }
  return row;
}
