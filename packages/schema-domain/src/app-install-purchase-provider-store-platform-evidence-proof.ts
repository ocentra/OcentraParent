import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel } from './app-install-purchase-provider-store-execution-preflight-proof';
import {
  AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel,
  AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema,
  type AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof,
} from './app-install-purchase-windows-package-source-adapter-evidence';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ProofVersion = 'app-install-purchase-provider-store-platform-evidence-proof';
const SourceProviderStoreExecutionPreflightProofVersion =
  'app-install-purchase-provider-store-execution-preflight-proof';
const SourceWindowsPackageSourceRuntimeHandoffProofVersion =
  'app-install-purchase-windows-package-source-adapter-evidence';
const UpdatedAt = '2026-06-07T13:45:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const ProviderStorePlatformEvidenceStates = [
  'manual-provider-store-platform-evidence-required',
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
const ProviderStorePreflightStates = [
  'preflight-ready',
  'manual-provider-proof-required',
  'provider-unavailable',
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
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'provider store platform evidence proof boundary only; links provider store execution preflight rows with Windows package-source runtime handoff rows and records exact missing provider store platform and child-device artifacts before any product claim no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no production platform adapter no runtime writer execution no runtime writer delivery no child-device delivery no runtime report delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'provider store platform evidence proof boundary only',
  'provider store execution preflight rows',
  'Windows package-source runtime handoff rows',
  'exact missing provider store platform and child-device artifacts',
  'before any product claim',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no production platform adapter',
  'no runtime writer execution',
  'no runtime writer delivery',
  'no child-device delivery',
  'no runtime report delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const ProviderStorePlatformEvidenceStateSchema = withParser(Schema.Literal(...ProviderStorePlatformEvidenceStates));
const RuntimeHandoffStateSchema = withParser(Schema.Literal(...RuntimeHandoffStates));
const ProviderStorePreflightStateSchema = withParser(Schema.Literal(...ProviderStorePreflightStates));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStorePlatformEvidenceRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStorePlatformEvidenceBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema,
  providerStorePlatformEvidenceRowId: RefSchema,
  sourceProviderStoreExecutionPreflightProofVersion: Schema.Literal(SourceProviderStoreExecutionPreflightProofVersion),
  sourceProviderStoreExecutionPreflightRowId: RefSchema,
  sourceProviderStoreExecutionPreflightState: ProviderStorePreflightStateSchema,
  sourceWindowsPackageSourceRuntimeHandoffProofVersion: Schema.Literal(
    SourceWindowsPackageSourceRuntimeHandoffProofVersion
  ),
  sourceWindowsPackageSourceRuntimeHandoffRowId: RefSchema,
  sourceRuntimeHandoffState: RuntimeHandoffStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  providerStorePlatformEvidenceState: ProviderStorePlatformEvidenceStateSchema,
  packageSourceEvidenceRefs: Schema.Array(RefSchema),
  providerStorePreflightRefs: Schema.Array(RefSchema),
  missingProviderStoreArtifactRefs: Schema.Array(RefSchema),
  missingPlatformArtifactRefs: Schema.Array(RefSchema),
  missingChildDeviceArtifactRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
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

type ProviderStorePlatformEvidenceRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProviderStorePlatformEvidenceRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStorePlatformEvidenceRowIsHonest(row) ||
        'Expected provider/store platform evidence rows to attach preflight and runtime-handoff refs, record exact missing artifacts, and preserve provider/store/platform/delivery/portal/blocking/custody non-claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStorePlatformEvidenceProofSchemaVersionSchema,
  sourceProviderStoreExecutionPreflightProofVersion: Schema.Literal(SourceProviderStoreExecutionPreflightProofVersion),
  sourceWindowsPackageSourceRuntimeHandoffProofVersion: Schema.Literal(
    SourceWindowsPackageSourceRuntimeHandoffProofVersion
  ),
  providerStorePlatformEvidenceRows: Schema.Array(AppInstallPurchaseProviderStorePlatformEvidenceRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStorePlatformEvidenceProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProviderStorePlatformEvidenceProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStorePlatformEvidenceProofIsHonest(proof) ||
        'Expected provider/store platform evidence proof to cover every platform/store row with manual, unavailable, and blocked states plus all non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStorePlatformEvidenceKnownGaps = [
  'Provider/store platform evidence rows are manual or blocked proof rows only; no Google Play Apple App Store Microsoft Store provider API execution or billing provider contact is implemented.',
  'Windows package-source runtime handoff can attach sanitized local package-source evidence, but Microsoft Store credential provider response platform adapter and child-device delivery artifacts remain missing.',
  'macOS remains manual-required, Linux is unavailable, and Android/iOS stay blocked-before-claim until store/provider credentials platform adapter evidence and child-device delivery proof exist.',
] as const;

export function buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
  runtimeHandoffProof: unknown
): AppInstallPurchaseProviderStorePlatformEvidenceProof {
  const parsedRuntimeHandoffProof =
    AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofSchema.parse(runtimeHandoffProof);
  return AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProviderStoreExecutionPreflightProofVersion: SourceProviderStoreExecutionPreflightProofVersion,
    sourceWindowsPackageSourceRuntimeHandoffProofVersion: SourceWindowsPackageSourceRuntimeHandoffProofVersion,
    providerStorePlatformEvidenceRows:
      AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows.map((row) =>
        providerStorePlatformEvidenceRow(row, parsedRuntimeHandoffProof)
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProviderStorePlatformEvidenceKnownGaps,
    updatedAt: UpdatedAt,
  });
}

export const AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel =
  buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
    AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel
  );

export function summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof(
  proof: AppInstallPurchaseProviderStorePlatformEvidenceProof
) {
  return {
    providerStorePlatformEvidenceRows: proof.providerStorePlatformEvidenceRows.length,
    manualRequiredRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'manual-provider-store-platform-evidence-required'
    ).length,
    platformUnavailableRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'blocked-before-claim'
    ).length,
    providerExecutedRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    platformAdapterImplementedRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.productionPlatformAdapterClaim !== 'not-implemented'
    ).length,
    childDeliveredRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function providerStorePlatformEvidenceRow(
  preflightRow: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number],
  runtimeHandoffProof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
) {
  const runtimeHandoffRow = matchingRuntimeHandoffRow(
    preflightRow.platform,
    preflightRow.storeSurface,
    runtimeHandoffProof
  );
  const missingArtifacts = missingArtifactRefs(preflightRow.platform);
  return {
    schemaVersion: ProofVersion,
    providerStorePlatformEvidenceRowId: `provider-store-platform-evidence-${preflightRow.platform}-${preflightRow.storeSurface}`,
    sourceProviderStoreExecutionPreflightProofVersion: SourceProviderStoreExecutionPreflightProofVersion,
    sourceProviderStoreExecutionPreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
    sourceProviderStoreExecutionPreflightState: preflightRow.providerStoreExecutionPreflightState,
    sourceWindowsPackageSourceRuntimeHandoffProofVersion: SourceWindowsPackageSourceRuntimeHandoffProofVersion,
    sourceWindowsPackageSourceRuntimeHandoffRowId: runtimeHandoffRow.runtimeHandoffRowId,
    sourceRuntimeHandoffState: runtimeHandoffRow.runtimeHandoffState,
    platform: preflightRow.platform,
    storeSurface: preflightRow.storeSurface,
    providerStorePlatformEvidenceState: providerStorePlatformEvidenceState(preflightRow, runtimeHandoffRow),
    packageSourceEvidenceRefs: runtimeHandoffRow.packageSourceEvidenceRefs,
    providerStorePreflightRefs: preflightRow.requiredProviderEvidenceRefs,
    missingProviderStoreArtifactRefs: missingArtifacts.providerStore,
    missingPlatformArtifactRefs: missingArtifacts.platform,
    missingChildDeviceArtifactRefs: missingArtifacts.childDevice,
    requiredPortalTestRefs: runtimeHandoffRow.requiredPortalTestRefs,
    blockerRefs: uniqueRefs([
      ...preflightRow.requiredProviderEvidenceRefs,
      ...runtimeHandoffRow.blockerRefs,
      ...missingArtifacts.providerStore,
      ...missingArtifacts.platform,
      ...missingArtifacts.childDevice,
    ]),
    auditEventRefs: uniqueRefs([...preflightRow.auditEventRefs, ...runtimeHandoffRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefs([...preflightRow.reportRuntimeRefs, ...runtimeHandoffRow.reportRuntimeRefs]),
    productClaimApprovalClaim: 'not-claimed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    productionPlatformAdapterClaim: 'not-implemented',
    runtimeWriterExecutionClaim: 'not-executed',
    runtimeWriterDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
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

function providerStorePlatformEvidenceState(
  preflightRow: (typeof AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows)[number],
  runtimeHandoffRow: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof['runtimeHandoffRows'][number]
): (typeof ProviderStorePlatformEvidenceStates)[number] {
  if (
    preflightRow.providerStoreExecutionPreflightState === 'provider-unavailable' ||
    runtimeHandoffRow.runtimeHandoffState === 'platform-unavailable'
  ) {
    return 'platform-unavailable';
  }
  if (runtimeHandoffRow.runtimeHandoffState === 'blocked-before-claim') {
    return 'blocked-before-claim';
  }
  return 'manual-provider-store-platform-evidence-required';
}

function matchingRuntimeHandoffRow(
  platform: string,
  storeSurface: string,
  runtimeHandoffProof: AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof
) {
  const row = runtimeHandoffProof.runtimeHandoffRows.find(
    (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`missing runtime handoff row for ${platform}:${storeSurface}`);
  }
  return row;
}

function missingArtifactRefs(platform: string) {
  const refs = {
    windows: {
      providerStore: [
        'missing-microsoft-store-provider-credential-proof',
        'missing-microsoft-store-provider-api-response-proof',
        'missing-billing-provider-contact-proof',
      ],
      platform: [
        'missing-windows-production-platform-adapter-execution-proof',
        'missing-windows-platform-interception-policy-proof',
      ],
      childDevice: ['missing-windows-child-device-delivery-receipt-proof'],
    },
    macos: {
      providerStore: [
        'missing-mac-app-store-credential-proof',
        'missing-mac-app-store-receipt-response-proof',
        'missing-billing-provider-contact-proof',
      ],
      platform: ['missing-macos-signing-receipt-entitlement-proof', 'missing-macos-platform-adapter-execution-proof'],
      childDevice: ['missing-macos-child-device-delivery-receipt-proof'],
    },
    linux: {
      providerStore: ['missing-linux-package-manager-provider-proof'],
      platform: ['missing-tested-linux-distro-package-manager-source-proof'],
      childDevice: ['missing-linux-child-device-delivery-receipt-proof'],
    },
    android: {
      providerStore: ['missing-google-play-api-policy-proof', 'missing-google-play-provider-response-proof'],
      platform: [
        'missing-android-device-owner-managed-profile-proof',
        'missing-android-platform-adapter-execution-proof',
      ],
      childDevice: ['missing-android-child-device-delivery-receipt-proof'],
    },
    ios: {
      providerStore: ['missing-apple-app-store-family-controls-evidence-proof', 'missing-apple-review-proof'],
      platform: ['missing-ios-family-controls-entitlement-proof', 'missing-ios-platform-adapter-execution-proof'],
      childDevice: ['missing-ios-child-device-delivery-receipt-proof'],
    },
  } as const;
  return refs[platform as keyof typeof refs];
}

function providerStorePlatformEvidenceRowIsHonest(row: ProviderStorePlatformEvidenceRowCandidate): boolean {
  return (
    row.sourceProviderStoreExecutionPreflightProofVersion === SourceProviderStoreExecutionPreflightProofVersion &&
    row.sourceWindowsPackageSourceRuntimeHandoffProofVersion === SourceWindowsPackageSourceRuntimeHandoffProofVersion &&
    providerStorePlatformEvidenceStateMatchesSources(row) &&
    providerStorePlatformEvidenceRefsAreComplete(row) &&
    providerStorePlatformEvidenceClaimsStayUnimplemented(row) &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function providerStorePlatformEvidenceStateMatchesSources(row: ProviderStorePlatformEvidenceRowCandidate): boolean {
  if (
    row.sourceProviderStoreExecutionPreflightState === 'provider-unavailable' ||
    row.sourceRuntimeHandoffState === 'platform-unavailable'
  ) {
    return row.providerStorePlatformEvidenceState === 'platform-unavailable';
  }
  if (row.sourceRuntimeHandoffState === 'blocked-before-claim') {
    return row.providerStorePlatformEvidenceState === 'blocked-before-claim';
  }
  return row.providerStorePlatformEvidenceState === 'manual-provider-store-platform-evidence-required';
}

function providerStorePlatformEvidenceRefsAreComplete(row: ProviderStorePlatformEvidenceRowCandidate): boolean {
  return (
    row.sourceProviderStoreExecutionPreflightRowId.length > 0 &&
    row.sourceWindowsPackageSourceRuntimeHandoffRowId.length > 0 &&
    row.packageSourceEvidenceRefs.length > 0 &&
    row.providerStorePreflightRefs.length > 0 &&
    row.missingProviderStoreArtifactRefs.length > 0 &&
    row.missingPlatformArtifactRefs.length > 0 &&
    row.missingChildDeviceArtifactRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function providerStorePlatformEvidenceClaimsStayUnimplemented(row: ProviderStorePlatformEvidenceRowCandidate): boolean {
  const nonClaimChecks = [
    row.productClaimApprovalClaim === 'not-claimed',
    row.googlePlayExecutionClaim === 'not-executed',
    row.appleAppStoreExecutionClaim === 'not-executed',
    row.microsoftStoreExecutionClaim === 'not-executed',
    row.billingProviderContactClaim === 'not-executed',
    row.providerApiExecutionClaim === 'not-executed',
    row.storeIntegrationClaim === 'not-claimed',
    row.platformInterceptionClaim === 'not-claimed',
    row.productionPlatformAdapterClaim === 'not-implemented',
    row.runtimeWriterExecutionClaim === 'not-executed',
    row.runtimeWriterDeliveryClaim === 'not-delivered',
    row.childDeviceDeliveryClaim === 'not-delivered',
    row.runtimeReportDeliveryClaim === 'not-delivered',
    row.portalApprovalUiClaim === 'not-claimed',
    row.portalReportUiClaim === 'not-claimed',
    row.appBlockingClaim === 'not-claimed',
    row.childDataCustody === 'no-child-activity-data',
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed',
  ];
  return nonClaimChecks.every(Boolean);
}

function providerStorePlatformEvidenceProofIsHonest(
  proof: AppInstallPurchaseProviderStorePlatformEvidenceProof
): boolean {
  const keys = new Set(proof.providerStorePlatformEvidenceRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.providerStorePlatformEvidenceRows.map((row) => row.providerStorePlatformEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreExecutionPreflightProofVersion === SourceProviderStoreExecutionPreflightProofVersion &&
    proof.sourceWindowsPackageSourceRuntimeHandoffProofVersion ===
      SourceWindowsPackageSourceRuntimeHandoffProofVersion &&
    proof.providerStorePlatformEvidenceRows.length === StoreSurfaces.length &&
    keys.size === proof.providerStorePlatformEvidenceRows.length &&
    ProviderStorePlatformEvidenceStates.every((state) => states.has(state)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.providerStorePlatformEvidenceRows.every(providerStorePlatformEvidenceRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

