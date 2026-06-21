import { AppGameSchemaVersion } from './app-game-primitives';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const PolicyReadinessCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGamePolicyReadinessKind = {
  PolicyEvidence: 'policyEvidence',
  ApprovalAuthority: 'approvalAuthority',
  ApprovalActionResult: 'approvalActionResult',
  PlatformAuthority: 'platformAuthority',
  AiClassifierContext: 'aiClassifierContext',
  CategoryCandidate: 'categoryCandidate',
  UnknownReview: 'unknownReview',
} as const;

export const AgentAppGamePolicyReadinessState = {
  Ready: 'ready',
  Missing: 'missing',
  ManualRequired: 'manual-required',
} as const;

export const AgentAppGamePolicyReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rowId: NonEmptyStringSchema,
    readinessKind: Schema.Literal(
      AgentAppGamePolicyReadinessKind.PolicyEvidence,
      AgentAppGamePolicyReadinessKind.ApprovalAuthority,
      AgentAppGamePolicyReadinessKind.ApprovalActionResult,
      AgentAppGamePolicyReadinessKind.PlatformAuthority,
      AgentAppGamePolicyReadinessKind.AiClassifierContext,
      AgentAppGamePolicyReadinessKind.CategoryCandidate,
      AgentAppGamePolicyReadinessKind.UnknownReview
    ),
    readinessState: Schema.Literal(
      AgentAppGamePolicyReadinessState.Ready,
      AgentAppGamePolicyReadinessState.Missing,
      AgentAppGamePolicyReadinessState.ManualRequired
    ),
    rowCount: PolicyReadinessCount,
    evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGamePolicyReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: NonEmptyStringSchema,
    custodyLabel: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
    returned: PolicyReadinessCount,
    policyEvaluationReady: Schema.Boolean,
    categoryRoutingReady: Schema.Boolean,
    unknownReviewRequired: Schema.Boolean,
    manualReviewRequired: Schema.Boolean,
    adapterDispatchClaimed: Schema.Literal(false),
    evidenceClaimRowCount: PolicyReadinessCount,
    identityRowCount: PolicyReadinessCount,
    approvalAuthorityRowCount: PolicyReadinessCount,
    approvalActionResultRowCount: PolicyReadinessCount,
    platformAuthorityRowCount: PolicyReadinessCount,
    aiClassifierResultRowCount: PolicyReadinessCount,
    categoryCandidateRowCount: PolicyReadinessCount,
    unknownReviewRowCount: PolicyReadinessCount,
    rows: Schema.Array(AgentAppGamePolicyReadinessRowSchema),
  })
);

export type AgentAppGamePolicyReadinessKind = Infer<typeof AgentAppGamePolicyReadinessRowSchema>['readinessKind'];
export type AgentAppGamePolicyReadinessState = Infer<typeof AgentAppGamePolicyReadinessRowSchema>['readinessState'];
export type AgentAppGamePolicyReadinessRow = Infer<typeof AgentAppGamePolicyReadinessRowSchema>;
export type AgentAppGamePolicyReadinessReadModel = Infer<typeof AgentAppGamePolicyReadinessReadModelSchema>;
