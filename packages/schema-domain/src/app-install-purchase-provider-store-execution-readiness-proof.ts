import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovedApiEntitlementProofReadModel } from './app-install-purchase-approved-api-entitlement-proof';
import { AppInstallPurchasePackageSourceAdapterExecutionProofReadModel } from './app-install-purchase-package-source-adapter-execution-proof';
import { AppInstallPurchaseParentActionDeliveryReadinessProofReadModel } from './app-install-purchase-parent-action-delivery-readiness-proof';
import { AppInstallPurchaseStoreStatusHandoffProofReadModel } from './app-install-purchase-store-status-handoff-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ProviderStoreExecutionReadinessProofVersion = 'app-install-purchase-provider-store-execution-readiness-proof';
const SourceApprovedApiEntitlementProofVersion = 'app-install-purchase-approved-api-entitlement-proof';
const SourceStoreStatusHandoffProofVersion = 'app-install-purchase-store-status-handoff-proof';
const SourcePackageSourceAdapterExecutionProofVersion = 'app-install-purchase-package-source-adapter-execution-proof';
const SourceParentActionDeliveryReadinessProofVersion = 'app-install-purchase-parent-action-delivery-readiness-proof';
const ProviderStoreExecutionReadinessTimestamp = '2026-06-05T17:10:00.000Z';
const ProviderStoreExecutionReadinessClaimBoundary =
  'provider store execution readiness proof only; no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime writer delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredProviderStoreExecutionReadinessStates = [
  'provider-store-execution-ready',
  'manual-required',
  'unavailable',
] as const;
const ProviderStoreExecutionReadinessNonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-writer-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ProviderStoreExecutionReadinessBoundaryFragments = [
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime writer delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreExecutionReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ProviderStoreExecutionReadinessProofVersion)
);
const AppInstallPurchaseProviderStoreExecutionReadinessStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseProviderStoreExecutionReadinessStateSchema = withParser(
  Schema.Literal(...RequiredProviderStoreExecutionReadinessStates)
);
const AppInstallPurchaseProviderStoreExecutionReadinessEvidenceStatusSchema = withParser(
  Schema.Literal(
    'approved-api-evidence-required',
    'store-entitlement-evidence-required',
    'manual-platform-review-required',
    'platform-unavailable'
  )
);
const AppInstallPurchaseProviderStoreExecutionReadinessStoreStatusSchema = withParser(
  Schema.Literal(
    'approved-api-status-proof-required',
    'store-entitlement-status-proof-required',
    'manual-platform-status-review-required',
    'platform-store-status-unavailable'
  )
);
const AppInstallPurchaseProviderStoreExecutionReadinessAdapterExecutionStateSchema = withParser(
  Schema.Literal(
    'local-adapter-executed',
    'manual-host-proof-required',
    'device-management-required',
    'apple-entitlement-required',
    'platform-unavailable'
  )
);
const AppInstallPurchaseProviderStoreExecutionReadinessParentActionStateSchema = withParser(
  Schema.Literal('parent-action-delivery-ready', 'manual-review-required')
);
const AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseProviderStoreExecutionReadinessIntegrationClaimSchema = withParser(
  Schema.Literal('not-claimed')
);
const AppInstallPurchaseProviderStoreExecutionReadinessAdapterClaimSchema = withParser(
  Schema.Literal('not-implemented')
);
const AppInstallPurchaseProviderStoreExecutionReadinessDeliveryClaimSchema = withParser(
  Schema.Literal('not-delivered')
);
const AppInstallPurchaseProviderStoreExecutionReadinessCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchaseProviderStoreExecutionReadinessNonClaimSchema = withParser(
  Schema.Literal(...ProviderStoreExecutionReadinessNonClaims)
);

const ProviderStoreExecutionReadinessRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionReadinessRowId'
);
const ProviderStoreExecutionReadinessRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionReadinessRef'
);
const ProviderStoreExecutionReadinessAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionReadinessAuditRef'
);
const ProviderStoreExecutionReadinessClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionReadinessClaimBoundary'
);

const ProviderStoreExecutionReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreExecutionReadinessProofSchemaVersionSchema,
  providerStoreExecutionReadinessRowId: ProviderStoreExecutionReadinessRowIdSchema,
  sourceApprovedApiEntitlementProofVersion: Schema.Literal(SourceApprovedApiEntitlementProofVersion),
  sourceApprovedApiEntitlementRowId: ProviderStoreExecutionReadinessRefSchema,
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  sourceStoreStatusHandoffRowId: ProviderStoreExecutionReadinessRefSchema,
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  sourcePackageSourceAdapterExecutionRowId: ProviderStoreExecutionReadinessRefSchema,
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  sourceParentActionDeliveryReadinessRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  sourceParentActionDeliveryReadinessStates: Schema.Array(
    AppInstallPurchaseProviderStoreExecutionReadinessParentActionStateSchema
  ),
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseProviderStoreExecutionReadinessStoreSurfaceSchema,
  sourceApiEntitlementEvidenceStatus: AppInstallPurchaseProviderStoreExecutionReadinessEvidenceStatusSchema,
  sourceStoreStatusHandoffState: AppInstallPurchaseProviderStoreExecutionReadinessStoreStatusSchema,
  sourcePackageSourceAdapterExecutionState:
    AppInstallPurchaseProviderStoreExecutionReadinessAdapterExecutionStateSchema,
  providerStoreExecutionReadinessState: AppInstallPurchaseProviderStoreExecutionReadinessStateSchema,
  approvedApiEvidenceRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  entitlementEvidenceRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  storeStatusHandoffEvidenceRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  packageSourceAdapterArtifactRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  parentActionAuditEventRefs: Schema.Array(ProviderStoreExecutionReadinessAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  requiredProofRefs: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  googlePlayExecutionClaim: AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema,
  appleAppStoreExecutionClaim: AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema,
  microsoftStoreExecutionClaim: AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema,
  billingProviderContactClaim: AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseProviderStoreExecutionReadinessClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseProviderStoreExecutionReadinessIntegrationClaimSchema,
  platformInterceptionClaim: AppInstallPurchaseProviderStoreExecutionReadinessIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseProviderStoreExecutionReadinessAdapterClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseProviderStoreExecutionReadinessDeliveryClaimSchema,
  runtimeWriterDeliveryClaim: AppInstallPurchaseProviderStoreExecutionReadinessDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseProviderStoreExecutionReadinessDeliveryClaimSchema,
  appBlockingClaim: AppInstallPurchaseProviderStoreExecutionReadinessIntegrationClaimSchema,
  childDataCustody: AppInstallPurchaseProviderStoreExecutionReadinessCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseProviderStoreExecutionReadinessIntegrationClaimSchema,
  claimBoundary: ProviderStoreExecutionReadinessClaimBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreExecutionReadinessRowCandidate = Infer<typeof ProviderStoreExecutionReadinessRowBaseSchema>;

export const AppInstallPurchaseProviderStoreExecutionReadinessRowSchema = withParser(
  ProviderStoreExecutionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreExecutionReadinessRowIsHonest(row) ||
        'Expected provider/store execution readiness rows to link approved API, store handoff, package-source adapter, and parent action readiness refs without provider/store/platform/delivery/custody/blocking claims'
    )
  )
);

const ProviderStoreExecutionReadinessProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreExecutionReadinessProofSchemaVersionSchema,
  sourceApprovedApiEntitlementProofVersion: Schema.Literal(SourceApprovedApiEntitlementProofVersion),
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  providerStoreExecutionReadinessRows: Schema.Array(AppInstallPurchaseProviderStoreExecutionReadinessRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseProviderStoreExecutionReadinessNonClaimSchema),
  knownGaps: Schema.Array(ProviderStoreExecutionReadinessRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreExecutionReadinessProof = Infer<
  typeof ProviderStoreExecutionReadinessProofBaseSchema
>;

export const AppInstallPurchaseProviderStoreExecutionReadinessProofSchema = withParser(
  ProviderStoreExecutionReadinessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreExecutionReadinessProofIsHonest(proof) ||
        'Expected app install/purchase provider/store execution readiness proof to cover platform sources and preserve provider/store non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreExecutionReadinessKnownGaps = [
  'Provider/store execution readiness rows are contract/proof rows only; no Google Play Apple App Store Microsoft Store or billing provider contact is implemented.',
  'Package-source adapter execution and parent action delivery readiness remain proof-backed source rows; runtime writer delivery, child-device delivery, provider/store execution, platform interception, app blocking, and hosted family custody remain unimplemented.',
] as const;

export const AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel =
  AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.parse({
    schemaVersion: ProviderStoreExecutionReadinessProofVersion,
    sourceApprovedApiEntitlementProofVersion: SourceApprovedApiEntitlementProofVersion,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    sourcePackageSourceAdapterExecutionProofVersion: SourcePackageSourceAdapterExecutionProofVersion,
    sourceParentActionDeliveryReadinessProofVersion: SourceParentActionDeliveryReadinessProofVersion,
    providerStoreExecutionReadinessRows: AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.map(
      providerStoreExecutionReadinessRow
    ),
    nonClaims: ProviderStoreExecutionReadinessNonClaims,
    knownGaps: AppInstallPurchaseProviderStoreExecutionReadinessKnownGaps,
    updatedAt: ProviderStoreExecutionReadinessTimestamp,
  });

export function summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof(
  proof: AppInstallPurchaseProviderStoreExecutionReadinessProof
) {
  return {
    providerStoreExecutionReadinessRows: proof.providerStoreExecutionReadinessRows.length,
    executionReadyRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.providerStoreExecutionReadinessState === 'provider-store-execution-ready'
    ).length,
    manualRequiredRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.providerStoreExecutionReadinessState === 'manual-required'
    ).length,
    unavailableRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.providerStoreExecutionReadinessState === 'unavailable'
    ).length,
    packageSourceAdapterLinkedRows: proof.providerStoreExecutionReadinessRows.filter(
      packageSourceAdapterCoverageIsComplete
    ).length,
    parentActionReadinessLinkedRows: proof.providerStoreExecutionReadinessRows.filter(
      parentActionDeliveryReadinessCoverageIsComplete
    ).length,
    providerExecutedRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function providerStoreExecutionReadinessRow(
  row: (typeof AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows)[number]
) {
  const apiEntitlementRow = apiEntitlementRowFor(row.platform, row.storeSurface);
  const packageSourceAdapterRow = packageSourceAdapterRowFor(row.platform, row.storeSurface);
  const parentActionReadinessRows =
    AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows;
  return {
    schemaVersion: ProviderStoreExecutionReadinessProofVersion,
    providerStoreExecutionReadinessRowId: `provider-store-execution-readiness-${row.platform}-${row.storeSurface}`,
    sourceApprovedApiEntitlementProofVersion: SourceApprovedApiEntitlementProofVersion,
    sourceApprovedApiEntitlementRowId: apiEntitlementRow.evidenceRowId,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    sourceStoreStatusHandoffRowId: row.storeStatusHandoffRowId,
    sourcePackageSourceAdapterExecutionProofVersion: SourcePackageSourceAdapterExecutionProofVersion,
    sourcePackageSourceAdapterExecutionRowId: packageSourceAdapterRow.packageSourceAdapterExecutionRowId,
    sourceParentActionDeliveryReadinessProofVersion: SourceParentActionDeliveryReadinessProofVersion,
    sourceParentActionDeliveryReadinessRefs: parentActionReadinessRows.map(
      (readinessRow) => readinessRow.parentActionDeliveryReadinessRowId
    ),
    sourceParentActionDeliveryReadinessStates: parentActionReadinessRows.map(
      (readinessRow) => readinessRow.parentActionDeliveryReadinessState
    ),
    platform: row.platform,
    storeSurface: row.storeSurface,
    sourceApiEntitlementEvidenceStatus: apiEntitlementRow.evidenceStatus,
    sourceStoreStatusHandoffState: row.storeStatusHandoffState,
    sourcePackageSourceAdapterExecutionState: packageSourceAdapterRow.adapterExecutionState,
    providerStoreExecutionReadinessState: providerStoreExecutionReadinessState(row.platform),
    approvedApiEvidenceRefs: [apiEntitlementRow.approvedApiEvidenceRef],
    entitlementEvidenceRefs: [apiEntitlementRow.entitlementEvidenceRef],
    storeStatusHandoffEvidenceRefs: row.storeStatusHandoffEvidenceRefs,
    packageSourceAdapterArtifactRefs: packageSourceAdapterRow.adapterExecutionArtifactRefs,
    parentActionAuditEventRefs: parentActionReadinessRows.flatMap(
      (readinessRow) => readinessRow.parentActionAuditEventRefs
    ),
    reportRuntimeRefs: uniqueRefs([...row.sourceReportRuntimeRefs, ...packageSourceAdapterRow.reportRefs]),
    requiredProofRefs: uniqueRefs([
      ...apiEntitlementRow.requiredProofRefs,
      ...packageSourceAdapterRow.requiredProofRefs,
    ]),
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: ProviderStoreExecutionReadinessClaimBoundary,
    evaluatedAt: ProviderStoreExecutionReadinessTimestamp,
  } as const;
}

function apiEntitlementRowFor(platform: typeof ParentPlatformSchema.Type, storeSurface: string) {
  return AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows.find(
    (row) => row.platform === platform && row.storeSurface === storeSurface
  )!;
}

function packageSourceAdapterRowFor(platform: typeof ParentPlatformSchema.Type, storeSurface: string) {
  return AppInstallPurchasePackageSourceAdapterExecutionProofReadModel.packageSourceAdapterExecutionRows.find(
    (row) => row.platform === platform && row.storeSurface === storeSurface
  )!;
}

function providerStoreExecutionReadinessState(platform: typeof ParentPlatformSchema.Type) {
  if (platform === 'windows') {
    return 'provider-store-execution-ready';
  }
  if (platform === 'linux') {
    return 'unavailable';
  }
  return 'manual-required';
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function providerStoreExecutionReadinessRowIsHonest(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    providerStoreExecutionReadinessStateMatchesSources(row) &&
    approvedApiEntitlementCoverageIsComplete(row) &&
    storeStatusHandoffCoverageIsComplete(row) &&
    packageSourceAdapterCoverageIsComplete(row) &&
    parentActionDeliveryReadinessCoverageIsComplete(row) &&
    providerStoreExecutionReadinessClaimsStayUnimplemented(row) &&
    providerStoreExecutionReadinessBoundaryIsExplicit(row.claimBoundary)
  );
}

function providerStoreExecutionReadinessStateMatchesSources(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  const expectedState = providerStoreExecutionReadinessState(row.platform);
  if (expectedState === 'provider-store-execution-ready') {
    return (
      row.providerStoreExecutionReadinessState === expectedState &&
      row.sourceApiEntitlementEvidenceStatus === 'approved-api-evidence-required' &&
      row.sourceStoreStatusHandoffState === 'approved-api-status-proof-required' &&
      row.sourcePackageSourceAdapterExecutionState === 'local-adapter-executed'
    );
  }
  if (expectedState === 'unavailable') {
    return (
      row.providerStoreExecutionReadinessState === expectedState &&
      row.sourceApiEntitlementEvidenceStatus === 'platform-unavailable' &&
      row.sourceStoreStatusHandoffState === 'platform-store-status-unavailable' &&
      row.sourcePackageSourceAdapterExecutionState === 'platform-unavailable'
    );
  }
  return (
    row.providerStoreExecutionReadinessState === expectedState &&
    row.sourceApiEntitlementEvidenceStatus !== 'platform-unavailable' &&
    row.sourcePackageSourceAdapterExecutionState !== 'local-adapter-executed'
  );
}

function approvedApiEntitlementCoverageIsComplete(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    row.sourceApprovedApiEntitlementRowId.length > 0 &&
    row.approvedApiEvidenceRefs.length > 0 &&
    row.entitlementEvidenceRefs.length > 0 &&
    row.requiredProofRefs.length > 0
  );
}

function storeStatusHandoffCoverageIsComplete(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    row.sourceStoreStatusHandoffRowId.length > 0 &&
    row.storeStatusHandoffEvidenceRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function packageSourceAdapterCoverageIsComplete(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    row.sourcePackageSourceAdapterExecutionRowId.length > 0 &&
    row.packageSourceAdapterArtifactRefs.length > 0
  );
}

function parentActionDeliveryReadinessCoverageIsComplete(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  const readinessStates = new Set(row.sourceParentActionDeliveryReadinessStates);
  return (
    row.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    row.sourceParentActionDeliveryReadinessRefs.length ===
      AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows.length &&
    readinessStates.has('parent-action-delivery-ready') &&
    readinessStates.has('manual-review-required') &&
    row.parentActionAuditEventRefs.length > 0
  );
}

function providerStoreExecutionReadinessClaimsStayUnimplemented(
  row: ProviderStoreExecutionReadinessRowCandidate
): boolean {
  return providerExecutionClaimsStayUnimplemented(row) && deliveryAndCustodyClaimsStayUnimplemented(row);
}

function providerExecutionClaimsStayUnimplemented(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented'
  );
}

function deliveryAndCustodyClaimsStayUnimplemented(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function providerStoreExecutionReadinessProofIsHonest(
  proof: AppInstallPurchaseProviderStoreExecutionReadinessProof
): boolean {
  const keys = new Set(proof.providerStoreExecutionReadinessRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(
    proof.providerStoreExecutionReadinessRows.map((row) => row.providerStoreExecutionReadinessState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    proof.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    proof.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    proof.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    proof.providerStoreExecutionReadinessRows.length === RequiredPlatformSources.length &&
    keys.size === proof.providerStoreExecutionReadinessRows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    RequiredProviderStoreExecutionReadinessStates.every((state) => states.has(state)) &&
    ProviderStoreExecutionReadinessNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.providerStoreExecutionReadinessRows.every(providerStoreExecutionReadinessRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function providerStoreExecutionReadinessBoundaryIsExplicit(
  boundary: typeof ProviderStoreExecutionReadinessClaimBoundarySchema.Type
): boolean {
  return ProviderStoreExecutionReadinessBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
