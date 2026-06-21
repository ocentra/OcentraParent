import { describe, expect, it } from 'vitest';
import {
  type AgentAppGamePolicyReadinessResult,
} from '@ocentra-parent/agent-protocol-domain/app-game-policy-readiness';
import {
  AgentAppGamePolicyReadinessKind,
  AgentAppGamePolicyReadinessState,
} from '@ocentra-parent/schema-domain/app-game-policy-readiness';
import { createAppGamePolicyReadinessPanelIntent, PortalDetails } from '../../src/contracts';

const AppGameSchemaVersion = 1;

const PolicyReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-05T11:45:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'notClaimed',
  returned: 4,
  policyEvaluationReady: true,
  categoryRoutingReady: true,
  unknownReviewRequired: true,
  manualReviewRequired: true,
  adapterDispatchClaimed: false,
  evidenceClaimRowCount: 1,
  identityRowCount: 1,
  approvalAuthorityRowCount: 1,
  approvalActionResultRowCount: 0,
  platformAuthorityRowCount: 1,
  aiClassifierResultRowCount: 0,
  categoryCandidateRowCount: 1,
  unknownReviewRowCount: 1,
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
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGamePolicyReadinessKind.CategoryCandidate,
      readinessKind: AgentAppGamePolicyReadinessKind.CategoryCandidate,
      readinessState: AgentAppGamePolicyReadinessState.Ready,
      rowCount: 1,
      evidenceReferenceIds: ['category-candidate-1'],
      evidence: [],
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGamePolicyReadinessKind.UnknownReview,
      readinessKind: AgentAppGamePolicyReadinessKind.UnknownReview,
      readinessState: AgentAppGamePolicyReadinessState.ManualRequired,
      rowCount: 1,
      evidenceReferenceIds: ['unknown-review-1'],
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
      label: PortalDetails.CategoryCandidateRows,
      value: '1',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.UnknownReview,
      value: 'Manual required',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.ProductClaim,
      value: 'Readiness rendering only; policy execution and adapter dispatch are not proved.',
    });
    expect(intent.rows.map((row) => row.title)).toEqual([
      'Policy evidence',
      'AI classifier context',
      'Category candidate',
      'Unknown review',
    ]);
    expect(intent.rows[0]?.details).toContainEqual({
      label: PortalDetails.EvidenceReferences,
      value: 'claim-1 | identity-1',
    });
    expect(intent.rows[1]?.details).toContainEqual({
      label: PortalDetails.Status,
      value: 'Manual required',
    });
    expect(intent.rows[2]?.details).toContainEqual({
      label: PortalDetails.EvidenceReferences,
      value: 'category-candidate-1',
    });
    expect(intent.rows[3]?.details).toContainEqual({
      label: PortalDetails.Reason,
      value: 'Unknown evidence requires manual review',
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
