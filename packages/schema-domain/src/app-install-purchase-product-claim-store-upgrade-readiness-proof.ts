import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimGateProofReadModel } from './app-install-purchase-product-claim-gate-proof';
import { AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel } from './app-install-purchase-product-claim-portal-test-readiness-proof';
import { AppInstallPurchaseProductClaimProviderStoreProofReadModel } from './app-install-purchase-product-claim-provider-store-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-store-upgrade-readiness-proof';
const SourceProductClaimGateProofVersion = 'app-install-purchase-product-claim-gate-proof';
const SourcePortalTestReadinessProofVersion = 'app-install-purchase-product-claim-portal-test-readiness-proof';
const SourceProviderStoreProofVersion = 'app-install-purchase-product-claim-provider-store-proof';
const UpdatedAt = '2026-06-06T15:20:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const StoreUpgradeReadinessStates = [
  'product-claim-store-upgrade-blocked',
  'manual-store-upgrade-required',
  'unsupported-store-upgrade-blocked',
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
  'no-platform-adapter-implementation',
  'no-runtime-device-delivery',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-child-device-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'product claim store upgrade readiness proof only; links product-claim gate portal test readiness and provider/store proof rows so product-claim upgrades remain blocked until real portal approval report tests child-device delivery provider/store API execution and platform adapter proof exist no product claim approval no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no runtime device delivery no runtime writer delivery no runtime report delivery no child-device delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'product claim store upgrade readiness proof only',
  'product-claim gate',
  'portal test readiness',
  'provider/store proof rows',
  'product-claim upgrades remain blocked',
  'real portal approval report tests',
  'child-device delivery',
  'provider/store API execution',
  'platform adapter proof',
  'no product claim approval',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no runtime device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no child-device delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const StoreUpgradeReadinessStateSchema = withParser(Schema.Literal(...StoreUpgradeReadinessStates));
const ProductClaimGateStateSchema = withParser(Schema.Literal('product-claim-denied', 'manual-required', 'blocked'));
const PortalTestReadinessStateSchema = withParser(
  Schema.Literal('portal-test-ready', 'manual-portal-test-required', 'unsupported-portal-test-blocked')
);
const ProviderStoreProductClaimStateSchema = withParser(
  Schema.Literal(
    'provider-store-proof-required',
    'manual-provider-store-proof-required',
    'unsupported-store-proof-blocked'
  )
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimStoreUpgradeReadinessRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimStoreUpgradeReadinessBoundary');

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchemaVersionSchema,
  storeUpgradeReadinessRowId: RefSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceProductClaimGateProofVersion),
  sourceProductClaimGateRowId: RefSchema,
  sourceProductClaimGateState: ProductClaimGateStateSchema,
  sourcePortalTestReadinessProofVersion: Schema.Literal(SourcePortalTestReadinessProofVersion),
  sourcePortalTestReadinessRowId: RefSchema,
  sourcePortalTestReadinessState: PortalTestReadinessStateSchema,
  sourceProviderStoreProofVersion: Schema.Literal(SourceProviderStoreProofVersion),
  sourceProviderStoreProductClaimRowId: RefSchema,
  sourceProviderStoreProductClaimState: ProviderStoreProductClaimStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  storeUpgradeReadinessState: StoreUpgradeReadinessStateSchema,
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredProviderEvidenceRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  runtimeWriterReceiptRefs: Schema.Array(RefSchema),
  runtimeReportRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  runtimeDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type StoreUpgradeReadinessRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        storeUpgradeReadinessRowIsHonest(row) ||
        'Expected product-claim store upgrade readiness rows to keep product claims blocked until portal, child delivery, provider/store API, and platform adapter proof exists'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchemaVersionSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceProductClaimGateProofVersion),
  sourcePortalTestReadinessProofVersion: Schema.Literal(SourcePortalTestReadinessProofVersion),
  sourceProviderStoreProofVersion: Schema.Literal(SourceProviderStoreProofVersion),
  storeUpgradeReadinessRows: Schema.Array(AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimStoreUpgradeReadinessProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        storeUpgradeReadinessProofIsHonest(proof) ||
        'Expected product-claim store upgrade readiness proof to cover every store surface without approving product claims'
    )
  )
);

export const AppInstallPurchaseProductClaimStoreUpgradeReadinessKnownGaps = [
  'Store upgrade readiness rows consume gate, portal-test, and provider/store proof rows but do not approve any product claim.',
  'Windows remains product-claim-store-upgrade-blocked until real portal approval/report tests, child delivery proof, provider/store API execution proof, and platform adapter proof exist.',
  'macOS remains manual-store-upgrade-required, while Linux, Android, and iOS remain unsupported-store-upgrade-blocked before product claims.',
] as const;

export const AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel =
  AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProductClaimGateProofVersion: SourceProductClaimGateProofVersion,
    sourcePortalTestReadinessProofVersion: SourcePortalTestReadinessProofVersion,
    sourceProviderStoreProofVersion: SourceProviderStoreProofVersion,
    storeUpgradeReadinessRows:
      AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows.map(storeUpgradeReadinessRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimStoreUpgradeReadinessKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimStoreUpgradeReadinessProof(
  proof: AppInstallPurchaseProductClaimStoreUpgradeReadinessProof
) {
  return {
    storeUpgradeReadinessRows: proof.storeUpgradeReadinessRows.length,
    productClaimStoreUpgradeBlockedRows: proof.storeUpgradeReadinessRows.filter(
      (row) => row.storeUpgradeReadinessState === 'product-claim-store-upgrade-blocked'
    ).length,
    manualStoreUpgradeRequiredRows: proof.storeUpgradeReadinessRows.filter(
      (row) => row.storeUpgradeReadinessState === 'manual-store-upgrade-required'
    ).length,
    unsupportedStoreUpgradeBlockedRows: proof.storeUpgradeReadinessRows.filter(
      (row) => row.storeUpgradeReadinessState === 'unsupported-store-upgrade-blocked'
    ).length,
    providerExecutedRows: proof.storeUpgradeReadinessRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    portalUiClaimedRows: proof.storeUpgradeReadinessRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-claimed' || row.portalReportUiClaim !== 'not-claimed'
    ).length,
    productClaimApprovedRows: proof.storeUpgradeReadinessRows.filter(productClaimStoreUpgradeIsApproved).length,
  } as const;
}

function storeUpgradeReadinessRow(
  gateRow: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]
) {
  const portalRow = matchingPortalTestReadinessRow(gateRow.platform, gateRow.storeSurface);
  const providerStoreRow = matchingProviderStoreRow(gateRow.platform, gateRow.storeSurface);
  return {
    schemaVersion: ProofVersion,
    storeUpgradeReadinessRowId: `app-install-product-claim-store-upgrade-readiness-${gateRow.platform}-${gateRow.storeSurface}`,
    sourceProductClaimGateProofVersion: SourceProductClaimGateProofVersion,
    sourceProductClaimGateRowId: gateRow.productClaimGateRowId,
    sourceProductClaimGateState: gateRow.productClaimGateState,
    sourcePortalTestReadinessProofVersion: SourcePortalTestReadinessProofVersion,
    sourcePortalTestReadinessRowId: portalRow.portalTestReadinessRowId,
    sourcePortalTestReadinessState: portalRow.portalTestReadinessState,
    sourceProviderStoreProofVersion: SourceProviderStoreProofVersion,
    sourceProviderStoreProductClaimRowId: providerStoreRow.providerStoreProductClaimRowId,
    sourceProviderStoreProductClaimState: providerStoreRow.providerStoreProductClaimState,
    platform: gateRow.platform,
    storeSurface: gateRow.storeSurface,
    storeUpgradeReadinessState: storeUpgradeReadinessState(
      gateRow.productClaimGateState,
      portalRow.portalTestReadinessState,
      providerStoreRow.providerStoreProductClaimState
    ),
    requiredPortalTestRefs: uniqueRefs([portalRow.portalApprovalTestRef, portalRow.portalReportTestRef]),
    requiredProviderStoreExecutionRefs: providerStoreRow.requiredProviderStoreExecutionRefs,
    requiredProviderEvidenceRefs: providerStoreRow.requiredProviderEvidenceRefs,
    requiredChildDeliveryRefs: uniqueRefs([
      ...gateRow.requiredChildDeliveryRefs,
      ...portalRow.requiredChildDeliveryRefs,
    ]),
    requiredPlatformAdapterRefs: uniqueRefs([
      ...gateRow.requiredPlatformAdapterRefs,
      ...portalRow.requiredPlatformAdapterRefs,
      ...providerStoreRow.requiredPlatformAdapterRefs,
    ]),
    runtimeWriterReceiptRefs: providerStoreRow.runtimeWriterReceiptRefs,
    runtimeReportRefs: uniqueRefs([...portalRow.reportRuntimeRefs, ...providerStoreRow.reportRuntimeRefs]),
    auditEventRefs: uniqueRefs([
      ...gateRow.auditEventRefs,
      ...portalRow.auditEventRefs,
      ...providerStoreRow.auditEventRefs,
    ]),
    productClaimApprovalClaim: 'not-claimed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: Boundary,
    evaluatedAt: UpdatedAt,
  } as const;
}

function matchingPortalTestReadinessRow(platform: string, storeSurface: string) {
  const row = AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing portal test readiness row for ${platform}:${storeSurface}`);
  }
  return row;
}

function matchingProviderStoreRow(platform: string, storeSurface: string) {
  const row = AppInstallPurchaseProductClaimProviderStoreProofReadModel.providerStoreProductClaimRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing provider store product-claim row for ${platform}:${storeSurface}`);
  }
  return row;
}

function storeUpgradeReadinessState(
  gateState: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]['productClaimGateState'],
  portalState: (typeof AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows)[number]['portalTestReadinessState'],
  providerStoreState: (typeof AppInstallPurchaseProductClaimProviderStoreProofReadModel.providerStoreProductClaimRows)[number]['providerStoreProductClaimState']
): (typeof StoreUpgradeReadinessStates)[number] {
  if (
    gateState === 'blocked' ||
    portalState === 'unsupported-portal-test-blocked' ||
    providerStoreState === 'unsupported-store-proof-blocked'
  ) {
    return 'unsupported-store-upgrade-blocked';
  }
  if (gateState === 'manual-required' || portalState === 'manual-portal-test-required') {
    return 'manual-store-upgrade-required';
  }
  return 'product-claim-store-upgrade-blocked';
}

function storeUpgradeReadinessRowIsHonest(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.sourceProductClaimGateProofVersion === SourceProductClaimGateProofVersion &&
    row.sourcePortalTestReadinessProofVersion === SourcePortalTestReadinessProofVersion &&
    row.sourceProviderStoreProofVersion === SourceProviderStoreProofVersion &&
    storeUpgradeReadinessStateMatchesSources(row) &&
    storeUpgradeRefsAreComplete(row) &&
    !productClaimStoreUpgradeIsApproved(row) &&
    storeUpgradeClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function storeUpgradeReadinessStateMatchesSources(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    storeUpgradeReadinessState(
      row.sourceProductClaimGateState,
      row.sourcePortalTestReadinessState,
      row.sourceProviderStoreProductClaimState
    ) === row.storeUpgradeReadinessState
  );
}

function storeUpgradeRefsAreComplete(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.sourceProductClaimGateRowId.length > 0 &&
    row.sourcePortalTestReadinessRowId.length > 0 &&
    row.sourceProviderStoreProductClaimRowId.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.runtimeReportRefs.length > 0 &&
    row.auditEventRefs.length > 0
  );
}

function productClaimStoreUpgradeIsApproved(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim !== 'not-claimed' ||
    row.providerApiExecutionClaim !== 'not-executed' ||
    row.storeIntegrationClaim !== 'not-claimed' ||
    row.platformAdapterClaim !== 'not-implemented' ||
    row.childDeviceDeliveryClaim !== 'not-delivered' ||
    row.portalApprovalUiClaim !== 'not-claimed' ||
    row.portalReportUiClaim !== 'not-claimed'
  );
}

function storeUpgradeClaimsStayUnimplemented(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    storeUpgradeApprovalAndExecutionClaimsStayUnimplemented(row) &&
    storeUpgradeDeliveryClaimsStayUnimplemented(row) &&
    storeUpgradePortalAndCustodyClaimsStayUnimplemented(row)
  );
}

function storeUpgradeApprovalAndExecutionClaimsStayUnimplemented(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented'
  );
}

function storeUpgradeDeliveryClaimsStayUnimplemented(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.runtimeDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed'
  );
}

function storeUpgradePortalAndCustodyClaimsStayUnimplemented(row: StoreUpgradeReadinessRowCandidate): boolean {
  return (
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function storeUpgradeReadinessProofIsHonest(proof: AppInstallPurchaseProductClaimStoreUpgradeReadinessProof): boolean {
  const keys = new Set(proof.storeUpgradeReadinessRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.storeUpgradeReadinessRows.map((row) => row.storeUpgradeReadinessState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.storeUpgradeReadinessRows.length === StoreSurfaces.length &&
    keys.size === proof.storeUpgradeReadinessRows.length &&
    StoreUpgradeReadinessStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.storeUpgradeReadinessRows.every(storeUpgradeReadinessRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
