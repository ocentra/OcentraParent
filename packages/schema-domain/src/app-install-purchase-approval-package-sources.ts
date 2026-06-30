import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppInstallPurchaseApprovalContractRuntime,
  GeneratedAppInstallPurchaseApprovalInterceptionClaims,
  GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathStates,
  GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaims,
  GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatuses,
  GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustodyStates,
  GeneratedAppInstallPurchaseApprovalPackageSourceFields,
  GeneratedAppInstallPurchaseApprovalPackageSourceKinds,
  GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims,
  GeneratedAppInstallPurchaseApprovalRequestKinds,
  GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims,
  type GeneratedAppInstallPurchaseApprovalStoreSurface,
} from './generated/app-install-purchase-approval-contracts';
import {
  appInstallPurchaseApprovalPackageSourceArtifactRowIsHonestGenerated,
  appInstallPurchaseApprovalPackageSourceArtifactRowsAreCompleteGenerated,
} from './generated/app-install-purchase-proof-helpers';

const RequiredRequestKinds = GeneratedAppInstallPurchaseApprovalRequestKinds;
const RequiredPackageSourceFields = GeneratedAppInstallPurchaseApprovalPackageSourceFields;
const AppInstallPurchaseApprovalPackageSourceSchemaVersionSchema = withParser(
  Schema.Literal(AppInstallPurchaseApprovalContractRuntime.SchemaVersion)
);
const PackageSourceStoreSurfaces = [
  'google-play',
  'apple-app-store',
  'mac-app-store',
  'microsoft-store',
  'linux-package-manager',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalStoreSurface[];
const PackageSourceKinds = GeneratedAppInstallPurchaseApprovalPackageSourceKinds;
const PackageSourceApprovalPathStates = GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathStates;
const PackageSourceArtifactStatuses = GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatuses;
const PackageSourceArtifactEvidenceClaims =
  GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaims;
const PackageSourceStoreIntegrationClaims = GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims;
const PackageSourcePlatformAdapterClaims = GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims;
const PackageSourceInterceptionClaims = GeneratedAppInstallPurchaseApprovalInterceptionClaims;
const PackageSourceChildDataCustodyStates =
  GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustodyStates;
const AppInstallPurchaseApprovalPackageSourceRequestKindSchema = withParser(
  Schema.Literal(...RequiredRequestKinds)
);
const AppInstallPurchaseApprovalPackageSourceStoreSurfaceSchema = withParser(
  Schema.Literal(...PackageSourceStoreSurfaces)
);
const AppInstallPurchaseApprovalPackageSourceKindSchema = withParser(Schema.Literal(...PackageSourceKinds));
export const AppInstallPurchaseApprovalPackageSourceArtifactStatusSchema = withParser(
  Schema.Literal(...PackageSourceArtifactStatuses)
);
const AppInstallPurchaseApprovalPackageSourceApprovalPathStateSchema = withParser(
  Schema.Literal(...PackageSourceApprovalPathStates)
);
const AppInstallPurchaseApprovalPackageSourceFieldSchema = withParser(Schema.Literal(...RequiredPackageSourceFields));
const AppInstallPurchaseApprovalPackageSourceArtifactClaimSchema = withParser(
  Schema.Literal(...PackageSourceArtifactEvidenceClaims)
);
const AppInstallPurchaseApprovalPackageSourceStoreIntegrationClaimSchema = withParser(
  Schema.Literal(...PackageSourceStoreIntegrationClaims)
);
const AppInstallPurchaseApprovalPackageSourcePlatformAdapterClaimSchema = withParser(
  Schema.Literal(...PackageSourcePlatformAdapterClaims)
);
const AppInstallPurchaseApprovalPackageSourceInterceptionClaimSchema = withParser(
  Schema.Literal(...PackageSourceInterceptionClaims)
);
const AppInstallPurchaseApprovalPackageSourceChildDataCustodySchema = withParser(
  Schema.Literal(...PackageSourceChildDataCustodyStates)
);

const AppInstallPurchaseApprovalPackageSourceArtifactRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceArtifactRowId'
);
const AppInstallPurchaseApprovalPackageSourceMetadataRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceMetadataRowId'
);
const AppInstallPurchaseApprovalPackageSourceArtifactRequirementSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceArtifactRequirement'
);
const AppInstallPurchaseApprovalPackageSourceLimitationReasonSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceLimitationReason'
);
const AppInstallPurchaseApprovalPackageSourceReportRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceReportRef'
);
const AppInstallPurchaseApprovalPackageSourceClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPackageSourceClaimBoundary'
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
        appInstallPurchaseApprovalPackageSourceArtifactRowIsHonestGenerated(row) ||
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
  return appInstallPurchaseApprovalPackageSourceArtifactRowsAreCompleteGenerated(
    rows satisfies ReadonlyArray<AppInstallPurchaseApprovalPackageSourceArtifactRowCandidate>
  );
}
