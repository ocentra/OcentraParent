import { spawn, spawnSync } from 'node:child_process';
import { once } from 'node:events';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
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
const proofRoot = path.join(repoRoot, 'output', 'network-plan-proof', '36c-parent-ui-digest-indicators');
const screenshotDir = path.join(proofRoot, '08-ui-snapshots');
const proofResultDir = path.join(repoRoot, 'test-results', 'network-parent-ui-digest-indicator-proof');
const proofPath = path.join(proofResultDir, 'proof.json');
const outputProofPath = path.join(proofRoot, 'proof-summary.json');
const validationLogPath = path.join(proofRoot, '12-validation-commands.log');
const securityLogPath = path.join(proofRoot, '09-security-negative-proof.log');
const playwrightLogPath = path.join(screenshotDir, 'network-digest-indicator-playwright.log');
const screenshotPath = path.join(screenshotDir, 'network-digest-indicator.png');
const runRoot = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-network-digest-ui-'));
const activityDbPath = path.join(runRoot, 'activity.sqlite');
const sqlPath = path.join(runRoot, 'seed-network-digest-ui.sql');
const commands = [];
const children = [];
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
  await mkdir(screenshotDir, { recursive: true });
  await mkdir(proofResultDir, { recursive: true });
  assertSourceContracts();
  await seedActivityStore();
  await runNpm(['run', 'build:contracts']);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'lint:exec']);
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
  await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

  const agent = spawnAgent();
  trackChild(agent, 'agent');
  await waitForHttp(createAgentHealthUrl(agentPort));

  const portal = spawnVitePortal(portalPort, portalEnv(), repoRoot);
  trackChild(portal, 'portal');
  await waitForHttp(createPortalCommandsUrl(portalPort));

  const playwright = await runPlaywright();
  await runCommand('node', ['scripts/check-source-shape.mjs']);
  await runCommand('git', ['diff', '--check']);
  await writeProof(playwright);

  console.log('network-parent-ui-digest-indicator-proof-ok');
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
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function portalEnv() {
  return {
    ...process.env,
    [ParentDevEnv.ActivityDbPath]: activityDbPath,
    [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
    [ParentDevEnv.PortalPort]: String(portalPort),
    NETWORK_DIGEST_INDICATOR_SCREENSHOT: screenshotPath,
  };
}

function trackChild(child, label) {
  children.push(child);
  child.stdout?.on('data', (chunk) => process.stdout.write(chunk));
  child.stderr?.on('data', (chunk) => process.stderr.write(chunk));
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
    'network-digest-indicator-proof.spec.ts',
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
    capabilityStatus: 'available',
    adapterId: 'windows-network-snapshot',
    networkProtocol: 'tcp',
    tcpState: 'close-wait',
    localIp: '127.0.0.1',
    localPort: 4242,
    destinationIp: '203.0.113.80',
    destinationPort: 8080,
    domainAttributionStatus: 'ip-only',
    processAttributionStatus: 'process-unknown',
  };
  const evidence = [
    {
      evidenceId: 'network-digest-evidence-1',
      kind: 'local-db-row',
      digest: 'sha256:network-digest-evidence-1',
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
  'network-digest-flow-1',
  '2026-06-05T00:36:00Z',
  'child-device-network-digest-ui',
  'windows',
  'windows-network',
  'activity.domain.observed',
  'domain',
  '203.0.113.80',
  '203.0.113.80',
  '${sqlString(JSON.stringify(fields))}',
  '${sqlString(JSON.stringify(evidence))}'
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
  const result = spawnSync(process.platform === 'win32' ? 'where' : 'which', ['sqlite3'], { encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('sqlite3 is required for network parent UI digest indicator proof.');
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
    throw new Error(`${commandLine} failed with exit code ${exitCode}`);
  }
  return { exitCode, output: chunks.join('') };
}

async function waitForHttp(url) {
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      // keep waiting for managed local service readiness
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function writeProof(playwright) {
  const checkedAt = new Date().toISOString();
  const proof = {
    checkedAt,
    planRow: '36c',
    branch: runText('git', ['branch', '--show-current']).trim(),
    commit: runText('git', ['rev-parse', 'HEAD']).trim(),
    originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
    mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
    sourceStatusShort: sourceStatusShort(),
    proofMode: 'service-backed-parent-network-digest-indicators',
    artifacts: {
      proof: relativePath(proofPath),
      outputProof: relativePath(outputProofPath),
      playwrightLog: playwright.log,
      screenshot: relativePath(screenshotPath),
      validationCommands: relativePath(validationLogPath),
      securityNegativeLog: relativePath(securityLogPath),
    },
    serviceBoundary: {
      command: 'agent.network.flow.read-model.get',
      event: 'agent.network.flow.read-model.reported',
      sourceStore: 'temporary ActivityStore SQLite activity_events',
      route: '#/activity',
      digestField: 'activityDigest',
      evidenceReferenceIds: ['network-digest-evidence-1'],
      renderedIndicators: [
        'unusual-unknown-process',
        'encrypted-content-unavailable',
        'repeated-failure',
        'vpn-proxy-tunnel',
      ],
    },
    assertions: [
      'Portal parses the service-backed ActivityNetworkFlowDigest payload.',
      'Drawer renders unusual indicator kinds from the validated digest.',
      'Drawer carries indicator evidence refs without inventing domain, URL, content, policy, or adapter claims.',
      'The proof exercises the real Rust service, WebSocket command/event path, Vite portal, and Playwright UI.',
    ],
    nonClaims: [
      'No exact URL is claimed from network-only evidence.',
      'No decrypted HTTPS/page/message/search content is shown or claimed.',
      'No policy decision, notification provider delivery, or adapter execution is claimed.',
      'The temporary seeded ActivityStore proves the UI/read-model path, not live packet capture or host filtering.',
    ],
    commands,
  };
  const proofContent = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, proofContent);
  await writeFile(outputProofPath, proofContent);
  await writeFile(
    validationLogPath,
    commands.map((entry) => `${entry.command} -> ${entry.exitCode}`).join('\n') + '\n'
  );
  await writeFile(
    securityLogPath,
    [
      `checkedAt=${checkedAt}`,
      'asserted=no exact URL claim from network-only evidence',
      'asserted=no decrypted payload, page content, message content, or search query rendering',
      'asserted=no UI-owned policy or adapter command path',
      'asserted=no live packet capture, VPN adapter, proxy installation, firewall mutation, or host filtering claim',
    ].join('\n') + '\n'
  );
}

function assertSourceContracts() {
  const digestParser = readFileSync('apps/portal/src/network-flow-read-model.ts', 'utf8');
  const liveState = readFileSync('apps/portal/src/live-activity-state.ts', 'utf8');
  const drawerSummary = readFileSync('apps/portal/src/network-evidence-drawer.ts', 'utf8');
  const drawerPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-digest-indicator-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [digestParser, 'parseNetworkFlowDigest'],
    [liveState, 'networkFlowDigest'],
    [drawerSummary, 'digestIndicators'],
    [drawerPanel, 'summary.digestIndicators'],
    [portalSpec, 'vpn-proxy-tunnel'],
    [featureDoc, 'network-parent-ui-digest-indicator-proof'],
    [checklist, '36c parent UI digest indicators'],
    [workpacks, '36c'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    if (!haystack.includes(needle)) {
      throw new Error(`missing source contract snippet: ${needle}`);
    }
  }
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceStatusShort() {
  const status = runText('git', ['status', '--short']);
  return status
    .split(/\r?\n/)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/36c-parent-ui-digest-indicators/') &&
        !filePath.startsWith('test-results/network-parent-ui-digest-indicator-proof/')
      );
    })
    .join('\n');
}

function sqlString(value) {
  return value.replaceAll("'", "''");
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
