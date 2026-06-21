import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  enforcementProofClaimFlagsAreUnset,
  enforcementProofEntriesHaveUniqueField,
} from '@ocentra-parent/schema-domain/enforcement-proof-shape';
import { V08OsAdapterProductProofReadModel } from './enforcement-os-adapter-product-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  type ParentPlatform,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { V08BroadOsAdapterProofReadModel } from './v0-8-broad-os-adapter-proof';
import { V08BrowserDomainAdapterProofReadModel } from '@ocentra-parent/schema-domain/v0-8-browser-domain-adapter-proof';
import { V08OsAdapterManualArtifactGateReadModel } from './v0-8-os-adapter-manual-artifact-gates';

export const V08BroadOsAdapterRuntimeProofReadModelIdSchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofReadModelId');
export const V08BroadOsAdapterRuntimeProofEntryIdSchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofEntryId');
export const V08BroadOsAdapterRuntimeProofReferenceSchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofReference');
export const V08BroadOsAdapterRuntimeProofRequirementSchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofRequirement');
export const V08BroadOsAdapterRuntimeProofBoundarySchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofBoundary');
export const V08BroadOsAdapterRuntimeProofFallbackSchema = brandedNonEmptyStringSchema('V08BroadOsAdapterRuntimeProofFallback');

export const V08BroadOsAdapterRuntimeSurfaceSchema = withParser(
  Schema.Literal(
    'windows-owned-process-and-timer-runtime-boundary',
    'windows-managed-browser-session-runtime-boundary',
    'windows-broad-installed-app-runtime-gate',
    'windows-network-domain-runtime-gate',
    'windows-managed-browser-exact-url-runtime-gate',
    'windows-unmanaged-browser-exact-evidence-runtime-gap',
    'linux-host-runtime-unavailable',
    'macos-host-runtime-manual-gate',
    'android-mobile-runtime-manual-gate',
    'ios-mobile-runtime-manual-gate'
  )
);

export const V08BroadOsAdapterRuntimeClaimStateSchema = withParser(
  Schema.Literal('implemented-boundary', 'manual-required', 'unavailable', 'not-claimed')
);

export const V08BroadOsAdapterRuntimeEvidenceStateSchema = withParser(
  Schema.Literal('composite-runtime-proof', 'manual-artifact-required', 'target-unavailable', 'not-implemented')
);

const V08BroadOsAdapterRuntimeProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08BroadOsAdapterRuntimeProofEntryIdSchema,
  runtimeSurface: V08BroadOsAdapterRuntimeSurfaceSchema,
  platform: ParentPlatformSchema,
  productClaimState: V08BroadOsAdapterRuntimeClaimStateSchema,
  evidenceState: V08BroadOsAdapterRuntimeEvidenceStateSchema,
  sourceProofIds: Schema.Array(V08BroadOsAdapterRuntimeProofReferenceSchema),
  linkedProofCommands: Schema.Array(V08BroadOsAdapterRuntimeProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08BroadOsAdapterRuntimeProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08BroadOsAdapterRuntimeProofRequirementSchema),
  claimBoundary: V08BroadOsAdapterRuntimeProofBoundarySchema,
  fallbackBehavior: V08BroadOsAdapterRuntimeProofFallbackSchema,
  broadInstalledAppBlockingClaimed: Schema.Boolean,
  networkDomainBlockingClaimed: Schema.Boolean,
  managedBrowserExactUrlClaimed: Schema.Boolean,
  unmanagedBrowserExactEvidenceClaimed: Schema.Boolean,
  unsupportedPlatformClaimed: Schema.Boolean,
  mobilePrivilegeClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08BroadOsAdapterRuntimeProofEntryCandidate = Infer<typeof V08BroadOsAdapterRuntimeProofEntryBaseSchema>;

export const V08BroadOsAdapterRuntimeProofEntrySchema = withParser(
  V08BroadOsAdapterRuntimeProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        broadOsAdapterRuntimeProofEntryIsHonest(entry) ||
        'Expected V0.8 broad OS adapter runtime proof entries to preserve implemented-boundary, manual-required, unavailable, and not-claimed states without broad app/domain/browser claim upgrades'
    )
  )
);

export const V08BroadOsAdapterRuntimeProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08BroadOsAdapterRuntimeProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08BroadOsAdapterRuntimeProofReferenceSchema),
    entries: Schema.Array(V08BroadOsAdapterRuntimeProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        enforcementProofEntriesHaveUniqueField(readModel.entries, (entry) => entry.proofEntryId) ||
        'Expected V0.8 broad OS adapter runtime proof entry ids to be unique'
    )
  )
);

function broadOsAdapterRuntimeProofEntryIsHonest(entry: V08BroadOsAdapterRuntimeProofEntryCandidate): boolean {
  if (broadOsAdapterRuntimeProofEntryHasClaimUpgrade(entry)) {
    return false;
  }

  switch (entry.productClaimState) {
    case 'implemented-boundary':
      return (
        entry.platform === 'windows' &&
        entry.evidenceState === 'composite-runtime-proof' &&
        entry.sourceProofIds.length >= 2 &&
        entry.linkedProofArtifacts.length > 0
      );
    case 'manual-required':
      return entry.evidenceState === 'manual-artifact-required' && entry.manualProofRequirements.length > 0;
    case 'unavailable':
      return entry.evidenceState === 'target-unavailable' && entry.manualProofRequirements.length > 0;
    case 'not-claimed':
      return entry.evidenceState === 'not-implemented' && entry.manualProofRequirements.length > 0;
  }
}

function broadOsAdapterRuntimeProofEntryHasClaimUpgrade(entry: V08BroadOsAdapterRuntimeProofEntryCandidate): boolean {
  return !enforcementProofClaimFlagsAreUnset([
    entry.broadInstalledAppBlockingClaimed,
    entry.networkDomainBlockingClaimed,
    entry.managedBrowserExactUrlClaimed,
    entry.unmanagedBrowserExactEvidenceClaimed,
    entry.unsupportedPlatformClaimed,
    entry.mobilePrivilegeClaimed,
  ]);
}

export type V08BroadOsAdapterRuntimeProofReadModelId = typeof V08BroadOsAdapterRuntimeProofReadModelIdSchema.Type;
export type V08BroadOsAdapterRuntimeProofEntryId = typeof V08BroadOsAdapterRuntimeProofEntryIdSchema.Type;
export type V08BroadOsAdapterRuntimeProofReference = typeof V08BroadOsAdapterRuntimeProofReferenceSchema.Type;
export type V08BroadOsAdapterRuntimeProofRequirement = typeof V08BroadOsAdapterRuntimeProofRequirementSchema.Type;
export type V08BroadOsAdapterRuntimeProofBoundary = typeof V08BroadOsAdapterRuntimeProofBoundarySchema.Type;
export type V08BroadOsAdapterRuntimeProofFallback = typeof V08BroadOsAdapterRuntimeProofFallbackSchema.Type;
export type V08BroadOsAdapterRuntimeSurface = Infer<typeof V08BroadOsAdapterRuntimeSurfaceSchema>;
export type V08BroadOsAdapterRuntimeClaimState = Infer<typeof V08BroadOsAdapterRuntimeClaimStateSchema>;
export type V08BroadOsAdapterRuntimeEvidenceState = Infer<typeof V08BroadOsAdapterRuntimeEvidenceStateSchema>;
export type V08BroadOsAdapterRuntimeProofEntry = Infer<typeof V08BroadOsAdapterRuntimeProofEntrySchema>;
export type V08BroadOsAdapterRuntimeProofReadModel = Infer<typeof V08BroadOsAdapterRuntimeProofReadModelSchema>;

type V08BroadOsAdapterRuntimeProofEntryInput = {
  proofEntryId: string;
  runtimeSurface: V08BroadOsAdapterRuntimeSurface;
  platform: ParentPlatform;
  productClaimState: V08BroadOsAdapterRuntimeClaimState;
  evidenceState: V08BroadOsAdapterRuntimeEvidenceState;
  sourceProofIds: readonly string[];
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08BroadOsAdapterRuntimeSurface = {
  WindowsOwnedProcessAndTimerRuntimeBoundary: V08BroadOsAdapterRuntimeSurfaceSchema.parse(
    'windows-owned-process-and-timer-runtime-boundary'
  ),
  WindowsManagedBrowserSessionRuntimeBoundary: V08BroadOsAdapterRuntimeSurfaceSchema.parse(
    'windows-managed-browser-session-runtime-boundary'
  ),
  WindowsBroadInstalledAppRuntimeGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse(
    'windows-broad-installed-app-runtime-gate'
  ),
  WindowsNetworkDomainRuntimeGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse('windows-network-domain-runtime-gate'),
  WindowsManagedBrowserExactUrlRuntimeGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse(
    'windows-managed-browser-exact-url-runtime-gate'
  ),
  WindowsUnmanagedBrowserExactEvidenceRuntimeGap: V08BroadOsAdapterRuntimeSurfaceSchema.parse(
    'windows-unmanaged-browser-exact-evidence-runtime-gap'
  ),
  LinuxHostRuntimeUnavailable: V08BroadOsAdapterRuntimeSurfaceSchema.parse('linux-host-runtime-unavailable'),
  MacosHostRuntimeManualGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse('macos-host-runtime-manual-gate'),
  AndroidMobileRuntimeManualGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse('android-mobile-runtime-manual-gate'),
  IosMobileRuntimeManualGate: V08BroadOsAdapterRuntimeSurfaceSchema.parse('ios-mobile-runtime-manual-gate'),
} as const;

export const V08BroadOsAdapterRuntimeClaimState = {
  ImplementedBoundary: V08BroadOsAdapterRuntimeClaimStateSchema.parse('implemented-boundary'),
  ManualRequired: V08BroadOsAdapterRuntimeClaimStateSchema.parse('manual-required'),
  Unavailable: V08BroadOsAdapterRuntimeClaimStateSchema.parse('unavailable'),
  NotClaimed: V08BroadOsAdapterRuntimeClaimStateSchema.parse('not-claimed'),
} as const;

export const V08BroadOsAdapterRuntimeEvidenceState = {
  CompositeRuntimeProof: V08BroadOsAdapterRuntimeEvidenceStateSchema.parse('composite-runtime-proof'),
  ManualArtifactRequired: V08BroadOsAdapterRuntimeEvidenceStateSchema.parse('manual-artifact-required'),
  TargetUnavailable: V08BroadOsAdapterRuntimeEvidenceStateSchema.parse('target-unavailable'),
  NotImplemented: V08BroadOsAdapterRuntimeEvidenceStateSchema.parse('not-implemented'),
} as const;

const SourceProofIds = {
  BroadOsAdapterProof: String(V08BroadOsAdapterProofReadModel.readModelId),
  BrowserDomainAdapterProof: String(V08BrowserDomainAdapterProofReadModel.readModelId),
  OsAdapterManualArtifactGates: String(V08OsAdapterManualArtifactGateReadModel.readModelId),
  OsAdapterProductProof: String(V08OsAdapterProductProofReadModel.readModelId),
} as const;

export const V08BroadOsAdapterRuntimeProofSourceReadModelIds = Object.values(SourceProofIds);

export const V08BroadOsAdapterRuntimeProofReadModel = V08BroadOsAdapterRuntimeProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-broad-os-adapter-runtime-proof',
  generatedAt: '2026-05-30T22:55:00.000Z',
  sourceReadModelIds: V08BroadOsAdapterRuntimeProofSourceReadModelIds,
  entries: [
    runtimeProofEntry({
      proofEntryId: 'windows-owned-process-and-timer-runtime-boundary',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsOwnedProcessAndTimerRuntimeBoundary,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ImplementedBoundary,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.CompositeRuntimeProof,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.OsAdapterProductProof],
      linkedProofCommands: [
        'node scripts/test/v0-8-broad-os-adapter-proof.mjs',
        'cargo test -p ocentra-parent-agent-service enforcement_os_adapter_product_proof_read_model',
      ],
      linkedProofArtifacts: [
        'test-results/v0-8-broad-os-adapter-proof/proof.json',
        'crates/agent-service/src/enforcement_os_adapter_product_proof_read_model.rs',
      ],
      manualProofRequirements: [],
      claimBoundary:
        'Owned-process pid/name guardrails and app timer lifecycle are runtime boundaries only; they are not broad installed-app blocking.',
      fallbackBehavior:
        'Inputs outside the owned-process or timer boundary remain manual-required or unavailable instead of escalating to broad blocking.',
    }),
    runtimeProofEntry({
      proofEntryId: 'windows-managed-browser-session-runtime-boundary',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsManagedBrowserSessionRuntimeBoundary,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ImplementedBoundary,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.CompositeRuntimeProof,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.BrowserDomainAdapterProof],
      linkedProofCommands: [
        'node scripts/test/v0-8-browser-domain-adapter-proof.mjs',
        'cargo test -p ocentra-parent-agent-service enforcement_browser_domain_adapter_proof_read_model',
      ],
      linkedProofArtifacts: [
        'test-results/v0-8-browser-domain-adapter-proof/proof.json',
        'crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model.rs',
      ],
      manualProofRequirements: [],
      claimBoundary:
        'Managed-browser runtime proof is limited to the owned managed-session intervention state and does not prove exact active-tab URL enforcement.',
      fallbackBehavior:
        'Exact URL control and unmanaged browser evidence stay manual-required or not-claimed until browser integration artifacts exist.',
    }),
    runtimeProofEntry({
      proofEntryId: 'windows-broad-installed-app-runtime-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsBroadInstalledAppRuntimeGate,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-os-adapter-manual-artifact-gates/proof.json'],
      manualProofRequirements: [
        'same app identity proof',
        'host block apply artifact',
        'rollback artifact',
        'audit custody artifact',
      ],
      claimBoundary:
        'Broad installed-app blocking stays manual-required even though owned-process and timer mechanics are proved.',
      fallbackBehavior:
        'The runtime must report manual-required for global app blocking until target host artifacts prove apply and rollback.',
    }),
    runtimeProofEntry({
      proofEntryId: 'windows-network-domain-runtime-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsNetworkDomainRuntimeGate,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BrowserDomainAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-browser-domain-adapter-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-browser-domain-adapter-proof/proof.json'],
      manualProofRequirements: ['host DNS or filter apply artifact', 'rollback artifact', 'audit custody artifact'],
      claimBoundary:
        'Network/domain runtime proof records manual-required and unavailable states only; domain observation is not host blocking.',
      fallbackBehavior:
        'The runtime must return manual-required or unavailable rather than claim a host filter when no adapter artifact exists.',
    }),
    runtimeProofEntry({
      proofEntryId: 'windows-managed-browser-exact-url-runtime-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsManagedBrowserExactUrlRuntimeGate,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BrowserDomainAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-browser-domain-adapter-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-browser-domain-adapter-proof/proof.json'],
      manualProofRequirements: ['active tab artifact', 'exact URL apply artifact', 'rollback artifact'],
      claimBoundary:
        'Managed exact URL blocking remains manual-required and is distinct from managed-session intervention.',
      fallbackBehavior:
        'The runtime exposes the managed-session boundary while leaving exact URL enforcement gated by manual artifacts.',
    }),
    runtimeProofEntry({
      proofEntryId: 'windows-unmanaged-browser-exact-evidence-runtime-gap',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
      platform: 'windows',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.NotClaimed,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.NotImplemented,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.BrowserDomainAdapterProof],
      linkedProofCommands: ['node scripts/test/v0-8-browser-domain-adapter-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-browser-domain-adapter-proof/proof.json'],
      manualProofRequirements: [
        'browser integration artifact for URL, title, page, download, HTTPS content, and intent',
      ],
      claimBoundary:
        'Unmanaged browser exact evidence is not claimed; process terminate and warn boundaries do not prove URL or page certainty.',
      fallbackBehavior:
        'The runtime may terminate or warn by process boundary only and must keep exact unmanaged evidence not-claimed.',
    }),
    runtimeProofEntry({
      proofEntryId: 'linux-host-runtime-unavailable',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.LinuxHostRuntimeUnavailable,
      platform: 'linux',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.Unavailable,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.TargetUnavailable,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.BrowserDomainAdapterProof],
      linkedProofCommands: ['node scripts/test/v0-8-broad-os-adapter-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-broad-os-adapter-proof/proof.json'],
      manualProofRequirements: [
        'Linux service manager, package identity, permission, apply, rollback, and audit artifacts',
      ],
      claimBoundary:
        'Linux host OS adapter support is unavailable in this final pass and cannot inherit Windows proof.',
      fallbackBehavior:
        'Linux targets must report unavailable or manual-required platform states until a target adapter proves support.',
    }),
    runtimeProofEntry({
      proofEntryId: 'macos-host-runtime-manual-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.MacosHostRuntimeManualGate,
      platform: 'macos',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-os-adapter-manual-artifact-gates/proof.json'],
      manualProofRequirements: ['macOS permission, package, service, apply, rollback, and audit artifacts'],
      claimBoundary: 'macOS host support stays manual-required until target-specific artifacts exist.',
      fallbackBehavior: 'macOS targets must not reuse Windows runtime proof for host enforcement claims.',
    }),
    runtimeProofEntry({
      proofEntryId: 'android-mobile-runtime-manual-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.AndroidMobileRuntimeManualGate,
      platform: 'android',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-os-adapter-manual-artifact-gates/proof.json'],
      manualProofRequirements: [
        'device-owner or managed-profile artifact',
        'UsageStats artifact',
        'accessibility or VPN/DNS artifact',
        'package lifecycle artifact',
      ],
      claimBoundary: 'Android child enforcement remains manual-required and is not proved by host OS adapters.',
      fallbackBehavior:
        'Android targets keep privileged mobile states manual-required until real device policy artifacts exist.',
    }),
    runtimeProofEntry({
      proofEntryId: 'ios-mobile-runtime-manual-gate',
      runtimeSurface: V08BroadOsAdapterRuntimeSurface.IosMobileRuntimeManualGate,
      platform: 'ios',
      productClaimState: V08BroadOsAdapterRuntimeClaimState.ManualRequired,
      evidenceState: V08BroadOsAdapterRuntimeEvidenceState.ManualArtifactRequired,
      sourceProofIds: [SourceProofIds.BroadOsAdapterProof, SourceProofIds.OsAdapterManualArtifactGates],
      linkedProofCommands: ['node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-os-adapter-manual-artifact-gates/proof.json'],
      manualProofRequirements: [
        'Family Controls entitlement artifact',
        'DeviceActivity artifact',
        'Network Extension artifact',
        'signing and TestFlight device artifact',
      ],
      claimBoundary: 'iOS child enforcement remains manual-required and is not proved by Windows host runtime proof.',
      fallbackBehavior:
        'iOS targets keep entitlement and device states manual-required until Apple-approved artifacts exist.',
    }),
  ],
});

function runtimeProofEntry(input: V08BroadOsAdapterRuntimeProofEntryInput): V08BroadOsAdapterRuntimeProofEntry {
  return V08BroadOsAdapterRuntimeProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofEntryId: input.proofEntryId,
    runtimeSurface: input.runtimeSurface,
    platform: input.platform,
    productClaimState: input.productClaimState,
    evidenceState: input.evidenceState,
    sourceProofIds: input.sourceProofIds,
    linkedProofCommands: input.linkedProofCommands,
    linkedProofArtifacts: input.linkedProofArtifacts,
    manualProofRequirements: input.manualProofRequirements,
    claimBoundary: input.claimBoundary,
    fallbackBehavior: input.fallbackBehavior,
    broadInstalledAppBlockingClaimed: false,
    networkDomainBlockingClaimed: false,
    managedBrowserExactUrlClaimed: false,
    unmanagedBrowserExactEvidenceClaimed: false,
    unsupportedPlatformClaimed: false,
    mobilePrivilegeClaimed: false,
    lastCheckedAt: '2026-05-30T22:55:00.000Z',
  });
}

