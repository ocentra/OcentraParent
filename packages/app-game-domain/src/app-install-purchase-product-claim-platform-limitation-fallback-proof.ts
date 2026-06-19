import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformLimitationActionProofReadModel } from './app-install-purchase-platform-limitation-action-proof';
import { AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel } from './app-install-purchase-product-claim-platform-preclaim-proof';
import { AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel } from './app-install-purchase-product-claim-safe-parent-workflow-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-product-claim-platform-limitation-fallback-proof';
const SourcePlatformPreclaimProofVersion = 'app-install-purchase-product-claim-platform-preclaim-proof';
const SourceSafeParentWorkflowProofVersion = 'app-install-purchase-product-claim-safe-parent-workflow-proof';
const SourcePlatformLimitationActionProofVersion = 'app-install-purchase-platform-limitation-action-proof';
const UpdatedAt = '2026-06-06T22:35:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const FallbackStates = [
  'fallback-parent-workflow-ready',
  'manual-platform-limitation-fallback-required',
  'unsupported-platform-limitation-fallback-blocked',
] as const;
const NonClaims = [
  'no-product-claim-approval',
  'no-portal-approval-ui',
  'no-portal-report-ui',
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
  'no-runtime-report-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'product claim platform limitation fallback proof only; links platform preclaim safe parent workflow and platform limitation action rows so product claims remain blocked or manual while parent-visible fallback workflow refs stay available no product claim approval no portal approval UI no portal report UI no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'product claim platform limitation fallback proof only',
  'platform preclaim',
  'safe parent workflow',
  'platform limitation action rows',
  'product claims remain blocked or manual',
  'parent-visible fallback workflow refs',
  'no product claim approval',
  'no portal approval UI',
  'no portal report UI',
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
  'no runtime report delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const FallbackStateSchema = withParser(Schema.Literal(...FallbackStates));
const PlatformPreclaimStateSchema = withParser(
  Schema.Literal('manual-platform-preclaim-required', 'unsupported-platform-preclaim-blocked')
);
const SafeParentWorkflowStateSchema = withParser(
  Schema.Literal('safe-parent-review-ready', 'manual-parent-review-required', 'unsupported-store-workflow-blocked')
);
const PlatformLimitationActionStateSchema = withParser(
  Schema.Literal('parent-action-ready', 'manual-required', 'unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPlatformLimitationFallbackRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProductClaimPlatformLimitationFallbackBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchemaVersionSchema,
  platformLimitationFallbackRowId: RefSchema,
  sourcePlatformPreclaimProofVersion: Schema.Literal(SourcePlatformPreclaimProofVersion),
  sourcePlatformPreclaimRowId: RefSchema,
  sourcePlatformPreclaimState: PlatformPreclaimStateSchema,
  sourceSafeParentWorkflowProofVersion: Schema.Literal(SourceSafeParentWorkflowProofVersion),
  sourceSafeParentWorkflowRowId: RefSchema,
  sourceSafeParentWorkflowState: SafeParentWorkflowStateSchema,
  sourcePlatformLimitationActionProofVersion: Schema.Literal(SourcePlatformLimitationActionProofVersion),
  sourcePlatformLimitationActionRowId: RefSchema,
  sourcePlatformLimitationActionState: PlatformLimitationActionStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  fallbackState: FallbackStateSchema,
  fallbackParentWorkflowRefs: Schema.Array(RefSchema),
  parentLimitationActionRef: RefSchema,
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredManualPlatformEvidenceRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredProviderStoreExecutionRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  limitationRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
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

type PlatformLimitationFallbackRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformLimitationFallbackRowIsHonest(row) ||
        'Expected product-claim platform limitation fallback rows to preserve source refs and keep product claims, portal UI, provider execution, platform adapters, delivery, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchemaVersionSchema,
  sourcePlatformPreclaimProofVersion: Schema.Literal(SourcePlatformPreclaimProofVersion),
  sourceSafeParentWorkflowProofVersion: Schema.Literal(SourceSafeParentWorkflowProofVersion),
  sourcePlatformLimitationActionProofVersion: Schema.Literal(SourcePlatformLimitationActionProofVersion),
  platformLimitationFallbackRows: Schema.Array(AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProductClaimPlatformLimitationFallbackProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformLimitationFallbackProofIsHonest(proof) ||
        'Expected product-claim platform limitation fallback proof to cover every store surface without approving product claims'
    )
  )
);

export const AppInstallPurchaseProductClaimPlatformLimitationFallbackKnownGaps = [
  'Platform limitation fallback rows consume platform preclaim, safe parent workflow, and limitation action rows but do not approve any product claim.',
  'Windows keeps fallback-parent-workflow-ready status for parent-visible manual review, while macOS remains manual-platform-limitation-fallback-required.',
  'Linux, Android, and iOS remain unsupported-platform-limitation-fallback-blocked until store/provider APIs, platform adapters, child delivery, and portal UI proof exist.',
] as const;

export const AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel =
  AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema.parse({
    schemaVersion: ProofVersion,
    sourcePlatformPreclaimProofVersion: SourcePlatformPreclaimProofVersion,
    sourceSafeParentWorkflowProofVersion: SourceSafeParentWorkflowProofVersion,
    sourcePlatformLimitationActionProofVersion: SourcePlatformLimitationActionProofVersion,
    platformLimitationFallbackRows:
      AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel.platformPreclaimRows.map(
        platformLimitationFallbackRow
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProductClaimPlatformLimitationFallbackKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProductClaimPlatformLimitationFallbackProof(
  proof: AppInstallPurchaseProductClaimPlatformLimitationFallbackProof
) {
  return {
    platformLimitationFallbackRows: proof.platformLimitationFallbackRows.length,
    fallbackParentWorkflowReadyRows: proof.platformLimitationFallbackRows.filter(
      (row) => row.fallbackState === 'fallback-parent-workflow-ready'
    ).length,
    manualPlatformLimitationFallbackRequiredRows: proof.platformLimitationFallbackRows.filter(
      (row) => row.fallbackState === 'manual-platform-limitation-fallback-required'
    ).length,
    unsupportedPlatformLimitationFallbackBlockedRows: proof.platformLimitationFallbackRows.filter(
      (row) => row.fallbackState === 'unsupported-platform-limitation-fallback-blocked'
    ).length,
    productClaimApprovedRows: proof.platformLimitationFallbackRows.filter(productClaimFallbackIsApproved).length,
    providerExecutedRows: proof.platformLimitationFallbackRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    platformAdapterImplementedRows: proof.platformLimitationFallbackRows.filter(
      (row) => row.platformAdapterClaim !== 'not-implemented'
    ).length,
  } as const;
}

function platformLimitationFallbackRow(
  preclaimRow: (typeof AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel.platformPreclaimRows)[number]
) {
  const workflowRow = matchingSafeParentWorkflowRow(preclaimRow.platform, preclaimRow.storeSurface);
  const limitationActionRow = matchingPlatformLimitationActionRow(preclaimRow.platform, preclaimRow.storeSurface);
  return {
    schemaVersion: ProofVersion,
    platformLimitationFallbackRowId: `app-install-product-claim-platform-limitation-fallback-${preclaimRow.platform}-${preclaimRow.storeSurface}`,
    sourcePlatformPreclaimProofVersion: SourcePlatformPreclaimProofVersion,
    sourcePlatformPreclaimRowId: preclaimRow.platformPreclaimRowId,
    sourcePlatformPreclaimState: preclaimRow.platformPreclaimState,
    sourceSafeParentWorkflowProofVersion: SourceSafeParentWorkflowProofVersion,
    sourceSafeParentWorkflowRowId: workflowRow.safeParentWorkflowRowId,
    sourceSafeParentWorkflowState: workflowRow.safeParentWorkflowState,
    sourcePlatformLimitationActionProofVersion: SourcePlatformLimitationActionProofVersion,
    sourcePlatformLimitationActionRowId: limitationActionRow.platformLimitationActionRowId,
    sourcePlatformLimitationActionState: limitationActionRow.platformLimitationActionState,
    platform: preclaimRow.platform,
    storeSurface: preclaimRow.storeSurface,
    fallbackState: platformLimitationFallbackState(
      preclaimRow.platformPreclaimState,
      workflowRow.safeParentWorkflowState,
      limitationActionRow.platformLimitationActionState
    ),
    fallbackParentWorkflowRefs: workflowRow.parentWorkflowRefs,
    parentLimitationActionRef: limitationActionRow.parentLimitationActionRef,
    requiredPortalTestRefs: uniqueRefs([preclaimRow.portalApprovalTestRef, preclaimRow.portalReportTestRef]),
    requiredManualPlatformEvidenceRefs: preclaimRow.requiredManualPlatformEvidenceRefs,
    requiredChildDeliveryRefs: uniqueRefs([
      ...preclaimRow.requiredChildDeliveryRefs,
      ...workflowRow.requiredChildDeliveryRefs,
    ]),
    requiredProviderStoreExecutionRefs: uniqueRefs([
      ...preclaimRow.requiredProviderStoreExecutionRefs,
      ...workflowRow.requiredProviderStoreExecutionRefs,
    ]),
    requiredPlatformAdapterRefs: uniqueRefs([
      ...preclaimRow.requiredPlatformAdapterRefs,
      ...workflowRow.requiredPlatformAdapterRefs,
    ]),
    limitationRefs: uniqueRefs([...preclaimRow.limitationRefs, ...workflowRow.limitationRefs]),
    auditEventRefs: uniqueRefs([
      ...preclaimRow.auditEventRefs,
      ...workflowRow.auditEventRefs,
      ...limitationActionRow.auditEventRefs,
    ]),
    reportRuntimeRefs: uniqueRefs([...preclaimRow.reportRuntimeRefs, ...workflowRow.reportRuntimeRefs]),
    productClaimApprovalClaim: 'not-claimed',
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
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
    claimBoundary: Boundary,
    evaluatedAt: UpdatedAt,
  } as const;
}

function matchingSafeParentWorkflowRow(platform: string, storeSurface: string) {
  const row = AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing safe parent workflow row for ${platform}:${storeSurface}`);
  }
  return row;
}

function matchingPlatformLimitationActionRow(platform: string, storeSurface: string) {
  const row = AppInstallPurchasePlatformLimitationActionProofReadModel.platformLimitationActionRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing platform limitation action row for ${platform}:${storeSurface}`);
  }
  return row;
}

function platformLimitationFallbackState(
  preclaimState: (typeof AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel.platformPreclaimRows)[number]['platformPreclaimState'],
  workflowState: (typeof AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows)[number]['safeParentWorkflowState'],
  limitationActionState: (typeof AppInstallPurchasePlatformLimitationActionProofReadModel.platformLimitationActionRows)[number]['platformLimitationActionState']
): (typeof FallbackStates)[number] {
  if (preclaimState === 'unsupported-platform-preclaim-blocked' || limitationActionState === 'unavailable') {
    return 'unsupported-platform-limitation-fallback-blocked';
  }
  if (workflowState === 'safe-parent-review-ready' && limitationActionState === 'parent-action-ready') {
    return 'fallback-parent-workflow-ready';
  }
  return 'manual-platform-limitation-fallback-required';
}

function platformLimitationFallbackRowIsHonest(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    row.sourcePlatformPreclaimProofVersion === SourcePlatformPreclaimProofVersion &&
    row.sourceSafeParentWorkflowProofVersion === SourceSafeParentWorkflowProofVersion &&
    row.sourcePlatformLimitationActionProofVersion === SourcePlatformLimitationActionProofVersion &&
    platformLimitationFallbackState(
      row.sourcePlatformPreclaimState,
      row.sourceSafeParentWorkflowState,
      row.sourcePlatformLimitationActionState
    ) === row.fallbackState &&
    platformLimitationFallbackRefsStayAttached(row) &&
    !productClaimFallbackIsApproved(row) &&
    platformLimitationFallbackClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function platformLimitationFallbackRefsStayAttached(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    platformLimitationFallbackSourceRefsStayAttached(row) &&
    platformLimitationFallbackRequiredRefsStayAttached(row) &&
    platformLimitationFallbackEvidenceRefsStayAttached(row)
  );
}

function platformLimitationFallbackSourceRefsStayAttached(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    row.sourcePlatformPreclaimRowId.length > 0 &&
    row.sourceSafeParentWorkflowRowId.length > 0 &&
    row.sourcePlatformLimitationActionRowId.length > 0 &&
    row.fallbackParentWorkflowRefs.length > 0 &&
    row.parentLimitationActionRef.length > 0
  );
}

function platformLimitationFallbackRequiredRefsStayAttached(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredManualPlatformEvidenceRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredProviderStoreExecutionRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0
  );
}

function platformLimitationFallbackEvidenceRefsStayAttached(row: PlatformLimitationFallbackRowCandidate): boolean {
  return row.limitationRefs.length > 0 && row.auditEventRefs.length > 0 && row.reportRuntimeRefs.length > 0;
}

function productClaimFallbackIsApproved(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim !== 'not-claimed' ||
    row.portalApprovalUiClaim !== 'not-claimed' ||
    row.portalReportUiClaim !== 'not-claimed' ||
    row.providerApiExecutionClaim !== 'not-executed' ||
    row.storeIntegrationClaim !== 'not-claimed' ||
    row.platformInterceptionClaim !== 'not-claimed' ||
    row.platformAdapterClaim !== 'not-implemented' ||
    row.childDeviceDeliveryClaim !== 'not-delivered' ||
    row.appBlockingClaim !== 'not-claimed'
  );
}

function platformLimitationFallbackClaimsStayUnimplemented(row: PlatformLimitationFallbackRowCandidate): boolean {
  return (
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function platformLimitationFallbackProofIsHonest(
  proof: AppInstallPurchaseProductClaimPlatformLimitationFallbackProof
): boolean {
  const keys = new Set(proof.platformLimitationFallbackRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.platformLimitationFallbackRows.map((row) => row.fallbackState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.platformLimitationFallbackRows.length === StoreSurfaces.length &&
    keys.size === proof.platformLimitationFallbackRows.length &&
    FallbackStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.platformLimitationFallbackRows.every(platformLimitationFallbackRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

