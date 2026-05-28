import { expect, it } from 'vitest';
import {
  ParentAssistantAdapterPayloadField,
  createParentAssistantRuntimeCommand,
  parseParentAssistantActionConfirmEvent,
  parseParentAssistantAnswerEvent,
  parseParentAssistantProviderStatusEvent,
  parseParentAssistantRunCancelEvent,
  parseParentAssistantThreadEvent,
} from '../src/parent-assistant-adapter';
import { AgentEvent, AgentProtocolDefaults } from '../src/contracts';

const Source = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const Target = {
  deviceId: 'local-dev-agent',
  platform: 'windows',
  route: 'localhost',
} as const;

it('creates runtime commands for portal message and action-preview handoff', () => {
  const message = createParentAssistantRuntimeCommand('message-send', commandInput());
  const preview = createParentAssistantRuntimeCommand('action-preview', commandInput());
  const provider = createParentAssistantRuntimeCommand('provider-status', commandInput());

  expect(message.command).toBe('agent.parent-assistant.message.send');
  expect(preview.command).toBe('agent.parent-assistant.action.preview');
  expect(provider.command).toBe('agent.parent-assistant.provider.status.get');
  expect(message.payload[AgentProtocolDefaults.Field.ParentAssistantQuestion]).toBe(
    'Suggest a policy rule from recent activity.'
  );
  expect(message.payload[AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]).toBe(
    'Recent Activity evidence is available.'
  );
});

it('creates runtime commands with typed Activity report JSON for report-backed citations', () => {
  const message = createParentAssistantRuntimeCommand('message-send', {
    ...commandInput(),
    activityReport: activityReport(),
  });
  const report = JSON.parse(String(message.payload[AgentProtocolDefaults.Field.ActivityReportDocument])) as {
    readonly savedMetadata: { readonly savedState: string };
  };

  expect(report.savedMetadata.savedState).toBe('saved');
});

it('creates thread, cancel, and confirm commands with stable payload fields', () => {
  const thread = createParentAssistantRuntimeCommand('thread-open', commandInput());
  const cancel = createParentAssistantRuntimeCommand('run-cancel', commandInput());
  const confirm = createParentAssistantRuntimeCommand('action-confirm', commandInput());

  expect(thread.command).toBe('agent.parent-assistant.thread.open');
  expect(cancel.payload[AgentProtocolDefaults.Field.ParentAssistantRunId]).toBe('parent-assistant-run-1');
  expect(confirm.payload[AgentProtocolDefaults.Field.ParentAssistantActionIntentId]).toBe(
    'parent-assistant-action-intent-1'
  );
});

it('parses full answer payloads with citations, preview, and API custody boundary', () => {
  const parsed = parseParentAssistantAnswerEvent(
    eventEnvelope(AgentEvent.ParentAssistantAnswerReported, {
      [AgentProtocolDefaults.Field.ParentAssistantAnswer]: JSON.stringify(answerPayload()),
    })
  );

  expect(parsed.ok).toBe(true);
  expect(parsed.ok ? parsed.value.answerState : null).toBe('unavailable');
  expect(parsed.ok ? parsed.value.citations.length : 0).toBe(1);
  expect(parsed.ok ? parsed.value.actionPreview.enforcementApplied : true).toBe(false);
  expect(parsed.ok ? parsed.value.apiProviderBoundary.authorizationState : null).toBe('not-authorized');
});

it('rejects wrong events and invalid answer JSON', () => {
  const wrong = parseParentAssistantAnswerEvent(eventEnvelope(AgentEvent.HealthReported, {}));
  const invalid = parseParentAssistantAnswerEvent(
    eventEnvelope(AgentEvent.ParentAssistantAnswerReported, {
      [AgentProtocolDefaults.Field.ParentAssistantAnswer]: '{',
    })
  );

  expect(wrong.ok).toBe(false);
  expect(invalid.ok).toBe(false);
});

it('parses thread runtime events', () => {
  const thread = parseParentAssistantThreadEvent(
    eventEnvelope(AgentEvent.ParentAssistantThreadUpdated, {
      [ParentAssistantAdapterPayloadField.ThreadResponse]: JSON.stringify(threadResponse()),
    })
  );

  expect(thread.ok ? thread.value.activeThread?.state : null).toBe('open');
});

it('parses provider status runtime events', () => {
  const provider = parseParentAssistantProviderStatusEvent(
    eventEnvelope(AgentEvent.ParentAssistantProviderDegraded, {
      [ParentAssistantAdapterPayloadField.ProviderStatus]: JSON.stringify(providerStatus()),
    })
  );

  expect(provider.ok ? provider.value.apiProviderBoundary.authorizationState : null).toBe('not-authorized');
});

it('parses run cancel runtime events', () => {
  const cancel = parseParentAssistantRunCancelEvent(
    eventEnvelope(AgentEvent.ParentAssistantErrorReported, {
      [ParentAssistantAdapterPayloadField.RunCancelResult]: JSON.stringify(runCancelResult()),
    })
  );

  expect(cancel.ok ? cancel.value.cancelState : null).toBe('not-running');
});

it('parses action confirm runtime events', () => {
  const confirm = parseParentAssistantActionConfirmEvent(
    eventEnvelope(AgentEvent.ParentAssistantActionConfirmed, {
      [ParentAssistantAdapterPayloadField.ActionConfirmResult]: JSON.stringify(actionConfirmResult()),
    })
  );

  expect(confirm.ok ? confirm.value.enforcementApplied : true).toBe(false);
});

function commandInput() {
  return {
    messageId: 'cmd-parent-assistant-1',
    sentAt: '2026-05-28T14:55:00Z',
    source: Source,
    target: Target,
    requestId: 'parent-assistant-request-local',
    threadId: 'parent-assistant-thread-1',
    runId: 'parent-assistant-run-1',
    actionIntentId: 'parent-assistant-action-intent-1',
    question: 'Suggest a policy rule from recent activity.',
    evidenceSummary: 'Recent Activity evidence is available.',
    maxOutputTokens: 120,
    timeoutMs: 1000,
  } as const;
}

function threadResponse() {
  const thread = {
    schemaVersion: 'v0.6',
    threadId: 'parent-assistant-thread-1',
    title: 'Parent Assistant local thread',
    state: 'open',
    backendState: 'volatile-local',
    createdAt: '2026-05-28T17:20:00Z',
    updatedAt: '2026-05-28T17:20:01Z',
    messageCount: 0,
  };

  return {
    schemaVersion: 'v0.6',
    backendState: 'volatile-local',
    activeThread: thread,
    threads: [thread],
    reason: 'Parent Assistant thread state is service-backed but volatile.',
  } as const;
}

function providerStatus() {
  return {
    schemaVersion: 'v0.6',
    backendState: 'runtime-backed',
    providerId: 'local-llama-cli',
    modelId: 'gemma-4-default',
    providerState: 'unavailable',
    schedulerJobStatus: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-runtime-unconfigured',
    queueDepth: 0,
    busy: false,
    apiProviderBoundary: answerPayload().apiProviderBoundary,
  } as const;
}

function runCancelResult() {
  return {
    schemaVersion: 'v0.6',
    backendState: 'runtime-backed',
    threadId: 'parent-assistant-thread-1',
    runId: 'parent-assistant-run-1',
    cancelState: 'not-running',
    providerState: 'unavailable',
    unavailableReason: 'parent-assistant-run-not-running',
  } as const;
}

function actionConfirmResult() {
  return {
    schemaVersion: 'v0.6',
    backendState: 'contract-required',
    actionIntentId: 'parent-assistant-action-intent-1',
    previewId: 'parent-assistant-action-preview-local',
    actionKind: 'policy-suggestion',
    confirmState: 'contract-required',
    requiresControllerLease: true,
    childAgentContractRequired: true,
    enforcementApplied: false,
    policyWritten: false,
    reason: 'Controller lease and child-agent policy contract are required before applying this action.',
  } as const;
}

function answerPayload() {
  const citation = {
    evidence: {
      evidenceReferenceId: 'activityDigest',
      kind: 'query-store-summary',
      observedAt: '2026-05-28T14:55:00Z',
    },
    citationLabel: 'Recent activity',
    allowedSummary: 'Recent Activity evidence is available.',
  };

  return {
    schemaVersion: 'v0.6',
    requestId: 'parent-assistant-request-local',
    threadId: 'parent-assistant-thread-local',
    messageId: 'parent-assistant-message-local',
    answeredAt: '2026-05-28T14:55:01Z',
    providerId: 'local-llama-cli',
    modelId: 'gemma-4-default',
    providerState: 'unavailable',
    answerState: 'unavailable',
    schedulerJobStatus: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-runtime-unconfigured',
    localAiResultId: null,
    answerText: null,
    citations: [citation],
    actionPreview: {
      previewId: 'parent-assistant-action-preview-local',
      actionKind: 'policy-suggestion',
      summary:
        'Policy suggestion preview only. Controller lease and child-agent contract execution are required before any rule changes.',
      actionReference: null,
      requiresControllerLease: true,
      childAgentContractRequired: true,
      enforcementApplied: false,
    },
    apiProviderBoundary: {
      schemaVersion: 'v0.6',
      providerId: 'api-provider-not-authorized',
      authorizationState: 'not-authorized',
      custodyLabel: 'parent-authorized-api-ai',
      retentionPolicy: 'no-retention-without-parent-authorization',
      deletionPolicy: 'delete-provider-cache-on-parent-request',
      citations: [citation],
      providerState: 'unavailable',
      unavailableReason: 'api-ai-provider-not-authorized',
      childSafetyOrEnforcementUseAllowed: false,
    },
    promptVersion: 'parent-assistant-local-v1',
  } as const;
}

function activityReport() {
  return {
    schemaVersion: 1,
    reportId: 'activity-report-daily-local',
    frequency: 'daily',
    scope: {
      scopeKind: 'family',
      familyId: 'family-local',
      deviceId: null,
    },
    requestedAt: '2026-05-28T14:54:00Z',
    rangeStart: '2026-05-28T00:00:00Z',
    rangeEnd: '2026-05-28T14:54:00Z',
    generatedAt: '2026-05-28T14:54:01Z',
    savedMetadata: {
      reportId: 'activity-report-daily-local',
      fileName: 'activity-report-daily-local.json',
      savedState: 'saved',
      savedAt: '2026-05-28T14:54:02Z',
      storageReason: 'Activity report is saved in local parent report storage.',
    },
    sourceStates: [
      {
        deviceId: 'local-dev-agent',
        reachabilityState: 'reachable',
        state: 'ready',
        reason: null,
        lastUpdatedAt: '2026-05-28T14:53:00Z',
      },
    ],
    sections: [
      {
        sectionKind: 'summary',
        title: 'Summary',
        state: 'ready',
        summary: 'Activity data is available from the local query store.',
        itemCount: 1,
        evidence: [],
      },
    ],
  } as const;
}

function eventEnvelope(event: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return {
    schemaVersion: 1,
    eventId: 'parent-assistant-event-1',
    correlationId: 'cmd-parent-assistant-1',
    sentAt: '2026-05-28T14:55:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: Source,
    event,
    severity: 'warn',
    payload,
    snapshot: null,
  } as const;
}
