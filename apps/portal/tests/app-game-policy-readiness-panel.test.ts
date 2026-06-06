import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentAppGamePolicyReadinessKind,
  AgentAppGamePolicyReadinessState,
} from '@ocentra-parent/agent-protocol-domain/app-game-policy-readiness';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderAppGamePolicyReadinessRoute } from '../src/AppGamePolicyReadinessRoutePanel';
import { createAppGamePolicyReadinessPanelIntent } from '../src/app-game-policy-readiness-panel';
import { resolveLiveActivityState } from '../src/live-activity-state';

const AppGameSchemaVersion = 1;

const PolicyReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-05T11:45:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'notClaimed',
  returned: 2,
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

describe('app-game policy readiness portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGamePolicyReadinessRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGamePolicyReadinessRoute(PortalRoute.Overview)).toBe(false);
  });

  it('uses the latest service-backed policy readiness event for the route intent', () => {
    const liveActivity = resolveLiveActivityState([policyReadinessEvent(JSON.stringify(PolicyReadinessReadModel))]);

    expect(liveActivity.appGamePolicyReadinessReadModel).toMatchObject({
      ok: true,
      value: {
        returned: 2,
        adapterDispatchClaimed: false,
      },
    });

    const intent = createAppGamePolicyReadinessPanelIntent(liveActivity.appGamePolicyReadinessReadModel);
    expect(intent.summaryDetails).toContainEqual({
      label: 'Capability',
      value: 'Not claimed',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: 'Adapter dispatch',
      value: 'Not claimed',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: 'Manual review',
      value: 'Manual required',
    });
    expect(intent.rows.map((row) => row.title)).toEqual(['Policy evidence', 'AI classifier context']);
  });
});

function policyReadinessEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-policy-readiness-event',
    correlationId: 'app-game-policy-readiness-command',
    sentAt: '2026-06-05T11:45:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGamePolicyReadinessReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGamePolicyReadinessReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}
