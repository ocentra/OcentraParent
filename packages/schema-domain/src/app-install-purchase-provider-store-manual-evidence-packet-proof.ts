import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel } from './app-install-purchase-provider-store-execution-preflight-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStoreManualEvidencePacketRowGenerated,
  providerStoreManualEvidencePacketProofIsHonestGenerated,
  providerStoreManualEvidencePacketRowIsHonestGenerated,
  summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

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
  return summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProofGenerated(proof);
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
  return buildAppInstallPurchaseProviderStoreManualEvidencePacketRowGenerated(
    {
      platform: preflightRow.platform,
      storeSurface: preflightRow.storeSurface,
      providerStoreExecutionPreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
      providerStoreExecutionPreflightState: preflightRow.providerStoreExecutionPreflightState,
      requiredProviderEvidenceRefs: preflightRow.requiredProviderEvidenceRefs,
      runtimeWriterReceiptRefs: preflightRow.runtimeWriterReceiptRefs,
      auditEventRefs: preflightRow.auditEventRefs,
      reportRuntimeRefs: preflightRow.reportRuntimeRefs,
    },
    platformRow,
    PlatformProofReadinessVersion,
    ProviderStorePreflightVersion,
    Boundary,
    UpdatedAt
  );
}

function rowIsHonest(row: RowCandidate): boolean {
  return (
    row.sourcePlatformProofReadinessVersion === PlatformProofReadinessVersion &&
    row.sourceProviderStorePreflightVersion === ProviderStorePreflightVersion &&
    providerStoreManualEvidencePacketRowIsHonestGenerated(row, BoundaryFragments)
  );
}

function proofIsHonest(proof: AppInstallPurchaseProviderStoreManualEvidencePacketProof): boolean {
  return (
    proof.sourcePlatformProofReadinessVersion === PlatformProofReadinessVersion &&
    proof.sourceProviderStorePreflightVersion === ProviderStorePreflightVersion &&
    providerStoreManualEvidencePacketProofIsHonestGenerated(proof, StoreSurfaces, PacketStates, NonClaims) &&
    proof.manualEvidencePacketRows.every(rowIsHonest)
  );
}
