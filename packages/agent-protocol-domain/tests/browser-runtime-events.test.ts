import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
  AgentProtocolDefaults,
  deriveAgentBrowserRuntimeActionIntentStatus,
  deriveAgentBrowserRuntimeSocialProviderReceiptStatus,
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
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.audit.entry.committed',
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
    'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.policy.evaluation.requested',
} as const;

const FinalPolicyDecisionPayload = {
  ...PolicyDecisionPayload,
  policyPreviewId: null,
  assistantActionIntentId: null,
  dryRun: false,
} as const;

const AiAnalysisRequestedPayload = {
  ...EvidenceObservedPayload,
  phase: AgentBrowserRuntimePhase.AiAnalysisRequested,
  aiRequestRef: 'browser-ai-request-ref-test',
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.evidence.journaled',
} as const;

const AiAnalysisCompletedPayload = {
  ...AiAnalysisRequestedPayload,
  phase: AgentBrowserRuntimePhase.AiAnalysisCompleted,
  aiAnalysisRef: 'browser-ai-analysis-ref-test',
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.ai.analysis.requested',
} as const;

const PolicyEvaluationPayload = {
  ...AiAnalysisCompletedPayload,
  phase: AgentBrowserRuntimePhase.PolicyEvaluationRequested,
  policyEvaluationRef: 'browser-policy-evaluation-ref-test',
  policyAuthority: true,
  previousPhaseRef: 'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.ai.analysis.completed',
} as const;

const InterventionCommandPayload = {
  ...FinalPolicyDecisionPayload,
  phase: AgentBrowserRuntimePhase.InterventionCommandIssued,
  adapterDispatchClaimed: true,
  interventionCommandAllowed: true,
  interventionCommandRef: 'browser-intervention-command-ref-test',
  previousPhaseRef:
    'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.policy.decision.completed',
} as const;

const InterventionResultPayload = {
  ...InterventionCommandPayload,
  phase: AgentBrowserRuntimePhase.InterventionResultObserved,
  interventionResultRef: 'browser-intervention-result-ref-test',
  previousPhaseRef:
    'browser-runtime-correlation-browser-evidence.1-2026-06-07T19:30:00Z-browser.intervention.command.issued',
} as const;

describe('agent browser runtime event contracts', () => {
  it('parses service-backed browser runtime stream fields', specifyStreamParsing);
  it('parses every Rust browser runtime event type without name drift', specifyRustEventNameParity);
  it('parses dry-run policy action handoff without adapter dispatch', specifyDryRunActionHandoffParsing);
  it('derives pending action-intent subscriber status from dry-run stream entries', specifyActionIntentStatus);
  it('derives social provider receipt boundary status from public stream fields', specifySocialProviderReceiptStatus);
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
  expect(parsed.value.actionIntentHandoffCandidates).toBe(0);
  expect(parsed.value.actionIntentHandoffOutboxRefs).toEqual([]);
  expect(parsed.value.actionIntentHandoffRefs).toEqual([]);
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

function specifyRustEventNameParity() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields([
      entry(AgentBrowserRuntimeEventType.EvidenceObserved, EvidenceObservedPayload),
      entry(AgentBrowserRuntimeEventType.EvidenceJournaled, EvidenceJournaledPayload),
      entry(AgentBrowserRuntimeEventType.AiAnalysisRequested, AiAnalysisRequestedPayload),
      entry(AgentBrowserRuntimeEventType.AiAnalysisCompleted, AiAnalysisCompletedPayload),
      entry(AgentBrowserRuntimeEventType.PolicyEvaluationRequested, PolicyEvaluationPayload),
      entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, FinalPolicyDecisionPayload),
      entry(AgentBrowserRuntimeEventType.InterventionCommandIssued, InterventionCommandPayload),
      entry(AgentBrowserRuntimeEventType.InterventionResultObserved, InterventionResultPayload),
      entry(AgentBrowserRuntimeEventType.AuditEntryCommitted, AuditCommittedPayload),
      entry(AgentBrowserRuntimeEventType.ReadModelProjected, ReadModelProjectedPayload),
    ])
  );

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  expect(parsed.value.entries.map((entry) => entry.eventType)).toEqual([
    'browser.evidence.observed',
    'browser.evidence.journaled',
    'browser.ai.analysis.requested',
    'browser.ai.analysis.completed',
    'browser.policy.evaluation.requested',
    'browser.policy.decision.completed',
    'browser.intervention.command.issued',
    'browser.intervention.result.observed',
    'browser.audit.entry.committed',
    'browser.read-model.projected',
  ]);
  expect(parsed.value.entries.map((entry) => entry.payload.phase)).toEqual([
    AgentBrowserRuntimePhase.EvidenceObserved,
    AgentBrowserRuntimePhase.EvidenceJournaled,
    AgentBrowserRuntimePhase.AiAnalysisRequested,
    AgentBrowserRuntimePhase.AiAnalysisCompleted,
    AgentBrowserRuntimePhase.PolicyEvaluationRequested,
    AgentBrowserRuntimePhase.PolicyDecisionCompleted,
    AgentBrowserRuntimePhase.InterventionCommandIssued,
    AgentBrowserRuntimePhase.InterventionResultObserved,
    AgentBrowserRuntimePhase.AuditEntryCommitted,
    AgentBrowserRuntimePhase.ReadModelProjected,
  ]);
}

function specifyDryRunActionHandoffParsing() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields([entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionPayload)], {
      actionIntentCandidates: 1,
      actionIntentHandoffCandidates: 1,
      actionIntentHandoffOutboxRefs: ['browser-action-intent-outbox-ref-test'],
      actionIntentHandoffRefs: ['browser-action-intent-handoff-ref-test'],
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
  expect(parsed.value.actionIntentHandoffCandidates).toBe(1);
  expect(parsed.value.actionIntentHandoffOutboxRefs).toEqual(['browser-action-intent-outbox-ref-test']);
  expect(parsed.value.actionIntentHandoffRefs).toEqual(['browser-action-intent-handoff-ref-test']);
}

function specifyActionIntentStatus() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields(
      [
        entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionPayload),
        entry(AgentBrowserRuntimeEventType.ReadModelProjected, ReadModelProjectedPayload),
      ],
      {
        actionIntentCandidates: 1,
        actionIntentHandoffCandidates: 1,
        actionIntentHandoffOutboxRefs: ['browser-action-intent-outbox-ref-test'],
        actionIntentHandoffRefs: ['browser-action-intent-handoff-ref-test'],
      }
    )
  );

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  const status = deriveAgentBrowserRuntimeActionIntentStatus(parsed.value);
  expect(status).toMatchObject({
    candidateCount: 1,
    handoffCandidateCount: 1,
    handoffOutboxRefs: ['browser-action-intent-outbox-ref-test'],
    handoffRefs: ['browser-action-intent-handoff-ref-test'],
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
  expect(emptyStatus.handoffCandidateCount).toBe(0);
  expect(emptyStatus.handoffOutboxRefs).toEqual([]);
  expect(emptyStatus.handoffRefs).toEqual([]);
}

function specifySocialProviderReceiptStatus() {
  const parsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields([entry(AgentBrowserRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionPayload)], {
      actionIntentCandidates: 1,
      actionIntentHandoffCandidates: 1,
      actionIntentHandoffOutboxRefs: ['browser-action-intent-outbox-ref-test'],
      actionIntentHandoffRefs: ['browser-action-intent-handoff-ref-test'],
      socialProviderReceiptBoundaryRows: 1,
      socialProviderDispatchRequiredRows: 1,
      socialProviderAttemptRefs: ['browser-social-provider-attempt-ref-test'],
      socialProviderReceiptProofRefs: ['browser-social-provider-receipt-proof-ref-test'],
      socialProviderDurableRows: 1,
      socialProviderDurableResultRefs: ['browser-social-provider-durable-result-ref-test'],
      socialProviderDurableStoreRefs: ['browser-social-provider-durable-store-ref-test'],
      socialProviderReadModelRefs: ['browser-social-provider-read-model-ref-test'],
      socialProviderSupportStatusRefs: ['browser-social-provider-support-status-ref-test'],
    })
  );

  expect(parsed.ok).toBe(true);
  if (!parsed.ok) {
    return;
  }

  const status = deriveAgentBrowserRuntimeSocialProviderReceiptStatus(parsed.value);
  expect(status).toMatchObject({
    receiptBoundaryRows: 1,
    providerDispatchRequiredRows: 1,
    manualReceiptRequiredRows: 0,
    providerAttemptRefs: ['browser-social-provider-attempt-ref-test'],
    providerReceiptProofRefs: ['browser-social-provider-receipt-proof-ref-test'],
    durableRows: 1,
    durableResultRefs: ['browser-social-provider-durable-result-ref-test'],
    durableStoreRefs: ['browser-social-provider-durable-store-ref-test'],
    readModelRefs: ['browser-social-provider-read-model-ref-test'],
    supportStatusRefs: ['browser-social-provider-support-status-ref-test'],
    providerDeliveryClaimed: false,
    receiptIngestionClaimed: false,
    parentNotificationDeliveryClaimed: false,
    reportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });

  const manualParsed = parseAgentBrowserRuntimeEventChainStreamFields(
    streamFields(undefined, {
      socialProviderReceiptBoundaryRows: 1,
      socialProviderManualReceiptRequiredRows: 1,
    })
  );
  expect(manualParsed.ok).toBe(true);
  if (!manualParsed.ok) {
    return;
  }
  expect(deriveAgentBrowserRuntimeSocialProviderReceiptStatus(manualParsed.value)).toMatchObject({
    receiptBoundaryRows: 1,
    providerDispatchRequiredRows: 0,
    manualReceiptRequiredRows: 1,
    durableRows: 0,
    providerDeliveryClaimed: false,
  });
}

function specifyRejections() {
  specifyStreamEnvelopeRejections();
  specifyRuntimePayloadRejections();
  specifyActionIntentOverclaimRejections();
  specifySocialProviderReceiptOverclaimRejections();
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
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields({
      ...streamFields(),
      [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]: 0,
      [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify([
        'browser-action-intent-outbox-ref-test',
      ]),
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

function specifySocialProviderReceiptOverclaimRejections() {
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields(undefined, {
        socialProviderReceiptBoundaryRows: 1,
        socialProviderManualReceiptRequiredRows: 1,
        socialProviderDurableRows: 1,
        socialProviderDurableResultRefs: ['browser-social-provider-durable-result-ref-test'],
      })
    )
  ).toEqual({ ok: false, reason: 'invalid-stream' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields(undefined, {
        socialProviderReceiptBoundaryRows: 1,
        socialProviderDispatchRequiredRows: 1,
        socialProviderDurableRows: 1,
        socialProviderDurableResultRefs: ['browser-social-provider-durable-result-ref-test'],
        socialProviderDurableStoreRefs: ['browser-social-provider-durable-store-ref-test'],
        socialProviderReadModelRefs: ['browser-social-provider-read-model-ref-test'],
        socialProviderSupportStatusRefs: ['browser-social-provider-support-status-ref-test'],
      })
    )
  ).toEqual({ ok: false, reason: 'invalid-stream' });
  expect(
    parseAgentBrowserRuntimeEventChainStreamFields(
      streamFields(undefined, {
        socialProviderReceiptBoundaryRows: 1,
        socialProviderDispatchRequiredRows: 1,
        socialProviderManualReceiptRequiredRows: 1,
      })
    )
  ).toEqual({ ok: false, reason: 'invalid-stream' });
}

function streamFields(
  entries = validEntries(),
  counters: {
    readonly actionIntentCandidates?: number;
    readonly actionIntentHandoffCandidates?: number;
    readonly actionIntentHandoffOutboxRefs?: readonly string[];
    readonly actionIntentHandoffRefs?: readonly string[];
    readonly actionIntentDispatchAttempts?: number;
    readonly actionIntentAdapterExecutions?: number;
    readonly actionIntentChildInterventionExecutions?: number;
    readonly actionIntentEnforcementExecutions?: number;
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
  } = {}
) {
  return {
    ...streamCountFields(entries),
    ...streamActionIntentFields(counters),
    ...streamSocialProviderReceiptFields(counters),
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  };
}

function streamCountFields(entries: readonly unknown[]) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
  };
}

function streamActionIntentFields(counters: {
  readonly actionIntentCandidates?: number;
  readonly actionIntentHandoffCandidates?: number;
  readonly actionIntentHandoffOutboxRefs?: readonly string[];
  readonly actionIntentHandoffRefs?: readonly string[];
  readonly actionIntentDispatchAttempts?: number;
  readonly actionIntentAdapterExecutions?: number;
  readonly actionIntentChildInterventionExecutions?: number;
  readonly actionIntentEnforcementExecutions?: number;
}) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: counters.actionIntentCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]:
      counters.actionIntentHandoffCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify(
      counters.actionIntentHandoffOutboxRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs]: JSON.stringify(
      counters.actionIntentHandoffRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]:
      counters.actionIntentDispatchAttempts ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]:
      counters.actionIntentAdapterExecutions ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]:
      counters.actionIntentChildInterventionExecutions ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]:
      counters.actionIntentEnforcementExecutions ?? 0,
  };
}

function streamSocialProviderReceiptFields(counters: {
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
}) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]:
      counters.socialProviderReceiptBoundaryRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]:
      counters.socialProviderDispatchRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]:
      counters.socialProviderManualReceiptRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify(
      counters.socialProviderAttemptRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify(
      counters.socialProviderReceiptProofRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: counters.socialProviderDurableRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify(
      counters.socialProviderDurableResultRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify(
      counters.socialProviderDurableStoreRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify(
      counters.socialProviderReadModelRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify(
      counters.socialProviderSupportStatusRefs ?? []
    ),
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
