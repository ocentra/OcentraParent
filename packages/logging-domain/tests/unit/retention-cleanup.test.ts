import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendAppLogEntries, listAppLogSessionFiles, pruneAppLogSessions } from '../../src/app-log/appNdjsonWriter';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { listNdjsonFiles } from '../../src/test-log/ndjsonPaths';
import { pruneTestLogRuns } from '../../src/test-log/testLogRetention';
import { RunType, TestLogScope, TestLogSchemaVersion } from '../../src/test-log/types';

function makeTestEntry(runId: string, timestamp: number) {
  return {
    schemaVersion: TestLogSchemaVersion,
    type: 'log' as const,
    scope: TestLogScope.ParentCodex,
    runId,
    runType: RunType.Single,
    suiteType: 'unit' as const,
    testName: `${runId}-test`,
    timestamp,
    level: 'info' as const,
    source: 'codex',
    context: 'retention',
    message: runId,
    data: null,
    file: 'runner.ts',
    filePath: 'scripts/dev/runner.ts',
    line: null,
    column: null,
    correlationId: null,
    tags: [],
    stack: null,
    origin: 'codex' as const,
    environment: 'test',
  };
}

const retentionCleanupTempDirs: string[] = [];

afterEach(() => {
  for (const tempDir of retentionCleanupTempDirs.splice(0, retentionCleanupTempDirs.length)) {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});

describe('retention cleanup test-log runs', () => {
  it('keeps only the newest test runs for a scope', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-retention-'));
    retentionCleanupTempDirs.push(tempDir);

    appendTestLogEntries([makeTestEntry('run-old', 1)], tempDir);
    appendTestLogEntries([makeTestEntry('run-new', 2)], tempDir);

    const files = listNdjsonFiles(path.join(tempDir, 'test-logs'));
    const oldFile = files.find((filePath) => path.basename(filePath) === 'run-old.ndjson');
    const newFile = files.find((filePath) => path.basename(filePath) === 'run-new.ndjson');
    if (oldFile == null || newFile == null) {
      throw new Error('Expected run-old and run-new NDJSON files');
    }
    fs.utimesSync(oldFile, new Date(1_000), new Date(1_000));
    fs.utimesSync(newFile, new Date(2_000), new Date(2_000));

    const deleted = pruneTestLogRuns(TestLogScope.ParentCodex, 1, tempDir);

    expect(deleted).toBe(1);
    const remaining = listNdjsonFiles(path.join(tempDir, 'test-logs'));
    expect(remaining).toHaveLength(1);
    expect(path.basename(remaining[0]!)).toBe('run-new.ndjson');
  });
});

describe('retention cleanup app-log sessions', () => {
  it('keeps only the newest app-log sessions for a scope', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-retention-'));
    retentionCleanupTempDirs.push(tempDir);

    appendAppLogEntries(
      TestLogScope.ParentCodex,
      'session-old',
      [
        {
          schemaVersion: 1,
          sessionId: 'session-old',
          scope: TestLogScope.ParentCodex,
          timestamp: 1,
          level: 'info',
          source: 'portal',
          context: 'retention',
          message: 'old',
          data: null,
          file: null,
          filePath: null,
          line: null,
          column: null,
          correlationId: null,
          environment: 'test',
        },
      ],
      tempDir
    );
    pruneAppLogSessions(TestLogScope.ParentCodex, 2, tempDir);

    appendAppLogEntries(
      TestLogScope.ParentCodex,
      'session-new',
      [
        {
          schemaVersion: 1,
          sessionId: 'session-new',
          scope: TestLogScope.ParentCodex,
          timestamp: 2,
          level: 'info',
          source: 'portal',
          context: 'retention',
          message: 'new',
          data: null,
          file: null,
          filePath: null,
          line: null,
          column: null,
          correlationId: null,
          environment: 'test',
        },
      ],
      tempDir
    );
    pruneAppLogSessions(TestLogScope.ParentCodex, 1, tempDir);

    const files = listAppLogSessionFiles(TestLogScope.ParentCodex, tempDir);
    expect(files).toHaveLength(1);
    expect(path.basename(files[0]!)).toContain('session-new');
  });
});
