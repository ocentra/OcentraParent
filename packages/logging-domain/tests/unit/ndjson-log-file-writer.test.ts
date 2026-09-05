import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { writeLogEntry, writeSummary } from '../../src/test-log/ndjsonLogFileWriter';
import { getDefaultLogRoot } from '../../src/test-log/ndjsonPaths';
import { TestLogScope } from '../../src/test-log/types';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';
import { asFileKey, asNdjsonSummaryContent, asTestName } from '../../src/test-log/ndjsonBrands';
import { readLocalArtifactText } from '../../src/local-artifact-file';

describe.skipIf(process.platform !== 'win32')('ndjson log file writer', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('writes summary and entry records into the scoped NDJSON file', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-ndjson-writer-'));
    tempDirs.push(tempDir);
    const previousLogDir = process.env.OCENTRA_PARENT_LOG_DIR;
    process.env.OCENTRA_PARENT_LOG_DIR = tempDir;

    try {
      const scope = {
        scope: TestLogScope.ParentTest,
        runType: 'single' as const,
        suiteType: 'unit' as const,
      };
      const fileKey = asFileKey('file-key');
      writeSummary(
        scope,
        fileKey,
        asNdjsonSummaryContent(
          '{"schemaVersion":1,"type":"log","scope":"parent-test","runId":"file-key","runType":"single","suiteType":"unit","testName":"summary","timestamp":1,"level":"info","source":null,"context":null,"message":"summary-line","data":null,"file":null,"filePath":null,"line":null,"column":null,"correlationId":null,"tags":[],"stack":null,"origin":"test","environment":null}\n'
        )
      );

      writeLogEntry(
        scope,
        fileKey,
        asTestName('Test Entry'),
        '{"schemaVersion":1,"type":"log","scope":"parent-test","runId":"file-key","runType":"single","suiteType":"unit","testName":"Test Entry","timestamp":2,"level":"info","source":null,"context":null,"message":"first","data":null,"file":null,"filePath":null,"line":null,"column":null,"correlationId":null,"tags":[],"stack":null,"origin":"test","environment":null}\n'
      );

      const root = getDefaultLogRoot();
      const summaryPath = path.join(root, 'test-logs', 'parent-test', 'single', 'unit', 'file-key.ndjson');
      const stored = readLocalArtifactText(summaryPath, root);
      expect(stored).toContain('summary-line');
      expect(stored).toContain('"message":"first"');
    } finally {
      if (previousLogDir == null) {
        delete process.env.OCENTRA_PARENT_LOG_DIR;
      } else {
        process.env.OCENTRA_PARENT_LOG_DIR = previousLogDir;
      }
    }
  });
});
