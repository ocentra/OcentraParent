import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentAppGamePolicyReadinessKind } from './app-game-policy-readiness';

const PolicyEvaluationText = Schema.String.pipe(Schema.minLength(1));
const PolicyEvaluationCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

const AgentAppGamePolicyEvaluationRequiredReadinessKindSchema = Schema.Literal(
  AgentAppGamePolicyReadinessKind.PolicyEvidence,
  AgentAppGamePolicyReadinessKind.ApprovalAuthority,
  AgentAppGamePolicyReadinessKind.PlatformAuthority,
  AgentAppGamePolicyReadinessKind.AiClassifierContext
);

export const AgentAppGamePolicyEvaluationKind = {
  TimeLimit: 'timeLimit',
  ApprovalRequest: 'approvalRequest',
  CategoryRiskReview: 'categoryRiskReview',
  BlockLaunch: 'blockLaunch',
} as const;

export const AgentAppGamePolicyEvaluationRequestedAction = {
  TimeLimit: 'time-limit',
  AskParent: 'ask-parent',
  Warn: 'warn',
  BlockLaunch: 'block-launch',
} as const;

export const AgentAppGamePolicyEvaluationPolicyAction = {
  TimeLimit: 'time-limit',
  AskParent: 'ask-parent',
  Warn: 'warn',
  Block: 'block',
} as const;

export const AgentAppGamePolicyEvaluationDecisionState = {
  DryRunReady: 'dry-run-ready',
  ManualRequired: 'manual-required',
} as const;

export const AgentAppGamePolicyEvaluationRejectionReason = {
  None: 'none',
  MissingPolicyEvidence: 'missing-policy-evidence',
  MissingApprovalAuthority: 'missing-approval-authority',
  MissingPlatformAuthority: 'missing-platform-authority',
  MissingClassifierContext: 'missing-classifier-context',
  BlockLaunchManualRequired: 'block-launch-manual-required',
} as const;

export const AgentAppGamePolicyEvaluationRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    evaluationId: PolicyEvaluationText,
    evaluationKind: Schema.Literal(
      AgentAppGamePolicyEvaluationKind.TimeLimit,
      AgentAppGamePolicyEvaluationKind.ApprovalRequest,
      AgentAppGamePolicyEvaluationKind.CategoryRiskReview,
      AgentAppGamePolicyEvaluationKind.BlockLaunch
    ),
    requestedAction: Schema.Literal(
      AgentAppGamePolicyEvaluationRequestedAction.TimeLimit,
      AgentAppGamePolicyEvaluationRequestedAction.AskParent,
      AgentAppGamePolicyEvaluationRequestedAction.Warn,
      AgentAppGamePolicyEvaluationRequestedAction.BlockLaunch
    ),
    policyAction: Schema.Literal(
      AgentAppGamePolicyEvaluationPolicyAction.TimeLimit,
      AgentAppGamePolicyEvaluationPolicyAction.AskParent,
      AgentAppGamePolicyEvaluationPolicyAction.Warn,
      AgentAppGamePolicyEvaluationPolicyAction.Block
    ),
    decisionState: Schema.Literal(
      AgentAppGamePolicyEvaluationDecisionState.DryRunReady,
      AgentAppGamePolicyEvaluationDecisionState.ManualRequired
    ),
    rejectionReason: Schema.Literal(
      AgentAppGamePolicyEvaluationRejectionReason.None,
      AgentAppGamePolicyEvaluationRejectionReason.MissingPolicyEvidence,
      AgentAppGamePolicyEvaluationRejectionReason.MissingApprovalAuthority,
      AgentAppGamePolicyEvaluationRejectionReason.MissingPlatformAuthority,
      AgentAppGamePolicyEvaluationRejectionReason.MissingClassifierContext,
      AgentAppGamePolicyEvaluationRejectionReason.BlockLaunchManualRequired
    ),
    reasonCodes: Schema.Array(PolicyEvaluationText),
    requiredReadinessKinds: Schema.Array(AgentAppGamePolicyEvaluationRequiredReadinessKindSchema),
    evidenceReferenceIds: Schema.Array(PolicyEvaluationText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
    dryRun: Schema.Literal(true),
    enforcementHandoffState: Schema.Literal('disabled'),
    adapterDispatchState: Schema.Literal('not-dispatched'),
  })
);

export const AgentAppGamePolicyEvaluationReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: PolicyEvaluationText,
    custodyLabel: PolicyEvaluationText,
    capabilityStatus: PolicyEvaluationText,
    returned: PolicyEvaluationCount,
    policyEvaluationReady: Schema.Boolean,
    manualReviewRequired: Schema.Boolean,
    dryRun: Schema.Literal(true),
    enforcementHandoffState: Schema.Literal('disabled'),
    adapterDispatchClaimed: Schema.Literal(false),
    readinessRowCount: PolicyEvaluationCount,
    evaluatedRowCount: PolicyEvaluationCount,
    evidenceClaimRowCount: PolicyEvaluationCount,
    identityRowCount: PolicyEvaluationCount,
    approvalAuthorityRowCount: PolicyEvaluationCount,
    approvalActionResultRowCount: PolicyEvaluationCount,
    platformAuthorityRowCount: PolicyEvaluationCount,
    aiClassifierResultRowCount: PolicyEvaluationCount,
    rows: Schema.Array(AgentAppGamePolicyEvaluationRowSchema),
  })
);

export type AgentAppGamePolicyEvaluationKind = Infer<typeof AgentAppGamePolicyEvaluationRowSchema>['evaluationKind'];
export type AgentAppGamePolicyEvaluationDecisionState = Infer<
  typeof AgentAppGamePolicyEvaluationRowSchema
>['decisionState'];
export type AgentAppGamePolicyEvaluationRejectionReason = Infer<
  typeof AgentAppGamePolicyEvaluationRowSchema
>['rejectionReason'];
export type AgentAppGamePolicyEvaluationRow = Infer<typeof AgentAppGamePolicyEvaluationRowSchema>;
export type AgentAppGamePolicyEvaluationReadModel = Infer<typeof AgentAppGamePolicyEvaluationReadModelSchema>;

export type AgentAppGamePolicyEvaluationFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePolicyEvaluationResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGamePolicyEvaluationReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePolicyEvaluationFailureReason;
    };

export function parseAgentAppGamePolicyEvaluationEvent(event: AgentEventEnvelope): AgentAppGamePolicyEvaluationResult {
  if (event.event !== AgentEvent.ActivityAppGamePolicyEvaluationReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGamePolicyEvaluationReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGamePolicyEvaluationReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentAppGamePolicyEvaluationFailureReason): AgentAppGamePolicyEvaluationResult {
  return {
    ok: false,
    reason,
  };
}
