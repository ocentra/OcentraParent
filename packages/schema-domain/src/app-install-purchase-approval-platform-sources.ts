import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppInstallPurchaseApprovalContractRuntime,
  GeneratedAppInstallPurchaseApprovalInterceptionClaims,
  GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims,
  GeneratedAppInstallPurchaseApprovalPlatformSourceAuthorities,
  GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceStates,
  GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallbacks,
  GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields,
  GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataStates,
  GeneratedAppInstallPurchaseApprovalRequestKinds,
  GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims,
  type GeneratedAppInstallPurchaseApprovalStoreSurface,
} from './generated/app-install-purchase-approval-contracts';
import {
  appInstallPurchaseApprovalPlatformSourceMetadataRowIsHonestGenerated,
  appInstallPurchaseApprovalPlatformSourceMetadataRowsAreCompleteGenerated,
} from './generated/app-install-purchase-proof-helpers';

const RequiredRequestKinds = GeneratedAppInstallPurchaseApprovalRequestKinds;
const RequiredMetadataFields = GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields;
const AppInstallPurchaseApprovalPlatformSourceSchemaVersionSchema = withParser(
  Schema.Literal(AppInstallPurchaseApprovalContractRuntime.SchemaVersion)
);
const PlatformSourceStoreSurfaces = [
  'google-play',
  'apple-app-store',
  'mac-app-store',
  'microsoft-store',
  'linux-package-manager',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalStoreSurface[];
const PlatformSourceAuthorities = GeneratedAppInstallPurchaseApprovalPlatformSourceAuthorities;
const PlatformSourceMetadataStates = GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataStates;
const PlatformSourceEvidenceStates = GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceStates;
const PlatformSourceManualFallbacks = GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallbacks;
const PlatformSourceStoreIntegrationClaims = GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims;
const PlatformSourcePlatformAdapterClaims = GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims;
const PlatformSourceInterceptionClaims = GeneratedAppInstallPurchaseApprovalInterceptionClaims;
const AppInstallPurchaseApprovalPlatformSourceRequestKindSchema = withParser(
  Schema.Literal(...RequiredRequestKinds)
);
const AppInstallPurchaseApprovalPlatformSourceStoreSurfaceSchema = withParser(
  Schema.Literal(...PlatformSourceStoreSurfaces)
);
const AppInstallPurchaseApprovalPlatformSourceAuthoritySchema = withParser(
  Schema.Literal(...PlatformSourceAuthorities)
);
export const AppInstallPurchaseApprovalPlatformSourceMetadataStateSchema = withParser(
  Schema.Literal(...PlatformSourceMetadataStates)
);
const AppInstallPurchaseApprovalPlatformSourceEvidenceStateSchema = withParser(
  Schema.Literal(...PlatformSourceEvidenceStates)
);
const AppInstallPurchaseApprovalPlatformSourceMetadataFieldSchema = withParser(
  Schema.Literal(...RequiredMetadataFields)
);
const AppInstallPurchaseApprovalPlatformSourceManualFallbackSchema = withParser(
  Schema.Literal(...PlatformSourceManualFallbacks)
);
const AppInstallPurchaseApprovalPlatformSourceStoreIntegrationClaimSchema = withParser(
  Schema.Literal(...PlatformSourceStoreIntegrationClaims)
);
const AppInstallPurchaseApprovalPlatformSourcePlatformAdapterClaimSchema = withParser(
  Schema.Literal(...PlatformSourcePlatformAdapterClaims)
);
const AppInstallPurchaseApprovalPlatformSourceInterceptionClaimSchema = withParser(
  Schema.Literal(...PlatformSourceInterceptionClaims)
);

const AppInstallPurchaseApprovalPlatformSourceRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPlatformSourceRowId'
);
const AppInstallPurchaseApprovalPlatformSourceArtifactRequirementSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPlatformSourceArtifactRequirement'
);
const AppInstallPurchaseApprovalPlatformSourceLimitationReasonSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPlatformSourceLimitationReason'
);
const AppInstallPurchaseApprovalPlatformSourceReportRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPlatformSourceReportRef'
);
const AppInstallPurchaseApprovalPlatformSourceClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPlatformSourceClaimBoundary'
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
        appInstallPurchaseApprovalPlatformSourceMetadataRowIsHonestGenerated(row) ||
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
  return appInstallPurchaseApprovalPlatformSourceMetadataRowsAreCompleteGenerated(
    rows satisfies ReadonlyArray<AppInstallPurchaseApprovalPlatformSourceMetadataRowCandidate>
  );
}
