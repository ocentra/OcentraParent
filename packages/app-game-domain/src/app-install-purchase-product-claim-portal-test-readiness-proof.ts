import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimGateProofReadModel } from './app-install-purchase-product-claim-gate-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-portal-test-readiness-proof';
const SourceGateProofVersion = 'app-install-purchase-product-claim-gate-proof';
const UpdatedAt = '2026-06-06T12:35:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const ReadinessStates = [
  'portal-test-ready',
  'manual-portal-test-required',
  'unsupported-portal-test-blocked',
] as const;
const NonClaims = [
  'no-portal-approval-ui',
  'no-portal-report-ui',
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
  'portal test readiness proof only; names portal approval and report test refs required before product claims without portal approval UI portal report UI provider API execution store integration platform adapter implementation child-device delivery runtime writer delivery runtime report delivery app blocking child activity data or Ocentra-hosted family data custody';
const BoundaryFragments = [
  'portal test readiness proof only',
  'portal approval and report test refs',
  'before product claims',
  'without portal approval UI',
  'portal report UI',
  'provider API execution',
  'store integration',
  'platform adapter implementation',
  'child-device delivery',
  'runtime writer delivery',
  'runtime report delivery',
  'app blocking',
  'child activity data',
  'Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProductClaimPortalTestReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const ReadinessStateSchema = withParser(Schema.Literal(...ReadinessStates));
const GateStateSchema = withParser(Schema.Literal('product-claim-denied', 'manual-required', 'blocked'));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPortalTestReadinessRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPortalTestReadinessBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPortalTestReadinessProofSchemaVersionSchema,
  portalTestReadinessRowId: RefSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceGateProofVersion),
  sourceProductClaimGateRowId: RefSchema,
  sourceProductClaimGateState: GateStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  portalTestReadinessState: ReadinessStateSchema,
  portalApprovalTestRef: RefSchema,
  portalReportTestRef: RefSchema,
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  limitationRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
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

type PortalTestReadinessRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimPortalTestReadinessRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        portalTestReadinessRowIsHonest(row) ||
        'Expected app install product-claim portal test readiness rows to keep UI, execution, delivery, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPortalTestReadinessProofSchemaVersionSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceGateProofVersion),
  portalTestReadinessRows: Schema.Array(AppInstallPurchaseProductClaimPortalTestReadinessRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimPortalTestReadinessProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimPortalTestReadinessProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        portalTestReadinessProofIsHonest(proof) ||
        'Expected app install product-claim portal test readiness proof to cover every store surface without approving product claims'
    )
  )
);

export const AppInstallPurchaseProductClaimPortalTestReadinessKnownGaps = [
  'Portal test readiness rows name approval and report test refs required before product claims; no portal UI is implemented.',
  'Windows can reach portal-test-ready from denied product-claim gate rows but still requires child delivery, provider/store API execution, and platform adapter proof.',
  'macOS remains manual-portal-test-required, while Linux, Android, and iOS remain unsupported-portal-test-blocked before product claims.',
] as const;

export const AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel =
  AppInstallPurchaseProductClaimPortalTestReadinessProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProductClaimGateProofVersion: SourceGateProofVersion,
    portalTestReadinessRows:
      AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows.map(portalTestReadinessRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimPortalTestReadinessKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimPortalTestReadinessProof(
  proof: AppInstallPurchaseProductClaimPortalTestReadinessProof
) {
  return {
    portalTestReadinessRows: proof.portalTestReadinessRows.length,
    portalTestReadyRows: proof.portalTestReadinessRows.filter(
      (row) => row.portalTestReadinessState === 'portal-test-ready'
    ).length,
    manualPortalTestRequiredRows: proof.portalTestReadinessRows.filter(
      (row) => row.portalTestReadinessState === 'manual-portal-test-required'
    ).length,
    unsupportedPortalTestBlockedRows: proof.portalTestReadinessRows.filter(
      (row) => row.portalTestReadinessState === 'unsupported-portal-test-blocked'
    ).length,
    portalUiClaimedRows: proof.portalTestReadinessRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-claimed' || row.portalReportUiClaim !== 'not-claimed'
    ).length,
    productClaimApprovedRows: proof.portalTestReadinessRows.filter(productClaimPortalRowIsApproved).length,
  } as const;
}

function portalTestReadinessRow(
  gateRow: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]
) {
  return {
    schemaVersion: ProofVersion,
    portalTestReadinessRowId: `app-install-product-claim-portal-test-readiness-${gateRow.platform}-${gateRow.storeSurface}`,
    sourceProductClaimGateProofVersion: SourceGateProofVersion,
    sourceProductClaimGateRowId: gateRow.productClaimGateRowId,
    sourceProductClaimGateState: gateRow.productClaimGateState,
    platform: gateRow.platform,
    storeSurface: gateRow.storeSurface,
    portalTestReadinessState: portalTestReadinessState(gateRow.productClaimGateState),
    portalApprovalTestRef: gateRow.requiredPortalTestRefs[0],
    portalReportTestRef: `portal-report-test-${gateRow.platform}-${gateRow.storeSurface}`,
    requiredChildDeliveryRefs: gateRow.requiredChildDeliveryRefs,
    requiredProviderStoreExecutionRefs: gateRow.requiredProviderStoreExecutionRefs,
    requiredPlatformAdapterRefs: gateRow.requiredPlatformAdapterRefs,
    limitationRefs: gateRow.limitationRefs,
    auditEventRefs: gateRow.auditEventRefs,
    reportRuntimeRefs: gateRow.reportRuntimeRefs,
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
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

function portalTestReadinessState(
  gateState: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]['productClaimGateState']
): (typeof ReadinessStates)[number] {
  if (gateState === 'product-claim-denied') {
    return 'portal-test-ready';
  }
  if (gateState === 'manual-required') {
    return 'manual-portal-test-required';
  }
  return 'unsupported-portal-test-blocked';
}

function portalTestReadinessRowIsHonest(row: PortalTestReadinessRowCandidate): boolean {
  return (
    row.sourceProductClaimGateProofVersion === SourceGateProofVersion &&
    row.sourceProductClaimGateRowId.length > 0 &&
    row.portalApprovalTestRef.length > 0 &&
    row.portalReportTestRef.length > 0 &&
    gateRefsStayAttached(row) &&
    portalTestReadinessState(row.sourceProductClaimGateState) === row.portalTestReadinessState &&
    !productClaimPortalRowIsApproved(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function gateRefsStayAttached(row: PortalTestReadinessRowCandidate): boolean {
  return (
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.limitationRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function productClaimPortalRowIsApproved(row: PortalTestReadinessRowCandidate): boolean {
  return (
    row.portalApprovalUiClaim !== 'not-claimed' ||
    row.portalReportUiClaim !== 'not-claimed' ||
    row.providerApiExecutionClaim !== 'not-executed' ||
    row.storeIntegrationClaim !== 'not-claimed' ||
    row.platformAdapterClaim !== 'not-implemented' ||
    row.childDeviceDeliveryClaim !== 'not-delivered' ||
    row.runtimeWriterDeliveryClaim !== 'not-delivered' ||
    row.runtimeReportDeliveryClaim !== 'not-delivered' ||
    row.appBlockingClaim !== 'not-claimed' ||
    row.childDataCustody !== 'no-child-activity-data' ||
    row.ocentraHostedFamilyDataCustodyClaim !== 'not-claimed'
  );
}

function portalTestReadinessProofIsHonest(proof: AppInstallPurchaseProductClaimPortalTestReadinessProof): boolean {
  const keys = new Set(proof.portalTestReadinessRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.portalTestReadinessRows.map((row) => row.portalTestReadinessState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProductClaimGateProofVersion === SourceGateProofVersion &&
    proof.portalTestReadinessRows.length === StoreSurfaces.length &&
    keys.size === proof.portalTestReadinessRows.length &&
    ReadinessStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.portalTestReadinessRows.every(portalTestReadinessRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

