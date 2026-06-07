import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
  AgentProtocolDefaults,
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
  interventionCommandRef: null,
  interventionResultRef: null,
  auditEntryRef: 'browser-audit.1',
  readModelRef: 'browser-read-model.1',
  previousPhaseRef: null,
  exactUrlClaimed: true,
  aiAuthority: false,
  policyAuthority: false,
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

describe('agent browser runtime event contracts', () => {
  it('parses service-backed browser runtime stream fields', specifyStreamParsing);
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
}

function specifyRejections() {
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
    parseAgentBrowserRuntimeEventChainStreamFields({
      ...streamFields(),
      [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: 5,
    })
  ).toEqual({ ok: false, reason: 'invalid-stream' });
}

function streamFields(entries = validEntries()) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
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
