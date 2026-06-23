import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreReportStatusProofReadModel } from './app-install-purchase-provider-store-report-status-proof';
import { AppInstallPurchaseReportStatusReadModelHandoffProofReadModel } from './app-install-purchase-report-status-read-model-handoff-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const PlatformLimitationActionProofVersion = 'app-install-purchase-platform-limitation-action-proof';
const SourceProviderStoreReportStatusProofVersion = 'app-install-purchase-provider-store-report-status-proof';
const SourceReportStatusReadModelProofVersion = 'app-install-purchase-report-status-read-model-handoff-proof';
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
  'Platform limitation action rows are parent-domain proof rows only; no portal approval UI or portal report UI is implemented.',
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
  return {
    platformLimitationActionRows: proof.platformLimitationActionRows.length,
    readyRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'parent-action-ready'
    ).length,
    manualRequiredRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'manual-required'
    ).length,
    unavailableRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'unavailable'
    ).length,
    reportStatusLinkedRows: proof.platformLimitationActionRows.filter(reportStatusCoverageIsComplete).length,
    providerExecutedRows: proof.platformLimitationActionRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    portalRows: proof.platformLimitationActionRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-implemented' || row.portalReportUiClaim !== 'not-implemented'
    ).length,
  } as const;
}

function platformLimitationActionRow(
  row: (typeof AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows)[number]
) {
  const reportRows = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows;
  return {
    schemaVersion: PlatformLimitationActionProofVersion,
    platformLimitationActionRowId: `platform-limitation-action-${row.platform}-${row.storeSurface}`,
    sourceProviderStoreReportStatusProofVersion: SourceProviderStoreReportStatusProofVersion,
    sourceProviderStoreReportStatusRowId: row.providerStoreReportStatusRowId,
    sourceProviderStoreReportStatusState: row.providerStoreReportStatusState,
    sourceReportStatusReadModelProofVersion: SourceReportStatusReadModelProofVersion,
    sourceReportStatusReadModelRowIds: reportRows.map((reportRow) => reportRow.reportStatusReadModelRowId),
    sourceReportStatusReadModelStates: reportRows.map((reportRow) => reportRow.parentVisibleReportStatusState),
    parentVisibleReportStatusRefs: reportRows.map((reportRow) => reportRow.parentVisibleReportStatusRef),
    auditEventRefs: uniqueRefs([
      ...row.sourceAuditEventRefs,
      ...reportRows.flatMap((reportRow) => reportRow.reportAuditEventRefs),
    ]),
    platform: row.platform,
    storeSurface: row.storeSurface,
    platformLimitationActionState: platformLimitationActionState(row.providerStoreReportStatusState),
    parentLimitationActionRef: `parent-limitation-action-${row.platform}-${row.storeSurface}`,
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
    claimBoundary: PlatformLimitationActionBoundary,
    recordedAt: PlatformLimitationActionTimestamp,
  } as const;
}

function platformLimitationActionState(state: string) {
  if (state === 'provider-store-report-status-ready') {
    return 'parent-action-ready';
  }
  if (state === 'unavailable') {
    return 'unavailable';
  }
  return 'manual-required';
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function platformLimitationActionRowIsHonest(row: PlatformLimitationActionRowCandidate): boolean {
  return (
    platformLimitationActionState(row.sourceProviderStoreReportStatusState) === row.platformLimitationActionState &&
    reportStatusCoverageIsComplete(row) &&
    row.parentVisibleReportStatusRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    platformLimitationActionClaimsStayUnimplemented(row) &&
    PlatformLimitationActionBoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function reportStatusCoverageIsComplete(row: PlatformLimitationActionRowCandidate): boolean {
  const states = new Set(row.sourceReportStatusReadModelStates);
  return (
    row.sourceReportStatusReadModelRowIds.length ===
      AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows.length &&
    states.has('parent-report-status-ready') &&
    states.has('manual-required')
  );
}

function platformLimitationActionClaimsStayUnimplemented(row: PlatformLimitationActionRowCandidate): boolean {
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

function platformLimitationActionProofIsHonest(proof: AppInstallPurchasePlatformLimitationActionProof): boolean {
  const states = new Set(proof.platformLimitationActionRows.map((row) => row.platformLimitationActionState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreReportStatusProofVersion === SourceProviderStoreReportStatusProofVersion &&
    proof.sourceReportStatusReadModelProofVersion === SourceReportStatusReadModelProofVersion &&
    proof.platformLimitationActionRows.length ===
      AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows.length &&
    PlatformLimitationActionStates.every((state) => states.has(state)) &&
    PlatformLimitationActionNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.platformLimitationActionRows.every(platformLimitationActionRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
