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
  EnforcementReadinessProofLevel,
  EnforcementReadinessProofLevelSchema,
  EnforcementReadinessRuntimeOwner,
  EnforcementReadinessRuntimeOwnerSchema,
} from './enforcement-readiness';
import {
  EnforcementHostAdapterPreflightGate,
  EnforcementHostAdapterPreflightIdSchema,
} from './enforcement-host-adapter-preflight';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatform,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

export const EnforcementProcessPackageIdentityBridgeIdSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageIdentityBridgeId');
export const EnforcementProcessPackageIdentityMatrixIdSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageIdentityMatrixId');
export const EnforcementProcessPackageEvidenceRequirementSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageEvidenceRequirement');
export const EnforcementProcessPackageManualStepSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageManualStep');
export const EnforcementProcessPackageAcceptanceSignalSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageAcceptanceSignal');
export const EnforcementProcessPackageUnsafeUpgradeExampleSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageUnsafeUpgradeExample');
export const EnforcementProcessPackageFallbackSchema = brandedNonEmptyStringSchema('EnforcementProcessPackageFallback');

export const EnforcementProcessPackageProofPointSchema = withParser(
  Schema.Literal(
    'installed-app-inventory',
    'process-lineage',
    'executable-identity',
    'package-identity',
    'publisher-signature',
    'inventory-process-link',
    'unsupported-identity',
    'rollback-readiness',
    'audit-custody'
  )
);

export const EnforcementProcessPackageEvidenceClassSchema = withParser(
  Schema.Literal('inventory', 'process', 'executable', 'package', 'publisher-signature', 'rollback', 'audit')
);

export const EnforcementProcessPackageBridgeStateSchema = withParser(
  Schema.Literal('manual-required', 'unavailable', 'not-claimed')
);

const EnforcementProcessPackageIdentityBridgeEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  bridgeId: EnforcementProcessPackageIdentityBridgeIdSchema,
  proofPoint: EnforcementProcessPackageProofPointSchema,
  evidenceClass: EnforcementProcessPackageEvidenceClassSchema,
  capability: EnforcementBroadAdapterCapabilitySchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  bridgeState: EnforcementProcessPackageBridgeStateSchema,
  proofLevel: EnforcementReadinessProofLevelSchema,
  runtimeOwner: EnforcementReadinessRuntimeOwnerSchema,
  preflightIds: Schema.Array(EnforcementHostAdapterPreflightIdSchema),
  preflightGate: Schema.Literal(EnforcementHostAdapterPreflightGate.ProcessPackageIdentity),
  hostEvidenceRequirement: EnforcementProcessPackageEvidenceRequirementSchema,
  requiredEvidenceArtifacts: Schema.Array(EnforcementProcessPackageEvidenceRequirementSchema),
  manualProofSteps: Schema.Array(EnforcementProcessPackageManualStepSchema),
  acceptanceSignals: Schema.Array(EnforcementProcessPackageAcceptanceSignalSchema),
  unsafeUpgradeExamples: Schema.Array(EnforcementProcessPackageUnsafeUpgradeExampleSchema),
  fallbackBehavior: EnforcementProcessPackageFallbackSchema,
  lastCheckedAt: ParentTimestampSchema,
});

type EnforcementProcessPackageIdentityBridgeEntryCandidate = Infer<
  typeof EnforcementProcessPackageIdentityBridgeEntryBaseSchema
>;

export const EnforcementProcessPackageIdentityBridgeEntrySchema = withParser(
  EnforcementProcessPackageIdentityBridgeEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        processPackageIdentityBridgeEntryIsHonest(entry) ||
        'Expected process/package identity bridge entries to stay manual-required, unavailable, or not-claimed until real Windows artifacts exist'
    )
  )
);

export const EnforcementProcessPackageIdentityMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: EnforcementProcessPackageIdentityMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    entries: Schema.Array(EnforcementProcessPackageIdentityBridgeEntrySchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.entries.map((entry) => entry.bridgeId)).size === matrix.entries.length ||
        'Expected process/package identity bridge ids to be unique'
    )
  )
);

function processPackageIdentityBridgeEntryIsHonest(
  entry: EnforcementProcessPackageIdentityBridgeEntryCandidate
): boolean {
  if (!proofPointBridgeStateIsAllowed(entry)) {
    return false;
  }

  if (entry.bridgeState === 'not-claimed') {
    return notClaimedBridgeEntryIsHonest(entry);
  }

  if (entry.bridgeState === 'unavailable') {
    return unavailableBridgeEntryIsHonest(entry);
  }

  return manualBridgeEntryIsHonest(entry);
}

function proofPointBridgeStateIsAllowed(entry: EnforcementProcessPackageIdentityBridgeEntryCandidate): boolean {
  return (
    (entry.proofPoint !== 'rollback-readiness' || entry.bridgeState === 'not-claimed') &&
    (entry.proofPoint !== 'unsupported-identity' || entry.bridgeState === 'unavailable')
  );
}

function notClaimedBridgeEntryIsHonest(entry: EnforcementProcessPackageIdentityBridgeEntryCandidate): boolean {
  return (
    entry.proofLevel === 'not-proved' &&
    entry.runtimeOwner === 'not-implemented' &&
    entry.capabilityState === 'manual-required' &&
    bridgeEntryHasRequiredProofDetails(entry)
  );
}

function unavailableBridgeEntryIsHonest(entry: EnforcementProcessPackageIdentityBridgeEntryCandidate): boolean {
  return (
    entry.capabilityState === 'unavailable' &&
    entry.proofLevel === 'manual-proof-required' &&
    entry.runtimeOwner === 'manual-proof' &&
    bridgeEntryHasRequiredProofDetails(entry)
  );
}

function manualBridgeEntryIsHonest(entry: EnforcementProcessPackageIdentityBridgeEntryCandidate): boolean {
  return (
    entry.capabilityState === 'manual-required' &&
    entry.proofLevel === 'manual-proof-required' &&
    entry.runtimeOwner === 'manual-proof' &&
    bridgeEntryHasRequiredProofDetails(entry)
  );
}

function bridgeEntryHasRequiredProofDetails(entry: EnforcementProcessPackageIdentityBridgeEntryCandidate): boolean {
  return (
    entry.requiredEvidenceArtifacts.length > 0 &&
    entry.manualProofSteps.length > 0 &&
    entry.acceptanceSignals.length > 0 &&
    entry.unsafeUpgradeExamples.length > 0
  );
}

export type EnforcementProcessPackageIdentityBridgeId = typeof EnforcementProcessPackageIdentityBridgeIdSchema.Type;
export type EnforcementProcessPackageIdentityMatrixId = typeof EnforcementProcessPackageIdentityMatrixIdSchema.Type;
export type EnforcementProcessPackageEvidenceRequirement =
  typeof EnforcementProcessPackageEvidenceRequirementSchema.Type;
export type EnforcementProcessPackageManualStep = typeof EnforcementProcessPackageManualStepSchema.Type;
export type EnforcementProcessPackageAcceptanceSignal = typeof EnforcementProcessPackageAcceptanceSignalSchema.Type;
export type EnforcementProcessPackageUnsafeUpgradeExample =
  typeof EnforcementProcessPackageUnsafeUpgradeExampleSchema.Type;
export type EnforcementProcessPackageFallback = typeof EnforcementProcessPackageFallbackSchema.Type;
export type EnforcementProcessPackageProofPoint = Infer<typeof EnforcementProcessPackageProofPointSchema>;
export type EnforcementProcessPackageEvidenceClass = Infer<typeof EnforcementProcessPackageEvidenceClassSchema>;
export type EnforcementProcessPackageBridgeState = Infer<typeof EnforcementProcessPackageBridgeStateSchema>;
export type EnforcementProcessPackageIdentityBridgeEntry = Infer<
  typeof EnforcementProcessPackageIdentityBridgeEntrySchema
>;
export type EnforcementProcessPackageIdentityMatrix = Infer<typeof EnforcementProcessPackageIdentityMatrixSchema>;

export const EnforcementProcessPackageProofPoint = {
  InstalledAppInventory: EnforcementProcessPackageProofPointSchema.parse('installed-app-inventory'),
  ProcessLineage: EnforcementProcessPackageProofPointSchema.parse('process-lineage'),
  ExecutableIdentity: EnforcementProcessPackageProofPointSchema.parse('executable-identity'),
  PackageIdentity: EnforcementProcessPackageProofPointSchema.parse('package-identity'),
  PublisherSignature: EnforcementProcessPackageProofPointSchema.parse('publisher-signature'),
  InventoryProcessLink: EnforcementProcessPackageProofPointSchema.parse('inventory-process-link'),
  UnsupportedIdentity: EnforcementProcessPackageProofPointSchema.parse('unsupported-identity'),
  RollbackReadiness: EnforcementProcessPackageProofPointSchema.parse('rollback-readiness'),
  AuditCustody: EnforcementProcessPackageProofPointSchema.parse('audit-custody'),
} as const;

export const EnforcementProcessPackageBridgeState = {
  ManualRequired: EnforcementProcessPackageBridgeStateSchema.parse('manual-required'),
  Unavailable: EnforcementProcessPackageBridgeStateSchema.parse('unavailable'),
  NotClaimed: EnforcementProcessPackageBridgeStateSchema.parse('not-claimed'),
} as const;

const documentedAt = '2026-05-29T18:35:00.000Z';
const processPackagePreflightId = 'preflight-broad-app-process-package-identity';

export const V08ProcessPackageIdentityProofBridgeMatrix = EnforcementProcessPackageIdentityMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'v0-8-process-package-identity-proof-bridge',
  generatedAt: documentedAt,
  entries: [
    bridgeEntry(
      'process-package-installed-app-inventory',
      EnforcementProcessPackageProofPoint.InstalledAppInventory,
      'inventory',
      'Installed app inventory must come from a real Windows host source before broad app blocking can target it.',
      [
        'Windows installed app inventory source and timestamp',
        'package or executable identity for each inventory row',
        'source adapter id and permission state for the inventory read',
      ],
      [
        'Run the inventory proof on the target Windows child host from the current commit.',
        'Record inventory source, adapter id, permission state, and the raw evidence ref ids kept by the local service.',
      ],
      [
        'Inventory row has a stable package id or explicit unpackaged executable identity.',
        'Inventory evidence timestamp and adapter id are present.',
      ],
      [
        'A parent-entered app name presented as installed app inventory.',
        'A process name promoted into a durable installed application target.',
      ],
      'Keep broad app targets manual-required when installed inventory is missing or stale.'
    ),
    bridgeEntry(
      'process-package-process-lineage',
      EnforcementProcessPackageProofPoint.ProcessLineage,
      'process',
      'Process lineage must identify pid, parent pid when available, executable path, start time, and observation source.',
      [
        'process id and parent process id when available',
        'executable path and process start timestamp',
        'observation adapter id, freshness, and custody evidence',
      ],
      [
        'Observe the running process through the host adapter.',
        'Record pid, parent pid when visible, executable path, start time, adapter id, and evidence freshness.',
      ],
      [
        'Process lineage is tied to a current evidence ref.',
        'Missing parent process data is represented as unavailable rather than invented.',
      ],
      [
        'A process name alone presented as lineage.',
        'A stale pid reused after process exit presented as the active child process.',
      ],
      'Treat unknown or stale lineage as unavailable for broad app identity matching.'
    ),
    bridgeEntry(
      'process-package-executable-identity',
      EnforcementProcessPackageProofPoint.ExecutableIdentity,
      'executable',
      'Executable identity must include canonical path plus a host-derived fingerprint before it can support app identity.',
      [
        'canonical executable path',
        'file fingerprint or version metadata from the host',
        'path normalization and custody evidence',
      ],
      [
        'Read executable metadata from the real host process path.',
        'Record canonical path, fingerprint or version metadata, and custody evidence.',
      ],
      [
        'Executable identity is stable across one inventory/process join.',
        'Fingerprint unavailable states remain typed and do not become proof.',
      ],
      [
        'A display name presented as executable identity.',
        'A mutable shortcut path presented as the executable being controlled.',
      ],
      'Use unavailable or manual-required when canonical path or fingerprint evidence cannot be collected.'
    ),
    bridgeEntry(
      'process-package-package-identity',
      EnforcementProcessPackageProofPoint.PackageIdentity,
      'package',
      'Package identity must distinguish packaged apps from unpackaged Win32 executables without silently upgrading unknown apps.',
      [
        'package family name, product id, or explicit unpackaged identity',
        'inventory source that produced the package or executable identity',
        'unknown or unpackaged status when package metadata is unavailable',
      ],
      [
        'Query the real host package or inventory source.',
        'Record package family name, product id, unpackaged identity, or typed unknown state with source ids.',
      ],
      [
        'Packaged and unpackaged identities are represented separately.',
        'Unknown app identity remains unknown until a supported source proves it.',
      ],
      [
        'An unknown executable silently promoted to a known app.',
        'A launcher process presented as the child app package without supporting inventory evidence.',
      ],
      'Keep the app target manual-required when package identity is unknown, ambiguous, or unpackaged without proof.'
    ),
    bridgeEntry(
      'process-package-publisher-signature',
      EnforcementProcessPackageProofPoint.PublisherSignature,
      'publisher-signature',
      'Publisher and signature evidence must be captured or explicitly unavailable before trust-sensitive app identity claims upgrade.',
      [
        'publisher name or certificate chain when available',
        'signature verification result or unsigned state',
        'verification source, timestamp, and custody evidence',
      ],
      [
        'Verify publisher/signature state through a Windows host source.',
        'Record publisher, signature status, unsigned or unavailable state, verification source, and evidence ids.',
      ],
      [
        'Unsigned, invalid, unavailable, and valid signatures are distinct states.',
        'Publisher evidence is tied to the same executable or package identity.',
      ],
      [
        'A company name typed by a parent presented as publisher proof.',
        'Unsigned or unavailable signature state treated as known risky behavior without a parent rule.',
      ],
      'Represent missing signature evidence as manual-required or unavailable; do not invent trust state.'
    ),
    bridgeEntry(
      'process-package-inventory-process-link',
      EnforcementProcessPackageProofPoint.InventoryProcessLink,
      'inventory',
      'Inventory and running-process evidence must agree before the runtime can claim a target is the same app.',
      [
        'joined inventory evidence id and process evidence id',
        'matching package id or executable identity',
        'freshness window and mismatch reason when the join fails',
      ],
      [
        'Join installed inventory evidence to running process evidence on the child host.',
        'Record the join key, freshness window, accepted match, or mismatch reason.',
      ],
      [
        'The join uses package id or executable identity, not display text.',
        'Mismatches produce typed unavailable/manual-required output.',
      ],
      [
        'A matching display label presented as proof of same app identity.',
        'A launcher inventory row presented as the active child app without process evidence.',
      ],
      'Reject broad app targeting when inventory and process evidence cannot be joined.'
    ),
    bridgeEntry(
      'process-package-unsupported-identity',
      EnforcementProcessPackageProofPoint.UnsupportedIdentity,
      'package',
      'Unsupported, permission-limited, or unknown host identity must remain unavailable instead of becoming app proof.',
      [
        'unsupported or permission-limited host state',
        'missing package identity reason',
        'manual remediation or alternate source requirement',
      ],
      [
        'Run the identity bridge on the target host and record unsupported or permission-limited output.',
        'Record the missing source, remediation path, and evidence id if available.',
      ],
      [
        'Unsupported identity is visible as unavailable.',
        'The fallback tells runtime and Portal not to treat unknown apps as known targets.',
      ],
      [
        'An unsupported identity source treated as successful app inventory.',
        'Unknown app identity upgraded to blocked target or risky app.',
      ],
      'Return unavailable and require manual proof when host identity is unsupported or permission-limited.',
      EnforcementProcessPackageBridgeState.Unavailable,
      EnforcementCapabilityState.Unavailable
    ),
    bridgeEntry(
      'process-package-rollback-readiness',
      EnforcementProcessPackageProofPoint.RollbackReadiness,
      'rollback',
      'Rollback readiness for broad app blocking is not claimed until the same app identity has apply and rollback artifacts.',
      [
        'block apply artifact for the same app identity',
        'rollback token and rollback result for the same app identity',
        'failure, unavailable, and audit evidence for rollback attempts',
      ],
      [
        'Run block apply and rollback proof on a real Windows host.',
        'Record target identity, apply result, rollback token, rollback result, failure state, and audit ids.',
      ],
      [
        'Rollback evidence references the same package or executable identity.',
        'Unavailable rollback remains visible and blocks product-ready claims.',
      ],
      [
        'A rollback token field presented as completed rollback.',
        'Owned-process termination cleanup presented as broad app block rollback.',
      ],
      'Keep broad app rollback not-claimed until apply and rollback artifacts exist for the same identity.',
      EnforcementProcessPackageBridgeState.NotClaimed,
      EnforcementCapabilityState.ManualRequired,
      EnforcementReadinessProofLevel.NotProved,
      EnforcementReadinessRuntimeOwner.NotImplemented
    ),
    bridgeEntry(
      'process-package-audit-custody',
      EnforcementProcessPackageProofPoint.AuditCustody,
      'audit',
      'Audit custody must tie identity evidence, parent rule, adapter outcome, fallback, and evidence refs together.',
      [
        'parent rule or policy decision id',
        'identity evidence refs used by the adapter decision',
        'adapter outcome, fallback state, and audit event ids',
      ],
      [
        'Run the proof through the real service audit path on a Windows host.',
        'Record identity refs, policy decision id, adapter result or manual-required state, fallback, and audit event ids.',
      ],
      [
        'Audit events include the identity evidence refs used for the decision.',
        'Manual-required, unavailable, and not-claimed outcomes are auditable.',
      ],
      [
        'A local proof JSON row presented as product audit.',
        'Portal-local state presented as child-agent audit custody.',
      ],
      'Require real service audit custody before any process/package identity claim can upgrade.'
    ),
  ],
});

function bridgeEntry(
  bridgeId: string,
  proofPoint: EnforcementProcessPackageProofPoint,
  evidenceClass: EnforcementProcessPackageEvidenceClass,
  hostEvidenceRequirement: string,
  requiredEvidenceArtifacts: readonly string[],
  manualProofSteps: readonly string[],
  acceptanceSignals: readonly string[],
  unsafeUpgradeExamples: readonly string[],
  fallbackBehavior: string,
  bridgeState: EnforcementProcessPackageBridgeState = EnforcementProcessPackageBridgeState.ManualRequired,
  capabilityState: typeof EnforcementCapabilityStateSchema.Type = EnforcementCapabilityState.ManualRequired,
  proofLevel: typeof EnforcementReadinessProofLevelSchema.Type = EnforcementReadinessProofLevel.ManualProofRequired,
  runtimeOwner: typeof EnforcementReadinessRuntimeOwnerSchema.Type = EnforcementReadinessRuntimeOwner.ManualProof
): EnforcementProcessPackageIdentityBridgeEntry {
  return EnforcementProcessPackageIdentityBridgeEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    bridgeId,
    proofPoint,
    evidenceClass,
    capability: EnforcementBroadAdapterCapability.BroadAppBlocking,
    platform: ParentPlatform.Windows,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState,
    bridgeState,
    proofLevel,
    runtimeOwner,
    preflightIds: [processPackagePreflightId],
    preflightGate: EnforcementHostAdapterPreflightGate.ProcessPackageIdentity,
    hostEvidenceRequirement,
    requiredEvidenceArtifacts,
    manualProofSteps,
    acceptanceSignals,
    unsafeUpgradeExamples,
    fallbackBehavior,
    lastCheckedAt: documentedAt,
  });
}

export const decodeEnforcementProcessPackageIdentityBridgeEntry = Schema.decodeUnknownSync(
  EnforcementProcessPackageIdentityBridgeEntrySchema
);
export const decodeEnforcementProcessPackageIdentityMatrix = Schema.decodeUnknownSync(
  EnforcementProcessPackageIdentityMatrixSchema
);

