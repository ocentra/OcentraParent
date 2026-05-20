import { spawn } from 'node:child_process';
import { mkdtemp, readdir, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
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
import { resolveDebugAgentServicePath } from './agent-service-process.mjs';

const port = ParentDevPort.WebSocketSmokeAgent;
const healthUrl = createAgentHealthUrl(port);
const wsUrl = createAgentWebSocketUrl(port);
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-dev-log-'));

await ensurePortFree(port, isLikelyParentAgentOccupant, console.log);

const service = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(port),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
    [ParentDevEnv.DevLogDir]: devLogDir,
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
  if (!received.includes('agent.activity.ingest.status.reported')) {
    throw new Error(`Expected activity ingest status event, received ${received.join(',')}`);
  }
  await assertAgentDevLogWritten();
  console.log(`websocket-local-smoke-ok:${received.join(',')}`);
} finally {
  stopProcess(service);
  await rm(devLogDir, { recursive: true, force: true });
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
        socket.send(
          JSON.stringify({
            schemaVersion: 1,
            messageId: 'cmd-integration-activity-ingest-status',
            sentAt: new Date().toISOString(),
            source: { peerId: 'portal-dev', role: 'portal' },
            target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
            command: 'agent.activity.ingest.status.get',
            payload: {},
          })
        );
      }
      if (parsed.event === 'agent.activity.ingest.status.reported') {
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

async function assertAgentDevLogWritten() {
  const files = await readdir(devLogDir);
  const agentLog = files.find((file) => file.startsWith('agent-service-') && file.endsWith('.ndjson'));
  if (agentLog === undefined) {
    throw new Error(`Agent dev log was not written in ${devLogDir}`);
  }

  const content = await readFile(join(devLogDir, agentLog), 'utf8');
  if (!content.includes('Agent health endpoint requested.')) {
    throw new Error(`Agent dev log did not include health request entry:\n${content}`);
  }
}
