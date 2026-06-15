import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { RunType, TestLogScope } from '../../src/test-log/types';

describe('ndjson writer', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('writes one JSON object per line', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-ndjson-'));
    tempDirs.push(tempDir);

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
    const lines = fs.readFileSync(filePath, 'utf8').trim().split(/\r?\n/);
    expect(lines).toHaveLength(2);
    expect(JSON.parse(lines[0])).toMatchObject({ message: 'first' });
    expect(JSON.parse(lines[1])).toMatchObject({ message: 'second' });
  });
});
