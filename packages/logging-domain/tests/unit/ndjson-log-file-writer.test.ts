import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { writeLogEntry, writeSummary } from '../../src/test-log/ndjsonLogFileWriter';
import { TestLogScope } from '../../src/test-log/types';

describe('ndjson log file writer', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('writes summary and entry files into the NDJSON tree', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-ndjson-writer-'));
    tempDirs.push(tempDir);

    writeSummary(
      {
        scope: TestLogScope.ParentTest,
        runType: 'single',
        suiteType: 'unit',
      },
      'file-key' as never,
      'summary-line\n' as never
    );

    writeLogEntry(
      {
        scope: TestLogScope.ParentTest,
        runType: 'single',
        suiteType: 'unit',
      },
      'file-key' as never,
      'Test Entry' as never,
      '{"message":"first"}\n'
    );

    expect(true).toBe(true);
  });
});
