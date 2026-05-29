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
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatform,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './reference-primitives';

const NonEmptyReadinessText = Schema.String.pipe(Schema.minLength(1));

export const EnforcementBroadAdapterReadinessIdSchema = NonEmptyReadinessText.pipe(
  Schema.brand('EnforcementBroadAdapterReadinessId')
);
export const EnforcementBroadAdapterReadinessMatrixIdSchema = NonEmptyReadinessText.pipe(
  Schema.brand('EnforcementBroadAdapterReadinessMatrixId')
);
export const EnforcementReadinessClaimBoundarySchema = NonEmptyReadinessText.pipe(
  Schema.brand('EnforcementReadinessClaimBoundary')
);
export const EnforcementReadinessFallbackSchema = NonEmptyReadinessText.pipe(
  Schema.brand('EnforcementReadinessFallback')
);
export const EnforcementReadinessArtifactRequirementSchema = NonEmptyReadinessText.pipe(
  Schema.brand('EnforcementReadinessArtifactRequirement')
);

export const EnforcementBroadAdapterCapabilitySchema = withParser(
  Schema.Literal(
    'owned-process-terminate',
    'app-time-limit',
    'broad-app-blocking',
    'network-domain-blocking',
    'managed-browser-service-command',
    'managed-browser-exact-url-control',
    'unmanaged-browser-process-only',
    'unmanaged-browser-exact-evidence',
    'admin-anti-tamper-rollback'
  )
);

export const EnforcementReadinessStateSchema = withParser(
  Schema.Literal('implemented', 'manual-required', 'unavailable', 'not-claimed')
);

export const EnforcementReadinessProofLevelSchema = withParser(
  Schema.Literal('real-service-proof', 'ci-mechanical-proof', 'manual-proof-required', 'not-proved')
);

export const EnforcementReadinessRuntimeOwnerSchema = withParser(
  Schema.Literal('rust-service', 'os-adapter', 'managed-browser-boundary', 'manual-proof', 'not-implemented')
);

const EnforcementBroadAdapterReadinessEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: EnforcementBroadAdapterReadinessIdSchema,
  capability: EnforcementBroadAdapterCapabilitySchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  readinessState: EnforcementReadinessStateSchema,
  proofLevel: EnforcementReadinessProofLevelSchema,
  runtimeOwner: EnforcementReadinessRuntimeOwnerSchema,
  supportedModes: Schema.Array(EnforcementModeSchema),
  claimBoundary: EnforcementReadinessClaimBoundarySchema,
  fallbackBehavior: EnforcementReadinessFallbackSchema,
  requiredArtifacts: Schema.Array(EnforcementReadinessArtifactRequirementSchema),
  lastCheckedAt: ParentTimestampSchema,
});

type EnforcementBroadAdapterReadinessEntryCandidate = Infer<typeof EnforcementBroadAdapterReadinessEntryBaseSchema>;

export const EnforcementBroadAdapterReadinessEntrySchema = withParser(
  EnforcementBroadAdapterReadinessEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        broadAdapterReadinessEntryIsHonest(entry) ||
        'Expected broad adapter readiness to preserve implemented, manual-required, unavailable, and not-claimed boundaries'
    )
  )
);

export const EnforcementBroadOsAdapterReadinessMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: EnforcementBroadAdapterReadinessMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    entries: Schema.Array(EnforcementBroadAdapterReadinessEntrySchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.entries.map((entry) => entry.readinessId)).size === matrix.entries.length ||
        'Expected broad adapter readiness ids to be unique'
    )
  )
);

function broadAdapterReadinessEntryIsHonest(entry: EnforcementBroadAdapterReadinessEntryCandidate): boolean {
  if (entry.readinessState === 'implemented') {
    return entry.capabilityState === 'supported' && entry.proofLevel !== 'not-proved';
  }

  if (entry.readinessState === 'manual-required') {
    return (
      entry.capabilityState === 'manual-required' &&
      entry.proofLevel === 'manual-proof-required' &&
      entry.requiredArtifacts.length > 0
    );
  }

  if (entry.readinessState === 'unavailable') {
    return entry.capabilityState === 'unavailable' && entry.requiredArtifacts.length > 0;
  }

  return (
    entry.readinessState === 'not-claimed' &&
    entry.proofLevel === 'not-proved' &&
    entry.runtimeOwner === 'not-implemented' &&
    entry.requiredArtifacts.length > 0
  );
}

export type EnforcementBroadAdapterReadinessId = typeof EnforcementBroadAdapterReadinessIdSchema.Type;
export type EnforcementBroadAdapterReadinessMatrixId = typeof EnforcementBroadAdapterReadinessMatrixIdSchema.Type;
export type EnforcementReadinessClaimBoundary = typeof EnforcementReadinessClaimBoundarySchema.Type;
export type EnforcementReadinessFallback = typeof EnforcementReadinessFallbackSchema.Type;
export type EnforcementReadinessArtifactRequirement = typeof EnforcementReadinessArtifactRequirementSchema.Type;
export type EnforcementBroadAdapterCapability = Infer<typeof EnforcementBroadAdapterCapabilitySchema>;
export type EnforcementReadinessState = Infer<typeof EnforcementReadinessStateSchema>;
export type EnforcementReadinessProofLevel = Infer<typeof EnforcementReadinessProofLevelSchema>;
export type EnforcementReadinessRuntimeOwner = Infer<typeof EnforcementReadinessRuntimeOwnerSchema>;
export type EnforcementBroadAdapterReadinessEntry = Infer<typeof EnforcementBroadAdapterReadinessEntrySchema>;
export type EnforcementBroadOsAdapterReadinessMatrix = Infer<typeof EnforcementBroadOsAdapterReadinessMatrixSchema>;

export const EnforcementBroadAdapterCapability = {
  OwnedProcessTerminate: EnforcementBroadAdapterCapabilitySchema.parse('owned-process-terminate'),
  AppTimeLimit: EnforcementBroadAdapterCapabilitySchema.parse('app-time-limit'),
  BroadAppBlocking: EnforcementBroadAdapterCapabilitySchema.parse('broad-app-blocking'),
  NetworkDomainBlocking: EnforcementBroadAdapterCapabilitySchema.parse('network-domain-blocking'),
  ManagedBrowserServiceCommand: EnforcementBroadAdapterCapabilitySchema.parse('managed-browser-service-command'),
  ManagedBrowserExactUrlControl: EnforcementBroadAdapterCapabilitySchema.parse('managed-browser-exact-url-control'),
  UnmanagedBrowserProcessOnly: EnforcementBroadAdapterCapabilitySchema.parse('unmanaged-browser-process-only'),
  UnmanagedBrowserExactEvidence: EnforcementBroadAdapterCapabilitySchema.parse('unmanaged-browser-exact-evidence'),
  AdminAntiTamperRollback: EnforcementBroadAdapterCapabilitySchema.parse('admin-anti-tamper-rollback'),
} as const;

export const EnforcementReadinessState = {
  Implemented: EnforcementReadinessStateSchema.parse('implemented'),
  ManualRequired: EnforcementReadinessStateSchema.parse('manual-required'),
  Unavailable: EnforcementReadinessStateSchema.parse('unavailable'),
  NotClaimed: EnforcementReadinessStateSchema.parse('not-claimed'),
} as const;

export const EnforcementReadinessProofLevel = {
  RealServiceProof: EnforcementReadinessProofLevelSchema.parse('real-service-proof'),
  CiMechanicalProof: EnforcementReadinessProofLevelSchema.parse('ci-mechanical-proof'),
  ManualProofRequired: EnforcementReadinessProofLevelSchema.parse('manual-proof-required'),
  NotProved: EnforcementReadinessProofLevelSchema.parse('not-proved'),
} as const;

export const EnforcementReadinessRuntimeOwner = {
  RustService: EnforcementReadinessRuntimeOwnerSchema.parse('rust-service'),
  OsAdapter: EnforcementReadinessRuntimeOwnerSchema.parse('os-adapter'),
  ManagedBrowserBoundary: EnforcementReadinessRuntimeOwnerSchema.parse('managed-browser-boundary'),
  ManualProof: EnforcementReadinessRuntimeOwnerSchema.parse('manual-proof'),
  NotImplemented: EnforcementReadinessRuntimeOwnerSchema.parse('not-implemented'),
} as const;

const documentedAt = '2026-05-29T16:45:00.000Z';

export const V08BroadOsAdapterReadinessMatrix = EnforcementBroadOsAdapterReadinessMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'v0-8-broad-os-adapter-readiness',
  generatedAt: documentedAt,
  entries: [
    readinessEntry(
      'readiness-owned-process-terminate',
      EnforcementBroadAdapterCapability.OwnedProcessTerminate,
      EnforcementAdapterKind.ProcessControl,
      EnforcementCapabilityState.Supported,
      EnforcementReadinessState.Implemented,
      EnforcementReadinessProofLevel.RealServiceProof,
      EnforcementReadinessRuntimeOwner.OsAdapter,
      [EnforcementMode.TerminateProcess],
      'Only owned-process pid plus expected-process-name termination is proved; this is not global app blocking.',
      'Reject missing pid/name mismatch and return unavailable on unsupported hosts.',
      []
    ),
    readinessEntry(
      'readiness-app-time-limit',
      EnforcementBroadAdapterCapability.AppTimeLimit,
      EnforcementAdapterKind.ProcessControl,
      EnforcementCapabilityState.Supported,
      EnforcementReadinessState.Implemented,
      EnforcementReadinessProofLevel.RealServiceProof,
      EnforcementReadinessRuntimeOwner.RustService,
      [EnforcementMode.TimeLimit],
      'App time-limit proof is tied to owned-process expiration, restart recovery, cancel, expiry, audit, and storage.',
      'Return unavailable when the active timer state or platform adapter cannot support the request.',
      []
    ),
    readinessEntry(
      'readiness-broad-app-blocking',
      EnforcementBroadAdapterCapability.BroadAppBlocking,
      EnforcementAdapterKind.ProcessControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.ManualRequired,
      EnforcementReadinessProofLevel.ManualProofRequired,
      EnforcementReadinessRuntimeOwner.ManualProof,
      [EnforcementMode.BlockProcess],
      'Broad installed-app blocking is not proved by owned-process termination or app time-limit behavior.',
      'Return manual-required or unavailable and avoid an adapter request until OS-approved proof exists.',
      ['OS-approved app/package identity proof', 'installed-app block and rollback proof']
    ),
    readinessEntry(
      'readiness-network-domain-blocking',
      EnforcementBroadAdapterCapability.NetworkDomainBlocking,
      EnforcementAdapterKind.NetworkControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.ManualRequired,
      EnforcementReadinessProofLevel.ManualProofRequired,
      EnforcementReadinessRuntimeOwner.ManualProof,
      [EnforcementMode.TemporaryBlock],
      'Network flow metadata is not decrypted content and does not prove domain blocking enforcement.',
      'Return manual-required or unavailable until a host network control adapter has proof.',
      ['OS network filter adapter proof', 'domain block apply and rollback proof']
    ),
    readinessEntry(
      'readiness-managed-browser-service-command',
      EnforcementBroadAdapterCapability.ManagedBrowserServiceCommand,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.ManualRequired,
      EnforcementReadinessProofLevel.ManualProofRequired,
      EnforcementReadinessRuntimeOwner.ManualProof,
      [EnforcementMode.TemporaryBlock],
      'A managed-browser service-command target string is not exact URL enforcement proof.',
      'Return manual-required or unavailable until managed browser command enforcement proof exists.',
      ['managed-browser command enforcement proof', 'exact URL apply and audit proof']
    ),
    readinessEntry(
      'readiness-managed-browser-exact-url-control',
      EnforcementBroadAdapterCapability.ManagedBrowserExactUrlControl,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.ManualRequired,
      EnforcementReadinessProofLevel.ManualProofRequired,
      EnforcementReadinessRuntimeOwner.ManagedBrowserBoundary,
      [EnforcementMode.TemporaryBlock],
      'Exact URL, active tab, and page-title control require the managed browser boundary.',
      'Keep exact URL control manual-required unless managed browser evidence and enforcement proof are present.',
      ['managed browser active tab proof', 'managed exact URL enforcement artifact']
    ),
    readinessEntry(
      'readiness-unmanaged-browser-process-only',
      EnforcementBroadAdapterCapability.UnmanagedBrowserProcessOnly,
      EnforcementAdapterKind.ProcessControl,
      EnforcementCapabilityState.Supported,
      EnforcementReadinessState.Implemented,
      EnforcementReadinessProofLevel.RealServiceProof,
      EnforcementReadinessRuntimeOwner.OsAdapter,
      [EnforcementMode.TerminateProcess, EnforcementMode.ObserveOnly],
      'Unmanaged browser proof is process-only and cannot become URL, tab, title, download, page, or intent evidence.',
      'Restrict control to pid/name guardrails and preserve exact browser evidence as not-claimed.',
      []
    ),
    readinessEntry(
      'readiness-unmanaged-browser-exact-evidence',
      EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.NotClaimed,
      EnforcementReadinessProofLevel.NotProved,
      EnforcementReadinessRuntimeOwner.NotImplemented,
      [],
      'Unmanaged browser process/window/network evidence does not prove exact URL, active tab, title, download source, page text, HTTPS content, or intent.',
      'Use managed browser or another explicit browser integration before representing exact evidence.',
      ['managed browser or explicit browser integration proof']
    ),
    readinessEntry(
      'readiness-admin-anti-tamper-rollback',
      EnforcementBroadAdapterCapability.AdminAntiTamperRollback,
      EnforcementAdapterKind.ProcessControl,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessState.ManualRequired,
      EnforcementReadinessProofLevel.ManualProofRequired,
      EnforcementReadinessRuntimeOwner.ManualProof,
      [],
      'Admin hardening, anti-tamper, bypass resistance, and broad rollback are not proved by V0.8 adapter tests.',
      'Keep product claims manual-required until real host hardening and rollback evidence exists.',
      ['admin hardening proof', 'anti-tamper proof', 'rollback and bypass-resistance proof']
    ),
  ],
});

function readinessEntry(
  readinessId: string,
  capability: EnforcementBroadAdapterCapability,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  capabilityState: typeof EnforcementCapabilityStateSchema.Type,
  readinessState: EnforcementReadinessState,
  proofLevel: EnforcementReadinessProofLevel,
  runtimeOwner: EnforcementReadinessRuntimeOwner,
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>,
  claimBoundary: string,
  fallbackBehavior: string,
  requiredArtifacts: readonly string[]
): EnforcementBroadAdapterReadinessEntry {
  return EnforcementBroadAdapterReadinessEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessId,
    capability,
    platform: ParentPlatform.Windows,
    adapterKind,
    capabilityState,
    readinessState,
    proofLevel,
    runtimeOwner,
    supportedModes,
    claimBoundary,
    fallbackBehavior,
    requiredArtifacts,
    lastCheckedAt: documentedAt,
  });
}

export const decodeEnforcementBroadAdapterReadinessEntry = Schema.decodeUnknownSync(
  EnforcementBroadAdapterReadinessEntrySchema
);
export const decodeEnforcementBroadOsAdapterReadinessMatrix = Schema.decodeUnknownSync(
  EnforcementBroadOsAdapterReadinessMatrixSchema
);
