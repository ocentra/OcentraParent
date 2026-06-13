import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  approvalDecisionPersistenceIsConsistent,
  approvalDecisionResponseScopeIsConsistent,
  approvalRequestCandidateRefsAreConsistent,
  actionResultApprovalStateIsConsistent,
  actionResultCapabilityIsConsistent,
  actionResultEvidenceProofIsConsistent,
  authorityGrantStateIsConsistent,
  decisionParentActionPresenceIsConsistent,
  decisionPolicyVersionMatchesParentAction,
  requestSettingRefsMatchPolicyKind,
} from './app-game-control-authority-rules';
import {
  EnforcementCapabilityStateSchema,
  EnforcementCapabilityStatusSchema,
  EnforcementModeSchema,
  EnforcementResultSchema,
} from '@ocentra-parent/enforcement-domain/enforcement';
import { EnforcementPolicyDispatchApprovalStateSchema } from '@ocentra-parent/enforcement-domain/enforcement-policy-dispatch';
import {
  AppGameControlApprovalCandidateSchema,
  AppGameControlApprovalFlowReferenceSchema,
  AppGameControlApprovalPersistenceStateSchema,
  AppGameControlChildReasonStateSchema,
  AppGameControlSettingReferenceSchema,
  AppGameControlUnansweredFallbackSchema,
  AppGameControlParentResponseScopeSchema,
} from './app-game-control-approval-flow';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyTargetSchema } from '@ocentra-parent/policy-domain/policy';

export * from './app-game-control-platform-authority';
export * from './app-game-control-approval-flow';
const ApprovalFlowRefsSchema = Schema.Array(AppGameControlApprovalFlowReferenceSchema);
const OptionalApprovalFlowRefsSchema = Schema.optionalWith(ApprovalFlowRefsSchema, { default: () => [] });

export const AppGameControlPolicyKindSchema = withParser(Schema.Literal('app-control', 'game-control'));

export const AppGameControlAuthorityStateSchema = withParser(
  Schema.Literal('active', 'observe-only', 'manual-required', 'unavailable')
);

export const AppGameControlApprovalDecisionStateSchema = withParser(
  Schema.Literal('approved', 'denied', 'expired', 'override', 'manual-required')
);

export const AppGameControlActionResultStatusSchema = withParser(
  Schema.Literal('not-dispatched', 'dispatch-ready', 'would-enforce', 'enforced', 'manual-required', 'unavailable')
);

export const AppGameControlEvidenceProofKindSchema = withParser(
  Schema.Literal(
    'app-identity-proof',
    'gameplay-proof',
    'launcher-only',
    'unknown-app',
    'unknown-game-like',
    'catalog-match',
    'process-observation'
  )
);

const AppGameControlApprovalAuthorityBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  authorityId: brandedNonEmptyStringSchema('AppGameControlAuthorityId'),
  actor: ParentActorReferenceSchema,
  device: ParentDeviceReferenceSchema,
  policyVersion: ParentPolicyVersionSchema,
  authorityState: AppGameControlAuthorityStateSchema,
  allowedPolicyKinds: Schema.Array(AppGameControlPolicyKindSchema),
  canApprove: Schema.Boolean,
  canDeny: Schema.Boolean,
  canExtend: Schema.Boolean,
  canOverride: Schema.Boolean,
  canObserveOnly: Schema.Boolean,
  checkedAt: ParentTimestampSchema,
});

export const AppGameControlApprovalAuthoritySchema = withParser(
  AppGameControlApprovalAuthorityBaseSchema.pipe(
    Schema.filter(
      (authority) =>
        authorityGrantStateIsConsistent(authority) ||
        'Expected observe-only, manual-required, and unavailable app/game authority to avoid write grants'
    )
  )
);

const AppGameControlApprovalRequestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  requestId: brandedNonEmptyStringSchema('AppGameControlApprovalRequestId'),
  policyKind: AppGameControlPolicyKindSchema,
  device: ParentDeviceReferenceSchema,
  target: PolicyTargetSchema,
  requestedAction: PolicyActionSchema,
  requestedMode: Schema.Union(EnforcementModeSchema, Schema.Null),
  requestedSettingRefs: Schema.Array(AppGameControlSettingReferenceSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  candidate: Schema.optionalWith(Schema.Union(AppGameControlApprovalCandidateSchema, Schema.Null), {
    default: () => null,
  }),
  childReasonState: Schema.optionalWith(AppGameControlChildReasonStateSchema, {
    default: () => 'not-requested' as const,
  }),
  childReasonReferences: OptionalApprovalFlowRefsSchema,
  childStatusReferences: OptionalApprovalFlowRefsSchema,
  expiresAt: ParentTimestampSchema,
  unansweredFallback: AppGameControlUnansweredFallbackSchema,
});

export const AppGameControlApprovalRequestSchema = withParser(
  AppGameControlApprovalRequestBaseSchema.pipe(
    Schema.filter(
      (request) => request.evidenceReferences.length > 0 || 'Expected app/game approval requests to cite evidence'
    )
  )
    .pipe(
      Schema.filter(
        (request) =>
          requestSettingRefsMatchPolicyKind(request) ||
          'Expected app/game approval request setting refs to match the policy kind'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          approvalRequestCandidateRefsAreConsistent(request) ||
          'Expected app/game approval candidates to carry evidence, child status refs, and safe weak-game fallback'
      )
    )
);

const AppGameControlApprovalDecisionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  decisionId: brandedNonEmptyStringSchema('AppGameControlApprovalDecisionId'),
  requestId: brandedNonEmptyStringSchema('AppGameControlApprovalRequestId'),
  policyKind: AppGameControlPolicyKindSchema,
  decisionState: AppGameControlApprovalDecisionStateSchema,
  parentAction: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  reasonCodes: Schema.Array(PolicyReasonCodeSchema),
  policyVersion: ParentPolicyVersionSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  responseScope: Schema.optionalWith(Schema.Union(AppGameControlParentResponseScopeSchema, Schema.Null), {
    default: () => null,
  }),
  decisionExpiresAt: Schema.optionalWith(Schema.Union(ParentTimestampSchema, Schema.Null), {
    default: () => null,
  }),
  auditReferences: OptionalApprovalFlowRefsSchema,
  persistenceState: Schema.optionalWith(AppGameControlApprovalPersistenceStateSchema, {
    default: () => 'not-persisted' as const,
  }),
  decidedAt: ParentTimestampSchema,
});

export const AppGameControlApprovalDecisionSchema = withParser(
  AppGameControlApprovalDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) => decision.evidenceReferences.length > 0 || 'Expected app/game approval decisions to cite evidence'
    )
  )
    .pipe(
      Schema.filter(
        (decision) =>
          decisionPolicyVersionMatchesParentAction(decision) ||
          'Expected app/game approval decision policy version to match the parent action reference'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decisionParentActionPresenceIsConsistent(decision) ||
          'Expected approved or override app/game decisions to include a parent action reference'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          approvalDecisionResponseScopeIsConsistent(decision) ||
          'Expected app/game approval decision response scope to match decision state and expiry'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          approvalDecisionPersistenceIsConsistent(decision) ||
          'Expected replayable app/game approval decisions to carry audit refs'
      )
    )
);

const AppGameControlActionResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  resultId: brandedNonEmptyStringSchema('AppGameControlActionResultId'),
  request: AppGameControlApprovalRequestSchema,
  decision: AppGameControlApprovalDecisionSchema,
  approvalState: EnforcementPolicyDispatchApprovalStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  capability: EnforcementCapabilityStatusSchema,
  evidenceProofKind: AppGameControlEvidenceProofKindSchema,
  resultStatus: AppGameControlActionResultStatusSchema,
  enforcementResult: Schema.Union(EnforcementResultSchema, Schema.Null),
  recordedAt: ParentTimestampSchema,
});

export const AppGameControlActionResultSchema = withParser(
  AppGameControlActionResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        result.request.policyKind === result.decision.policyKind ||
        'Expected app/game action result request and decision policy kinds to match'
    )
  )
    .pipe(
      Schema.filter(
        (result) =>
          actionResultApprovalStateIsConsistent(result) ||
          'Expected app/game action result status to match approval state'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          actionResultCapabilityIsConsistent(result) ||
          'Expected app/game action result to keep manual-required, degraded, and unavailable capabilities out of dispatch-ready execution'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          actionResultEvidenceProofIsConsistent(result) ||
          'Expected launcher-only and unknown app evidence to remain non-enforceable without stronger proof'
      )
    )
);

