import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel } from './app-install-purchase-product-claim-portal-test-readiness-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-platform-preclaim-proof';
const SourcePortalTestReadinessProofVersion = 'app-install-purchase-product-claim-portal-test-readiness-proof';
const SourcePlatformProofReadinessProofVersion = 'app-install-purchase-platform-proof-readiness';
const UpdatedAt = '2026-06-06T21:20:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const PreclaimStates = ['manual-platform-preclaim-required', 'unsupported-platform-preclaim-blocked'] as const;
const NonClaims = [
  'no-product-claim-approval',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'product claim platform preclaim proof only; links portal approval report test readiness with platform proof readiness before product claims remain manual or blocked no product claim approval no portal approval UI no portal report UI no Google Play execution no Apple App Store execution no Microsoft Store execution no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'product claim platform preclaim proof only',
  'portal approval report test readiness',
  'platform proof readiness before product claims',
  'manual or blocked',
  'no product claim approval',
  'no portal approval UI',
  'no portal report UI',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProductClaimPlatformPreclaimProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const PreclaimStateSchema = withParser(Schema.Literal(...PreclaimStates));
const PortalTestReadinessStateSchema = withParser(
  Schema.Literal('portal-test-ready', 'manual-portal-test-required', 'unsupported-portal-test-blocked')
);
const PlatformProofReadinessStateSchema = withParser(
  Schema.Literal('manual-proof-required', 'policy-blocked', 'unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPlatformPreclaimRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPlatformPreclaimBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPlatformPreclaimProofSchemaVersionSchema,
  platformPreclaimRowId: RefSchema,
  sourcePortalTestReadinessProofVersion: Schema.Literal(SourcePortalTestReadinessProofVersion),
  sourcePortalTestReadinessRowId: RefSchema,
  sourcePortalTestReadinessState: PortalTestReadinessStateSchema,
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  sourcePlatformProofReadinessRowId: RefSchema,
  sourcePlatformProofReadinessState: PlatformProofReadinessStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  platformPreclaimState: PreclaimStateSchema,
  portalApprovalTestRef: RefSchema,
  portalReportTestRef: RefSchema,
  requiredManualPlatformEvidenceRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  limitationRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type PlatformPreclaimRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimPlatformPreclaimRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformPreclaimRowIsHonest(row) ||
        'Expected product-claim platform preclaim rows to keep portal UI, platform adapters, provider execution, delivery, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPlatformPreclaimProofSchemaVersionSchema,
  sourcePortalTestReadinessProofVersion: Schema.Literal(SourcePortalTestReadinessProofVersion),
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  platformPreclaimRows: Schema.Array(AppInstallPurchaseProductClaimPlatformPreclaimRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimPlatformPreclaimProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimPlatformPreclaimProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformPreclaimProofIsHonest(proof) ||
        'Expected product-claim platform preclaim proof to cover every store surface without approving product claims'
    )
  )
);

export const AppInstallPurchaseProductClaimPlatformPreclaimKnownGaps = [
  'Platform preclaim rows link portal approval/report test refs with platform proof readiness rows before any product claim can be upgraded.',
  'Windows and macOS remain manual-platform-preclaim-required until real portal approval/report tests and platform adapter evidence are attached.',
  'Linux, Android, and iOS remain unsupported-platform-preclaim-blocked until package-manager source, managed-profile, entitlement, or review proof exists.',
] as const;

export const AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel =
  AppInstallPurchaseProductClaimPlatformPreclaimProofSchema.parse({
    schemaVersion: ProofVersion,
    sourcePortalTestReadinessProofVersion: SourcePortalTestReadinessProofVersion,
    sourcePlatformProofReadinessProofVersion: SourcePlatformProofReadinessProofVersion,
    platformPreclaimRows:
      AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows.map(platformPreclaimRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimPlatformPreclaimKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimPlatformPreclaimProof(
  proof: AppInstallPurchaseProductClaimPlatformPreclaimProof
) {
  return {
    platformPreclaimRows: proof.platformPreclaimRows.length,
    manualPlatformPreclaimRequiredRows: proof.platformPreclaimRows.filter(
      (row) => row.platformPreclaimState === 'manual-platform-preclaim-required'
    ).length,
    unsupportedPlatformPreclaimBlockedRows: proof.platformPreclaimRows.filter(
      (row) => row.platformPreclaimState === 'unsupported-platform-preclaim-blocked'
    ).length,
    portalUiClaimedRows: proof.platformPreclaimRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-claimed' || row.portalReportUiClaim !== 'not-claimed'
    ).length,
    platformAdapterImplementedRows: proof.platformPreclaimRows.filter(
      (row) => row.platformAdapterClaim !== 'not-implemented'
    ).length,
    productClaimApprovedRows: proof.platformPreclaimRows.filter(platformPreclaimRowApprovesProductClaim).length,
  } as const;
}

function platformPreclaimRow(
  portalRow: (typeof AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows)[number]
) {
  const platformRow = matchingPlatformProofReadinessRow(portalRow.platform);
  return {
    schemaVersion: ProofVersion,
    platformPreclaimRowId: `app-install-product-claim-platform-preclaim-${portalRow.platform}-${portalRow.storeSurface}`,
    sourcePortalTestReadinessProofVersion: SourcePortalTestReadinessProofVersion,
    sourcePortalTestReadinessRowId: portalRow.portalTestReadinessRowId,
    sourcePortalTestReadinessState: portalRow.portalTestReadinessState,
    sourcePlatformProofReadinessProofVersion: SourcePlatformProofReadinessProofVersion,
    sourcePlatformProofReadinessRowId: `app-install-platform-proof-readiness-${platformRow.platform}`,
    sourcePlatformProofReadinessState: platformRow.platformProofReadinessState,
    platform: portalRow.platform,
    storeSurface: portalRow.storeSurface,
    platformPreclaimState: platformPreclaimState(
      portalRow.portalTestReadinessState,
      platformRow.platformProofReadinessState
    ),
    portalApprovalTestRef: portalRow.portalApprovalTestRef,
    portalReportTestRef: portalRow.portalReportTestRef,
    requiredManualPlatformEvidenceRefs: platformRow.requiredManualEvidenceRefs,
    requiredChildDeliveryRefs: portalRow.requiredChildDeliveryRefs,
    requiredProviderStoreExecutionRefs: portalRow.requiredProviderStoreExecutionRefs,
    requiredPlatformAdapterRefs: portalRow.requiredPlatformAdapterRefs,
    limitationRefs: uniqueRefs([...portalRow.limitationRefs, ...platformRow.sourceLimitationSummaryRowIds]),
    auditEventRefs: portalRow.auditEventRefs,
    reportRuntimeRefs: portalRow.reportRuntimeRefs,
    productClaimApprovalClaim: 'not-claimed',
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: Boundary,
    evaluatedAt: UpdatedAt,
  } as const;
}

function matchingPlatformProofReadinessRow(platform: string) {
  const row = AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows.find(
    (candidate) => candidate.platform === platform
  );
  if (!row) {
    throw new Error(`missing platform proof readiness row for ${platform}`);
  }
  return row;
}

function platformPreclaimState(
  portalTestReadinessState: (typeof AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows)[number]['portalTestReadinessState'],
  platformProofReadinessState: (typeof AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows)[number]['platformProofReadinessState']
): (typeof PreclaimStates)[number] {
  if (
    portalTestReadinessState === 'unsupported-portal-test-blocked' ||
    platformProofReadinessState !== 'manual-proof-required'
  ) {
    return 'unsupported-platform-preclaim-blocked';
  }
  return 'manual-platform-preclaim-required';
}

function platformPreclaimRowIsHonest(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.sourcePortalTestReadinessProofVersion === SourcePortalTestReadinessProofVersion &&
    row.sourcePlatformProofReadinessProofVersion === SourcePlatformProofReadinessProofVersion &&
    platformPreclaimSourceRefsStayAttached(row) &&
    platformPreclaimState(row.sourcePortalTestReadinessState, row.sourcePlatformProofReadinessState) ===
      row.platformPreclaimState &&
    !platformPreclaimRowApprovesProductClaim(row) &&
    platformPreclaimClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function platformPreclaimSourceRefsStayAttached(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.sourcePortalTestReadinessRowId.length > 0 &&
    row.sourcePlatformProofReadinessRowId.length > 0 &&
    row.portalApprovalTestRef.length > 0 &&
    row.portalReportTestRef.length > 0 &&
    row.requiredManualPlatformEvidenceRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.limitationRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function platformPreclaimRowApprovesProductClaim(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim !== 'not-claimed' ||
    row.portalApprovalUiClaim !== 'not-claimed' ||
    row.portalReportUiClaim !== 'not-claimed' ||
    row.providerApiExecutionClaim !== 'not-executed' ||
    row.storeIntegrationClaim !== 'not-claimed' ||
    row.platformAdapterClaim !== 'not-implemented' ||
    row.childDeviceDeliveryClaim !== 'not-delivered'
  );
}

function platformPreclaimClaimsStayUnimplemented(row: PlatformPreclaimRowCandidate): boolean {
  return (
    platformPreclaimProductAndPortalClaimsStayUnimplemented(row) &&
    platformPreclaimExecutionClaimsStayUnimplemented(row) &&
    platformPreclaimDeliveryAndCustodyClaimsStayUnimplemented(row)
  );
}

function platformPreclaimProductAndPortalClaimsStayUnimplemented(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed'
  );
}

function platformPreclaimExecutionClaimsStayUnimplemented(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented'
  );
}

function platformPreclaimDeliveryAndCustodyClaimsStayUnimplemented(row: PlatformPreclaimRowCandidate): boolean {
  return (
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function platformPreclaimProofIsHonest(proof: AppInstallPurchaseProductClaimPlatformPreclaimProof): boolean {
  const keys = new Set(proof.platformPreclaimRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.platformPreclaimRows.map((row) => row.platformPreclaimState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.platformPreclaimRows.length === StoreSurfaces.length &&
    keys.size === proof.platformPreclaimRows.length &&
    PreclaimStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.platformPreclaimRows.every(platformPreclaimRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

