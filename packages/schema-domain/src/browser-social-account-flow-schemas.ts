import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserSocialPlatformSchema,
  type BrowserSocialRouteEvidence,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteEvidenceSchema,
  type BrowserSocialRouteKind,
  BrowserSocialRouteKindSchema,
} from './browser-social-platform-route-schemas';
const OptionalSocialAccountFlowTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const SocialAccountFlowSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social account flow source evidence ids')
);

export const BrowserSocialAccountFlowSchemaVersion = 1;

export const BrowserSocialAccountFlowEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialAccountFlowEvidenceId')
);

export const BrowserSocialAccountFlowKindSchema = withParser(
  Schema.Literal('signup-route', 'login-route', 'account-switch-route', 'manual-required', 'unavailable')
);

export const BrowserSocialAccountFlowEvidenceStateSchema = withParser(
  Schema.Literal('route-only', 'manual-required', 'unavailable')
);

const BrowserSocialAccountFlowEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialAccountFlowSchemaVersion),
  accountFlowEvidenceId: BrowserSocialAccountFlowEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialAccountFlowSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  accountFlowKind: BrowserSocialAccountFlowKindSchema,
  evidenceState: BrowserSocialAccountFlowEvidenceStateSchema,
  accountIdentityRef: OptionalSocialAccountFlowTextSchema,
  parentApprovalRequestRef: OptionalSocialAccountFlowTextSchema,
  exactManagedBrowserRouteEvidence: Schema.Boolean,
  manualRequired: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  formFieldValuesCaptured: Schema.Boolean,
  formSubmissionClaimed: Schema.Boolean,
  accountCreationCompletedClaimed: Schema.Boolean,
  loginSuccessClaimed: Schema.Boolean,
  accountSwitchCompletedClaimed: Schema.Boolean,
  parentApprovalDecisionClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});

export const BrowserSocialAccountFlowEvidenceSchema = withParser(
  BrowserSocialAccountFlowEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialAccountFlowEvidenceIsConsistent(value) ||
        'Expected browser social account flow evidence to preserve route-only account-flow boundaries'
    )
  )
);

const BrowserSocialAccountFlowFromRouteInputSchema = withParser(
  Schema.Struct({
    accountFlowEvidenceId: BrowserSocialAccountFlowEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    sourceEvidenceIds: SocialAccountFlowSourceEvidenceIdsSchema,
    routeEvidence: BrowserSocialRouteEvidenceSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        routeEvidenceCanBuildAccountFlow(value.routeEvidence) ||
        'Expected managed browser signup/login/account-switch social route evidence'
    )
  )
);

export const decodeBrowserSocialAccountFlowEvidence = Schema.decodeUnknownSync(BrowserSocialAccountFlowEvidenceSchema);

export type BrowserSocialAccountFlowEvidence = Infer<typeof BrowserSocialAccountFlowEvidenceSchema>;
export type BrowserSocialAccountFlowEvidenceId = Infer<typeof BrowserSocialAccountFlowEvidenceIdSchema>;
export type BrowserSocialAccountFlowEvidenceState = Infer<typeof BrowserSocialAccountFlowEvidenceStateSchema>;
export type BrowserSocialAccountFlowFromRouteInput = Infer<typeof BrowserSocialAccountFlowFromRouteInputSchema>;
export type BrowserSocialAccountFlowKind = Infer<typeof BrowserSocialAccountFlowKindSchema>;

export function buildBrowserSocialAccountFlowEvidenceFromRoute(
  input: BrowserSocialAccountFlowFromRouteInput
): BrowserSocialAccountFlowEvidence {
  const parsed = BrowserSocialAccountFlowFromRouteInputSchema.parse(input);

  return BrowserSocialAccountFlowEvidenceSchema.parse({
    schemaVersion: BrowserSocialAccountFlowSchemaVersion,
    accountFlowEvidenceId: parsed.accountFlowEvidenceId,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    socialRouteEvidenceId: parsed.routeEvidence.socialRouteEvidenceId,
    platform: parsed.routeEvidence.platform,
    routeKind: parsed.routeEvidence.routeKind,
    accountFlowKind: accountFlowKindForRouteKind(parsed.routeEvidence.routeKind),
    evidenceState: 'route-only',
    accountIdentityRef: null,
    parentApprovalRequestRef: null,
    exactManagedBrowserRouteEvidence: true,
    manualRequired: false,
    accountIdentityClaimed: false,
    credentialCaptured: false,
    formFieldValuesCaptured: false,
    formSubmissionClaimed: false,
    accountCreationCompletedClaimed: false,
    loginSuccessClaimed: false,
    accountSwitchCompletedClaimed: false,
    parentApprovalDecisionClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function browserSocialAccountFlowEvidenceIsConsistent(value: Infer<typeof BrowserSocialAccountFlowEvidenceBaseSchema>) {
  if (socialAccountFlowEvidenceClaimsAuthority(value)) {
    return false;
  }
  if (value.accountIdentityRef !== null || value.parentApprovalRequestRef !== null) {
    return false;
  }
  if (value.evidenceState === 'route-only') {
    return routeOnlyAccountFlowEvidenceIsConsistent(value);
  }
  return manualOrUnavailableAccountFlowEvidenceIsConsistent(value);
}

function socialAccountFlowEvidenceClaimsAuthority(value: Infer<typeof BrowserSocialAccountFlowEvidenceBaseSchema>) {
  return SocialAccountFlowAuthorityClaimFields.some((field) => value[field] === true);
}

type BrowserSocialAccountFlowEvidenceCandidate = Infer<typeof BrowserSocialAccountFlowEvidenceBaseSchema>;

const SocialAccountFlowAuthorityClaimFields = [
  'accountIdentityClaimed',
  'credentialCaptured',
  'formFieldValuesCaptured',
  'formSubmissionClaimed',
  'accountCreationCompletedClaimed',
  'loginSuccessClaimed',
  'accountSwitchCompletedClaimed',
  'parentApprovalDecisionClaimed',
  'aiDecisionClaimed',
  'policyDecisionClaimed',
  'enforcementClaimed',
  'nativeAppControlClaimed',
  'platformConnectorClaimed',
] as const satisfies ReadonlyArray<keyof BrowserSocialAccountFlowEvidenceCandidate>;

function routeOnlyAccountFlowEvidenceIsConsistent(value: Infer<typeof BrowserSocialAccountFlowEvidenceBaseSchema>) {
  return (
    value.exactManagedBrowserRouteEvidence &&
    !value.manualRequired &&
    accountFlowKindForRouteKind(value.routeKind) === value.accountFlowKind
  );
}

function manualOrUnavailableAccountFlowEvidenceIsConsistent(
  value: Infer<typeof BrowserSocialAccountFlowEvidenceBaseSchema>
) {
  const expectedKind = value.evidenceState === 'manual-required' ? 'manual-required' : 'unavailable';
  return (
    !value.exactManagedBrowserRouteEvidence &&
    value.manualRequired &&
    value.routeKind === 'unknown-social-route' &&
    value.accountFlowKind === expectedKind
  );
}

function routeEvidenceCanBuildAccountFlow(value: BrowserSocialRouteEvidence) {
  return (
    value.sourceKind === 'managed-browser-url-shape' &&
    value.exactManagedBrowserRouteEvidence &&
    value.proofState === 'route-evidence' &&
    accountFlowKindForRouteKind(value.routeKind) !== null
  );
}

function accountFlowKindForRouteKind(value: BrowserSocialRouteKind) {
  if (value === 'account-signup') {
    return 'signup-route' as const;
  }
  if (value === 'login') {
    return 'login-route' as const;
  }
  if (value === 'account-switch') {
    return 'account-switch-route' as const;
  }
  return null;
}
