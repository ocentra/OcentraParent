import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { RunType, TestLogScope } from '../../src/test-log/types';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const queryScriptPath = path.join(packageRoot, 'scripts', 'query-test-logs.ts');

function runQueryScript(args: readonly string[], logRoot: string): string {
  const result = spawnSync(process.execPath, ['--import', 'tsx', queryScriptPath, ...args], {
    cwd: packageRoot,
    env: {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: logRoot,
    },
    encoding: 'utf8',
  });

  expect(result.status).toBe(0);
  return result.stdout.trim();
}

describe.skipIf(process.platform !== 'win32')('query-test-logs script', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('returns stats, failures, and search output from DuckDB', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-query-'));
    tempDirs.push(tempDir);

    appendTestLogEntries(
      [
        {
          schemaVersion: 1,
          type: 'log',
          scope: TestLogScope.ParentTest,
          runId: 'run-script',
          runType: RunType.Single,
          suiteType: 'unit',
          testName: 'script smoke',
          timestamp: 123,
          level: 'error',
          source: 'portal',
          context: 'script',
          message: 'queryable failure',
          data: '{"kind":"smoke"}',
          file: null,
          filePath: null,
          line: null,
          column: null,
          correlationId: null,
          tags: ['failure'],
          stack: null,
          origin: 'portal',
          environment: 'test',
        },
      ],
      tempDir
    );
    await closeLocalArtifactMutationProvider(tempDir);

    const stats = JSON.parse(runQueryScript(['stats', '--scope=parent-test'], tempDir)) as {
      totalLogs: number;
      errorLogs: number;
    };
    expect(stats.totalLogs).toBe(1);
    expect(stats.errorLogs).toBe(1);

    const failures = JSON.parse(runQueryScript(['latest-failures', '--scope=parent-test'], tempDir)) as Array<{
      message: string;
    }>;
    expect(failures).toHaveLength(1);
    expect(failures[0]?.message).toBe('queryable failure');

    const search = JSON.parse(runQueryScript(['search', 'queryable', '--scope=parent-test'], tempDir)) as Array<{
      message: string;
    }>;
    expect(search).toHaveLength(1);
    expect(search[0]?.message).toBe('queryable failure');
  });
});
