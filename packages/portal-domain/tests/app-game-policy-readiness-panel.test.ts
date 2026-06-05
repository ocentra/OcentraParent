import { describe, expect, it } from 'vitest';
import {
  AgentAppGamePolicyReadinessKind,
  AgentAppGamePolicyReadinessState,
  type AgentAppGamePolicyReadinessResult,
} from '@ocentra-parent/agent-protocol-domain/app-game-policy-readiness';
import { createAppGamePolicyReadinessPanelIntent, PortalDetails } from '../src/contracts';

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

describe('app-game policy readiness panel intent', () => {
  it('summarizes the service-backed readiness read model without adapter claims', () => {
    const intent = createAppGamePolicyReadinessPanelIntent({
      ok: true,
      value: PolicyReadinessReadModel,
    });

    expect(intent.title).toBe('App/game policy readiness');
    expect(intent.loadState).toBe('Review');
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.AdapterDispatch,
      value: 'Not claimed',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.ProductClaim,
      value: 'Readiness rendering only; policy execution and adapter dispatch are not proved.',
    });
    expect(intent.rows.map((row) => row.title)).toEqual(['Policy evidence', 'AI classifier context']);
    expect(intent.rows[0]?.details).toContainEqual({
      label: PortalDetails.EvidenceReferences,
      value: 'claim-1 | identity-1',
    });
    expect(intent.rows[1]?.details).toContainEqual({
      label: PortalDetails.Status,
      value: 'Manual required',
    });
  });

  it('keeps parser failures and missing events visibly non-product-ready', () => {
    const parserFailure: AgentAppGamePolicyReadinessResult = {
      ok: false,
      reason: 'invalid-payload',
    };

    expect(createAppGamePolicyReadinessPanelIntent(null)).toMatchObject({
      loadState: 'Unavailable',
      rows: [],
      emptyMessage: 'No app/game policy readiness read model has been reported yet.',
    });
    expect(createAppGamePolicyReadinessPanelIntent(parserFailure)).toMatchObject({
      loadState: 'Review',
      rows: [],
    });
  });
});
