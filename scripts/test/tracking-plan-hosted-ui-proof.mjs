import { spawn, spawnSync } from 'node:child_process';
import { once } from 'node:events';
import { mkdtemp, mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  createPortalCommandsUrl,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
  resolveParentDevPort,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath, spawnVitePortal, stopProcessTreeAndWait } from './agent-service-process.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const portalRoot = path.join(repoRoot, 'apps', 'portal');
const workpack30 = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const proofResultDir = path.join(repoRoot, 'test-results', 'tracking-plan-hosted-ui-proof');
const proofPath = path.join(proofResultDir, 'proof.json');
const outputProofPath = path.join(workpack30, '17-hosted-ui-proof.json');
const playwrightLogPath = path.join(workpack30, '12-playwright-proof.log');
const desktopScreenshot = path.join(workpack30, '11-ui-snapshots', 'hosted-policy-tracking-live-summary.png');
const mobileScreenshot = path.join(workpack30, '11-ui-snapshots', 'hosted-policy-tracking-live-summary-mobile.png');
const accessibilitySummaryPath = path.join(proofResultDir, 'accessibility-summary.json');
const runRoot = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-tracking-hosted-ui-'));
const devLogDir = path.join(runRoot, 'dev-log');
const activityDbPath = path.join(runRoot, 'activity.sqlite');
const sqlPath = path.join(runRoot, 'seed-tracking.sql');
const commands = [];
const children = [];
const agentOutput = [];
const portalOutput = [];
const agentPort = resolveParentDevPort(
  process.env[ParentDevEnv.AgentPort],
  ParentDevPort.PortalSmokeAgent,
  ParentDevEnv.AgentPort
);
const portalPort = resolveParentDevPort(
  process.env[ParentDevEnv.PortalPort],
  ParentDevPort.PortalSmokePortal,
  ParentDevEnv.PortalPort
);

let stopping = false;

try {
  await mkdir(devLogDir, { recursive: true });
  await mkdir(workpack30, { recursive: true });
  await mkdir(proofResultDir, { recursive: true });
  await seedActivityStore();
  await runNpm(['run', 'build:contracts']);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'test', '--', 'tracking-status-panel']);
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
  await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

  const agent = spawnAgent();
  trackChild(agent, 'agent', agentOutput);
  await waitForHttp(createAgentHealthUrl(agentPort));

  const portal = spawnVitePortal(portalPort, portalEnv(), repoRoot);
  trackChild(portal, 'portal', portalOutput);
  await waitForHttp(createPortalCommandsUrl(portalPort));

  const playwright = await runPlaywright();
  await assertPortalDevLogWritten();
  await writeProof(playwright);

  console.log('tracking-plan-hosted-ui-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
} finally {
  stopping = true;
  await Promise.all(children.map((child) => stopProcessTreeAndWait(child)));
  await rm(runRoot, { recursive: true, force: true });
}

function spawnAgent() {
  return spawn(resolveDebugAgentServicePath(repoRoot), [], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.DevLogDir]: devLogDir,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function portalEnv() {
  return {
    ...process.env,
    [ParentDevEnv.ActivityDbPath]: activityDbPath,
    [ParentDevEnv.DevLogDir]: devLogDir,
    [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
    [ParentDevEnv.PortalPort]: String(portalPort),
  };
}

function trackChild(child, label, output) {
  children.push(child);
  child.stdout?.on('data', (chunk) => {
    output.push(String(chunk));
    process.stdout.write(chunk);
  });
  child.stderr?.on('data', (chunk) => {
    output.push(String(chunk));
    process.stderr.write(chunk);
  });
  child.once('exit', (code, signal) => {
    if (!stopping && code !== 0) {
      console.error(`${label} process exited early: code=${code ?? 'null'} signal=${signal ?? 'null'}`);
    }
  });
}

async function runPlaywright() {
  const cliPath = path.join(repoRoot, 'node_modules', '@playwright', 'test', 'cli.js');
  const args = [
    cliPath,
    'test',
    '--config',
    path.join(portalRoot, 'playwright.config.ts'),
    'tracking-hosted-ui-proof.spec.ts',
    '--workers=1',
  ];
  const result = await runCommand(process.execPath, args, { cwd: portalRoot, env: portalEnv(), capture: true });
  await writeFile(playwrightLogPath, `${result.output.trimEnd()}\n`);
  return {
    command: [process.execPath, ...args].join(' '),
    exitCode: result.exitCode,
    log: relativePath(playwrightLogPath),
  };
}

async function seedActivityStore() {
  const fields = {
    capabilityStatus: 'recent',
    evidenceReferenceIds: 'location-evidence-hosted-1,location-evidence-hosted-2',
  };
  const evidence = [
    {
      evidenceId: 'location-evidence-hosted-1',
      kind: 'local-db-row',
      digest: 'sha256:tracking-hosted-location-row',
      uri: null,
    },
    {
      evidenceId: 'location-evidence-hosted-2',
      kind: 'journal-entry',
      digest: 'sha256:tracking-hosted-journal-entry',
      uri: null,
    },
  ];
  const sql = `
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
CREATE TABLE IF NOT EXISTS activity_events (
  event_id TEXT PRIMARY KEY,
  observed_at TEXT NOT NULL,
  device_id TEXT NOT NULL,
  platform TEXT NOT NULL,
  observer TEXT NOT NULL,
  kind TEXT NOT NULL,
  subject_kind TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  subject_display_name TEXT,
  fields_json TEXT NOT NULL,
  evidence_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS activity_events_recent_idx
  ON activity_events (observed_at DESC, event_id DESC);
CREATE TABLE IF NOT EXISTS parent_rule_contexts (
  parent_rule_ref_id TEXT PRIMARY KEY,
  updated_at TEXT NOT NULL,
  expires_at TEXT,
  context_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS parent_rule_contexts_recent_idx
  ON parent_rule_contexts (updated_at DESC, parent_rule_ref_id DESC);
DELETE FROM activity_events;
INSERT INTO activity_events (
  event_id,
  observed_at,
  device_id,
  platform,
  observer,
  kind,
  subject_kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
) VALUES (
  'tracking-hosted-location-event',
  '2026-06-04T10:09:00.000Z',
  'child-android-hosted-proof',
  'android',
  'android-location',
  'activity.location.observed',
  'location',
  'school-location',
  'School location',
  ${sqlString(JSON.stringify(fields))},
  ${sqlString(JSON.stringify(evidence))}
);
INSERT INTO activity_events (
  event_id,
  observed_at,
  device_id,
  platform,
  observer,
  kind,
  subject_kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
) VALUES (
  'tracking-hosted-expected-place-event',
  '2026-06-04T10:10:00.000Z',
  'child-android-hosted-proof',
  'android',
  'tracking-engine',
  'activity.tracking.expected-place.evaluated',
  'tracking-rule',
  'expected-place-school',
  'School',
  ${sqlString(JSON.stringify(fields))},
  ${sqlString(JSON.stringify(evidence))}
);
`;
  await writeFile(sqlPath, sql);
  const sqlite = resolveSqlite();
  const result = spawnSync(sqlite, [activityDbPath, `.read ${sqlPath}`], { cwd: repoRoot, encoding: 'utf8' });
  commands.push({ command: `${sqlite} ${activityDbPath} .read ${sqlPath}`, exitCode: result.status ?? 1 });
  if (result.status !== 0) {
    throw new Error(`sqlite seed failed: ${result.stderr || result.stdout}`);
  }
}

function resolveSqlite() {
  const command = process.platform === 'win32' ? 'where' : 'which';
  const result = spawnSync(command, ['sqlite3'], { encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('sqlite3 is required for tracking hosted UI proof.');
  }
  return result.stdout.split(/\r?\n/u).find(Boolean);
}

async function runNpmWorkspace(workspaceName, args) {
  await runNpm(['--workspace', workspaceName, ...args]);
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(command, args, options = {}) {
  const commandLine = [command, ...args].join(' ');
  const chunks = [];
  const child = spawn(command, args, {
    cwd: options.cwd ?? repoRoot,
    env: options.env ?? process.env,
    stdio: ['ignore', options.capture ? 'pipe' : 'inherit', options.capture ? 'pipe' : 'inherit'],
    windowsHide: true,
  });
  if (options.capture) {
    child.stdout?.on('data', (chunk) => {
      chunks.push(String(chunk));
      process.stdout.write(chunk);
    });
    child.stderr?.on('data', (chunk) => {
      chunks.push(String(chunk));
      process.stderr.write(chunk);
    });
  }
  const [code, signal] = await once(child, 'exit');
  const exitCode = signal === null ? (code ?? 1) : 1;
  commands.push({ command: commandLine, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${exitCode}`);
  }
  return {
    exitCode,
    output: chunks.join(''),
  };
}

async function waitForHttp(url, timeoutMs = 30000) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await delay(250);
  }
  throw lastError instanceof Error ? lastError : new Error(`Timed out waiting for ${url}`);
}

async function assertPortalDevLogWritten() {
  const content = await waitForDevLogContent('portal-', 'Portal command sent.');
  if (!content.includes('Portal WebSocket event received.')) {
    throw new Error(`Portal dev log did not include WebSocket event entry:\n${content}`);
  }
}

async function waitForDevLogContent(prefix, expectedText) {
  const deadline = Date.now() + 10000;
  while (Date.now() < deadline) {
    const files = await readdir(devLogDir);
    const logFile = files.find((file) => file.startsWith(prefix) && file.endsWith('.ndjson'));
    if (logFile !== undefined) {
      const content = await readFile(path.join(devLogDir, logFile), 'utf8');
      if (content.includes(expectedText)) {
        return content;
      }
    }
    await delay(250);
  }
  throw new Error(`Timed out waiting for dev log ${prefix} in ${devLogDir}`);
}

async function writeProof(playwright) {
  const accessibilitySummary = JSON.parse(await readFile(accessibilitySummaryPath, 'utf8'));
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    workpackIds: [
      '30-parent-and-child-ui-ux-surfaces',
      '32-journal-sqlite-and-read-model-proof',
      '33-proof-gates-fixtures-rollout-and-pr-gate',
    ],
    proofMode: 'tracking-hosted-portal-service-citation-proof',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: 'proved',
    productClaimReady: false,
    ports: {
      agent: agentPort,
      portal: portalPort,
    },
    activityStoreSeed: {
      path: 'temporary ActivityStore SQLite database',
      rowsSeeded: 2,
      latestEventId: 'tracking-hosted-expected-place-event',
      latestObservedAt: '2026-06-04T10:10:00.000Z',
      evidenceReferenceIds: ['location-evidence-hosted-1', 'location-evidence-hosted-2'],
    },
    serviceBoundary: {
      command: 'agent.activity.tracking.read-model.get',
      event: 'agent.activity.tracking.read-model.reported',
      payloadField: 'trackingReadModel',
      sourceStore: 'ActivityStore SQLite activity_events',
      route: '#/policy-tracking',
      renderedCitations: ['Event ID', 'Rows returned', 'Last observed', 'Capability', 'Custody', 'Evidence references'],
    },
    artifacts: {
      proof: relativePath(proofPath),
      outputProof: relativePath(outputProofPath),
      playwrightLog: playwright.log,
      desktopScreenshot: relativePath(desktopScreenshot),
      mobileScreenshot: relativePath(mobileScreenshot),
      accessibilitySummary: relativePath(accessibilitySummaryPath),
    },
    accessibilitySummary,
    commands,
    nonClaims: [
      'This proof does not claim Android or iOS physical background tracking behavior.',
      'This proof does not claim real physical device location, geofence, provider, or notification delivery.',
      'This proof uses a seeded temporary ActivityStore SQLite database to prove hosted portal rendering against the real Rust service command.',
      'This proof does not claim full child-device UI or authority-enrolled hard-control readiness.',
    ],
    remainingGapsBeforeProductReady: [
      'Android Studio/emulator package proof artifacts remain separate from this hosted portal proof.',
      'iOS simulator and physical-device proof artifacts remain pending.',
      'Full child/parent tracking UI beyond the first tracking proof route remains pending.',
      'Authority-enrolled hard-control and production pilot proof remain absent.',
    ],
  };
  const content = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, content);
  await writeFile(outputProofPath, content);
}

async function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function sqlString(value) {
  return `'${value.replaceAll("'", "''")}'`;
}

function relativePath(value) {
  return path.relative(repoRoot, value).replace(/\\/gu, '/');
}
