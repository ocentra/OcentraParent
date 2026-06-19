import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const SchemaVersion = 'app-game-platform-extension-proof-pack-readiness';
const GeneratedAt = '2026-06-05T20:47:00.000Z';
const RequiredProductMeanings = ['native-app', 'native-game'] as const;
const RequiredNonClaims = [
  'no-live-platform-adapter',
  'no-adapter-dispatch',
  'no-broad-installed-app-blocking',
  'no-mobile-privileged-control',
  'no-store-or-mdm-provider-execution',
  'no-child-device-delivery',
] as const;

const PlatformRows = [
  {
    platform: 'macos',
    checklistRowIds: ['MAC-01', 'MAC-02', 'MAC-03', 'MAC-04', 'MAC-05', 'MAC-06', 'MAC-07', 'MAC-08'],
    authorityTier: 'user-permission-and-mdm-required',
    setupState: 'manual-required',
    proofPackState: 'manual-proof-pack-required',
    requiredProofRefs: [
      'launchservices-inventory-artifact',
      'nsworkspace-runtime-artifact',
      'accessibility-foreground-artifact',
      'endpoint-security-or-mdm-enforcement-artifact',
    ],
  },
  {
    platform: 'ios',
    checklistRowIds: ['IOS-01', 'IOS-02', 'IOS-03', 'IOS-04', 'IOS-05', 'IOS-06', 'IOS-07', 'IOS-08', 'IOS-09'],
    authorityTier: 'apple-entitlement-and-supervision-required',
    setupState: 'manual-required',
    proofPackState: 'privileged-mobile-proof-required',
    requiredProofRefs: [
      'familycontrols-entitlement-artifact',
      'deviceactivity-token-artifact',
      'managedsettings-shield-artifact',
      'supervised-mdm-restriction-artifact',
    ],
  },
  {
    platform: 'android',
    checklistRowIds: ['ANDROID-01', 'ANDROID-02', 'ANDROID-03', 'ANDROID-04', 'ANDROID-06', 'ANDROID-07', 'ANDROID-08'],
    authorityTier: 'usage-access-and-device-owner-required',
    setupState: 'manual-required',
    proofPackState: 'privileged-mobile-proof-required',
    requiredProofRefs: [
      'package-visibility-artifact',
      'usagestats-usageevents-artifact',
      'accessibility-overlay-artifact',
      'device-owner-or-profile-owner-artifact',
    ],
  },
  {
    platform: 'linux',
    checklistRowIds: ['LINUX-01', 'LINUX-02', 'LINUX-03', 'LINUX-04', 'LINUX-05', 'LINUX-06', 'LINUX-07', 'LINUX-08'],
    authorityTier: 'desktop-environment-and-system-policy-dependent',
    setupState: 'manual-required',
    proofPackState: 'manual-proof-pack-required',
    requiredProofRefs: [
      'desktop-entry-inventory-artifact',
      'package-manager-inventory-artifact',
      'procfs-runtime-artifact',
      'x11-wayland-foreground-artifact',
    ],
  },
] as const;

export const AppGamePlatformExtensionProofPackReadinessSchemaVersionSchema = withParser(Schema.Literal(SchemaVersion));
export const AppGamePlatformExtensionProductMeaningSchema = withParser(Schema.Literal(...RequiredProductMeanings));
export const AppGamePlatformExtensionSetupStateSchema = withParser(
  Schema.Literal('manual-required', 'unavailable', 'not-claimed')
);
export const AppGamePlatformExtensionProofPackStateSchema = withParser(
  Schema.Literal('manual-proof-pack-required', 'privileged-mobile-proof-required', 'unavailable', 'not-claimed')
);
export const AppGamePlatformExtensionAdapterExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
export const AppGamePlatformExtensionNonClaimSchema = withParser(Schema.Literal(...RequiredNonClaims));

const PlatformExtensionRowIdSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionProofPackReadinessRowId');
const PlatformExtensionRefSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionProofPackReadinessRef');
const PlatformExtensionClaimBoundarySchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionProofPackReadinessClaimBoundary');

const PlatformExtensionReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppGamePlatformExtensionProofPackReadinessSchemaVersionSchema,
  rowId: PlatformExtensionRowIdSchema,
  platform: ParentPlatformSchema,
  productMeanings: Schema.Array(AppGamePlatformExtensionProductMeaningSchema),
  checklistRowIds: Schema.Array(PlatformExtensionRefSchema),
  authorityTier: PlatformExtensionRefSchema,
  setupState: AppGamePlatformExtensionSetupStateSchema,
  proofPackState: AppGamePlatformExtensionProofPackStateSchema,
  requiredProofRefs: Schema.Array(PlatformExtensionRefSchema),
  adapterExecutionClaim: AppGamePlatformExtensionAdapterExecutionClaimSchema,
  broadBlockingClaimed: Schema.Boolean,
  privilegedMobileClaimed: Schema.Boolean,
  storeOrMdmProviderExecutionClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  claimBoundary: PlatformExtensionClaimBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type PlatformExtensionReadinessRowCandidate = Infer<typeof PlatformExtensionReadinessRowBaseSchema>;

export const AppGamePlatformExtensionProofPackReadinessRowSchema = withParser(
  PlatformExtensionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformExtensionReadinessRowIsHonest(row) ||
        'Expected app/game platform extension rows to preserve native app/game meaning while keeping adapter, broad blocking, privileged mobile, provider, and child delivery claims false'
    )
  )
);

const PlatformExtensionReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGamePlatformExtensionProofPackReadinessSchemaVersionSchema,
  readModelId: PlatformExtensionRefSchema,
  sourceChecklistRefs: Schema.Array(PlatformExtensionRefSchema),
  rows: Schema.Array(AppGamePlatformExtensionProofPackReadinessRowSchema),
  nonClaims: Schema.Array(AppGamePlatformExtensionNonClaimSchema),
  knownGaps: Schema.Array(PlatformExtensionRefSchema),
  generatedAt: ParentTimestampSchema,
});

export type AppGamePlatformExtensionProofPackReadinessRow = Infer<
  typeof AppGamePlatformExtensionProofPackReadinessRowSchema
>;
export type AppGamePlatformExtensionProofPackReadinessReadModel = Infer<typeof PlatformExtensionReadModelBaseSchema>;

export const AppGamePlatformExtensionProofPackReadinessReadModelSchema = withParser(
  PlatformExtensionReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        platformExtensionReadinessReadModelIsComplete(readModel) ||
        'Expected app/game platform extension proof-pack readiness to cover macOS, iOS, Android, and Linux once with non-claims preserved'
    )
  )
);

export const AppGamePlatformExtensionProofPackKnownGaps = [
  'Platform proof-pack rows are readiness contracts only; no macOS, iOS, Android, or Linux adapter executes.',
  'Native app and native game meanings are kept separate on top of the shared app/game evidence spine.',
  'Broad installed-app blocking, privileged mobile controls, store or MDM provider execution, and child-device delivery remain unclaimed.',
] as const;

export const AppGamePlatformExtensionProofPackReadinessReadModel =
  AppGamePlatformExtensionProofPackReadinessReadModelSchema.parse({
    schemaVersion: SchemaVersion,
    readModelId: 'app-game-platform-extension-proof-pack-readiness',
    sourceChecklistRefs: [
      'docs/plans/app-plan/implementation-checklist.md',
      'docs/plans/app-game-plan/implementation-checklist.md',
    ],
    rows: PlatformRows.map(platformExtensionReadinessRow),
    nonClaims: RequiredNonClaims,
    knownGaps: AppGamePlatformExtensionProofPackKnownGaps,
    generatedAt: GeneratedAt,
  });

export function summarizeAppGamePlatformExtensionProofPackReadiness(
  readModel: AppGamePlatformExtensionProofPackReadinessReadModel
) {
  return {
    rows: readModel.rows.length,
    platforms: new Set(readModel.rows.map((row) => row.platform)).size,
    nativeAppRows: readModel.rows.filter((row) => row.productMeanings.includes('native-app')).length,
    nativeGameRows: readModel.rows.filter((row) => row.productMeanings.includes('native-game')).length,
    manualRequiredRows: readModel.rows.filter((row) => row.setupState === 'manual-required').length,
    adapterExecutedRows: readModel.rows.filter((row) => row.adapterExecutionClaim !== 'not-executed').length,
    broadBlockingClaimedRows: readModel.rows.filter((row) => row.broadBlockingClaimed).length,
    privilegedMobileClaimedRows: readModel.rows.filter((row) => row.privilegedMobileClaimed).length,
  } as const;
}

function platformExtensionReadinessRow(row: (typeof PlatformRows)[number]) {
  return {
    schemaVersion: SchemaVersion,
    rowId: `app-game-platform-extension-${row.platform}`,
    platform: row.platform,
    productMeanings: RequiredProductMeanings,
    checklistRowIds: row.checklistRowIds,
    authorityTier: row.authorityTier,
    setupState: row.setupState,
    proofPackState: row.proofPackState,
    requiredProofRefs: row.requiredProofRefs,
    adapterExecutionClaim: 'not-executed',
    broadBlockingClaimed: false,
    privilegedMobileClaimed: false,
    storeOrMdmProviderExecutionClaimed: false,
    childDeviceDeliveryClaimed: false,
    claimBoundary: `readiness proof-pack row for ${row.platform}; no adapter dispatch, broad installed-app blocking, privileged mobile control, provider execution, or child-device delivery claimed`,
    evaluatedAt: GeneratedAt,
  };
}

function platformExtensionReadinessRowIsHonest(row: PlatformExtensionReadinessRowCandidate): boolean {
  return (
    row.productMeanings.includes('native-app') &&
    row.productMeanings.includes('native-game') &&
    row.checklistRowIds.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    row.adapterExecutionClaim === 'not-executed' &&
    !row.broadBlockingClaimed &&
    !row.privilegedMobileClaimed &&
    !row.storeOrMdmProviderExecutionClaimed &&
    !row.childDeviceDeliveryClaimed
  );
}

function platformExtensionReadinessReadModelIsComplete(
  readModel: AppGamePlatformExtensionProofPackReadinessReadModel
): boolean {
  const platforms = new Set(readModel.rows.map((row) => row.platform));
  return (
    readModel.rows.length === PlatformRows.length &&
    platforms.has('macos') &&
    platforms.has('ios') &&
    platforms.has('android') &&
    platforms.has('linux') &&
    RequiredNonClaims.every((nonClaim) => readModel.nonClaims.includes(nonClaim))
  );
}

