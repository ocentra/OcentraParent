import { expect, it } from 'vitest';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '../src/contracts';

it('AgentCommandEnvelopeSchema: accepts a policy preview read-model command', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-policy-preview-1',
    sentAt: '2026-05-21T11:06:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: AgentCommand.PolicyPreviewReadModelGet,
    payload: {},
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.command).toBe('agent.policy.preview.read-model.get');
  }
});

it('AgentEventEnvelopeSchema: accepts a policy preview read-model report payload', () => {
  const parsed = AgentEventEnvelopeSchema.safeParse({
    schemaVersion: 1,
    eventId: 'policy-preview-read-model-reported',
    correlationId: 'cmd-policy-preview-1',
    sentAt: '2026-05-21T11:06:01Z',
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
      [AgentProtocolDefaults.Field.SchemaVersion]: 'v0.6',
      [AgentProtocolDefaults.Field.PolicyPreviewId]: 'policy-preview-1',
      [AgentProtocolDefaults.Field.PolicyDecisionId]: 'policy-preview-decision-1',
      [AgentProtocolDefaults.Field.PolicyAction]: 'allow',
      [AgentProtocolDefaults.Field.PolicyDryRun]: true,
      [AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount]: 1,
      [AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount]: 1,
      [AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds]: 'parent-rule-context-1',
      [AgentProtocolDefaults.Field.PolicyHandoffState]: 'disabled',
      [AgentProtocolDefaults.Field.PolicyReasonCodes]: 'no-matching-parent-rule',
      [AgentProtocolDefaults.Field.PolicyRuleIds]: 'rule-allow',
      [AgentProtocolDefaults.Field.PolicyTargetType]: 'domain',
      [AgentProtocolDefaults.Field.PolicyTargetValue]: 'video.example',
      [AgentProtocolDefaults.Field.LocalAiResultId]: 'local-ai-result-1',
      [AgentProtocolDefaults.Field.TargetId]: 'target-1',
    },
    snapshot: null,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.event).toBe('agent.policy.preview.read-model.reported');
    expect(parsed.data.payload[AgentProtocolDefaults.Field.PolicyHandoffState]).toBe('disabled');
    expect(parsed.data.payload[AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds]).toBe(
      'parent-rule-context-1'
    );
  }
});

it('AgentProtocolDefaults.Field: exposes policy preview payload fields', () => {
  expect(AgentProtocolDefaults.Field.LocalAiResultId).toBe('localAiResultId');
  expect(AgentProtocolDefaults.Field.PolicyAction).toBe('policyAction');
  expect(AgentProtocolDefaults.Field.PolicyDecisionId).toBe('policyDecisionId');
  expect(AgentProtocolDefaults.Field.PolicyDryRun).toBe('dryRun');
  expect(AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount).toBe('evidenceReferenceCount');
  expect(AgentProtocolDefaults.Field.PolicyParentRuleContextReferenceCount).toBe('parentRuleContextReferenceCount');
  expect(AgentProtocolDefaults.Field.PolicyParentRuleContextRefIds).toBe('parentRuleContextRefIds');
  expect(AgentProtocolDefaults.Field.PolicyHandoffState).toBe('enforcementHandoffState');
  expect(AgentProtocolDefaults.Field.PolicyPreviewId).toBe('policyPreviewId');
  expect(AgentProtocolDefaults.Field.PolicyReasonCodes).toBe('reasonCodes');
  expect(AgentProtocolDefaults.Field.PolicyRuleIds).toBe('ruleIds');
  expect(AgentProtocolDefaults.Field.PolicyTargetType).toBe('targetType');
  expect(AgentProtocolDefaults.Field.PolicyTargetValue).toBe('targetValue');
  expect(AgentProtocolDefaults.Field.SchemaVersion).toBe('schemaVersion');
  expect(AgentProtocolDefaults.Field.TargetId).toBe('targetId');
});

it('AgentCommand and AgentEvent: expose policy preview read-model constants', () => {
  expect(AgentCommand.PolicyPreviewReadModelGet).toBe('agent.policy.preview.read-model.get');
  expect(AgentEvent.PolicyPreviewReadModelReported).toBe('agent.policy.preview.read-model.reported');
});
