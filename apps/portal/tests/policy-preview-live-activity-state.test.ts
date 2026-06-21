import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal policy-preview live activity state', () => {
  registerPolicyPreviewParsingTests();
  registerPolicyPreviewRejectionTests();
});

function registerPolicyPreviewParsingTests(): void {
  it('parses real service policy-preview read-model payload fields', () => {
    const state = resolveLiveActivityState([policyPreviewEvent()]);

    expect(state.policyPreviewEvent?.severity).toBe('info');
    expect(state.policyPreviewReadModel).toMatchObject({
      returned: 1,
      previewId: 'policy-preview-1',
      targetValue: 'https://example.test/learn',
      decisionAction: 'allow',
      parentRuleContextReferenceCount: 1,
      parentRuleContextRefIds: 'parent-rule-context-1',
      dryRun: true,
      enforcementHandoffState: 'disabled-preview-only',
      policyPreviewSaveState: 'ready-to-save',
      policyPreviewManualReviewState: 'not-required',
      policyPreviewTargetState: 'supported',
      policyPreviewTargetExplanationCode: 'target-supported',
      policyPreviewFindingKinds: 'policy-match',
      policySourceStatus: 'preview',
      policySourceSurface: 'parent-portal',
      policyRequestOrigin: 'assistant-draft',
      policyAssistantConfirmationState: 'parent-confirmation-required',
      policyRequestStatus: 'pending-parent-review',
      networkEvidenceGrade: 'B',
      networkRequestedPolicyAction: 'block',
      networkMappedPolicyAction: 'ask-parent',
      networkPolicyMappingMode: 'parent-review',
      networkAdapterActionAuthorized: false,
      networkEnforcementCommandAuthorized: false,
    });
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
    expect(state.policyPreviewReadModel?.policyPreviewSaveState).toBeNull();
    expect(state.policyPreviewReadModel?.policySourceStatus).toBeNull();
    expect(state.policyPreviewReadModel?.policyRequestStatus).toBeNull();
  });

  it('keeps unavailable policy-preview responses visible without inventing rows', () => {
    const state = resolveLiveActivityState([unavailablePolicyPreviewEvent()]);

    expect(state.policyPreviewReadModel).toBeNull();
    expect(state.policyPreviewEvent?.severity).toBe('error');
    expect(state.policyPreviewEvent?.payload['reason']).toBe('Policy preview store is unavailable.');
  });
}

function registerPolicyPreviewRejectionTests(): void {
  it('rejects flattened policy-preview payloads with untyped numeric and boolean fields', () => {
    const returnedAsText = resolveLiveActivityState([policyPreviewEventWith({ returned: '1' })]);
    const countAsText = resolveLiveActivityState([policyPreviewEventWith({ evidenceReferenceCount: '1' })]);
    const dryRunAsText = resolveLiveActivityState([policyPreviewEventWith({ dryRun: 'true' })]);

    expect(returnedAsText.policyPreviewReadModel).toBeNull();
    expect(countAsText.policyPreviewReadModel).toBeNull();
    expect(dryRunAsText.policyPreviewReadModel).toBeNull();
  });

  it('rejects policy-preview payloads that claim network authorization', () => {
    const adapterAuthorized = resolveLiveActivityState([
      policyPreviewEventWith({ networkAdapterActionAuthorized: true }),
    ]);
    const enforcementAuthorized = resolveLiveActivityState([
      policyPreviewEventWith({ networkEnforcementCommandAuthorized: true }),
    ]);

    expect(adapterAuthorized.policyPreviewReadModel).toBeNull();
    expect(enforcementAuthorized.policyPreviewReadModel).toBeNull();
  });

  it('rejects invalid policy-preview enums and non-preview network handoff', () => {
    const invalidTarget = resolveLiveActivityState([policyPreviewEventWith({ targetType: 'packet-payload' })]);
    const invalidDecisionAction = resolveLiveActivityState([policyPreviewEventWith({ policyAction: 'kill-process' })]);
    const invalidGrade = resolveLiveActivityState([policyPreviewEventWith({ networkEvidenceGrade: 'AA' })]);
    const invalidMapping = resolveLiveActivityState([
      policyPreviewEventWith({ networkPolicyMappingMode: 'direct-enforcement' }),
    ]);
    const nonPreviewNetworkFields = resolveLiveActivityState([policyPreviewEventWith({ dryRun: false })]);
    const nonAdvisoryHandoff = resolveLiveActivityState([
      policyPreviewEventWith({ enforcementHandoffState: 'adapter-dispatch-ready' }),
    ]);

    expect(invalidTarget.policyPreviewReadModel).toBeNull();
    expect(invalidDecisionAction.policyPreviewReadModel).toBeNull();
    expect(invalidGrade.policyPreviewReadModel).toBeNull();
    expect(invalidMapping.policyPreviewReadModel).toBeNull();
    expect(nonPreviewNetworkFields.policyPreviewReadModel).toBeNull();
    expect(nonAdvisoryHandoff.policyPreviewReadModel).toBeNull();
  });
}

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
      [AgentProtocolDefaults.Field.PolicyPreviewSaveState]: 'ready-to-save',
      [AgentProtocolDefaults.Field.PolicyPreviewManualReviewState]: 'not-required',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetState]: 'supported',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetExplanationCode]: 'target-supported',
      [AgentProtocolDefaults.Field.PolicyPreviewFindingKinds]: 'policy-match',
      [AgentProtocolDefaults.Field.PolicySourceStatus]: 'preview',
      [AgentProtocolDefaults.Field.PolicySourceSurface]: 'parent-portal',
      [AgentProtocolDefaults.Field.PolicyRequestOrigin]: 'assistant-draft',
      [AgentProtocolDefaults.Field.PolicyAssistantConfirmationState]: 'parent-confirmation-required',
      [AgentProtocolDefaults.Field.PolicyRequestStatus]: 'pending-parent-review',
      networkEvidenceGrade: 'B',
      networkRequestedPolicyAction: 'block',
      networkMappedPolicyAction: 'ask-parent',
      networkPolicyMappingMode: 'parent-review',
      networkAdapterActionAuthorized: false,
      networkEnforcementCommandAuthorized: false,
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
      [AgentProtocolDefaults.Field.PolicyPreviewSaveState]: null,
      [AgentProtocolDefaults.Field.PolicyPreviewManualReviewState]: null,
      [AgentProtocolDefaults.Field.PolicyPreviewTargetState]: null,
      [AgentProtocolDefaults.Field.PolicyPreviewTargetExplanationCode]: null,
      [AgentProtocolDefaults.Field.PolicyPreviewFindingKinds]: null,
      [AgentProtocolDefaults.Field.PolicySourceStatus]: null,
      [AgentProtocolDefaults.Field.PolicySourceSurface]: null,
      [AgentProtocolDefaults.Field.PolicyRequestOrigin]: null,
      [AgentProtocolDefaults.Field.PolicyAssistantConfirmationState]: null,
      [AgentProtocolDefaults.Field.PolicyRequestStatus]: null,
      networkEvidenceGrade: null,
      networkRequestedPolicyAction: null,
      networkMappedPolicyAction: null,
      networkPolicyMappingMode: null,
      networkAdapterActionAuthorized: null,
      networkEnforcementCommandAuthorized: null,
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
