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

function writeFile(filePath: string, content: string): void {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content, 'utf8');
}

function createProofInventoryFixture(rootDir: string): void {
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'PROOF_INDEX.md'),
    [
      '# Proof Index',
      'output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/',
      'output/logging-domain-parity-proof/06-validation-and-enforcement/',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'WORKPACK_INDEX.md'),
    [
      '| Status | Workpack | Boxes | Primary source doc |',
      '| --- | --- | ---: | --- |',
      '| source-present | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 0/11 | `01-parent-logging-architecture.md` |',
      '| partial-proof | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 0/12 | `04-validation-and-enforcement.md` |',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'CHECKLIST_INDEX.md'),
    [
      '## WP03 Parent Logging Architecture and Routing',
      '- [ ] Proof root written.',
      '- [ ] Workpack completion section filled.',
      '',
      '## WP06 Validation and Enforcement',
      '- [x] Proof root written.',
      '- [x] Workpack completion section filled.',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'PLAN_STATE.md'),
    [
      'Proof inventory root: output/logging-domain-parity-proof/ now exists in this checkout, but only WP07 and WP10 roots are restored so far',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(
      rootDir,
      'output',
      'logging-domain-parity-proof',
      '03-parent-logging-architecture-and-routing',
      '16-validation-commands.log'
    ),
    'command: fixture\nexit: 0\nresult: pass\nnotes: fixture proof\n'
  );
}

function runScript(
  scriptPath: string,
  args: readonly string[],
  env: NodeJS.ProcessEnv
): { stdout: string; stderr: string; status: number | null } {
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
    let terminationRequested = false;

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
        terminationRequested = true;
        child.kill();
      }
    });
    child.stderr?.on('data', (chunk) => {
      stdout += String(chunk);
    });
    child.on('error', (error) => finish(error));
    child.on('exit', (code, signal) => {
      if (code === 0 || signal != null || terminationRequested) {
        finish(null);
        return;
      }
      finish(new Error(`bridge exited with code ${code}`));
    });
  });
}

const loggingScriptTempDirs: string[] = [];

afterEach(() => {
  for (const tempDir of loggingScriptTempDirs.splice(0, loggingScriptTempDirs.length)) {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});

describe('logging scripts bridge and query flow', () => {
  it('runs the bridge and query scripts against a temp root', async () => {
    const tempDir = makeTempDir();
    loggingScriptTempDirs.push(tempDir);
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

describe('logging scripts proof inventory query', () => {
  it('reports missing and stale logging proof inventory through agent-query', () => {
    const tempDir = makeTempDir();
    loggingScriptTempDirs.push(tempDir);
    createProofInventoryFixture(tempDir);

    const result = spawnSync(
      process.execPath,
      [path.join(workspaceRoot(), 'scripts/dev/agent-query.mjs'), 'proof-inventory'],
      {
        cwd: workspaceRoot(),
        env: {
          ...process.env,
          OCENTRA_PARENT_WORKSPACE_ROOT: tempDir,
        },
        encoding: 'utf8',
        windowsHide: true,
      }
    );

    expect(result.status).toBe(0);
    const inventory = JSON.parse(result.stdout) as {
      readonly actualPresentWorkpackIds: ReadonlyArray<string>;
      readonly actualMissingWorkpackIds: ReadonlyArray<string>;
      readonly gaps: ReadonlyArray<{ readonly kind: string; readonly workpackId?: string }>;
    };

    expect(inventory.actualPresentWorkpackIds).toEqual(['03']);
    expect(inventory.actualMissingWorkpackIds).toEqual(['06']);
    expect(inventory.gaps).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          kind: 'status-underclaims-existing-proof-root',
          workpackId: '03',
        }),
        expect.objectContaining({
          kind: 'status-claims-proof-root-but-root-missing',
          workpackId: '06',
        }),
        expect.objectContaining({
          kind: 'checklist-claims-proof-root-written-but-root-missing',
          workpackId: '06',
        }),
        expect.objectContaining({
          kind: 'checklist-claims-workpack-completion-without-proof-root',
          workpackId: '06',
        }),
        expect.objectContaining({
          kind: 'plan-state-restored-roots-drift',
        }),
      ])
    );
  });
});
