import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { ParentOwnedLocalExportRuntimeProofReadModel } from '@ocentra-parent/schema-domain/parent-owned-local-export-runtime';
import {
  createParentOwnedLocalExportRuntimeExecutor,
  type ParentOwnedLocalExportRuntimeAuditEntry,
} from '../../src/parent-owned-local-export-runtime-executor';

const log = Logger.instance;
log.register(
  (import.meta as ImportMeta & { readonly url?: string }).url ??
    'packages/parent-domain/tests/unit/parent-owned-local-export-runtime-executor.test.ts'
);

const tempDirectories: string[] = [];

afterEach(() => {
  while (tempDirectories.length > 0) {
    const target = tempDirectories.pop();
    if (target != null) {
      fs.rmSync(target, { recursive: true, force: true });
    }
  }
});

describe('parent-owned local export runtime executor', () => {
  it('writes encrypted output, persists audit history, and deletes exported output', async () => {
    log.logInfo('Starting export/delete executor smoke test', getStackTrace(), undefined, false);

    const runtimeRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'parent-local-export-runtime-'));
    tempDirectories.push(runtimeRoot);
    const exportFixture = ParentOwnedLocalExportRuntimeProofReadModel.jobs.find((job) => job.state === 'export-written');
    if (exportFixture === undefined || exportFixture.output === null) {
      throw new Error('missing export-written fixture');
    }

    const executor = createParentOwnedLocalExportRuntimeExecutor({
      runtimeRoot,
      encryptionSecret: 'parent-owned-local-export-runtime-test-secret',
    });

    const exportResult = await executor.executeExport({
      scope: exportFixture.scope,
      payload: {
        recoveryBundle: {
          settlementState: 'delete-confirmed',
        },
        generatedSummary: 'support-safe proof payload',
      },
      sourceEvidenceRefs: exportFixture.output.sourceEvidenceRefs,
      auditRefs: exportFixture.auditRefs,
      jobId: 'executor-export-proof-job',
      queueRef: 'executor-export-proof-queue',
      requestedAt: '2026-06-18T06:20:00.000Z',
    });

    expect(exportResult.job.state).toBe('export-written');
    expect(exportResult.job.output).not.toBeNull();
    expect(fs.existsSync(exportResult.bundlePath)).toBe(true);
    expect(fs.existsSync(exportResult.outputPath)).toBe(true);
    expect(fs.readFileSync(exportResult.bundlePath, 'utf8')).not.toContain('support-safe proof payload');
    expect(exportResult.auditEntries).toHaveLength(1);

    const persistedAudit = await executor.readAuditEntries();
    expect(persistedAudit).toHaveLength(1);
    expect(persistedAudit[0]).toMatchObject({
      operation: 'export',
      state: 'export-written',
      jobId: 'executor-export-proof-job',
    } satisfies Partial<ParentOwnedLocalExportRuntimeAuditEntry>);

    const deleteResult = await executor.executeDelete({
      scope: exportFixture.scope,
      output: exportResult.job.output!,
      auditRefs: exportFixture.auditRefs,
      jobId: 'executor-delete-proof-job',
      queueRef: 'executor-delete-proof-queue',
      requestedAt: '2026-06-18T06:21:00.000Z',
    });

    expect(deleteResult.job.state).toBe('delete-confirmed');
    expect(fs.existsSync(exportResult.bundlePath)).toBe(false);
    expect(fs.existsSync(exportResult.outputPath)).toBe(false);
    expect(deleteResult.auditEntries).toHaveLength(2);

    const missingTargetResult = await executor.executeDelete({
      scope: exportFixture.scope,
      output: exportResult.job.output!,
      auditRefs: exportFixture.auditRefs,
      jobId: 'executor-delete-missing-job',
      queueRef: 'executor-delete-missing-queue',
      requestedAt: '2026-06-18T06:22:00.000Z',
    });

    expect(missingTargetResult.job.state).toBe('delete-failed');
    expect(missingTargetResult.job.deleteReceipt?.failureReasonRef).toBe('delete-target-missing');
    expect(missingTargetResult.auditEntries).toHaveLength(3);
  });
});
