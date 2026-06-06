import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreReportStatusProofReadModel } from './app-install-purchase-provider-store-report-status-proof';
import { AppInstallPurchaseReportStatusReadModelHandoffProofReadModel } from './app-install-purchase-report-status-read-model-handoff-proof';
import { ParentTimestampSchema } from './reference-primitives';

const LimitationSummaryText = Schema.String.pipe(Schema.minLength(1));
const LimitationSummaryProofVersion = 'app-install-purchase-limitation-summary-proof';
const SourceProviderStoreReportStatusProofVersion = 'app-install-purchase-provider-store-report-status-proof';
const SourceReportStatusReadModelProofVersion = 'app-install-purchase-report-status-read-model-handoff-proof';
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

const LimitationSummaryRowIdSchema = LimitationSummaryText.pipe(
  Schema.brand('AppInstallPurchaseLimitationSummaryRowId')
);
const LimitationSummaryRefSchema = LimitationSummaryText.pipe(Schema.brand('AppInstallPurchaseLimitationSummaryRef'));
const LimitationSummaryBoundarySchema = LimitationSummaryText.pipe(
  Schema.brand('AppInstallPurchaseLimitationSummaryBoundary')
);

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
  'Limitation summary rows are parent-domain proof rows only; no portal approval UI or report UI is implemented.',
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
  return {
    limitationSummaryRows: proof.limitationSummaryRows.length,
    readyRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'ready').length,
    manualRequiredRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'manual-required')
      .length,
    unavailableRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'unavailable').length,
    sourceProviderStoreRows: proof.limitationSummaryRows.flatMap((row) => row.sourceProviderStoreReportStatusRowIds)
      .length,
    sourceReportStatusRows: proof.limitationSummaryRows.flatMap((row) => row.sourceReportStatusReadModelRowIds).length,
    providerExecutedRows: proof.limitationSummaryRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed')
      .length,
    externallyDeliveredRows: proof.limitationSummaryRows.filter(
      (row) => row.runtimeReportDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function limitationSummaryRow(state: (typeof LimitationSummaryStates)[number]) {
  const providerRows = AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows.filter(
    (row) => providerStateMapsToSummary(row.providerStoreReportStatusState) === state
  );
  const reportRows = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows.filter(
    (row) => reportStateMapsToSummary(row.parentVisibleReportStatusState) === state
  );
  return {
    schemaVersion: LimitationSummaryProofVersion,
    limitationSummaryRowId: `app-install-limitation-summary-${state}`,
    limitationSummaryState: state,
    sourceProviderStoreReportStatusProofVersion: SourceProviderStoreReportStatusProofVersion,
    sourceProviderStoreReportStatusRowIds: providerRows.map((row) => row.providerStoreReportStatusRowId),
    sourceProviderStoreReportStatusStates: providerRows.map((row) => row.providerStoreReportStatusState),
    sourceReportStatusReadModelProofVersion: SourceReportStatusReadModelProofVersion,
    sourceReportStatusReadModelRowIds: reportRows.map((row) => row.reportStatusReadModelRowId),
    sourceReportStatusReadModelStates: reportRows.map((row) => row.parentVisibleReportStatusState),
    sourceAuditEventRefs: uniqueRefs([
      ...providerRows.flatMap((row) => row.sourceAuditEventRefs),
      ...reportRows.flatMap((row) => row.reportAuditEventRefs),
    ]),
    parentVisibleSummaryRef: `parent-visible-app-install-limitation-summary-${state}`,
    portalApprovalUiClaim: 'not-implemented',
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    billingProviderContactClaim: 'not-executed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: LimitationSummaryBoundary,
    summarizedAt: LimitationSummaryTimestamp,
  } as const;
}

function providerStateMapsToSummary(state: string): (typeof LimitationSummaryStates)[number] {
  if (state === 'provider-store-report-status-ready') {
    return 'ready';
  }
  if (state === 'unavailable') {
    return 'unavailable';
  }
  return 'manual-required';
}

function reportStateMapsToSummary(state: string): (typeof LimitationSummaryStates)[number] {
  return state === 'parent-report-status-ready' ? 'ready' : 'manual-required';
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function limitationSummaryRowIsHonest(row: LimitationSummaryRowCandidate): boolean {
  return (
    limitationSummaryRowHasExpectedRefs(row) &&
    limitationSummaryClaimsStayUnimplemented(row) &&
    LimitationSummaryBoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function limitationSummaryRowHasExpectedRefs(row: LimitationSummaryRowCandidate): boolean {
  return (
    row.sourceProviderStoreReportStatusProofVersion === SourceProviderStoreReportStatusProofVersion &&
    row.sourceReportStatusReadModelProofVersion === SourceReportStatusReadModelProofVersion &&
    row.parentVisibleSummaryRef.length > 0 &&
    row.sourceAuditEventRefs.length > 0 &&
    row.sourceProviderStoreReportStatusStates.every(
      (state) => providerStateMapsToSummary(state) === row.limitationSummaryState
    ) &&
    row.sourceReportStatusReadModelStates.every(
      (state) => reportStateMapsToSummary(state) === row.limitationSummaryState
    )
  );
}

function limitationSummaryClaimsStayUnimplemented(row: LimitationSummaryRowCandidate): boolean {
  return (
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function limitationSummaryProofIsHonest(proof: AppInstallPurchaseLimitationSummaryProof): boolean {
  const states = new Set(proof.limitationSummaryRows.map((row) => row.limitationSummaryState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreReportStatusProofVersion === SourceProviderStoreReportStatusProofVersion &&
    proof.sourceReportStatusReadModelProofVersion === SourceReportStatusReadModelProofVersion &&
    proof.limitationSummaryRows.length === LimitationSummaryStates.length &&
    LimitationSummaryStates.every((state) => states.has(state)) &&
    LimitationSummaryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.limitationSummaryRows.every(limitationSummaryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
