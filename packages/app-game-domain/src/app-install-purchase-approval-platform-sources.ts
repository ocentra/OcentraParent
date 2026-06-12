import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyPlatformSourceText = Schema.String.pipe(Schema.minLength(1));
const RequiredRequestKinds = ['install', 'purchase', 'subscription'] as const;
const RequiredMetadataFields = [
  'store-listing-id',
  'app-title',
  'publisher-name',
  'category',
  'age-rating',
  'price-display',
  'subscription-period',
  'source-url',
] as const;
const RequiredPlatformSources = [
  { platform: 'windows', storeSurface: 'microsoft-store' },
  { platform: 'macos', storeSurface: 'mac-app-store' },
  { platform: 'linux', storeSurface: 'linux-package-manager' },
  { platform: 'android', storeSurface: 'google-play' },
  { platform: 'ios', storeSurface: 'apple-app-store' },
] as const;

const AppInstallPurchaseApprovalPlatformSourceSchemaVersionSchema = withParser(
  Schema.Literal('app-install-purchase-approval-contract-proof')
);
const AppInstallPurchaseApprovalPlatformSourceRequestKindSchema = withParser(
  Schema.Literal('install', 'purchase', 'subscription')
);
const AppInstallPurchaseApprovalPlatformSourceStoreSurfaceSchema = withParser(
  Schema.Literal('google-play', 'apple-app-store', 'mac-app-store', 'microsoft-store', 'linux-package-manager')
);
const AppInstallPurchaseApprovalPlatformSourceAuthoritySchema = withParser(
  Schema.Literal(
    'google-play-listing',
    'apple-app-store-listing',
    'mac-app-store-listing',
    'microsoft-store-listing',
    'linux-package-manager-index'
  )
);
export const AppInstallPurchaseApprovalPlatformSourceMetadataStateSchema = withParser(
  Schema.Literal('contract-only', 'manual-required', 'unavailable')
);
const AppInstallPurchaseApprovalPlatformSourceEvidenceStateSchema = withParser(
  Schema.Literal('requires-approved-api-proof', 'requires-store-artifact-proof', 'platform-unavailable')
);
const AppInstallPurchaseApprovalPlatformSourceMetadataFieldSchema = withParser(
  Schema.Literal(
    'store-listing-id',
    'app-title',
    'publisher-name',
    'category',
    'age-rating',
    'price-display',
    'subscription-period',
    'source-url'
  )
);
const AppInstallPurchaseApprovalPlatformSourceManualFallbackSchema = withParser(
  Schema.Literal('contract-only-parent-review')
);
const AppInstallPurchaseApprovalPlatformSourceStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovalPlatformSourcePlatformAdapterClaimSchema = withParser(
  Schema.Literal('not-implemented')
);
const AppInstallPurchaseApprovalPlatformSourceInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));

const AppInstallPurchaseApprovalPlatformSourceRowIdSchema = NonEmptyPlatformSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPlatformSourceRowId')
);
const AppInstallPurchaseApprovalPlatformSourceArtifactRequirementSchema = NonEmptyPlatformSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPlatformSourceArtifactRequirement')
);
const AppInstallPurchaseApprovalPlatformSourceLimitationReasonSchema = NonEmptyPlatformSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPlatformSourceLimitationReason')
);
const AppInstallPurchaseApprovalPlatformSourceReportRefSchema = NonEmptyPlatformSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPlatformSourceReportRef')
);
const AppInstallPurchaseApprovalPlatformSourceClaimBoundarySchema = NonEmptyPlatformSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPlatformSourceClaimBoundary')
);

const AppInstallPurchaseApprovalPlatformSourceMetadataRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalPlatformSourceSchemaVersionSchema,
  sourceRowId: AppInstallPurchaseApprovalPlatformSourceRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseApprovalPlatformSourceStoreSurfaceSchema,
  sourceAuthority: AppInstallPurchaseApprovalPlatformSourceAuthoritySchema,
  metadataState: AppInstallPurchaseApprovalPlatformSourceMetadataStateSchema,
  sourceEvidenceState: AppInstallPurchaseApprovalPlatformSourceEvidenceStateSchema,
  fieldsAvailableFromContract: Schema.Array(AppInstallPurchaseApprovalPlatformSourceMetadataFieldSchema),
  fieldsRequiringPlatformProof: Schema.Array(AppInstallPurchaseApprovalPlatformSourceMetadataFieldSchema),
  requestKindCoverage: Schema.Array(AppInstallPurchaseApprovalPlatformSourceRequestKindSchema),
  requiredArtifacts: Schema.Array(AppInstallPurchaseApprovalPlatformSourceArtifactRequirementSchema),
  limitationReason: AppInstallPurchaseApprovalPlatformSourceLimitationReasonSchema,
  limitationReportRef: AppInstallPurchaseApprovalPlatformSourceReportRefSchema,
  parentManualFallback: AppInstallPurchaseApprovalPlatformSourceManualFallbackSchema,
  storeIntegrationClaim: AppInstallPurchaseApprovalPlatformSourceStoreIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseApprovalPlatformSourcePlatformAdapterClaimSchema,
  interceptionClaim: AppInstallPurchaseApprovalPlatformSourceInterceptionClaimSchema,
  claimBoundary: AppInstallPurchaseApprovalPlatformSourceClaimBoundarySchema,
  lastCheckedAt: ParentTimestampSchema,
});

type AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate = Infer<
  typeof AppInstallPurchaseApprovalPlatformSourceMetadataRowBaseSchema
>;

export const AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema = withParser(
  AppInstallPurchaseApprovalPlatformSourceMetadataRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformSourceMetadataRowIsHonest(row) ||
        'Expected platform-source metadata rows to cite limitation proof without store integration or interception claims'
    )
  )
);

export type AppInstallPurchaseApprovalPlatformSourceMetadataRow = Infer<
  typeof AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema
>;

export function appInstallPurchaseApprovalPlatformSourceMetadataRowsAreComplete(
  rows: ReadonlyArray<AppInstallPurchaseApprovalPlatformSourceMetadataRow>
): boolean {
  const rowKeys = new Set(rows.map((row) => platformSourceKey(row)));
  return (
    rows.length === RequiredPlatformSources.length &&
    RequiredPlatformSources.every((source) => rowKeys.has(platformSourceKey(source))) &&
    rows.every((row) => platformSourceMetadataRowIsHonest(row))
  );
}

function platformSourceMetadataRowIsHonest(row: AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate): boolean {
  if (
    !platformSourceClaimsStayContractOnly(row) ||
    !rowRequestCoverageIsComplete(row) ||
    !rowMetadataFieldsAreExplicit(row) ||
    !rowStoreSourceMatchesAuthority(row)
  ) {
    return false;
  }

  if (row.metadataState === 'unavailable') {
    return row.sourceEvidenceState === 'platform-unavailable' && row.fieldsAvailableFromContract.length === 0;
  }

  return (
    row.metadataState === 'manual-required' &&
    row.sourceEvidenceState !== 'platform-unavailable' &&
    row.fieldsAvailableFromContract.length === 0
  );
}

function platformSourceClaimsStayContractOnly(
  row: AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate
): boolean {
  return (
    row.requiredArtifacts.length > 0 &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.interceptionClaim === 'not-claimed' &&
    row.parentManualFallback === 'contract-only-parent-review' &&
    row.claimBoundary.includes('no store integration') &&
    row.claimBoundary.includes('no platform adapter') &&
    row.claimBoundary.includes('no real install or purchase interception')
  );
}

function rowRequestCoverageIsComplete(row: AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate): boolean {
  return arrayContainsEvery(row.requestKindCoverage, RequiredRequestKinds) && arrayIsUnique(row.requestKindCoverage);
}

function rowMetadataFieldsAreExplicit(row: AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate): boolean {
  return (
    row.fieldsRequiringPlatformProof.length === RequiredMetadataFields.length &&
    arrayContainsEvery(row.fieldsRequiringPlatformProof, RequiredMetadataFields) &&
    arrayIsUnique(row.fieldsRequiringPlatformProof) &&
    arrayIsUnique(row.fieldsAvailableFromContract)
  );
}

function rowStoreSourceMatchesAuthority(row: AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate): boolean {
  if (row.storeSurface === 'google-play') {
    return row.sourceAuthority === 'google-play-listing';
  }
  if (row.storeSurface === 'apple-app-store') {
    return row.sourceAuthority === 'apple-app-store-listing';
  }
  if (row.storeSurface === 'mac-app-store') {
    return row.sourceAuthority === 'mac-app-store-listing';
  }
  if (row.storeSurface === 'microsoft-store') {
    return row.sourceAuthority === 'microsoft-store-listing';
  }
  return row.sourceAuthority === 'linux-package-manager-index';
}

function arrayContainsEvery<T extends string>(values: readonly T[], requiredValues: readonly T[]): boolean {
  const valueSet = new Set(values);
  return requiredValues.every((value) => valueSet.has(value));
}

function arrayIsUnique<T extends string>(values: readonly T[]): boolean {
  return new Set(values).size === values.length;
}

function platformSourceKey(input: {
  readonly platform: (typeof RequiredPlatformSources)[number]['platform'];
  readonly storeSurface: (typeof RequiredPlatformSources)[number]['storeSurface'];
}): string {
  return `${input.platform}:${input.storeSurface}`;
}
