import {
  AgentEventEnvelopeSchema,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/schema-domain/activity-surface';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
} from '@ocentra-parent/schema-domain/agent-browser-runtime-events';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { ActivityEventKind } from '@ocentra-parent/schema-domain/evidence-kinds';

export type BrowserRuntimeStreamEntry = ReturnType<typeof browserRuntimeStreamEntry>;

export type BrowserRuntimeEventChainStreamEventInput = {
  readonly entries?: readonly BrowserRuntimeStreamEntry[];
  readonly streamedEvents?: number;
  readonly actionIntentCandidates?: number;
  readonly actionIntentHandoffCandidates?: number;
  readonly actionIntentHandoffOutboxRefs?: readonly string[];
  readonly actionIntentHandoffRefs?: readonly string[];
  readonly actionIntentChildAcceptedRows?: number;
  readonly actionIntentChildCommandRefs?: readonly string[];
  readonly actionIntentChildAcceptedEventRefs?: readonly string[];
  readonly actionIntentParentReadModelRefs?: readonly string[];
  readonly actionIntentDispatchAttempts?: number;
  readonly socialProviderReceiptBoundaryRows?: number;
  readonly socialProviderDispatchRequiredRows?: number;
  readonly socialProviderManualReceiptRequiredRows?: number;
  readonly socialProviderAttemptRefs?: readonly string[];
  readonly socialProviderReceiptProofRefs?: readonly string[];
  readonly socialProviderDurableRows?: number;
  readonly socialProviderDurableResultRefs?: readonly string[];
  readonly socialProviderDurableStoreRefs?: readonly string[];
  readonly socialProviderReadModelRefs?: readonly string[];
  readonly socialProviderSupportStatusRefs?: readonly string[];
};

const NoClaimBoundary = {
  exactUrlAvailable: false,
  decryptedHttpsPayloadAvailable: false,
  messageContentAvailable: false,
  searchQueryAvailable: false,
  adapterActionExecuted: false,
} as const;

export const FlowObserved = {
  schemaVersion: 1,
  flowEventRef: 'event.network.flow.observed.1',
  observedAt: '2026-06-08T22:45:00Z',
  deviceRef: 'device.child.windows-1',
  flowEvidenceRef: 'evidence.network.flow.1',
  custody: 'child-device-query-store',
  evidenceGrade: 'A',
  claimBoundary: NoClaimBoundary,
} as const;

export function recentSummaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T18:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.recent.summary.reported',
    severity: 'info',
    payload: {
      limit: 25,
      returned: 1,
      firstObservedAt: '2026-05-20T18:44:59Z',
      lastObservedAt: '2026-05-20T18:44:59Z',
      lastEventId: 'activity-event-1',
      mostRecentKind: ActivityEventKind.ProcessObserved,
      mostRecentObserver: 'windows-process',
      mostRecentSubjectKind: 'process',
      mostRecentSubjectId: 'process-1',
      mostRecentSubjectName: 'notepad.exe',
    },
    snapshot: null,
  });
}

export function ingestStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-ingest',
    correlationId: 'cmd-ingest',
    sentAt: '2026-05-20T18:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.ingest.status.reported',
    severity: 'info',
    payload: {
      databaseReady: true,
      eventsIngested: 0,
      eventsStored: 1,
      duplicateEvents: 0,
      lastEventId: 'activity-event-1',
    },
    snapshot: null,
  });
}

export function browserEvidenceEvent(
  eventId = 'evt-browser',
  url = 'https://example.test/learn',
  sentAt = '2026-05-21T01:00:01Z'
) {
  const origin = new URL(url).origin;
  const domain = new URL(url).hostname;

  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId,
    correlationId: 'cmd-browser',
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.evidence.recent.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-url-observed-1',
      latestObservedAt: '2026-05-21T01:00:00Z',
      browserEvidenceId: 'browser-evidence-1',
      sourceId: 'managed-chromium-devtools',
      adapterId: 'managed-chromium-devtools-adapter',
      managedBrowserSessionId: 'managed-browser-session-1',
      browserFamily: 'edge',
      browserChannel: 'stable',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      windowId: null,
      tabId: null,
      targetId: 'target-1',
      activeState: 'unknown',
      activeProofSource: 'target-list-only',
      url,
      origin,
      domain,
      title: 'Example learning page',
      freshUntil: '2026-05-21T01:00:30Z',
      staleAt: '2026-05-21T01:00:30Z',
      capabilityStatus: 'tab-list-only',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
    },
    snapshot: null,
  });
}

export function appGameAdapterDispatchExecutedEvent(eventId: string, commandId: string) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId,
    correlationId: commandId,
    sentAt: commandId === 'latest-execute-command' ? '2026-06-08T12:45:02Z' : '2026-06-08T12:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.app-game.adapter-dispatch.executed',
    severity: 'info',
    payload: {
      appGameAdapterDispatchExecuteResult: JSON.stringify({
        schemaVersion: 1,
        commandId,
        generatedAt: '2026-06-08T12:45:00Z',
        sourceReadModelId: 'app-game-adapter-dispatch-result',
        sourceDispatchRowId: 'app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit',
        sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
        executionCommandName: 'agent.enforcement.execute',
        executionEventName: 'agent.enforcement.audit.reported',
        executionResultId: 'enforcement-result-app-game-owned-process',
        executionStatus: 'actually-enforced',
        executionAdapterResultCode: 'process-already-exited',
        executionAuditEventId: 'enforcement-audit-app-game-owned-process',
        readbackCommandName: 'agent.activity.app-game.adapter-dispatch-result.read-model.get',
        adapterDispatchExecutedClaimed: true,
        broadInstalledAppBlockingClaimed: false,
        childDeviceDeliveryClaimed: false,
        platformEnforcementClaimed: false,
        providerDeliveryClaimed: false,
        privateDiagnosticsClaimed: false,
      }),
    },
    snapshot: null,
  });
}

export function activityReportEvent(input: {
  readonly eventId: unknown;
  readonly event: AgentEventName;
  readonly reportId: unknown;
  readonly sentAt?: string;
}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: input.eventId,
    correlationId: 'cmd-report',
    sentAt: input.sentAt ?? '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: input.event,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
      [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify({
        schemaVersion: ActivitySurfaceSchemaVersion,
        reportId: input.reportId,
        frequency: 'daily',
        scope: {
          scopeKind: 'device',
          familyId: null,
          deviceId: 'local-dev-agent',
        },
        requestedAt: '2026-05-21T01:00:00Z',
        rangeStart: '2026-05-21T00:00:00Z',
        rangeEnd: '2026-05-21T01:00:00Z',
        generatedAt: '2026-05-21T01:00:01Z',
        savedMetadata: null,
        sourceStates: [
          {
            deviceId: 'local-dev-agent',
            reachabilityState: 'reachable',
            state: 'ready',
            reason: null,
            lastUpdatedAt: '2026-05-21T01:00:00Z',
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
      }),
    },
    snapshot: null,
  });
}

export function browserInventoryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-inventory',
    correlationId: 'cmd-browser-inventory',
    sentAt: '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.inventory.read-model.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 20,
      returned: 1,
      latestObservedAt: '2026-05-21T01:00:00Z',
      capabilityStatus: 'tab-list-only',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
      browserInventoryRowId: 'browser-inventory-row-1',
      browserFamily: 'edge',
      browserChannel: 'stable',
      productName: 'Microsoft Edge',
      browserVersion: '124.0.0.0',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      executablePathRef: 'managed-edge-path-ref',
      installState: 'installed',
      runningState: 'running-managed',
      managementTier: 'managed',
      supportTier: 'managed-target-list',
      exactUrlCapability: 'managed-target-list-only',
      activeTabCapability: 'target-list-only',
      managedProfileState: 'ready',
      unmanagedFallbackCapability: 'report-only',
      reason: 'managed-target-list-only',
      publisherSignatureRef: null,
      fileHashRef: null,
    },
    snapshot: null,
  });
}

export function browserRuntimeEventChainStreamEvent(input: BrowserRuntimeEventChainStreamEventInput = {}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-runtime-stream',
    correlationId: 'cmd-browser-runtime-stream',
    sentAt: '2026-05-21T01:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.runtime.event-chain.stream.reported',
    severity: 'info',
    payload: browserRuntimeEventChainStreamPayload(input),
    snapshot: null,
  });
}

function browserRuntimeEventChainStreamPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  const entries = input.entries ?? defaultBrowserRuntimeStreamEntries();
  return {
    ...browserRuntimeCounterPayload(input, entries),
    ...browserRuntimeActionIntentPayload(input),
    ...browserRuntimeSocialProviderReceiptPayload(input),
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  };
}

function defaultBrowserRuntimeStreamEntries(): readonly BrowserRuntimeStreamEntry[] {
  return [
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.EvidenceObserved,
      'cmd-browser-runtime-stream-browser.evidence.observed'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.EvidenceJournaled,
      'cmd-browser-runtime-stream-browser.evidence.journaled'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.AuditEntryCommitted,
      'cmd-browser-runtime-stream-browser.audit-entry.committed'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.ReadModelProjected,
      'cmd-browser-runtime-stream-browser.read-model.projected'
    ),
  ];
}

function browserRuntimeCounterPayload(
  input: BrowserRuntimeEventChainStreamEventInput,
  entries: readonly BrowserRuntimeStreamEntry[]
) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: input.streamedEvents ?? entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
  };
}

function browserRuntimeActionIntentPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: input.actionIntentCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]: input.actionIntentHandoffCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify(
      input.actionIntentHandoffOutboxRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs]: JSON.stringify(
      input.actionIntentHandoffRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedRows]: input.actionIntentChildAcceptedRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildCommandRefs]: JSON.stringify(
      input.actionIntentChildCommandRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedEventRefs]: JSON.stringify(
      input.actionIntentChildAcceptedEventRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentParentReadModelRefs]: JSON.stringify(
      input.actionIntentParentReadModelRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: input.actionIntentDispatchAttempts ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]: 0,
  };
}

function browserRuntimeSocialProviderReceiptPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]:
      input.socialProviderReceiptBoundaryRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]:
      input.socialProviderDispatchRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]:
      input.socialProviderManualReceiptRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify(
      input.socialProviderAttemptRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify(
      input.socialProviderReceiptProofRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: input.socialProviderDurableRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify(
      input.socialProviderDurableResultRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify(
      input.socialProviderDurableStoreRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify(
      input.socialProviderReadModelRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify(
      input.socialProviderSupportStatusRefs ?? []
    ),
  };
}

export function browserRuntimeStreamEntry(
  eventType: AgentBrowserRuntimeEventType,
  eventRef: string,
  payloadOverrides: Partial<{
    readonly phase: AgentBrowserRuntimePhase;
    readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
    readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
    readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
    readonly degradedReason: string | null;
    readonly exactUrlClaimed: boolean;
    readonly aiAuthority: boolean;
    readonly policyPreviewId: string | null;
    readonly assistantActionIntentId: string | null;
    readonly dryRun: boolean;
  }> = {}
) {
  return {
    [AgentProtocolDefaults.Field.EventType]: eventType,
    [AgentProtocolDefaults.Field.EventRef]: eventRef,
    [AgentProtocolDefaults.Field.Payload]: {
      phase: payloadOverrides.phase ?? browserRuntimePhaseForEventType(eventType),
      sourceRef: 'browser-runtime-source-ref',
      evidenceRef: 'browser-runtime-evidence-ref',
      capabilityStatus: payloadOverrides.capabilityStatus ?? AgentBrowserRuntimeCapabilityStatus.BridgeMissing,
      custodyLabel: payloadOverrides.custodyLabel ?? AgentBrowserRuntimeCustodyLabel.ChildDeviceLocal,
      queryVisibility: payloadOverrides.queryVisibility ?? AgentBrowserRuntimeQueryVisibility.Unavailable,
      degradedReason: payloadOverrides.degradedReason ?? 'browser-bridge-no-page-targets',
      journalRef: 'browser-runtime-journal-ref',
      aiRequestRef: null,
      aiAnalysisRef: null,
      policyEvaluationRef: null,
      policyDecisionRef: null,
      policyPreviewId: payloadOverrides.policyPreviewId ?? null,
      assistantActionIntentId: payloadOverrides.assistantActionIntentId ?? null,
      interventionCommandRef: null,
      interventionResultRef: null,
      auditEntryRef: 'browser-runtime-audit-ref',
      readModelRef: 'browser-runtime-read-model-ref',
      previousPhaseRef: 'browser-runtime-previous-phase-ref',
      exactUrlClaimed: payloadOverrides.exactUrlClaimed ?? false,
      aiAuthority: payloadOverrides.aiAuthority ?? false,
      policyAuthority: true,
      dryRun: payloadOverrides.dryRun ?? false,
      adapterDispatchClaimed: false,
      interventionCommandAllowed: false,
      observedAt: '2026-05-21T01:00:00Z',
    },
  };
}

function browserRuntimePhaseForEventType(eventType: AgentBrowserRuntimeEventType): AgentBrowserRuntimePhase {
  switch (eventType) {
    case AgentBrowserRuntimeEventType.EvidenceObserved:
      return AgentBrowserRuntimePhase.EvidenceObserved;
    case AgentBrowserRuntimeEventType.EvidenceJournaled:
      return AgentBrowserRuntimePhase.EvidenceJournaled;
    case AgentBrowserRuntimeEventType.AiAnalysisRequested:
      return AgentBrowserRuntimePhase.AiAnalysisRequested;
    case AgentBrowserRuntimeEventType.AiAnalysisCompleted:
      return AgentBrowserRuntimePhase.AiAnalysisCompleted;
    case AgentBrowserRuntimeEventType.PolicyEvaluationRequested:
      return AgentBrowserRuntimePhase.PolicyEvaluationRequested;
    case AgentBrowserRuntimeEventType.PolicyDecisionCompleted:
      return AgentBrowserRuntimePhase.PolicyDecisionCompleted;
    case AgentBrowserRuntimeEventType.InterventionCommandIssued:
      return AgentBrowserRuntimePhase.InterventionCommandIssued;
    case AgentBrowserRuntimeEventType.InterventionResultObserved:
      return AgentBrowserRuntimePhase.InterventionResultObserved;
    case AgentBrowserRuntimeEventType.AuditEntryCommitted:
      return AgentBrowserRuntimePhase.AuditEntryCommitted;
    case AgentBrowserRuntimeEventType.ReadModelProjected:
      return AgentBrowserRuntimePhase.ReadModelProjected;
  }
}

export function eventWithPayload(
  event: AgentEventEnvelope['event'],
  payload: AgentEventEnvelope['payload']
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'portal-live-activity-event',
    correlationId: 'portal-live-activity-correlation',
    sentAt: '2026-06-08T22:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
