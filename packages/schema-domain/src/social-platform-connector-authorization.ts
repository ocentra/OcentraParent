import { type Infer, Schema, withParser } from './effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentActorIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  SocialPlatformConnectorAuthorizationIdSchema,
  SocialPlatformConnectorAuthorizationSchemaVersionSchema,
  SocialPlatformConnectorAuthorizationStateSchema,
  SocialPlatformConnectorBoundarySchema,
  SocialPlatformConnectorCustodyStateSchema,
  type SocialPlatformConnectorProvider,
  SocialPlatformConnectorProviderSchema,
  SocialPlatformConnectorProofRefsSchema,
  SocialPlatformConnectorProofStateSchema,
  SocialPlatformConnectorReasonsSchema,
  SocialPlatformConnectorScopesSchema,
} from './social-platform-connector-authorization-values';

const OptionalParentActorIdSchema = Schema.Union(ParentActorIdSchema, Schema.Null);
const OptionalParentTimestampSchema = Schema.Union(ParentTimestampSchema, Schema.Null);
const OptionalParentEvidenceReferenceIdSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);

const SocialPlatformConnectorAuthorizationRowBaseSchema = Schema.Struct({
  provider: SocialPlatformConnectorProviderSchema,
  authorizationState: SocialPlatformConnectorAuthorizationStateSchema,
  proofState: SocialPlatformConnectorProofStateSchema,
  custodyState: SocialPlatformConnectorCustodyStateSchema,
  scopes: SocialPlatformConnectorScopesSchema,
  reasons: SocialPlatformConnectorReasonsSchema,
  proofRefs: SocialPlatformConnectorProofRefsSchema,
  authorizedByActorId: OptionalParentActorIdSchema,
  authorizedAt: OptionalParentTimestampSchema,
  expiresAt: OptionalParentTimestampSchema,
  revokedAt: OptionalParentTimestampSchema,
  visibleParentSettingRef: OptionalParentEvidenceReferenceIdSchema,
  coreGatingDependency: Schema.Literal('not-required'),
  rawTokenStoredClaimed: Schema.Boolean,
  oauthClientImplementedClaimed: Schema.Boolean,
  providerApiCallClaimed: Schema.Boolean,
  rawAccountDataCaptured: Schema.Boolean,
  messageContentCaptured: Schema.Boolean,
  feedContentCaptured: Schema.Boolean,
  accountIdentityVerifiedClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  aiRuntimeClaimed: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialPlatformConnectorAuthorizationRowCandidate = Infer<typeof SocialPlatformConnectorAuthorizationRowBaseSchema>;

export const SocialPlatformConnectorAuthorizationRowSchema = withParser(
  SocialPlatformConnectorAuthorizationRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialPlatformConnectorAuthorizationRowIsHonest(row) ||
        'Expected social platform connector row to stay parent-authorized, manual-required, unavailable, or not-implemented without token, API, content, UI, policy, AI, native, or enforcement claims'
    )
  )
);

export const SocialPlatformConnectorClaimBoundariesSchema = withParser(
  Schema.Struct({
    tokenStorage: Schema.Literal('not-claimed'),
    oauthClient: Schema.Literal('not-claimed'),
    providerApiCalls: Schema.Literal('not-claimed'),
    rawAccountData: Schema.Literal('not-claimed'),
    messageContent: Schema.Literal('not-claimed'),
    feedContent: Schema.Literal('not-claimed'),
    accountIdentityVerification: Schema.Literal('not-claimed'),
    coreGatingDependency: Schema.Literal('not-claimed'),
    policyDecision: Schema.Literal('not-claimed'),
    aiRuntime: Schema.Literal('not-claimed'),
    uiDelivery: Schema.Literal('not-claimed'),
    nativeAppControl: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
    reviewerSummary: SocialPlatformConnectorBoundarySchema,
  })
);

const SocialPlatformConnectorAuthorizationBoundaryBaseSchema = Schema.Struct({
  schemaVersion: SocialPlatformConnectorAuthorizationSchemaVersionSchema,
  authorizationBoundaryId: SocialPlatformConnectorAuthorizationIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  generatedAt: ParentTimestampSchema,
  rows: Schema.Array(SocialPlatformConnectorAuthorizationRowSchema),
  claimBoundaries: SocialPlatformConnectorClaimBoundariesSchema,
});

type SocialPlatformConnectorAuthorizationBoundaryCandidate = Infer<
  typeof SocialPlatformConnectorAuthorizationBoundaryBaseSchema
>;

export const SocialPlatformConnectorAuthorizationBoundarySchema = withParser(
  SocialPlatformConnectorAuthorizationBoundaryBaseSchema.pipe(
    Schema.filter(
      (boundary) =>
        socialPlatformConnectorAuthorizationBoundaryIsHonest(boundary) ||
        'Expected social platform connector boundary to include all optional connector providers without core gating dependency or runtime claims'
    )
  )
);

export const decodeSocialPlatformConnectorAuthorizationBoundary = Schema.decodeUnknownSync(
  SocialPlatformConnectorAuthorizationBoundarySchema
);

export type SocialPlatformConnectorAuthorizationRow = Infer<typeof SocialPlatformConnectorAuthorizationRowSchema>;
export type SocialPlatformConnectorAuthorizationBoundary = Infer<
  typeof SocialPlatformConnectorAuthorizationBoundarySchema
>;

const RequiredSocialPlatformConnectorProviders = [
  'google-youtube-supervision',
  'meta-family-center',
  'tiktok-family-pairing',
  'platform-export-import',
  'parent-provided-account-ref',
] as const satisfies ReadonlyArray<SocialPlatformConnectorProvider>;

function socialPlatformConnectorAuthorizationBoundaryIsHonest(
  boundary: SocialPlatformConnectorAuthorizationBoundaryCandidate
): boolean {
  const providers = new Set(boundary.rows.map((row) => row.provider));
  return (
    providers.size === boundary.rows.length &&
    RequiredSocialPlatformConnectorProviders.every((provider) => providers.has(provider))
  );
}

function socialPlatformConnectorAuthorizationRowIsHonest(
  row: SocialPlatformConnectorAuthorizationRowCandidate
): boolean {
  if (socialPlatformConnectorAuthorizationRowClaimsRuntime(row)) {
    return false;
  }
  if (row.provider === 'parent-provided-account-ref') {
    return parentProvidedAccountRefRowIsHonest(row);
  }
  if (row.provider === 'platform-export-import') {
    return manualExportConnectorRowIsHonest(row);
  }
  return optionalProviderConnectorRowIsHonest(row);
}

function parentProvidedAccountRefRowIsHonest(row: SocialPlatformConnectorAuthorizationRowCandidate): boolean {
  return (
    row.authorizationState === 'parent-authorized' &&
    row.proofState === 'parent-consent-record-only' &&
    row.custodyState === 'redacted-parent-input-only' &&
    row.authorizedByActorId !== null &&
    row.authorizedAt !== null &&
    row.visibleParentSettingRef !== null &&
    row.revokedAt === null &&
    row.scopes.includes('parent-declared-account-ref') &&
    row.reasons.includes('redacted-input-required')
  );
}

function manualExportConnectorRowIsHonest(row: SocialPlatformConnectorAuthorizationRowCandidate): boolean {
  return (
    row.authorizationState === 'manual-required' &&
    row.proofState === 'manual-export-required' &&
    row.custodyState === 'manual-export-required' &&
    row.authorizedByActorId === null &&
    row.authorizedAt === null &&
    row.revokedAt === null &&
    row.scopes.includes('manual-export-file') &&
    row.reasons.includes('manual-export-required')
  );
}

function optionalProviderConnectorRowIsHonest(row: SocialPlatformConnectorAuthorizationRowCandidate): boolean {
  return (
    row.authorizationState === 'not-implemented' &&
    row.proofState === 'provider-artifact-required' &&
    row.custodyState === 'parent-owned-token-required' &&
    row.authorizedByActorId === null &&
    row.authorizedAt === null &&
    row.revokedAt === null &&
    row.visibleParentSettingRef === null &&
    row.reasons.includes('parent-authorization-required') &&
    row.reasons.includes('provider-api-not-implemented') &&
    row.reasons.includes('token-storage-not-implemented') &&
    providerScopeIsHonest(row)
  );
}

function providerScopeIsHonest(row: SocialPlatformConnectorAuthorizationRowCandidate): boolean {
  if (row.provider === 'google-youtube-supervision') {
    return row.scopes.includes('account-supervision-state') && row.scopes.includes('video-channel-metadata');
  }
  if (row.provider === 'meta-family-center') {
    return row.scopes.includes('family-center-state');
  }
  return row.scopes.includes('family-pairing-state');
}

function socialPlatformConnectorAuthorizationRowClaimsRuntime(
  row: SocialPlatformConnectorAuthorizationRowCandidate
): boolean {
  return (
    row.rawTokenStoredClaimed ||
    row.oauthClientImplementedClaimed ||
    row.providerApiCallClaimed ||
    row.rawAccountDataCaptured ||
    row.messageContentCaptured ||
    row.feedContentCaptured ||
    row.accountIdentityVerifiedClaimed ||
    row.nativeAppControlClaimed ||
    row.policyDecisionClaimed ||
    row.aiRuntimeClaimed ||
    row.uiDeliveredClaimed ||
    row.enforcementClaimed
  );
}
