import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreReportStatusProofReadModel } from './app-install-purchase-provider-store-report-status-proof';
import { AppInstallPurchaseReportStatusReadModelHandoffProofReadModel } from './app-install-purchase-report-status-read-model-handoff-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseLimitationSummaryRowGenerated,
  limitationSummaryProofIsHonestGenerated,
  limitationSummaryRowIsHonestGenerated,
  providerStateMapsToSummaryGenerated,
  reportStateMapsToSummaryGenerated,
  summarizeAppInstallPurchaseLimitationSummaryProofGenerated,
} from './generated/app-install-purchase-report-status-helpers';
const LimitationSummaryProofVersion = 'app-install-purchase-limitation-summary-proof';
const SourceProviderStoreReportStatusProofVersion = AppInstallPurchaseProviderStoreReportStatusProofReadModel.schemaVersion;
const SourceReportStatusReadModelProofVersion = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.schemaVersion;
const LimitationSummaryTimestamp = '2026-06-06T03:32:00.000Z';
const LimitationSummaryBoundary =
  'limitation summary proof only; parent-visible ready manual-required and unavailable buckets link provider store report status rows to report status read-model rows no portal approval UI no portal report UI no external runtime report delivery no provider API execution no store integration no billing provider contact no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const LimitationSummaryStates = ['ready', 'manual-required', 'unavailable'] as const;
const LimitationSummaryNonClaims = [
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
const LimitationSummaryBoundaryFragments = [
  'parent-visible ready manual-required and unavailable buckets',
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

export const AppInstallPurchaseLimitationSummaryProofSchemaVersionSchema = withParser(
  Schema.Literal(LimitationSummaryProofVersion)
);
const LimitationSummaryStateSchema = withParser(Schema.Literal(...LimitationSummaryStates));
const LimitationSummaryProviderStoreStateSchema = withParser(
  Schema.Literal('provider-store-report-status-ready', 'manual-required', 'unavailable')
);
const LimitationSummaryReportStatusStateSchema = withParser(
  Schema.Literal('parent-report-status-ready', 'manual-required')
);
const LimitationSummaryNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const LimitationSummaryNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const LimitationSummaryNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const LimitationSummaryNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const LimitationSummaryCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const LimitationSummaryNonClaimSchema = withParser(Schema.Literal(...LimitationSummaryNonClaims));

const LimitationSummaryRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseLimitationSummaryRowId');
const LimitationSummaryRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseLimitationSummaryRef');
const LimitationSummaryBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseLimitationSummaryBoundary');

const LimitationSummaryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseLimitationSummaryProofSchemaVersionSchema,
  limitationSummaryRowId: LimitationSummaryRowIdSchema,
  limitationSummaryState: LimitationSummaryStateSchema,
  sourceProviderStoreReportStatusProofVersion: Schema.Literal(SourceProviderStoreReportStatusProofVersion),
  sourceProviderStoreReportStatusRowIds: Schema.Array(LimitationSummaryRefSchema),
  sourceProviderStoreReportStatusStates: Schema.Array(LimitationSummaryProviderStoreStateSchema),
  sourceReportStatusReadModelProofVersion: Schema.Literal(SourceReportStatusReadModelProofVersion),
  sourceReportStatusReadModelRowIds: Schema.Array(LimitationSummaryRefSchema),
  sourceReportStatusReadModelStates: Schema.Array(LimitationSummaryReportStatusStateSchema),
  sourceAuditEventRefs: Schema.Array(LimitationSummaryRefSchema),
  parentVisibleSummaryRef: LimitationSummaryRefSchema,
  portalApprovalUiClaim: LimitationSummaryNotImplementedSchema,
  portalReportUiClaim: LimitationSummaryNotImplementedSchema,
  runtimeReportDeliveryClaim: LimitationSummaryNotDeliveredSchema,
  providerApiExecutionClaim: LimitationSummaryNotExecutedSchema,
  storeIntegrationClaim: LimitationSummaryNotClaimedSchema,
  billingProviderContactClaim: LimitationSummaryNotExecutedSchema,
  platformAdapterClaim: LimitationSummaryNotImplementedSchema,
  childDeviceDeliveryClaim: LimitationSummaryNotDeliveredSchema,
  appBlockingClaim: LimitationSummaryNotClaimedSchema,
  childDataCustody: LimitationSummaryCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: LimitationSummaryNotClaimedSchema,
  claimBoundary: LimitationSummaryBoundarySchema,
  summarizedAt: ParentTimestampSchema,
});

type LimitationSummaryRowCandidate = Infer<typeof LimitationSummaryRowBaseSchema>;

export const AppInstallPurchaseLimitationSummaryRowSchema = withParser(
  LimitationSummaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        limitationSummaryRowIsHonest(row) ||
        'Expected app install/purchase limitation summary rows to link provider/store and report status rows without portal, provider, platform, delivery, custody, or blocking claims'
    )
  )
);

const LimitationSummaryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseLimitationSummaryProofSchemaVersionSchema,
  sourceProviderStoreReportStatusProofVersion: Schema.Literal(SourceProviderStoreReportStatusProofVersion),
  sourceReportStatusReadModelProofVersion: Schema.Literal(SourceReportStatusReadModelProofVersion),
  limitationSummaryRows: Schema.Array(AppInstallPurchaseLimitationSummaryRowSchema),
  nonClaims: Schema.Array(LimitationSummaryNonClaimSchema),
  knownGaps: Schema.Array(LimitationSummaryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseLimitationSummaryProof = Infer<typeof LimitationSummaryProofBaseSchema>;

export const AppInstallPurchaseLimitationSummaryProofSchema = withParser(
  LimitationSummaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        limitationSummaryProofIsHonest(proof) ||
        'Expected app install/purchase limitation summary proof to cover ready manual-required and unavailable buckets'
    )
  )
);

export const AppInstallPurchaseLimitationSummaryKnownGaps = [
  'Limitation summary rows are rust-parent-runtime proof rows only; no portal approval UI or report UI is implemented.',
  'Provider/store execution, billing/provider contact, platform adapters, external runtime report delivery, child-device delivery, app blocking, child activity data, and hosted family data custody remain unimplemented.',
  'The unavailable bucket reflects provider/store source limitations only; parent-visible report rows stay ready or manual-required until real portal/platform runtime exists.',
] as const;

export const AppInstallPurchaseLimitationSummaryProofReadModel = AppInstallPurchaseLimitationSummaryProofSchema.parse({
  schemaVersion: LimitationSummaryProofVersion,
  sourceProviderStoreReportStatusProofVersion: SourceProviderStoreReportStatusProofVersion,
  sourceReportStatusReadModelProofVersion: SourceReportStatusReadModelProofVersion,
  limitationSummaryRows: LimitationSummaryStates.map(limitationSummaryRow),
  nonClaims: LimitationSummaryNonClaims,
  knownGaps: AppInstallPurchaseLimitationSummaryKnownGaps,
  updatedAt: LimitationSummaryTimestamp,
});

export function summarizeAppInstallPurchaseLimitationSummaryProof(proof: AppInstallPurchaseLimitationSummaryProof) {
  return summarizeAppInstallPurchaseLimitationSummaryProofGenerated(proof);
}

function limitationSummaryRow(state: (typeof LimitationSummaryStates)[number]) {
  const providerRows = AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows.filter(
    (row) => providerStateMapsToSummaryGenerated(row.providerStoreReportStatusState) === state
  );
  const reportRows = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows.filter(
    (row) => reportStateMapsToSummaryGenerated(row.parentVisibleReportStatusState) === state
  );
  return buildAppInstallPurchaseLimitationSummaryRowGenerated(
    state,
    providerRows,
    reportRows,
    SourceProviderStoreReportStatusProofVersion,
    SourceReportStatusReadModelProofVersion,
    LimitationSummaryBoundary,
    LimitationSummaryTimestamp
  );
}

function limitationSummaryRowIsHonest(row: LimitationSummaryRowCandidate): boolean {
  return limitationSummaryRowIsHonestGenerated(
    row,
    SourceProviderStoreReportStatusProofVersion,
    SourceReportStatusReadModelProofVersion,
    LimitationSummaryBoundaryFragments
  );
}

function limitationSummaryProofIsHonest(proof: AppInstallPurchaseLimitationSummaryProof): boolean {
  return (
    limitationSummaryProofIsHonestGenerated(
      proof,
      SourceProviderStoreReportStatusProofVersion,
      SourceReportStatusReadModelProofVersion,
      LimitationSummaryStates,
      LimitationSummaryNonClaims
    ) && proof.limitationSummaryRows.every(limitationSummaryRowIsHonest)
  );
}
