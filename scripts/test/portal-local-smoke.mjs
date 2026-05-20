import { spawn } from 'node:child_process';
import { mkdtemp, readdir, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';

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
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath, spawnVitePortal, stopProcessTree } from './agent-service-process.mjs';

const agentPort = ParentDevPort.PortalSmokeAgent;
const portalPort = ParentDevPort.PortalSmokePortal;
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-portal-log-'));

await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.duckdb'),
    [ParentDevEnv.DevLogDir]: devLogDir,
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const portal = spawnVitePortal(portalPort, {
  ...process.env,
  [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
  [ParentDevEnv.DevLogDir]: devLogDir,
});

try {
  await waitForHttp(createAgentHealthUrl(agentPort));
  const portalResponse = await waitForHttp(createPortalCommandsUrl(portalPort));
  const html = await portalResponse.text();
  if (!html.includes('Ocentra Parent Dev Portal')) {
    throw new Error('Portal HTML shell did not include the expected title.');
  }
  await assertDevServerLogWritten();
  console.log('portal-local-smoke-ok');
} finally {
  stopProcess(portal);
  stopProcess(agent);
  await rm(devLogDir, { recursive: true, force: true });
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return response;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

function stopProcess(child) {
  stopProcessTree(child);
}

async function assertDevServerLogWritten() {
  const files = await readdir(devLogDir);
  const devServerLog = files.find((file) => file.startsWith('dev-server-') && file.endsWith('.ndjson'));
  if (devServerLog === undefined) {
    throw new Error(`Vite dev server log was not written in ${devLogDir}`);
  }

  const content = await readFile(join(devLogDir, devServerLog), 'utf8');
  if (!content.includes('Vite dev server started.')) {
    throw new Error(`Vite dev server log did not include startup entry:\n${content}`);
  }
}
