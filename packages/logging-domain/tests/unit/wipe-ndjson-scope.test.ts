import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendTestLogEntries, readTestLogEntriesFromFile } from '../../src/test-log/ndjsonWriter';
import { getRunNdjsonFilePath, getTestLogScopeDir, listNdjsonFiles } from '../../src/test-log/ndjsonPaths';
import { RunType, TestLogScope, TestSuiteType, TestLogSchemaVersion } from '../../src/test-log/types';
import { wipeNdjsonScope } from '../../src/test-log/wipeNdjsonScope';

function makeEntry(runId: string, filePath: string) {
  return {
    schemaVersion: TestLogSchemaVersion,
    type: 'log' as const,
    scope: TestLogScope.ParentTest,
    runId,
    runType: RunType.Single,
    suiteType: TestSuiteType.Unit,
    testName: `${runId}-test`,
    timestamp: 1,
    level: 'info' as const,
    source: 'portal',
    context: 'wipe',
    message: `${runId}-message`,
    data: null,
    file: path.basename(filePath),
    filePath,
    line: 1,
    column: 1,
    correlationId: null,
    tags: [],
    stack: null,
    origin: 'test' as const,
    environment: 'test',
  };
}

describe('wipe ndjson scope', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('wipes only the selected file entries within a scope', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-wipe-'));
    tempDirs.push(tempDir);

    const targetFile = 'apps/portal/src/dev-logger.ts';
    const otherFile = 'crates/agent-service/src/dev_log.rs';
    appendTestLogEntries([makeEntry('run-a', targetFile), makeEntry('run-a', otherFile)], tempDir);

    const result = wipeNdjsonScope({
      scope: TestLogScope.ParentTest,
      runType: RunType.Single,
      suiteType: TestSuiteType.Unit,
      filePath: targetFile,
      rootDir: tempDir,
    });

    expect(result.deletedEntries).toBe(1);
    expect(result.deletedFiles).toHaveLength(0);
    expect(result.rewrittenFiles).toHaveLength(1);

    const filePath = getRunNdjsonFilePath(
      TestLogScope.ParentTest,
      RunType.Single,
      'run-a',
      TestSuiteType.Unit,
      tempDir
    );
    const remaining = readTestLogEntriesFromFile(filePath);
    expect(remaining).toHaveLength(1);
    expect(remaining[0]?.filePath).toBe(otherFile);
  });

  it('can wipe an entire run file for a scope', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-wipe-'));
    tempDirs.push(tempDir);

    appendTestLogEntries([makeEntry('run-a', 'apps/portal/src/dev-logger.ts')], tempDir);
    appendTestLogEntries([makeEntry('run-b', 'apps/portal/src/dev-logger.ts')], tempDir);

    const result = wipeNdjsonScope({
      scope: TestLogScope.ParentTest,
      runId: 'run-a',
      rootDir: tempDir,
    });

    expect(result.deletedEntries).toBe(1);
    expect(result.deletedFiles).toHaveLength(1);
    const files = listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, tempDir));
    expect(files).toHaveLength(1);
    expect(files[0]).toContain('run-b.ndjson');
  });
});
