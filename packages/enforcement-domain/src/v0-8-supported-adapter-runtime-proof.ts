import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

export * from './v0-8-integrity-alert-status-bridge';
export * from '@ocentra-parent/notification-domain/v0-8-notification-provider-status-boundary';
export * from './v0-8-enforcement-integrity-runtime-audit';

export const V08SupportedAdapterRuntimeProofReadModelIdSchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofReadModelId');
export const V08SupportedAdapterRuntimeProofEntryIdSchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofEntryId');
export const V08SupportedAdapterRuntimeProofReferenceSchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofReference');
export const V08SupportedAdapterRuntimeProofRequirementSchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofRequirement');
export const V08SupportedAdapterRuntimeProofBoundarySchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofBoundary');
export const V08SupportedAdapterRuntimeProofFallbackSchema = brandedNonEmptyStringSchema('V08SupportedAdapterRuntimeProofFallback');

export const V08SupportedAdapterRuntimeBoundarySchema = withParser(
  Schema.Literal(
    'windows-app-game-owned-process-time-limit',
    'windows-network-flow-observe-policy-handoff',
    'windows-broad-installed-app-blocking-manual-gate',
    'windows-host-network-domain-blocking-manual-gate',
    'windows-broad-installed-app-artifact-status',
    'windows-host-network-domain-artifact-status',
    'windows-managed-browser-artifact-status',
    'windows-managed-exact-active-tab-not-claimed',
    'windows-adapter-permission-dependency-degraded',
    'linux-host-adapter-unavailable',
    'macos-host-adapter-unsupported',
    'android-mobile-control-manual-gate',
    'ios-mobile-control-manual-gate'
  )
);

export const V08SupportedAdapterCapabilitySchema = withParser(
  Schema.Literal(
    'app-game-owned-process-time-limit',
    'network-flow-observe-policy-handoff',
    'broad-installed-app-blocking',
    'host-network-domain-blocking',
    'broad-installed-app-artifact-status',
    'host-network-domain-artifact-status',
    'managed-browser-artifact-status',
    'managed-exact-active-tab-enforcement',
    'adapter-permission-dependency',
    'desktop-host-platform-adapter',
    'mobile-child-control-adapter'
  )
);

export const V08SupportedAdapterRuntimeStateSchema = withParser(
  Schema.Literal('implemented-boundary', 'manual-required', 'unavailable', 'not-claimed', 'unsupported', 'degraded')
);

export const V08SupportedAdapterResultSchema = withParser(
  Schema.Literal(
    'supported-boundary-proved',
    'manual-proof-required',
    'target-unavailable',
    'not-claimed',
    'unsupported-platform',
    'degraded-permission-or-dependency'
  )
);

export const V08SupportedAdapterPlatformSupportStateSchema = withParser(
  Schema.Literal('supported-on-windows', 'manual-required', 'unavailable-on-target', 'unsupported-platform', 'degraded')
);

export const V08SupportedAdapterTargetIdentityStateSchema = withParser(
  Schema.Literal(
    'process-session-evidence-backed',
    'network-flow-evidence-backed',
    'insufficient-for-broad-target',
    'not-applicable',
    'unsupported-platform-target'
  )
);

export const V08SupportedAdapterRollbackReferenceStateSchema = withParser(
  Schema.Literal('timer-recovery-backed', 'observe-only-not-needed', 'manual-required', 'unavailable', 'not-claimed')
);

export const V08SupportedAdapterAuditReferenceStateSchema = withParser(
  Schema.Literal('audit-reference-backed', 'manual-required', 'unavailable', 'not-claimed')
);

export const V08SupportedAdapterRefusalReasonSchema = withParser(
  Schema.Literal(
    'none',
    'manual-artifact-required',
    'target-unavailable',
    'not-claimed-boundary',
    'unsupported-platform',
    'permission-or-dependency-degraded'
  )
);

const V08SupportedAdapterRuntimeProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08SupportedAdapterRuntimeProofEntryIdSchema,
  runtimeBoundary: V08SupportedAdapterRuntimeBoundarySchema,
  platform: ParentPlatformSchema,
  adapterCapability: V08SupportedAdapterCapabilitySchema,
  runtimeState: V08SupportedAdapterRuntimeStateSchema,
  adapterResult: V08SupportedAdapterResultSchema,
  platformSupportState: V08SupportedAdapterPlatformSupportStateSchema,
  targetIdentityState: V08SupportedAdapterTargetIdentityStateSchema,
  rollbackReferenceState: V08SupportedAdapterRollbackReferenceStateSchema,
  auditReferenceState: V08SupportedAdapterAuditReferenceStateSchema,
  refusalReason: V08SupportedAdapterRefusalReasonSchema,
  evidenceRefs: Schema.Array(V08SupportedAdapterRuntimeProofReferenceSchema),
  linkedProofCommands: Schema.Array(V08SupportedAdapterRuntimeProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08SupportedAdapterRuntimeProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08SupportedAdapterRuntimeProofRequirementSchema),
  claimBoundary: V08SupportedAdapterRuntimeProofBoundarySchema,
  fallbackBehavior: V08SupportedAdapterRuntimeProofFallbackSchema,
  broadInstalledAppBlockingClaimed: Schema.Boolean,
  networkDomainBlockingClaimed: Schema.Boolean,
  exactActiveTabEnforcementClaimed: Schema.Boolean,
  notificationDeliveryClaimed: Schema.Boolean,
  tamperHardeningClaimed: Schema.Boolean,
  mobileControlClaimed: Schema.Boolean,
  unsupportedPlatformBehaviorClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08SupportedAdapterRuntimeProofEntryCandidate = Infer<typeof V08SupportedAdapterRuntimeProofEntryBaseSchema>;

export const V08SupportedAdapterRuntimeProofEntrySchema = withParser(
  V08SupportedAdapterRuntimeProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportedAdapterRuntimeProofEntryIsHonest(entry) ||
        'Expected V0.8 supported adapter runtime proof entries to distinguish implemented-boundary, manual-required, unavailable, not-claimed, unsupported, and degraded states without broad blocking, exact active-tab, notification, tamper, mobile, or unsupported-platform behavior claim upgrades'
    )
  )
);

export const V08SupportedAdapterRuntimeProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08SupportedAdapterRuntimeProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08SupportedAdapterRuntimeProofReferenceSchema),
    entries: Schema.Array(V08SupportedAdapterRuntimeProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.proofEntryId)).size === readModel.entries.length ||
        'Expected V0.8 supported adapter runtime proof entry ids to be unique'
    )
  )
);

function supportedAdapterRuntimeProofEntryIsHonest(entry: V08SupportedAdapterRuntimeProofEntryCandidate): boolean {
  if (supportedAdapterRuntimeProofEntryHasClaimUpgrade(entry)) {
    return false;
  }

  return supportedAdapterRuntimeProofEntryStateMatchesProof(entry);
}

function supportedAdapterRuntimeProofEntryStateMatchesProof(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  switch (entry.runtimeState) {
    case 'implemented-boundary':
      return supportedAdapterRuntimeProofImplementedBoundaryIsHonest(entry);
    case 'manual-required':
      return supportedAdapterRuntimeProofManualRequiredIsHonest(entry);
    case 'unavailable':
      return supportedAdapterRuntimeProofUnavailableIsHonest(entry);
    case 'not-claimed':
      return supportedAdapterRuntimeProofNotClaimedIsHonest(entry);
    case 'unsupported':
      return supportedAdapterRuntimeProofUnsupportedIsHonest(entry);
    case 'degraded':
      return supportedAdapterRuntimeProofDegradedIsHonest(entry);
  }
}

function supportedAdapterRuntimeProofImplementedBoundaryIsHonest(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  return (
    entry.platform === 'windows' &&
    entry.adapterResult === 'supported-boundary-proved' &&
    entry.platformSupportState === 'supported-on-windows' &&
    entry.refusalReason === 'none' &&
    entry.evidenceRefs.length > 0 &&
    entry.linkedProofArtifacts.length > 0 &&
    entry.manualProofRequirements.length === 0
  );
}

function supportedAdapterRuntimeProofManualRequiredIsHonest(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  return entry.adapterResult === 'manual-proof-required' && entry.manualProofRequirements.length > 0;
}

function supportedAdapterRuntimeProofUnavailableIsHonest(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  return (
    entry.adapterResult === 'target-unavailable' &&
    entry.platformSupportState === 'unavailable-on-target' &&
    entry.manualProofRequirements.length > 0
  );
}

function supportedAdapterRuntimeProofNotClaimedIsHonest(entry: V08SupportedAdapterRuntimeProofEntryCandidate): boolean {
  return entry.adapterResult === 'not-claimed' && entry.manualProofRequirements.length > 0;
}

function supportedAdapterRuntimeProofUnsupportedIsHonest(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  return (
    entry.adapterResult === 'unsupported-platform' &&
    entry.platformSupportState === 'unsupported-platform' &&
    entry.manualProofRequirements.length > 0
  );
}

function supportedAdapterRuntimeProofDegradedIsHonest(entry: V08SupportedAdapterRuntimeProofEntryCandidate): boolean {
  return (
    entry.adapterResult === 'degraded-permission-or-dependency' &&
    entry.platformSupportState === 'degraded' &&
    entry.refusalReason === 'permission-or-dependency-degraded' &&
    entry.manualProofRequirements.length > 0
  );
}

function supportedAdapterRuntimeProofEntryHasClaimUpgrade(
  entry: V08SupportedAdapterRuntimeProofEntryCandidate
): boolean {
  return [
    entry.broadInstalledAppBlockingClaimed,
    entry.networkDomainBlockingClaimed,
    entry.exactActiveTabEnforcementClaimed,
    entry.notificationDeliveryClaimed,
    entry.tamperHardeningClaimed,
    entry.mobileControlClaimed,
    entry.unsupportedPlatformBehaviorClaimed,
  ].some(Boolean);
}

export type V08SupportedAdapterRuntimeProofReadModelId = typeof V08SupportedAdapterRuntimeProofReadModelIdSchema.Type;
export type V08SupportedAdapterRuntimeProofEntryId = typeof V08SupportedAdapterRuntimeProofEntryIdSchema.Type;
export type V08SupportedAdapterRuntimeProofReference = typeof V08SupportedAdapterRuntimeProofReferenceSchema.Type;
export type V08SupportedAdapterRuntimeProofRequirement = typeof V08SupportedAdapterRuntimeProofRequirementSchema.Type;
export type V08SupportedAdapterRuntimeProofBoundary = typeof V08SupportedAdapterRuntimeProofBoundarySchema.Type;
export type V08SupportedAdapterRuntimeProofFallback = typeof V08SupportedAdapterRuntimeProofFallbackSchema.Type;
export type V08SupportedAdapterRuntimeBoundary = Infer<typeof V08SupportedAdapterRuntimeBoundarySchema>;
export type V08SupportedAdapterCapability = Infer<typeof V08SupportedAdapterCapabilitySchema>;
export type V08SupportedAdapterRuntimeState = Infer<typeof V08SupportedAdapterRuntimeStateSchema>;
export type V08SupportedAdapterResult = Infer<typeof V08SupportedAdapterResultSchema>;
export type V08SupportedAdapterPlatformSupportState = Infer<typeof V08SupportedAdapterPlatformSupportStateSchema>;
export type V08SupportedAdapterTargetIdentityState = Infer<typeof V08SupportedAdapterTargetIdentityStateSchema>;
export type V08SupportedAdapterRollbackReferenceState = Infer<typeof V08SupportedAdapterRollbackReferenceStateSchema>;
export type V08SupportedAdapterAuditReferenceState = Infer<typeof V08SupportedAdapterAuditReferenceStateSchema>;
export type V08SupportedAdapterRefusalReason = Infer<typeof V08SupportedAdapterRefusalReasonSchema>;
export type V08SupportedAdapterRuntimeProofEntry = Infer<typeof V08SupportedAdapterRuntimeProofEntrySchema>;
export type V08SupportedAdapterRuntimeProofReadModel = Infer<typeof V08SupportedAdapterRuntimeProofReadModelSchema>;

type V08SupportedAdapterRuntimeProofEntryInput = {
  proofEntryId: string;
  runtimeBoundary: V08SupportedAdapterRuntimeBoundary;
  platform: typeof ParentPlatformSchema.Type;
  adapterCapability: V08SupportedAdapterCapability;
  runtimeState: V08SupportedAdapterRuntimeState;
  adapterResult: V08SupportedAdapterResult;
  platformSupportState: V08SupportedAdapterPlatformSupportState;
  targetIdentityState: V08SupportedAdapterTargetIdentityState;
  rollbackReferenceState: V08SupportedAdapterRollbackReferenceState;
  auditReferenceState: V08SupportedAdapterAuditReferenceState;
  refusalReason: V08SupportedAdapterRefusalReason;
  evidenceRefs: readonly string[];
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08SupportedAdapterRuntimeBoundary = {
  WindowsAppGameOwnedProcessTimeLimit: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-app-game-owned-process-time-limit'
  ),
  WindowsNetworkFlowObservePolicyHandoff: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-network-flow-observe-policy-handoff'
  ),
  WindowsBroadInstalledAppBlockingManualGate: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-broad-installed-app-blocking-manual-gate'
  ),
  WindowsHostNetworkDomainBlockingManualGate: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-host-network-domain-blocking-manual-gate'
  ),
  WindowsBroadInstalledAppArtifactStatus: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-broad-installed-app-artifact-status'
  ),
  WindowsHostNetworkDomainArtifactStatus: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-host-network-domain-artifact-status'
  ),
  WindowsManagedBrowserArtifactStatus: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-managed-browser-artifact-status'
  ),
  WindowsManagedExactActiveTabNotClaimed: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-managed-exact-active-tab-not-claimed'
  ),
  WindowsAdapterPermissionDependencyDegraded: V08SupportedAdapterRuntimeBoundarySchema.parse(
    'windows-adapter-permission-dependency-degraded'
  ),
  LinuxHostAdapterUnavailable: V08SupportedAdapterRuntimeBoundarySchema.parse('linux-host-adapter-unavailable'),
  MacosHostAdapterUnsupported: V08SupportedAdapterRuntimeBoundarySchema.parse('macos-host-adapter-unsupported'),
  AndroidMobileControlManualGate: V08SupportedAdapterRuntimeBoundarySchema.parse('android-mobile-control-manual-gate'),
  IosMobileControlManualGate: V08SupportedAdapterRuntimeBoundarySchema.parse('ios-mobile-control-manual-gate'),
} as const;

const SourceReadModelIds = {
  BroadAdapterProof: 'v0-8-broad-os-adapter-runtime-proof',
  PolicyDispatchProof: 'v0-8-enforcement-policy-dispatch-proof',
  ProductControlProof: 'v0-8-enforcement-product-control-spine',
  NetworkFlowEvidence: 'network-flow-read-model',
  WindowsAdapterCapabilityProof: 'v0-8-windows-adapter-capability-proof',
  WindowsAdapterArtifactGate: 'v0-8-windows-adapter-artifact-gate',
  WindowsAdapterArtifactIngestionProof: 'v0-8-windows-adapter-artifact-ingestion-proof',
} as const;

const generatedAt = '2026-06-02T09:03:36.000Z';

export const V08SupportedAdapterRuntimeProofReadModel = V08SupportedAdapterRuntimeProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-supported-adapter-runtime-proof',
  generatedAt,
  sourceReadModelIds: Object.values(SourceReadModelIds),
  entries: [
    entry({
      proofEntryId: 'windows-app-game-owned-process-time-limit',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.WindowsAppGameOwnedProcessTimeLimit,
      platform: 'windows',
      adapterCapability: 'app-game-owned-process-time-limit',
      runtimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      platformSupportState: 'supported-on-windows',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'timer-recovery-backed',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
      evidenceRefs: ['app-game-session-evidence-ref', 'owned-process-identity-ref', 'timer-state-ref'],
      linkedProofCommands: [
        'node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs',
        'cargo test -p ocentra-parent-agent-service enforcement_timer',
      ],
      linkedProofArtifacts: [
        'test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json',
        'crates/agent-service/src/enforcement_timer_state.rs',
      ],
      manualProofRequirements: [],
      claimBoundary:
        'App/game support is limited to owned-process identity, app-session evidence, timer state, audit refs, and recoverable expiry; it is not broad installed-app blocking.',
      fallbackBehavior:
        'Targets without process/session identity or timer custody return manual-required or degraded instead of escalating to broad block.',
    }),
    entry({
      proofEntryId: 'windows-network-flow-observe-policy-handoff',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.WindowsNetworkFlowObservePolicyHandoff,
      platform: 'windows',
      adapterCapability: 'network-flow-observe-policy-handoff',
      runtimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      platformSupportState: 'supported-on-windows',
      targetIdentityState: 'network-flow-evidence-backed',
      rollbackReferenceState: 'observe-only-not-needed',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
      evidenceRefs: ['network-flow-summary-ref', 'domain-attribution-state-ref', 'policy-preview-ref'],
      linkedProofCommands: [
        'cargo test -p ocentra-parent-agent-service network_flow_digest',
        'node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs',
      ],
      linkedProofArtifacts: [
        'crates/agent-service/src/network_flow_digest.rs',
        'test-results/v0-8-enforcement-policy-dispatch-proof/proof.json',
      ],
      manualProofRequirements: [],
      claimBoundary:
        'Network/domain support is observe-only policy handoff over stored flow evidence; it is not DNS, VPN, packet, or host filter enforcement.',
      fallbackBehavior:
        'Network controls without a host filter adapter report manual-required for enforcement while preserving observe-only evidence refs.',
    }),
    manualEntry(
      'windows-broad-installed-app-blocking-manual-gate',
      V08SupportedAdapterRuntimeBoundary.WindowsBroadInstalledAppBlockingManualGate,
      'windows',
      'broad-installed-app-blocking',
      'insufficient-for-broad-target',
      ['same app identity proof', 'host block apply artifact', 'rollback artifact', 'audit custody artifact'],
      'Broad installed-app blocking remains manual-required because scoped process/timer proof does not prove package-wide blocking.',
      'The runtime refuses broad app blocking claims until target host apply, rollback, and audit artifacts exist.'
    ),
    manualEntry(
      'windows-host-network-domain-blocking-manual-gate',
      V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainBlockingManualGate,
      'windows',
      'host-network-domain-blocking',
      'insufficient-for-broad-target',
      ['host DNS or filter apply artifact', 'rollback artifact', 'audit custody artifact'],
      'Host network/domain blocking remains manual-required because flow evidence and policy handoff are not filter apply proof.',
      'The runtime refuses network/domain blocking claims until a host filter or DNS adapter proves apply and rollback.'
    ),
    artifactStatusEntry(
      'windows-broad-installed-app-artifact-status',
      V08SupportedAdapterRuntimeBoundary.WindowsBroadInstalledAppArtifactStatus,
      'broad-installed-app-artifact-status',
      'insufficient-for-broad-target',
      [
        'same-identity app package evidence',
        'adapter apply result',
        'adapter rollback result',
        'audit custody event',
        'manual review after artifact gate',
      ],
      'Windows app artifacts can make a broad-app target ready for manual review only; they do not prove broad installed-app blocking.',
      'Missing, mismatched, or uncustodied app artifacts stay refused and complete artifact sets remain manual-review-only.'
    ),
    artifactStatusEntry(
      'windows-host-network-domain-artifact-status',
      V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainArtifactStatus,
      'host-network-domain-artifact-status',
      'insufficient-for-broad-target',
      [
        'network/domain filter apply result',
        'network/domain filter rollback result',
        'audit custody event',
        'manual review after artifact gate',
      ],
      'Windows network/domain artifacts can make a host-filter target ready for manual review only; they do not prove DNS, VPN, packet, or domain blocking.',
      'Missing, mismatched, or uncustodied network artifacts stay refused and complete artifact sets remain manual-review-only.'
    ),
    artifactStatusEntry(
      'windows-managed-browser-artifact-status',
      V08SupportedAdapterRuntimeBoundary.WindowsManagedBrowserArtifactStatus,
      'managed-browser-artifact-status',
      'insufficient-for-broad-target',
      ['managed-browser exact URL evidence', 'audit custody event', 'manual review after artifact gate'],
      'Windows managed-browser artifacts can make exact-URL control ready for manual review only; they do not prove active-tab enforcement.',
      'Missing, mismatched, or uncustodied managed-browser artifacts stay refused and complete artifact sets remain manual-review-only.'
    ),
    entry({
      proofEntryId: 'windows-managed-exact-active-tab-not-claimed',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.WindowsManagedExactActiveTabNotClaimed,
      platform: 'windows',
      adapterCapability: 'managed-exact-active-tab-enforcement',
      runtimeState: 'not-claimed',
      adapterResult: 'not-claimed',
      platformSupportState: 'manual-required',
      targetIdentityState: 'insufficient-for-broad-target',
      rollbackReferenceState: 'not-claimed',
      auditReferenceState: 'not-claimed',
      refusalReason: 'not-claimed-boundary',
      evidenceRefs: [],
      linkedProofCommands: ['node scripts/test/v0-8-browser-domain-adapter-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-browser-domain-adapter-proof/proof.json'],
      manualProofRequirements: [
        'managed active-tab evidence artifact',
        'exact URL apply artifact',
        'rollback artifact',
      ],
      claimBoundary:
        'Exact active-tab enforcement is not claimed by supported app/game or network observe-only runtime proof.',
      fallbackBehavior:
        'The runtime may report managed-session or process fallback states, but exact active-tab enforcement remains not-claimed.',
    }),
    entry({
      proofEntryId: 'windows-adapter-permission-dependency-degraded',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.WindowsAdapterPermissionDependencyDegraded,
      platform: 'windows',
      adapterCapability: 'adapter-permission-dependency',
      runtimeState: 'degraded',
      adapterResult: 'degraded-permission-or-dependency',
      platformSupportState: 'degraded',
      targetIdentityState: 'not-applicable',
      rollbackReferenceState: 'manual-required',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'permission-or-dependency-degraded',
      evidenceRefs: ['adapter-capability-state-ref'],
      linkedProofCommands: ['node scripts/test/v0-8-windows-adapter-capability-proof.mjs'],
      linkedProofArtifacts: ['test-results/v0-8-windows-adapter-capability-proof/proof.json'],
      manualProofRequirements: [
        'permission restoration artifact',
        'dependency reinstall artifact',
        'operator-visible degraded state',
      ],
      claimBoundary:
        'Supported-boundary adapters can degrade when permissions or dependencies are missing; degraded state is not enforcement success.',
      fallbackBehavior:
        'The runtime emits degraded capability and keeps evidence capture or observe-only paths available where possible.',
    }),
    entry({
      proofEntryId: 'linux-host-adapter-unavailable',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.LinuxHostAdapterUnavailable,
      platform: 'linux',
      adapterCapability: 'desktop-host-platform-adapter',
      runtimeState: 'unavailable',
      adapterResult: 'target-unavailable',
      platformSupportState: 'unavailable-on-target',
      targetIdentityState: 'unsupported-platform-target',
      rollbackReferenceState: 'unavailable',
      auditReferenceState: 'unavailable',
      refusalReason: 'target-unavailable',
      evidenceRefs: [],
      linkedProofCommands: [],
      linkedProofArtifacts: [],
      manualProofRequirements: [
        'Linux service manager artifact',
        'Linux permission artifact',
        'Linux rollback artifact',
      ],
      claimBoundary: 'Linux host adapter support is unavailable in this proof and cannot inherit Windows results.',
      fallbackBehavior: 'Linux targets report unavailable until a target-specific adapter proves support.',
    }),
    entry({
      proofEntryId: 'macos-host-adapter-unsupported',
      runtimeBoundary: V08SupportedAdapterRuntimeBoundary.MacosHostAdapterUnsupported,
      platform: 'macos',
      adapterCapability: 'desktop-host-platform-adapter',
      runtimeState: 'unsupported',
      adapterResult: 'unsupported-platform',
      platformSupportState: 'unsupported-platform',
      targetIdentityState: 'unsupported-platform-target',
      rollbackReferenceState: 'unavailable',
      auditReferenceState: 'unavailable',
      refusalReason: 'unsupported-platform',
      evidenceRefs: [],
      linkedProofCommands: [],
      linkedProofArtifacts: [],
      manualProofRequirements: [
        'macOS permission artifact',
        'macOS package identity artifact',
        'macOS rollback artifact',
      ],
      claimBoundary: 'macOS host adapter support is unsupported in this proof and cannot reuse Windows host evidence.',
      fallbackBehavior: 'macOS targets report unsupported until a macOS-specific adapter and artifacts exist.',
    }),
    mobileManualEntry(
      'android-mobile-control-manual-gate',
      V08SupportedAdapterRuntimeBoundary.AndroidMobileControlManualGate,
      'android',
      ['device-owner or managed-profile artifact', 'UsageStats artifact', 'accessibility or VPN/DNS artifact']
    ),
    mobileManualEntry(
      'ios-mobile-control-manual-gate',
      V08SupportedAdapterRuntimeBoundary.IosMobileControlManualGate,
      'ios',
      ['Family Controls entitlement artifact', 'DeviceActivity artifact', 'Network Extension artifact']
    ),
  ],
});

function manualEntry(
  proofEntryId: string,
  runtimeBoundary: V08SupportedAdapterRuntimeBoundary,
  platform: typeof ParentPlatformSchema.Type,
  adapterCapability: V08SupportedAdapterCapability,
  targetIdentityState: V08SupportedAdapterTargetIdentityState,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08SupportedAdapterRuntimeProofEntry {
  return entry({
    proofEntryId,
    runtimeBoundary,
    platform,
    adapterCapability,
    runtimeState: 'manual-required',
    adapterResult: 'manual-proof-required',
    platformSupportState: 'manual-required',
    targetIdentityState,
    rollbackReferenceState: 'manual-required',
    auditReferenceState: 'manual-required',
    refusalReason: 'manual-artifact-required',
    evidenceRefs: [],
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function artifactStatusEntry(
  proofEntryId: string,
  runtimeBoundary: V08SupportedAdapterRuntimeBoundary,
  adapterCapability: V08SupportedAdapterCapability,
  targetIdentityState: V08SupportedAdapterTargetIdentityState,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08SupportedAdapterRuntimeProofEntry {
  return entry({
    proofEntryId,
    runtimeBoundary,
    platform: 'windows',
    adapterCapability,
    runtimeState: 'manual-required',
    adapterResult: 'manual-proof-required',
    platformSupportState: 'manual-required',
    targetIdentityState,
    rollbackReferenceState: 'manual-required',
    auditReferenceState: 'manual-required',
    refusalReason: 'manual-artifact-required',
    evidenceRefs: ['windows-adapter-artifact-gate-ref', 'windows-adapter-artifact-ingestion-ref'],
    linkedProofCommands: [
      'node scripts/test/v0-8-windows-adapter-capability-proof.mjs',
      'node scripts/test/v0-8-windows-adapter-artifact-gate.mjs',
      'node scripts/test/v0-8-windows-adapter-artifact-ingestion-proof.mjs',
    ],
    linkedProofArtifacts: [
      'test-results/v0-8-windows-adapter-capability-proof/proof.json',
      'test-results/v0-8-windows-adapter-artifact-gate/proof.json',
      'test-results/v0-8-windows-adapter-artifact-ingestion-proof/proof.json',
    ],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function mobileManualEntry(
  proofEntryId: string,
  runtimeBoundary: V08SupportedAdapterRuntimeBoundary,
  platform: typeof ParentPlatformSchema.Type,
  manualProofRequirements: readonly string[]
): V08SupportedAdapterRuntimeProofEntry {
  return manualEntry(
    proofEntryId,
    runtimeBoundary,
    platform,
    'mobile-child-control-adapter',
    'unsupported-platform-target',
    manualProofRequirements,
    'Mobile child control remains manual-required and is not proved by Windows host supported-boundary adapters.',
    'Mobile targets keep privileged platform states manual-required until real mobile artifacts exist.'
  );
}

function entry(input: V08SupportedAdapterRuntimeProofEntryInput): V08SupportedAdapterRuntimeProofEntry {
  return V08SupportedAdapterRuntimeProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    broadInstalledAppBlockingClaimed: false,
    networkDomainBlockingClaimed: false,
    exactActiveTabEnforcementClaimed: false,
    notificationDeliveryClaimed: false,
    tamperHardeningClaimed: false,
    mobileControlClaimed: false,
    unsupportedPlatformBehaviorClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeV08SupportedAdapterRuntimeProofEntry = Schema.decodeUnknownSync(
  V08SupportedAdapterRuntimeProofEntrySchema
);
export const decodeV08SupportedAdapterRuntimeProofReadModel = Schema.decodeUnknownSync(
  V08SupportedAdapterRuntimeProofReadModelSchema
);

