import { spawn } from 'node:child_process';
import { setTimeout as delay } from 'node:timers/promises';
import path from 'node:path';

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

const agentPort = ParentDevPort.PortalSmokeAgent;
const portalPort = ParentDevPort.PortalSmokePortal;

await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawn('.\\target\\debug\\ocentra-parent-agent-service.exe', [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const portal = spawn('cmd.exe', ['/c', `npm exec -- vite --host 127.0.0.1 --port ${portalPort} --strictPort`], {
  cwd: path.join(process.cwd(), 'apps', 'portal'),
  env: {
    ...process.env,
    [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
  },
  stdio: ['ignore', 'pipe', 'pipe'],
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
  if (process.platform === 'win32' && child.pid !== undefined) {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }
  child.kill();
}
