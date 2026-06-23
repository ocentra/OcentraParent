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
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-retention-'));
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

it('retention-logs reports both log families', () => {
  const tempDir = makeTempDir();
  try {
    const scopeDir = path.join(tempDir, 'test-logs', 'parent-test', 'single', 'unit');
    const appDir = path.join(tempDir, 'app-logs', 'parent-test');
    fs.mkdirSync(scopeDir, { recursive: true });
    fs.mkdirSync(appDir, { recursive: true });
    fs.writeFileSync(path.join(scopeDir, 'run-a.ndjson'), '{"message":"one"}\n', 'utf8');
    fs.writeFileSync(path.join(appDir, 'session-a.ndjson'), '{"message":"two"}\n', 'utf8');

    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: tempDir,
    };
    const result = runScript(path.join(workspaceRoot(), 'packages/logging-domain/scripts/retention-logs.ts'), env, [
      '--scope=parent-test',
      '--keep=0',
    ]);
    expect(result.status).toBe(0);
    expect(result.stdout).toContain('testRunsDeleted');
    expect(result.stdout).toContain('appSessionsDeleted');
    expect(fs.existsSync(path.join(scopeDir, 'run-a.ndjson'))).toBe(false);
    expect(fs.existsSync(path.join(appDir, 'session-a.ndjson'))).toBe(false);
  } finally {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});
