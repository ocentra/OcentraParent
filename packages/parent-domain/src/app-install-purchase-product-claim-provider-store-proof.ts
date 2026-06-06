import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimGateProofReadModel } from './app-install-purchase-product-claim-gate-proof';
import { AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel } from './app-install-purchase-provider-store-execution-preflight-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-provider-store-proof';
const SourceProductClaimGateProofVersion = 'app-install-purchase-product-claim-gate-proof';
const SourceProviderStorePreflightProofVersion = 'app-install-purchase-provider-store-execution-preflight-proof';
const UpdatedAt = '2026-06-06T13:30:00.000Z';
const Text = Schema.String.pipe(Schema.minLength(1));
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const ProviderStoreProductClaimStates = [
  'provider-store-proof-required',
  'manual-provider-store-proof-required',
  'unsupported-store-proof-blocked',
] as const;
const NonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-runtime-device-delivery',
  'no-child-device-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'product claim provider store proof only; links product-claim gate rows to provider/store execution preflight rows and keeps product claims blocked until provider/store API execution proof exists no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no runtime device delivery no child-device delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'product-claim gate rows',
  'provider/store execution preflight rows',
  'product claims blocked',
  'provider/store API execution proof',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no runtime device delivery',
  'no child-device delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProductClaimProviderStoreProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const ProviderStoreProductClaimStateSchema = withParser(Schema.Literal(...ProviderStoreProductClaimStates));
const ProductClaimGateStateSchema = withParser(Schema.Literal('product-claim-denied', 'manual-required', 'blocked'));
const ProviderStorePreflightStateSchema = withParser(
  Schema.Literal('preflight-ready', 'manual-provider-proof-required', 'provider-unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const RefSchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimProviderStoreRef'));
const BoundarySchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimProviderStoreBoundary'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimProviderStoreProofSchemaVersionSchema,
  providerStoreProductClaimRowId: RefSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceProductClaimGateProofVersion),
  sourceProductClaimGateRowId: RefSchema,
  sourceProductClaimGateState: ProductClaimGateStateSchema,
  sourceProviderStorePreflightProofVersion: Schema.Literal(SourceProviderStorePreflightProofVersion),
  sourceProviderStorePreflightRowId: RefSchema,
  sourceProviderStorePreflightState: ProviderStorePreflightStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  providerStoreProductClaimState: ProviderStoreProductClaimStateSchema,
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredProviderEvidenceRefs: Schema.Array(RefSchema),
  runtimeWriterReceiptRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  runtimeDeviceDeliveryClaim: NotDeliveredSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreProductClaimRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimProviderStoreRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreProductClaimRowIsHonest(row) ||
        'Expected product-claim provider/store rows to keep product claims blocked until provider/store execution proof exists without provider/store execution claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimProviderStoreProofSchemaVersionSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceProductClaimGateProofVersion),
  sourceProviderStorePreflightProofVersion: Schema.Literal(SourceProviderStorePreflightProofVersion),
  providerStoreProductClaimRows: Schema.Array(AppInstallPurchaseProductClaimProviderStoreRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimProviderStoreProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimProviderStoreProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreProductClaimProofIsHonest(proof) ||
        'Expected product-claim provider/store proof to cover every store surface and preserve provider/store non-claims'
    )
  )
);

export const AppInstallPurchaseProductClaimProviderStoreKnownGaps = [
  'Provider/store product-claim rows attach preflight evidence but do not approve product claims or execute provider/store APIs.',
  'Windows can identify provider/store proof requirements from preflight-ready evidence; macOS, Android, and iOS still require manual provider proof, and Linux remains unsupported before product claims.',
  'Portal tests, child delivery proof, provider/store API execution proof, and platform adapter proof remain required before any product claim can be allowed.',
] as const;

export const AppInstallPurchaseProductClaimProviderStoreProofReadModel =
  AppInstallPurchaseProductClaimProviderStoreProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProductClaimGateProofVersion: SourceProductClaimGateProofVersion,
    sourceProviderStorePreflightProofVersion: SourceProviderStorePreflightProofVersion,
    providerStoreProductClaimRows:
      AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows.map(providerStoreProductClaimRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimProviderStoreKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimProviderStoreProof(
  proof: AppInstallPurchaseProductClaimProviderStoreProof
) {
  return {
    providerStoreProductClaimRows: proof.providerStoreProductClaimRows.length,
    providerStoreProofRequiredRows: proof.providerStoreProductClaimRows.filter(
      (row) => row.providerStoreProductClaimState === 'provider-store-proof-required'
    ).length,
    manualProviderStoreProofRequiredRows: proof.providerStoreProductClaimRows.filter(
      (row) => row.providerStoreProductClaimState === 'manual-provider-store-proof-required'
    ).length,
    unsupportedStoreProofBlockedRows: proof.providerStoreProductClaimRows.filter(
      (row) => row.providerStoreProductClaimState === 'unsupported-store-proof-blocked'
    ).length,
    providerExecutedRows: proof.providerStoreProductClaimRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    productClaimAllowedRows: proof.providerStoreProductClaimRows.filter(productClaimProviderStoreIsAllowed).length,
  } as const;
}

function providerStoreProductClaimRow(
  gateRow: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]
) {
  const preflightRow = matchingPreflightRow(gateRow.platform, gateRow.storeSurface);
  return {
    schemaVersion: ProofVersion,
    providerStoreProductClaimRowId: `app-install-product-claim-provider-store-${gateRow.platform}-${gateRow.storeSurface}`,
    sourceProductClaimGateProofVersion: SourceProductClaimGateProofVersion,
    sourceProductClaimGateRowId: gateRow.productClaimGateRowId,
    sourceProductClaimGateState: gateRow.productClaimGateState,
    sourceProviderStorePreflightProofVersion: SourceProviderStorePreflightProofVersion,
    sourceProviderStorePreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
    sourceProviderStorePreflightState: preflightRow.providerStoreExecutionPreflightState,
    platform: gateRow.platform,
    storeSurface: gateRow.storeSurface,
    providerStoreProductClaimState: providerStoreProductClaimState(gateRow, preflightRow),
    requiredProviderStoreExecutionRefs: gateRow.requiredProviderStoreExecutionRefs,
    requiredProviderEvidenceRefs: preflightRow.requiredProviderEvidenceRefs,
    runtimeWriterReceiptRefs: preflightRow.runtimeWriterReceiptRefs,
    requiredPortalTestRefs: gateRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: gateRow.requiredChildDeliveryRefs,
    requiredPlatformAdapterRefs: gateRow.requiredPlatformAdapterRefs,
    auditEventRefs: uniqueRefs([...gateRow.auditEventRefs, ...preflightRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefs([...gateRow.reportRuntimeRefs, ...preflightRow.reportRuntimeRefs]),
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeDeviceDeliveryClaim: 'not-delivered',
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

function matchingPreflightRow(platform: string, storeSurface: string) {
  const row = AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing provider store preflight row for ${platform}:${storeSurface}`);
  }
  return row;
}

function providerStoreProductClaimState(
  gateRow: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number],
  preflightRow: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number]
): (typeof ProviderStoreProductClaimStates)[number] {
  if (
    gateRow.productClaimGateState === 'blocked' ||
    preflightRow.providerStoreExecutionPreflightState === 'provider-unavailable'
  ) {
    return 'unsupported-store-proof-blocked';
  }
  if (preflightRow.providerStoreExecutionPreflightState === 'preflight-ready') {
    return 'provider-store-proof-required';
  }
  return 'manual-provider-store-proof-required';
}

function providerStoreProductClaimRowIsHonest(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
    row.sourceProductClaimGateProofVersion === SourceProductClaimGateProofVersion &&
    row.sourceProviderStorePreflightProofVersion === SourceProviderStorePreflightProofVersion &&
    providerStoreProductClaimStateMatchesSources(row) &&
    providerStoreProductClaimRefsAreComplete(row) &&
    !productClaimProviderStoreIsAllowed(row) &&
    providerStoreProductClaimClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function providerStoreProductClaimStateMatchesSources(row: ProviderStoreProductClaimRowCandidate): boolean {
  if (
    row.sourceProductClaimGateState === 'blocked' ||
    row.sourceProviderStorePreflightState === 'provider-unavailable'
  ) {
    return row.providerStoreProductClaimState === 'unsupported-store-proof-blocked';
  }
  if (row.sourceProviderStorePreflightState === 'preflight-ready') {
    return row.providerStoreProductClaimState === 'provider-store-proof-required';
  }
  return row.providerStoreProductClaimState === 'manual-provider-store-proof-required';
}

function providerStoreProductClaimRefsAreComplete(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
    row.sourceProductClaimGateRowId.length > 0 &&
    row.sourceProviderStorePreflightRowId.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function productClaimProviderStoreIsAllowed(row: ProviderStoreProductClaimRowCandidate): boolean {
  return row.providerApiExecutionClaim !== 'not-executed' || row.storeIntegrationClaim !== 'not-claimed';
}

function providerStoreProductClaimClaimsStayUnimplemented(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
    providerExecutionClaimsStayUnimplemented(row) &&
    providerDeliveryClaimsStayUnimplemented(row) &&
    providerPortalAndCustodyClaimsStayUnimplemented(row)
  );
}

function providerExecutionClaimsStayUnimplemented(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
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

function providerDeliveryClaimsStayUnimplemented(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
    row.runtimeDeviceDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed'
  );
}

function providerPortalAndCustodyClaimsStayUnimplemented(row: ProviderStoreProductClaimRowCandidate): boolean {
  return (
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function providerStoreProductClaimProofIsHonest(proof: AppInstallPurchaseProductClaimProviderStoreProof): boolean {
  const keys = new Set(proof.providerStoreProductClaimRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.providerStoreProductClaimRows.map((row) => row.providerStoreProductClaimState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStoreProductClaimRows.length === StoreSurfaces.length &&
    keys.size === proof.providerStoreProductClaimRows.length &&
    ProviderStoreProductClaimStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.providerStoreProductClaimRows.every(providerStoreProductClaimRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
