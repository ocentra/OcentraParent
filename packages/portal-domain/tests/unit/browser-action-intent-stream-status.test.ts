import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
  AgentProtocolDefaults,
  parseAgentBrowserRuntimeEventChainStreamFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { createBrowserActionIntentStreamStatusIntent } from '../../src/browser-action-intent-stream-status';

describe('browser action-intent stream status intent', () => {
  it('renders handoff refs without dispatch, adapter, child intervention, or enforcement claims', () => {
    const parsed = parseAgentBrowserRuntimeEventChainStreamFields(streamFields());
    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      return;
    }

    const intent = createBrowserActionIntentStreamStatusIntent(parsed.value);

    expect(intent.summary).toBe('1 action candidates');
    expect(intent.productClaim).toContain('adapter dispatch');
    expect(intent.productClaim).toContain('enforcement remain unclaimed');
    expect(intent.details.some((detail) => detail.value === 'browser-action-intent-outbox-ref-test')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'browser-action-intent-handoff-ref-test')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'browser-parent-read-model-ref-test')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'not-claimed')).toBe(true);
  });
});

function streamFields() {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify([
      'browser-action-intent-outbox-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs]: JSON.stringify([
      'browser-action-intent-handoff-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildCommandRefs]: JSON.stringify([
      'browser-child-command-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedEventRefs]: JSON.stringify([
      'browser-child-accepted-event-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentParentReadModelRefs]: JSON.stringify([
      'browser-parent-read-model-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify([policyDecisionEntry()]),
  };
}

function policyDecisionEntry() {
  return {
    [AgentProtocolDefaults.Field.EventType]: AgentBrowserRuntimeEventType.PolicyDecisionCompleted,
    [AgentProtocolDefaults.Field.EventRef]: 'browser-action-intent-policy-decision-event-ref-test',
    [AgentProtocolDefaults.Field.Payload]: {
      phase: AgentBrowserRuntimePhase.PolicyDecisionCompleted,
      sourceRef: 'browser-source.managed-devtools',
      evidenceRef: 'browser-evidence.1',
      capabilityStatus: AgentBrowserRuntimeCapabilityStatus.TabListOnly,
      custodyLabel: AgentBrowserRuntimeCustodyLabel.ChildDeviceLocal,
      queryVisibility: AgentBrowserRuntimeQueryVisibility.LiveLocal,
      degradedReason: null,
      journalRef: 'browser-journal.1',
      aiRequestRef: null,
      aiAnalysisRef: null,
      policyEvaluationRef: 'browser-policy-evaluation-ref-test',
      policyDecisionRef: 'browser-policy-decision-ref-test',
      policyPreviewId: 'browser-policy-preview-test',
      assistantActionIntentId: 'browser-action-intent-test',
      interventionCommandRef: null,
      interventionResultRef: null,
      auditEntryRef: 'browser-audit.1',
      readModelRef: 'browser-read-model.1',
      previousPhaseRef: 'browser-runtime-policy-evaluation-ref-test',
      exactUrlClaimed: true,
      aiAuthority: false,
      policyAuthority: true,
      dryRun: true,
      adapterDispatchClaimed: false,
      interventionCommandAllowed: false,
      observedAt: '2026-06-08T06:14:00Z',
    },
  };
}
