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
  createParentDevBridgeUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  createPortalCommandsUrl,
  isLikelyParentAgentOccupant,
  isLikelyParentBridgeOccupant,
  isLikelyParentPortalOccupant,
  resolveParentDevPort,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import {
  buildPortalE2eRustServices,
  createLoopbackOnlyTestEnvironment,
  ensureParentDevBridgeBinaryUnlocked,
  spawnAgentService,
  spawnParentDevBridge,
  spawnVitePortal,
  stopProcessTreeAndWait,
} from './agent-service-process.mjs';
import {
  assertAgentNetworkActivityReadModel,
  describeAgentNetworkActivityReadModel,
} from './portal-network-activity-service-preflight.mjs';
import {
  describePortalNetworkActivitySeedState,
  seedPortalNetworkActivityStore,
} from './portal-network-activity-seed.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const portalRoot = path.join(repoRoot, 'apps', 'portal');
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
const parentBridgePort = resolveParentDevPort(
  process.env[ParentDevEnv.ParentBridgePort],
  portalPort >= 65535 ? ParentDevPort.ParentBridge : portalPort + 1,
  ParentDevEnv.ParentBridgePort
);
const logBridgePort = resolveParentDevPort(
  process.env['OCENTRA_PARENT_LOG_BRIDGE_PORT'],
  parentBridgePort >= 65535 ? ParentDevPort.ParentBridge + 1 : parentBridgePort + 1,
  'OCENTRA_PARENT_LOG_BRIDGE_PORT'
);
const parentBridgeUrl = createParentDevBridgeUrl(parentBridgePort);
const logBridgeUrl = createHttpOrigin(ParentDevHost.Loopback, logBridgePort);
const portalLogBridgeEnvKey = 'VITE_OCENTRA_PARENT_LOG_BRIDGE_URL';
const activityCaptureStartupDisabledEnv = 'OCENTRA_PARENT_ACTIVITY_CAPTURE_STARTUP_DISABLED';
const devLogDir = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-e2e-log-'));
const loopbackTestEnvironment = createLoopbackOnlyTestEnvironment();
const managedBrowserStatusEnvironment = {
  OCENTRA_PARENT_MANAGED_BROWSER_EXECUTABLE: path.join(devLogDir, 'managed-browser-unavailable.exe'),
  OCENTRA_PARENT_MANAGED_BROWSER_PROFILE_DIR: path.join(devLogDir, 'managed-browser-profile'),
};
const activityDbPath = path.join(devLogDir, 'activity.sqlite');
const children = [];
const playwrightArgs = playwrightArguments(process.argv.slice(2));
const agentStartupTimeoutMs = 120000;

let exitCode = 1;
let stopping = false;

function playwrightArguments(argumentsToFilter) {
  const result = [];
  for (let index = 0; index < argumentsToFilter.length; index += 1) {
    const argument = argumentsToFilter[index];
    // Enforcer adds these harness options after the child command. They are not Playwright options.
    if (argument === '--root' || argument === '--profile') {
      index += 1;
      continue;
    }
    result.push(argument);
  }
  return result;
}

try {
  buildPortalE2eRustServices(repoRoot);
  await requireManagedPortFree('agent', agentPort, isLikelyParentAgentOccupant, ParentDevEnv.AgentPort);
  await requireManagedPortFree('portal', portalPort, isLikelyParentPortalOccupant, ParentDevEnv.PortalPort);
  await requireManagedPortFree(
    'parent dev bridge',
    parentBridgePort,
    isLikelyParentBridgeOccupant,
    ParentDevEnv.ParentBridgePort
  );
  await requireManagedPortFree(
    'logging bridge',
    logBridgePort,
    isLikelyParentLogBridgeOccupant,
    'OCENTRA_PARENT_LOG_BRIDGE_PORT'
  );
  seedPortalNetworkActivityStore(activityDbPath);

  const agent = spawnAgent();
  trackChild(agent, 'agent');
  await waitForHttp(createAgentHealthUrl(agentPort), agentStartupTimeoutMs);
  await assertAgentNetworkActivityReadModel(createAgentWebSocketUrl(agentPort), activityDbPath);

  // Windows can leave the bridge image locked after a prior run even when the
  // bridge port is free, which prevents `cargo run` from replacing the binary.
  await ensureParentDevBridgeBinaryUnlocked(repoRoot);
  const bridge = spawnParentDevBridge(
    {
      ...loopbackTestEnvironment,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.DevLogDir]: devLogDir,
      [ParentDevEnv.ParentBridgePort]: String(parentBridgePort),
    },
    repoRoot
  );
  trackChild(bridge, 'bridge');
  await waitForParentDevBridge(parentBridgeUrl);

  const logBridge = spawnLogBridge();
  trackChild(logBridge, 'log bridge');
  await waitForHttp(`${logBridgeUrl}/__health__`);

  const portal = spawnVitePortal(
    portalPort,
    {
      ...loopbackTestEnvironment,
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.DevLogDir]: devLogDir,
      [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
      [ParentDevEnv.PortalParentBridgeUrl]: parentBridgeUrl,
      [portalLogBridgeEnvKey]: logBridgeUrl,
    },
    repoRoot
  );
  trackChild(portal, 'portal');
  await waitForHttp(createPortalCommandsUrl(portalPort));

  exitCode = await runPlaywright();
  if (exitCode === 0) {
    await assertPortalDevLogWritten();
  } else {
    await printPlaywrightFailureDiagnostics();
  }
} finally {
  stopping = true;
  await stopChildren();
  await rm(devLogDir, { recursive: true, force: true });
}

async function printPlaywrightFailureDiagnostics() {
  console.error('Network evidence drawer E2E failed; dumping service-visible seed diagnostics before cleanup.');
  console.error(`activityDbPath=${activityDbPath}`);
  console.error(`seedState=${JSON.stringify(describePortalNetworkActivitySeedState(activityDbPath))}`);
  try {
    const serviceState = await describeAgentNetworkActivityReadModel(createAgentWebSocketUrl(agentPort));
    console.error(`serviceReadModel=${serviceState}`);
  } catch (error) {
    console.error(`serviceReadModelError=${error instanceof Error ? error.message : String(error)}`);
  }
  await printDevLogs();
}

async function printDevLogs() {
  try {
    const files = await readdir(devLogDir);
    for (const file of files.filter((entry) => entry.endsWith('.ndjson')).sort()) {
      const content = await readFile(path.join(devLogDir, file), 'utf8');
      console.error(`devLog=${file}`);
      console.error(content);
    }
  } catch (error) {
    console.error(`devLogError=${error instanceof Error ? error.message : String(error)}`);
  }
}

process.exit(exitCode);

function spawnAgent() {
  return spawnAgentService(
    {
      ...loopbackTestEnvironment,
      ...managedBrowserStatusEnvironment,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.ActivityDbPath]: activityDbPath,
      [ParentDevEnv.DevLogDir]: devLogDir,
      [activityCaptureStartupDisabledEnv]: 'true',
    },
    repoRoot
  );
}

function spawnLogBridge() {
  const command = process.platform === 'win32' ? 'cmd.exe' : 'npm';
  const args =
    process.platform === 'win32'
      ? ['/c', 'npm run bridge --workspace @ocentra-parent/logging-domain']
      : ['run', 'bridge', '--workspace', '@ocentra-parent/logging-domain'];
  return spawn(command, args, {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...loopbackTestEnvironment,
      OCENTRA_PARENT_LOG_BRIDGE_HOST: ParentDevHost.Loopback,
      OCENTRA_PARENT_LOG_BRIDGE_PORT: String(logBridgePort),
      OCENTRA_PARENT_LOG_DIR: devLogDir,
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
  const spec = process.env['OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC'];
  const specArgs = spec === undefined || spec.trim().length === 0 ? [] : [spec.trim()];
  const child = spawn(
    process.execPath,
    [
      cliPath,
      'test',
      ...specArgs,
      '--config',
      path.join(portalRoot, 'playwright.config.ts'),
      '--workers=1',
      ...playwrightArgs,
    ],
    {
      cwd: portalRoot,
      env: {
        ...loopbackTestEnvironment,
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

async function requireManagedPortFree(label, port, shouldKill, envName) {
  const released = await ensurePortFree(port, shouldKill, console.log);
  if (released) {
    return;
  }

  throw new Error(
    [
      `Required ${label} port ${port} is occupied by a non-Parent process.`,
      `Set ${envName} to a free port or stop the foreign process before rerunning.`,
    ].join(' ')
  );
}

async function waitForHttp(url, timeoutMs = 30000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < timeoutMs) {
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
  const content = await waitForDevLogContent([
    'Portal command sent.',
    'Portal host bridge event received.',
    'Portal dev runtime started.',
    'Vite dev server started.',
  ]);
  if (
    !content.includes('Portal host bridge event received.') &&
    !content.includes('Portal dev runtime started.') &&
    !content.includes('Portal command sent.') &&
    !content.includes('Vite dev server started.')
  ) {
    throw new Error(`Portal dev log did not include startup, host-bridge, or command proof entries:\n${content}`);
  }
}

async function waitForDevLogContent(expectedText) {
  const expectedTexts = Array.isArray(expectedText) ? expectedText : [expectedText];
  const startedAt = Date.now();
  while (Date.now() - startedAt < 10000) {
    const logFiles = await listNdjsonFiles(devLogDir);
    for (const logFile of logFiles) {
      const content = await readFile(logFile, 'utf8');
      if (expectedTexts.some((entry) => content.includes(entry))) {
        return content;
      }
    }
    await delay(250);
  }
  throw new Error(`Timed out waiting for dev log entries in ${devLogDir}`);
}

async function listNdjsonFiles(directory) {
  const files = [];
  const entries = await readdir(directory, { withFileTypes: true });
  for (const entry of entries) {
    const entryPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await listNdjsonFiles(entryPath)));
      continue;
    }
    if (entry.name.endsWith('.ndjson')) {
      files.push(entryPath);
    }
  }
  return files.sort((left, right) => left.localeCompare(right));
}

async function waitForParentDevBridge(url) {
  const startedAt = Date.now();
  const loadRouteUrl = `${url}/load-route`;
  while (Date.now() - startedAt < 120000) {
    try {
      const response = await fetch(loadRouteUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          route: 'commands',
          context: null,
        }),
      });
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${loadRouteUrl}`);
}

async function stopChildren() {
  await Promise.all(children.map((child) => stopChild(child)));
}

async function stopChild(child) {
  await stopProcessTreeAndWait(child);
}

function isLikelyParentLogBridgeOccupant(occupant) {
  const text = `${occupant.name} ${occupant.commandLine}`.toLowerCase();
  return (
    text.includes('log-bridge') ||
    text.includes('ocentra_parent_log_bridge') ||
    text.includes('ocentra-parent/logging-domain') ||
    text.includes('@ocentra-parent/logging-domain')
  );
}
