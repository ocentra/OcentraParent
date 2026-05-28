import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { basename, join, relative } from 'node:path';
import { mkdir, mkdtemp, readFile, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { setTimeout as delay } from 'node:timers/promises';

import { resolveDebugAgentServicePath, stopProcessTreeAndWait } from './agent-service-process.mjs';

const repoRoot = process.cwd();
const evidenceDirectory = join(repoRoot, 'test-results', 'windows-managed-unmanaged-browser-enforcement-proof');
const timeoutMs = envNumber('OCENTRA_PARENT_BROWSER_ENFORCEMENT_TIMEOUT_MS', 30_000);

await main();

async function main() {
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await mkdir(evidenceDirectory, { recursive: true });
  const runRoot = await mkdtemp(join(evidenceDirectory, 'run-'));
  const agentPort = await freePort();
  const service = spawnAgentService(runRoot, agentPort);
  const serviceOutput = collectOutput(service);
  const launchedBrowser = await launchUnmanagedBrowser(runRoot);
  const assertions = [];

  try {
    await waitForHealth(agentPort, serviceOutput);
    if (launchedBrowser === null) {
      assertions.push({
        id: 'unmanaged-browser-terminate',
        state: 'manual-required',
        reason: 'no-supported-browser-executable-found',
        exactUrlClaimState: 'not-claimed',
      });
    } else {
      assertions.push(await assertUnmanagedTerminate(agentPort, launchedBrowser));
    }
    assertions.push(await assertUnmanagedWarn(agentPort));
    assertions.push(await assertManagedBrowserManualRequired(agentPort));
    const evidence = await writeEvidence(runRoot, assertions);
    printSummary(evidence);
    if (assertions.some((assertion) => assertion.state === 'failed')) {
      process.exitCode = 1;
    }
  } finally {
    if (launchedBrowser !== null) {
      await stopProcessTreeAndWait(launchedBrowser.child);
    }
    await stopProcessTreeAndWait(service);
  }
}

async function assertUnmanagedTerminate(agentPort, launchedBrowser) {
  const event = await requestEvent(
    agentPort,
    enforcementCommand({
      id: 'unmanaged-browser-terminate',
      policyAction: 'block',
      targetType: 'process',
      targetValue: basename(launchedBrowser.executablePath),
      processId: launchedBrowser.child.pid,
    })
  );
  const action = JSON.parse(event.payload.enforcementAction);
  const result = JSON.parse(event.payload.enforcementResult);
  const audit = JSON.parse(event.payload.enforcementAuditEvent);
  assertEqual(event.event, 'agent.enforcement.audit.reported', 'unmanaged terminate event');
  assertEqual(action.target.targetType, 'process', 'unmanaged terminate target type');
  assertEqual(action.mode, 'terminate-process', 'unmanaged terminate mode');
  assertEqual(result.status, 'actually-enforced', 'unmanaged terminate status');
  assertOneOf(
    result.adapterResultCode,
    ['process-terminated', 'process-already-exited'],
    'unmanaged terminate adapter result'
  );
  assertEqual(audit.auditEventKind, 'succeeded', 'unmanaged terminate audit');
  assertEqual(action.target.targetValue.includes('://'), false, 'unmanaged terminate target is not URL');
  return {
    id: 'unmanaged-browser-terminate',
    state: 'terminated',
    browserBoundaryState: 'unmanaged-browser-process',
    exactUrlClaimState: 'not-claimed',
    unmanagedDetectionState: 'terminated',
    processName: basename(launchedBrowser.executablePath),
    status: result.status,
    adapterResultCode: result.adapterResultCode,
    auditEventKind: audit.auditEventKind,
  };
}

async function assertUnmanagedWarn(agentPort) {
  const event = await requestEvent(
    agentPort,
    enforcementCommand({
      id: 'unmanaged-browser-warn',
      policyAction: 'warn',
      targetType: 'process',
      targetValue: 'browser-like-process',
      processId: null,
    })
  );
  const action = JSON.parse(event.payload.enforcementAction);
  const result = JSON.parse(event.payload.enforcementResult);
  const audit = JSON.parse(event.payload.enforcementAuditEvent);
  assertEqual(action.mode, 'observe-only', 'unmanaged warn mode');
  assertEqual(result.status, 'no-op', 'unmanaged warn status');
  assertEqual(result.adapterResultCode, 'no-op', 'unmanaged warn adapter result');
  assertEqual(audit.auditEventKind, 'attempted', 'unmanaged warn audit');
  return {
    id: 'unmanaged-browser-warn',
    state: 'warned',
    browserBoundaryState: 'unmanaged-browser-process',
    exactUrlClaimState: 'not-claimed',
    unmanagedDetectionState: 'warned',
    status: result.status,
    adapterResultCode: result.adapterResultCode,
    auditEventKind: audit.auditEventKind,
  };
}

async function assertManagedBrowserManualRequired(agentPort) {
  const event = await requestEvent(
    agentPort,
    enforcementCommand({
      id: 'managed-browser-manual-required',
      policyAction: 'block',
      targetType: 'site',
      targetValue: 'https://example.invalid/watch',
      processId: null,
    })
  );
  const action = JSON.parse(event.payload.enforcementAction);
  const result = JSON.parse(event.payload.enforcementResult);
  const audit = JSON.parse(event.payload.enforcementAuditEvent);
  assertEqual(action.adapterKind, 'managed-browser-control', 'managed browser adapter kind');
  assertEqual(action.mode, 'temporary-block', 'managed browser mode');
  assertEqual(result.status, 'unavailable', 'managed browser status');
  assertEqual(result.unavailableStatus.unavailableReason, 'manual-required', 'managed browser unavailable reason');
  assertEqual(audit.auditEventKind, 'unavailable', 'managed browser audit');
  return {
    id: 'managed-browser-manual-required',
    state: 'manual-required',
    browserBoundaryState: 'managed-session',
    exactUrlClaimState: 'exact-url-proven',
    unmanagedDetectionState: 'none',
    status: result.status,
    adapterResultCode: result.adapterResultCode,
    unavailableReason: result.unavailableStatus.unavailableReason,
    auditEventKind: audit.auditEventKind,
  };
}

function enforcementCommand({ id, policyAction, targetType, targetValue, processId }) {
  const now = new Date();
  const payload = {
    policyDecisionId: `decision-browser-boundary-${id}`,
    policyVersion: 'policy-browser-boundary',
    policyAction,
    targetType,
    targetId: `target-browser-boundary-${id}`,
    targetValue,
    dryRun: false,
    reasonCodes: 'parent-explicit-block',
    ruleIds: 'rule-browser-boundary',
    evidenceReferenceIds: `evidence-browser-boundary-${id}`,
    requestedAt: now.toISOString(),
    expiresAt: new Date(now.getTime() + 300_000).toISOString(),
    enforcementActionId: `action-browser-boundary-${id}`,
    enforcementResultId: `result-browser-boundary-${id}`,
    enforcementAuditEventId: `audit-browser-boundary-${id}`,
    enforcementTimerEventId: `timer-browser-boundary-${id}`,
  };
  if (processId !== null && processId !== undefined) {
    payload.processId = processId;
  }
  return {
    schemaVersion: 1,
    messageId: `cmd-browser-boundary-${id}`,
    sentAt: now.toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
    command: 'agent.enforcement.execute',
    payload,
  };
}

async function launchUnmanagedBrowser(runRoot) {
  const browser = await firstInstalledBrowser();
  if (browser === null) {
    return null;
  }
  const profile = join(runRoot, 'unmanaged-browser-profile');
  await mkdir(profile, { recursive: true });
  const child = spawn(
    browser.executablePath,
    [`--user-data-dir=${profile}`, '--no-first-run', '--no-default-browser-check', '--new-window', 'about:blank'],
    { stdio: 'ignore', windowsHide: true }
  );
  await delay(750);
  return { ...browser, child };
}

async function firstInstalledBrowser() {
  for (const candidate of browserCandidates()) {
    if (await fileExists(candidate.executablePath)) {
      return candidate;
    }
  }
  return null;
}

function browserCandidates() {
  if (process.platform !== 'win32') {
    return [];
  }
  return windowsRoots().flatMap((root) => [
    { id: 'edge-stable', executablePath: join(root, 'Microsoft', 'Edge', 'Application', 'msedge.exe') },
    { id: 'chrome-stable', executablePath: join(root, 'Google', 'Chrome', 'Application', 'chrome.exe') },
  ]);
}

function windowsRoots() {
  return [process.env.ProgramFiles, process.env['ProgramFiles(x86)'], process.env.LOCALAPPDATA].filter(Boolean);
}

function spawnAgentService(runRoot, agentPort) {
  return spawn(resolveDebugAgentServicePath(), [], {
    cwd: repoRoot,
    env: {
      ...process.env,
      OCENTRA_PARENT_AGENT_ADDR: `127.0.0.1:${agentPort}`,
      OCENTRA_PARENT_ACTIVITY_DB_PATH: join(runRoot, 'activity.sqlite'),
      OCENTRA_PARENT_ACTIVITY_JOURNAL_PATH: join(runRoot, 'activity.ndjson'),
      OCENTRA_PARENT_ACTIVITY_JOURNAL_KEY_PATH: join(runRoot, 'activity.key'),
      OCENTRA_PARENT_AGENT_ENFORCEMENT_TIMER_STATE_PATH: join(runRoot, 'enforcement-timers.json'),
      OCENTRA_PARENT_DEV_LOG_DIR: join(runRoot, 'logs'),
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
}

async function waitForHealth(agentPort, serviceOutput) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`http://127.0.0.1:${agentPort}/health`);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for service health. ${serviceOutput()}`);
}

function requestEvent(agentPort, command) {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(`ws://127.0.0.1:${agentPort}/api/dev/ws`);
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error(`Timed out waiting for ${command.messageId}.`));
    }, timeoutMs);
    socket.addEventListener('open', () => socket.send(JSON.stringify(command)));
    socket.addEventListener('message', (message) => {
      const event = JSON.parse(String(message.data));
      if (event.event === 'agent.connection.ready') {
        return;
      }
      clearTimeout(timer);
      socket.close();
      resolve(event);
    });
    socket.addEventListener('error', () => {
      clearTimeout(timer);
      reject(new Error(`WebSocket error while requesting ${command.messageId}.`));
    });
  });
}

async function writeEvidence(runRoot, assertions) {
  const generatedAt = new Date().toISOString();
  const evidence = {
    schemaVersion: 1,
    generatedAt,
    platform: process.platform,
    agentEndpoint: 'loopback-redacted',
    runRoot: relative(repoRoot, runRoot),
    states: {
      managedBrowserInterventionCapability: 'manual-required-with-live-managed-proof-script',
      unmanagedBrowserBoundary: assertions.find((assertion) => assertion.id === 'unmanaged-browser-terminate')?.state,
      exactUnmanagedUrlClaim: 'not-claimed',
    },
    assertions,
    artifacts: {
      activityJournal: relative(repoRoot, join(runRoot, 'activity.ndjson')),
      activityStore: relative(repoRoot, join(runRoot, 'activity.sqlite')),
      devLogDirectory: relative(repoRoot, join(runRoot, 'logs')),
    },
  };
  const path = join(evidenceDirectory, `${generatedAt.replaceAll(':', '-').replaceAll('.', '-')}.json`);
  await writeFile(path, `${JSON.stringify(evidence, null, 2)}\n`);
  evidence.path = path;
  await assertNoPlaintextUrlInJournal(evidence.artifacts.activityJournal);
  return evidence;
}

async function assertNoPlaintextUrlInJournal(relativeJournalPath) {
  const path = join(repoRoot, relativeJournalPath);
  const text = await readFile(path, 'utf8').catch(() => '');
  if (text.includes('https://example.invalid/watch')) {
    throw new Error('Activity journal leaked plaintext managed browser URL from enforcement proof.');
  }
}

function printSummary(evidence) {
  console.log('windows-managed-unmanaged-browser-enforcement-proof-ok=true');
  console.log(`evidence=${evidence.path}`);
  for (const assertion of evidence.assertions) {
    console.log(`${assertion.id}:${assertion.state}:${assertion.exactUrlClaimState}`);
  }
}

async function fileExists(pathValue) {
  try {
    return (await stat(pathValue)).isFile();
  } catch {
    return false;
  }
}

async function freePort() {
  const server = createServer();
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  const port = server.address().port;
  await new Promise((resolve) => server.close(resolve));
  return port;
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function envNumber(name, fallback) {
  const parsed = Number(process.env[name]);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertOneOf(actual, expected, label) {
  if (!expected.includes(actual)) {
    throw new Error(`${label}: expected one of ${expected.join(', ')}, received ${actual}`);
  }
}
