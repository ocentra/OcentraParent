import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendAppLogEntries, listAppLogSessionFiles, pruneAppLogSessions } from '../../src/app-log/appNdjsonWriter';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { listNdjsonFiles } from '../../src/test-log/ndjsonPaths';
import { testLogDerivedArtifactPaths } from '../../src/test-log/testLogMutation';
import { pruneTestLogRuns } from '../../src/test-log/testLogRetention';
import { RunType, TestLogScope, TestLogSchemaVersion } from '../../src/test-log/types';

function makeTestEntry(runId: string, timestamp: number, scope = TestLogScope.ParentCodex) {
  return {
    schemaVersion: TestLogSchemaVersion,
    type: 'log' as const,
    scope,
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

describe('retention cleanup', () => {
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
    const derivedArtifacts = testLogDerivedArtifactPaths(TestLogScope.ParentCodex, tempDir);
    fs.mkdirSync(path.dirname(derivedArtifacts.manifest), { recursive: true });
    fs.mkdirSync(path.dirname(derivedArtifacts.database), { recursive: true });
    fs.writeFileSync(derivedArtifacts.manifest, '{}', 'utf8');
    fs.writeFileSync(derivedArtifacts.database, 'db', 'utf8');
    fs.writeFileSync(derivedArtifacts.databaseWal, 'wal', 'utf8');
    fs.utimesSync(oldFile, new Date(1_000), new Date(1_000));
    fs.utimesSync(newFile, new Date(2_000), new Date(2_000));

    const deleted = pruneTestLogRuns(TestLogScope.ParentCodex, 1, tempDir);

    expect(deleted).toBe(1);
    const remaining = listNdjsonFiles(path.join(tempDir, 'test-logs'));
    expect(remaining).toHaveLength(1);
    expect(path.basename(remaining[0]!)).toBe('run-new.ndjson');
    expect(fs.existsSync(derivedArtifacts.manifest)).toBe(false);
    expect(fs.existsSync(derivedArtifacts.database)).toBe(false);
    expect(fs.existsSync(derivedArtifacts.databaseWal)).toBe(false);
  });

  it('uses a deterministic path order when test runs have the same modification time', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-retention-'));
    retentionCleanupTempDirs.push(tempDir);

    appendTestLogEntries([makeTestEntry('run-b', 1), makeTestEntry('run-a', 2)], tempDir);
    const files = listNdjsonFiles(path.join(tempDir, 'test-logs'));
    for (const filePath of files) {
      fs.utimesSync(filePath, new Date(1_000), new Date(1_000));
    }

    const deleted = pruneTestLogRuns(TestLogScope.ParentCodex, 1, tempDir);

    expect(deleted).toBe(1);
    const remaining = listNdjsonFiles(path.join(tempDir, 'test-logs'));
    expect(remaining.map((filePath) => path.basename(filePath))).toEqual(['run-a.ndjson']);
  });

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

  it('does not prune app-log sessions from another scope', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-retention-'));
    retentionCleanupTempDirs.push(tempDir);

    const entry = {
      schemaVersion: 1 as const,
      sessionId: 'portal-session',
      scope: TestLogScope.ParentPortal,
      timestamp: 1,
      level: 'info' as const,
      source: 'portal',
      context: 'retention',
      message: 'portal',
      data: null,
      file: null,
      filePath: null,
      line: null,
      column: null,
      correlationId: null,
      environment: 'test',
    };
    appendAppLogEntries(TestLogScope.ParentPortal, 'portal-session', [entry], tempDir);
    appendAppLogEntries(
      TestLogScope.ParentCodex,
      'codex-session',
      [{ ...entry, sessionId: 'codex-session', scope: TestLogScope.ParentCodex }],
      tempDir
    );

    const deleted = pruneAppLogSessions(TestLogScope.ParentCodex, 0, tempDir);

    expect(deleted).toBe(1);
    expect(
      listAppLogSessionFiles(TestLogScope.ParentPortal, tempDir).map((filePath) => path.basename(filePath))
    ).toEqual(['portal-session.ndjson']);
  });
});
