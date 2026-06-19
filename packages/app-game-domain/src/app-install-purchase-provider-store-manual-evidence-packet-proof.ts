import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel } from './app-install-purchase-provider-store-execution-preflight-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-provider-store-manual-evidence-packet-proof';
const PlatformProofReadinessVersion = 'app-install-purchase-platform-proof-readiness';
const ProviderStorePreflightVersion = 'app-install-purchase-provider-store-execution-preflight-proof';
const UpdatedAt = '2026-06-06T09:45:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const PacketStates = ['manual-evidence-packet-ready', 'manual-review-required', 'provider-unavailable'] as const;
const NonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'provider store manual evidence packet proof only; parent-owned packet links platform manual evidence refs and provider preflight refs no Google Play execution no Apple App Store execution no Microsoft Store execution no provider API execution no store integration no platform adapter implementation no runtime writer delivery no runtime report delivery no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'parent-owned packet',
  'platform manual evidence refs',
  'provider preflight refs',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreManualEvidencePacketProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const PacketStateSchema = withParser(Schema.Literal(...PacketStates));
const PlatformReadinessStateSchema = withParser(
  Schema.Literal('manual-proof-required', 'policy-blocked', 'unavailable')
);
const PreflightStateSchema = withParser(
  Schema.Literal('preflight-ready', 'manual-provider-proof-required', 'provider-unavailable')
);
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreManualEvidencePacketRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreManualEvidencePacketBoundary');

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreManualEvidencePacketProofSchemaVersionSchema,
  manualEvidencePacketRowId: RefSchema,
  sourcePlatformProofReadinessVersion: Schema.Literal(PlatformProofReadinessVersion),
  sourcePlatformProofReadinessState: PlatformReadinessStateSchema,
  sourceProviderStorePreflightVersion: Schema.Literal(ProviderStorePreflightVersion),
  sourceProviderStorePreflightRowId: RefSchema,
  sourceProviderStorePreflightState: PreflightStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  manualEvidencePacketState: PacketStateSchema,
  requiredManualEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderEvidenceRefs: Schema.Array(RefSchema),
  runtimeWriterReceiptRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  providerApiExecutionClaim: NotExecutedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type RowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        rowIsHonest(row) ||
        'Expected provider/store manual evidence packet rows to link platform evidence and provider preflight refs without execution, integration, adapter, delivery, custody, or blocking claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreManualEvidencePacketProofSchemaVersionSchema,
  sourcePlatformProofReadinessVersion: Schema.Literal(PlatformProofReadinessVersion),
  sourceProviderStorePreflightVersion: Schema.Literal(ProviderStorePreflightVersion),
  manualEvidencePacketRows: Schema.Array(AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreManualEvidencePacketProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        proofIsHonest(proof) ||
        'Expected provider/store manual evidence packet proof to cover all store surfaces and preserve execution non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreManualEvidencePacketKnownGaps = [
  'Manual evidence packet rows are parent-owned proof packets only; no provider or store execution is implemented.',
  'Windows can be packet-ready from current preflight and manual evidence refs, but still needs real Microsoft Store or package-source proof before product claim.',
  'macOS Android and iOS remain manual-review-required until signing entitlement managed-profile provider and policy review evidence exists.',
  'Linux remains provider-unavailable until a tested package-manager source path and provider/store preflight are proved.',
] as const;

export const AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel =
  AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema.parse({
    schemaVersion: ProofVersion,
    sourcePlatformProofReadinessVersion: PlatformProofReadinessVersion,
    sourceProviderStorePreflightVersion: ProviderStorePreflightVersion,
    manualEvidencePacketRows:
      AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows.map(
        manualEvidencePacketRow
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProviderStoreManualEvidencePacketKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProof(
  proof: AppInstallPurchaseProviderStoreManualEvidencePacketProof
) {
  return {
    manualEvidencePacketRows: proof.manualEvidencePacketRows.length,
    packetReadyRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'manual-evidence-packet-ready'
    ).length,
    manualReviewRequiredRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'manual-review-required'
    ).length,
    providerUnavailableRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'provider-unavailable'
    ).length,
    providerExecutedRows: proof.manualEvidencePacketRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.manualEvidencePacketRows.filter((row) => row.childDeviceDeliveryClaim !== 'not-delivered')
      .length,
  } as const;
}

function manualEvidencePacketRow(
  preflightRow: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number]
) {
  const platformRow = AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows.find(
    (row) => row.platform === preflightRow.platform
  );
  if (!platformRow) {
    throw new Error(`Missing platform proof readiness row for ${preflightRow.platform}`);
  }
  return {
    schemaVersion: ProofVersion,
    manualEvidencePacketRowId: `provider-store-manual-evidence-packet-${preflightRow.platform}-${preflightRow.storeSurface}`,
    sourcePlatformProofReadinessVersion: PlatformProofReadinessVersion,
    sourcePlatformProofReadinessState: platformRow.platformProofReadinessState,
    sourceProviderStorePreflightVersion: ProviderStorePreflightVersion,
    sourceProviderStorePreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
    sourceProviderStorePreflightState: preflightRow.providerStoreExecutionPreflightState,
    platform: preflightRow.platform,
    storeSurface: preflightRow.storeSurface,
    manualEvidencePacketState: packetState(
      platformRow.platformProofReadinessState,
      preflightRow.providerStoreExecutionPreflightState
    ),
    requiredManualEvidenceRefs: platformRow.requiredManualEvidenceRefs,
    requiredProviderEvidenceRefs: preflightRow.requiredProviderEvidenceRefs,
    runtimeWriterReceiptRefs: preflightRow.runtimeWriterReceiptRefs,
    auditEventRefs: preflightRow.auditEventRefs,
    reportRuntimeRefs: preflightRow.reportRuntimeRefs,
    providerApiExecutionClaim: 'not-executed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: Boundary,
    evaluatedAt: UpdatedAt,
  } as const;
}

function packetState(
  platformState: (typeof AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows)[number]['platformProofReadinessState'],
  preflightState: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number]['providerStoreExecutionPreflightState']
): (typeof PacketStates)[number] {
  if (platformState === 'unavailable' || preflightState === 'provider-unavailable') {
    return 'provider-unavailable';
  }
  if (platformState === 'manual-proof-required' && preflightState === 'preflight-ready') {
    return 'manual-evidence-packet-ready';
  }
  return 'manual-review-required';
}

function rowIsHonest(row: RowCandidate): boolean {
  return (
    row.sourcePlatformProofReadinessVersion === PlatformProofReadinessVersion &&
    row.sourceProviderStorePreflightVersion === ProviderStorePreflightVersion &&
    rowHasEvidenceReferences(row) &&
    rowHasNoExecutionClaims(row) &&
    rowHasNoDeliveryCustodyClaims(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function rowHasEvidenceReferences(row: RowCandidate): boolean {
  return (
    row.sourceProviderStorePreflightRowId.length > 0 &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function rowHasNoExecutionClaims(row: RowCandidate): boolean {
  return (
    row.providerApiExecutionClaim === 'not-executed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented'
  );
}

function rowHasNoDeliveryCustodyClaims(row: RowCandidate): boolean {
  return (
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function proofIsHonest(proof: AppInstallPurchaseProviderStoreManualEvidencePacketProof): boolean {
  const keys = new Set(proof.manualEvidencePacketRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.manualEvidencePacketRows.map((row) => row.manualEvidencePacketState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourcePlatformProofReadinessVersion === PlatformProofReadinessVersion &&
    proof.sourceProviderStorePreflightVersion === ProviderStorePreflightVersion &&
    proof.manualEvidencePacketRows.length === StoreSurfaces.length &&
    keys.size === proof.manualEvidencePacketRows.length &&
    PacketStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.manualEvidencePacketRows.every(rowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

