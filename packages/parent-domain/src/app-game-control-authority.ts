import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
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
} from './enforcement';
import { EnforcementPolicyDispatchApprovalStateSchema } from './enforcement-policy-dispatch';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyTargetSchema } from './policy';

const NonEmptyAppGameControlText = Schema.String.pipe(Schema.minLength(1));

export const AppGameControlPolicyKindSchema = withParser(Schema.Literal('app-control', 'game-control'));

export const AppGameControlAuthorityStateSchema = withParser(
  Schema.Literal('active', 'observe-only', 'manual-required', 'unavailable')
);

export const AppGameControlApprovalDecisionStateSchema = withParser(
  Schema.Literal('approved', 'denied', 'expired', 'override', 'manual-required')
);

export const AppGameControlUnansweredFallbackSchema = withParser(
  Schema.Literal('deny', 'expire', 'observe-only', 'manual-required')
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
    'catalog-match',
    'process-observation'
  )
);

const AppGameControlSettingReferenceSchema = withParser(
  Schema.Struct({
    settingId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlSettingId')),
    writesTo: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlWritePath')),
  })
);

const AppGameControlApprovalAuthorityBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  authorityId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlAuthorityId')),
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
  requestId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlApprovalRequestId')),
  policyKind: AppGameControlPolicyKindSchema,
  device: ParentDeviceReferenceSchema,
  target: PolicyTargetSchema,
  requestedAction: PolicyActionSchema,
  requestedMode: Schema.Union(EnforcementModeSchema, Schema.Null),
  requestedSettingRefs: Schema.Array(AppGameControlSettingReferenceSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  expiresAt: ParentTimestampSchema,
  unansweredFallback: AppGameControlUnansweredFallbackSchema,
});

export const AppGameControlApprovalRequestSchema = withParser(
  AppGameControlApprovalRequestBaseSchema.pipe(
    Schema.filter(
      (request) => request.evidenceReferences.length > 0 || 'Expected app/game approval requests to cite evidence'
    )
  ).pipe(
    Schema.filter(
      (request) =>
        requestSettingRefsMatchPolicyKind(request) ||
        'Expected app/game approval request setting refs to match the policy kind'
    )
  )
);

const AppGameControlApprovalDecisionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  decisionId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlApprovalDecisionId')),
  requestId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlApprovalRequestId')),
  policyKind: AppGameControlPolicyKindSchema,
  decisionState: AppGameControlApprovalDecisionStateSchema,
  parentAction: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  reasonCodes: Schema.Array(PolicyReasonCodeSchema),
  policyVersion: ParentPolicyVersionSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
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
);

const AppGameControlActionResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  resultId: NonEmptyAppGameControlText.pipe(Schema.brand('AppGameControlActionResultId')),
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
