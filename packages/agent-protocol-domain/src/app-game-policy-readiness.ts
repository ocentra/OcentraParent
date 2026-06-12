import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const PolicyReadinessText = Schema.String.pipe(Schema.minLength(1));
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
    rowId: PolicyReadinessText,
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
    evidenceReferenceIds: Schema.Array(PolicyReadinessText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGamePolicyReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: PolicyReadinessText,
    custodyLabel: PolicyReadinessText,
    capabilityStatus: PolicyReadinessText,
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

export type AgentAppGamePolicyReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePolicyReadinessResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGamePolicyReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePolicyReadinessFailureReason;
    };

export function parseAgentAppGamePolicyReadinessEvent(event: AgentEventEnvelope): AgentAppGamePolicyReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGamePolicyReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGamePolicyReadinessReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGamePolicyReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentAppGamePolicyReadinessFailureReason): AgentAppGamePolicyReadinessResult {
  return {
    ok: false,
    reason,
  };
}
