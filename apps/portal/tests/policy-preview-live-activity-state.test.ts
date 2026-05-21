import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal policy-preview live activity state', () => {
  it('parses real service policy-preview read-model payload fields', () => {
    const state = resolveLiveActivityState([policyPreviewEvent()]);

    expect(state.policyPreviewEvent?.severity).toBe('info');
    expect(state.policyPreviewReadModel?.returned).toBe(1);
    expect(state.policyPreviewReadModel?.previewId).toBe('policy-preview-1');
    expect(state.policyPreviewReadModel?.targetValue).toBe('https://example.test/learn');
    expect(state.policyPreviewReadModel?.decisionAction).toBe('allow');
    expect(state.policyPreviewReadModel?.parentRuleContextReferenceCount).toBe(1);
    expect(state.policyPreviewReadModel?.parentRuleContextRefIds).toBe('parent-rule-context-1');
    expect(state.policyPreviewReadModel?.dryRun).toBe(true);
    expect(state.policyPreviewReadModel?.enforcementHandoffState).toBe('disabled-preview-only');
  });

  it('parses service policy schema versions without weakening typed payload fields', () => {
    const state = resolveLiveActivityState([policyPreviewEventWith({ schemaVersion: 'v0.6' })]);

    expect(state.policyPreviewReadModel?.schemaVersion).toBe('v0.6');
    expect(state.policyPreviewReadModel?.returned).toBe(1);
    expect(state.policyPreviewReadModel?.parentRuleContextReferenceCount).toBe(1);
  });

  it('keeps empty policy-preview read models visible without inventing a decision', () => {
    const state = resolveLiveActivityState([emptyPolicyPreviewEvent()]);

    expect(state.policyPreviewReadModel?.returned).toBe(0);
    expect(state.policyPreviewReadModel?.previewId).toBeNull();
    expect(state.policyPreviewReadModel?.decisionAction).toBeNull();
  });

  it('keeps unavailable policy-preview responses visible without inventing rows', () => {
    const state = resolveLiveActivityState([unavailablePolicyPreviewEvent()]);

    expect(state.policyPreviewReadModel).toBeNull();
    expect(state.policyPreviewEvent?.severity).toBe('error');
    expect(state.policyPreviewEvent?.payload['reason']).toBe('Policy preview store is unavailable.');
  });

  it('rejects flattened policy-preview payloads with untyped numeric and boolean fields', () => {
    const returnedAsText = resolveLiveActivityState([policyPreviewEventWith({ returned: '1' })]);
    const countAsText = resolveLiveActivityState([policyPreviewEventWith({ evidenceReferenceCount: '1' })]);
    const dryRunAsText = resolveLiveActivityState([policyPreviewEventWith({ dryRun: 'true' })]);

    expect(returnedAsText.policyPreviewReadModel).toBeNull();
    expect(countAsText.policyPreviewReadModel).toBeNull();
    expect(dryRunAsText.policyPreviewReadModel).toBeNull();
  });
});

function policyPreviewEvent() {
  return policyPreviewEventWith({});
}

function policyPreviewEventWith(payloadOverrides: Record<string, unknown>) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-policy-preview',
    correlationId: 'cmd-policy-preview',
    sentAt: '2026-05-21T01:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {
      schemaVersion: 1,
      generatedAt: '2026-05-21T01:00:02Z',
      custody: 'child-device-local',
      limit: 1,
      returned: 1,
      capabilityStatus: 'available',
      policyPreviewId: 'policy-preview-1',
      latestEventId: 'activity-browser-url-observed-1',
      latestObservedAt: '2026-05-21T01:00:00Z',
      targetId: 'browser-evidence-1',
      targetType: 'url',
      targetValue: 'https://example.test/learn',
      evidenceReferenceCount: 1,
      parentRuleContextReferenceCount: 1,
      parentRuleContextRefIds: 'parent-rule-context-1',
      policyDecisionId: 'policy-decision-1',
      policyAction: 'allow',
      reasonCodes: 'educational-domain',
      ruleIds: 'allow-learning-sites',
      localAiResultId: 'local-ai-result-1',
      dryRun: true,
      enforcementHandoffState: 'disabled-preview-only',
      ...payloadOverrides,
    },
    snapshot: null,
  });
}

function emptyPolicyPreviewEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-policy-preview',
    correlationId: 'cmd-policy-preview',
    sentAt: '2026-05-21T01:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {
      schemaVersion: 1,
      generatedAt: '2026-05-21T01:00:02Z',
      custody: 'child-device-local',
      limit: 1,
      returned: 0,
      capabilityStatus: 'empty',
      policyPreviewId: null,
      latestEventId: null,
      latestObservedAt: null,
      targetId: null,
      targetType: null,
      targetValue: null,
      evidenceReferenceCount: null,
      parentRuleContextReferenceCount: null,
      parentRuleContextRefIds: null,
      policyDecisionId: null,
      policyAction: null,
      reasonCodes: null,
      ruleIds: null,
      localAiResultId: null,
      dryRun: null,
      enforcementHandoffState: 'disabled-preview-only',
    },
    snapshot: null,
  });
}

function unavailablePolicyPreviewEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-policy-preview',
    correlationId: 'cmd-policy-preview',
    sentAt: '2026-05-21T01:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'error',
    payload: {
      reason: 'Policy preview store is unavailable.',
    },
    snapshot: null,
  });
}
