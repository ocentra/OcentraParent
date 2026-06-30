import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel } from './app-install-purchase-provider-store-execution-preflight-proof';
import {
  AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel,
  AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema,
  type AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof,
} from './app-install-purchase-windows-package-source-adapter-evidence';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStorePlatformEvidenceRowGenerated,
  providerStorePlatformEvidenceProofIsHonestGenerated,
  providerStorePlatformEvidenceRowIsHonestGenerated,
  summarizeAppInstallPurchaseProviderStorePlatformEvidenceProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const ProofVersion = 'app-install-purchase-provider-store-platform-evidence-proof';
const SourceProviderStoreExecutionPreflightProofVersion =
  'app-install-purchase-provider-store-execution-preflight-proof';
const SourceWindowsPackageSourceRuntimeHandoffProofVersion =
  'app-install-purchase-windows-package-source-adapter-evidence';
const UpdatedAt = '2026-06-07T13:45:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const ProviderStorePlatformEvidenceStates = [
  'manual-provider-store-platform-evidence-required',
  'platform-unavailable',
  'blocked-before-claim',
] as const;
const RuntimeHandoffStates = [
  'windows-runtime-handoff-ready',
  'windows-runtime-handoff-manual-required',
  'manual-runtime-handoff-required',
  'platform-unavailable',
  'blocked-before-claim',
] as const;
const ProviderStorePreflightStates = [
  'preflight-ready',
  'manual-provider-proof-required',
  'provider-unavailable',
] as const;
const NonClaims = [
  'no-product-claim-approval',
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-production-platform-adapter',
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'provider store platform evidence proof boundary only; links provider store execution preflight rows with Windows package-source runtime handoff rows and records exact missing provider store platform and child-device artifacts before any product claim no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no production platform adapter no runtime writer execution no runtime writer delivery no child-device delivery no runtime report delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'provider store platform evidence proof boundary only',
  'provider store execution preflight rows',
  'Windows package-source runtime handoff rows',
  'exact missing provider store platform and child-device artifacts',
  'before any product claim',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no production platform adapter',
  'no runtime writer execution',
  'no runtime writer delivery',
  'no child-device delivery',
  'no runtime report delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const ProviderStorePlatformEvidenceStateSchema = withParser(Schema.Literal(...ProviderStorePlatformEvidenceStates));
const RuntimeHandoffStateSchema = withParser(Schema.Literal(...RuntimeHandoffStates));
const ProviderStorePreflightStateSchema = withParser(Schema.Literal(...ProviderStorePreflightStates));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStorePlatformEvidenceRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStorePlatformEvidenceBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema,
  providerStorePlatformEvidenceRowId: RefSchema,
  sourceProviderStoreExecutionPreflightProofVersion: Schema.Literal(SourceProviderStoreExecutionPreflightProofVersion),
  sourceProviderStoreExecutionPreflightRowId: RefSchema,
  sourceProviderStoreExecutionPreflightState: ProviderStorePreflightStateSchema,
  sourceWindowsPackageSourceRuntimeHandoffProofVersion: Schema.Literal(
    SourceWindowsPackageSourceRuntimeHandoffProofVersion
  ),
  sourceWindowsPackageSourceRuntimeHandoffRowId: RefSchema,
  sourceRuntimeHandoffState: RuntimeHandoffStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  providerStorePlatformEvidenceState: ProviderStorePlatformEvidenceStateSchema,
  packageSourceEvidenceRefs: Schema.Array(RefSchema),
  providerStorePreflightRefs: Schema.Array(RefSchema),
  missingProviderStoreArtifactRefs: Schema.Array(RefSchema),
  missingPlatformArtifactRefs: Schema.Array(RefSchema),
  missingChildDeviceArtifactRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  productionPlatformAdapterClaim: NotImplementedSchema,
  runtimeWriterExecutionClaim: NotExecutedSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStorePlatformEvidenceRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProviderStorePlatformEvidenceRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStorePlatformEvidenceRowIsHonest(row) ||
        'Expected provider/store platform evidence rows to attach preflight and runtime-handoff refs, record exact missing artifacts, and preserve provider/store/platform/delivery/portal/blocking/custody non-claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema,
  sourceProviderStoreExecutionPreflightProofVersion: Schema.Literal(SourceProviderStoreExecutionPreflightProofVersion),
  sourceWindowsPackageSourceRuntimeHandoffProofVersion: Schema.Literal(
    SourceWindowsPackageSourceRuntimeHandoffProofVersion
  ),
  providerStorePlatformEvidenceRows: Schema.Array(AppInstallPurchaseProviderStorePlatformEvidenceRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStorePlatformEvidenceProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProviderStorePlatformEvidenceProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStorePlatformEvidenceProofIsHonest(proof) ||
        'Expected provider/store platform evidence proof to cover every platform/store row with manual, unavailable, and blocked states plus all non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStorePlatformEvidenceKnownGaps = [
  'Provider/store platform evidence rows are manual or blocked proof rows only; no Google Play Apple App Store Microsoft Store provider API execution or billing provider contact is implemented.',
  'Windows package-source runtime handoff can attach sanitized local package-source evidence, but Microsoft Store credential provider response platform adapter and child-device delivery artifacts remain missing.',
  'macOS remains manual-required, Linux is unavailable, and Android/iOS stay blocked-before-claim until store/provider credentials platform adapter evidence and child-device delivery proof exist.',
] as const;

export function buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
  runtimeHandoffProof: unknown
): AppInstallPurchaseProviderStorePlatformEvidenceProof {
  const parsedRuntimeHandoffProof =
    AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema.parse(runtimeHandoffProof);
  return AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProviderStoreExecutionPreflightProofVersion: SourceProviderStoreExecutionPreflightProofVersion,
    sourceWindowsPackageSourceRuntimeHandoffProofVersion: SourceWindowsPackageSourceRuntimeHandoffProofVersion,
    providerStorePlatformEvidenceRows:
      AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows.map((row) =>
        providerStorePlatformEvidenceRow(row, parsedRuntimeHandoffProof)
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProviderStorePlatformEvidenceKnownGaps,
    updatedAt: UpdatedAt,
  });
}

export const AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel =
  buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
    AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel
  );

export function summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof(
  proof: AppInstallPurchaseProviderStorePlatformEvidenceProof
) {
  return summarizeAppInstallPurchaseProviderStorePlatformEvidenceProofGenerated(proof);
}

function providerStorePlatformEvidenceRow(
  preflightRow: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number],
  runtimeHandoffProof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
) {
  const runtimeHandoffRow = matchingRuntimeHandoffRow(
    preflightRow.platform,
    preflightRow.storeSurface,
    runtimeHandoffProof
  );
  return buildAppInstallPurchaseProviderStorePlatformEvidenceRowGenerated(
    preflightRow,
    runtimeHandoffRow,
    SourceProviderStoreExecutionPreflightProofVersion,
    SourceWindowsPackageSourceRuntimeHandoffProofVersion,
    Boundary,
    UpdatedAt
  );
}

function matchingRuntimeHandoffRow(
  platform: string,
  storeSurface: string,
  runtimeHandoffProof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
) {
  const row = runtimeHandoffProof.runtimeHandoffRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing runtime handoff row for ${platform}:${storeSurface}`);
  }
  return row;
}

function providerStorePlatformEvidenceRowIsHonest(row: ProviderStorePlatformEvidenceRowCandidate): boolean {
  return (
    row.sourceProviderStoreExecutionPreflightProofVersion === SourceProviderStoreExecutionPreflightProofVersion &&
    row.sourceWindowsPackageSourceRuntimeHandoffProofVersion ===
      SourceWindowsPackageSourceRuntimeHandoffProofVersion &&
    providerStorePlatformEvidenceRowIsHonestGenerated(row, BoundaryFragments)
  );
}

function providerStorePlatformEvidenceProofIsHonest(
  proof: AppInstallPurchaseProviderStorePlatformEvidenceProof
): boolean {
  return (
    proof.sourceProviderStoreExecutionPreflightProofVersion === SourceProviderStoreExecutionPreflightProofVersion &&
    proof.sourceWindowsPackageSourceRuntimeHandoffProofVersion ===
      SourceWindowsPackageSourceRuntimeHandoffProofVersion &&
    providerStorePlatformEvidenceProofIsHonestGenerated(
      proof,
      StoreSurfaces,
      ProviderStorePlatformEvidenceStates,
      NonClaims
    ) &&
    proof.providerStorePlatformEvidenceRows.every(providerStorePlatformEvidenceRowIsHonest)
  );
}
