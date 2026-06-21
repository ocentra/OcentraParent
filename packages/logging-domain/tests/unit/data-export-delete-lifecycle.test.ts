import { describe, expect, it } from 'vitest';

import {
  DataExportDeleteLifecycleEntrySchema,
  DataExportDeleteLifecycleReadModelSchema,
  DataExportDeleteLifecycleRequiredDataClasses,
} from '@ocentra-parent/schema-domain/data-export-delete-lifecycle';
import { DataExportDeleteLifecycleReadModel } from '@ocentra-parent/schema-domain/data-export-delete-lifecycle-read-model';

describe('data export delete lifecycle logging contract', () => {
  it('covers requested authorized queued running succeeded failed and manual-required states', () => {
    const readModel = DataExportDeleteLifecycleReadModelSchema.parse(DataExportDeleteLifecycleReadModel);

    expect(readModel.readModelId).toBe('production-support-data-export-delete-lifecycle-proof');
    expect(countBy(readModel.entries.map((entry) => entry.lifecycleState))).toEqual({
      requested: 2,
      authorized: 2,
      queued: 2,
      running: 2,
      succeeded: 2,
      failed: 2,
      'manual-required': 2,
    });
  });

  it('requires parent authorization redaction local custody and audit refs', () => {
    for (const entry of DataExportDeleteLifecycleReadModel.entries) {
      expectSupportSafeLifecycleEntry(entry);
    }
  });

  it('rejects sensitive data and runtime execution overclaims', () => {
    const succeeded = entryFor('data-export-succeeded');

    for (const invalidEntry of invalidLifecycleEntries(succeeded)) {
      expect(() => DataExportDeleteLifecycleEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
});

function expectSupportSafeLifecycleEntry(entry: (typeof DataExportDeleteLifecycleReadModel.entries)[number]) {
  expect(entry.parentInitiationState).toBe('parent-initiated');
  expect(entry.parentAuthorizationState).toBe('parent-authorized');
  expect(entry.payloadState).toBe('redacted-runtime-status-only');
  expect(entry.custodyState).toBe('parent-owned-local-output-only');
  expect(entry.disclosedDataClasses).toEqual([...DataExportDeleteLifecycleRequiredDataClasses]);
  expect(entry.requestRefs).toHaveLength(1);
  expect(entry.authorizationRefs).toHaveLength(1);
  expect(entry.queueRefs).toHaveLength(1);
  expect(entry.auditRefs).toHaveLength(1);
  expect(entry.custodyRefs).toEqual(['data-custody-local-export-delete-boundary-ref']);
  expect(entry.containsTokens).toBe(false);
  expect(entry.containsRawChildActivity).toBe(false);
  expect(entry.containsRawUrls).toBe(false);
  expect(entry.containsScreenshots).toBe(false);
  expect(entry.containsJournals).toBe(false);
  expect(entry.containsSqliteSnapshots).toBe(false);
  expect(entry.containsPrivatePaths).toBe(false);
  expect(entry.containsCommandLines).toBe(false);
  expect(entry.containsKeystrokes).toBe(false);
  expect(entry.containsClipboardData).toBe(false);
  expect(entry.containsMessageContents).toBe(false);
  expect(entry.containsProviderSecrets).toBe(false);
  expect(entry.containsRemoteSupportTranscripts).toBe(false);
  expect(entry.realBackendUploadExecuted).toBe(false);
  expect(entry.publicRuntimeExecuted).toBe(false);
  expect(entry.providerExecutionOccurred).toBe(false);
  expect(entry.productionSlaClaimed).toBe(false);
  expect(entry.remoteSupportSessionExecuted).toBe(false);
  expect(entry.childActivityCustodyClaimed).toBe(false);
  expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
}

function invalidLifecycleEntries(succeeded: ReturnType<typeof entryFor>) {
  return [
    { ...succeeded, lifecycleId: 'invalid-token', containsTokens: true },
    { ...succeeded, lifecycleId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...succeeded, lifecycleId: 'invalid-raw-url', containsRawUrls: true },
    { ...succeeded, lifecycleId: 'invalid-screenshot', containsScreenshots: true },
    { ...succeeded, lifecycleId: 'invalid-journal', containsJournals: true },
    { ...succeeded, lifecycleId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...succeeded, lifecycleId: 'invalid-private-path', containsPrivatePaths: true },
    { ...succeeded, lifecycleId: 'invalid-command-line', containsCommandLines: true },
    { ...succeeded, lifecycleId: 'invalid-keystroke', containsKeystrokes: true },
    { ...succeeded, lifecycleId: 'invalid-clipboard', containsClipboardData: true },
    { ...succeeded, lifecycleId: 'invalid-message-content', containsMessageContents: true },
    { ...succeeded, lifecycleId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...succeeded, lifecycleId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...succeeded, lifecycleId: 'invalid-backend-upload', realBackendUploadExecuted: true },
    { ...succeeded, lifecycleId: 'invalid-public-runtime', publicRuntimeExecuted: true },
    { ...succeeded, lifecycleId: 'invalid-provider-execution', providerExecutionOccurred: true },
    { ...succeeded, lifecycleId: 'invalid-sla', productionSlaClaimed: true },
    { ...succeeded, lifecycleId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...succeeded, lifecycleId: 'invalid-custody', childActivityCustodyClaimed: true },
    { ...succeeded, lifecycleId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...succeeded, lifecycleId: 'invalid-no-auth', authorizationRefs: [] },
    { ...succeeded, lifecycleId: 'invalid-duplicate-class', disclosedDataClasses: ['manual-proof-ref'] },
  ];
}

function entryFor(lifecycleId: string) {
  const entry = DataExportDeleteLifecycleReadModel.entries.find((candidate) => candidate.lifecycleId === lifecycleId);
  if (entry === undefined) {
    throw new Error(`Missing data export/delete lifecycle entry: ${lifecycleId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
