import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { TestLogDuckDb } from '../../src/test-log/testLogDuckDb';
import { RunType, TestLogScope } from '../../src/test-log/types';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';

async function removeDirWithRetries(dirPath: string): Promise<void> {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    try {
      fs.rmSync(dirPath, { force: true, recursive: true });
      return;
    } catch (error) {
      const isBusyError = error instanceof Error && 'code' in error && error.code === 'EBUSY';
      if (!isBusyError) {
        throw error;
      }
      if (attempt === 39) {
        return;
      }
      await new Promise((resolve) => setTimeout(resolve, 125));
    }
  }
}

function createStoredLog(runId: string, message: string, level: 'info' | 'error') {
  return {
    schemaVersion: 1 as const,
    type: 'log' as const,
    scope: TestLogScope.ParentTest,
    runId,
    runType: RunType.Single,
    suiteType: 'unit' as const,
    testName: `${runId}-test`,
    timestamp: runId === 'run-a' ? 100 : 200,
    level,
    source: 'portal',
    context: 'duckdb',
    message,
    data: null,
    file: null,
    filePath: null,
    line: null,
    column: null,
    correlationId: null,
    tags: level === 'error' ? ['failure'] : [],
    stack: null,
    origin: 'portal' as const,
    environment: 'test',
  };
}

function appendRunLog(tempDir: string, runId: string, message: string, level: 'info' | 'error'): void {
  appendTestLogEntries([createStoredLog(runId, message, level)], tempDir);
}

describe.skipIf(process.platform !== 'win32')('test-log duckdb ingest', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      await removeDirWithRetries(tempDir);
    }
  });

  it('rebuilds from NDJSON and incrementally ingests new files', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-duckdb-'));
    tempDirs.push(tempDir);
    appendRunLog(tempDir, 'run-a', 'first log', 'info');

    const db = await TestLogDuckDb.create(TestLogScope.ParentTest, tempDir);
    try {
      const rebuild = await db.ingestFromScope(TestLogScope.ParentTest, tempDir, true);
      expect(rebuild.mode).toBe('rebuild');
      expect(rebuild.filesProcessed).toBe(1);
      expect(rebuild.logsInserted).toBe(1);
      appendRunLog(tempDir, 'run-b', 'second log', 'error');

      const incremental = await db.ingestFromScope(TestLogScope.ParentTest, tempDir, false);
      expect(incremental.mode).toBe('incremental');
      expect(incremental.filesProcessed).toBe(1);
      expect(incremental.logsInserted).toBe(1);

      const stats = await db.getStats(TestLogScope.ParentTest);
      expect(stats.totalLogs).toBe(2);
      expect(stats.errorLogs).toBe(1);
      expect(stats.distinctRuns).toBe(2);
    } finally {
      await db.close();
    }
  });
});
