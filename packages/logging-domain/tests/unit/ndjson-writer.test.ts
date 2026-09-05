import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendTestLogEntries, readTestLogEntriesFromFile } from '../../src/test-log/ndjsonWriter';
import { RunType, TestLogScope } from '../../src/test-log/types';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';

const ndjsonWriterTempDirs: string[] = [];

afterEach(async () => {
  for (const tempDir of ndjsonWriterTempDirs.splice(0, ndjsonWriterTempDirs.length)) {
    await closeLocalArtifactMutationProvider(tempDir);
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});

describe.skipIf(process.platform !== 'win32')('ndjson writer', () => {
  it('writes one JSON object per line', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-ndjson-'));
    ndjsonWriterTempDirs.push(tempDir);
    const previousLogDir = process.env.OCENTRA_PARENT_LOG_DIR;
    process.env.OCENTRA_PARENT_LOG_DIR = tempDir;

    try {
      const [filePath] = appendTestLogEntries(
        [
          {
            schemaVersion: 1,
            type: 'log',
            scope: TestLogScope.ParentTest,
            runId: 'run-1',
            runType: RunType.Single,
            suiteType: 'unit',
            testName: 'writes first line',
            timestamp: 1,
            level: 'info',
            source: 'portal',
            context: 'ndjson',
            message: 'first',
            data: null,
            file: null,
            filePath: null,
            line: null,
            column: null,
            correlationId: null,
            tags: [],
            stack: null,
            origin: 'portal',
            environment: 'test',
          },
          {
            schemaVersion: 1,
            type: 'log',
            scope: TestLogScope.ParentTest,
            runId: 'run-1',
            runType: RunType.Single,
            suiteType: 'unit',
            testName: 'writes second line',
            timestamp: 2,
            level: 'warn',
            source: 'portal',
            context: 'ndjson',
            message: 'second',
            data: null,
            file: null,
            filePath: null,
            line: null,
            column: null,
            correlationId: null,
            tags: [],
            stack: null,
            origin: 'portal',
            environment: 'test',
          },
        ],
        tempDir
      );

      if (filePath == null) {
        throw new Error('Expected NDJSON file path');
      }
      const entries = readTestLogEntriesFromFile(filePath, tempDir);
      expect(entries).toHaveLength(2);
      expect(entries[0]).toMatchObject({ message: 'first' });
      expect(entries[1]).toMatchObject({ message: 'second' });
    } finally {
      if (previousLogDir == null) {
        delete process.env.OCENTRA_PARENT_LOG_DIR;
      } else {
        process.env.OCENTRA_PARENT_LOG_DIR = previousLogDir;
      }
    }
  });
});
