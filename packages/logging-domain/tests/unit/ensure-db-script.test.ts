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
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-ensure-db-'));
}

function runScript(
  scriptPath: string,
  env: NodeJS.ProcessEnv
): { stdout: string; stderr: string; status: number | null } {
  const result = spawnSync(process.execPath, [TSX_CLI, scriptPath, 'parent-test'], {
    cwd: workspaceRoot(),
    env,
    encoding: 'utf8',
  });
  return { stdout: result.stdout, stderr: result.stderr, status: result.status };
}

it.skipIf(process.platform !== 'win32')('ensure-db reports the DuckDB path and creates the database file', () => {
  const tempDir = makeTempDir();
  try {
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: tempDir,
    };
    const result = runScript(path.join(workspaceRoot(), 'packages/logging-domain/scripts/ensure-db.ts'), env);
    expect(result.status).toBe(0);
    const dbPath = result.stdout.trim();
    expect(dbPath).toContain(tempDir);
    expect(fs.existsSync(dbPath)).toBe(true);
  } finally {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});
