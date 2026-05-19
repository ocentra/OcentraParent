import { spawn } from 'node:child_process';
import { setTimeout as delay } from 'node:timers/promises';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  ParentDevEnv,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  isLikelyParentAgentOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';

const port = ParentDevPort.WebSocketSmokeAgent;
const healthUrl = createAgentHealthUrl(port);
const wsUrl = createAgentWebSocketUrl(port);

await ensurePortFree(port, isLikelyParentAgentOccupant, console.log);

const service = spawn('.\\target\\debug\\ocentra-parent-agent-service.exe', [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(port),
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const serviceOutput = collectOutput(service);

try {
  await waitForHttp(healthUrl);
  const received = await runWebSocketSmoke();
  if (!received.includes('agent.health.reported')) {
    throw new Error(`Expected health event, received ${received.join(',')}`);
  }
  console.log(`websocket-local-smoke-ok:${received.join(',')}`);
} finally {
  stopProcess(service);
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
  throw new Error(`Timed out waiting for ${url}\n${serviceOutput()}`);
}

function runWebSocketSmoke() {
  return new Promise((resolve, reject) => {
    const events = [];
    const socket = new WebSocket(wsUrl);
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error('WebSocket smoke timed out'));
    }, 10000);

    socket.addEventListener('open', () => {
      socket.send(
        JSON.stringify({
          schemaVersion: 1,
          messageId: 'cmd-integration-health',
          sentAt: new Date().toISOString(),
          source: { peerId: 'portal-dev', role: 'portal' },
          target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
          command: 'agent.health.check',
          payload: {},
        })
      );
    });

    socket.addEventListener('message', (message) => {
      const parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
      events.push(parsed.event);
      if (parsed.event === 'agent.health.reported') {
        clearTimeout(timer);
        socket.close();
        resolve(events);
      }
    });

    socket.addEventListener('error', () => {
      clearTimeout(timer);
      reject(new Error('WebSocket smoke failed'));
    });
  });
}

function stopProcess(child) {
  if (process.platform === 'win32' && child.pid !== undefined) {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }
  child.kill();
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}
