import { spawn } from 'node:child_process';
import { mkdtemp, readdir, readFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import {
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';

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
import {
  removeDirectoryWithRetry,
  resolveDebugAgentServicePath,
  spawnVitePortal,
  stopProcessTreeAndWait,
} from './agent-service-process.mjs';

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
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-portal-log-'));

await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
    [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
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
  if (!html.includes('Ocentra Parent')) {
    throw new Error('Portal HTML shell did not include the expected title.');
  }
  await assertTypedActivityAdapterStates();
  await assertDevServerLogWritten();
  console.log('portal-local-smoke-ok');
} finally {
  await Promise.all([stopProcess(portal), stopProcess(agent)]);
  await removeDirectoryWithRetry(devLogDir);
}

function assertTypedActivityAdapterStates() {
  const steps = [
    {
      messageId: 'cmd-portal-smoke-activity-report',
      command: AgentCommand.ActivityReportDailyGenerate,
      event: AgentEvent.ActivityReportGenerated,
      field: AgentProtocolDefaults.Field.ActivityReportDocument,
    },
    {
      messageId: 'cmd-portal-smoke-activity-report-history',
      command: AgentCommand.ActivityReportHistoryList,
      event: AgentEvent.ActivityReportHistoryReported,
      field: AgentProtocolDefaults.Field.ActivityReports,
    },
    {
      messageId: 'cmd-portal-smoke-activity-screen',
      command: AgentCommand.ActivityScreenReadModelGet,
      event: AgentEvent.ActivityScreenReadModelReported,
      field: AgentProtocolDefaults.Field.ActivityReadModel,
      readModelKind: 'screen',
    },
    {
      messageId: 'cmd-portal-smoke-activity-app-use',
      command: AgentCommand.ActivityAppUseReadModelGet,
      event: AgentEvent.ActivityAppUseReadModelReported,
      field: AgentProtocolDefaults.Field.ActivityReadModel,
      readModelKind: 'app-use',
    },
    {
      messageId: 'cmd-portal-smoke-activity-browser',
      command: AgentCommand.ActivityBrowserReadModelGet,
      event: AgentEvent.ActivityBrowserReadModelReported,
      field: AgentProtocolDefaults.Field.ActivityReadModel,
      readModelKind: 'browser',
    },
    {
      messageId: 'cmd-portal-smoke-activity-games',
      command: AgentCommand.ActivityGamesReadModelGet,
      event: AgentEvent.ActivityGamesReadModelReported,
      field: AgentProtocolDefaults.Field.ActivityReadModel,
      readModelKind: 'games',
    },
    {
      messageId: 'cmd-portal-smoke-activity-network',
      command: AgentCommand.ActivityNetworkReadModelGet,
      event: AgentEvent.ActivityNetworkReadModelReported,
      field: AgentProtocolDefaults.Field.ActivityReadModel,
      readModelKind: 'network',
    },
  ];

  return new Promise((resolve, reject) => {
    const socket = new WebSocket(createAgentWebSocketUrl(agentPort));
    let stepIndex = 0;
    let settled = false;
    const timer = setTimeout(() => fail(new Error('Typed Activity adapter smoke timed out')), 10000);

    const fail = (error) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      reject(error);
    };

    const complete = () => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      resolve();
    };

    const sendCurrentStep = () => {
      const step = steps[stepIndex];
      socket.send(JSON.stringify(commandEnvelope(step.messageId, step.command, activityPayload())));
    };

    socket.addEventListener('open', sendCurrentStep);

    socket.addEventListener('message', (message) => {
      try {
        const parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
        if (parsed.event === AgentEvent.ConnectionReady) {
          return;
        }

        const step = steps[stepIndex];
        if (parsed.event !== step.event) {
          fail(new Error(`Expected ${step.event}, received ${parsed.event}`));
          return;
        }
        assertSurfacePayload(parsed.payload, step.field, step.readModelKind);
        stepIndex += 1;
        if (stepIndex === steps.length) {
          complete();
          return;
        }
        sendCurrentStep();
      } catch (error) {
        fail(error instanceof Error ? error : new Error(String(error)));
      }
    });

    socket.addEventListener('error', () => fail(new Error('Typed Activity adapter smoke WebSocket failed')));
  });
}

function assertSurfacePayload(payload, jsonField, readModelKind) {
  const state = payload[AgentProtocolDefaults.Field.ActivitySurfaceState];
  if (!allowedActivityStates().has(state)) {
    throw new Error(`Activity adapter state was not typed: ${JSON.stringify(payload)}`);
  }
  if (readModelKind !== undefined && payload[AgentProtocolDefaults.Field.ActivityReadModelKind] !== readModelKind) {
    throw new Error(`Activity adapter returned wrong read-model kind: ${JSON.stringify(payload)}`);
  }
  const jsonValue = payload[jsonField];
  if (typeof jsonValue !== 'string') {
    throw new Error(`Activity adapter did not include JSON field ${jsonField}: ${JSON.stringify(payload)}`);
  }
  const parsed = JSON.parse(jsonValue);
  if (parsed.schemaVersion !== 1) {
    throw new Error(`Activity adapter returned unexpected schema version: ${jsonValue}`);
  }
  if (typeof parsed.state === 'string' && parsed.state !== state) {
    throw new Error(`Activity adapter payload state did not match event state: ${jsonValue}`);
  }
}

function allowedActivityStates() {
  return new Set(['ready', 'empty', 'unavailable', 'offline', 'stale', 'permission-required', 'scaffold-only']);
}

function activityPayload() {
  return {
    [AgentProtocolDefaults.Field.ScopeKind]: 'family',
    [AgentProtocolDefaults.Field.FamilyId]: 'family-local',
    [AgentProtocolDefaults.Field.RangeStart]: '1970-01-01T00:00:00Z',
    [AgentProtocolDefaults.Field.RangeEnd]: new Date().toISOString(),
  };
}

function commandEnvelope(messageId, command, payload) {
  return {
    schemaVersion: 1,
    messageId,
    sentAt: new Date().toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
    command,
    payload,
  };
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

async function stopProcess(child) {
  await stopProcessTreeAndWait(child);
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
