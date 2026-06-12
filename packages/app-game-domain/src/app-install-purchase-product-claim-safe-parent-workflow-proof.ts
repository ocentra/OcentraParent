import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimGateProofReadModel } from './app-install-purchase-product-claim-gate-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-safe-parent-workflow-proof';
const SourceGateProofVersion = 'app-install-purchase-product-claim-gate-proof';
const UpdatedAt = '2026-06-06T11:55:00.000Z';
const Text = Schema.String.pipe(Schema.minLength(1));
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const WorkflowStates = [
  'safe-parent-review-ready',
  'manual-parent-review-required',
  'unsupported-store-workflow-blocked',
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
  'safe parent workflow proof only; converts product-claim gate rows into parent review manual-required or unsupported workflow rows without portal approval UI portal report UI provider API execution store integration platform adapter implementation child-device delivery runtime writer delivery runtime report delivery app blocking child activity data or Ocentra-hosted family data custody';
const BoundaryFragments = [
  'safe parent workflow proof only',
  'product-claim gate rows',
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

export const AppInstallPurchaseProductClaimSafeParentWorkflowProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const WorkflowStateSchema = withParser(Schema.Literal(...WorkflowStates));
const ProductClaimGateStateSchema = withParser(Schema.Literal('product-claim-denied', 'manual-required', 'blocked'));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimSafeParentWorkflowRef'));
const BoundarySchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimSafeParentWorkflowBoundary'));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimSafeParentWorkflowProofSchemaVersionSchema,
  safeParentWorkflowRowId: RefSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceGateProofVersion),
  sourceProductClaimGateRowId: RefSchema,
  sourceProductClaimGateState: ProductClaimGateStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  safeParentWorkflowState: WorkflowStateSchema,
  parentWorkflowRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
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

type SafeParentWorkflowRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        safeParentWorkflowRowIsHonest(row) ||
        'Expected app install product-claim safe parent workflow rows to keep real execution, delivery, portal UI, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimSafeParentWorkflowProofSchemaVersionSchema,
  sourceProductClaimGateProofVersion: Schema.Literal(SourceGateProofVersion),
  safeParentWorkflowRows: Schema.Array(AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimSafeParentWorkflowProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        safeParentWorkflowProofIsHonest(proof) ||
        'Expected app install product-claim safe parent workflow proof to cover every store surface and keep product claims unapproved'
    )
  )
);

export const AppInstallPurchaseProductClaimSafeParentWorkflowKnownGaps = [
  'Safe parent workflow rows consume product-claim gate rows but do not approve product claims.',
  'Windows can route packet-ready evidence into parent review, but portal approval/report tests, child delivery proof, provider/store API execution proof, and platform adapter proof remain required.',
  'macOS stays manual-parent-review-required; Linux, Android, and iOS stay unsupported-store-workflow-blocked before any product claim.',
] as const;

export const AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel =
  AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProductClaimGateProofVersion: SourceGateProofVersion,
    safeParentWorkflowRows:
      AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows.map(safeParentWorkflowRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimSafeParentWorkflowKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimSafeParentWorkflowProof(
  proof: AppInstallPurchaseProductClaimSafeParentWorkflowProof
) {
  return {
    safeParentWorkflowRows: proof.safeParentWorkflowRows.length,
    safeParentReviewReadyRows: proof.safeParentWorkflowRows.filter(
      (row) => row.safeParentWorkflowState === 'safe-parent-review-ready'
    ).length,
    manualParentReviewRequiredRows: proof.safeParentWorkflowRows.filter(
      (row) => row.safeParentWorkflowState === 'manual-parent-review-required'
    ).length,
    unsupportedStoreWorkflowBlockedRows: proof.safeParentWorkflowRows.filter(
      (row) => row.safeParentWorkflowState === 'unsupported-store-workflow-blocked'
    ).length,
    providerExecutedRows: proof.safeParentWorkflowRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed')
      .length,
    portalUiClaimedRows: proof.safeParentWorkflowRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-claimed' || row.portalReportUiClaim !== 'not-claimed'
    ).length,
    productClaimApprovedRows: proof.safeParentWorkflowRows.filter((row) => productClaimWorkflowIsApproved(row)).length,
  } as const;
}

function safeParentWorkflowRow(
  gateRow: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]
) {
  return {
    schemaVersion: ProofVersion,
    safeParentWorkflowRowId: `app-install-product-claim-safe-parent-workflow-${gateRow.platform}-${gateRow.storeSurface}`,
    sourceProductClaimGateProofVersion: SourceGateProofVersion,
    sourceProductClaimGateRowId: gateRow.productClaimGateRowId,
    sourceProductClaimGateState: gateRow.productClaimGateState,
    platform: gateRow.platform,
    storeSurface: gateRow.storeSurface,
    safeParentWorkflowState: safeParentWorkflowState(gateRow.productClaimGateState),
    parentWorkflowRefs: parentWorkflowRefs(gateRow.platform, gateRow.storeSurface),
    requiredPortalTestRefs: gateRow.requiredPortalTestRefs,
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

function safeParentWorkflowState(
  gateState: (typeof AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows)[number]['productClaimGateState']
): (typeof WorkflowStates)[number] {
  if (gateState === 'product-claim-denied') {
    return 'safe-parent-review-ready';
  }
  if (gateState === 'manual-required') {
    return 'manual-parent-review-required';
  }
  return 'unsupported-store-workflow-blocked';
}

function parentWorkflowRefs(platform: string, storeSurface: string) {
  return [
    `parent-review-manual-evidence-${platform}-${storeSurface}`,
    `parent-report-product-claim-not-approved-${platform}-${storeSurface}`,
  ] as const;
}

function safeParentWorkflowRowIsHonest(row: SafeParentWorkflowRowCandidate): boolean {
  return (
    row.sourceProductClaimGateProofVersion === SourceGateProofVersion &&
    row.sourceProductClaimGateRowId.length > 0 &&
    row.parentWorkflowRefs.length > 0 &&
    requiredGateRefsStayAttached(row) &&
    safeParentWorkflowState(row.sourceProductClaimGateState) === row.safeParentWorkflowState &&
    !productClaimWorkflowIsApproved(row) &&
    safeParentWorkflowClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function requiredGateRefsStayAttached(row: SafeParentWorkflowRowCandidate): boolean {
  return (
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.limitationRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function productClaimWorkflowIsApproved(row: SafeParentWorkflowRowCandidate): boolean {
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

function safeParentWorkflowClaimsStayUnimplemented(row: SafeParentWorkflowRowCandidate): boolean {
  return !productClaimWorkflowIsApproved(row);
}

function safeParentWorkflowProofIsHonest(proof: AppInstallPurchaseProductClaimSafeParentWorkflowProof): boolean {
  const keys = new Set(proof.safeParentWorkflowRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.safeParentWorkflowRows.map((row) => row.safeParentWorkflowState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProductClaimGateProofVersion === SourceGateProofVersion &&
    proof.safeParentWorkflowRows.length === StoreSurfaces.length &&
    keys.size === proof.safeParentWorkflowRows.length &&
    WorkflowStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.safeParentWorkflowRows.every(safeParentWorkflowRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
