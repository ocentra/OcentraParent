import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

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
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-rebuild-db-'));
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

it.skipIf(process.platform !== 'win32')('rebuild-db-from-ndjson rebuilds from the scoped NDJSON store', () => {
  const tempDir = makeTempDir();
  try {
    const scopeDir = path.join(tempDir, 'test-logs', 'parent-test', 'single', 'unit');
    fs.mkdirSync(scopeDir, { recursive: true });
    fs.writeFileSync(
      path.join(scopeDir, 'run-a.ndjson'),
      JSON.stringify({
        schemaVersion: 1,
        type: 'log',
        scope: 'parent-test',
        runId: 'run-a',
        runType: 'single',
        suiteType: 'unit',
        testName: 'rebuild-db',
        timestamp: 1,
        level: 'info',
        source: 'portal',
        context: 'rebuild',
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
      }) + '\n',
      'utf8'
    );

    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: tempDir,
    };
    const result = runScript(
      path.join(workspaceRoot(), 'packages/logging-domain/scripts/rebuild-db-from-ndjson.ts'),
      env,
      ['--scope=parent-test']
    );
    expect(result.status).toBe(0);
    expect(JSON.parse(result.stdout.trim())).toMatchObject({
      mode: 'rebuild',
      filesProcessed: 1,
      logsInserted: 1,
    });
  } finally {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});
