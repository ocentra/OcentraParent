import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel } from './app-install-purchase-provider-store-manual-evidence-packet-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-gate-proof';
const SourcePacketVersion = 'app-install-purchase-provider-store-manual-evidence-packet-proof';
const UpdatedAt = '2026-06-06T10:45:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const GateStates = ['product-claim-denied', 'manual-required', 'blocked'] as const;
const MissingStates = ['missing', 'present'] as const;
const LimitationStates = ['not-limited', 'unsupported-os-store-limitation'] as const;
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
  'product claim gate proof only; denies app install product claim until portal approval report tests child delivery proof provider store API execution proof and platform adapter proof are present no portal approval UI no portal report UI no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'denies app install product claim',
  'portal approval report tests',
  'child delivery proof',
  'provider store API execution proof',
  'platform adapter proof',
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

export const AppInstallPurchaseProductClaimGateProofSchemaVersionSchema = withParser(Schema.Literal(ProofVersion));
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const GateStateSchema = withParser(Schema.Literal(...GateStates));
const MissingStateSchema = withParser(Schema.Literal(...MissingStates));
const LimitationStateSchema = withParser(Schema.Literal(...LimitationStates));
const PacketStateSchema = withParser(
  Schema.Literal('manual-evidence-packet-ready', 'manual-review-required', 'provider-unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimGateRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimGateBoundary');

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimGateProofSchemaVersionSchema,
  productClaimGateRowId: RefSchema,
  sourceManualEvidencePacketProofVersion: Schema.Literal(SourcePacketVersion),
  sourceManualEvidencePacketRowId: RefSchema,
  sourceManualEvidencePacketState: PacketStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  productClaimGateState: GateStateSchema,
  portalApprovalReportTestState: MissingStateSchema,
  childDeviceDeliveryProofState: MissingStateSchema,
  providerStoreApiExecutionProofState: MissingStateSchema,
  platformAdapterProofState: MissingStateSchema,
  unsupportedOsStoreLimitationState: LimitationStateSchema,
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

type ProductClaimGateRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimGateRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        productClaimGateRowIsHonest(row) ||
        'Expected app install product-claim gate rows to deny claims until portal, child delivery, provider/store API, and platform adapter proof is present'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimGateProofSchemaVersionSchema,
  sourceManualEvidencePacketProofVersion: Schema.Literal(SourcePacketVersion),
  productClaimGateRows: Schema.Array(AppInstallPurchaseProductClaimGateRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimGateProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimGateProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        productClaimGateProofIsHonest(proof) ||
        'Expected app install product-claim gate proof to cover all store surfaces and keep product claims denied'
    )
  )
);

export const AppInstallPurchaseProductClaimGateKnownGaps = [
  'Product-claim gate rows deny product claims until portal approval/report tests, child delivery proof, provider/store API execution proof, and platform adapter proof are present.',
  'Windows can reach packet-ready evidence but still has product-claim-denied status until the missing proof refs are attached.',
  'macOS remains manual-required; Linux, Android, and iOS remain blocked by unsupported OS/store limitation rows before product claims.',
] as const;

export const AppInstallPurchaseProductClaimGateProofReadModel = AppInstallPurchaseProductClaimGateProofSchema.parse({
  schemaVersion: ProofVersion,
  sourceManualEvidencePacketProofVersion: SourcePacketVersion,
  productClaimGateRows:
    AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows.map(productClaimGateRow),
  nonClaims: NonClaims,
  knownGaps: AppInstallPurchaseProductClaimGateKnownGaps,
  updatedAt: UpdatedAt,
});

export function summarizeAppInstallPurchaseProductClaimGateProof(proof: AppInstallPurchaseProductClaimGateProof) {
  return {
    productClaimGateRows: proof.productClaimGateRows.length,
    productClaimDeniedRows: proof.productClaimGateRows.filter(
      (row) => row.productClaimGateState === 'product-claim-denied'
    ).length,
    manualRequiredRows: proof.productClaimGateRows.filter((row) => row.productClaimGateState === 'manual-required')
      .length,
    blockedRows: proof.productClaimGateRows.filter((row) => row.productClaimGateState === 'blocked').length,
    missingPortalTestRows: proof.productClaimGateRows.filter((row) => row.portalApprovalReportTestState === 'missing')
      .length,
    missingChildDeliveryRows: proof.productClaimGateRows.filter(
      (row) => row.childDeviceDeliveryProofState === 'missing'
    ).length,
    missingProviderStoreApiRows: proof.productClaimGateRows.filter(
      (row) => row.providerStoreApiExecutionProofState === 'missing'
    ).length,
    missingPlatformAdapterRows: proof.productClaimGateRows.filter((row) => row.platformAdapterProofState === 'missing')
      .length,
    unsupportedLimitationRows: proof.productClaimGateRows.filter(
      (row) => row.unsupportedOsStoreLimitationState === 'unsupported-os-store-limitation'
    ).length,
    productClaimAllowedRows: proof.productClaimGateRows.filter((row) => productClaimProofsArePresent(row)).length,
  } as const;
}

function productClaimGateRow(
  packetRow: (typeof AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows)[number]
) {
  return {
    schemaVersion: ProofVersion,
    productClaimGateRowId: `app-install-product-claim-gate-${packetRow.platform}-${packetRow.storeSurface}`,
    sourceManualEvidencePacketProofVersion: SourcePacketVersion,
    sourceManualEvidencePacketRowId: packetRow.manualEvidencePacketRowId,
    sourceManualEvidencePacketState: packetRow.manualEvidencePacketState,
    platform: packetRow.platform,
    storeSurface: packetRow.storeSurface,
    productClaimGateState: gateState(packetRow),
    portalApprovalReportTestState: 'missing',
    childDeviceDeliveryProofState: 'missing',
    providerStoreApiExecutionProofState: 'missing',
    platformAdapterProofState: 'missing',
    unsupportedOsStoreLimitationState: unsupportedLimitationState(packetRow.platform),
    requiredPortalTestRefs: [`portal-approval-report-test-${packetRow.platform}-${packetRow.storeSurface}`],
    requiredChildDeliveryRefs: [`child-device-delivery-proof-${packetRow.platform}-${packetRow.storeSurface}`],
    requiredProviderStoreExecutionRefs: [
      `provider-store-api-execution-proof-${packetRow.platform}-${packetRow.storeSurface}`,
    ],
    requiredPlatformAdapterRefs: [`platform-adapter-proof-${packetRow.platform}-${packetRow.storeSurface}`],
    limitationRefs: packetRow.requiredManualEvidenceRefs,
    auditEventRefs: packetRow.auditEventRefs,
    reportRuntimeRefs: packetRow.reportRuntimeRefs,
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

function gateState(
  row: (typeof AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows)[number]
): (typeof GateStates)[number] {
  if (row.platform === 'linux' || row.platform === 'android' || row.platform === 'ios') {
    return 'blocked';
  }
  if (row.manualEvidencePacketState === 'manual-evidence-packet-ready') {
    return 'product-claim-denied';
  }
  return 'manual-required';
}

function unsupportedLimitationState(platform: string): (typeof LimitationStates)[number] {
  return platform === 'linux' || platform === 'android' || platform === 'ios'
    ? 'unsupported-os-store-limitation'
    : 'not-limited';
}

function productClaimGateRowIsHonest(row: ProductClaimGateRowCandidate): boolean {
  return (
    row.sourceManualEvidencePacketProofVersion === SourcePacketVersion &&
    row.sourceManualEvidencePacketRowId.length > 0 &&
    gateStateMatchesEvidence(row) &&
    productClaimGateProofsStayMissing(row) &&
    productClaimGateRefsAreComplete(row) &&
    !productClaimProofsArePresent(row) &&
    productClaimGateClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function gateStateMatchesEvidence(row: ProductClaimGateRowCandidate): boolean {
  if (row.unsupportedOsStoreLimitationState === 'unsupported-os-store-limitation') {
    return row.productClaimGateState === 'blocked';
  }
  if (row.sourceManualEvidencePacketState === 'manual-evidence-packet-ready') {
    return row.productClaimGateState === 'product-claim-denied';
  }
  return row.productClaimGateState === 'manual-required';
}

function productClaimGateRefsAreComplete(row: ProductClaimGateRowCandidate): boolean {
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

function productClaimGateProofsStayMissing(row: ProductClaimGateRowCandidate): boolean {
  return (
    row.portalApprovalReportTestState === 'missing' &&
    row.childDeviceDeliveryProofState === 'missing' &&
    row.providerStoreApiExecutionProofState === 'missing' &&
    row.platformAdapterProofState === 'missing'
  );
}

function productClaimProofsArePresent(row: ProductClaimGateRowCandidate): boolean {
  return (
    row.portalApprovalReportTestState === 'present' &&
    row.childDeviceDeliveryProofState === 'present' &&
    row.providerStoreApiExecutionProofState === 'present' &&
    row.platformAdapterProofState === 'present' &&
    row.unsupportedOsStoreLimitationState === 'not-limited'
  );
}

function productClaimGateClaimsStayUnimplemented(row: ProductClaimGateRowCandidate): boolean {
  return (
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

function productClaimGateProofIsHonest(proof: AppInstallPurchaseProductClaimGateProof): boolean {
  const keys = new Set(proof.productClaimGateRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.productClaimGateRows.map((row) => row.productClaimGateState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceManualEvidencePacketProofVersion === SourcePacketVersion &&
    proof.productClaimGateRows.length === StoreSurfaces.length &&
    keys.size === proof.productClaimGateRows.length &&
    GateStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.productClaimGateRows.every(productClaimGateRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
