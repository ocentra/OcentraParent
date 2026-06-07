import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel } from './app-install-purchase-platform-adapter-evidence-gap-proof';
import { AppInstallPurchasePackageSourceAdapterExecutionProofReadModel } from './app-install-purchase-package-source-adapter-execution-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const ProofVersion = 'app-install-purchase-windows-package-source-adapter-evidence';
const SourcePlatformAdapterEvidenceGapProofVersion = 'app-install-purchase-platform-adapter-evidence-gap-proof';
const SourcePackageSourceAdapterExecutionProofVersion = 'app-install-purchase-package-source-adapter-execution-proof';
const UpdatedAt = '2026-06-07T02:30:00.000Z';
const Text = Schema.String.pipe(Schema.minLength(1));
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const HostEvidenceStates = [
  'windows-host-evidence-collected',
  'windows-host-manual-required',
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
  'no-production-platform-adapter',
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
  'Windows package-source adapter evidence proof boundary only; records local Windows host command evidence for package-source inspection readiness while keeping provider store API execution, store integration, platform interception, production platform adapter implementation, product claim approval, child-device delivery, portal approval UI, portal report UI, app blocking, child activity data, and Ocentra-hosted family data custody unclaimed';
const BoundaryFragments = [
  'Windows package-source adapter evidence proof boundary only',
  'local Windows host command evidence',
  'package-source inspection readiness',
  'provider store API execution',
  'store integration',
  'platform interception',
  'production platform adapter implementation',
  'product claim approval',
  'child-device delivery',
  'portal approval UI',
  'portal report UI',
  'app blocking',
  'child activity data',
  'Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseWindowsPackageSourceAdapterEvidenceSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const HostEvidenceStateSchema = withParser(Schema.Literal(...HostEvidenceStates));
const SourceEvidenceGapStateSchema = withParser(
  Schema.Literal(
    'adapter-evidence-gap',
    'manual-adapter-evidence-required',
    'platform-unavailable',
    'blocked-before-claim'
  )
);
const SourceAdapterExecutionStateSchema = withParser(
  Schema.Literal(
    'local-adapter-executed',
    'manual-host-proof-required',
    'device-management-required',
    'apple-entitlement-required',
    'platform-unavailable'
  )
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = Text.pipe(Schema.brand('AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRef'));
const BoundarySchema = Text.pipe(Schema.brand('AppInstallPurchaseWindowsPackageSourceAdapterEvidenceBoundary'));
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const HostEvidenceArtifactSchema = Schema.Struct({
  artifactRef: RefSchema,
  hostPlatform: Text,
  commandName: Text,
  commandAvailable: Schema.Boolean,
  commandExitCode: Schema.Number,
  evidenceSummary: Text,
  collectedAt: ParentTimestampSchema,
});
const HostEvidenceArtifactParser = withParser(HostEvidenceArtifactSchema);

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceSchemaVersionSchema,
  windowsPackageSourceAdapterEvidenceRowId: RefSchema,
  sourcePlatformAdapterEvidenceGapProofVersion: Schema.Literal(SourcePlatformAdapterEvidenceGapProofVersion),
  sourcePlatformAdapterEvidenceGapRowId: RefSchema,
  sourcePlatformAdapterEvidenceGapState: SourceEvidenceGapStateSchema,
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  sourcePackageSourceAdapterExecutionRowId: RefSchema,
  sourcePackageSourceAdapterExecutionState: SourceAdapterExecutionStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  hostEvidenceState: HostEvidenceStateSchema,
  hostEvidenceArtifactRefs: Schema.Array(RefSchema),
  requiredManualEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderStoreEvidenceRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  productionPlatformAdapterClaim: NotImplementedSchema,
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

type EvidenceRowCandidate = Infer<typeof RowBaseSchema>;
type HostEvidenceArtifact = Infer<typeof HostEvidenceArtifactSchema>;

export const AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        windowsPackageSourceAdapterEvidenceRowIsHonest(row) ||
        'Expected Windows package-source adapter evidence rows to link source adapter gap and package-source adapter execution rows while preserving provider, store, platform, portal, delivery, blocking, and custody non-claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceSchemaVersionSchema,
  sourcePlatformAdapterEvidenceGapProofVersion: Schema.Literal(SourcePlatformAdapterEvidenceGapProofVersion),
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  hostEvidenceArtifact: HostEvidenceArtifactSchema,
  windowsPackageSourceAdapterEvidenceRows: Schema.Array(AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        windowsPackageSourceAdapterEvidenceProofIsHonest(proof) ||
        'Expected Windows package-source adapter evidence proof to cover every store surface, attach real or manual Windows host evidence state, and preserve required non-claims'
    )
  )
);

export const AppInstallPurchaseWindowsPackageSourceAdapterEvidenceKnownGaps = [
  'Windows host evidence is limited to local package-source command availability and sanitized command outcome; it is not Microsoft Store API execution or a production platform adapter.',
  'macOS remains manual-adapter-evidence-required; Linux remains platform-unavailable; Android and iOS stay blocked-before-claim until device-owner managed-profile or Apple entitlement proof exists.',
  'Provider/store API execution, store integration, portal approval/report UI, child-device delivery, app blocking, child activity data, and hosted custody remain unimplemented.',
] as const;

export function buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(
  hostEvidenceArtifact: unknown
): AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof {
  const parsedHostEvidenceArtifact = HostEvidenceArtifactParser.parse(hostEvidenceArtifact);
  return AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema.parse({
    schemaVersion: ProofVersion,
    sourcePlatformAdapterEvidenceGapProofVersion: SourcePlatformAdapterEvidenceGapProofVersion,
    sourcePackageSourceAdapterExecutionProofVersion: SourcePackageSourceAdapterExecutionProofVersion,
    hostEvidenceArtifact: parsedHostEvidenceArtifact,
    windowsPackageSourceAdapterEvidenceRows:
      AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows.map((row) =>
        evidenceRow(row, parsedHostEvidenceArtifact)
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceKnownGaps,
    updatedAt: UpdatedAt,
  });
}

export const AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel =
  buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof({
    artifactRef: 'windows-package-source-host-evidence-manual-required',
    hostPlatform: 'not-collected-in-static-read-model',
    commandName: 'Get-AppxPackage',
    commandAvailable: false,
    commandExitCode: 1,
    evidenceSummary:
      'Static read model keeps Windows host evidence manual until the proof harness records host command evidence.',
    collectedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(
  proof: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof
) {
  return {
    windowsPackageSourceAdapterEvidenceRows: proof.windowsPackageSourceAdapterEvidenceRows.length,
    windowsHostEvidenceCollectedRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'windows-host-evidence-collected'
    ).length,
    windowsHostManualRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'windows-host-manual-required'
    ).length,
    manualAdapterEvidenceRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'manual-adapter-evidence-required'
    ).length,
    platformUnavailableRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'blocked-before-claim'
    ).length,
    providerExecutedRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function evidenceRow(
  gapRow: (typeof AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows)[number],
  hostEvidenceArtifact: HostEvidenceArtifact
) {
  const adapterExecutionRow = matchingAdapterExecutionRow(gapRow.platform);
  return {
    schemaVersion: ProofVersion,
    windowsPackageSourceAdapterEvidenceRowId: `windows-package-source-adapter-evidence-${gapRow.platform}-${gapRow.storeSurface}`,
    sourcePlatformAdapterEvidenceGapProofVersion: SourcePlatformAdapterEvidenceGapProofVersion,
    sourcePlatformAdapterEvidenceGapRowId: gapRow.platformAdapterEvidenceGapRowId,
    sourcePlatformAdapterEvidenceGapState: gapRow.platformAdapterEvidenceGapState,
    sourcePackageSourceAdapterExecutionProofVersion: SourcePackageSourceAdapterExecutionProofVersion,
    sourcePackageSourceAdapterExecutionRowId: adapterExecutionRow.packageSourceAdapterExecutionRowId,
    sourcePackageSourceAdapterExecutionState: adapterExecutionRow.adapterExecutionState,
    platform: gapRow.platform,
    storeSurface: gapRow.storeSurface,
    hostEvidenceState: hostEvidenceState(gapRow, hostEvidenceArtifact),
    hostEvidenceArtifactRefs: hostEvidenceRefs(gapRow, hostEvidenceArtifact),
    requiredManualEvidenceRefs: uniqueRefs([...gapRow.requiredManualPlatformEvidenceRefs, ...gapRow.blockerRefs]),
    requiredProviderStoreEvidenceRefs: uniqueRefs([
      ...gapRow.providerStoreApiExecutionEvidenceRefs,
      ...gapRow.requiredProviderCredentialRefs,
    ]),
    requiredPortalTestRefs: gapRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: gapRow.requiredChildDeliveryRefs,
    blockerRefs: uniqueRefs([...gapRow.blockerRefs, ...adapterExecutionRow.requiredProofRefs]),
    auditEventRefs: uniqueRefs([...gapRow.auditEventRefs, ...adapterExecutionRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefs([...gapRow.reportRuntimeRefs, ...adapterExecutionRow.reportRefs]),
    productClaimApprovalClaim: 'not-claimed',
    microsoftStoreExecutionClaim: 'not-executed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    productionPlatformAdapterClaim: 'not-implemented',
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

function hostEvidenceState(
  gapRow: (typeof AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows)[number],
  hostEvidenceArtifact: HostEvidenceArtifact
): (typeof HostEvidenceStates)[number] {
  if (gapRow.platform === 'windows') {
    return hostEvidenceArtifact.commandAvailable ? 'windows-host-evidence-collected' : 'windows-host-manual-required';
  }
  if (gapRow.platformAdapterEvidenceGapState === 'platform-unavailable') {
    return 'platform-unavailable';
  }
  if (gapRow.platformAdapterEvidenceGapState === 'blocked-before-claim') {
    return 'blocked-before-claim';
  }
  return 'manual-adapter-evidence-required';
}

function hostEvidenceRefs(
  gapRow: (typeof AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows)[number],
  hostEvidenceArtifact: HostEvidenceArtifact
) {
  if (gapRow.platform === 'windows') {
    return [hostEvidenceArtifact.artifactRef];
  }
  return gapRow.requiredPlatformAdapterEvidenceRefs;
}

function matchingAdapterExecutionRow(platform: string) {
  const row = AppInstallPurchasePackageSourceAdapterExecutionProofReadModel.packageSourceAdapterExecutionRows.find(
    (candidate) => candidate.platform === platform
  );
  if (!row) {
    throw new Error(`missing package-source adapter execution row for ${platform}`);
  }
  return row;
}

function windowsPackageSourceAdapterEvidenceRowIsHonest(row: EvidenceRowCandidate): boolean {
  return (
    row.sourcePlatformAdapterEvidenceGapProofVersion === SourcePlatformAdapterEvidenceGapProofVersion &&
    row.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    hostEvidenceStateMatchesSource(row) &&
    evidenceRefsAreComplete(row) &&
    claimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function hostEvidenceStateMatchesSource(row: EvidenceRowCandidate): boolean {
  if (row.platform === 'windows') {
    return (
      row.hostEvidenceState === 'windows-host-evidence-collected' ||
      row.hostEvidenceState === 'windows-host-manual-required'
    );
  }
  if (row.sourcePlatformAdapterEvidenceGapState === 'platform-unavailable') {
    return row.hostEvidenceState === 'platform-unavailable';
  }
  if (row.sourcePlatformAdapterEvidenceGapState === 'blocked-before-claim') {
    return row.hostEvidenceState === 'blocked-before-claim';
  }
  return row.hostEvidenceState === 'manual-adapter-evidence-required';
}

function evidenceRefsAreComplete(row: EvidenceRowCandidate): boolean {
  return (
    row.sourcePlatformAdapterEvidenceGapRowId.length > 0 &&
    row.sourcePackageSourceAdapterExecutionRowId.length > 0 &&
    row.hostEvidenceArtifactRefs.length > 0 &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderStoreEvidenceRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function claimsStayUnimplemented(row: EvidenceRowCandidate): boolean {
  return (
    providerAndStoreClaimsStayUnimplemented(row) &&
    platformAndDeliveryClaimsStayUnimplemented(row) &&
    portalBlockingAndCustodyClaimsStayUnimplemented(row)
  );
}

function providerAndStoreClaimsStayUnimplemented(row: EvidenceRowCandidate): boolean {
  return (
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed'
  );
}

function platformAndDeliveryClaimsStayUnimplemented(row: EvidenceRowCandidate): boolean {
  return (
    row.platformInterceptionClaim === 'not-claimed' &&
    row.productionPlatformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered'
  );
}

function portalBlockingAndCustodyClaimsStayUnimplemented(row: EvidenceRowCandidate): boolean {
  return (
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function windowsPackageSourceAdapterEvidenceProofIsHonest(
  proof: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof
): boolean {
  const keys = new Set(
    proof.windowsPackageSourceAdapterEvidenceRows.map((row) => `${row.platform}:${row.storeSurface}`)
  );
  const states = new Set(proof.windowsPackageSourceAdapterEvidenceRows.map((row) => row.hostEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourcePlatformAdapterEvidenceGapProofVersion === SourcePlatformAdapterEvidenceGapProofVersion &&
    proof.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    proof.windowsPackageSourceAdapterEvidenceRows.length === StoreSurfaces.length &&
    keys.size === proof.windowsPackageSourceAdapterEvidenceRows.length &&
    states.has('windows-host-evidence-collected') !== states.has('windows-host-manual-required') &&
    states.has('manual-adapter-evidence-required') &&
    states.has('platform-unavailable') &&
    states.has('blocked-before-claim') &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.windowsPackageSourceAdapterEvidenceRows.every(windowsPackageSourceAdapterEvidenceRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
