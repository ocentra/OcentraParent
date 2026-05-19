import { spawn } from 'node:child_process';
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

await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const portal = spawnVitePortal(portalPort, {
  ...process.env,
  [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
});

try {
  await waitForHttp(createAgentHealthUrl(agentPort));
  const portalResponse = await waitForHttp(createPortalCommandsUrl(portalPort));
  const html = await portalResponse.text();
  if (!html.includes('Ocentra Parent Dev Portal')) {
    throw new Error('Portal HTML shell did not include the expected title.');
  }
  console.log('portal-local-smoke-ok');
} finally {
  stopProcess(portal);
  stopProcess(agent);
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
