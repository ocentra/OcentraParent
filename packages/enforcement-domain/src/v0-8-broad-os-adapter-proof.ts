import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  EnforcementAdapterKind,
  EnforcementAdapterKindSchema,
  EnforcementCapabilityState,
  EnforcementCapabilityStateSchema,
  EnforcementMode,
  EnforcementModeSchema,
} from './enforcement';
import {
  EnforcementReadinessProofLevel,
  EnforcementReadinessProofLevelSchema,
  EnforcementReadinessRuntimeOwner,
  EnforcementReadinessRuntimeOwnerSchema,
  EnforcementReadinessState,
  EnforcementReadinessStateSchema,
} from './enforcement-readiness';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatform,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
const NonEmptyBroadOsProofText = Schema.String.pipe(Schema.minLength(1));
export const V08BroadOsAdapterProofReadModelIdSchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofReadModelId')
);
export const V08BroadOsAdapterProofEntryIdSchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofEntryId')
);
export const V08BroadOsAdapterProofReferenceSchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofReference')
);
export const V08BroadOsAdapterProofRequirementSchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofRequirement')
);
export const V08BroadOsAdapterProofClaimBoundarySchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofClaimBoundary')
);
export const V08BroadOsAdapterProofFallbackSchema = NonEmptyBroadOsProofText.pipe(
  Schema.brand('V08BroadOsAdapterProofFallback')
);

export const V08BroadOsAdapterProofSurfaceSchema = withParser(
  Schema.Literal(
    'windows-managed-session-intervention',
    'windows-owned-process-guardrail',
    'windows-unmanaged-process-boundary',
    'windows-app-time-limit-lifecycle',
    'windows-broad-installed-app-blocking',
    'windows-network-domain-blocking',
    'windows-managed-browser-exact-url',
    'windows-unmanaged-exact-evidence',
    'windows-admin-rollback-hardening',
    'linux-broad-os-adapter',
    'macos-broad-os-adapter',
    'android-child-os-adapter',
    'ios-child-os-adapter'
  )
);

export const V08BroadOsAdapterTargetSupportSchema = withParser(
  Schema.Literal('host-supported', 'manual-proof-required', 'unavailable-on-target', 'not-implemented')
);

export const V08BroadOsAdapterRuntimeProofStateSchema = withParser(
  Schema.Literal('real-service-proof', 'manual-required', 'unavailable', 'not-claimed')
);

const V08BroadOsAdapterProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08BroadOsAdapterProofEntryIdSchema,
  surface: V08BroadOsAdapterProofSurfaceSchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  readinessState: EnforcementReadinessStateSchema,
  proofLevel: EnforcementReadinessProofLevelSchema,
  runtimeOwner: EnforcementReadinessRuntimeOwnerSchema,
  supportedModes: Schema.Array(EnforcementModeSchema),
  targetSupport: V08BroadOsAdapterTargetSupportSchema,
  runtimeProofState: V08BroadOsAdapterRuntimeProofStateSchema,
  linkedProofCommands: Schema.Array(V08BroadOsAdapterProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08BroadOsAdapterProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08BroadOsAdapterProofRequirementSchema),
  claimBoundary: V08BroadOsAdapterProofClaimBoundarySchema,
  fallbackBehavior: V08BroadOsAdapterProofFallbackSchema,
  claimUpgradeAllowed: Schema.Boolean,
  broadOsBlockingClaimed: Schema.Boolean,
  exactUrlClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08BroadOsAdapterProofEntryCandidate = Infer<typeof V08BroadOsAdapterProofEntryBaseSchema>;

export const V08BroadOsAdapterProofEntrySchema = withParser(
  V08BroadOsAdapterProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        broadOsAdapterProofEntryIsHonest(entry) ||
        'Expected V0.8 broad OS adapter proof entries to preserve real, manual-required, unavailable, and not-claimed boundaries'
    )
  )
);

export const V08BroadOsAdapterProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08BroadOsAdapterProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08BroadOsAdapterProofReferenceSchema),
    entries: Schema.Array(V08BroadOsAdapterProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.proofEntryId)).size === readModel.entries.length ||
        'Expected V0.8 broad OS adapter proof entry ids to be unique'
    )
  )
);

function broadOsAdapterProofEntryIsHonest(entry: V08BroadOsAdapterProofEntryCandidate): boolean {
  if (entry.claimUpgradeAllowed || entry.broadOsBlockingClaimed || entry.exactUrlClaimed) {
    return false;
  }

  switch (entry.runtimeProofState) {
    case 'real-service-proof':
      return broadOsAdapterProofEntryIsRealProof(entry);
    case 'manual-required':
      return broadOsAdapterProofEntryIsManualRequired(entry);
    case 'unavailable':
      return broadOsAdapterProofEntryIsUnavailable(entry);
    case 'not-claimed':
      return broadOsAdapterProofEntryIsNotClaimed(entry);
  }
}

function broadOsAdapterProofEntryIsRealProof(entry: V08BroadOsAdapterProofEntryCandidate): boolean {
  return (
    entry.platform === 'windows' &&
    entry.capabilityState === 'supported' &&
    entry.readinessState === 'implemented' &&
    entry.proofLevel === 'real-service-proof' &&
    entry.targetSupport === 'host-supported' &&
    entry.linkedProofArtifacts.length > 0
  );
}

function broadOsAdapterProofEntryIsManualRequired(entry: V08BroadOsAdapterProofEntryCandidate): boolean {
  return (
    entry.capabilityState === 'manual-required' &&
    entry.readinessState === 'manual-required' &&
    entry.proofLevel === 'manual-proof-required' &&
    entry.targetSupport === 'manual-proof-required' &&
    entry.manualProofRequirements.length > 0
  );
}

function broadOsAdapterProofEntryIsUnavailable(entry: V08BroadOsAdapterProofEntryCandidate): boolean {
  return (
    entry.capabilityState === 'unavailable' &&
    entry.readinessState === 'unavailable' &&
    entry.targetSupport === 'unavailable-on-target' &&
    entry.manualProofRequirements.length > 0
  );
}

function broadOsAdapterProofEntryIsNotClaimed(entry: V08BroadOsAdapterProofEntryCandidate): boolean {
  return (
    entry.readinessState === 'not-claimed' &&
    entry.proofLevel === 'not-proved' &&
    entry.runtimeOwner === 'not-implemented' &&
    entry.targetSupport === 'not-implemented' &&
    entry.manualProofRequirements.length > 0
  );
}

export type V08BroadOsAdapterProofReadModelId = typeof V08BroadOsAdapterProofReadModelIdSchema.Type;
export type V08BroadOsAdapterProofEntryId = typeof V08BroadOsAdapterProofEntryIdSchema.Type;
export type V08BroadOsAdapterProofReference = typeof V08BroadOsAdapterProofReferenceSchema.Type;
export type V08BroadOsAdapterProofRequirement = typeof V08BroadOsAdapterProofRequirementSchema.Type;
export type V08BroadOsAdapterProofClaimBoundary = typeof V08BroadOsAdapterProofClaimBoundarySchema.Type;
export type V08BroadOsAdapterProofFallback = typeof V08BroadOsAdapterProofFallbackSchema.Type;
export type V08BroadOsAdapterProofSurface = Infer<typeof V08BroadOsAdapterProofSurfaceSchema>;
export type V08BroadOsAdapterTargetSupport = Infer<typeof V08BroadOsAdapterTargetSupportSchema>;
export type V08BroadOsAdapterRuntimeProofState = Infer<typeof V08BroadOsAdapterRuntimeProofStateSchema>;
export type V08BroadOsAdapterProofEntry = Infer<typeof V08BroadOsAdapterProofEntrySchema>;
export type V08BroadOsAdapterProofReadModel = Infer<typeof V08BroadOsAdapterProofReadModelSchema>;

type V08BroadOsAdapterProofEntryInput = {
  proofEntryId: string;
  surface: V08BroadOsAdapterProofSurface;
  platform: ParentPlatform;
  adapterKind: typeof EnforcementAdapterKindSchema.Type;
  capabilityState: typeof EnforcementCapabilityStateSchema.Type;
  readinessState: EnforcementReadinessState;
  proofLevel: EnforcementReadinessProofLevel;
  runtimeOwner: EnforcementReadinessRuntimeOwner;
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>;
  targetSupport: V08BroadOsAdapterTargetSupport;
  runtimeProofState: V08BroadOsAdapterRuntimeProofState;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08BroadOsAdapterProofSurface = {
  WindowsManagedSessionIntervention: V08BroadOsAdapterProofSurfaceSchema.parse('windows-managed-session-intervention'),
  WindowsOwnedProcessGuardrail: V08BroadOsAdapterProofSurfaceSchema.parse('windows-owned-process-guardrail'),
  WindowsUnmanagedProcessBoundary: V08BroadOsAdapterProofSurfaceSchema.parse('windows-unmanaged-process-boundary'),
  WindowsAppTimeLimitLifecycle: V08BroadOsAdapterProofSurfaceSchema.parse('windows-app-time-limit-lifecycle'),
  WindowsBroadInstalledAppBlocking: V08BroadOsAdapterProofSurfaceSchema.parse('windows-broad-installed-app-blocking'),
  WindowsNetworkDomainBlocking: V08BroadOsAdapterProofSurfaceSchema.parse('windows-network-domain-blocking'),
  WindowsManagedBrowserExactUrl: V08BroadOsAdapterProofSurfaceSchema.parse('windows-managed-browser-exact-url'),
  WindowsUnmanagedExactEvidence: V08BroadOsAdapterProofSurfaceSchema.parse('windows-unmanaged-exact-evidence'),
  WindowsAdminRollbackHardening: V08BroadOsAdapterProofSurfaceSchema.parse('windows-admin-rollback-hardening'),
  LinuxBroadOsAdapter: V08BroadOsAdapterProofSurfaceSchema.parse('linux-broad-os-adapter'),
  MacosBroadOsAdapter: V08BroadOsAdapterProofSurfaceSchema.parse('macos-broad-os-adapter'),
  AndroidChildOsAdapter: V08BroadOsAdapterProofSurfaceSchema.parse('android-child-os-adapter'),
  IosChildOsAdapter: V08BroadOsAdapterProofSurfaceSchema.parse('ios-child-os-adapter'),
} as const;

const documentedAt = '2026-05-30T16:45:00.000Z';

export const V08BroadOsAdapterProofReadModel = V08BroadOsAdapterProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-broad-os-adapter-proof',
  generatedAt: documentedAt,
  sourceReadModelIds: [
    'v0-8-broad-os-adapter-readiness',
    'v0-8-os-adapter-product-proof',
    'v0-8-host-adapter-proof-preflight',
    'v0-8-windows-adapter-artifact-ingestion-proof',
  ],
  entries: [
    realWindowsEntry(
      'v0-8-broad-proof-managed-session-intervention',
      V08BroadOsAdapterProofSurface.WindowsManagedSessionIntervention,
      EnforcementAdapterKind.ManagedBrowserControl,
      [EnforcementMode.TemporaryBlock],
      ['node scripts/test/managed-browser-intervention-proof.mjs'],
      ['test-results/managed-browser-intervention-proof/proof.json'],
      'Managed-session intervention is proved only for the managed browser path and cannot upgrade unmanaged browser or system-wide URL claims.',
      'Return manual-required when managed browser launch/control proof is unavailable on the host.'
    ),
    realWindowsEntry(
      'v0-8-broad-proof-owned-process-guardrail',
      V08BroadOsAdapterProofSurface.WindowsOwnedProcessGuardrail,
      EnforcementAdapterKind.ProcessControl,
      [EnforcementMode.TerminateProcess],
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
      'Owned-process pid/name termination is proved; this is not broad installed-app blocking.',
      'Reject missing pid or expected-name mismatch and report unavailable when the host adapter cannot act.'
    ),
    realWindowsEntry(
      'v0-8-broad-proof-unmanaged-process-boundary',
      V08BroadOsAdapterProofSurface.WindowsUnmanagedProcessBoundary,
      EnforcementAdapterKind.ProcessControl,
      [EnforcementMode.TerminateProcess, EnforcementMode.ObserveOnly],
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
      'Unmanaged browser proof is process terminate/warn only and never exact URL, tab, title, page, or HTTPS content proof.',
      'Keep unmanaged browser exact evidence unavailable unless a managed or explicit browser integration exists.'
    ),
    realWindowsEntry(
      'v0-8-broad-proof-app-time-limit-lifecycle',
      V08BroadOsAdapterProofSurface.WindowsAppTimeLimitLifecycle,
      EnforcementAdapterKind.TimerControl,
      [EnforcementMode.TimeLimit, EnforcementMode.AskParent],
      ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
      ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      'App time-limit lifecycle proof covers local timer custody, restart recovery, expiry, cancel, and audit only.',
      'Return unavailable when active timer state, persisted state, or adapter support is missing.'
    ),
    manualEntry(
      'v0-8-broad-proof-broad-installed-app-blocking',
      V08BroadOsAdapterProofSurface.WindowsBroadInstalledAppBlocking,
      ParentPlatform.Windows,
      EnforcementAdapterKind.ProcessControl,
      [EnforcementMode.BlockProcess],
      ['OS-approved installed-app identity', 'apply result', 'rollback result', 'audit custody'],
      'Broad installed-app blocking remains manual-required; owned-process termination and timers do not prove it.',
      'Avoid broad block claims and return manual-required until package identity plus apply/rollback proof exists.'
    ),
    manualEntry(
      'v0-8-broad-proof-network-domain-blocking',
      V08BroadOsAdapterProofSurface.WindowsNetworkDomainBlocking,
      ParentPlatform.Windows,
      EnforcementAdapterKind.NetworkControl,
      [EnforcementMode.TemporaryBlock],
      ['host network filter adapter', 'domain filter apply result', 'rollback result', 'audit custody'],
      'Network/domain blocking remains manual-required; flow metadata is not decrypted content or enforcement proof.',
      'Return manual-required until a host network filter or DNS/VPN adapter has real apply and rollback proof.'
    ),
    manualEntry(
      'v0-8-broad-proof-managed-browser-exact-url',
      V08BroadOsAdapterProofSurface.WindowsManagedBrowserExactUrl,
      ParentPlatform.Windows,
      EnforcementAdapterKind.ManagedBrowserControl,
      [EnforcementMode.TemporaryBlock],
      ['managed active-tab proof', 'exact URL apply result', 'rollback result', 'audit custody'],
      'Managed exact URL control remains manual-required until active-tab and exact URL enforcement proof exists.',
      'Keep managed browser command or launch proof separate from exact URL enforcement claims.'
    ),
    notClaimedEntry(
      'v0-8-broad-proof-unmanaged-exact-evidence',
      V08BroadOsAdapterProofSurface.WindowsUnmanagedExactEvidence,
      ParentPlatform.Windows,
      EnforcementAdapterKind.ManagedBrowserControl,
      ['managed browser or explicit browser integration proof'],
      'Unmanaged browser process/window/network evidence is not exact URL, active tab, title, download, page text, HTTPS content, or intent proof.',
      'Represent unmanaged browser exact evidence as not-claimed until an explicit browser boundary exists.'
    ),
    manualEntry(
      'v0-8-broad-proof-admin-rollback-hardening',
      V08BroadOsAdapterProofSurface.WindowsAdminRollbackHardening,
      ParentPlatform.Windows,
      EnforcementAdapterKind.ProcessControl,
      [],
      ['admin hardening proof', 'anti-tamper proof', 'same-identity rollback proof', 'bypass-resistance proof'],
      'Admin hardening, anti-tamper, and broad rollback remain manual-required and are not proved by local enforcement tests.',
      'Keep rollback and bypass-resistance claims manual-required until real host hardening artifacts exist.'
    ),
    unavailableEntry(
      'v0-8-broad-proof-linux-broad-os-adapter',
      V08BroadOsAdapterProofSurface.LinuxBroadOsAdapter,
      ParentPlatform.Linux,
      EnforcementAdapterKind.ProcessControl,
      ['Linux package identity, permissions, service manager, and rollback artifacts'],
      'Linux broad OS enforcement is unavailable in the current proof and cannot inherit Windows adapter results.',
      'Report unavailable/manual-required states until a Linux-specific adapter and real host proof exist.'
    ),
    manualEntry(
      'v0-8-broad-proof-macos-broad-os-adapter',
      V08BroadOsAdapterProofSurface.MacosBroadOsAdapter,
      ParentPlatform.Macos,
      EnforcementAdapterKind.ProcessControl,
      [],
      ['macOS permissions', 'app identity proof', 'network extension or managed browser proof', 'rollback proof'],
      'macOS broad OS enforcement remains manual-required until platform permissions and adapter artifacts are proved.',
      'Do not present Windows proof as macOS support; require macOS-specific host proof before upgrade.'
    ),
    manualEntry(
      'v0-8-broad-proof-android-child-os-adapter',
      V08BroadOsAdapterProofSurface.AndroidChildOsAdapter,
      ParentPlatform.Android,
      EnforcementAdapterKind.ProcessControl,
      [],
      [
        'device-owner or managed-profile proof',
        'UsageStats proof',
        'accessibility or VPN/DNS proof',
        'package lifecycle proof',
      ],
      'Android child enforcement remains manual-required until device policy and package evidence are real.',
      'Keep Android support manual-required unless emulator or physical-device artifacts prove the capability.'
    ),
    manualEntry(
      'v0-8-broad-proof-ios-child-os-adapter',
      V08BroadOsAdapterProofSurface.IosChildOsAdapter,
      ParentPlatform.Ios,
      EnforcementAdapterKind.ManagedBrowserControl,
      [],
      [
        'Family Controls entitlement',
        'DeviceActivity proof',
        'Network Extension proof',
        'signing and TestFlight proof',
      ],
      'iOS child enforcement remains manual-required until Apple entitlements and device artifacts are real.',
      'Keep iOS support manual-required unless entitlement, signing, and device proof exist.'
    ),
  ],
});

function realWindowsEntry(
  proofEntryId: string,
  surface: V08BroadOsAdapterProofSurface,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>,
  linkedProofCommands: readonly string[],
  linkedProofArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BroadOsAdapterProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform: ParentPlatform.Windows,
    adapterKind,
    capabilityState: EnforcementCapabilityState.Supported,
    readinessState: EnforcementReadinessState.Implemented,
    proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
    runtimeOwner: EnforcementReadinessRuntimeOwner.OsAdapter,
    supportedModes,
    targetSupport: 'host-supported',
    runtimeProofState: 'real-service-proof',
    linkedProofCommands,
    linkedProofArtifacts,
    manualProofRequirements: [],
    claimBoundary,
    fallbackBehavior,
  });
}

function manualEntry(
  proofEntryId: string,
  surface: V08BroadOsAdapterProofSurface,
  platform: ParentPlatform,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BroadOsAdapterProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    adapterKind,
    capabilityState: EnforcementCapabilityState.ManualRequired,
    readinessState: EnforcementReadinessState.ManualRequired,
    proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
    runtimeOwner: EnforcementReadinessRuntimeOwner.ManualProof,
    supportedModes,
    targetSupport: 'manual-proof-required',
    runtimeProofState: 'manual-required',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function unavailableEntry(
  proofEntryId: string,
  surface: V08BroadOsAdapterProofSurface,
  platform: ParentPlatform,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BroadOsAdapterProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    adapterKind,
    capabilityState: EnforcementCapabilityState.Unavailable,
    readinessState: EnforcementReadinessState.Unavailable,
    proofLevel: EnforcementReadinessProofLevel.NotProved,
    runtimeOwner: EnforcementReadinessRuntimeOwner.NotImplemented,
    supportedModes: [],
    targetSupport: 'unavailable-on-target',
    runtimeProofState: 'unavailable',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function notClaimedEntry(
  proofEntryId: string,
  surface: V08BroadOsAdapterProofSurface,
  platform: ParentPlatform,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BroadOsAdapterProofEntry {
  return proofEntry({
    proofEntryId,
    surface,
    platform,
    adapterKind,
    capabilityState: EnforcementCapabilityState.ManualRequired,
    readinessState: EnforcementReadinessState.NotClaimed,
    proofLevel: EnforcementReadinessProofLevel.NotProved,
    runtimeOwner: EnforcementReadinessRuntimeOwner.NotImplemented,
    supportedModes: [],
    targetSupport: 'not-implemented',
    runtimeProofState: 'not-claimed',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function proofEntry(entry: V08BroadOsAdapterProofEntryInput): V08BroadOsAdapterProofEntry {
  return V08BroadOsAdapterProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    claimUpgradeAllowed: false,
    broadOsBlockingClaimed: false,
    exactUrlClaimed: false,
    lastCheckedAt: documentedAt,
    ...entry,
  });
}

export const decodeV08BroadOsAdapterProofEntry = Schema.decodeUnknownSync(V08BroadOsAdapterProofEntrySchema);
export const decodeV08BroadOsAdapterProofReadModel = Schema.decodeUnknownSync(V08BroadOsAdapterProofReadModelSchema);
