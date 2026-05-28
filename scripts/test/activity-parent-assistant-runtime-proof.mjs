import { spawn } from 'node:child_process';
import { mkdtemp, rm } from 'node:fs/promises';
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
import { resolveDebugAgentServicePath, stopProcessTreeAndWait } from './agent-service-process.mjs';

const proofPort = ParentDevPort.WebSocketSmokeAgent;
const healthUrl = createAgentHealthUrl(proofPort);
const wsUrl = createAgentWebSocketUrl(proofPort);
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-activity-parent-assistant-proof-'));
let AgentCommand;
let AgentEvent;
let AgentEventEnvelopeSchema;
let AgentProtocolDefaults;

await runPackageCommand(['run', 'build:contracts']);
({ AgentCommand, AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } =
  await import('@ocentra-parent/agent-protocol-domain/contracts'));
await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'activity_surface']);
await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'parent_assistant']);
await ensurePortFree(proofPort, isLikelyParentAgentOccupant, console.log);

const service = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(proofPort),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
    [ParentDevEnv.DevLogDir]: devLogDir,
    OCENTRA_PARENT_LOCAL_AI_EXECUTION_ENABLED: 'false',
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const serviceOutput = collectOutput(service);

try {
  await waitForHttp(healthUrl);
  await runRuntimeProof();
  console.log('activity-parent-assistant-runtime-proof-ok');
} finally {
  await stopProcessTreeAndWait(service);
  await rm(devLogDir, { recursive: true, force: true });
}

function runRuntimeProof() {
  const steps = [
    activityStep(
      'cmd-activity-daily-report',
      AgentCommand.ActivityReportDailyGenerate,
      AgentEvent.ActivityReportGenerated,
      assertReportDocument
    ),
    activityStep(
      'cmd-activity-save-report',
      AgentCommand.ActivityReportSave,
      AgentEvent.ActivityReportSaved,
      assertSavedReportDocument
    ),
    activityStep(
      'cmd-activity-report-history',
      AgentCommand.ActivityReportHistoryList,
      AgentEvent.ActivityReportHistoryReported,
      assertReportHistory
    ),
    activityStep(
      'cmd-activity-screen',
      AgentCommand.ActivityScreenReadModelGet,
      AgentEvent.ActivityScreenReadModelReported,
      (event) => assertActivityReadModel(event, 'screen')
    ),
    activityStep(
      'cmd-activity-app-use',
      AgentCommand.ActivityAppUseReadModelGet,
      AgentEvent.ActivityAppUseReadModelReported,
      (event) => assertActivityReadModel(event, 'app-use')
    ),
    activityStep(
      'cmd-activity-browser',
      AgentCommand.ActivityBrowserReadModelGet,
      AgentEvent.ActivityBrowserReadModelReported,
      (event) => assertActivityReadModel(event, 'browser')
    ),
    activityStep(
      'cmd-activity-games',
      AgentCommand.ActivityGamesReadModelGet,
      AgentEvent.ActivityGamesReadModelReported,
      (event) => assertActivityReadModel(event, 'games')
    ),
    activityStep(
      'cmd-activity-network',
      AgentCommand.ActivityNetworkReadModelGet,
      AgentEvent.ActivityNetworkReadModelReported,
      (event) => assertActivityReadModel(event, 'network')
    ),
    {
      messageId: 'cmd-parent-assistant-answer',
      command: AgentCommand.ParentAssistantAnswerGenerate,
      expectedEvent: AgentEvent.ParentAssistantAnswerReported,
      payload: {
        [AgentProtocolDefaults.Field.ParentAssistantQuestion]: 'Suggest a policy rule from recent activity.',
        [AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]:
          'Recent local Activity tab data is available as parent-visible evidence.',
      },
      assertEvent: assertParentAssistantUnavailable,
    },
  ];

  return new Promise((resolve, reject) => {
    const socket = new WebSocket(wsUrl);
    let stepIndex = 0;
    let settled = false;
    const timer = setTimeout(() => fail(new Error('Activity parent assistant proof timed out')), 45000);

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
      socket.send(JSON.stringify(commandEnvelope(step.messageId, step.command, step.payload)));
    };

    socket.addEventListener('open', sendCurrentStep);

    socket.addEventListener('message', (message) => {
      try {
        const parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
        if (parsed.event === AgentEvent.ConnectionReady) {
          return;
        }

        const step = steps[stepIndex];
        if (parsed.event !== step.expectedEvent) {
          fail(new Error(`Expected ${step.expectedEvent}, received ${parsed.event}`));
          return;
        }

        step.assertEvent(parsed);
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

    socket.addEventListener('error', () => fail(new Error('Activity parent assistant proof WebSocket failed')));
  });
}

function activityStep(messageId, command, expectedEvent, assertEvent) {
  return {
    messageId,
    command,
    expectedEvent,
    payload: activityPayload(),
    assertEvent,
  };
}

function activityPayload() {
  return {
    [AgentProtocolDefaults.Field.ScopeKind]: 'family',
    [AgentProtocolDefaults.Field.FamilyId]: 'family-local',
    [AgentProtocolDefaults.Field.RangeStart]: '1970-01-01T00:00:00Z',
    [AgentProtocolDefaults.Field.RangeEnd]: new Date().toISOString(),
    [AgentProtocolDefaults.Field.ActivityFamilySources]: JSON.stringify([
      {
        deviceId: 'child-device-offline',
        reachabilityState: 'offline',
        state: 'offline',
        reason: 'Child source is offline for this report.',
        lastUpdatedAt: null,
      },
      {
        deviceId: 'child-device-error',
        reachabilityState: 'error',
        state: 'unavailable',
        reason: 'Child source returned an error.',
        lastUpdatedAt: null,
      },
    ]),
  };
}

function assertReportDocument(event) {
  const payload = event.payload;
  assertSurfaceState(payload);
  const report = parseJsonField(payload, AgentProtocolDefaults.Field.ActivityReportDocument);
  if (!Array.isArray(report.sections) || report.sections.length < 6) {
    throw new Error(`Activity report did not include all typed sections: ${JSON.stringify(report)}`);
  }
  if (report.frequency !== 'daily') {
    throw new Error(`Activity report frequency was not daily: ${JSON.stringify(report)}`);
  }
  assertFamilySourceStates(report);
}

function assertSavedReportDocument(event) {
  const payload = event.payload;
  assertSurfaceState(payload);
  const report = parseJsonField(payload, AgentProtocolDefaults.Field.ActivityReportDocument);
  if (report.savedMetadata?.savedState !== 'saved') {
    throw new Error(`Activity report save did not persist saved metadata: ${JSON.stringify(report)}`);
  }
  if (typeof report.savedMetadata?.fileName !== 'string' || !report.savedMetadata.fileName.endsWith('.json')) {
    throw new Error(`Activity report save did not return a saved JSON file name: ${JSON.stringify(report)}`);
  }
}

function assertReportHistory(event) {
  const payload = event.payload;
  assertSurfaceState(payload);
  const history = parseJsonField(payload, AgentProtocolDefaults.Field.ActivityReports);
  if (history.state !== 'ready') {
    throw new Error(`Activity report history did not become ready after save: ${JSON.stringify(history)}`);
  }
  if (!Array.isArray(history.reports) || history.reports.length < 1) {
    throw new Error(`Activity report history did not include the saved report: ${JSON.stringify(history)}`);
  }
  if (history.reports[0]?.parsedReport?.savedMetadata?.savedState !== 'saved') {
    throw new Error(`Activity report history did not carry saved metadata: ${JSON.stringify(history)}`);
  }
}

function assertActivityReadModel(event, expectedKind) {
  const payload = event.payload;
  assertSurfaceState(payload);
  if (payload[AgentProtocolDefaults.Field.ActivityReadModelKind] !== expectedKind) {
    throw new Error(`Activity read model kind mismatch: ${JSON.stringify(payload)}`);
  }
  const readModel = parseJsonField(payload, AgentProtocolDefaults.Field.ActivityReadModel);
  if (!allowedSurfaceStates().has(readModel.state) || !Array.isArray(readModel.rows)) {
    throw new Error(`Activity read model was not typed: ${JSON.stringify(readModel)}`);
  }
}

function assertParentAssistantUnavailable(event) {
  const payload = event.payload;
  if (payload[AgentProtocolDefaults.Field.ParentAssistantProviderState] !== 'unavailable') {
    throw new Error(`Parent Assistant did not degrade unavailable: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.ParentAssistantAnswerState] !== 'unavailable') {
    throw new Error(`Parent Assistant answer state was not unavailable: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.ParentAssistantAnswerText] !== null) {
    throw new Error(`Parent Assistant produced answer text while unavailable: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.ParentAssistantCitationCount] < 1) {
    throw new Error(`Parent Assistant did not cite evidence context: ${JSON.stringify(payload)}`);
  }
  const preview = parseJsonField(payload, AgentProtocolDefaults.Field.ParentAssistantActionPreview);
  if (preview.childAgentContractRequired !== true || preview.enforcementApplied !== false) {
    throw new Error(`Parent Assistant bypassed child-agent contract or enforced directly: ${JSON.stringify(preview)}`);
  }
  if (preview.actionKind !== 'policy-suggestion' || preview.requiresControllerLease !== true) {
    throw new Error(`Parent Assistant did not prepare policy preview boundary: ${JSON.stringify(preview)}`);
  }
  const apiBoundary = parseJsonField(payload, AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary);
  if (
    apiBoundary.authorizationState !== 'not-authorized' ||
    apiBoundary.providerState !== 'unavailable' ||
    apiBoundary.childSafetyOrEnforcementUseAllowed !== false ||
    !Array.isArray(apiBoundary.citations) ||
    apiBoundary.citations.length < 1
  ) {
    throw new Error(`Parent Assistant API AI boundary was not custody-safe: ${JSON.stringify(apiBoundary)}`);
  }
}

function assertFamilySourceStates(report) {
  const reachabilityStates = new Set(report.sourceStates?.map((source) => source.reachabilityState));
  if (!reachabilityStates.has('reachable') || !reachabilityStates.has('offline') || !reachabilityStates.has('error')) {
    throw new Error(`Activity family fan-out source states were not preserved: ${JSON.stringify(report.sourceStates)}`);
  }
}

function assertSurfaceState(payload) {
  const state = payload[AgentProtocolDefaults.Field.ActivitySurfaceState];
  if (!allowedSurfaceStates().has(state)) {
    throw new Error(`Activity surface state was not typed: ${JSON.stringify(payload)}`);
  }
}

function allowedSurfaceStates() {
  return new Set(['ready', 'empty', 'unavailable', 'offline', 'stale', 'permission-required', 'scaffold-only']);
}

function parseJsonField(payload, field) {
  const value = payload[field];
  if (typeof value !== 'string') {
    throw new Error(`Expected string JSON field ${field}: ${JSON.stringify(payload)}`);
  }
  return JSON.parse(value);
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
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}\n${serviceOutput()}`);
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: process.cwd(),
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const output = collectOutput(child);
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} failed with ${code}\n${output()}`));
    });
  });
}

function runPackageCommand(args) {
  if (process.platform === 'win32') {
    return runCommand('cmd', ['/c', 'npm', ...args]);
  }

  return runCommand('npm', args);
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}
