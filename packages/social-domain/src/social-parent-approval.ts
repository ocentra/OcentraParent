import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentActorIdSchema,
  ParentContractSchemaVersionSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptySocialParentApprovalText = Schema.String.pipe(Schema.minLength(1));
const OptionalSocialParentApprovalTextSchema = Schema.Union(NonEmptySocialParentApprovalText, Schema.Null);
const SocialParentApprovalEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social parent approval evidence refs')
);

export const SocialParentApprovalRequestIdSchema = withParser(
  NonEmptySocialParentApprovalText.pipe(Schema.brand('SocialParentApprovalRequestId'))
);

export const SocialParentApprovalDecisionIdSchema = withParser(
  NonEmptySocialParentApprovalText.pipe(Schema.brand('SocialParentApprovalDecisionId'))
);

export const SocialParentApprovalSubjectKindSchema = withParser(
  Schema.Literal('social-account-signup', 'social-login', 'social-account-switch', 'social-route-manual-required')
);

export const SocialParentApprovalRequestStateSchema = withParser(
  Schema.Literal('pending', 'expired', 'cancelled', 'manual-required')
);

export const SocialParentApprovalDecisionKindSchema = withParser(
  Schema.Literal('allow-once', 'allow-account', 'deny', 'manual-required')
);

export const SocialParentApprovalDecisionStateSchema = withParser(Schema.Literal('recorded', 'manual-required'));

const SocialParentApprovalRequestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  approvalRequestId: SocialParentApprovalRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  requestedByDeviceId: ParentDeviceIdSchema,
  createdAt: ParentTimestampSchema,
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  subjectKind: SocialParentApprovalSubjectKindSchema,
  requestState: SocialParentApprovalRequestStateSchema,
  sourceEvidenceRefs: SocialParentApprovalEvidenceRefsSchema,
  socialRouteEvidenceRef: OptionalSocialParentApprovalTextSchema,
  accountFlowEvidenceRef: OptionalSocialParentApprovalTextSchema,
  formShapeEvidenceRef: OptionalSocialParentApprovalTextSchema,
  accountIdentityRef: OptionalSocialParentApprovalTextSchema,
  deliveryState: Schema.Literal('contract-only'),
  rawMessageCaptured: Schema.Boolean,
  rawAccountIdentityCaptured: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
});

const SocialParentApprovalDecisionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  approvalDecisionId: SocialParentApprovalDecisionIdSchema,
  approvalRequestId: SocialParentApprovalRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  decidedAt: ParentTimestampSchema,
  decidedByActorId: Schema.Union(ParentActorIdSchema, Schema.Null),
  decisionKind: SocialParentApprovalDecisionKindSchema,
  decisionState: SocialParentApprovalDecisionStateSchema,
  sourceEvidenceRefs: SocialParentApprovalEvidenceRefsSchema,
  policyVersionRef: OptionalSocialParentApprovalTextSchema,
  actionRef: OptionalSocialParentApprovalTextSchema,
  deliveryState: Schema.Literal('contract-only'),
  notificationDeliveredClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  childNotifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
});

export const SocialParentApprovalRequestSchema = withParser(
  SocialParentApprovalRequestBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialParentApprovalRequestIsConsistent(value) ||
        'Expected social parent approval request to preserve contract-only request boundaries'
    )
  )
);

export const SocialParentApprovalDecisionSchema = withParser(
  SocialParentApprovalDecisionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialParentApprovalDecisionIsConsistent(value) ||
        'Expected social parent approval decision to preserve contract-only decision boundaries'
    )
  )
);

export const decodeSocialParentApprovalRequest = Schema.decodeUnknownSync(SocialParentApprovalRequestSchema);
export const decodeSocialParentApprovalDecision = Schema.decodeUnknownSync(SocialParentApprovalDecisionSchema);

export type SocialParentApprovalDecision = Infer<typeof SocialParentApprovalDecisionSchema>;
export type SocialParentApprovalDecisionId = Infer<typeof SocialParentApprovalDecisionIdSchema>;
export type SocialParentApprovalDecisionKind = Infer<typeof SocialParentApprovalDecisionKindSchema>;
export type SocialParentApprovalDecisionState = Infer<typeof SocialParentApprovalDecisionStateSchema>;
export type SocialParentApprovalRequest = Infer<typeof SocialParentApprovalRequestSchema>;
export type SocialParentApprovalRequestId = Infer<typeof SocialParentApprovalRequestIdSchema>;
export type SocialParentApprovalRequestState = Infer<typeof SocialParentApprovalRequestStateSchema>;
export type SocialParentApprovalSubjectKind = Infer<typeof SocialParentApprovalSubjectKindSchema>;

function socialParentApprovalRequestIsConsistent(value: Infer<typeof SocialParentApprovalRequestBaseSchema>) {
  if (socialParentApprovalRequestClaimsRuntime(value)) {
    return false;
  }
  if (value.subjectKind === 'social-route-manual-required') {
    return value.requestState === 'manual-required' && value.accountFlowEvidenceRef === null;
  }
  return value.requestState !== 'manual-required' && value.accountFlowEvidenceRef !== null;
}

function socialParentApprovalDecisionIsConsistent(value: Infer<typeof SocialParentApprovalDecisionBaseSchema>) {
  if (socialParentApprovalDecisionClaimsRuntime(value)) {
    return false;
  }
  if (value.decisionKind === 'manual-required') {
    return (
      value.decisionState === 'manual-required' &&
      value.decidedByActorId === null &&
      value.policyVersionRef === null &&
      value.actionRef === null
    );
  }
  return value.decisionState === 'recorded' && value.decidedByActorId !== null && value.actionRef === null;
}

function socialParentApprovalRequestClaimsRuntime(value: Infer<typeof SocialParentApprovalRequestBaseSchema>) {
  return (
    value.rawMessageCaptured ||
    value.rawAccountIdentityCaptured ||
    value.credentialCaptured ||
    value.notificationDeliveredClaimed ||
    value.uiRenderedClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.connectorAuthorizationClaimed
  );
}

function socialParentApprovalDecisionClaimsRuntime(value: Infer<typeof SocialParentApprovalDecisionBaseSchema>) {
  return (
    value.notificationDeliveredClaimed ||
    value.uiRenderedClaimed ||
    value.childNotifiedClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.connectorAuthorizationClaimed
  );
}
