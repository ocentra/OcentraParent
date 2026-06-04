import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import {
  AgentAppGamePolicyReadinessKind,
  AgentAppGamePolicyReadinessState,
  parseAgentAppGamePolicyReadinessEvent,
} from '../src/app-game-policy-readiness';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const PolicyReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-04T16:45:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'notClaimed',
  returned: 5,
  policyEvaluationReady: true,
  manualReviewRequired: true,
  adapterDispatchClaimed: false,
  evidenceClaimRowCount: 1,
  identityRowCount: 1,
  approvalAuthorityRowCount: 1,
  approvalActionResultRowCount: 0,
  platformAuthorityRowCount: 1,
  aiClassifierResultRowCount: 0,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGamePolicyReadinessKind.PolicyEvidence,
      readinessKind: AgentAppGamePolicyReadinessKind.PolicyEvidence,
      readinessState: AgentAppGamePolicyReadinessState.Ready,
      rowCount: 2,
      evidenceReferenceIds: ['claim-1', 'identity-1'],
      evidence: [
        {
          evidenceId: 'claim-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGamePolicyReadinessKind.AiClassifierContext,
      readinessKind: AgentAppGamePolicyReadinessKind.AiClassifierContext,
      readinessState: AgentAppGamePolicyReadinessState.ManualRequired,
      rowCount: 0,
      evidenceReferenceIds: [],
      evidence: [],
    },
  ],
} as const;

describe('agent app-game policy readiness parser', () => {
  it('parses the dedicated policy readiness read-model event payload', () => {
    const parsed = parseAgentAppGamePolicyReadinessEvent(
      policyReadinessEvent(JSON.stringify(PolicyReadinessReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: PolicyReadinessReadModel,
    });
  });

  it('rejects invalid readiness payloads and adapter dispatch claims', () => {
    expect(
      parseAgentAppGamePolicyReadinessEvent({
        ...policyReadinessEvent(JSON.stringify(PolicyReadinessReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGamePolicyReadinessEvent(policyReadinessEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGamePolicyReadinessEvent(
        policyReadinessEvent(JSON.stringify({ ...PolicyReadinessReadModel, adapterDispatchClaimed: true }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function policyReadinessEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-policy-readiness-event',
    correlationId: 'app-game-policy-readiness-command',
    sentAt: '2026-06-04T16:45:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGamePolicyReadinessReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGamePolicyReadinessReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
