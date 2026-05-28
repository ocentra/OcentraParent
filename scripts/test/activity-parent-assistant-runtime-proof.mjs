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
let savedActivityReport;

const ParentAssistantRuntimeField = {
  actionConfirmResult: 'parentAssistantActionConfirmResult',
  providerStatus: 'parentAssistantProviderStatus',
  runCancelResult: 'parentAssistantRunCancelResult',
  threadResponse: 'parentAssistantThreadResponse',
};

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
      messageId: 'cmd-parent-assistant-message',
      command: AgentCommand.ParentAssistantMessageSend,
      expectedEvent: AgentEvent.ParentAssistantAnswerReported,
      payload: parentAssistantPayload,
      assertEvent: assertParentAssistantUnavailable,
    },
    {
      messageId: 'cmd-parent-assistant-thread-create',
      command: AgentCommand.ParentAssistantThreadCreate,
      expectedEvent: AgentEvent.ParentAssistantThreadUpdated,
      payload: parentAssistantThreadPayload,
      assertEvent: assertParentAssistantThreadRuntime,
    },
    {
      messageId: 'cmd-parent-assistant-provider-status',
      command: AgentCommand.ParentAssistantProviderStatusGet,
      expectedEvent: AgentEvent.ParentAssistantProviderDegraded,
      payload: parentAssistantThreadPayload,
      assertEvent: assertParentAssistantProviderStatus,
    },
    {
      messageId: 'cmd-parent-assistant-run-cancel',
      command: AgentCommand.ParentAssistantRunCancel,
      expectedEvent: AgentEvent.ParentAssistantErrorReported,
      payload: parentAssistantRunPayload,
      assertEvent: assertParentAssistantRunCancel,
    },
    {
      messageId: 'cmd-parent-assistant-action-confirm',
      command: AgentCommand.ParentAssistantActionConfirm,
      expectedEvent: AgentEvent.ParentAssistantActionConfirmed,
      payload: parentAssistantActionPayload,
      assertEvent: assertParentAssistantActionConfirm,
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
      const payload = typeof step.payload === 'function' ? step.payload() : step.payload;
      socket.send(JSON.stringify(commandEnvelope(step.messageId, step.command, payload)));
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
  if (report.savedMetadata?.savedState !== 'draft' || report.savedMetadata.savedAt !== null) {
    throw new Error(`Activity report generation did not return an unsaved draft: ${JSON.stringify(report)}`);
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
  savedActivityReport = report;
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
  const answer = parseJsonField(payload, AgentProtocolDefaults.Field.ParentAssistantAnswer);
  if (answer.answerState !== 'unavailable' || !Array.isArray(answer.citations) || answer.citations.length < 1) {
    throw new Error(`Parent Assistant did not return a full typed answer payload: ${JSON.stringify(answer)}`);
  }
  const reportCitation = answer.citations.find((citation) => citation.citationLabel === 'Activity report');
  if (
    reportCitation?.evidence?.evidenceReferenceId !== savedActivityReport?.reportId ||
    !String(reportCitation?.allowedSummary).includes('savedState=saved')
  ) {
    throw new Error(`Parent Assistant did not cite the saved Activity report: ${JSON.stringify(answer.citations)}`);
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

function parentAssistantPayload() {
  if (savedActivityReport === undefined) {
    throw new Error('Parent Assistant proof reached report-backed step before saving Activity report');
  }

  return {
    [AgentProtocolDefaults.Field.ParentAssistantQuestion]: 'Suggest a policy rule from recent activity.',
    [AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]:
      'Recent local Activity tab data is available as parent-visible evidence.',
    [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(savedActivityReport),
  };
}

function parentAssistantThreadPayload() {
  return {
    [AgentProtocolDefaults.Field.ParentAssistantThreadId]: 'parent-assistant-thread-proof',
  };
}

function parentAssistantRunPayload() {
  return {
    [AgentProtocolDefaults.Field.ParentAssistantThreadId]: 'parent-assistant-thread-proof',
    [AgentProtocolDefaults.Field.ParentAssistantRunId]: 'parent-assistant-run-proof',
  };
}

function parentAssistantActionPayload() {
  return {
    [AgentProtocolDefaults.Field.ParentAssistantActionIntentId]: 'parent-assistant-action-intent-proof',
  };
}

function assertParentAssistantThreadRuntime(event) {
  const response = parseJsonField(event.payload, ParentAssistantRuntimeField.threadResponse);
  if (response.backendState !== 'volatile-local' || response.activeThread?.state !== 'open') {
    throw new Error(`Parent Assistant thread runtime did not return volatile local state: ${JSON.stringify(response)}`);
  }
  if (!Array.isArray(response.threads) || response.threads.length < 1) {
    throw new Error(`Parent Assistant thread runtime did not return thread list: ${JSON.stringify(response)}`);
  }
}

function assertParentAssistantProviderStatus(event) {
  const status = parseJsonField(event.payload, ParentAssistantRuntimeField.providerStatus);
  if (status.backendState !== 'runtime-backed' || status.providerState !== 'unavailable') {
    throw new Error(`Parent Assistant provider status was not runtime-backed unavailable: ${JSON.stringify(status)}`);
  }
  if (
    status.apiProviderBoundary?.authorizationState !== 'not-authorized' ||
    status.apiProviderBoundary?.childSafetyOrEnforcementUseAllowed !== false
  ) {
    throw new Error(`Parent Assistant provider status did not preserve API custody: ${JSON.stringify(status)}`);
  }
}

function assertParentAssistantRunCancel(event) {
  const result = parseJsonField(event.payload, ParentAssistantRuntimeField.runCancelResult);
  if (result.cancelState !== 'not-running' || result.backendState !== 'runtime-backed') {
    throw new Error(`Parent Assistant run cancel did not return typed no-active-run state: ${JSON.stringify(result)}`);
  }
}

function assertParentAssistantActionConfirm(event) {
  const result = parseJsonField(event.payload, ParentAssistantRuntimeField.actionConfirmResult);
  if (
    result.confirmState !== 'contract-required' ||
    result.enforcementApplied !== false ||
    result.policyWritten !== false ||
    result.childAgentContractRequired !== true
  ) {
    throw new Error(
      `Parent Assistant action confirm bypassed policy/child contract boundary: ${JSON.stringify(result)}`
    );
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
