import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { writeLogEntry, writeSummary } from '../../src/test-log/ndjsonLogFileWriter';
import { getDefaultLogRoot } from '../../src/test-log/ndjsonPaths';
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
    const previousLogDir = process.env.OCENTRA_PARENT_LOG_DIR;
    process.env.OCENTRA_PARENT_LOG_DIR = tempDir;

    try {
      writeSummary(
        {
          scope: TestLogScope.ParentTest,
          runType: 'single',
          suiteType: 'unit',
        },
        'file-key' as never,
        '{"schemaVersion":1,"type":"log","scope":"parent-test","runId":"file-key","runType":"single","suiteType":"unit","testName":"summary","timestamp":1,"level":"info","source":null,"context":null,"message":"summary-line","data":null,"file":null,"filePath":null,"line":null,"column":null,"correlationId":null,"tags":[],"stack":null,"origin":"test","environment":null}\n' as never
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

      const root = getDefaultLogRoot();
      const summaryPath = path.join(root, 'test-logs', 'parent-test', 'single', 'unit', 'file-key.ndjson');
      const entryPath = path.join(root, 'test-logs', 'parent-test', 'single', 'unit', 'test-entry.ndjson');
      expect(fs.readFileSync(summaryPath, 'utf8')).toContain('summary-line');
      expect(fs.readFileSync(entryPath, 'utf8')).toContain('"message":"first"');
    } finally {
      if (previousLogDir == null) {
        delete process.env.OCENTRA_PARENT_LOG_DIR;
      } else {
        process.env.OCENTRA_PARENT_LOG_DIR = previousLogDir;
      }
    }
  });
});
