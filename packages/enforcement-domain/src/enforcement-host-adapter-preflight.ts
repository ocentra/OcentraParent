import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  EnforcementAdapterKind,
  EnforcementAdapterKindSchema,
  EnforcementCapabilityState,
  EnforcementCapabilityStateSchema,
} from './enforcement';
import {
  EnforcementBroadAdapterCapability,
  EnforcementBroadAdapterCapabilitySchema,
  EnforcementBroadAdapterReadinessIdSchema,
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

export const EnforcementHostAdapterPreflightIdSchema = brandedNonEmptyStringSchema('EnforcementHostAdapterPreflightId');
export const EnforcementHostAdapterPreflightMatrixIdSchema = brandedNonEmptyStringSchema('EnforcementHostAdapterPreflightMatrixId');
export const EnforcementHostAdapterEvidenceRequirementSchema = brandedNonEmptyStringSchema('EnforcementHostAdapterEvidenceRequirement');
export const EnforcementHostAdapterManualStepSchema = brandedNonEmptyStringSchema('EnforcementHostAdapterManualStep');
export const EnforcementHostAdapterRejectionReasonSchema = brandedNonEmptyStringSchema('EnforcementHostAdapterRejectionReason');

export const EnforcementHostAdapterPreflightGateSchema = withParser(
  Schema.Literal(
    'process-package-identity',
    'host-network-filter',
    'managed-browser-boundary',
    'explicit-browser-integration',
    'rollback-anti-tamper'
  )
);

export const EnforcementHostAdapterPreflightStatusSchema = withParser(
  Schema.Literal('blocked-by-missing-artifact', 'not-claimable-from-current-proof')
);

export const EnforcementHostAdapterProductClaimStateSchema = withParser(
  Schema.Literal('manual-required', 'unavailable', 'not-claimed')
);

const EnforcementHostAdapterPreflightEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  preflightId: EnforcementHostAdapterPreflightIdSchema,
  readinessId: EnforcementBroadAdapterReadinessIdSchema,
  capability: EnforcementBroadAdapterCapabilitySchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  existingReadinessState: EnforcementReadinessStateSchema,
  productClaimState: EnforcementHostAdapterProductClaimStateSchema,
  proofLevel: EnforcementReadinessProofLevelSchema,
  runtimeOwner: EnforcementReadinessRuntimeOwnerSchema,
  preflightGate: EnforcementHostAdapterPreflightGateSchema,
  preflightStatus: EnforcementHostAdapterPreflightStatusSchema,
  claimBoundary: EnforcementHostAdapterRejectionReasonSchema,
  prerequisite: EnforcementHostAdapterEvidenceRequirementSchema,
  requiredEvidenceArtifacts: Schema.Array(EnforcementHostAdapterEvidenceRequirementSchema),
  manualProofSteps: Schema.Array(EnforcementHostAdapterManualStepSchema),
  unsafeUpgradeExamples: Schema.Array(EnforcementHostAdapterRejectionReasonSchema),
  fallbackBehavior: EnforcementHostAdapterRejectionReasonSchema,
  lastCheckedAt: ParentTimestampSchema,
});

type EnforcementHostAdapterPreflightEntryCandidate = Infer<typeof EnforcementHostAdapterPreflightEntryBaseSchema>;

export const EnforcementHostAdapterPreflightEntrySchema = withParser(
  EnforcementHostAdapterPreflightEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        hostAdapterPreflightEntryIsHonest(entry) ||
        'Expected host adapter preflight to keep broad app, domain, browser, rollback, and exact evidence claims manual-required or not-claimed'
    )
  )
);

export const EnforcementHostAdapterPreflightMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: EnforcementHostAdapterPreflightMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    entries: Schema.Array(EnforcementHostAdapterPreflightEntrySchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.entries.map((entry) => entry.preflightId)).size === matrix.entries.length ||
        'Expected host adapter preflight ids to be unique'
    )
  )
);

function hostAdapterPreflightEntryIsHonest(entry: EnforcementHostAdapterPreflightEntryCandidate): boolean {
  const hasManualProof = entry.requiredEvidenceArtifacts.length > 0 && entry.manualProofSteps.length > 0;
  const hasUnsafeExamples = entry.unsafeUpgradeExamples.length > 0;

  if (entry.capability === 'unmanaged-browser-exact-evidence') {
    return exactBrowserEvidencePreflightIsHonest(entry, hasManualProof, hasUnsafeExamples);
  }

  if (entry.productClaimState === 'not-claimed') {
    return notClaimedPreflightIsHonest(entry, hasManualProof, hasUnsafeExamples);
  }

  return manualPreflightIsHonest(entry, hasManualProof, hasUnsafeExamples);
}

function exactBrowserEvidencePreflightIsHonest(
  entry: EnforcementHostAdapterPreflightEntryCandidate,
  hasManualProof: boolean,
  hasUnsafeExamples: boolean
): boolean {
  return (
    entry.productClaimState === 'not-claimed' && notClaimedPreflightIsHonest(entry, hasManualProof, hasUnsafeExamples)
  );
}

function notClaimedPreflightIsHonest(
  entry: EnforcementHostAdapterPreflightEntryCandidate,
  hasManualProof: boolean,
  hasUnsafeExamples: boolean
): boolean {
  return (
    entry.preflightStatus === 'not-claimable-from-current-proof' &&
    entry.proofLevel === 'not-proved' &&
    entry.runtimeOwner === 'not-implemented' &&
    hasManualProof &&
    hasUnsafeExamples
  );
}

function manualPreflightIsHonest(
  entry: EnforcementHostAdapterPreflightEntryCandidate,
  hasManualProof: boolean,
  hasUnsafeExamples: boolean
): boolean {
  return (
    entry.preflightStatus === 'blocked-by-missing-artifact' &&
    entry.proofLevel === 'manual-proof-required' &&
    entry.runtimeOwner !== 'not-implemented' &&
    hasManualProof &&
    hasUnsafeExamples
  );
}

export type EnforcementHostAdapterPreflightId = typeof EnforcementHostAdapterPreflightIdSchema.Type;
export type EnforcementHostAdapterPreflightMatrixId = typeof EnforcementHostAdapterPreflightMatrixIdSchema.Type;
export type EnforcementHostAdapterEvidenceRequirement = typeof EnforcementHostAdapterEvidenceRequirementSchema.Type;
export type EnforcementHostAdapterManualStep = typeof EnforcementHostAdapterManualStepSchema.Type;
export type EnforcementHostAdapterRejectionReason = typeof EnforcementHostAdapterRejectionReasonSchema.Type;
export type EnforcementHostAdapterPreflightGate = Infer<typeof EnforcementHostAdapterPreflightGateSchema>;
export type EnforcementHostAdapterPreflightStatus = Infer<typeof EnforcementHostAdapterPreflightStatusSchema>;
export type EnforcementHostAdapterProductClaimState = Infer<typeof EnforcementHostAdapterProductClaimStateSchema>;
export type EnforcementHostAdapterPreflightEntry = Infer<typeof EnforcementHostAdapterPreflightEntrySchema>;
export type EnforcementHostAdapterPreflightMatrix = Infer<typeof EnforcementHostAdapterPreflightMatrixSchema>;

export const EnforcementHostAdapterPreflightGate = {
  ProcessPackageIdentity: EnforcementHostAdapterPreflightGateSchema.parse('process-package-identity'),
  HostNetworkFilter: EnforcementHostAdapterPreflightGateSchema.parse('host-network-filter'),
  ManagedBrowserBoundary: EnforcementHostAdapterPreflightGateSchema.parse('managed-browser-boundary'),
  ExplicitBrowserIntegration: EnforcementHostAdapterPreflightGateSchema.parse('explicit-browser-integration'),
  RollbackAntiTamper: EnforcementHostAdapterPreflightGateSchema.parse('rollback-anti-tamper'),
} as const;

export const EnforcementHostAdapterPreflightStatus = {
  BlockedByMissingArtifact: EnforcementHostAdapterPreflightStatusSchema.parse('blocked-by-missing-artifact'),
  NotClaimableFromCurrentProof: EnforcementHostAdapterPreflightStatusSchema.parse('not-claimable-from-current-proof'),
} as const;

export const EnforcementHostAdapterProductClaimState = {
  ManualRequired: EnforcementHostAdapterProductClaimStateSchema.parse('manual-required'),
  Unavailable: EnforcementHostAdapterProductClaimStateSchema.parse('unavailable'),
  NotClaimed: EnforcementHostAdapterProductClaimStateSchema.parse('not-claimed'),
} as const;

const documentedAt = '2026-05-29T17:45:00.000Z';

export const V08HostAdapterProofPreflightMatrix = EnforcementHostAdapterPreflightMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'v0-8-host-adapter-proof-preflight',
  generatedAt: documentedAt,
  entries: [
    preflightEntry(
      'preflight-broad-app-process-package-identity',
      'readiness-broad-app-blocking',
      EnforcementBroadAdapterCapability.BroadAppBlocking,
      EnforcementAdapterKind.ProcessControl,
      EnforcementHostAdapterPreflightGate.ProcessPackageIdentity,
      EnforcementReadinessRuntimeOwner.ManualProof,
      'Broad app blocking requires OS-approved process/package identity before any block claim can upgrade.',
      'Package identifier, executable path, publisher/signature, process lineage, and installed app inventory must agree.',
      [
        'OS-approved package identifier evidence',
        'executable path and publisher or signature evidence',
        'installed app inventory and process lineage evidence',
        'block apply and rollback evidence for the same app identity',
      ],
      [
        'Run the proof on the target Windows child host from the current commit.',
        'Record package identity, process lineage, block apply result, rollback result, and audit event ids.',
      ],
      [
        'Owned-process pid termination presented as global installed app blocking.',
        'A process name alone presented as durable package identity.',
      ],
      'Keep broad app blocking manual-required or unavailable and route only owned-process actions to proved adapters.'
    ),
    preflightEntry(
      'preflight-network-domain-filter',
      'readiness-network-domain-blocking',
      EnforcementBroadAdapterCapability.NetworkDomainBlocking,
      EnforcementAdapterKind.NetworkControl,
      EnforcementHostAdapterPreflightGate.HostNetworkFilter,
      EnforcementReadinessRuntimeOwner.ManualProof,
      'Network/domain blocking requires a host network filter or DNS/VPN adapter proof and cannot be inferred from metadata.',
      'Domain decision, adapter install state, privilege state, apply result, rollback result, and no decrypted payload capture must be recorded.',
      [
        'host network filter or DNS/VPN adapter evidence',
        'domain block apply and rollback evidence',
        'privilege and permission state evidence',
        'metadata-only custody evidence with no decrypted HTTPS content',
      ],
      [
        'Run the domain block proof on a real host with normal network activity.',
        'Record adapter state, domain target, apply result, rollback result, service logs, and audit ids.',
      ],
      [
        'Network flow metadata presented as decrypted content or user intent.',
        'A domain string in a policy payload presented as applied network blocking.',
      ],
      'Return manual-required or unavailable until a real host network adapter proves apply and rollback.'
    ),
    preflightEntry(
      'preflight-managed-browser-service-command',
      'readiness-managed-browser-service-command',
      EnforcementBroadAdapterCapability.ManagedBrowserServiceCommand,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementHostAdapterPreflightGate.ManagedBrowserBoundary,
      EnforcementReadinessRuntimeOwner.ManualProof,
      'Managed-browser service commands require command enforcement proof and are not exact URL proof by themselves.',
      'Managed session id, bridge state, command request, command result, active document evidence, and audit ids must be recorded.',
      [
        'managed-browser command enforcement evidence',
        'managed session and bridge state evidence',
        'active document or tab evidence from the managed browser boundary',
        'command audit and rejection evidence',
      ],
      [
        'Run managed browser proof through the service command path.',
        'Record managed session id, bridge state, command result, active document evidence, and audit ids.',
      ],
      [
        'A service-command target URL presented as active browser evidence.',
        'A managed browser launch plan presented as enforcement success.',
      ],
      'Keep managed-browser service commands manual-required unless the managed boundary proves command enforcement.'
    ),
    preflightEntry(
      'preflight-managed-browser-exact-url',
      'readiness-managed-browser-exact-url-control',
      EnforcementBroadAdapterCapability.ManagedBrowserExactUrlControl,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementHostAdapterPreflightGate.ManagedBrowserBoundary,
      EnforcementReadinessRuntimeOwner.ManagedBrowserBoundary,
      'Exact URL control requires managed browser active-tab evidence plus enforcement proof from that same boundary.',
      'Active URL, title/domain, tab state, source id, adapter id, command result, and evidence freshness must be recorded.',
      [
        'managed browser active URL and tab evidence',
        'managed exact URL enforcement evidence',
        'evidence freshness and custody evidence',
        'URL command apply and rollback or rejection evidence',
      ],
      [
        'Run the exact URL proof inside a managed browser session.',
        'Record active URL, tab/title state, command result, rollback or rejection state, and evidence ids.',
      ],
      [
        'A manually entered URL presented as active tab proof.',
        'Unmanaged browser process/window evidence presented as exact URL control.',
      ],
      'Keep exact URL control manual-required until managed browser evidence and command enforcement are tied together.'
    ),
    preflightEntry(
      'preflight-unmanaged-browser-exact-evidence',
      'readiness-unmanaged-browser-exact-evidence',
      EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence,
      EnforcementAdapterKind.ManagedBrowserControl,
      EnforcementHostAdapterPreflightGate.ExplicitBrowserIntegration,
      EnforcementReadinessRuntimeOwner.NotImplemented,
      'Unmanaged browser exact URL, tab, title, download, page, HTTPS content, and intent claims are not claimable from process/window/network evidence.',
      'A managed browser boundary or explicit browser integration must produce exact evidence before representation.',
      [
        'managed browser or explicit browser integration evidence',
        'active URL, tab, title/domain, and source adapter evidence',
        'freshness, permission, and custody evidence',
      ],
      [
        'Install or enable an explicit browser integration on the real host.',
        'Record exact browser evidence through that integration and reject process-only substitutes.',
      ],
      [
        'Browser process name presented as active URL evidence.',
        'Network metadata presented as page text, HTTPS content, download source, or user intent.',
      ],
      'Represent unmanaged browser evidence as process-only or possible bypass until exact integration proof exists.',
      EnforcementHostAdapterPreflightStatus.NotClaimableFromCurrentProof,
      EnforcementHostAdapterProductClaimState.NotClaimed,
      EnforcementReadinessProofLevel.NotProved,
      EnforcementReadinessState.NotClaimed
    ),
    preflightEntry(
      'preflight-admin-rollback-anti-tamper',
      'readiness-admin-anti-tamper-rollback',
      EnforcementBroadAdapterCapability.AdminAntiTamperRollback,
      EnforcementAdapterKind.ProcessControl,
      EnforcementHostAdapterPreflightGate.RollbackAntiTamper,
      EnforcementReadinessRuntimeOwner.ManualProof,
      'Admin hardening, anti-tamper, rollback, and bypass resistance require real host evidence before product claims upgrade.',
      'Admin state, service install state, tamper attempt, rollback token, rollback result, and bypass-resistance artifacts must be recorded.',
      [
        'admin hardening and service install evidence',
        'anti-tamper attempt and result evidence',
        'rollback token and rollback result evidence',
        'bypass-resistance and audit custody evidence',
      ],
      [
        'Run hardening and rollback proof on a real host with the installed service.',
        'Record admin state, tamper attempt, rollback result, bypass-resistance result, and audit ids.',
      ],
      [
        'A dev-service unavailable result presented as anti-tamper proof.',
        'A rollback token field presented as completed rollback behavior.',
      ],
      'Keep admin hardening, anti-tamper, rollback, and bypass resistance manual-required until real host artifacts exist.'
    ),
  ],
});

function preflightEntry(
  preflightId: string,
  readinessId: string,
  capability: typeof EnforcementBroadAdapterCapabilitySchema.Type,
  adapterKind: typeof EnforcementAdapterKindSchema.Type,
  preflightGate: EnforcementHostAdapterPreflightGate,
  runtimeOwner: typeof EnforcementReadinessRuntimeOwnerSchema.Type,
  claimBoundary: string,
  prerequisite: string,
  requiredEvidenceArtifacts: readonly string[],
  manualProofSteps: readonly string[],
  unsafeUpgradeExamples: readonly string[],
  fallbackBehavior: string,
  preflightStatus: EnforcementHostAdapterPreflightStatus = EnforcementHostAdapterPreflightStatus.BlockedByMissingArtifact,
  productClaimState: EnforcementHostAdapterProductClaimState = EnforcementHostAdapterProductClaimState.ManualRequired,
  proofLevel: typeof EnforcementReadinessProofLevelSchema.Type = EnforcementReadinessProofLevel.ManualProofRequired,
  existingReadinessState: typeof EnforcementReadinessStateSchema.Type = EnforcementReadinessState.ManualRequired
): EnforcementHostAdapterPreflightEntry {
  return EnforcementHostAdapterPreflightEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preflightId,
    readinessId,
    capability,
    platform: ParentPlatform.Windows,
    adapterKind,
    capabilityState: EnforcementCapabilityState.ManualRequired,
    existingReadinessState,
    productClaimState,
    proofLevel,
    runtimeOwner,
    preflightGate,
    preflightStatus,
    claimBoundary,
    prerequisite,
    requiredEvidenceArtifacts,
    manualProofSteps,
    unsafeUpgradeExamples,
    fallbackBehavior,
    lastCheckedAt: documentedAt,
  });
}

export const decodeEnforcementHostAdapterPreflightEntry = Schema.decodeUnknownSync(
  EnforcementHostAdapterPreflightEntrySchema
);
export const decodeEnforcementHostAdapterPreflightMatrix = Schema.decodeUnknownSync(
  EnforcementHostAdapterPreflightMatrixSchema
);

