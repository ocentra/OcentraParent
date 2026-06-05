import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentActionRuntimeHandoffProofReadModel } from './app-install-purchase-parent-action-runtime-handoff-proof';
import { AppInstallPurchasePlatformAdapterBoundaryProofReadModel } from './app-install-purchase-platform-adapter-boundary-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const StoreStatusHandoffText = Schema.String.pipe(Schema.minLength(1));
const StoreStatusHandoffProofVersion = 'app-install-purchase-store-status-handoff-proof';
const SourceParentActionRuntimeHandoffProofVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourcePlatformAdapterBoundaryProofVersion = 'app-install-purchase-platform-adapter-boundary-proof';
const StoreStatusHandoffTimestamp = '2026-06-05T05:30:00.000Z';
const StoreStatusHandoffClaimBoundary =
  'store status handoff proof only; no provider API execution no store integration no platform adapter implementation no parent action runtime delivery no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredParentActionRuntimeStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredStoreStatusHandoffStates = [
  'approved-api-status-proof-required',
  'store-entitlement-status-proof-required',
  'manual-platform-status-review-required',
  'platform-store-status-unavailable',
] as const;
const StoreStatusHandoffNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-parent-action-runtime-delivery',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(StoreStatusHandoffProofVersion)
);
const AppInstallPurchaseStoreStatusHandoffStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseStoreStatusHandoffStateSchema = withParser(Schema.Literal(...RequiredStoreStatusHandoffStates));
const AppInstallPurchaseStoreStatusRuntimeStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'unavailable')
);
const AppInstallPurchaseStoreStatusAdapterEvidenceStateSchema = withParser(
  Schema.Literal(
    'approved-api-adapter-evidence-required',
    'entitlement-adapter-evidence-required',
    'manual-platform-review-required',
    'platform-unavailable'
  )
);
const AppInstallPurchaseStoreStatusParentActionRuntimeStatusSchema = withParser(
  Schema.Literal(...RequiredParentActionRuntimeStatuses)
);
const AppInstallPurchaseStoreStatusClaimSchema = withParser(Schema.Literal('status-handoff-proof-only'));
const AppInstallPurchaseStoreStatusDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseStoreStatusProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseStoreStatusIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseStoreStatusInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseStoreStatusHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusNonClaimSchema = withParser(Schema.Literal(...StoreStatusHandoffNonClaims));

const StoreStatusHandoffRowIdSchema = StoreStatusHandoffText.pipe(
  Schema.brand('AppInstallPurchaseStoreStatusHandoffRowId')
);
const StoreStatusHandoffSourceRowIdSchema = StoreStatusHandoffText.pipe(
  Schema.brand('AppInstallPurchaseStoreStatusHandoffSourceRowId')
);
const StoreStatusHandoffRefSchema = StoreStatusHandoffText.pipe(
  Schema.brand('AppInstallPurchaseStoreStatusHandoffRef')
);
const StoreStatusHandoffReportRefSchema = StoreStatusHandoffText.pipe(
  Schema.brand('AppInstallPurchaseStoreStatusHandoffReportRef')
);
const StoreStatusHandoffClaimBoundarySchema = StoreStatusHandoffText.pipe(
  Schema.brand('AppInstallPurchaseStoreStatusHandoffClaimBoundary')
);

const StoreStatusHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema,
  storeStatusHandoffRowId: StoreStatusHandoffRowIdSchema,
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  sourcePlatformAdapterBoundaryRowId: StoreStatusHandoffSourceRowIdSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceParentActionRuntimeHandoffRefs: Schema.Array(StoreStatusHandoffRefSchema),
  sourceParentActionRuntimeStatuses: Schema.Array(AppInstallPurchaseStoreStatusParentActionRuntimeStatusSchema),
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseStoreStatusHandoffStoreSurfaceSchema,
  sourceAdapterEvidenceState: AppInstallPurchaseStoreStatusAdapterEvidenceStateSchema,
  sourceAdapterRuntimeState: AppInstallPurchaseStoreStatusRuntimeStateSchema,
  storeStatusHandoffState: AppInstallPurchaseStoreStatusHandoffStateSchema,
  storeStatusRuntimeState: AppInstallPurchaseStoreStatusRuntimeStateSchema,
  storeStatusHandoffEvidenceRefs: Schema.Array(StoreStatusHandoffRefSchema),
  sourceReportRuntimeRefs: Schema.Array(StoreStatusHandoffReportRefSchema),
  storeStatusHandoffClaim: AppInstallPurchaseStoreStatusClaimSchema,
  statusHandoffDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseStoreStatusProviderClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseStoreStatusIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseStoreStatusAdapterClaimSchema,
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  childDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseStoreStatusInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseStoreStatusBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseStoreStatusCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseStoreStatusHostedCustodyClaimSchema,
  claimBoundary: StoreStatusHandoffClaimBoundarySchema,
  handedOffAt: ParentTimestampSchema,
});

type StoreStatusHandoffRowCandidate = Infer<typeof StoreStatusHandoffRowBaseSchema>;

export const AppInstallPurchaseStoreStatusHandoffRowSchema = withParser(
  StoreStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        storeStatusHandoffRowIsHonest(row) ||
        'Expected app install/purchase store status handoff rows to link adapter and parent action runtime evidence without provider, store, adapter, delivery, custody, interception, or blocking claims'
    )
  )
);

const StoreStatusHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema,
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  storeStatusHandoffRows: Schema.Array(AppInstallPurchaseStoreStatusHandoffRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseStoreStatusNonClaimSchema),
  knownGaps: Schema.Array(StoreStatusHandoffRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseStoreStatusHandoffProof = Infer<typeof StoreStatusHandoffProofBaseSchema>;

export const AppInstallPurchaseStoreStatusHandoffProofSchema = withParser(
  StoreStatusHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        storeStatusHandoffProofIsHonest(proof) ||
        'Expected app install/purchase store status handoff proof to cover platform sources and preserve runtime non-claims'
    )
  )
);

export const AppInstallPurchaseStoreStatusHandoffKnownGaps = [
  'Store status handoff rows are status proof only; no provider/store status API execution is implemented.',
  'Platform adapter rows remain readiness/manual/unavailable boundaries only and do not deliver parent actions to store or child-device runtimes.',
  'Portal approval UX, parent action runtime delivery, child-device delivery, report writer/delivery, real interception, app blocking, and Ocentra-hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchaseStoreStatusHandoffProofReadModel = AppInstallPurchaseStoreStatusHandoffProofSchema.parse(
  {
    schemaVersion: StoreStatusHandoffProofVersion,
    sourcePlatformAdapterBoundaryProofVersion: SourcePlatformAdapterBoundaryProofVersion,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    storeStatusHandoffRows:
      AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows.map(storeStatusHandoffRow),
    nonClaims: StoreStatusHandoffNonClaims,
    knownGaps: AppInstallPurchaseStoreStatusHandoffKnownGaps,
    updatedAt: StoreStatusHandoffTimestamp,
  }
);

export function summarizeAppInstallPurchaseStoreStatusHandoffProof(proof: AppInstallPurchaseStoreStatusHandoffProof) {
  return {
    storeStatusHandoffRows: proof.storeStatusHandoffRows.length,
    approvedApiRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'approved-api-status-proof-required'
    ).length,
    entitlementRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'store-entitlement-status-proof-required'
    ).length,
    manualRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'manual-platform-status-review-required'
    ).length,
    unavailableRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'platform-store-status-unavailable'
    ).length,
    parentActionRuntimeLinkedRows: proof.storeStatusHandoffRows.filter(parentActionRuntimeCoverageIsComplete).length,
    deliveredRows: proof.storeStatusHandoffRows.filter((row) => row.statusHandoffDeliveryClaim !== 'not-delivered')
      .length,
  } as const;
}

function storeStatusHandoffRow(
  row: (typeof AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows)[number]
) {
  return {
    schemaVersion: StoreStatusHandoffProofVersion,
    storeStatusHandoffRowId: `store-status-handoff-${row.platform}-${row.storeSurface}`,
    sourcePlatformAdapterBoundaryProofVersion: SourcePlatformAdapterBoundaryProofVersion,
    sourcePlatformAdapterBoundaryRowId: row.adapterBoundaryRowId,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRefs:
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.map(
        (handoffRow) => handoffRow.runtimeHandoffRowId
      ),
    sourceParentActionRuntimeStatuses:
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.map(
        (handoffRow) => handoffRow.runtimeHandoffStatus
      ),
    platform: row.platform,
    storeSurface: row.storeSurface,
    sourceAdapterEvidenceState: row.adapterEvidenceState,
    sourceAdapterRuntimeState: row.adapterRuntimeState,
    storeStatusHandoffState: storeStatusHandoffState(row.adapterEvidenceState),
    storeStatusRuntimeState: row.adapterRuntimeState,
    storeStatusHandoffEvidenceRefs: [
      row.approvedApiEvidenceRef,
      row.entitlementEvidenceRef,
      row.limitationReportRef,
      ...row.adapterReadinessEvidenceRefs,
    ],
    sourceReportRuntimeRefs: row.reportRuntimeRefs,
    storeStatusHandoffClaim: 'status-handoff-proof-only',
    statusHandoffDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: 'not-implemented',
    parentActionRuntimeDeliveryClaim: 'not-delivered',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: StoreStatusHandoffClaimBoundary,
    handedOffAt: StoreStatusHandoffTimestamp,
  } as const;
}

function storeStatusHandoffState(
  adapterEvidenceState: (typeof AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows)[number]['adapterEvidenceState']
) {
  if (adapterEvidenceState === 'approved-api-adapter-evidence-required') {
    return 'approved-api-status-proof-required';
  }
  if (adapterEvidenceState === 'entitlement-adapter-evidence-required') {
    return 'store-entitlement-status-proof-required';
  }
  if (adapterEvidenceState === 'manual-platform-review-required') {
    return 'manual-platform-status-review-required';
  }
  return 'platform-store-status-unavailable';
}

function storeStatusHandoffRowIsHonest(row: StoreStatusHandoffRowCandidate): boolean {
  return (
    storeStatusMatchesAdapterEvidence(row) &&
    parentActionRuntimeCoverageIsComplete(row) &&
    storeStatusHandoffEvidenceIsComplete(row) &&
    storeStatusHandoffClaimsStayUnimplemented(row) &&
    storeStatusHandoffBoundaryIsExplicit(row.claimBoundary)
  );
}

function storeStatusMatchesAdapterEvidence(row: StoreStatusHandoffRowCandidate): boolean {
  if (row.sourceAdapterEvidenceState === 'approved-api-adapter-evidence-required') {
    return row.storeStatusHandoffState === 'approved-api-status-proof-required' && notImplemented(row);
  }
  if (row.sourceAdapterEvidenceState === 'entitlement-adapter-evidence-required') {
    return row.storeStatusHandoffState === 'store-entitlement-status-proof-required' && notImplemented(row);
  }
  if (row.sourceAdapterEvidenceState === 'manual-platform-review-required') {
    return (
      row.storeStatusHandoffState === 'manual-platform-status-review-required' &&
      row.sourceAdapterRuntimeState === 'manual-required' &&
      row.storeStatusRuntimeState === 'manual-required'
    );
  }
  return (
    row.storeStatusHandoffState === 'platform-store-status-unavailable' &&
    row.sourceAdapterRuntimeState === 'unavailable' &&
    row.storeStatusRuntimeState === 'unavailable'
  );
}

function notImplemented(row: StoreStatusHandoffRowCandidate): boolean {
  return row.sourceAdapterRuntimeState === 'not-implemented' && row.storeStatusRuntimeState === 'not-implemented';
}

function parentActionRuntimeCoverageIsComplete(row: StoreStatusHandoffRowCandidate): boolean {
  const statusSet = new Set(row.sourceParentActionRuntimeStatuses);
  return (
    row.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    row.sourceParentActionRuntimeHandoffRefs.length ===
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.length &&
    row.sourceParentActionRuntimeStatuses.length ===
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.length &&
    RequiredParentActionRuntimeStatuses.every((status) => statusSet.has(status))
  );
}

function storeStatusHandoffEvidenceIsComplete(row: StoreStatusHandoffRowCandidate): boolean {
  return (
    row.sourcePlatformAdapterBoundaryProofVersion === SourcePlatformAdapterBoundaryProofVersion &&
    row.sourcePlatformAdapterBoundaryRowId.length > 0 &&
    row.storeStatusHandoffEvidenceRefs.length >= 4 &&
    row.sourceReportRuntimeRefs.length > 0
  );
}

function storeStatusHandoffClaimsStayUnimplemented(row: StoreStatusHandoffRowCandidate): boolean {
  return (
    row.storeStatusHandoffClaim === 'status-handoff-proof-only' &&
    row.statusHandoffDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function storeStatusHandoffProofIsHonest(proof: AppInstallPurchaseStoreStatusHandoffProof): boolean {
  return (
    proof.sourcePlatformAdapterBoundaryProofVersion === SourcePlatformAdapterBoundaryProofVersion &&
    proof.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    storeStatusHandoffRowsAreComplete(proof.storeStatusHandoffRows) &&
    storeStatusHandoffNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function storeStatusHandoffRowsAreComplete(rows: readonly StoreStatusHandoffRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(rows.map((row) => row.storeStatusHandoffState));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    RequiredStoreStatusHandoffStates.every((state) => states.has(state)) &&
    rows.every((row) => storeStatusHandoffRowIsHonest(row))
  );
}

function storeStatusHandoffNonClaimsAreComplete(
  nonClaims: readonly (typeof StoreStatusHandoffNonClaims)[number][]
): boolean {
  const claimSet = new Set(nonClaims);
  return StoreStatusHandoffNonClaims.every((claim) => claimSet.has(claim));
}

function storeStatusHandoffBoundaryIsExplicit(boundary: typeof StoreStatusHandoffClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no provider API execution') &&
    boundary.includes('no store integration') &&
    boundary.includes('no platform adapter implementation') &&
    boundary.includes('no parent action runtime delivery') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('no app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}
