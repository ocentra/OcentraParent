import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import {
  AgentAppGamePolicyEvaluationDecisionState,
  AgentAppGamePolicyEvaluationKind,
  AgentAppGamePolicyEvaluationRejectionReason,
  parseAgentAppGamePolicyEvaluationEvent,
} from '../src/app-game-policy-evaluation';
import { AgentAppGamePolicyReadinessKind } from '../src/app-game-policy-readiness';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const PolicyEvaluationReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-04T23:40:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'policy-evaluation-ready',
  returned: 2,
  policyEvaluationReady: true,
  manualReviewRequired: true,
  dryRun: true,
  enforcementHandoffState: 'disabled',
  adapterDispatchClaimed: false,
  readinessRowCount: 5,
  evaluatedRowCount: 2,
  evidenceClaimRowCount: 1,
  identityRowCount: 1,
  approvalAuthorityRowCount: 1,
  approvalActionResultRowCount: 0,
  platformAuthorityRowCount: 1,
  aiClassifierResultRowCount: 0,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      evaluationId: AgentAppGamePolicyEvaluationKind.TimeLimit,
      evaluationKind: AgentAppGamePolicyEvaluationKind.TimeLimit,
      requestedAction: 'time-limit',
      policyAction: 'time-limit',
      decisionState: AgentAppGamePolicyEvaluationDecisionState.DryRunReady,
      rejectionReason: AgentAppGamePolicyEvaluationRejectionReason.None,
      reasonCodes: ['app-game-policy-readiness-ready', 'adapter-dispatch-disabled'],
      requiredReadinessKinds: [
        AgentAppGamePolicyReadinessKind.PolicyEvidence,
        AgentAppGamePolicyReadinessKind.ApprovalAuthority,
        AgentAppGamePolicyReadinessKind.PlatformAuthority,
      ],
      evidenceReferenceIds: ['claim-1', 'identity-1'],
      evidence: [
        {
          evidenceId: 'claim-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
      dryRun: true,
      enforcementHandoffState: 'disabled',
      adapterDispatchState: 'not-dispatched',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      evaluationId: AgentAppGamePolicyEvaluationKind.BlockLaunch,
      evaluationKind: AgentAppGamePolicyEvaluationKind.BlockLaunch,
      requestedAction: 'block-launch',
      policyAction: 'block',
      decisionState: AgentAppGamePolicyEvaluationDecisionState.ManualRequired,
      rejectionReason: AgentAppGamePolicyEvaluationRejectionReason.BlockLaunchManualRequired,
      reasonCodes: ['block-launch-manual-required', 'adapter-dispatch-disabled'],
      requiredReadinessKinds: [
        AgentAppGamePolicyReadinessKind.PolicyEvidence,
        AgentAppGamePolicyReadinessKind.PlatformAuthority,
      ],
      evidenceReferenceIds: ['claim-1'],
      evidence: [],
      dryRun: true,
      enforcementHandoffState: 'disabled',
      adapterDispatchState: 'not-dispatched',
    },
  ],
} as const;

describe('agent app-game policy evaluation parser', () => {
  it('parses the dedicated policy evaluation read-model event payload', () => {
    const parsed = parseAgentAppGamePolicyEvaluationEvent(
      policyEvaluationEvent(JSON.stringify(PolicyEvaluationReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: PolicyEvaluationReadModel,
    });
  });

  it('rejects invalid evaluation payloads and adapter dispatch claims', () => {
    expect(
      parseAgentAppGamePolicyEvaluationEvent({
        ...policyEvaluationEvent(JSON.stringify(PolicyEvaluationReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGamePolicyEvaluationEvent(policyEvaluationEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGamePolicyEvaluationEvent(
        policyEvaluationEvent(JSON.stringify({ ...PolicyEvaluationReadModel, adapterDispatchClaimed: true }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentAppGamePolicyEvaluationEvent(
        policyEvaluationEvent(
          JSON.stringify({
            ...PolicyEvaluationReadModel,
            rows: [{ ...PolicyEvaluationReadModel.rows[0], dryRun: false }],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function policyEvaluationEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-policy-evaluation-event',
    correlationId: 'app-game-policy-evaluation-command',
    sentAt: '2026-06-04T23:40:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGamePolicyEvaluationReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGamePolicyEvaluationReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
