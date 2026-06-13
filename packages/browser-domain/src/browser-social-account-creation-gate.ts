import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  type BrowserSocialAccountFlowEvidence,
  BrowserSocialAccountFlowEvidenceIdSchema,
  BrowserSocialAccountFlowEvidenceSchema,
  BrowserSocialAccountFlowKindSchema,
} from './browser-social-account-flow-schemas';
import {
  type BrowserSocialFormShapeEvidence,
  BrowserSocialFormShapeEvidenceIdSchema,
  BrowserSocialFormShapeEvidenceSchema,
  BrowserSocialFormShapeKindSchema,
} from './browser-social-form-shape-detector';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
} from './browser-social-platform-route-schemas';
const OptionalSocialGateTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const SocialGateSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social account gate source evidence ids')
);

export const BrowserSocialAccountCreationGateSchemaVersion = 1;

export const BrowserSocialAccountCreationGatePlanIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialAccountCreationGatePlanId')
);

export const BrowserSocialAccountCreationGateActionSchema = withParser(
  Schema.Literal(
    'allow-navigation-candidate',
    'hold-for-parent-approval',
    'block-submit-candidate',
    'manual-review-required',
    'unknown-flow-warn-only'
  )
);

export const BrowserSocialAccountCreationGateStateSchema = withParser(
  Schema.Literal('planned', 'manual-required', 'unavailable')
);

export const BrowserSocialAccountCreationGateReasonSchema = withParser(
  Schema.Literal(
    'signup-flow',
    'login-flow',
    'account-switch-flow',
    'form-shape-detected',
    'parent-policy-requires-approval',
    'policy-block-candidate',
    'manual-required',
    'unknown-flow'
  )
);

const GateReasonsSchema = Schema.Array(BrowserSocialAccountCreationGateReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social account gate reasons')
);

const BrowserSocialAccountCreationGateInputBaseSchema = Schema.Struct({
  gatePlanId: BrowserSocialAccountCreationGatePlanIdSchema,
  plannedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialGateSourceEvidenceIdsSchema,
  accountFlowEvidence: BrowserSocialAccountFlowEvidenceSchema,
  formShapeEvidence: BrowserSocialFormShapeEvidenceSchema,
  policyDecisionCandidateRef: OptionalSocialGateTextSchema,
  parentApprovalRequestRef: OptionalSocialGateTextSchema,
  gateAction: BrowserSocialAccountCreationGateActionSchema,
  parentApprovalRequired: Schema.Boolean,
  reasons: GateReasonsSchema,
});
const BrowserSocialAccountCreationGateInputSchema = withParser(
  BrowserSocialAccountCreationGateInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialAccountGateInputIsConsistent(value) ||
        'Expected matching route-only account-flow and sanitized form-shape evidence for account gate plan'
    )
  )
);

const BrowserSocialAccountCreationGatePlanBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialAccountCreationGateSchemaVersion),
  gatePlanId: BrowserSocialAccountCreationGatePlanIdSchema,
  plannedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialGateSourceEvidenceIdsSchema,
  accountFlowEvidenceId: BrowserSocialAccountFlowEvidenceIdSchema,
  formShapeEvidenceId: BrowserSocialFormShapeEvidenceIdSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  accountFlowKind: BrowserSocialAccountFlowKindSchema,
  formShapeKind: BrowserSocialFormShapeKindSchema,
  gateState: BrowserSocialAccountCreationGateStateSchema,
  gateAction: BrowserSocialAccountCreationGateActionSchema,
  parentApprovalRequired: Schema.Boolean,
  policyDecisionCandidateRef: OptionalSocialGateTextSchema,
  parentApprovalRequestRef: OptionalSocialGateTextSchema,
  reasons: GateReasonsSchema,
  navigationPausedClaimed: Schema.Boolean,
  formSubmitBlockedClaimed: Schema.Boolean,
  childUiRenderedClaimed: Schema.Boolean,
  parentUiNotifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  formSubmittedClaimed: Schema.Boolean,
  accountCreatedClaimed: Schema.Boolean,
});
export const BrowserSocialAccountCreationGatePlanSchema = withParser(
  BrowserSocialAccountCreationGatePlanBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialAccountGatePlanIsConsistent(value) ||
        'Expected social account gate plan to remain planned, auditable, and non-enforcing'
    )
  )
);

export const decodeBrowserSocialAccountCreationGatePlan = Schema.decodeUnknownSync(
  BrowserSocialAccountCreationGatePlanSchema
);

export type BrowserSocialAccountCreationGateInput = Infer<typeof BrowserSocialAccountCreationGateInputSchema>;
export type BrowserSocialAccountCreationGatePlan = Infer<typeof BrowserSocialAccountCreationGatePlanSchema>;

export function planBrowserSocialAccountCreationGate(
  input: BrowserSocialAccountCreationGateInput
): BrowserSocialAccountCreationGatePlan {
  const parsed = BrowserSocialAccountCreationGateInputSchema.parse(input);

  return BrowserSocialAccountCreationGatePlanSchema.parse({
    schemaVersion: BrowserSocialAccountCreationGateSchemaVersion,
    gatePlanId: parsed.gatePlanId,
    plannedAt: parsed.plannedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    accountFlowEvidenceId: parsed.accountFlowEvidence.accountFlowEvidenceId,
    formShapeEvidenceId: parsed.formShapeEvidence.formShapeEvidenceId,
    socialRouteEvidenceId: parsed.accountFlowEvidence.socialRouteEvidenceId,
    platform: parsed.accountFlowEvidence.platform,
    accountFlowKind: parsed.accountFlowEvidence.accountFlowKind,
    formShapeKind: parsed.formShapeEvidence.formShapeKind,
    gateState: parsed.gateAction === 'manual-review-required' ? 'manual-required' : 'planned',
    gateAction: parsed.gateAction,
    parentApprovalRequired: parsed.parentApprovalRequired,
    policyDecisionCandidateRef: parsed.policyDecisionCandidateRef,
    parentApprovalRequestRef: parsed.parentApprovalRequestRef,
    reasons: parsed.reasons,
    navigationPausedClaimed: false,
    formSubmitBlockedClaimed: false,
    childUiRenderedClaimed: false,
    parentUiNotifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    credentialCaptured: false,
    formSubmittedClaimed: false,
    accountCreatedClaimed: false,
  });
}

function socialAccountGateInputIsConsistent(value: Infer<typeof BrowserSocialAccountCreationGateInputBaseSchema>) {
  if (!accountFlowAndFormShapeMatch(value.accountFlowEvidence, value.formShapeEvidence)) {
    return false;
  }
  return gateActionRefsAreConsistent(value);
}

function socialAccountGatePlanIsConsistent(value: Infer<typeof BrowserSocialAccountCreationGatePlanBaseSchema>) {
  if (socialAccountGatePlanClaimsRuntime(value)) {
    return false;
  }
  if (value.gateAction === 'manual-review-required') {
    return value.gateState === 'manual-required' && value.reasons.includes('manual-required');
  }
  if (value.gateAction === 'hold-for-parent-approval') {
    return (
      value.gateState === 'planned' &&
      value.parentApprovalRequired &&
      value.parentApprovalRequestRef !== null &&
      value.reasons.includes('parent-policy-requires-approval')
    );
  }
  if (value.gateAction === 'block-submit-candidate') {
    return value.gateState === 'planned' && value.policyDecisionCandidateRef !== null;
  }
  return value.gateState === 'planned';
}

function accountFlowAndFormShapeMatch(
  accountFlow: BrowserSocialAccountFlowEvidence,
  formShape: BrowserSocialFormShapeEvidence
) {
  return (
    accountFlow.evidenceState === 'route-only' &&
    formShape.detectionState === 'detected' &&
    accountFlow.accountFlowEvidenceId === formShape.accountFlowEvidenceId &&
    accountFlow.socialRouteEvidenceId === formShape.socialRouteEvidenceId &&
    accountFlow.platform === formShape.platform
  );
}

function gateActionRefsAreConsistent(value: Infer<typeof BrowserSocialAccountCreationGateInputBaseSchema>) {
  if (value.gateAction === 'hold-for-parent-approval') {
    return value.parentApprovalRequired && value.parentApprovalRequestRef !== null;
  }
  if (value.gateAction === 'block-submit-candidate') {
    return value.policyDecisionCandidateRef !== null && !value.parentApprovalRequired;
  }
  if (value.gateAction === 'manual-review-required') {
    return value.reasons.includes('manual-required');
  }
  return value.policyDecisionCandidateRef !== null;
}

function socialAccountGatePlanClaimsRuntime(value: Infer<typeof BrowserSocialAccountCreationGatePlanBaseSchema>) {
  return (
    value.navigationPausedClaimed ||
    value.formSubmitBlockedClaimed ||
    value.childUiRenderedClaimed ||
    value.parentUiNotifiedClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed ||
    value.credentialCaptured ||
    value.formSubmittedClaimed ||
    value.accountCreatedClaimed
  );
}

