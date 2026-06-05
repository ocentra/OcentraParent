import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import {
  AgentAppGamePolicyPreviewTargetDomain,
  AgentAppGamePolicyPreviewUnavailableReason,
  parseAgentAppGamePolicyPreviewEvent,
} from '../src/app-game-policy-preview-read-model';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const PolicyPreviewReadModel = {
  schemaVersion: 'policy-dry-run-v0.6',
  generatedAt: '2026-06-05T18:45:00Z',
  custody: 'activity-store',
  limit: 5,
  returned: 2,
  capabilityStatus: 'ready',
  rows: [
    {
      previewId: 'policy-preview-app-1',
      sourceEventId: 'activity-event-app-1',
      observedAt: '2026-06-05T18:44:00Z',
      target: {
        targetId: 'target-app-1',
        targetType: 'app',
        targetValue: 'opaque-app-ref-1',
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-app-1',
          kind: 'activity-event',
          observedAt: '2026-06-05T18:44:00Z',
        },
      ],
      parentRuleContextReferences: [
        {
          parentRuleRefId: 'parent-rule-context-1',
        },
      ],
      decision: {
        schemaVersion: 'policy-dry-run-v0.6',
        decisionId: 'policy-decision-app-1',
        action: 'time-limit',
        reasonCodes: ['parent-rule-time-limit'],
        evidenceReferences: [],
        ruleIds: ['parent-rule-1'],
        localAiResultId: null,
        dryRun: true,
        enforcementHandoffState: 'disabled',
        expiresAt: null,
      },
    },
    {
      previewId: 'policy-preview-domain-1',
      sourceEventId: 'activity-event-domain-1',
      observedAt: '2026-06-05T18:43:00Z',
      target: {
        targetId: 'target-domain-1',
        targetType: 'domain',
        targetValue: 'example.invalid',
      },
      evidenceReferences: [],
      parentRuleContextReferences: [],
      decision: {
        schemaVersion: 'policy-dry-run-v0.6',
        decisionId: 'policy-decision-domain-1',
        action: 'unknown',
        reasonCodes: ['no-matching-parent-rule'],
        evidenceReferences: [],
        ruleIds: [],
        localAiResultId: null,
        dryRun: true,
        enforcementHandoffState: 'disabled',
        expiresAt: null,
      },
    },
  ],
} as const;

describe('agent app-game policy preview parser', () => {
  it('parses the service policy preview read model without claiming native-game promotion', () => {
    const parsed = parseAgentAppGamePolicyPreviewEvent(policyPreviewEvent(JSON.stringify(PolicyPreviewReadModel)));

    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      return;
    }

    expect(parsed.value).toMatchObject({
      schemaVersion: AppGameSchemaVersion,
      generatedAt: PolicyPreviewReadModel.generatedAt,
      custodyLabel: PolicyPreviewReadModel.custody,
      capabilityStatus: PolicyPreviewReadModel.capabilityStatus,
      returned: 2,
      nativeAppPreviewRowCount: 1,
      nativeGamePreviewRowCount: 0,
      notAppGameRowCount: 1,
      nativeGamePromotionClaimed: false,
      nativeGameUnavailableReason: AgentAppGamePolicyPreviewUnavailableReason.SourceTargetKindNotPersisted,
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
    });
    expect(parsed.value.rows.map((row) => row.targetDomain)).toEqual([
      AgentAppGamePolicyPreviewTargetDomain.NativeApp,
      AgentAppGamePolicyPreviewTargetDomain.NotAppGame,
    ]);
  });

  it('rejects wrong events, bad JSON, and service rows that try to execute', () => {
    expect(
      parseAgentAppGamePolicyPreviewEvent({
        ...policyPreviewEvent(JSON.stringify(PolicyPreviewReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGamePolicyPreviewEvent(policyPreviewEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGamePolicyPreviewEvent(
        policyPreviewEvent(
          JSON.stringify({
            ...PolicyPreviewReadModel,
            rows: [
              {
                ...PolicyPreviewReadModel.rows[0],
                decision: {
                  ...PolicyPreviewReadModel.rows[0].decision,
                  dryRun: false,
                },
              },
            ],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function policyPreviewEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'policy-preview-event',
    correlationId: 'policy-preview-command',
    sentAt: '2026-06-05T18:45:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.Payload]: serializedReadModel,
    },
    snapshot: null,
  };
}
