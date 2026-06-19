import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  EnforcementAdapterKind,
  EnforcementAdapterKindSchema,
  EnforcementCapabilityState,
  EnforcementCapabilityStateSchema,
  EnforcementMode,
  EnforcementModeSchema,
} from './enforcement';
import {
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
} from '@ocentra-parent/schema-domain/family-reference-primitives';

export const V08OsAdapterProductProofReadModelIdSchema = brandedNonEmptyStringSchema('V08OsAdapterProductProofReadModelId');
export const V08OsAdapterProductProofEntryIdSchema = brandedNonEmptyStringSchema('V08OsAdapterProductProofEntryId');
export const V08OsAdapterCapabilityProofEntryIdSchema = brandedNonEmptyStringSchema('V08OsAdapterCapabilityProofEntryId');
export const V08OsAdapterArtifactGateEntryIdSchema = brandedNonEmptyStringSchema('V08OsAdapterArtifactGateEntryId');
export const V08OsAdapterProductProofRequirementSchema = brandedNonEmptyStringSchema('V08OsAdapterProductProofRequirement');
export const V08OsAdapterProductProofClaimBoundarySchema = brandedNonEmptyStringSchema('V08OsAdapterProductProofClaimBoundary');
export const V08OsAdapterProductProofFallbackSchema = brandedNonEmptyStringSchema('V08OsAdapterProductProofFallback');

export const V08OsAdapterProductProofSurfaceSchema = withParser(
  Schema.Literal(
    'owned-process-terminate',
    'app-time-limit-lifecycle',
    'broad-app-blocking',
    'network-domain-blocking',
    'managed-browser-service-command',
    'managed-browser-exact-url',
    'unmanaged-browser-process-only',
    'unmanaged-browser-exact-evidence',
    'restart-recovery',
    'parent-cancel-override',
    'audit-custody',
    'rollback-artifact-gate'
  )
);

export const V08OsAdapterProductProofResultStatusSchema = withParser(
  Schema.Literal('actually-enforced', 'expired', 'rolled-back', 'unavailable', 'no-op')
);

export const V08OsAdapterProductProofRollbackStateSchema = withParser(
  Schema.Literal('not-required', 'available', 'completed', 'unavailable')
);

export const V08OsAdapterProductProofTimerRecoveryStateSchema = withParser(
  Schema.Literal('not-required', 'restart-recovered', 'cancelled', 'expired', 'manual-required', 'unavailable')
);

export const V08OsAdapterProductProofAuditStateSchema = withParser(
  Schema.Literal('journaled', 'manual-required', 'unavailable')
);

export const V08OsAdapterProductProofParentOverrideStateSchema = withParser(
  Schema.Literal('not-required', 'cancel-supported', 'manual-required', 'unavailable')
);

const V08OsAdapterProductProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08OsAdapterProductProofEntryIdSchema,
  surface: V08OsAdapterProductProofSurfaceSchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  readinessState: EnforcementReadinessStateSchema,
  proofLevel: EnforcementReadinessProofLevelSchema,
  runtimeOwner: EnforcementReadinessRuntimeOwnerSchema,
  supportedModes: Schema.Array(EnforcementModeSchema),
  resultStatus: V08OsAdapterProductProofResultStatusSchema,
  rollbackState: V08OsAdapterProductProofRollbackStateSchema,
  timerRecoveryState: V08OsAdapterProductProofTimerRecoveryStateSchema,
  auditState: V08OsAdapterProductProofAuditStateSchema,
  parentOverrideState: V08OsAdapterProductProofParentOverrideStateSchema,
  linkedReadinessIds: Schema.Array(EnforcementBroadAdapterReadinessIdSchema),
  linkedCapabilityEntryIds: Schema.Array(V08OsAdapterCapabilityProofEntryIdSchema),
  linkedArtifactGateEntryIds: Schema.Array(V08OsAdapterArtifactGateEntryIdSchema),
  capabilityRequirement: V08OsAdapterProductProofRequirementSchema,
  proofRequirement: V08OsAdapterProductProofRequirementSchema,
  claimBoundary: V08OsAdapterProductProofClaimBoundarySchema,
  fallbackBehavior: V08OsAdapterProductProofFallbackSchema,
  claimUpgradeAllowed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  exactUrlClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08OsAdapterProductProofEntryCandidate = Infer<typeof V08OsAdapterProductProofEntryBaseSchema>;

export const V08OsAdapterProductProofEntrySchema = withParser(
  V08OsAdapterProductProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        productProofEntryPreservesClaimBoundary(entry) ||
        'Expected V0.8 OS-adapter product proof entries to preserve implemented, manual-required, unavailable, and not-claimed boundaries'
    )
  )
);

export const V08OsAdapterProductProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08OsAdapterProductProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
    entries: Schema.Array(V08OsAdapterProductProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.proofEntryId)).size === readModel.entries.length ||
        'Expected V0.8 OS-adapter product proof entry ids to be unique'
    )
  )
);

function productProofEntryPreservesClaimBoundary(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  if (!productProofEntryKeepsClaimsOff(entry)) {
    return false;
  }

  switch (entry.readinessState) {
    case 'implemented':
      return productProofEntryIsImplemented(entry);
    case 'manual-required':
      return productProofEntryIsManualRequired(entry);
    case 'unavailable':
      return productProofEntryIsUnavailable(entry);
    case 'not-claimed':
      return productProofEntryIsNotClaimed(entry);
  }
}

function productProofEntryKeepsClaimsOff(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  return !entry.claimUpgradeAllowed && !entry.broadBlockingClaimed && !entry.exactUrlClaimed;
}

function productProofEntryIsImplemented(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  return entry.capabilityState === 'supported' && entry.proofLevel === 'real-service-proof';
}

function productProofEntryIsManualRequired(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  return (
    entry.capabilityState === 'manual-required' &&
    entry.proofLevel === 'manual-proof-required' &&
    entry.resultStatus === 'unavailable'
  );
}

function productProofEntryIsUnavailable(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  return entry.capabilityState === 'unavailable' && entry.resultStatus === 'unavailable';
}

function productProofEntryIsNotClaimed(entry: V08OsAdapterProductProofEntryCandidate): boolean {
  return (
    entry.proofLevel === 'not-proved' && entry.runtimeOwner === 'not-implemented' && entry.resultStatus === 'no-op'
  );
}

export type V08OsAdapterProductProofReadModelId = typeof V08OsAdapterProductProofReadModelIdSchema.Type;
export type V08OsAdapterProductProofEntryId = typeof V08OsAdapterProductProofEntryIdSchema.Type;
export type V08OsAdapterCapabilityProofEntryId = typeof V08OsAdapterCapabilityProofEntryIdSchema.Type;
export type V08OsAdapterArtifactGateEntryId = typeof V08OsAdapterArtifactGateEntryIdSchema.Type;
export type V08OsAdapterProductProofRequirement = typeof V08OsAdapterProductProofRequirementSchema.Type;
export type V08OsAdapterProductProofClaimBoundary = typeof V08OsAdapterProductProofClaimBoundarySchema.Type;
export type V08OsAdapterProductProofFallback = typeof V08OsAdapterProductProofFallbackSchema.Type;
export type V08OsAdapterProductProofSurface = Infer<typeof V08OsAdapterProductProofSurfaceSchema>;
export type V08OsAdapterProductProofResultStatus = Infer<typeof V08OsAdapterProductProofResultStatusSchema>;
export type V08OsAdapterProductProofRollbackState = Infer<typeof V08OsAdapterProductProofRollbackStateSchema>;
export type V08OsAdapterProductProofTimerRecoveryState = Infer<typeof V08OsAdapterProductProofTimerRecoveryStateSchema>;
export type V08OsAdapterProductProofAuditState = Infer<typeof V08OsAdapterProductProofAuditStateSchema>;
export type V08OsAdapterProductProofParentOverrideState = Infer<
  typeof V08OsAdapterProductProofParentOverrideStateSchema
>;
export type V08OsAdapterProductProofEntry = Infer<typeof V08OsAdapterProductProofEntrySchema>;
export type V08OsAdapterProductProofReadModel = Infer<typeof V08OsAdapterProductProofReadModelSchema>;

type V08OsAdapterProductProofEntryInput = {
  proofEntryId: string;
  surface: V08OsAdapterProductProofSurface;
  adapterKind: typeof EnforcementAdapterKindSchema.Type;
  capabilityState: typeof EnforcementCapabilityStateSchema.Type;
  readinessState: EnforcementReadinessState;
  proofLevel: EnforcementReadinessProofLevel;
  runtimeOwner: EnforcementReadinessRuntimeOwner;
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>;
  resultStatus: V08OsAdapterProductProofResultStatus;
  rollbackState: V08OsAdapterProductProofRollbackState;
  timerRecoveryState: V08OsAdapterProductProofTimerRecoveryState;
  auditState: V08OsAdapterProductProofAuditState;
  parentOverrideState: V08OsAdapterProductProofParentOverrideState;
  linkedReadinessIds: readonly string[];
  linkedCapabilityEntryIds: readonly string[];
  linkedArtifactGateEntryIds: readonly string[];
  capabilityRequirement: string;
  proofRequirement: string;
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08OsAdapterProductProofSurface = {
  OwnedProcessTerminate: V08OsAdapterProductProofSurfaceSchema.parse('owned-process-terminate'),
  AppTimeLimitLifecycle: V08OsAdapterProductProofSurfaceSchema.parse('app-time-limit-lifecycle'),
  BroadAppBlocking: V08OsAdapterProductProofSurfaceSchema.parse('broad-app-blocking'),
  NetworkDomainBlocking: V08OsAdapterProductProofSurfaceSchema.parse('network-domain-blocking'),
  ManagedBrowserServiceCommand: V08OsAdapterProductProofSurfaceSchema.parse('managed-browser-service-command'),
  ManagedBrowserExactUrl: V08OsAdapterProductProofSurfaceSchema.parse('managed-browser-exact-url'),
  UnmanagedBrowserProcessOnly: V08OsAdapterProductProofSurfaceSchema.parse('unmanaged-browser-process-only'),
  UnmanagedBrowserExactEvidence: V08OsAdapterProductProofSurfaceSchema.parse('unmanaged-browser-exact-evidence'),
  RestartRecovery: V08OsAdapterProductProofSurfaceSchema.parse('restart-recovery'),
  ParentCancelOverride: V08OsAdapterProductProofSurfaceSchema.parse('parent-cancel-override'),
  AuditCustody: V08OsAdapterProductProofSurfaceSchema.parse('audit-custody'),
  RollbackArtifactGate: V08OsAdapterProductProofSurfaceSchema.parse('rollback-artifact-gate'),
} as const;

const documentedAt = '2026-05-30T13:30:00.000Z';

export const V08OsAdapterProductProofReadModel = V08OsAdapterProductProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-os-adapter-product-proof',
  generatedAt: documentedAt,
  sourceReadModelIds: [
    'v0-8-broad-os-adapter-readiness',
    'v0-8-windows-adapter-capability-proof',
    'v0-8-windows-adapter-artifact-gate',
  ],
  entries: [
    productProofEntry({
      proofEntryId: 'v0-8-proof-owned-process-terminate',
      surface: V08OsAdapterProductProofSurface.OwnedProcessTerminate,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.OsAdapter,
      supportedModes: [EnforcementMode.TerminateProcess],
      resultStatus: 'actually-enforced',
      rollbackState: 'not-required',
      timerRecoveryState: 'not-required',
      auditState: 'journaled',
      parentOverrideState: 'not-required',
      linkedReadinessIds: ['readiness-owned-process-terminate'],
      linkedCapabilityEntryIds: ['windows-adapter-unmanaged-browser-capability'],
      linkedArtifactGateEntryIds: [],
      capabilityRequirement: 'Owned process pid plus expected process name.',
      proofRequirement: 'Real service termination result and audit journal event.',
      claimBoundary: 'Only owned-process pid/name termination is proved; this is not global app blocking.',
      fallbackBehavior: 'Reject missing pid/name mismatch and return unavailable on unsupported hosts.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-app-time-limit-lifecycle',
      surface: V08OsAdapterProductProofSurface.AppTimeLimitLifecycle,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.RustService,
      supportedModes: [EnforcementMode.TimeLimit],
      resultStatus: 'expired',
      rollbackState: 'completed',
      timerRecoveryState: 'expired',
      auditState: 'journaled',
      parentOverrideState: 'cancel-supported',
      linkedReadinessIds: ['readiness-app-time-limit'],
      linkedCapabilityEntryIds: ['windows-adapter-app-target-capability'],
      linkedArtifactGateEntryIds: [],
      capabilityRequirement: 'Persisted app time-limit state and owned-process expiry path.',
      proofRequirement: 'Timer create, expiry, cancel, restart recovery, and audit tests.',
      claimBoundary: 'App time-limit proof is lifecycle proof, not broad installed-app blocking.',
      fallbackBehavior: 'Return unavailable when active timer state or platform adapter cannot support the request.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-broad-app-blocking',
      surface: V08OsAdapterProductProofSurface.BroadAppBlocking,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.ManualRequired,
      proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
      runtimeOwner: EnforcementReadinessRuntimeOwner.ManualProof,
      supportedModes: [EnforcementMode.BlockProcess],
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      timerRecoveryState: 'manual-required',
      auditState: 'manual-required',
      parentOverrideState: 'manual-required',
      linkedReadinessIds: ['readiness-broad-app-blocking'],
      linkedCapabilityEntryIds: ['windows-adapter-app-target-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-app-target'],
      capabilityRequirement: 'OS-approved installed app identity and apply/rollback adapter.',
      proofRequirement: 'Same-identity app package evidence, apply result, rollback result, and custody event.',
      claimBoundary: 'Broad installed-app blocking is not proved by owned-process termination.',
      fallbackBehavior:
        'Return manual-required or unavailable and avoid adapter requests until OS-approved proof exists.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-network-domain-blocking',
      surface: V08OsAdapterProductProofSurface.NetworkDomainBlocking,
      adapterKind: EnforcementAdapterKind.NetworkControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.ManualRequired,
      proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
      runtimeOwner: EnforcementReadinessRuntimeOwner.ManualProof,
      supportedModes: [EnforcementMode.TemporaryBlock],
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      timerRecoveryState: 'manual-required',
      auditState: 'manual-required',
      parentOverrideState: 'manual-required',
      linkedReadinessIds: ['readiness-network-domain-blocking'],
      linkedCapabilityEntryIds: ['windows-adapter-domain-network-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-domain-network-target'],
      capabilityRequirement: 'Host network filter adapter with apply and rollback proof.',
      proofRequirement: 'Network/domain filter apply result, rollback result, and audit custody event.',
      claimBoundary: 'Network flow metadata is not decrypted content and does not prove domain blocking.',
      fallbackBehavior: 'Return manual-required or unavailable until a host network control adapter has proof.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-managed-browser-service-command',
      surface: V08OsAdapterProductProofSurface.ManagedBrowserServiceCommand,
      adapterKind: EnforcementAdapterKind.ManagedBrowserControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.ManualRequired,
      proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
      runtimeOwner: EnforcementReadinessRuntimeOwner.ManualProof,
      supportedModes: [EnforcementMode.TemporaryBlock],
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      timerRecoveryState: 'manual-required',
      auditState: 'manual-required',
      parentOverrideState: 'manual-required',
      linkedReadinessIds: ['readiness-managed-browser-service-command'],
      linkedCapabilityEntryIds: ['windows-adapter-managed-browser-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-managed-browser-target'],
      capabilityRequirement: 'Managed browser command channel with audited apply behavior.',
      proofRequirement: 'Managed-browser command enforcement proof and exact URL apply/audit proof.',
      claimBoundary: 'A managed-browser service-command target string is not exact URL enforcement proof.',
      fallbackBehavior: 'Return manual-required or unavailable until managed browser command proof exists.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-managed-browser-exact-url',
      surface: V08OsAdapterProductProofSurface.ManagedBrowserExactUrl,
      adapterKind: EnforcementAdapterKind.ManagedBrowserControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.ManualRequired,
      proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
      runtimeOwner: EnforcementReadinessRuntimeOwner.ManagedBrowserBoundary,
      supportedModes: [EnforcementMode.TemporaryBlock],
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      timerRecoveryState: 'manual-required',
      auditState: 'manual-required',
      parentOverrideState: 'manual-required',
      linkedReadinessIds: ['readiness-managed-browser-exact-url-control'],
      linkedCapabilityEntryIds: ['windows-adapter-managed-browser-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-managed-browser-target'],
      capabilityRequirement: 'Managed browser active-tab and exact URL integration.',
      proofRequirement: 'Managed exact URL evidence, apply result, and custody audit.',
      claimBoundary: 'Exact URL, active tab, and page-title control require the managed browser boundary.',
      fallbackBehavior: 'Keep exact URL control manual-required unless managed browser proof is present.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-unmanaged-browser-process-only',
      surface: V08OsAdapterProductProofSurface.UnmanagedBrowserProcessOnly,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.OsAdapter,
      supportedModes: [EnforcementMode.TerminateProcess, EnforcementMode.ObserveOnly],
      resultStatus: 'actually-enforced',
      rollbackState: 'not-required',
      timerRecoveryState: 'not-required',
      auditState: 'journaled',
      parentOverrideState: 'not-required',
      linkedReadinessIds: ['readiness-unmanaged-browser-process-only'],
      linkedCapabilityEntryIds: ['windows-adapter-unmanaged-browser-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-unmanaged-browser-target'],
      capabilityRequirement: 'Unmanaged browser process pid/name evidence only.',
      proofRequirement: 'Process-only warn/terminate proof without exact URL or active tab evidence.',
      claimBoundary: 'Unmanaged browser proof is process-only and cannot become URL/tab/title/download/page evidence.',
      fallbackBehavior: 'Restrict control to pid/name guardrails and preserve exact browser evidence as not-claimed.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-unmanaged-browser-exact-evidence',
      surface: V08OsAdapterProductProofSurface.UnmanagedBrowserExactEvidence,
      adapterKind: EnforcementAdapterKind.ManagedBrowserControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.NotClaimed,
      proofLevel: EnforcementReadinessProofLevel.NotProved,
      runtimeOwner: EnforcementReadinessRuntimeOwner.NotImplemented,
      supportedModes: [],
      resultStatus: 'no-op',
      rollbackState: 'not-required',
      timerRecoveryState: 'not-required',
      auditState: 'unavailable',
      parentOverrideState: 'unavailable',
      linkedReadinessIds: ['readiness-unmanaged-browser-exact-evidence'],
      linkedCapabilityEntryIds: ['windows-adapter-unmanaged-browser-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-unmanaged-browser-target'],
      capabilityRequirement: 'Managed browser or another explicit browser integration.',
      proofRequirement: 'Exact URL, active tab, title, download, page, HTTPS content, or intent proof.',
      claimBoundary: 'Process/window/network evidence does not prove exact unmanaged browser activity.',
      fallbackBehavior:
        'Use managed browser or another explicit browser integration before representing exact evidence.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-restart-recovery',
      surface: V08OsAdapterProductProofSurface.RestartRecovery,
      adapterKind: EnforcementAdapterKind.TimerControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.RustService,
      supportedModes: [EnforcementMode.TimeLimit],
      resultStatus: 'expired',
      rollbackState: 'completed',
      timerRecoveryState: 'restart-recovered',
      auditState: 'journaled',
      parentOverrideState: 'cancel-supported',
      linkedReadinessIds: ['readiness-app-time-limit'],
      linkedCapabilityEntryIds: ['windows-adapter-app-target-capability'],
      linkedArtifactGateEntryIds: [],
      capabilityRequirement: 'Persisted timer state after service restart.',
      proofRequirement: 'Restart recovery test preserving action/result/audit/timer identity.',
      claimBoundary: 'Restart recovery proves local timer custody, not anti-tamper or bypass resistance.',
      fallbackBehavior: 'Return unavailable when persisted timer state is missing or inconsistent.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-parent-cancel-override',
      surface: V08OsAdapterProductProofSurface.ParentCancelOverride,
      adapterKind: EnforcementAdapterKind.TimerControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.RustService,
      supportedModes: [EnforcementMode.TimeLimit, EnforcementMode.AskParent],
      resultStatus: 'rolled-back',
      rollbackState: 'completed',
      timerRecoveryState: 'cancelled',
      auditState: 'journaled',
      parentOverrideState: 'cancel-supported',
      linkedReadinessIds: ['readiness-app-time-limit'],
      linkedCapabilityEntryIds: ['windows-adapter-app-target-capability'],
      linkedArtifactGateEntryIds: [],
      capabilityRequirement: 'Parent cancel/override reference tied to active timer state.',
      proofRequirement: 'Parent cancel path that records rollback and audit state.',
      claimBoundary: 'Parent cancel is timer-scoped and does not prove broad unblock rollback.',
      fallbackBehavior: 'Reject parent action when active timer state is missing or mismatched.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-audit-custody',
      surface: V08OsAdapterProductProofSurface.AuditCustody,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.Supported,
      readinessState: EnforcementReadinessState.Implemented,
      proofLevel: EnforcementReadinessProofLevel.RealServiceProof,
      runtimeOwner: EnforcementReadinessRuntimeOwner.RustService,
      supportedModes: [EnforcementMode.TerminateProcess, EnforcementMode.TimeLimit],
      resultStatus: 'actually-enforced',
      rollbackState: 'available',
      timerRecoveryState: 'not-required',
      auditState: 'journaled',
      parentOverrideState: 'cancel-supported',
      linkedReadinessIds: ['readiness-owned-process-terminate', 'readiness-app-time-limit'],
      linkedCapabilityEntryIds: ['windows-adapter-rollback-audit-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-rollback-audit-target'],
      capabilityRequirement: 'Local audit journal/store custody for enforcement outcomes.',
      proofRequirement:
        'Audit event and journal sequence for attempted, succeeded, unavailable, expired, and cancelled paths.',
      claimBoundary: 'Audit custody is local evidence recording, not production anti-tamper hardening.',
      fallbackBehavior: 'Keep broad rollback and bypass-resistance claims manual-required until artifact gate passes.',
    }),
    productProofEntry({
      proofEntryId: 'v0-8-proof-rollback-artifact-gate',
      surface: V08OsAdapterProductProofSurface.RollbackArtifactGate,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      readinessState: EnforcementReadinessState.ManualRequired,
      proofLevel: EnforcementReadinessProofLevel.ManualProofRequired,
      runtimeOwner: EnforcementReadinessRuntimeOwner.ManualProof,
      supportedModes: [],
      resultStatus: 'unavailable',
      rollbackState: 'unavailable',
      timerRecoveryState: 'manual-required',
      auditState: 'manual-required',
      parentOverrideState: 'manual-required',
      linkedReadinessIds: ['readiness-admin-anti-tamper-rollback'],
      linkedCapabilityEntryIds: ['windows-adapter-rollback-audit-capability'],
      linkedArtifactGateEntryIds: ['windows-adapter-artifact-gate-rollback-audit-target'],
      capabilityRequirement: 'Same-identity apply, rollback, and custody artifacts.',
      proofRequirement: 'Artifact gate proof before any broad rollback or anti-tamper product claim.',
      claimBoundary: 'Admin hardening, anti-tamper, bypass resistance, and broad rollback are not proved.',
      fallbackBehavior: 'Keep claims manual-required until real host hardening and rollback evidence exists.',
    }),
  ],
});

function productProofEntry(entry: V08OsAdapterProductProofEntryInput): V08OsAdapterProductProofEntry {
  return V08OsAdapterProductProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Windows,
    claimUpgradeAllowed: false,
    broadBlockingClaimed: false,
    exactUrlClaimed: false,
    lastCheckedAt: documentedAt,
    ...entry,
  });
}

export const decodeV08OsAdapterProductProofEntry = Schema.decodeUnknownSync(V08OsAdapterProductProofEntrySchema);
export const decodeV08OsAdapterProductProofReadModel = Schema.decodeUnknownSync(
  V08OsAdapterProductProofReadModelSchema
);

