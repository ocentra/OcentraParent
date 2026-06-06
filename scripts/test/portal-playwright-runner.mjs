import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { mkdtemp, readdir, readFile, rm } from 'node:fs/promises';
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
import { resolveDebugAgentServicePath, spawnVitePortal, stopProcessTree } from './agent-service-process.mjs';
import { seedPortalNetworkActivityStore } from './portal-network-activity-seed.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const portalRoot = path.join(repoRoot, 'apps', 'portal');
const forwardedPlaywrightArgs = process.argv.slice(2);
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
const devLogDir = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-e2e-log-'));
const activityDbPath = path.join(devLogDir, 'activity.sqlite');
const children = [];

let exitCode = 1;
let stopping = false;

try {
  await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
  await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);
  seedPortalNetworkActivityStore(activityDbPath);

  const agent = spawnAgent();
  trackChild(agent, 'agent');
  await waitForHttp(createAgentHealthUrl(agentPort));

  const portal = spawnVitePortal(
    portalPort,
    {
      ...process.env,
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.DevLogDir]: devLogDir,
      [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
    },
    repoRoot
  );
  trackChild(portal, 'portal');
  await waitForHttp(createPortalCommandsUrl(portalPort));

  exitCode = await runPlaywright();
  if (exitCode === 0) {
    await assertPortalDevLogWritten();
  }
} finally {
  stopping = true;
  await stopChildren();
  await rm(devLogDir, { recursive: true, force: true });
}

process.exit(exitCode);

function spawnAgent() {
  return spawn(resolveDebugAgentServicePath(repoRoot), [], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.DevLogDir]: devLogDir,
    },
    stdio: ['ignore', 'inherit', 'inherit'],
  });
}

function trackChild(child, label) {
  children.push(child);
  child.once('exit', (code, signal) => {
    if (!stopping && code !== 0) {
      console.error(
        `${label} process exited before Playwright completed: code=${code ?? 'null'} signal=${signal ?? 'null'}`
      );
    }
  });
}

function runPlaywright() {
  const cliPath = path.join(repoRoot, 'node_modules', '@playwright', 'test', 'cli.js');
  const child = spawn(
    process.execPath,
    [
      cliPath,
      'test',
      '--config',
      path.join(portalRoot, 'playwright.config.ts'),
      '--workers=1',
      ...forwardedPlaywrightArgs,
    ],
    {
      cwd: portalRoot,
      env: {
        ...process.env,
        [ParentDevEnv.ActivityDbPath]: activityDbPath,
        [ParentDevEnv.DevLogDir]: devLogDir,
      },
      stdio: 'inherit',
    }
  );

  return once(child, 'exit').then(([code, signal]) => {
    if (signal !== null) {
      return 1;
    }
    return code ?? 1;
  });
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function assertPortalDevLogWritten() {
  const content = await waitForDevLogContent('portal-', 'Portal command sent.');
  if (!content.includes('Portal WebSocket event received.')) {
    throw new Error(`Portal dev log did not include WebSocket event entry:\n${content}`);
  }
}

async function waitForDevLogContent(prefix, expectedText) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 10000) {
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

async function stopChildren() {
  await Promise.all(children.map((child) => stopChild(child)));
}

async function stopChild(child) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return;
  }

  const exitPromise = waitForExit(child, 5000);
  stopProcessTree(child);
  if (await exitPromise) {
    return;
  }

  forceKill(child);
  await waitForExit(child, 2000);
}

function waitForExit(child, timeout) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return true;
  }

  return Promise.race([
    once(child, 'exit').then(() => true),
    delay(timeout).then(() => child.exitCode !== null || child.signalCode !== null),
  ]);
}

function forceKill(child) {
  if (child.pid === undefined) {
    return;
  }

  if (process.platform === 'win32') {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }

  try {
    process.kill(-child.pid, 'SIGKILL');
  } catch {
    child.kill('SIGKILL');
  }
}
