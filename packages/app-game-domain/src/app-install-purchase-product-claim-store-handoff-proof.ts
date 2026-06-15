import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel } from './app-install-purchase-product-claim-safe-parent-workflow-proof';
import { AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel } from './app-install-purchase-provider-store-manual-evidence-packet-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-store-handoff-proof';
const SourceSafeWorkflowVersion = 'app-install-purchase-product-claim-safe-parent-workflow-proof';
const SourceManualEvidencePacketVersion = 'app-install-purchase-provider-store-manual-evidence-packet-proof';
const UpdatedAt = '2026-06-06T13:58:00.000Z';
const Text = Schema.String.pipe(Schema.minLength(1));
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const StoreHandoffStates = [
  'store-handoff-review-ready',
  'store-handoff-manual-required',
  'store-handoff-unavailable',
] as const;
const NonClaims = [
  'no-product-claim-approved',
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
  'product claim store handoff proof only; links safe parent workflow rows and provider store manual evidence packet rows into parent-visible store handoff rows without approving product claims no portal approval UI no portal report UI no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'product claim store handoff proof only',
  'safe parent workflow rows',
  'provider store manual evidence packet rows',
  'without approving product claims',
  'no portal approval UI',
  'no portal report UI',
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

export const AppInstallPurchaseProductClaimStoreHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const StoreHandoffStateSchema = withParser(Schema.Literal(...StoreHandoffStates));
const SafeWorkflowStateSchema = withParser(
  Schema.Literal('safe-parent-review-ready', 'manual-parent-review-required', 'unsupported-store-workflow-blocked')
);
const PacketStateSchema = withParser(
  Schema.Literal('manual-evidence-packet-ready', 'manual-review-required', 'provider-unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimStoreHandoffRef'));
const BoundarySchema = Text.pipe(Schema.brand('AppInstallPurchaseProductClaimStoreHandoffBoundary'));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimStoreHandoffProofSchemaVersionSchema,
  productClaimStoreHandoffRowId: RefSchema,
  sourceSafeParentWorkflowProofVersion: Schema.Literal(SourceSafeWorkflowVersion),
  sourceSafeParentWorkflowRowId: RefSchema,
  sourceSafeParentWorkflowState: SafeWorkflowStateSchema,
  sourceManualEvidencePacketProofVersion: Schema.Literal(SourceManualEvidencePacketVersion),
  sourceManualEvidencePacketRowId: RefSchema,
  sourceManualEvidencePacketState: PacketStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  storeHandoffState: StoreHandoffStateSchema,
  parentWorkflowRefs: Schema.Array(RefSchema),
  requiredManualEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderEvidenceRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  runtimeWriterReceiptRefs: Schema.Array(RefSchema),
  limitationRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovedClaim: NotClaimedSchema,
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

type StoreHandoffRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimStoreHandoffRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        storeHandoffRowIsHonest(row) ||
        'Expected app install product-claim store handoff rows to preserve safe workflow and manual evidence packet refs without approving product claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimStoreHandoffProofSchemaVersionSchema,
  sourceSafeParentWorkflowProofVersion: Schema.Literal(SourceSafeWorkflowVersion),
  sourceManualEvidencePacketProofVersion: Schema.Literal(SourceManualEvidencePacketVersion),
  productClaimStoreHandoffRows: Schema.Array(AppInstallPurchaseProductClaimStoreHandoffRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimStoreHandoffProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimStoreHandoffProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        storeHandoffProofIsHonest(proof) ||
        'Expected app install product-claim store handoff proof to cover every store surface and keep product claims unapproved'
    )
  )
);

export const AppInstallPurchaseProductClaimStoreHandoffKnownGaps = [
  'Store handoff rows consume safe parent workflow and manual evidence packet refs but do not approve app-install product claims.',
  'Windows can be store-handoff-review-ready only because the parent-owned packet is ready; portal tests provider/store API execution platform adapter proof and child delivery proof remain required.',
  'macOS stays store-handoff-manual-required; Linux Android and iOS stay store-handoff-unavailable before product claims.',
  'Public package export is validated for the store handoff proof boundary.',
] as const;

export const AppInstallPurchaseProductClaimStoreHandoffProofReadModel =
  AppInstallPurchaseProductClaimStoreHandoffProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceSafeParentWorkflowProofVersion: SourceSafeWorkflowVersion,
    sourceManualEvidencePacketProofVersion: SourceManualEvidencePacketVersion,
    productClaimStoreHandoffRows:
      AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows.map(storeHandoffRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimStoreHandoffKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimStoreHandoffProof(
  proof: AppInstallPurchaseProductClaimStoreHandoffProof
) {
  return {
    productClaimStoreHandoffRows: proof.productClaimStoreHandoffRows.length,
    reviewReadyRows: proof.productClaimStoreHandoffRows.filter(
      (row) => row.storeHandoffState === 'store-handoff-review-ready'
    ).length,
    manualRequiredRows: proof.productClaimStoreHandoffRows.filter(
      (row) => row.storeHandoffState === 'store-handoff-manual-required'
    ).length,
    unavailableRows: proof.productClaimStoreHandoffRows.filter(
      (row) => row.storeHandoffState === 'store-handoff-unavailable'
    ).length,
    productClaimApprovedRows: proof.productClaimStoreHandoffRows.filter(
      (row) => row.productClaimApprovedClaim !== 'not-claimed'
    ).length,
    providerExecutedRows: proof.productClaimStoreHandoffRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
  } as const;
}

function storeHandoffRow(
  workflowRow: (typeof AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows)[number]
) {
  const packetRow = AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows.find(
    (row) => row.platform === workflowRow.platform && row.storeSurface === workflowRow.storeSurface
  );
  if (!packetRow) {
    throw new Error(`Missing manual evidence packet row for ${workflowRow.platform}:${workflowRow.storeSurface}`);
  }
  return {
    schemaVersion: ProofVersion,
    productClaimStoreHandoffRowId: `app-install-product-claim-store-handoff-${workflowRow.platform}-${workflowRow.storeSurface}`,
    sourceSafeParentWorkflowProofVersion: SourceSafeWorkflowVersion,
    sourceSafeParentWorkflowRowId: workflowRow.safeParentWorkflowRowId,
    sourceSafeParentWorkflowState: workflowRow.safeParentWorkflowState,
    sourceManualEvidencePacketProofVersion: SourceManualEvidencePacketVersion,
    sourceManualEvidencePacketRowId: packetRow.manualEvidencePacketRowId,
    sourceManualEvidencePacketState: packetRow.manualEvidencePacketState,
    platform: workflowRow.platform,
    storeSurface: workflowRow.storeSurface,
    storeHandoffState: storeHandoffState(workflowRow.safeParentWorkflowState, packetRow.manualEvidencePacketState),
    parentWorkflowRefs: workflowRow.parentWorkflowRefs,
    requiredManualEvidenceRefs: packetRow.requiredManualEvidenceRefs,
    requiredProviderEvidenceRefs: packetRow.requiredProviderEvidenceRefs,
    requiredPortalTestRefs: workflowRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: workflowRow.requiredChildDeliveryRefs,
    requiredProviderStoreExecutionRefs: workflowRow.requiredProviderStoreExecutionRefs,
    requiredPlatformAdapterRefs: workflowRow.requiredPlatformAdapterRefs,
    runtimeWriterReceiptRefs: packetRow.runtimeWriterReceiptRefs,
    limitationRefs: workflowRow.limitationRefs,
    auditEventRefs: workflowRow.auditEventRefs,
    reportRuntimeRefs: workflowRow.reportRuntimeRefs,
    productClaimApprovedClaim: 'not-claimed',
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

function storeHandoffState(
  workflowState: (typeof AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows)[number]['safeParentWorkflowState'],
  packetState: (typeof AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows)[number]['manualEvidencePacketState']
): (typeof StoreHandoffStates)[number] {
  if (workflowState === 'unsupported-store-workflow-blocked' || packetState === 'provider-unavailable') {
    return 'store-handoff-unavailable';
  }
  if (workflowState === 'safe-parent-review-ready' && packetState === 'manual-evidence-packet-ready') {
    return 'store-handoff-review-ready';
  }
  return 'store-handoff-manual-required';
}

function storeHandoffRowIsHonest(row: StoreHandoffRowCandidate): boolean {
  return (
    row.sourceSafeParentWorkflowProofVersion === SourceSafeWorkflowVersion &&
    row.sourceManualEvidencePacketProofVersion === SourceManualEvidencePacketVersion &&
    row.storeHandoffState ===
      storeHandoffState(row.sourceSafeParentWorkflowState, row.sourceManualEvidencePacketState) &&
    storeHandoffRefsStayAttached(row) &&
    storeHandoffClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function storeHandoffRefsStayAttached(row: StoreHandoffRowCandidate): boolean {
  return (
    row.sourceSafeParentWorkflowRowId.length > 0 &&
    row.sourceManualEvidencePacketRowId.length > 0 &&
    row.parentWorkflowRefs.length > 0 &&
    storeHandoffManualRefsStayAttached(row) &&
    storeHandoffRuntimeRefsStayAttached(row)
  );
}

function storeHandoffManualRefsStayAttached(row: StoreHandoffRowCandidate): boolean {
  return (
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0
  );
}

function storeHandoffRuntimeRefsStayAttached(row: StoreHandoffRowCandidate): boolean {
  return (
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.limitationRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function storeHandoffClaimsStayUnimplemented(row: StoreHandoffRowCandidate): boolean {
  return (
    row.productClaimApprovedClaim === 'not-claimed' &&
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function storeHandoffProofIsHonest(proof: AppInstallPurchaseProductClaimStoreHandoffProof): boolean {
  const keys = new Set(proof.productClaimStoreHandoffRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.productClaimStoreHandoffRows.map((row) => row.storeHandoffState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceSafeParentWorkflowProofVersion === SourceSafeWorkflowVersion &&
    proof.sourceManualEvidencePacketProofVersion === SourceManualEvidencePacketVersion &&
    proof.productClaimStoreHandoffRows.length === StoreSurfaces.length &&
    keys.size === proof.productClaimStoreHandoffRows.length &&
    StoreHandoffStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.productClaimStoreHandoffRows.every(storeHandoffRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
