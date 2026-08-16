#!/usr/bin/env node

import { randomUUID } from 'node:crypto';
import { spawn } from 'node:child_process';
import os from 'node:os';

import { writeMetadataArtifact, writeTextArtifact } from './lib/agent-artifacts.mjs';
import { parseDiagnostics } from './lib/agent-diagnostic-parsers.mjs';
import { ingestAgentEvidence } from './lib/agent-evidence-db.mjs';
import {
  appendNdjson,
  detectLaneId,
  detectMachineName,
  getEvidenceScope,
  getWorkspaceRoot,
  toPosixPath,
} from './lib/agent-log-paths.mjs';
import { formatRunSummary } from './lib/agent-summary-format.mjs';

function parseArgs(argv) {
  const options = {
    raw: false,
    includeStdout: false,
    includeStderr: false,
    cwd: process.cwd(),
    scope: getEvidenceScope(),
  };

  const command = [];
  for (let index = 0; index < argv.length; index += 1) {
    const value = argv[index];
    if (value === '--') {
      command.push(...argv.slice(index + 1));
      break;
    }
    if (value === '--raw') {
      options.raw = true;
      continue;
    }
    if (value === '--include-stdout') {
      options.includeStdout = true;
      continue;
    }
    if (value === '--include-stderr') {
      options.includeStderr = true;
      continue;
    }
    if (value.startsWith('--cwd=')) {
      options.cwd = value.slice('--cwd='.length);
      continue;
    }
    if (value.startsWith('--scope=')) {
      options.scope = value.slice('--scope='.length);
      continue;
    }
    command.push(...argv.slice(index));
    break;
  }

  return { options, command };
}

function shouldUseShell(command) {
  if (process.platform !== 'win32') {
    return false;
  }
  return ['npm', 'npx', 'pnpm', 'yarn', 'cargo'].includes(command);
}

function runCommand(command, cwd) {
  return new Promise((resolve) => {
    const [file, ...args] = command;
    const child = spawn(file, args, {
      cwd,
      env: process.env,
      stdio: ['ignore', 'pipe', 'pipe'],
      shell: shouldUseShell(file),
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString();
    });

    child.on('close', (code, signal) => {
      resolve({
        stdout,
        stderr,
        exitCode: signal == null ? (code ?? 0) : 1,
        signal,
      });
    });
    child.on('error', (error) => {
      resolve({
        stdout,
        stderr: `${stderr}${error.stack ?? error.message}\n`,
        exitCode: 1,
        signal: null,
      });
    });
  });
}

async function main() {
  const { options, command } = parseArgs(process.argv.slice(2));
  if (command.length === 0) {
    process.stderr.write('Usage: npm run agent:run -- <command>\n');
    process.exitCode = 1;
    return;
  }

  const startedAt = new Date();
  const runId = `run-${startedAt
    .toISOString()
    .replace(/[^0-9]/g, '')
    .slice(0, 14)}-${randomUUID().slice(0, 8)}`;
  const commandId = `cmd-${randomUUID().slice(0, 12)}`;
  const laneId = detectLaneId();
  const machine = detectMachineName();
  const workspace = toPosixPath(getWorkspaceRoot());
  const cwd = toPosixPath(options.cwd);

  const commandResult = await runCommand(command, options.cwd);
  const endedAt = new Date();
  const createdAt = endedAt.toISOString();

  const stdoutArtifact = writeTextArtifact({
    scope: options.scope,
    runId,
    commandId,
    kind: 'stdout',
    fileName: 'stdout.log',
    content: commandResult.stdout,
    createdAt,
  });
  const stderrArtifact = writeTextArtifact({
    scope: options.scope,
    runId,
    commandId,
    kind: 'stderr',
    fileName: 'stderr.log',
    content: commandResult.stderr,
    createdAt,
  });

  const diagnostics = parseDiagnostics({
    runId,
    commandId,
    command,
    stdout: commandResult.stdout,
    stderr: commandResult.stderr,
    stdoutArtifactPath: stdoutArtifact.path,
    stderrArtifactPath: stderrArtifact.path,
    exitCode: commandResult.exitCode,
  });

  const runEvent = {
    schemaVersion: 1,
    eventType: 'agent-run',
    runId,
    commandId,
    laneId,
    machine,
    workspace,
    cwd,
    command,
    startedAt: startedAt.toISOString(),
    endedAt: endedAt.toISOString(),
    durationMs: endedAt.getTime() - startedAt.getTime(),
    status: commandResult.exitCode === 0 ? 'passed' : 'failed',
    exitCode: commandResult.exitCode,
    stdoutArtifact: stdoutArtifact.artifactId,
    stderrArtifact: stderrArtifact.artifactId,
    summary: diagnostics[0]?.message ?? null,
  };

  const metadataArtifact = writeMetadataArtifact({
    scope: options.scope,
    runId,
    commandId,
    metadata: {
      runId,
      commandId,
      laneId,
      machine,
      workspace,
      cwd,
      command,
      startedAt: runEvent.startedAt,
      endedAt: runEvent.endedAt,
      durationMs: runEvent.durationMs,
      status: runEvent.status,
      exitCode: runEvent.exitCode,
      diagnostics: diagnostics.length,
      signal: commandResult.signal,
      node: process.version,
      platform: process.platform,
      arch: process.arch,
      user: os.userInfo().username,
    },
    createdAt,
  });

  appendNdjson('agent-run', runEvent, options.scope, endedAt);
  for (const diagnostic of diagnostics) {
    appendNdjson('diagnostics', diagnostic, options.scope, endedAt);
  }
  for (const artifact of [stdoutArtifact, stderrArtifact, metadataArtifact]) {
    appendNdjson('artifacts', artifact, options.scope, endedAt);
  }

  await ingestAgentEvidence(options.scope, false);

  process.stdout.write(
    formatRunSummary({
      run: runEvent,
      diagnostics,
      artifacts: [stdoutArtifact, stderrArtifact, metadataArtifact],
      stdout: commandResult.stdout,
      stderr: commandResult.stderr,
      includeStdout: options.includeStdout,
      includeStderr: options.includeStderr,
      raw: options.raw,
    })
  );

  process.exitCode = commandResult.exitCode;
}

void main();
