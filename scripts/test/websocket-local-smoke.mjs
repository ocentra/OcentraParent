import { spawn } from 'node:child_process';
import { mkdtemp, readdir, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import {
  ParentDevEnv,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  isLikelyParentAgentOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { createLoopbackOnlyTestEnvironment, resolveDebugAgentServicePath } from './agent-service-process.mjs';
import { createPortalSmokeCommandEnvelope } from './websocket-command-envelope.mjs';
import { parseAgentEventEnvelope } from './websocket-event-envelope.mjs';
import { runAgentEventWebSocketSession } from './websocket-smoke-client.mjs';

const port = ParentDevPort.WebSocketSmokeAgent;
const healthUrl = createAgentHealthUrl(port);
const wsUrl = createAgentWebSocketUrl(port);
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-dev-log-'));
const loopbackTestEnvironment = createLoopbackOnlyTestEnvironment();

await ensurePortFree(port, isLikelyParentAgentOccupant, console.log);

const service = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...loopbackTestEnvironment,
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
  const events = [];
  return runAgentEventWebSocketSession({
    wsUrl,
    timeoutMs: 10000,
    timeoutMessage: 'WebSocket smoke timed out',
    errorMessage: 'WebSocket smoke failed',
    closeMessage: 'WebSocket smoke closed before receiving expected events',
    parseMessage: (message) => parseAgentEventEnvelope(JSON.parse(String(message.data))),
    onOpen: ({ sendJson }) => {
      sendJson(createPortalSmokeCommandEnvelope('cmd-integration-health', 'agent.health.check', {}));
    },
    onEvent: (parsed, { sendJson, complete }) => {
      events.push(parsed.event);
      if (parsed.event === 'agent.health.reported') {
        sendJson(
          createPortalSmokeCommandEnvelope(
            'cmd-integration-activity-ingest-status',
            'agent.activity.ingest.status.get',
            {}
          )
        );
      }
      if (parsed.event === 'agent.activity.ingest.status.reported') {
        complete(events);
      }
    },
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
