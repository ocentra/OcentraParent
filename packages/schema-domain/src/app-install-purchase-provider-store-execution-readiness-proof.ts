import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovedApiEntitlementProofReadModel } from './app-install-purchase-approved-api-entitlement-proof';
import { AppInstallPurchasePackageSourceAdapterExecutionProofReadModel } from './app-install-purchase-package-source-adapter-execution-proof';
import { AppInstallPurchaseParentActionDeliveryReadinessProofReadModel } from './app-install-purchase-parent-action-delivery-readiness-proof';
import { AppInstallPurchaseStoreStatusHandoffProofReadModel } from './app-install-purchase-store-status-handoff-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStoreExecutionReadinessRowGenerated,
  providerStoreExecutionReadinessProofIsHonestGenerated,
  providerStoreExecutionReadinessRowIsHonestGenerated,
  providerStoreExecutionReadinessStateGenerated,
  summarizeAppInstallPurchaseProviderStoreExecutionReadinessProofGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';
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
  return summarizeAppInstallPurchaseProviderStoreExecutionReadinessProofGenerated(proof);
}

function providerStoreExecutionReadinessRow(
  row: (typeof AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows)[number]
) {
  const apiEntitlementRow = apiEntitlementRowFor(row.platform, row.storeSurface);
  const packageSourceAdapterRow = packageSourceAdapterRowFor(row.platform, row.storeSurface);
  const parentActionReadinessRows =
    AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows;
  return buildAppInstallPurchaseProviderStoreExecutionReadinessRowGenerated(
    row,
    apiEntitlementRow,
    packageSourceAdapterRow,
    parentActionReadinessRows,
    ProviderStoreExecutionReadinessClaimBoundary,
    ProviderStoreExecutionReadinessTimestamp
  );
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

function providerStoreExecutionReadinessRowIsHonest(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  return (
    row.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    row.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    row.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    row.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    providerStoreExecutionReadinessStateMatchesSources(row) &&
    providerStoreExecutionReadinessRowIsHonestGenerated(
      row,
      AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows.length,
      ProviderStoreExecutionReadinessBoundaryFragments
    )
  );
}

function providerStoreExecutionReadinessStateMatchesSources(row: ProviderStoreExecutionReadinessRowCandidate): boolean {
  const expectedState = providerStoreExecutionReadinessStateGenerated(row.platform);
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

function providerStoreExecutionReadinessProofIsHonest(
  proof: AppInstallPurchaseProviderStoreExecutionReadinessProof
): boolean {
  return (
    proof.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    proof.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    proof.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    proof.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    providerStoreExecutionReadinessProofIsHonestGenerated(
      proof,
      RequiredPlatformSources,
      RequiredProviderStoreExecutionReadinessStates,
      ProviderStoreExecutionReadinessNonClaims
    ) &&
    proof.providerStoreExecutionReadinessRows.every(providerStoreExecutionReadinessRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
