import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
  AgentProtocolDefaults,
  deriveAgentBrowserRuntimeActionIntentStatus,
  parseAgentBrowserRuntimeEventChainStreamFields,
} from '../src/contracts';

const EvidenceObservedPayload = {
  phase: AgentBrowserRuntimePhase.EvidenceObserved,
  sourceRef: 'browser-source.managed-devtools',
  evidenceRef: 'browser-evidence.1',
  capabilityStatus: AgentBrowserRuntimeCapabilityStatus.TabListOnly,
  custodyLabel: AgentBrowserRuntimeCustodyLabel.ChildDeviceLocal,
  queryVisibility: AgentBrowserRuntimeQueryVisibility.LiveLocal,
  degradedReason: null,
  journalRef: 'browser-journal.1',
  aiRequestRef: null,
  aiAnalysisRef: null,
  policyEvaluationRef: null,
  policyDecisionRef: null,
  policyPreviewId: null,
  assistantActionIntentId: null,
  interventionCommandRef: null,
  interventionResultRef: null,
  auditEntryRef: 'browser-audit.1',
  readModelRef: 'browser-read-model.1',
  previousPhaseRef: null,
  exactUrlClaimed: true,
  aiAuthority: false,
  policyAuthority: false,
  dryRun: false,
  adapterDispatchClaimed: false,
  interventionCommandAllowed: false,
  observedAt: '2026-06-07T19:30:00Z',
} as const;

const EvidenceJournaledPayload = {
  ...EvidenceObservedPayload,
  phase: AgentBrowserRuntimePhase.EvidenceJournaled,
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.evidence.observed',
} as const;

const AuditCommittedPayload = {
  ...EvidenceObservedPayload,
  phase: AgentBrowserRuntimePhase.AuditEntryCommitted,
  exactUrlClaimed: false,
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.evidence.journaled',
} as const;

const ReadModelProjectedPayload = {
  ...AuditCommittedPayload,
  phase: AgentBrowserRuntimePhase.ReadModelProjected,
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.audit-entry.committed',
} as const;

const StaleBridgePayload = {
  ...EvidenceObservedPayload,
  capabilityStatus: AgentBrowserRuntimeCapabilityStatus.Stale,
  degradedReason: 'browser-bridge-stale-session',
  exactUrlClaimed: false,
} as const;

const UnsupportedLaterAdapterPayload = {
  ...EvidenceObservedPayload,
  capabilityStatus: AgentBrowserRuntimeCapabilityStatus.UnsupportedBrowser,
  queryVisibility: AgentBrowserRuntimeQueryVisibility.Unavailable,
  degradedReason: 'windows-unsupported-later-adapter',
  exactUrlClaimed: false,
} as const;

const PolicyDecisionPayload = {
  ...EvidenceObservedPayload,
  phase: AgentBrowserRuntimePhase.PolicyDecisionCompleted,
  policyEvaluationRef: 'browser-policy-evaluation-ref-test',
  policyDecisionRef: 'browser-policy-decision-ref-test',
  policyPreviewId: 'browser-policy-preview-test',
  assistantActionIntentId: 'browser-action-intent-test',
  exactUrlClaimed: true,
  policyAuthority: true,
  dryRun: true,
  adapterDispatchClaimed: false,
  previousPhaseRef:
    'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.policy-evaluation.requested',
} as const;

describe('agent browser runtime event contracts', () => {
  it('parses service-backed browser runtime stream fields', specifyStreamParsing);
  it('parses dry-run policy action handoff without adapter dispatch', specifyDryRunActionHandoffParsing);
  it('derives pending action-intent subscriber status from dry-run stream entries', specifyActionIntentStatus);
  it('rejects mismatched phases, overclaims, invalid json, and count drift', specifyRejections);
});

function specifyStreamParsing() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(streamFields());

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  expect(parsed.value.observedRows).toBe(1);
  expect(parsed.value.streamedEvents).toBe(4);
  expect(parsed.value.manualRequiredRows).toBe(1);
  expect(parsed.value.interventionCommandEvents).toBe(0);
  expect(parsed.value.actionIntentCandidates).toBe(0);
  expect(parsed.value.actionIntentDispatchAttempts).toBe(0);
  expect(parsed.value.actionIntentAdapterExecutions).toBe(0);
  expect(parsed.value.actionIntentChildInterventionExecutions).toBe(0);
  expect(parsed.value.actionIntentEnforcementExecutions).toBe(0);
  expect(parsed.value.entries.map((entry) => entry.eventType)).toEqual([
    AgentBrowserRuntimeEventType.EvidenceObserved,
    AgentBrowserRuntimeEventType.EvidenceJournaled,
    AgentBrowserRuntimeEventType.AuditEntryCommitted,
    AgentBrowserRuntimeEventType.ReadModelProjected,
  ]);
  expect(parsed.value.entries.at(0)?.payload.phase).toBe(AgentBrowserRuntimePhase.EvidenceObserved);
  expect(parsed.value.entries.at(0)?.payload.aiAuthority).toBe(false);
  expect(parsed.value.entries.at(0)?.payload.capabilityStatus).toBe(AgentBrowserRuntimeCapabilityStatus.TabListOnly);
  expect(parsed.value.entries.at(0)?.payload.queryVisibility).toBe(AgentBrowserRuntimeQueryVisibility.LiveLocal);
  expect(parsed.value.entries.at(0)?.payload.interventionCommandAllowed).toBe(false);
  expect(parsed.value.entries.at(0)?.payload.dryRun).toBe(false);
  expect(parsed.value.entries.at(0)?.payload.adapterDispatchClaimed).toBe(false);
}

function specifyDryRunActionHandoffParsing() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields([entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionPayload)], {
      actionIntentCandidates: 1,
    })
  );

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  expect(parsed.value.entries.at(0)?.payload.policyPreviewId).toBe('browser-policy-preview-test');
  expect(parsed.value.entries.at(0)?.payload.assistantActionIntentId).toBe('browser-action-intent-test');
  expect(parsed.value.entries.at(0)?.payload.dryRun).toBe(true);
  expect(parsed.value.entries.at(0)?.payload.adapterDispatchClaimed).toBe(false);
  expect(parsed.value.entries.at(0)?.payload.interventionCommandAllowed).toBe(false);
}

function specifyActionIntentStatus() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields(
      [
        entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionPayload),
        entry(AgentBrowserRuntimeEventType.ReadModelProjected, ReadModelProjectedPayload),
      ],
      { actionIntentCandidates: 1 }
    )
  );

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  const status = deriveAgentBrowserRuntimeActionIntentStatus(parsed.value);
  expect(status).toMatchObject({
    candidateCount: 1,
    dispatchAttemptCount: 0,
    adapterExecutionCount: 0,
    childInterventionExecutionCount: 0,
    enforcementExecutionCount: 0,
    dryRunOnly: true,
    policyAuthorityOnly: true,
  });
  expect(status.candidates).toEqual([
    {
      eventRef: `event-ref-${AgentBrowserRuntimeEventType.PolicyDecisionCompleted}`,
      policyPreviewId: 'browser-policy-preview-test',
      assistantActionIntentId: 'browser-action-intent-test',
      sourceRef: 'browser-source.managed-devtools',
      evidenceRef: 'browser-evidence.1',
      observedAt: '2026-06-07T19:30:00Z',
    },
  ]);

  const emptyParsed = parseAgentBrowserRuntimeEventChainStreamFields(streamFields());
  expect(emptyParsed.ok).toBe(true);
  if (!emptyParsed.ok) {
    return;
  }
  const emptyStatus = deriveAgentBrowserRuntimeActionIntentStatus(emptyParsed.value);
  expect(emptyStatus.candidateCount).toBe(0);
}

function specifyRejections() {
  specifyStreamEnvelopeRejections();
  specifyRuntimePayloadRejections();
  specifyActionIntentOverclaimRejections();
}

function specifyStreamEnvelopeRejections() {
  expect(parseAgentBrowserRuntimeEventChainStreamFields({})).toEqual({
    ok: false,
    reason: 'missing-json-field',
  });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields({
      ...streamFields(),
      [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: 'not-json',
    })
  ).toEqual({ ok: false, reason: 'invalid-json' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([entry(AgentBrowserRuntimeEventType.ReadModelProjected, EvidenceObservedPayload)])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields({
      ...streamFields(),
      [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: 5,
    })
  ).toEqual({ ok: false, reason: 'invalid-stream' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields({
      ...streamFields(),
      [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: 1,
    })
  ).toEqual({ ok: false, reason: 'invalid-stream' });
}

function specifyRuntimePayloadRejections() {
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.EvidenceObserved, {
          ...EvidenceObservedPayload,
          aiAuthority: true,
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.EvidenceObserved, {
          ...EvidenceObservedPayload,
          capabilityStatus: AgentBrowserRuntimeCapabilityStatus.BridgeMissing,
          queryVisibility: AgentBrowserRuntimeQueryVisibility.Unavailable,
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.EvidenceObserved, {
          ...EvidenceObservedPayload,
          exactUrlClaimed: false,
          capabilityStatus: AgentBrowserRuntimeCapabilityStatus.BridgeMissing,
          queryVisibility: AgentBrowserRuntimeQueryVisibility.Unavailable,
          degradedReason: 'browser-bridge-no-page-targets',
        }),
      ])
    ).ok
  ).toBe(true);
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([entry(AgentBrowserRuntimeEventType.EvidenceObserved, StaleBridgePayload)])
    ).ok
  ).toBe(true);
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([entry(AgentBrowserRuntimeEventType.EvidenceObserved, UnsupportedLaterAdapterPayload)])
    ).ok
  ).toBe(true);
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.EvidenceObserved, {
          ...StaleBridgePayload,
          exactUrlClaimed: true,
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.EvidenceObserved, {
          ...UnsupportedLaterAdapterPayload,
          exactUrlClaimed: true,
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
}

function specifyActionIntentOverclaimRejections() {
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, {
          ...PolicyDecisionPayload,
          adapterDispatchClaimed: true,
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields([
        entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, {
          ...PolicyDecisionPayload,
          interventionCommandAllowed: true,
          interventionCommandRef: 'browser-intervention-command-ref-test',
        }),
      ])
    )
  ).toEqual({ ok: false, reason: 'invalid-entry' });
}

function streamFields(
  entries = validEntries(),
  counters: {
    readonly actionIntentCandidates?: number;
    readonly actionIntentDispatchAttempts?: number;
    readonly actionIntentAdapterExecutions?: number;
    readonly actionIntentChildInterventionExecutions?: number;
    readonly actionIntentEnforcementExecutions?: number;
  } = {}
) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: counters.actionIntentCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]:
      counters.actionIntentDispatchAttempts ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]:
      counters.actionIntentAdapterExecutions ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]:
      counters.actionIntentChildInterventionExecutions ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]:
      counters.actionIntentEnforcementExecutions ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  };
}

function validEntries() {
  return [
    entry(AgentBrowserRuntimeEventType.EvidenceObserved, EvidenceObservedPayload),
    entry(AgentBrowserRuntimeEventType.EvidenceJournaled, EvidenceJournaledPayload),
    entry(AgentBrowserRuntimeEventType.AuditEntryCommitted, AuditCommittedPayload),
    entry(AgentBrowserRuntimeEventType.ReadModelProjected, ReadModelProjectedPayload),
  ];
}

function entry(eventType: AgentBrowserRuntimeEventType, payload: unknown) {
  return {
    [AgentProtocolDefaults.Field.EventType]: eventType,
    [AgentProtocolDefaults.Field.EventRef]: `event-ref-${eventType}`,
    [AgentProtocolDefaults.Field.Payload]: payload,
  };
}
