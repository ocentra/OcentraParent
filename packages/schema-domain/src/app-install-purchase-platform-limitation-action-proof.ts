import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreReportStatusProofReadModel } from './app-install-purchase-provider-store-report-status-proof';
import { AppInstallPurchaseReportStatusReadModelHandoffProofReadModel } from './app-install-purchase-report-status-read-model-handoff-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchasePlatformLimitationActionRowGenerated,
  platformLimitationActionProofIsHonestGenerated,
  platformLimitationActionRowIsHonestGenerated,
  summarizeAppInstallPurchasePlatformLimitationActionProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const PlatformLimitationActionProofVersion = 'app-install-purchase-platform-limitation-action-proof';
const SourceProviderStoreReportStatusProofVersion = AppInstallPurchaseProviderStoreReportStatusProofReadModel.schemaVersion;
const SourceReportStatusReadModelProofVersion = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.schemaVersion;
const PlatformLimitationActionTimestamp = '2026-06-06T03:52:00.000Z';
const PlatformLimitationActionBoundary =
  'platform limitation action proof only; parent-visible limitation follow-up rows link provider store report status rows to report status read-model rows no portal approval UI no portal report UI no external runtime report delivery no provider API execution no store integration no billing provider contact no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const PlatformLimitationActionStates = ['parent-action-ready', 'manual-required', 'unavailable'] as const;
const PlatformLimitationActionNonClaims = [
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-external-runtime-report-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-billing-provider-contact',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const PlatformLimitationActionBoundaryFragments = [
  'parent-visible limitation follow-up rows',
  'provider store report status rows',
  'report status read-model rows',
  'no portal approval UI',
  'no portal report UI',
  'no external runtime report delivery',
  'no provider API execution',
  'no store integration',
  'no billing provider contact',
  'no platform adapter implementation',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchasePlatformLimitationActionProofSchemaVersionSchema = withParser(
  Schema.Literal(PlatformLimitationActionProofVersion)
);
const PlatformLimitationActionStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const PlatformLimitationActionProviderStoreStatusSchema = withParser(
  Schema.Literal('provider-store-report-status-ready', 'manual-required', 'unavailable')
);
const PlatformLimitationActionReportStatusSchema = withParser(
  Schema.Literal('parent-report-status-ready', 'manual-required')
);
const PlatformLimitationActionStateSchema = withParser(Schema.Literal(...PlatformLimitationActionStates));
const PlatformLimitationActionNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const PlatformLimitationActionNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const PlatformLimitationActionNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const PlatformLimitationActionNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const PlatformLimitationActionCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const PlatformLimitationActionNonClaimSchema = withParser(Schema.Literal(...PlatformLimitationActionNonClaims));

const PlatformLimitationActionRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformLimitationActionRowId'
);
const PlatformLimitationActionRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformLimitationActionRef');
const PlatformLimitationActionBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformLimitationActionBoundary'
);

const PlatformLimitationActionRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformLimitationActionProofSchemaVersionSchema,
  platformLimitationActionRowId: PlatformLimitationActionRowIdSchema,
  sourceProviderStoreReportStatusProofVersion: Schema.Literal(SourceProviderStoreReportStatusProofVersion),
  sourceProviderStoreReportStatusRowId: PlatformLimitationActionRefSchema,
  sourceProviderStoreReportStatusState: PlatformLimitationActionProviderStoreStatusSchema,
  sourceReportStatusReadModelProofVersion: Schema.Literal(SourceReportStatusReadModelProofVersion),
  sourceReportStatusReadModelRowIds: Schema.Array(PlatformLimitationActionRefSchema),
  sourceReportStatusReadModelStates: Schema.Array(PlatformLimitationActionReportStatusSchema),
  parentVisibleReportStatusRefs: Schema.Array(PlatformLimitationActionRefSchema),
  auditEventRefs: Schema.Array(PlatformLimitationActionRefSchema),
  platform: ParentPlatformSchema,
  storeSurface: PlatformLimitationActionStoreSurfaceSchema,
  platformLimitationActionState: PlatformLimitationActionStateSchema,
  parentLimitationActionRef: PlatformLimitationActionRefSchema,
  portalApprovalUiClaim: PlatformLimitationActionNotImplementedSchema,
  portalReportUiClaim: PlatformLimitationActionNotImplementedSchema,
  runtimeReportDeliveryClaim: PlatformLimitationActionNotDeliveredSchema,
  providerApiExecutionClaim: PlatformLimitationActionNotExecutedSchema,
  storeIntegrationClaim: PlatformLimitationActionNotClaimedSchema,
  billingProviderContactClaim: PlatformLimitationActionNotExecutedSchema,
  platformAdapterClaim: PlatformLimitationActionNotImplementedSchema,
  childDeviceDeliveryClaim: PlatformLimitationActionNotDeliveredSchema,
  appBlockingClaim: PlatformLimitationActionNotClaimedSchema,
  childDataCustody: PlatformLimitationActionCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: PlatformLimitationActionNotClaimedSchema,
  claimBoundary: PlatformLimitationActionBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type PlatformLimitationActionRowCandidate = Infer<typeof PlatformLimitationActionRowBaseSchema>;

export const AppInstallPurchasePlatformLimitationActionRowSchema = withParser(
  PlatformLimitationActionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformLimitationActionRowIsHonest(row) ||
        'Expected app install/purchase platform limitation action rows to link provider/store status and report status refs without portal, delivery, provider, adapter, custody, or blocking claims'
    )
  )
);

const PlatformLimitationActionProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformLimitationActionProofSchemaVersionSchema,
  sourceProviderStoreReportStatusProofVersion: Schema.Literal(SourceProviderStoreReportStatusProofVersion),
  sourceReportStatusReadModelProofVersion: Schema.Literal(SourceReportStatusReadModelProofVersion),
  platformLimitationActionRows: Schema.Array(AppInstallPurchasePlatformLimitationActionRowSchema),
  nonClaims: Schema.Array(PlatformLimitationActionNonClaimSchema),
  knownGaps: Schema.Array(PlatformLimitationActionRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformLimitationActionProof = Infer<typeof PlatformLimitationActionProofBaseSchema>;

export const AppInstallPurchasePlatformLimitationActionProofSchema = withParser(
  PlatformLimitationActionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformLimitationActionProofIsHonest(proof) ||
        'Expected app install/purchase platform limitation action proof to cover ready/manual/unavailable follow-up rows and preserve non-claims'
    )
  )
);

export const AppInstallPurchasePlatformLimitationActionKnownGaps = [
  'Platform limitation action rows are rust-parent-runtime proof rows only; no portal approval UI or portal report UI is implemented.',
  'Provider/store execution, billing provider contact, platform adapters, child-device delivery, external report delivery, app blocking, and hosted custody remain unimplemented.',
  'Unavailable or manual-required rows stay honest until real provider/store APIs, platform adapters, and child-device delivery proof exist.',
] as const;

export const AppInstallPurchasePlatformLimitationActionProofReadModel =
  AppInstallPurchasePlatformLimitationActionProofSchema.parse({
    schemaVersion: PlatformLimitationActionProofVersion,
    sourceProviderStoreReportStatusProofVersion: SourceProviderStoreReportStatusProofVersion,
    sourceReportStatusReadModelProofVersion: SourceReportStatusReadModelProofVersion,
    platformLimitationActionRows:
      AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows.map(
        platformLimitationActionRow
      ),
    nonClaims: PlatformLimitationActionNonClaims,
    knownGaps: AppInstallPurchasePlatformLimitationActionKnownGaps,
    updatedAt: PlatformLimitationActionTimestamp,
  });

export function summarizeAppInstallPurchasePlatformLimitationActionProof(
  proof: AppInstallPurchasePlatformLimitationActionProof
) {
  return summarizeAppInstallPurchasePlatformLimitationActionProofGenerated(proof);
}

function platformLimitationActionRow(
  row: (typeof AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows)[number]
) {
  return buildAppInstallPurchasePlatformLimitationActionRowGenerated(
    row,
    AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows,
    SourceProviderStoreReportStatusProofVersion,
    SourceReportStatusReadModelProofVersion,
    PlatformLimitationActionBoundary,
    PlatformLimitationActionTimestamp
  );
}

function platformLimitationActionRowIsHonest(row: PlatformLimitationActionRowCandidate): boolean {
  return platformLimitationActionRowIsHonestGenerated(
    row,
    AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows.length,
    PlatformLimitationActionBoundaryFragments
  );
}

function platformLimitationActionProofIsHonest(proof: AppInstallPurchasePlatformLimitationActionProof): boolean {
  return (
    proof.sourceProviderStoreReportStatusProofVersion === SourceProviderStoreReportStatusProofVersion &&
    proof.sourceReportStatusReadModelProofVersion === SourceReportStatusReadModelProofVersion &&
    platformLimitationActionProofIsHonestGenerated(
      proof,
      AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows.length,
      PlatformLimitationActionStates,
      PlatformLimitationActionNonClaims
    ) &&
    proof.platformLimitationActionRows.every(platformLimitationActionRowIsHonest)
  );
}
