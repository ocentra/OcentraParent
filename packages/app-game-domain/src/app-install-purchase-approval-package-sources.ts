import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyPackageSourceText = Schema.String.pipe(Schema.minLength(1));
const RequiredRequestKinds = ['install', 'purchase', 'subscription'] as const;
const RequiredPackageSourceFields = [
  'package-identifier',
  'installer-source',
  'publisher-or-developer',
  'version-or-build',
  'signature-or-receipt',
  'source-captured-at',
] as const;
const RequiredPackageSourceRows = [
  {
    platform: 'windows',
    storeSurface: 'microsoft-store',
    packageSourceKind: 'windows-store-package-identity',
    artifactStatus: 'manual-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'macos',
    storeSurface: 'mac-app-store',
    packageSourceKind: 'macos-bundle-receipt',
    artifactStatus: 'manual-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'linux',
    storeSurface: 'linux-package-manager',
    packageSourceKind: 'linux-package-manager-record',
    artifactStatus: 'unavailable',
    approvalPathState: 'unavailable',
  },
  {
    platform: 'android',
    storeSurface: 'google-play',
    packageSourceKind: 'android-package-source-record',
    artifactStatus: 'device-proof-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'ios',
    storeSurface: 'apple-app-store',
    packageSourceKind: 'ios-app-source-record',
    artifactStatus: 'device-proof-required',
    approvalPathState: 'manual-required',
  },
] as const;

const AppInstallPurchaseApprovalPackageSourceSchemaVersionSchema = withParser(
  Schema.Literal('app-install-purchase-approval-contract-proof')
);
const AppInstallPurchaseApprovalPackageSourceRequestKindSchema = withParser(
  Schema.Literal('install', 'purchase', 'subscription')
);
const AppInstallPurchaseApprovalPackageSourceStoreSurfaceSchema = withParser(
  Schema.Literal('google-play', 'apple-app-store', 'mac-app-store', 'microsoft-store', 'linux-package-manager')
);
const AppInstallPurchaseApprovalPackageSourceKindSchema = withParser(
  Schema.Literal(
    'windows-store-package-identity',
    'macos-bundle-receipt',
    'linux-package-manager-record',
    'android-package-source-record',
    'ios-app-source-record'
  )
);
export const AppInstallPurchaseApprovalPackageSourceArtifactStatusSchema = withParser(
  Schema.Literal('manual-required', 'device-proof-required', 'unavailable')
);
const AppInstallPurchaseApprovalPackageSourceApprovalPathStateSchema = withParser(
  Schema.Literal('manual-required', 'unavailable')
);
const AppInstallPurchaseApprovalPackageSourceFieldSchema = withParser(
  Schema.Literal(
    'package-identifier',
    'installer-source',
    'publisher-or-developer',
    'version-or-build',
    'signature-or-receipt',
    'source-captured-at'
  )
);
const AppInstallPurchaseApprovalPackageSourceArtifactClaimSchema = withParser(Schema.Literal('not-attached'));
const AppInstallPurchaseApprovalPackageSourceStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovalPackageSourcePlatformAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseApprovalPackageSourceInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovalPackageSourceChildDataCustodySchema = withParser(
  Schema.Literal('no-child-activity-data')
);

const AppInstallPurchaseApprovalPackageSourceArtifactRowIdSchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceArtifactRowId')
);
const AppInstallPurchaseApprovalPackageSourceMetadataRowIdSchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceMetadataRowId')
);
const AppInstallPurchaseApprovalPackageSourceArtifactRequirementSchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceArtifactRequirement')
);
const AppInstallPurchaseApprovalPackageSourceLimitationReasonSchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceLimitationReason')
);
const AppInstallPurchaseApprovalPackageSourceReportRefSchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceReportRef')
);
const AppInstallPurchaseApprovalPackageSourceClaimBoundarySchema = NonEmptyPackageSourceText.pipe(
  Schema.brand('AppInstallPurchaseApprovalPackageSourceClaimBoundary')
);

const AppInstallPurchaseApprovalPackageSourceArtifactRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalPackageSourceSchemaVersionSchema,
  artifactRowId: AppInstallPurchaseApprovalPackageSourceArtifactRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseApprovalPackageSourceStoreSurfaceSchema,
  platformSourceRowId: AppInstallPurchaseApprovalPackageSourceMetadataRowIdSchema,
  packageSourceKind: AppInstallPurchaseApprovalPackageSourceKindSchema,
  artifactStatus: AppInstallPurchaseApprovalPackageSourceArtifactStatusSchema,
  approvalPathState: AppInstallPurchaseApprovalPackageSourceApprovalPathStateSchema,
  packageSourceFieldsRequired: Schema.Array(AppInstallPurchaseApprovalPackageSourceFieldSchema),
  packageSourceFieldsAttached: Schema.Array(AppInstallPurchaseApprovalPackageSourceFieldSchema),
  requestKindCoverage: Schema.Array(AppInstallPurchaseApprovalPackageSourceRequestKindSchema),
  requiredArtifacts: Schema.Array(AppInstallPurchaseApprovalPackageSourceArtifactRequirementSchema),
  artifactEvidenceClaim: AppInstallPurchaseApprovalPackageSourceArtifactClaimSchema,
  artifactEvidencePath: Schema.Null,
  artifactCapturedAt: Schema.Null,
  limitationReason: AppInstallPurchaseApprovalPackageSourceLimitationReasonSchema,
  limitationReportRef: AppInstallPurchaseApprovalPackageSourceReportRefSchema,
  storeIntegrationClaim: AppInstallPurchaseApprovalPackageSourceStoreIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseApprovalPackageSourcePlatformAdapterClaimSchema,
  interceptionClaim: AppInstallPurchaseApprovalPackageSourceInterceptionClaimSchema,
  childDataCustody: AppInstallPurchaseApprovalPackageSourceChildDataCustodySchema,
  claimBoundary: AppInstallPurchaseApprovalPackageSourceClaimBoundarySchema,
  lastCheckedAt: ParentTimestampSchema,
});

type AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate = Infer<
  typeof AppInstallPurchaseApprovalPackageSourceArtifactRowBaseSchema
>;

export const AppInstallPurchaseApprovalPackageSourceArtifactRowSchema = withParser(
  AppInstallPurchaseApprovalPackageSourceArtifactRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        packageSourceArtifactRowIsHonest(row) ||
        'Expected package-source artifact rows to require platform proof without attaching real child-device artifacts or claiming store integration'
    )
  )
);

export type AppInstallPurchaseApprovalPackageSourceArtifactRow = Infer<
  typeof AppInstallPurchaseApprovalPackageSourceArtifactRowSchema
>;

export function appInstallPurchaseApprovalPackageSourceArtifactRowsAreComplete(
  rows: ReadonlyArray<AppInstallPurchaseApprovalPackageSourceArtifactRow>
): boolean {
  const rowKeys = new Set(rows.map((row) => packageSourceKey(row)));
  return (
    rows.length === RequiredPackageSourceRows.length &&
    RequiredPackageSourceRows.every((source) => {
      const row = rows.find((entry) => packageSourceKey(entry) === packageSourceKey(source));
      return row !== undefined && packageSourceRowMatchesExpectedState(row, source);
    }) &&
    rowKeys.size === rows.length &&
    rows.every((row) => packageSourceArtifactRowIsHonest(row))
  );
}

function packageSourceArtifactRowIsHonest(row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate): boolean {
  return (
    packageSourceClaimsStayContractOnly(row) &&
    rowRequestCoverageIsComplete(row) &&
    rowPackageSourceFieldsAreExplicit(row) &&
    rowStoreSurfaceMatchesPackageSource(row) &&
    rowArtifactStatusMatchesApprovalPath(row)
  );
}

function packageSourceClaimsStayContractOnly(
  row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate
): boolean {
  return (
    row.requiredArtifacts.length > 0 &&
    packageSourceEvidenceIsNotAttached(row) &&
    packageSourceNonClaimsAreExplicit(row) &&
    packageSourceClaimBoundaryIsExplicit(row)
  );
}

function packageSourceEvidenceIsNotAttached(row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate): boolean {
  return (
    row.artifactEvidenceClaim === 'not-attached' && row.artifactEvidencePath === null && row.artifactCapturedAt === null
  );
}

function packageSourceNonClaimsAreExplicit(row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate): boolean {
  return (
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.interceptionClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data'
  );
}

function packageSourceClaimBoundaryIsExplicit(
  row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate
): boolean {
  return (
    row.claimBoundary.includes('contract proof') &&
    row.claimBoundary.includes('no store integration') &&
    row.claimBoundary.includes('no platform adapter') &&
    row.claimBoundary.includes('no real install or purchase interception') &&
    row.claimBoundary.includes('no child activity data')
  );
}

function rowRequestCoverageIsComplete(row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate): boolean {
  return arrayContainsEvery(row.requestKindCoverage, RequiredRequestKinds) && arrayIsUnique(row.requestKindCoverage);
}

function rowPackageSourceFieldsAreExplicit(row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate): boolean {
  return (
    row.packageSourceFieldsRequired.length === RequiredPackageSourceFields.length &&
    row.packageSourceFieldsAttached.length === 0 &&
    arrayContainsEvery(row.packageSourceFieldsRequired, RequiredPackageSourceFields) &&
    arrayIsUnique(row.packageSourceFieldsRequired)
  );
}

function rowStoreSurfaceMatchesPackageSource(
  row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate
): boolean {
  if (row.storeSurface === 'microsoft-store') {
    return row.packageSourceKind === 'windows-store-package-identity';
  }
  if (row.storeSurface === 'mac-app-store') {
    return row.packageSourceKind === 'macos-bundle-receipt';
  }
  if (row.storeSurface === 'linux-package-manager') {
    return row.packageSourceKind === 'linux-package-manager-record';
  }
  if (row.storeSurface === 'google-play') {
    return row.packageSourceKind === 'android-package-source-record';
  }
  return row.packageSourceKind === 'ios-app-source-record';
}

function rowArtifactStatusMatchesApprovalPath(
  row: AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate
): boolean {
  if (row.approvalPathState === 'unavailable') {
    return row.artifactStatus === 'unavailable';
  }
  return row.artifactStatus === 'manual-required' || row.artifactStatus === 'device-proof-required';
}

function packageSourceRowMatchesExpectedState(
  row: AppInstallPurchaseApprovalPackageSourceArtifactRow,
  expected: (typeof RequiredPackageSourceRows)[number]
): boolean {
  return (
    row.packageSourceKind === expected.packageSourceKind &&
    row.artifactStatus === expected.artifactStatus &&
    row.approvalPathState === expected.approvalPathState
  );
}

function arrayContainsEvery<T extends string>(values: readonly T[], requiredValues: readonly T[]): boolean {
  const valueSet = new Set(values);
  return requiredValues.every((value) => valueSet.has(value));
}

function arrayIsUnique<T extends string>(values: readonly T[]): boolean {
  return new Set(values).size === values.length;
}

function packageSourceKey(input: {
  readonly platform: (typeof RequiredPackageSourceRows)[number]['platform'];
  readonly storeSurface: (typeof RequiredPackageSourceRows)[number]['storeSurface'];
}): string {
  return `${input.platform}:${input.storeSurface}`;
}
