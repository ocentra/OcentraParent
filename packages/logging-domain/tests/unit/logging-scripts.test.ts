import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';
import tsxPackage from 'tsx/package.json' with { type: 'json' };

const TSX_CLI = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
  '..',
  '..',
  '..',
  'node_modules',
  'tsx',
  tsxPackage.bin
);

function workspaceRoot(): string {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
}

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-scripts-'));
}

function runScript(scriptPath: string, args: readonly string[], env: NodeJS.ProcessEnv): { stdout: string; stderr: string; status: number | null } {
  const result = spawnSync(process.execPath, [TSX_CLI, scriptPath, ...args], {
    cwd: workspaceRoot(),
    env,
    encoding: 'utf8',
  });
  return { stdout: result.stdout, stderr: result.stderr, status: result.status };
}

async function runBridgeScript(scriptPath: string, env: NodeJS.ProcessEnv): Promise<string> {
  return await new Promise<string>((resolve, reject) => {
    const child = spawn(process.execPath, [TSX_CLI, scriptPath], {
      cwd: workspaceRoot(),
      env,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });

    let stdout = '';
    let settled = false;

    const finish = (error: Error | null) => {
      if (settled) {
        return;
      }
      settled = true;
      if (error != null) {
        reject(error);
        return;
      }
      resolve(stdout);
    };

    child.stdout?.on('data', (chunk) => {
      stdout += String(chunk);
      if (stdout.includes('Logging bridge listening')) {
        child.kill();
      }
    });
    child.stderr?.on('data', (chunk) => {
      stdout += String(chunk);
    });
    child.on('error', (error) => finish(error));
    child.on('exit', (code, signal) => {
      if (code === 0 || signal != null) {
        finish(null);
        return;
      }
      finish(new Error(`bridge exited with code ${code}`));
    });
  });
}

describe('logging scripts', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('runs the bridge and query scripts against a temp root', async () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: tempDir,
      OCENTRA_PARENT_LOG_BRIDGE_PORT: '0',
      OCENTRA_PARENT_LOG_BRIDGE_HOST: '127.0.0.1',
    };

    const bridgeOutput = await runBridgeScript(
      path.join(workspaceRoot(), 'packages/logging-domain/scripts/log-bridge.ts'),
      env
    );
    expect(bridgeOutput).toContain('Logging bridge listening');

    const view = runScript(
      path.join(workspaceRoot(), 'packages/logging-domain/scripts/view-ndjson.ts'),
      ['--scope=parent-test'],
      env
    );
    expect(view.status).toBe(0);

    const query = runScript(
      path.join(workspaceRoot(), 'packages/logging-domain/scripts/query-test-logs.ts'),
      ['stats', '--scope=parent-test'],
      env
    );
    expect(query.status).toBe(0);
    expect(query.stdout).toContain('totalLogs');
  });
});
