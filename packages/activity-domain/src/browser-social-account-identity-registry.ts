import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  type BrowserSocialAccountFlowEvidence,
  BrowserSocialAccountFlowEvidenceIdSchema,
  BrowserSocialAccountFlowEvidenceSchema,
} from './browser-social-account-flow-schemas';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
} from './browser-social-platform-route-schemas';

const NonEmptySocialAccountIdentityText = Schema.String.pipe(Schema.minLength(1));
const OptionalSocialAccountIdentityTextSchema = Schema.Union(NonEmptySocialAccountIdentityText, Schema.Null);
const SocialAccountIdentitySourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social account identity source evidence ids')
);

export const BrowserSocialAccountIdentityRegistrySchemaVersion = 1;

export const BrowserSocialAccountIdentityRegistryEntryIdSchema = withParser(
  NonEmptySocialAccountIdentityText.pipe(Schema.brand('BrowserSocialAccountIdentityRegistryEntryId'))
);

export const BrowserSocialAccountIdentityRefSchema = withParser(
  NonEmptySocialAccountIdentityText.pipe(Schema.brand('BrowserSocialAccountIdentityRef'))
);

export const BrowserSocialAccountIdentitySourceKindSchema = withParser(
  Schema.Literal('route-context-unverified', 'parent-declared-hash', 'manual-required')
);

export const BrowserSocialAccountIdentityStateSchema = withParser(
  Schema.Literal('unverified-route-context', 'parent-declared', 'manual-required')
);

const BrowserSocialAccountIdentityRegistryEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialAccountIdentityRegistrySchemaVersion),
  registryEntryId: BrowserSocialAccountIdentityRegistryEntryIdSchema,
  accountIdentityRef: BrowserSocialAccountIdentityRefSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialAccountIdentitySourceEvidenceIdsSchema,
  sourceKind: BrowserSocialAccountIdentitySourceKindSchema,
  identityState: BrowserSocialAccountIdentityStateSchema,
  platform: BrowserSocialPlatformSchema,
  socialRouteEvidenceId: Schema.Union(BrowserSocialRouteEvidenceIdSchema, Schema.Null),
  accountFlowEvidenceId: Schema.Union(BrowserSocialAccountFlowEvidenceIdSchema, Schema.Null),
  parentAssertionRef: OptionalSocialAccountIdentityTextSchema,
  handleHashRef: OptionalSocialAccountIdentityTextSchema,
  displayNameHashRef: OptionalSocialAccountIdentityTextSchema,
  platformAccountIdHashRef: OptionalSocialAccountIdentityTextSchema,
  rawHandleCaptured: Schema.Boolean,
  rawDisplayNameCaptured: Schema.Boolean,
  rawPlatformAccountIdCaptured: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  identityVerifiedByPlatform: Schema.Boolean,
  parentDeclaredIdentity: Schema.Boolean,
  childDeclaredIdentity: Schema.Boolean,
  accountCreationClaimed: Schema.Boolean,
  loginSuccessClaimed: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
});

export const BrowserSocialAccountIdentityRegistryEntrySchema = withParser(
  BrowserSocialAccountIdentityRegistryEntryBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialAccountIdentityEntryIsConsistent(value) ||
        'Expected social account identity registry entry to preserve privacy and verification boundaries'
    )
  )
);

const BrowserSocialAccountIdentityFromFlowInputSchema = withParser(
  Schema.Struct({
    registryEntryId: BrowserSocialAccountIdentityRegistryEntryIdSchema,
    accountIdentityRef: BrowserSocialAccountIdentityRefSchema,
    observedAt: ActivityTimestampSchema,
    sourceEvidenceIds: SocialAccountIdentitySourceEvidenceIdsSchema,
    accountFlowEvidence: BrowserSocialAccountFlowEvidenceSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        accountFlowCanBuildUnverifiedIdentityContext(value.accountFlowEvidence) ||
        'Expected route-only social account-flow evidence for unverified identity context'
    )
  )
);

export const decodeBrowserSocialAccountIdentityRegistryEntry = Schema.decodeUnknownSync(
  BrowserSocialAccountIdentityRegistryEntrySchema
);

export type BrowserSocialAccountIdentityFromFlowInput = Infer<typeof BrowserSocialAccountIdentityFromFlowInputSchema>;
export type BrowserSocialAccountIdentityRef = Infer<typeof BrowserSocialAccountIdentityRefSchema>;
export type BrowserSocialAccountIdentityRegistryEntry = Infer<typeof BrowserSocialAccountIdentityRegistryEntrySchema>;
export type BrowserSocialAccountIdentityRegistryEntryId = Infer<
  typeof BrowserSocialAccountIdentityRegistryEntryIdSchema
>;
export type BrowserSocialAccountIdentitySourceKind = Infer<typeof BrowserSocialAccountIdentitySourceKindSchema>;
export type BrowserSocialAccountIdentityState = Infer<typeof BrowserSocialAccountIdentityStateSchema>;

export function buildUnverifiedSocialAccountIdentityContextFromFlow(
  input: BrowserSocialAccountIdentityFromFlowInput
): BrowserSocialAccountIdentityRegistryEntry {
  const parsed = BrowserSocialAccountIdentityFromFlowInputSchema.parse(input);

  return BrowserSocialAccountIdentityRegistryEntrySchema.parse({
    schemaVersion: BrowserSocialAccountIdentityRegistrySchemaVersion,
    registryEntryId: parsed.registryEntryId,
    accountIdentityRef: parsed.accountIdentityRef,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    sourceKind: 'route-context-unverified',
    identityState: 'unverified-route-context',
    platform: parsed.accountFlowEvidence.platform,
    socialRouteEvidenceId: parsed.accountFlowEvidence.socialRouteEvidenceId,
    accountFlowEvidenceId: parsed.accountFlowEvidence.accountFlowEvidenceId,
    parentAssertionRef: null,
    handleHashRef: null,
    displayNameHashRef: null,
    platformAccountIdHashRef: null,
    rawHandleCaptured: false,
    rawDisplayNameCaptured: false,
    rawPlatformAccountIdCaptured: false,
    credentialCaptured: false,
    identityVerifiedByPlatform: false,
    parentDeclaredIdentity: false,
    childDeclaredIdentity: false,
    accountCreationClaimed: false,
    loginSuccessClaimed: false,
    connectorAuthorizationClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
  });
}

function browserSocialAccountIdentityEntryIsConsistent(
  value: Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>
) {
  if (socialAccountIdentityEntryClaimsAuthority(value)) {
    return false;
  }
  if (value.sourceKind === 'route-context-unverified') {
    return routeContextIdentityEntryIsConsistent(value);
  }
  if (value.sourceKind === 'parent-declared-hash') {
    return parentDeclaredIdentityEntryIsConsistent(value);
  }
  return manualRequiredIdentityEntryIsConsistent(value);
}

function socialAccountIdentityEntryClaimsAuthority(
  value: Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>
) {
  return SocialAccountIdentityAuthorityClaimFields.some((field) => value[field] === true);
}

type BrowserSocialAccountIdentityEntryCandidate = Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>;

const SocialAccountIdentityAuthorityClaimFields = [
  'rawHandleCaptured',
  'rawDisplayNameCaptured',
  'rawPlatformAccountIdCaptured',
  'credentialCaptured',
  'identityVerifiedByPlatform',
  'childDeclaredIdentity',
  'accountCreationClaimed',
  'loginSuccessClaimed',
  'connectorAuthorizationClaimed',
  'aiDecisionClaimed',
  'policyDecisionClaimed',
  'enforcementClaimed',
  'nativeAppControlClaimed',
] as const satisfies ReadonlyArray<keyof BrowserSocialAccountIdentityEntryCandidate>;

function routeContextIdentityEntryIsConsistent(
  value: Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>
) {
  return (
    value.identityState === 'unverified-route-context' &&
    value.socialRouteEvidenceId !== null &&
    value.accountFlowEvidenceId !== null &&
    value.parentAssertionRef === null &&
    value.handleHashRef === null &&
    value.displayNameHashRef === null &&
    value.platformAccountIdHashRef === null &&
    !value.parentDeclaredIdentity
  );
}

function parentDeclaredIdentityEntryIsConsistent(
  value: Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>
) {
  return (
    value.identityState === 'parent-declared' &&
    value.parentAssertionRef !== null &&
    (value.handleHashRef !== null || value.displayNameHashRef !== null || value.platformAccountIdHashRef !== null) &&
    value.socialRouteEvidenceId === null &&
    value.accountFlowEvidenceId === null &&
    value.parentDeclaredIdentity
  );
}

function manualRequiredIdentityEntryIsConsistent(
  value: Infer<typeof BrowserSocialAccountIdentityRegistryEntryBaseSchema>
) {
  return (
    value.identityState === 'manual-required' &&
    value.socialRouteEvidenceId === null &&
    value.accountFlowEvidenceId === null &&
    value.parentAssertionRef === null &&
    value.handleHashRef === null &&
    value.displayNameHashRef === null &&
    value.platformAccountIdHashRef === null &&
    !value.parentDeclaredIdentity
  );
}

function accountFlowCanBuildUnverifiedIdentityContext(value: BrowserSocialAccountFlowEvidence) {
  return value.evidenceState === 'route-only' && !value.accountIdentityClaimed && value.accountIdentityRef === null;
}
