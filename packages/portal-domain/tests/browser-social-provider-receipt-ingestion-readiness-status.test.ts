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
import { createBrowserSocialProviderReceiptIngestionReadinessStatusIntent } from '../src/browser-social-provider-receipt-ingestion-readiness-status';

describe('browser social provider receipt ingestion readiness status intent', () => {
  it('projects dispatch-required receipt rows as ingestion-contract-required without runtime claims', () => {
    const parsed = parseAgentBrowserRuntimeEventChainStreamFields(streamFields());
    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      return;
    }

    const intent = createBrowserSocialProviderReceiptIngestionReadinessStatusIntent(parsed.value);

    expect(intent.summary).toBe('1 readiness rows');
    expect(intent.productClaim).toContain('receipt ingestion runtime');
    expect(intent.productClaim).toContain('observed provider receipts');
    expect(intent.details.some((detail) => detail.value === 'ingestion-contract-required')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'browser-social-provider-durable-store-ref-test')).toBe(
      true
    );
    expect(intent.details.some((detail) => detail.value === '0 provider receipts observed')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'not-claimed')).toBe(true);
  });

  it('projects manual receipt rows without durable/provider refs', () => {
    const parsed = parseAgentBrowserRuntimeEventChainStreamFields(manualReceiptFields());
    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      return;
    }

    const intent = createBrowserSocialProviderReceiptIngestionReadinessStatusIntent(parsed.value);

    expect(intent.summary).toBe('1 readiness rows');
    expect(intent.details.some((detail) => detail.value === 'manual-receipt-required')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'not-observed')).toBe(true);
    expect(intent.details.some((detail) => detail.value === 'not-claimed')).toBe(true);
  });
});

function streamFields() {
  return {
    ...baseStreamFields(),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify([
      'browser-social-provider-attempt-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify([
      'browser-social-provider-receipt-proof-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify([
      'browser-social-provider-durable-result-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify([
      'browser-social-provider-durable-store-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify([
      'browser-social-provider-read-model-ref-test',
    ]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify([
      'browser-social-provider-support-status-ref-test',
    ]),
  };
}

function manualReceiptFields() {
  return {
    ...baseStreamFields(),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify([]),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify([]),
  };
}

function baseStreamFields() {
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
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify([policyDecisionEntry()]),
  };
}

function policyDecisionEntry() {
  return {
    [AgentProtocolDefaults.Field.EventType]: AgentBrowserRuntimeEventType.PolicyDecisionCompleted,
    [AgentProtocolDefaults.Field.EventRef]: 'browser-social-receipt-policy-decision-event-ref-test',
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
      observedAt: '2026-06-08T06:34:00Z',
    },
  };
}
