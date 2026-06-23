#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { setTimeout as delay } from 'node:timers/promises';
import { getLatestFailures } from '../dev/lib/log-query-service.mjs';
import { getDuckDbPath, getNdjsonFilePath, getScopeRoot } from '../dev/lib/agent-log-paths.mjs';

const repoRoot = process.cwd();
const runtimeRoot = path.join(repoRoot, 'test-results', 'logging-local-evidence-smoke', 'runtime');
const evidenceScope = 'parent-codex';

function runCommand(command, args, env) {
  return new Promise((resolve) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      env,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });

    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString();
    });
    child.on('close', (code) => {
      resolve({ exitCode: code ?? 0, stdout, stderr });
    });
    child.on('error', (error) => {
      resolve({ exitCode: 1, stdout, stderr: `${stderr}${error.stack ?? error.message}\n` });
    });
  });
}

function runNpm(args, env) {
  if (process.platform === 'win32') {
    return runCommand('cmd', ['/c', 'npm', ...args], env);
  }
  return runCommand('npm', args, env);
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`logging evidence smoke failed: ${message}`);
  }
}

async function removeRuntimeRootWithRetry(targetPath) {
  let lastError = null;
  for (let attempt = 1; attempt <= 10; attempt += 1) {
    try {
      fs.rmSync(targetPath, { recursive: true, force: true });
      if (!fs.existsSync(targetPath)) {
        return;
      }
    } catch (error) {
      lastError = error;
    }
    await delay(150 * attempt);
  }

  if (fs.existsSync(targetPath)) {
    throw lastError ?? new Error(`Failed to remove ${targetPath}`);
  }
}

async function main() {
  fs.rmSync(runtimeRoot, { recursive: true, force: true });
  fs.mkdirSync(runtimeRoot, { recursive: true });

  const env = {
    ...process.env,
    OCENTRA_PARENT_LOG_ROOT: runtimeRoot,
  };
  process.env.OCENTRA_PARENT_LOG_ROOT = runtimeRoot;

  try {
    const passing = await runNpm(['run', 'agent:run', '--', 'node', '-e', 'process.exit(0)'], env);
    ensure(passing.exitCode === 0, 'controlled passing command did not return exit code 0');

    const failing = await runNpm(
      [
        'run',
        'agent:run',
        '--',
        'node',
        '-e',
        'process.stderr.write("logging-local-evidence-smoke\\n"); process.exit(2)',
      ],
      env
    );
    ensure(failing.exitCode === 2, 'controlled failing command did not preserve exit code 2');

    const agentRunNdjson = getNdjsonFilePath('agent-run', evidenceScope, new Date());
    const diagnosticsNdjson = getNdjsonFilePath('diagnostics', evidenceScope, new Date());
    const artifactsNdjson = getNdjsonFilePath('artifacts', evidenceScope, new Date());
    ensure(fs.existsSync(agentRunNdjson), 'agent-run NDJSON stream was not written');
    ensure(fs.existsSync(diagnosticsNdjson), 'diagnostics NDJSON stream was not written');
    ensure(fs.existsSync(artifactsNdjson), 'artifacts NDJSON stream was not written');

    const dbPath = getDuckDbPath(evidenceScope);
    ensure(fs.existsSync(dbPath), 'agent evidence DuckDB file was not created');

    const failures = await getLatestFailures({ limit: 5 });
    const latest = failures.find((entry) => entry.command.join(' ').includes('process.exit(2)'));
    ensure(latest != null, 'latest-failures query did not return the controlled failed run');
    ensure(latest.diagnostics.length > 0, 'latest-failures query did not include diagnostics');

    ensure(
      latest.artifacts.some((artifact) => artifact.kind === 'stdout'),
      'stdout artifact ref missing'
    );
    ensure(
      latest.artifacts.some((artifact) => artifact.kind === 'stderr'),
      'stderr artifact ref missing'
    );
    ensure(
      latest.artifacts.every((artifact) => fs.existsSync(artifact.path)),
      'artifact files were not written to disk'
    );

    const queryOutput = await runNpm(['run', 'agent:query', '--', 'latest-failures'], env);
    ensure(queryOutput.exitCode === 0, 'agent:query latest-failures failed');
    ensure(
      queryOutput.stdout.includes('run_id:') &&
        queryOutput.stdout.includes('unique_diagnostics:') &&
        queryOutput.stdout.includes('Command exited with code 2.'),
      'agent:query latest-failures did not include compact failed-run evidence'
    );

    const evidenceOutput = await runNpm(['run', 'codex:evidence', '--', 'latest-failures'], env);
    ensure(evidenceOutput.exitCode === 0, 'codex:evidence latest-failures failed');
    ensure(
      evidenceOutput.stdout.includes('# Evidence Packet') &&
        evidenceOutput.stdout.includes('run_id:') &&
        evidenceOutput.stdout.includes('## Diagnostics'),
      'codex:evidence latest-failures did not include compact diagnostic evidence'
    );

    const scopeRoot = getScopeRoot(evidenceScope);
    ensure(fs.existsSync(scopeRoot), 'evidence scope root was not created');

    process.stdout.write(`logging-local-evidence-smoke passed for ${latest.runId}\n`);
  } finally {
    delete process.env.OCENTRA_PARENT_LOG_ROOT;
    await removeRuntimeRootWithRetry(runtimeRoot);
  }
}

void main();
