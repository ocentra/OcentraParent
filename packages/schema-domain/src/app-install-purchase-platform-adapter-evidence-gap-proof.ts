import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { AppInstallPurchaseProviderStoreApiExecutionProofReadModel } from './app-install-purchase-provider-store-api-execution-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-platform-adapter-evidence-gap-proof';
const SourceProviderStoreApiExecutionProofVersion = 'app-install-purchase-provider-store-api-execution-proof';
const SourcePlatformProofReadinessProofVersion = 'app-install-purchase-platform-proof-readiness';
const UpdatedAt = '2026-06-07T00:36:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const PlatformAdapterEvidenceGapStates = [
  'adapter-evidence-gap',
  'manual-adapter-evidence-required',
  'platform-unavailable',
  'blocked-before-claim',
] as const;
const NonClaims = [
  'no-product-claim-approval',
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-real-platform-adapter-evidence-attached',
  'no-child-device-delivery',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'platform adapter evidence gap proof boundary only; links provider store API execution rows to per-platform adapter evidence requirements across Windows macOS Linux Android and iOS while keeping real adapter evidence separate from manual-required unavailable and blocked-before-claim states no product claim approval no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no real platform adapter evidence attached no child-device delivery no runtime writer delivery no runtime report delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'platform adapter evidence gap proof boundary only',
  'provider store API execution rows',
  'per-platform adapter evidence requirements',
  'Windows macOS Linux Android and iOS',
  'real adapter evidence separate',
  'manual-required unavailable and blocked-before-claim states',
  'no product claim approval',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no real platform adapter evidence attached',
  'no child-device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const PlatformAdapterEvidenceGapStateSchema = withParser(Schema.Literal(...PlatformAdapterEvidenceGapStates));
const SourceProviderStoreApiExecutionStateSchema = withParser(
  Schema.Literal('execution-ready', 'manual-required', 'unavailable', 'blocked-before-claim')
);
const SourcePlatformProofReadinessStateSchema = withParser(
  Schema.Literal('manual-proof-required', 'policy-blocked', 'unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformAdapterEvidenceGapRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformAdapterEvidenceGapBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const NoRealAdapterEvidenceSchema = withParser(Schema.Literal('no-real-adapter-evidence-attached'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema,
  platformAdapterEvidenceGapRowId: RefSchema,
  sourceProviderStoreApiExecutionProofVersion: Schema.Literal(SourceProviderStoreApiExecutionProofVersion),
  sourceProviderStoreApiExecutionRowId: RefSchema,
  sourceProviderStoreApiExecutionState: SourceProviderStoreApiExecutionStateSchema,
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  sourcePlatformProofReadinessState: SourcePlatformProofReadinessStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  platformAdapterEvidenceGapState: PlatformAdapterEvidenceGapStateSchema,
  providerStoreApiExecutionEvidenceRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterEvidenceRefs: Schema.Array(RefSchema),
  requiredManualPlatformEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderCredentialRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  realPlatformAdapterEvidenceState: NoRealAdapterEvidenceSchema,
  productClaimApprovalClaim: NotClaimedSchema,
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
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type PlatformAdapterEvidenceGapRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchasePlatformAdapterEvidenceGapRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformAdapterEvidenceGapRowIsHonest(row) ||
        'Expected platform adapter evidence gap rows to attach provider/store API source refs and keep adapter implementation, provider execution, delivery, portal, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema,
  sourceProviderStoreApiExecutionProofVersion: Schema.Literal(SourceProviderStoreApiExecutionProofVersion),
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  platformAdapterEvidenceGapRows: Schema.Array(AppInstallPurchasePlatformAdapterEvidenceGapRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformAdapterEvidenceGapProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformAdapterEvidenceGapProofIsHonest(proof) ||
        'Expected platform adapter evidence gap proof to cover every platform/store row and preserve adapter and product-claim non-claims'
    )
  )
);

export const AppInstallPurchasePlatformAdapterEvidenceGapKnownGaps = [
  'Platform adapter evidence gap rows consume provider/store API execution rows but do not attach real platform adapter evidence.',
  'Windows is an adapter-evidence-gap row because provider/store API execution proof is ready but a real Windows platform adapter proof is still missing.',
  'macOS remains manual-adapter-evidence-required; Linux is platform-unavailable; Android and iOS stay blocked-before-claim until device-owner managed-profile entitlement review platform adapter and child delivery evidence exist.',
] as const;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel =
  AppInstallPurchasePlatformAdapterEvidenceGapProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProviderStoreApiExecutionProofVersion: SourceProviderStoreApiExecutionProofVersion,
    sourcePlatformProofReadinessProofVersion: SourcePlatformProofReadinessProofVersion,
    platformAdapterEvidenceGapRows:
      AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows.map(
        platformAdapterEvidenceGapRow
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchasePlatformAdapterEvidenceGapKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchasePlatformAdapterEvidenceGapProof(
  proof: AppInstallPurchasePlatformAdapterEvidenceGapProof
) {
  return {
    platformAdapterEvidenceGapRows: proof.platformAdapterEvidenceGapRows.length,
    adapterEvidenceGapRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'adapter-evidence-gap'
    ).length,
    manualAdapterEvidenceRequiredRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'manual-adapter-evidence-required'
    ).length,
    platformUnavailableRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'blocked-before-claim'
    ).length,
    realAdapterEvidenceRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.realPlatformAdapterEvidenceState !== 'no-real-adapter-evidence-attached'
    ).length,
    adapterImplementedRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterClaim !== 'not-implemented'
    ).length,
    productClaimApprovedRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.productClaimApprovalClaim !== 'not-claimed'
    ).length,
  } as const;
}

function platformAdapterEvidenceGapRow(
  sourceRow: (typeof AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows)[number]
) {
  const platformReadinessRow = matchingPlatformProofReadinessRow(sourceRow.platform);
  return {
    schemaVersion: ProofVersion,
    platformAdapterEvidenceGapRowId: `app-install-platform-adapter-evidence-gap-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceProviderStoreApiExecutionProofVersion: SourceProviderStoreApiExecutionProofVersion,
    sourceProviderStoreApiExecutionRowId: sourceRow.providerStoreApiExecutionRowId,
    sourceProviderStoreApiExecutionState: sourceRow.providerStoreApiExecutionState,
    sourcePlatformProofReadinessProofVersion: SourcePlatformProofReadinessProofVersion,
    sourcePlatformProofReadinessState: platformReadinessRow.platformProofReadinessState,
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    platformAdapterEvidenceGapState: platformAdapterEvidenceGapState(sourceRow, platformReadinessRow),
    providerStoreApiExecutionEvidenceRefs: sourceRow.providerApiExecutionEvidenceRefs,
    requiredPlatformAdapterEvidenceRefs: platformAdapterEvidenceRefs(sourceRow, platformReadinessRow),
    requiredManualPlatformEvidenceRefs: uniqueRefs([
      ...sourceRow.manualPlatformEvidenceRefs,
      ...platformReadinessRow.requiredManualEvidenceRefs,
    ]),
    requiredProviderCredentialRefs: sourceRow.providerCredentialRequirementRefs,
    requiredPortalTestRefs: sourceRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: sourceRow.requiredChildDeliveryRefs,
    blockerRefs: uniqueRefs([...sourceRow.blockerRefs, ...sourceRow.requiredPlatformAdapterRefs]),
    auditEventRefs: sourceRow.auditEventRefs,
    reportRuntimeRefs: sourceRow.reportRuntimeRefs,
    realPlatformAdapterEvidenceState: 'no-real-adapter-evidence-attached',
    productClaimApprovalClaim: 'not-claimed',
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
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: Boundary,
    evaluatedAt: UpdatedAt,
  } as const;
}

function platformAdapterEvidenceGapState(
  sourceRow: (typeof AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows)[number],
  platformReadinessRow: (typeof AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows)[number]
): (typeof PlatformAdapterEvidenceGapStates)[number] {
  if (
    sourceRow.providerStoreApiExecutionState === 'unavailable' ||
    platformReadinessRow.platformProofReadinessState === 'unavailable'
  ) {
    return 'platform-unavailable';
  }
  if (
    sourceRow.providerStoreApiExecutionState === 'blocked-before-claim' ||
    platformReadinessRow.platformProofReadinessState === 'policy-blocked'
  ) {
    return 'blocked-before-claim';
  }
  if (sourceRow.providerStoreApiExecutionState === 'manual-required') {
    return 'manual-adapter-evidence-required';
  }
  return 'adapter-evidence-gap';
}

function platformAdapterEvidenceRefs(
  sourceRow: (typeof AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows)[number],
  platformReadinessRow: (typeof AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows)[number]
) {
  const refs = {
    windows: ['windows-app-install-adapter-manual-proof', 'windows-store-source-adapter-boundary-proof'],
    macos: ['macos-app-install-adapter-manual-proof', 'macos-receipt-signing-adapter-proof'],
    linux: ['linux-package-manager-source-adapter-proof'],
    android: ['android-device-owner-managed-profile-adapter-proof', 'google-play-policy-adapter-proof'],
    ios: ['ios-family-controls-adapter-entitlement-proof', 'apple-review-platform-adapter-proof'],
  } as const;
  return uniqueRefs([
    ...sourceRow.requiredPlatformAdapterRefs,
    ...platformReadinessRow.requiredManualEvidenceRefs,
    ...refs[sourceRow.platform],
  ]);
}

function matchingPlatformProofReadinessRow(platform: string) {
  const row = AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows.find(
    (candidate) => candidate.platform === platform
  );
  if (!row) {
    throw new Error(`missing platform proof readiness row for ${platform}`);
  }
  return row;
}

function platformAdapterEvidenceGapRowIsHonest(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.sourceProviderStoreApiExecutionProofVersion === SourceProviderStoreApiExecutionProofVersion &&
    row.sourcePlatformProofReadinessProofVersion === SourcePlatformProofReadinessProofVersion &&
    platformAdapterEvidenceGapStateMatchesSource(row) &&
    platformAdapterEvidenceRefsAreComplete(row) &&
    platformAdapterEvidenceClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function platformAdapterEvidenceGapStateMatchesSource(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  if (
    row.sourceProviderStoreApiExecutionState === 'unavailable' ||
    row.sourcePlatformProofReadinessState === 'unavailable'
  ) {
    return row.platformAdapterEvidenceGapState === 'platform-unavailable';
  }
  if (
    row.sourceProviderStoreApiExecutionState === 'blocked-before-claim' ||
    row.sourcePlatformProofReadinessState === 'policy-blocked'
  ) {
    return row.platformAdapterEvidenceGapState === 'blocked-before-claim';
  }
  if (row.sourceProviderStoreApiExecutionState === 'manual-required') {
    return row.platformAdapterEvidenceGapState === 'manual-adapter-evidence-required';
  }
  return row.platformAdapterEvidenceGapState === 'adapter-evidence-gap';
}

function platformAdapterEvidenceRefsAreComplete(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.sourceProviderStoreApiExecutionRowId.length > 0 &&
    row.sourcePlatformProofReadinessProofVersion === SourcePlatformProofReadinessProofVersion &&
    row.providerStoreApiExecutionEvidenceRefs.length > 0 &&
    row.requiredPlatformAdapterEvidenceRefs.length > 0 &&
    row.requiredManualPlatformEvidenceRefs.length > 0 &&
    row.requiredProviderCredentialRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function platformAdapterEvidenceClaimsStayUnimplemented(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.realPlatformAdapterEvidenceState === 'no-real-adapter-evidence-attached' &&
    platformAdapterEvidenceProviderClaimsStayUnimplemented(row) &&
    platformAdapterEvidencePlatformClaimsStayUnimplemented(row) &&
    platformAdapterEvidenceDeliveryClaimsStayUnimplemented(row) &&
    platformAdapterEvidencePortalAndCustodyClaimsStayUnimplemented(row)
  );
}

function platformAdapterEvidenceProviderClaimsStayUnimplemented(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed'
  );
}

function platformAdapterEvidencePlatformClaimsStayUnimplemented(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return row.platformInterceptionClaim === 'not-claimed' && row.platformAdapterClaim === 'not-implemented';
}

function platformAdapterEvidenceDeliveryClaimsStayUnimplemented(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed'
  );
}

function platformAdapterEvidencePortalAndCustodyClaimsStayUnimplemented(
  row: PlatformAdapterEvidenceGapRowCandidate
): boolean {
  return (
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function platformAdapterEvidenceGapProofIsHonest(proof: AppInstallPurchasePlatformAdapterEvidenceGapProof): boolean {
  const keys = new Set(proof.platformAdapterEvidenceGapRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.platformAdapterEvidenceGapRows.map((row) => row.platformAdapterEvidenceGapState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.platformAdapterEvidenceGapRows.length === StoreSurfaces.length &&
    keys.size === proof.platformAdapterEvidenceGapRows.length &&
    PlatformAdapterEvidenceGapStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.platformAdapterEvidenceGapRows.every(platformAdapterEvidenceGapRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
