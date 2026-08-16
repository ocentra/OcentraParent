import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';
import { RunType, TestLogScope } from '../../src/test-log/types';

const TSX_CLI = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
  '..',
  '..',
  '..',
  'node_modules',
  'tsx',
  'dist',
  'cli.mjs'
);

function workspaceRoot(): string {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
}

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-wipe-'));
}

function runScript(
  scriptPath: string,
  env: NodeJS.ProcessEnv,
  args: readonly string[]
): { stdout: string; stderr: string; status: number | null } {
  const result = spawnSync(process.execPath, [TSX_CLI, scriptPath, ...args], {
    cwd: workspaceRoot(),
    env,
    encoding: 'utf8',
  });
  return { stdout: result.stdout, stderr: result.stderr, status: result.status };
}

it('wipe-logs removes the scoped NDJSON tree', () => {
  const tempDir = makeTempDir();
  try {
    const scopeDir = path.join(tempDir, 'test-logs', 'parent-test', 'single', 'unit');
    fs.mkdirSync(scopeDir, { recursive: true });
    appendTestLogEntries(
      [
        {
          schemaVersion: 1,
          type: 'log',
          scope: TestLogScope.ParentTest,
          runId: 'run-a',
          runType: RunType.Single,
          suiteType: 'unit',
          testName: 'wipe-logs',
          timestamp: 1,
          level: 'info',
          source: 'portal',
          context: 'wipe',
          message: 'one',
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

    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: tempDir,
    };
    const result = runScript(path.join(workspaceRoot(), 'packages/logging-domain/scripts/wipe-logs.ts'), env, [
      '--scope=parent-test',
      '--wipe',
    ]);
    expect(result.status).toBe(0);
    expect(JSON.parse(result.stdout.trim())).toMatchObject({
      deletedEntries: 1,
    });
    expect(fs.existsSync(path.join(scopeDir, 'run-a.ndjson'))).toBe(false);
  } finally {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});
