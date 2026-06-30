import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel } from './app-install-purchase-platform-adapter-evidence-gap-proof';
import { AppInstallPurchasePackageSourceAdapterExecutionProofReadModel } from './app-install-purchase-package-source-adapter-execution-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowGenerated,
  buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowGenerated,
  summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofGenerated,
  summarizeAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofGenerated,
  windowsPackageSourceAdapterEvidenceProofIsHonestGenerated,
  windowsPackageSourceAdapterEvidenceRowIsHonestGenerated,
  windowsPackageSourceRuntimeHandoffProofIsHonestGenerated,
  windowsPackageSourceRuntimeHandoffRowIsHonestGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const ProofVersion = 'app-install-purchase-windows-package-source-adapter-evidence';
const SourcePlatformAdapterEvidenceGapProofVersion = 'app-install-purchase-platform-adapter-evidence-gap-proof';
const SourcePackageSourceAdapterExecutionProofVersion = 'app-install-purchase-package-source-adapter-execution-proof';
const UpdatedAt = '2026-06-07T02:30:00.000Z';
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
const RuntimeHandoffStates = [
  'windows-runtime-handoff-ready',
  'windows-runtime-handoff-manual-required',
  'manual-runtime-handoff-required',
  'platform-unavailable',
  'blocked-before-claim',
] as const;
const RuntimeProbeStatuses = [
  'sanitized-command-available',
  'sanitized-command-unavailable',
  'manual-required',
  'unavailable',
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
const RuntimeHandoffStateSchema = withParser(Schema.Literal(...RuntimeHandoffStates));
const RuntimeProbeStatusSchema = withParser(Schema.Literal(...RuntimeProbeStatuses));
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
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseWindowsPackageSourceAdapterEvidenceBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const HostEvidenceArtifactSchema = Schema.Struct({
  artifactRef: RefSchema,
  hostPlatform: NonEmptyStringSchema,
  commandName: NonEmptyStringSchema,
  commandAvailable: Schema.Boolean,
  commandExitCode: Schema.Number,
  evidenceSummary: NonEmptyStringSchema,
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

const RuntimeHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceSchemaVersionSchema,
  runtimeHandoffRowId: RefSchema,
  sourceWindowsPackageSourceAdapterEvidenceRowId: RefSchema,
  sourceWindowsPackageSourceAdapterEvidenceState: HostEvidenceStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  runtimeHandoffState: RuntimeHandoffStateSchema,
  sanitizedCommandProbeStatus: RuntimeProbeStatusSchema,
  packageSourceEvidenceRefs: Schema.Array(RefSchema),
  requiredManualEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderStoreEvidenceRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  productionPlatformAdapterClaim: NotImplementedSchema,
  runtimeWriterExecutionClaim: NotExecutedSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type RuntimeHandoffRowCandidate = Infer<typeof RuntimeHandoffRowBaseSchema>;

export const AppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowSchema = withParser(
  RuntimeHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        windowsPackageSourceRuntimeHandoffRowIsHonest(row) ||
        'Expected Windows package-source runtime handoff rows to preserve source evidence refs, manual/unavailable states, and provider/store/platform/portal/delivery/blocking/custody non-claims'
    )
  )
);

const RuntimeHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceSchemaVersionSchema,
  sourceWindowsPackageSourceAdapterEvidenceProofVersion: Schema.Literal(ProofVersion),
  hostEvidenceArtifact: HostEvidenceArtifactSchema,
  runtimeHandoffRows: Schema.Array(AppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof = Infer<typeof RuntimeHandoffProofBaseSchema>;

export const AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema = withParser(
  RuntimeHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        windowsPackageSourceRuntimeHandoffProofIsHonest(proof) ||
        'Expected Windows package-source runtime handoff proof to cover every platform row, attach sanitized command/probe refs, and preserve required non-claims'
    )
  )
);

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

export const AppInstallPurchaseWindowsPackageSourceRuntimeHandoffKnownGaps = [
  'Windows runtime handoff rows expose sanitized command/probe status and package-source evidence refs only; they do not execute a runtime writer or deliver to a child device.',
  'macOS remains manual-runtime-handoff-required; Linux remains platform-unavailable; Android and iOS stay blocked-before-claim until platform adapter and child-device proof exists.',
  'Provider/store execution, portal approval/report UI, child-device delivery, app blocking, child activity data, and hosted custody remain unimplemented.',
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

export function buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(
  hostEvidenceArtifact: unknown
): AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof {
  const parsedHostEvidenceArtifact = HostEvidenceArtifactParser.parse(hostEvidenceArtifact);
  const sourceProof = buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(parsedHostEvidenceArtifact);
  return AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceWindowsPackageSourceAdapterEvidenceProofVersion: ProofVersion,
    hostEvidenceArtifact: parsedHostEvidenceArtifact,
    runtimeHandoffRows: sourceProof.windowsPackageSourceAdapterEvidenceRows.map((row) =>
      runtimeHandoffRow(row, parsedHostEvidenceArtifact)
    ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffKnownGaps,
    updatedAt: UpdatedAt,
  });
}

export const AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel =
  buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof({
    artifactRef: 'windows-package-source-runtime-handoff-manual-required',
    hostPlatform: 'not-collected-in-static-read-model',
    commandName: 'Get-AppxPackage',
    commandAvailable: false,
    commandExitCode: 1,
    evidenceSummary:
      'Static read model keeps Windows package-source runtime handoff manual until the proof harness records sanitized host command evidence.',
    collectedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(
  proof: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof
) {
  return summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofGenerated(proof);
}

export function summarizeAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(
  proof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
) {
  return summarizeAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofGenerated(proof);
}

function evidenceRow(
  gapRow: (typeof AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows)[number],
  hostEvidenceArtifact: HostEvidenceArtifact
) {
  const adapterExecutionRow = matchingAdapterExecutionRow(gapRow.platform);
  return buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowGenerated(
    gapRow,
    adapterExecutionRow,
    hostEvidenceArtifact,
    SourcePlatformAdapterEvidenceGapProofVersion,
    SourcePackageSourceAdapterExecutionProofVersion,
    Boundary,
    UpdatedAt
  );
}

function runtimeHandoffRow(
  sourceRow: (typeof AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel.windowsPackageSourceAdapterEvidenceRows)[number],
  hostEvidenceArtifact: HostEvidenceArtifact
) {
  return buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowGenerated(
    sourceRow,
    hostEvidenceArtifact,
    Boundary,
    UpdatedAt
  );
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
    windowsPackageSourceAdapterEvidenceRowIsHonestGenerated(row, BoundaryFragments)
  );
}

function windowsPackageSourceAdapterEvidenceProofIsHonest(
  proof: AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof
): boolean {
  return (
    proof.sourcePlatformAdapterEvidenceGapProofVersion === SourcePlatformAdapterEvidenceGapProofVersion &&
    proof.sourcePackageSourceAdapterExecutionProofVersion === SourcePackageSourceAdapterExecutionProofVersion &&
    windowsPackageSourceAdapterEvidenceProofIsHonestGenerated(proof, StoreSurfaces, NonClaims) &&
    proof.windowsPackageSourceAdapterEvidenceRows.every(windowsPackageSourceAdapterEvidenceRowIsHonest)
  );
}

function windowsPackageSourceRuntimeHandoffRowIsHonest(row: RuntimeHandoffRowCandidate): boolean {
  return windowsPackageSourceRuntimeHandoffRowIsHonestGenerated(row, BoundaryFragments);
}

function windowsPackageSourceRuntimeHandoffProofIsHonest(
  proof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
): boolean {
  return (
    proof.sourceWindowsPackageSourceAdapterEvidenceProofVersion === ProofVersion &&
    windowsPackageSourceRuntimeHandoffProofIsHonestGenerated(proof, StoreSurfaces, NonClaims) &&
    proof.runtimeHandoffRows.every(windowsPackageSourceRuntimeHandoffRowIsHonest)
  );
}
