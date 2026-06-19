import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentControlCapabilityName,
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatus,
  ParentControlCapabilityStatusSchema,
  type ParentControlPlatform,
  ParentControlPlatformSchema,
} from '@ocentra-parent/capability-domain/capabilities';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  enforcementProofClaimFlagsAreUnset,
  enforcementProofEntriesHaveUniqueField,
} from './enforcement-proof-shape';

export const V08CrossPlatformEnforcementCapabilityProofReadModelIdSchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofReadModelId');
export const V08CrossPlatformEnforcementCapabilityProofEntryIdSchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofEntryId');
export const V08CrossPlatformEnforcementCapabilityProofReferenceSchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofReference');
export const V08CrossPlatformEnforcementCapabilityProofRequirementSchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofRequirement');
export const V08CrossPlatformEnforcementCapabilityProofClaimBoundarySchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofClaimBoundary');
export const V08CrossPlatformEnforcementCapabilityProofFallbackSchema = brandedNonEmptyStringSchema('V08CrossPlatformEnforcementCapabilityProofFallback');

export const V08CrossPlatformEnforcementCapabilitySurfaceSchema = withParser(
  Schema.Literal(
    'windows-owned-process-terminate',
    'windows-app-time-limit-lifecycle',
    'windows-managed-browser-boundary',
    'windows-unmanaged-browser-process-boundary',
    'windows-broad-installed-app-blocking',
    'windows-network-domain-blocking',
    'linux-enforcement-adapter-scaffold',
    'macos-enforcement-adapter-scaffold',
    'android-device-owner-policy',
    'android-package-lifecycle',
    'android-store-distribution',
    'ios-family-controls',
    'ios-signing-entitlements',
    'ios-testflight-distribution',
    'ios-store-distribution'
  )
);

export const V08CrossPlatformEnforcementCapabilityClaimStateSchema = withParser(
  Schema.Literal('implemented-boundary', 'manual-required', 'scaffold', 'unavailable', 'planned', 'not-claimed')
);

export const V08CrossPlatformEnforcementCapabilityAdapterExecutionStateSchema = withParser(
  Schema.Literal(
    'executes-real-service',
    'returns-manual-required',
    'returns-unavailable',
    'scaffold-only',
    'not-invoked'
  )
);

const V08CrossPlatformEnforcementCapabilityProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08CrossPlatformEnforcementCapabilityProofEntryIdSchema,
  surface: V08CrossPlatformEnforcementCapabilitySurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  productClaimState: V08CrossPlatformEnforcementCapabilityClaimStateSchema,
  adapterExecutionState: V08CrossPlatformEnforcementCapabilityAdapterExecutionStateSchema,
  linkedProofCommands: Schema.Array(V08CrossPlatformEnforcementCapabilityProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08CrossPlatformEnforcementCapabilityProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08CrossPlatformEnforcementCapabilityProofRequirementSchema),
  claimBoundary: V08CrossPlatformEnforcementCapabilityProofClaimBoundarySchema,
  fallbackBehavior: V08CrossPlatformEnforcementCapabilityProofFallbackSchema,
  broadBlockingClaimed: Schema.Boolean,
  exactUrlClaimed: Schema.Boolean,
  privilegedMobileClaimed: Schema.Boolean,
  productionDistributionClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08CrossPlatformEnforcementCapabilityProofEntryCandidate = Infer<
  typeof V08CrossPlatformEnforcementCapabilityProofEntryBaseSchema
>;

export const V08CrossPlatformEnforcementCapabilityProofEntrySchema = withParser(
  V08CrossPlatformEnforcementCapabilityProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        crossPlatformCapabilityEntryIsHonest(entry) ||
        'Expected cross-platform enforcement capability entries to preserve implemented-boundary, manual-required, scaffold, unavailable, planned, and not-claimed states'
    )
  )
);

export const V08CrossPlatformEnforcementCapabilityProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08CrossPlatformEnforcementCapabilityProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08CrossPlatformEnforcementCapabilityProofReferenceSchema),
    entries: Schema.Array(V08CrossPlatformEnforcementCapabilityProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        enforcementProofEntriesHaveUniqueField(readModel.entries, (entry) => entry.proofEntryId) ||
        'Expected cross-platform enforcement capability proof entry ids to be unique'
    )
  )
);

function crossPlatformCapabilityEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  if (crossPlatformCapabilityEntryHasClaimUpgrade(entry)) {
    return false;
  }

  return crossPlatformCapabilityClaimStateIsHonest(entry);
}

function crossPlatformCapabilityEntryHasClaimUpgrade(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return !enforcementProofClaimFlagsAreUnset([
    entry.broadBlockingClaimed,
    entry.exactUrlClaimed,
    entry.privilegedMobileClaimed,
    entry.productionDistributionClaimed,
  ]);
}

function crossPlatformCapabilityClaimStateIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  switch (entry.productClaimState) {
    case 'implemented-boundary':
      return crossPlatformCapabilityEntryIsImplementedBoundary(entry);
    case 'manual-required':
      return crossPlatformCapabilityManualEntryIsHonest(entry);
    case 'scaffold':
      return crossPlatformCapabilityScaffoldEntryIsHonest(entry);
    case 'unavailable':
      return crossPlatformCapabilityUnavailableEntryIsHonest(entry);
    case 'planned':
      return crossPlatformCapabilityPlannedEntryIsHonest(entry);
    case 'not-claimed':
      return crossPlatformCapabilityNotClaimedEntryIsHonest(entry);
  }
}

function crossPlatformCapabilityEntryIsImplementedBoundary(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return (
    entry.platform === 'windows' &&
    crossPlatformImplementedCapabilityStatuses.includes(entry.capabilityStatus) &&
    entry.adapterExecutionState === 'executes-real-service' &&
    entry.linkedProofCommands.length > 0 &&
    entry.linkedProofArtifacts.length > 0
  );
}

const crossPlatformImplementedCapabilityStatuses = [
  ParentControlCapabilityStatus.Implemented,
  ParentControlCapabilityStatus.Supported,
] as const;

const crossPlatformScaffoldCapabilityStatuses = [
  ParentControlCapabilityStatus.Scaffold,
  ParentControlCapabilityStatus.PreviewScaffold,
] as const;

function crossPlatformCapabilityManualEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return crossPlatformCapabilityEntryMatchesState(
    entry,
    ParentControlCapabilityStatus.ManualRequired,
    'returns-manual-required'
  );
}

function crossPlatformCapabilityScaffoldEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return (
    crossPlatformScaffoldCapabilityStatuses.includes(entry.capabilityStatus) &&
    crossPlatformCapabilityEntryMatchesExecution(entry, 'scaffold-only')
  );
}

function crossPlatformCapabilityUnavailableEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return crossPlatformCapabilityEntryMatchesState(
    entry,
    ParentControlCapabilityStatus.Unavailable,
    'returns-unavailable'
  );
}

function crossPlatformCapabilityPlannedEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return crossPlatformCapabilityEntryMatchesState(entry, ParentControlCapabilityStatus.Planned, 'not-invoked');
}

function crossPlatformCapabilityNotClaimedEntryIsHonest(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate
): boolean {
  return crossPlatformCapabilityEntryMatchesExecution(entry, 'not-invoked');
}

function crossPlatformCapabilityEntryMatchesState(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate,
  capabilityStatus: ParentControlCapabilityStatus,
  adapterExecutionState: V08CrossPlatformEnforcementCapabilityAdapterExecutionState
): boolean {
  return (
    entry.capabilityStatus === capabilityStatus &&
    crossPlatformCapabilityEntryMatchesExecution(entry, adapterExecutionState)
  );
}

function crossPlatformCapabilityEntryMatchesExecution(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryCandidate,
  adapterExecutionState: V08CrossPlatformEnforcementCapabilityAdapterExecutionState
): boolean {
  return entry.adapterExecutionState === adapterExecutionState && entry.manualProofRequirements.length > 0;
}

export type V08CrossPlatformEnforcementCapabilityProofReadModelId =
  typeof V08CrossPlatformEnforcementCapabilityProofReadModelIdSchema.Type;
export type V08CrossPlatformEnforcementCapabilityProofEntryId =
  typeof V08CrossPlatformEnforcementCapabilityProofEntryIdSchema.Type;
export type V08CrossPlatformEnforcementCapabilityProofReference =
  typeof V08CrossPlatformEnforcementCapabilityProofReferenceSchema.Type;
export type V08CrossPlatformEnforcementCapabilityProofRequirement =
  typeof V08CrossPlatformEnforcementCapabilityProofRequirementSchema.Type;
export type V08CrossPlatformEnforcementCapabilityProofClaimBoundary =
  typeof V08CrossPlatformEnforcementCapabilityProofClaimBoundarySchema.Type;
export type V08CrossPlatformEnforcementCapabilityProofFallback =
  typeof V08CrossPlatformEnforcementCapabilityProofFallbackSchema.Type;
export type V08CrossPlatformEnforcementCapabilitySurface = Infer<
  typeof V08CrossPlatformEnforcementCapabilitySurfaceSchema
>;
export type V08CrossPlatformEnforcementCapabilityClaimState = Infer<
  typeof V08CrossPlatformEnforcementCapabilityClaimStateSchema
>;
export type V08CrossPlatformEnforcementCapabilityAdapterExecutionState = Infer<
  typeof V08CrossPlatformEnforcementCapabilityAdapterExecutionStateSchema
>;
export type V08CrossPlatformEnforcementCapabilityProofEntry = Infer<
  typeof V08CrossPlatformEnforcementCapabilityProofEntrySchema
>;
export type V08CrossPlatformEnforcementCapabilityProofReadModel = Infer<
  typeof V08CrossPlatformEnforcementCapabilityProofReadModelSchema
>;

type V08CrossPlatformEnforcementCapabilityProofEntryInput = {
  proofEntryId: string;
  surface: V08CrossPlatformEnforcementCapabilitySurface;
  platform: ParentControlPlatform;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  productClaimState: V08CrossPlatformEnforcementCapabilityClaimState;
  adapterExecutionState: V08CrossPlatformEnforcementCapabilityAdapterExecutionState;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08CrossPlatformEnforcementCapabilitySurface = {
  WindowsOwnedProcessTerminate: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-owned-process-terminate'
  ),
  WindowsAppTimeLimitLifecycle: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-app-time-limit-lifecycle'
  ),
  WindowsManagedBrowserBoundary: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-managed-browser-boundary'
  ),
  WindowsUnmanagedBrowserProcessBoundary: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-unmanaged-browser-process-boundary'
  ),
  WindowsBroadInstalledAppBlocking: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-broad-installed-app-blocking'
  ),
  WindowsNetworkDomainBlocking: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'windows-network-domain-blocking'
  ),
  LinuxEnforcementAdapterScaffold: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'linux-enforcement-adapter-scaffold'
  ),
  MacosEnforcementAdapterScaffold: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse(
    'macos-enforcement-adapter-scaffold'
  ),
  AndroidDeviceOwnerPolicy: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('android-device-owner-policy'),
  AndroidPackageLifecycle: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('android-package-lifecycle'),
  AndroidStoreDistribution: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('android-store-distribution'),
  IosFamilyControls: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('ios-family-controls'),
  IosSigningEntitlements: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('ios-signing-entitlements'),
  IosTestflightDistribution: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('ios-testflight-distribution'),
  IosStoreDistribution: V08CrossPlatformEnforcementCapabilitySurfaceSchema.parse('ios-store-distribution'),
} as const;

const documentedAt = '2026-05-30T19:00:00.000Z';

export const V08CrossPlatformEnforcementCapabilityProofReadModel =
  V08CrossPlatformEnforcementCapabilityProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'v0-8-cross-platform-enforcement-capability-proof',
    generatedAt: documentedAt,
    sourceReadModelIds: [
      'v0-8-broad-os-adapter-proof',
      'v0-8-os-adapter-product-proof',
      'enforcement-lan-mobile-product-proof',
      'parent-control-platform-capabilities',
    ],
    entries: [
      implementedBoundaryEntry(
        'v0-8-cross-platform-windows-owned-process-terminate',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsOwnedProcessTerminate,
        ParentControlCapabilityName.OwnedProcessTerminate,
        ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
        'Windows owned-process terminate is limited to pid/name guarded process control and is not broad app blocking.',
        'Reject missing pid or process-name mismatch; return unavailable on unsupported hosts.'
      ),
      implementedBoundaryEntry(
        'v0-8-cross-platform-windows-app-time-limit',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsAppTimeLimitLifecycle,
        ParentControlCapabilityName.AppTimeLimit,
        ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
        ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
        'Windows app time-limit proof covers timer lifecycle, restart recovery, parent cancel, expiry, and audit only.',
        'Return unavailable when timer state, persisted state, or adapter support is missing.'
      ),
      implementedBoundaryEntry(
        'v0-8-cross-platform-windows-managed-browser-boundary',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsManagedBrowserBoundary,
        ParentControlCapabilityName.ManagedBrowserControl,
        ['node scripts/test/managed-browser-intervention-proof.mjs'],
        ['test-results/managed-browser-intervention-proof/proof.json'],
        'Managed-browser control is limited to the Ocentra-owned managed browser boundary and is not unmanaged exact URL proof.',
        'Return manual-required when active-tab or exact URL apply/rollback proof is missing.'
      ),
      implementedBoundaryEntry(
        'v0-8-cross-platform-windows-unmanaged-browser-boundary',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsUnmanagedBrowserProcessBoundary,
        ParentControlCapabilityName.UnmanagedBrowserDetection,
        ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
        'Unmanaged browser detection is process-only and cannot prove URL, active tab, title, page, HTTPS content, or intent.',
        'Keep exact unmanaged browser evidence not-claimed unless explicit browser integration proof exists.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-windows-broad-app-blocking',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsBroadInstalledAppBlocking,
        'windows',
        ParentControlCapabilityName.AppBlocking,
        ['OS-approved installed-app identity', 'block apply result', 'rollback result', 'audit custody artifact'],
        'Broad installed-app blocking remains manual-required beyond owned-process terminate and app timer proof.',
        'Return manual-required until package identity, apply, rollback, and audit custody artifacts exist.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-windows-network-domain-blocking',
        V08CrossPlatformEnforcementCapabilitySurface.WindowsNetworkDomainBlocking,
        'windows',
        ParentControlCapabilityName.NetworkDomainBlocking,
        ['host network filter adapter', 'domain filter apply result', 'rollback result', 'audit custody artifact'],
        'Network/domain blocking remains manual-required and is not proved by network observation metadata.',
        'Return manual-required until DNS/VPN/filter apply, rollback, and custody evidence exists.'
      ),
      scaffoldEntry(
        'v0-8-cross-platform-linux-adapter-scaffold',
        V08CrossPlatformEnforcementCapabilitySurface.LinuxEnforcementAdapterScaffold,
        'linux',
        ParentControlCapabilityName.HeadlessAgentService,
        ParentControlCapabilityStatus.PreviewScaffold,
        ['Linux service-manager install proof', 'Linux adapter apply/rollback proof'],
        'Linux package preview is scaffold evidence only and cannot inherit Windows enforcement behavior.',
        'Report scaffold-only until Linux-specific enforcement adapter proof exists.'
      ),
      scaffoldEntry(
        'v0-8-cross-platform-macos-adapter-scaffold',
        V08CrossPlatformEnforcementCapabilitySurface.MacosEnforcementAdapterScaffold,
        'macos',
        ParentControlCapabilityName.HeadlessAgentService,
        ParentControlCapabilityStatus.PreviewScaffold,
        ['macOS permissions proof', 'launchd/package proof', 'macOS adapter apply/rollback proof'],
        'macOS package preview is scaffold evidence only and cannot inherit Windows enforcement behavior.',
        'Report scaffold-only until macOS-specific enforcement adapter and permission proof exists.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-android-device-owner-policy',
        V08CrossPlatformEnforcementCapabilitySurface.AndroidDeviceOwnerPolicy,
        'android',
        ParentControlCapabilityName.DeviceOwnerPolicy,
        ['device-owner enrollment artifact', 'policy apply result', 'managed-profile compatibility proof'],
        'Android device-owner enforcement is manual-required and not implied by parent mobile or protocol scaffold.',
        'Return manual-required until real device-owner or managed-profile proof exists.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-android-package-lifecycle',
        V08CrossPlatformEnforcementCapabilitySurface.AndroidPackageLifecycle,
        'android',
        ParentControlCapabilityName.PackageLifecycle,
        ['debug/release package install artifact', 'background/reboot lifecycle proof', 'uninstall/update proof'],
        'Android package lifecycle remains manual-required before any child enforcement support upgrade.',
        'Return manual-required until emulator or physical-device package lifecycle artifacts exist.'
      ),
      plannedEntry(
        'v0-8-cross-platform-android-store-distribution',
        V08CrossPlatformEnforcementCapabilitySurface.AndroidStoreDistribution,
        'android',
        ParentControlCapabilityName.StoreDistribution,
        ['Google Play signing proof', 'release track artifact', 'policy compliance review'],
        'Android store distribution is planned and cannot be used as enforcement support evidence.',
        'Do not invoke privileged mobile enforcement until store/signing proof and device capability proof exist.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-ios-family-controls',
        V08CrossPlatformEnforcementCapabilitySurface.IosFamilyControls,
        'ios',
        ParentControlCapabilityName.FamilyControlsEntitlement,
        ['Family Controls entitlement approval', 'DeviceActivity proof', 'real device or TestFlight artifact'],
        'iOS Family Controls support is manual-required and cannot be inferred from simulator/package scaffolds.',
        'Return manual-required until approved entitlement and device proof exist.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-ios-signing-entitlements',
        V08CrossPlatformEnforcementCapabilitySurface.IosSigningEntitlements,
        'ios',
        ParentControlCapabilityName.SigningEntitlements,
        ['Apple signing credentials', 'approved entitlements', 'device or TestFlight install proof'],
        'iOS signing and entitlements are manual-required before any privileged child enforcement claim.',
        'Return manual-required until signing, entitlement, and install artifacts exist.'
      ),
      manualRequiredEntry(
        'v0-8-cross-platform-ios-testflight-distribution',
        V08CrossPlatformEnforcementCapabilitySurface.IosTestflightDistribution,
        'ios',
        ParentControlCapabilityName.TestflightDistribution,
        ['TestFlight build artifact', 'App Store Connect evidence', 'device install proof'],
        'iOS TestFlight distribution is manual-required before mobile enforcement support can be claimed.',
        'Return manual-required until TestFlight and device proof exist.'
      ),
      plannedEntry(
        'v0-8-cross-platform-ios-store-distribution',
        V08CrossPlatformEnforcementCapabilitySurface.IosStoreDistribution,
        'ios',
        ParentControlCapabilityName.StoreDistribution,
        ['Apple signing proof', 'App Store review path', 'release artifact'],
        'iOS store distribution is planned and is not privileged enforcement proof.',
        'Do not claim mobile child enforcement from planned store distribution work.'
      ),
    ],
  });

function implementedBoundaryEntry(
  proofEntryId: string,
  surface: V08CrossPlatformEnforcementCapabilitySurface,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  linkedProofCommands: readonly string[],
  linkedProofArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08CrossPlatformEnforcementCapabilityProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform: 'windows',
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    linkedProofCommands,
    linkedProofArtifacts,
    manualProofRequirements: [],
    claimBoundary,
    fallbackBehavior,
  });
}

function manualRequiredEntry(
  proofEntryId: string,
  surface: V08CrossPlatformEnforcementCapabilitySurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08CrossPlatformEnforcementCapabilityProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function scaffoldEntry(
  proofEntryId: string,
  surface: V08CrossPlatformEnforcementCapabilitySurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08CrossPlatformEnforcementCapabilityProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus,
    productClaimState: 'scaffold',
    adapterExecutionState: 'scaffold-only',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function plannedEntry(
  proofEntryId: string,
  surface: V08CrossPlatformEnforcementCapabilitySurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08CrossPlatformEnforcementCapabilityProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Planned,
    productClaimState: 'planned',
    adapterExecutionState: 'not-invoked',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function proofEntry(
  entry: V08CrossPlatformEnforcementCapabilityProofEntryInput
): V08CrossPlatformEnforcementCapabilityProofEntry {
  return V08CrossPlatformEnforcementCapabilityProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    broadBlockingClaimed: false,
    exactUrlClaimed: false,
    privilegedMobileClaimed: false,
    productionDistributionClaimed: false,
    lastCheckedAt: documentedAt,
    ...entry,
  });
}

export const decodeV08CrossPlatformEnforcementCapabilityProofEntry = Schema.decodeUnknownSync(
  V08CrossPlatformEnforcementCapabilityProofEntrySchema
);
export const decodeV08CrossPlatformEnforcementCapabilityProofReadModel = Schema.decodeUnknownSync(
  V08CrossPlatformEnforcementCapabilityProofReadModelSchema
);

