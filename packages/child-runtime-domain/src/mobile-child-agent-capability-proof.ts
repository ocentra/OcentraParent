import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from '@ocentra-parent/capability-domain/capabilities';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const MobileChildAgentCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal('mobile-child-agent-capability-proof')
);
export const MobileChildAgentPlatformSchema = withParser(Schema.Literal('android-child-agent', 'ios-child-agent'));
export const MobileChildAgentReadinessSchema = withParser(
  Schema.Literal('package-scaffold-only', 'manual-device-proof-required', 'entitlement-review-required')
);
export const MobileChildAgentCapabilityProofStateSchema = withParser(
  Schema.Literal(
    'ci-mechanical-proof',
    'package-local-scaffold',
    'simulator-scaffold',
    'manual-required',
    'settings-grant-required',
    'entitlement-required',
    'signing-required',
    'device-proof-required',
    'blocked',
    'not-implemented',
    'planned'
  )
);
export const MobileChildAgentCapabilitySurfaceSchema = withParser(
  Schema.Literal(
    'android-foreground-service',
    'android-storage-protocol-bridge',
    'android-typed-protocol-bridge',
    'android-notifications',
    'android-usage-stats',
    'android-accessibility-service',
    'android-vpn-dns',
    'android-device-owner',
    'android-managed-profile',
    'android-device-proof',
    'android-play-signing',
    'android-external-transport',
    'ios-simulator-status-surface',
    'ios-family-controls',
    'ios-device-activity',
    'ios-screen-time',
    'ios-network-extension',
    'ios-notifications',
    'ios-background-execution',
    'ios-signing',
    'ios-testflight',
    'ios-device-proof',
    'ios-app-store',
    'ios-external-transport'
  )
);
export const MobileChildAgentCapabilitySourceSchema = withParser(
  Schema.Literal(
    'child-android-protocol-package-lifecycle-proof',
    'child-android-storage-protocol-capability-proof',
    'child-android-service-protocol-capability-proof',
    'child-android-permission-capability-proof',
    'child-android-privileged-capability-proof',
    'child-android-device-proof-artifact-gate',
    'child-ios-entitlement-capability-proof'
  )
);
export const MobileChildAgentPackageRuntimeHookSchema = withParser(
  Schema.Literal(
    'android-debug-apk-checksum',
    'android-package-local-status',
    'android-device-install',
    'android-play-signing',
    'ios-xcode-target',
    'ios-simulator-status',
    'ios-signing-profile',
    'ios-testflight-device'
  )
);

export const MobileChildAgentCapabilityProofPathSchema = brandedNonEmptyStringSchema(
  'MobileChildAgentCapabilityProofPath'
);
export const MobileChildAgentCapabilityProofCommandSchema = brandedNonEmptyStringSchema(
  'MobileChildAgentCapabilityProofCommand'
);
export const MobileChildAgentCapabilityProofBoundarySchema = brandedNonEmptyStringSchema(
  'MobileChildAgentCapabilityProofBoundary'
);

export const MobileChildAgentPlatformSummarySchema = withParser(
  Schema.Struct({
    platform: MobileChildAgentPlatformSchema,
    childAgentReadiness: MobileChildAgentReadinessSchema,
    packageRuntimeState: MobileChildAgentCapabilityProofStateSchema,
    privilegedOsState: MobileChildAgentCapabilityProofStateSchema,
    externalTransportState: MobileChildAgentCapabilityProofStateSchema,
    reviewerSummary: MobileChildAgentCapabilityProofBoundarySchema,
  })
);

export const MobileChildAgentCapabilitySourceProofSchema = withParser(
  Schema.Struct({
    source: MobileChildAgentCapabilitySourceSchema,
    status: MobileChildAgentCapabilityProofStateSchema,
    command: MobileChildAgentCapabilityProofCommandSchema,
    outputPath: Schema.Union(MobileChildAgentCapabilityProofPathSchema, Schema.Null),
  })
);

export const MobileChildAgentCapabilityRowSchema = withParser(
  Schema.Struct({
    surface: MobileChildAgentCapabilitySurfaceSchema,
    platform: MobileChildAgentPlatformSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    proofState: MobileChildAgentCapabilityProofStateSchema,
    source: MobileChildAgentCapabilitySourceSchema,
    proofRequirement: MobileChildAgentCapabilityProofBoundarySchema,
    claimBoundary: MobileChildAgentCapabilityProofBoundarySchema,
  })
);

export const MobileChildAgentPackageRuntimeHookProofSchema = withParser(
  Schema.Struct({
    hook: MobileChildAgentPackageRuntimeHookSchema,
    platform: MobileChildAgentPlatformSchema,
    hookState: MobileChildAgentCapabilityProofStateSchema,
    evidencePath: Schema.Union(MobileChildAgentCapabilityProofPathSchema, Schema.Null),
    source: MobileChildAgentCapabilitySourceSchema,
  })
);

export const MobileChildAgentCapabilityClaimBoundariesSchema = withParser(
  Schema.Struct({
    parentMobileScope: Schema.Literal('separate-parent-mobile-workstream'),
    childAndroidParity: Schema.Literal('not-claimed'),
    childIosParity: Schema.Literal('not-claimed'),
    privilegedOsBehavior: Schema.Literal('not-claimed'),
    externalChildAgentTransport: Schema.Literal('not-claimed'),
    storeDistribution: Schema.Literal('not-claimed'),
    reviewerSummary: MobileChildAgentCapabilityProofBoundarySchema,
  })
);

const MobileChildAgentCapabilityReadModelBaseSchema = Schema.Struct({
  schemaVersion: MobileChildAgentCapabilityProofSchemaVersionSchema,
  checkedAt: ParentTimestampSchema,
  platforms: Schema.Array(MobileChildAgentPlatformSummarySchema),
  sourceProofs: Schema.Array(MobileChildAgentCapabilitySourceProofSchema),
  capabilityRows: Schema.Array(MobileChildAgentCapabilityRowSchema),
  packageRuntimeHooks: Schema.Array(MobileChildAgentPackageRuntimeHookProofSchema),
  claimBoundaries: MobileChildAgentCapabilityClaimBoundariesSchema,
  knownManualGaps: Schema.Array(MobileChildAgentCapabilityProofBoundarySchema),
});

type MobileChildAgentCapabilityReadModelCandidate = Infer<typeof MobileChildAgentCapabilityReadModelBaseSchema>;

export const MobileChildAgentCapabilityReadModelSchema = withParser(
  MobileChildAgentCapabilityReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        mobileChildAgentCapabilityReadModelIsHonest(readModel) ||
        'Expected mobile child-agent capability proof to keep Android/iOS privileged, entitlement, device, signing, store, and external transport claims manual-required, planned, blocked, or not-implemented until device artifacts exist'
    )
  )
);

export type MobileChildAgentPlatform = Infer<typeof MobileChildAgentPlatformSchema>;
export type MobileChildAgentReadiness = Infer<typeof MobileChildAgentReadinessSchema>;
export type MobileChildAgentCapabilityProofState = Infer<typeof MobileChildAgentCapabilityProofStateSchema>;
export type MobileChildAgentCapabilitySurface = Infer<typeof MobileChildAgentCapabilitySurfaceSchema>;
export type MobileChildAgentCapabilitySource = Infer<typeof MobileChildAgentCapabilitySourceSchema>;
export type MobileChildAgentPackageRuntimeHook = Infer<typeof MobileChildAgentPackageRuntimeHookSchema>;
export type MobileChildAgentCapabilityProofPath = Infer<typeof MobileChildAgentCapabilityProofPathSchema>;
export type MobileChildAgentCapabilityProofCommand = Infer<typeof MobileChildAgentCapabilityProofCommandSchema>;
export type MobileChildAgentCapabilityProofBoundary = Infer<typeof MobileChildAgentCapabilityProofBoundarySchema>;
export type MobileChildAgentPlatformSummary = Infer<typeof MobileChildAgentPlatformSummarySchema>;
export type MobileChildAgentCapabilitySourceProof = Infer<typeof MobileChildAgentCapabilitySourceProofSchema>;
export type MobileChildAgentCapabilityRow = Infer<typeof MobileChildAgentCapabilityRowSchema>;
export type MobileChildAgentPackageRuntimeHookProof = Infer<typeof MobileChildAgentPackageRuntimeHookProofSchema>;
export type MobileChildAgentCapabilityClaimBoundaries = Infer<typeof MobileChildAgentCapabilityClaimBoundariesSchema>;
export type MobileChildAgentCapabilityReadModel = Infer<typeof MobileChildAgentCapabilityReadModelSchema>;

const RequiredSourceProofs = [
  'child-android-protocol-package-lifecycle-proof',
  'child-android-storage-protocol-capability-proof',
  'child-android-service-protocol-capability-proof',
  'child-android-permission-capability-proof',
  'child-android-privileged-capability-proof',
  'child-android-device-proof-artifact-gate',
  'child-ios-entitlement-capability-proof',
] as const satisfies ReadonlyArray<MobileChildAgentCapabilitySource>;

const RequiredSurfaces = [
  'android-foreground-service',
  'android-storage-protocol-bridge',
  'android-typed-protocol-bridge',
  'android-notifications',
  'android-usage-stats',
  'android-accessibility-service',
  'android-vpn-dns',
  'android-device-owner',
  'android-managed-profile',
  'android-device-proof',
  'android-play-signing',
  'android-external-transport',
  'ios-simulator-status-surface',
  'ios-family-controls',
  'ios-device-activity',
  'ios-screen-time',
  'ios-network-extension',
  'ios-notifications',
  'ios-background-execution',
  'ios-signing',
  'ios-testflight',
  'ios-device-proof',
  'ios-app-store',
  'ios-external-transport',
] as const satisfies ReadonlyArray<MobileChildAgentCapabilitySurface>;

const RowExpectations = {
  'android-foreground-service': androidRow(
    'foreground-mobile-service',
    'manual-required',
    'device-proof-required',
    'child-android-service-protocol-capability-proof'
  ),
  'android-storage-protocol-bridge': androidRow(
    'local-storage',
    'scaffold',
    'package-local-scaffold',
    'child-android-storage-protocol-capability-proof'
  ),
  'android-typed-protocol-bridge': androidRow(
    'typed-protocol-bridge',
    'scaffold',
    'package-local-scaffold',
    'child-android-storage-protocol-capability-proof'
  ),
  'android-notifications': androidRow(
    'notifications',
    'manual-required',
    'manual-required',
    'child-android-permission-capability-proof'
  ),
  'android-usage-stats': androidRow(
    'usage-stats',
    'manual-required',
    'settings-grant-required',
    'child-android-privileged-capability-proof'
  ),
  'android-accessibility-service': androidRow(
    'accessibility-service',
    'not-implemented',
    'not-implemented',
    'child-android-privileged-capability-proof'
  ),
  'android-vpn-dns': androidRow(
    'vpn-dns-filtering',
    'not-implemented',
    'not-implemented',
    'child-android-privileged-capability-proof'
  ),
  'android-device-owner': androidRow(
    'device-owner-policy',
    'manual-required',
    'blocked',
    'child-android-privileged-capability-proof'
  ),
  'android-managed-profile': androidRow(
    'managed-profile',
    'manual-required',
    'blocked',
    'child-android-privileged-capability-proof'
  ),
  'android-device-proof': androidRow(
    'package-lifecycle',
    'manual-required',
    'device-proof-required',
    'child-android-device-proof-artifact-gate'
  ),
  'android-play-signing': androidRow(
    'store-distribution',
    'planned',
    'planned',
    'child-android-device-proof-artifact-gate'
  ),
  'android-external-transport': androidRow(
    'typed-protocol-bridge',
    'not-implemented',
    'not-implemented',
    'child-android-device-proof-artifact-gate'
  ),
  'ios-simulator-status-surface': iosRow(
    'typed-protocol-bridge',
    'scaffold',
    'simulator-scaffold',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-family-controls': iosRow(
    'family-controls-entitlement',
    'manual-required',
    'entitlement-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-device-activity': iosRow(
    'device-activity',
    'manual-required',
    'entitlement-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-screen-time': iosRow(
    'screen-time-api',
    'manual-required',
    'entitlement-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-network-extension': iosRow(
    'network-extension',
    'manual-required',
    'entitlement-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-notifications': iosRow(
    'notifications',
    'manual-required',
    'manual-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-background-execution': iosRow(
    'background-execution',
    'manual-required',
    'manual-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-signing': iosRow(
    'signing-entitlements',
    'manual-required',
    'signing-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-testflight': iosRow(
    'testflight-distribution',
    'manual-required',
    'device-proof-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-device-proof': iosRow(
    'package-lifecycle',
    'manual-required',
    'device-proof-required',
    'child-ios-entitlement-capability-proof'
  ),
  'ios-app-store': iosRow('store-distribution', 'planned', 'planned', 'child-ios-entitlement-capability-proof'),
  'ios-external-transport': iosRow(
    'typed-protocol-bridge',
    'not-implemented',
    'not-implemented',
    'child-ios-entitlement-capability-proof'
  ),
} as const satisfies Record<
  MobileChildAgentCapabilitySurface,
  Pick<
    MobileChildAgentCapabilityRow,
    'platform' | 'parentCapability' | 'parentCapabilityStatus' | 'proofState' | 'source'
  >
>;

const RequiredHooks = {
  'android-debug-apk-checksum': hook(
    'android-child-agent',
    'ci-mechanical-proof',
    'child-android-device-proof-artifact-gate'
  ),
  'android-package-local-status': hook(
    'android-child-agent',
    'package-local-scaffold',
    'child-android-device-proof-artifact-gate'
  ),
  'android-device-install': hook(
    'android-child-agent',
    'device-proof-required',
    'child-android-device-proof-artifact-gate'
  ),
  'android-play-signing': hook('android-child-agent', 'planned', 'child-android-device-proof-artifact-gate'),
  'ios-xcode-target': hook('ios-child-agent', 'ci-mechanical-proof', 'child-ios-entitlement-capability-proof'),
  'ios-simulator-status': hook('ios-child-agent', 'simulator-scaffold', 'child-ios-entitlement-capability-proof'),
  'ios-signing-profile': hook('ios-child-agent', 'signing-required', 'child-ios-entitlement-capability-proof'),
  'ios-testflight-device': hook('ios-child-agent', 'device-proof-required', 'child-ios-entitlement-capability-proof'),
} as const satisfies Record<
  MobileChildAgentPackageRuntimeHook,
  Pick<MobileChildAgentPackageRuntimeHookProof, 'platform' | 'hookState' | 'source'>
>;

function mobileChildAgentCapabilityReadModelIsHonest(readModel: MobileChildAgentCapabilityReadModelCandidate): boolean {
  return (
    platformSummariesAreHonest(readModel.platforms) &&
    sourceProofsAreHonest(readModel.sourceProofs) &&
    capabilityRowsAreHonest(readModel.capabilityRows) &&
    packageRuntimeHooksAreHonest(readModel.packageRuntimeHooks) &&
    readModel.knownManualGaps.length >= 12
  );
}

function platformSummariesAreHonest(platforms: ReadonlyArray<MobileChildAgentPlatformSummary>): boolean {
  const byPlatform = new Map(platforms.map((entry) => [entry.platform, entry] as const));
  const android = byPlatform.get('android-child-agent');
  const ios = byPlatform.get('ios-child-agent');
  return (
    byPlatform.size === 2 &&
    android?.childAgentReadiness === 'manual-device-proof-required' &&
    android.packageRuntimeState === 'package-local-scaffold' &&
    android.privilegedOsState === 'blocked' &&
    android.externalTransportState === 'not-implemented' &&
    ios?.childAgentReadiness === 'entitlement-review-required' &&
    ios.packageRuntimeState === 'simulator-scaffold' &&
    ios.privilegedOsState === 'entitlement-required' &&
    ios.externalTransportState === 'not-implemented'
  );
}

function sourceProofsAreHonest(proofs: ReadonlyArray<MobileChildAgentCapabilitySourceProof>): boolean {
  const bySource = new Map(proofs.map((proof) => [proof.source, proof] as const));
  return (
    bySource.size === proofs.length &&
    RequiredSourceProofs.every((source) => {
      const proof = bySource.get(source);
      return proof?.status === 'ci-mechanical-proof' && proof.command.includes(`test:${source}`);
    })
  );
}

function capabilityRowsAreHonest(rows: ReadonlyArray<MobileChildAgentCapabilityRow>): boolean {
  const bySurface = new Map(rows.map((row) => [row.surface, row] as const));
  return (
    bySurface.size === rows.length &&
    RequiredSurfaces.every((surface) => capabilityRowIsHonest(bySurface.get(surface), surface))
  );
}

function capabilityRowIsHonest(
  row: MobileChildAgentCapabilityRow | undefined,
  surface: MobileChildAgentCapabilitySurface
): boolean {
  const expected = RowExpectations[surface];
  return Boolean(
    row &&
    row.platform === expected.platform &&
    row.parentCapability === expected.parentCapability &&
    row.parentCapabilityStatus === expected.parentCapabilityStatus &&
    row.proofState === expected.proofState &&
    row.source === expected.source &&
    row.claimBoundary.length > 0 &&
    row.proofRequirement.length > 0
  );
}

function packageRuntimeHooksAreHonest(hooks: ReadonlyArray<MobileChildAgentPackageRuntimeHookProof>): boolean {
  const byHook = new Map(hooks.map((entry) => [entry.hook, entry] as const));
  return (
    byHook.size === hooks.length &&
    Object.entries(RequiredHooks).every(([runtimeHook, expected]) => {
      const entry = byHook.get(runtimeHook as MobileChildAgentPackageRuntimeHook);
      return (
        entry?.platform === expected.platform &&
        entry.hookState === expected.hookState &&
        entry.source === expected.source &&
        hookEvidenceReferenceIsHonest(entry)
      );
    })
  );
}

function hookEvidenceReferenceIsHonest(entry: MobileChildAgentPackageRuntimeHookProof): boolean {
  if (
    entry.hookState === 'ci-mechanical-proof' ||
    entry.hookState === 'package-local-scaffold' ||
    entry.hookState === 'simulator-scaffold'
  ) {
    return entry.evidencePath !== null;
  }
  return entry.evidencePath === null;
}

function androidRow(
  parentCapability: MobileChildAgentCapabilityRow['parentCapability'],
  parentCapabilityStatus: MobileChildAgentCapabilityRow['parentCapabilityStatus'],
  proofState: MobileChildAgentCapabilityRow['proofState'],
  source: MobileChildAgentCapabilityRow['source']
) {
  return { platform: 'android-child-agent', parentCapability, parentCapabilityStatus, proofState, source } as const;
}

function iosRow(
  parentCapability: MobileChildAgentCapabilityRow['parentCapability'],
  parentCapabilityStatus: MobileChildAgentCapabilityRow['parentCapabilityStatus'],
  proofState: MobileChildAgentCapabilityRow['proofState'],
  source: MobileChildAgentCapabilityRow['source']
) {
  return { platform: 'ios-child-agent', parentCapability, parentCapabilityStatus, proofState, source } as const;
}

function hook(
  platform: MobileChildAgentPackageRuntimeHookProof['platform'],
  hookState: MobileChildAgentPackageRuntimeHookProof['hookState'],
  source: MobileChildAgentPackageRuntimeHookProof['source']
) {
  return { platform, hookState, source } as const;
}
