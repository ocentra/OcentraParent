import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';

function workspaceRoot(): string {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
}

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'agent-evidence-scripts-'));
}

function runNodeScript(
  scriptPath: string,
  args: readonly string[],
  env: NodeJS.ProcessEnv
): { readonly status: number | null; readonly stdout: string; readonly stderr: string } {
  const result = spawnSync(process.execPath, [scriptPath, ...args], {
    cwd: workspaceRoot(),
    env,
    encoding: 'utf8',
  });

  return {
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  };
}

function captureValue(output: string, key: string): string {
  const match = output.match(new RegExp(`^${key}:\\s+(.+)$`, 'm'));
  if (match == null || match[1] == null) {
    throw new Error(`Missing ${key} in output:\n${output}`);
  }
  return match[1].trim();
}

function listNdjsonFiles(rootPath: string): string[] {
  if (!fs.existsSync(rootPath)) {
    return [];
  }
  const files: string[] = [];
  const stack = [rootPath];
  while (stack.length > 0) {
    const current = stack.pop();
    if (current == null) {
      continue;
    }
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      if (entry.name.endsWith('.ndjson')) {
        files.push(fullPath);
      }
    }
  }
  return files;
}

function readNdjson(rootPath: string): Array<Record<string, unknown>> {
  return listNdjsonFiles(rootPath).flatMap((filePath) =>
    fs
      .readFileSync(filePath, 'utf8')
      .trim()
      .split(/\r?\n/)
      .filter((line) => line.trim().length > 0)
      .map((line) => JSON.parse(line) as Record<string, unknown>)
  );
}

describe('agent evidence scripts', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('preserves run_id and command_id through agent-run, agent-query, and codex-evidence', () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);

    const sharedEnv = {
      ...process.env,
      OCENTRA_PARENT_LOG_ROOT: tempDir,
      LEDGER_LANE: 'wp08-script-proof',
    };

    const agentRun = runNodeScript(
      path.join(workspaceRoot(), 'scripts/dev/agent-run.mjs'),
      ['--', process.execPath, '-e', "process.stderr.write('boom\\n'); process.exit(1)"],
      sharedEnv
    );

    expect(agentRun.status).toBe(1);
    const runId = captureValue(agentRun.stdout, 'run_id');
    const commandId = captureValue(agentRun.stdout, 'command_id');
    expect(runId).not.toHaveLength(0);
    expect(commandId).not.toHaveLength(0);

    const ndjsonRoot = path.join(tempDir, 'parent-codex', 'ndjson');
    const storedEvents = readNdjson(ndjsonRoot);
    const runEvent = storedEvents.find((event) => event.eventType === 'agent-run');
    expect(runEvent?.runId).toBe(runId);
    expect(runEvent?.commandId).toBe(commandId);

    const diagnosticEvent = storedEvents.find((event) => event.eventType === 'diagnostic');
    expect(diagnosticEvent?.runId).toBe(runId);
    expect(diagnosticEvent?.commandId).toBe(commandId);

    const artifactEvent = storedEvents.find((event) => event.eventType === 'artifact');
    expect(artifactEvent?.runId).toBe(runId);
    expect(artifactEvent?.commandId).toBe(commandId);

    const agentQuery = runNodeScript(
      path.join(workspaceRoot(), 'scripts/dev/agent-query.mjs'),
      ['by-run', runId],
      sharedEnv
    );
    expect(agentQuery.status).toBe(0);
    expect(agentQuery.stdout).toContain(`run_id: ${runId}`);
    expect(agentQuery.stdout).toContain(`command_id: ${commandId}`);

    const latestFailures = runNodeScript(
      path.join(workspaceRoot(), 'scripts/dev/agent-query.mjs'),
      ['latest-failures'],
      sharedEnv
    );
    expect(latestFailures.status).toBe(0);
    expect(latestFailures.stdout).toContain(`run_id: ${runId}`);
    expect(latestFailures.stdout).toContain(`command_id: ${commandId}`);

    const codexEvidence = runNodeScript(
      path.join(workspaceRoot(), 'scripts/dev/codex-evidence.mjs'),
      ['by-run', runId],
      sharedEnv
    );
    expect(codexEvidence.status).toBe(0);
    expect(codexEvidence.stdout).toContain(`run_id: ${runId}`);
    expect(codexEvidence.stdout).toContain(`command_id: ${commandId}`);
  }, 120000);
});
