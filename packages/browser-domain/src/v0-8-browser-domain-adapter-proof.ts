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
} from '@ocentra-parent/family-domain/reference-primitives';

export const V08BrowserDomainAdapterProofReadModelIdSchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofReadModelId');
export const V08BrowserDomainAdapterProofEntryIdSchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofEntryId');
export const V08BrowserDomainAdapterProofReferenceSchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofReference');
export const V08BrowserDomainAdapterProofRequirementSchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofRequirement');
export const V08BrowserDomainAdapterProofClaimBoundarySchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofClaimBoundary');
export const V08BrowserDomainAdapterProofFallbackSchema = brandedNonEmptyStringSchema('V08BrowserDomainAdapterProofFallback');

export const V08BrowserDomainAdapterProofSurfaceSchema = withParser(
  Schema.Literal(
    'windows-managed-browser-intervention-state',
    'windows-managed-browser-exact-url-manual',
    'windows-unmanaged-browser-terminate-boundary',
    'windows-unmanaged-browser-warn-noop',
    'windows-unmanaged-browser-exact-evidence-not-claimed',
    'windows-network-domain-filter-manual',
    'windows-network-domain-adapter-unavailable',
    'windows-audit-visibility-boundary',
    'windows-restart-recovery-visibility-boundary',
    'windows-browser-policy-rollback-visibility',
    'linux-browser-domain-adapter-unavailable',
    'macos-browser-domain-adapter-unavailable',
    'android-browser-domain-adapter-manual',
    'ios-browser-domain-adapter-manual'
  )
);

export const V08BrowserDomainAdapterProofEvidenceKindSchema = withParser(
  Schema.Literal(
    'managed-browser',
    'unmanaged-browser',
    'network-domain',
    'audit',
    'restart-recovery',
    'rollback',
    'unsupported-target'
  )
);

export const V08BrowserDomainAdapterProofClaimStateSchema = withParser(
  Schema.Literal('implemented-boundary', 'degraded-boundary', 'manual-required', 'unavailable', 'not-claimed')
);

export const V08BrowserDomainAdapterExecutionStateSchema = withParser(
  Schema.Literal(
    'executes-real-service',
    'returns-degraded-noop',
    'returns-manual-required',
    'returns-unavailable',
    'not-invoked'
  )
);

export const V08WindowsAppControlProofStateIdSchema = brandedNonEmptyStringSchema('V08WindowsAppControlProofStateId');
export const V08WindowsAppControlReadinessStateSchema = withParser(
  Schema.Literal('readiness-check', 'audit-only', 'enforced', 'manual-required', 'unavailable', 'failed')
);
export const V08WindowsAppControlPolicyMutationStateSchema = withParser(
  Schema.Literal(
    'detect-only',
    'audit-only-visible',
    'create-update-manual-required',
    'manual-setup-required',
    'unavailable',
    'failed'
  )
);
export const V08WindowsAppControlRuleIdentityKindSchema = withParser(
  Schema.Literal('publisher', 'path', 'hash', 'package')
);
export const V08WindowsAppControlAdminRequirementSchema = withParser(
  Schema.Literal('administrator-required', 'service-permission-required', 'manual-operator-required', 'not-applicable')
);
export const V08WindowsAppControlEventStateSchema = withParser(
  Schema.Literal('audit-visible', 'rollback-visible', 'failure-visible', 'manual-proof-required', 'unavailable')
);

const V08BrowserDomainAdapterProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08BrowserDomainAdapterProofEntryIdSchema,
  surface: V08BrowserDomainAdapterProofSurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKindSchema,
  productClaimState: V08BrowserDomainAdapterProofClaimStateSchema,
  adapterExecutionState: V08BrowserDomainAdapterExecutionStateSchema,
  linkedProofCommands: Schema.Array(V08BrowserDomainAdapterProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08BrowserDomainAdapterProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08BrowserDomainAdapterProofRequirementSchema),
  claimBoundary: V08BrowserDomainAdapterProofClaimBoundarySchema,
  fallbackBehavior: V08BrowserDomainAdapterProofFallbackSchema,
  managedExactUrlClaimed: Schema.Boolean,
  unmanagedExactUrlClaimed: Schema.Boolean,
  networkDomainBlockingClaimed: Schema.Boolean,
  broadBrowserControlClaimed: Schema.Boolean,
  unsupportedOsClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

const V08WindowsAppControlProofStateBaseSchema = Schema.Struct({
  proofStateId: V08WindowsAppControlProofStateIdSchema,
  readinessState: V08WindowsAppControlReadinessStateSchema,
  policyMutationState: V08WindowsAppControlPolicyMutationStateSchema,
  ruleIdentityKinds: Schema.Array(V08WindowsAppControlRuleIdentityKindSchema),
  adminRequirement: V08WindowsAppControlAdminRequirementSchema,
  eventStates: Schema.Array(V08WindowsAppControlEventStateSchema),
  manualProofRequirements: Schema.Array(V08BrowserDomainAdapterProofRequirementSchema),
  claimBoundary: V08BrowserDomainAdapterProofClaimBoundarySchema,
  fallbackBehavior: V08BrowserDomainAdapterProofFallbackSchema,
  appControlPreventionClaimed: Schema.Boolean,
  policyCreationClaimed: Schema.Boolean,
  policyUpdateClaimed: Schema.Boolean,
  rollbackClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08BrowserDomainAdapterProofEntryCandidate = Infer<typeof V08BrowserDomainAdapterProofEntryBaseSchema>;
type V08WindowsAppControlProofStateCandidate = Infer<typeof V08WindowsAppControlProofStateBaseSchema>;

export const V08BrowserDomainAdapterProofEntrySchema = withParser(
  V08BrowserDomainAdapterProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserDomainAdapterEntryIsHonest(entry) ||
        'Expected V0.8 browser/domain adapter proof entries to preserve implemented, degraded, manual-required, unavailable, and not-claimed boundaries'
    )
  )
);

export const V08WindowsAppControlProofStateSchema = withParser(
  V08WindowsAppControlProofStateBaseSchema.pipe(
    Schema.filter(
      (state) =>
        windowsAppControlProofStateIsHonest(state) ||
        'Expected Windows AppLocker/App Control proof states to represent readiness, audit-only, enforced, manual-required, unavailable, and failed states without claiming OS prevention'
    )
  )
);

export const V08BrowserDomainAdapterProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08BrowserDomainAdapterProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08BrowserDomainAdapterProofReferenceSchema),
    windowsAppControlStates: Schema.Array(V08WindowsAppControlProofStateSchema),
    entries: Schema.Array(V08BrowserDomainAdapterProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.proofEntryId)).size === readModel.entries.length ||
        'Expected V0.8 browser/domain adapter proof entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        windowsAppControlProofStatesAreComplete(readModel.windowsAppControlStates) ||
        'Expected Windows AppLocker/App Control proof states to cover readiness, audit-only, enforced, manual-required, unavailable, and failed states'
    )
  )
);

function browserDomainAdapterEntryIsHonest(entry: V08BrowserDomainAdapterProofEntryCandidate): boolean {
  if (browserDomainAdapterEntryHasClaimUpgrade(entry)) {
    return false;
  }

  return browserDomainAdapterEntryMatchesSurfaceExpectation(entry);
}

function browserDomainAdapterEntryHasClaimUpgrade(entry: V08BrowserDomainAdapterProofEntryCandidate): boolean {
  return [
    entry.managedExactUrlClaimed,
    entry.unmanagedExactUrlClaimed,
    entry.networkDomainBlockingClaimed,
    entry.broadBrowserControlClaimed,
    entry.unsupportedOsClaimed,
  ].some(Boolean);
}

const requiredWindowsAppControlReadinessStates = [
  'readiness-check',
  'audit-only',
  'enforced',
  'manual-required',
  'unavailable',
  'failed',
] as const;

function windowsAppControlProofStatesAreComplete(states: readonly V08WindowsAppControlProofStateCandidate[]): boolean {
  return requiredWindowsAppControlReadinessStates.every((state) =>
    states.some((candidate) => candidate.readinessState === state)
  );
}

function windowsAppControlProofStateIsHonest(state: V08WindowsAppControlProofStateCandidate): boolean {
  if (
    state.appControlPreventionClaimed ||
    state.policyCreationClaimed ||
    state.policyUpdateClaimed ||
    state.rollbackClaimed
  ) {
    return false;
  }

  switch (state.readinessState) {
    case 'readiness-check':
      return appControlStateMatches(state, 'detect-only', 'administrator-required', true, 'manual-proof-required');
    case 'audit-only':
      return appControlStateMatches(state, 'audit-only-visible', 'administrator-required', true, 'audit-visible');
    case 'enforced':
      return appControlStateMatches(
        state,
        'create-update-manual-required',
        'administrator-required',
        true,
        'rollback-visible'
      );
    case 'manual-required':
      return appControlStateMatches(
        state,
        'manual-setup-required',
        'manual-operator-required',
        true,
        'manual-proof-required'
      );
    case 'unavailable':
      return appControlStateMatches(state, 'unavailable', 'service-permission-required', false, 'unavailable');
    case 'failed':
      return appControlStateMatches(state, 'failed', 'administrator-required', true, 'failure-visible');
  }
}

function appControlStateMatches(
  state: V08WindowsAppControlProofStateCandidate,
  policyMutationState: V08WindowsAppControlPolicyMutationState,
  adminRequirement: V08WindowsAppControlAdminRequirement,
  requiresRuleIdentityKinds: boolean,
  requiredEventState: V08WindowsAppControlEventState
): boolean {
  return (
    state.policyMutationState === policyMutationState &&
    state.adminRequirement === adminRequirement &&
    state.eventStates.includes(requiredEventState) &&
    state.manualProofRequirements.length > 0 &&
    (requiresRuleIdentityKinds ? state.ruleIdentityKinds.length === 4 : state.ruleIdentityKinds.length === 0)
  );
}

function browserDomainAdapterEntryMatchesSurfaceExpectation(
  entry: V08BrowserDomainAdapterProofEntryCandidate
): boolean {
  const expectation = browserDomainAdapterSurfaceExpectations.find((candidate) => candidate.surface === entry.surface);
  if (expectation === undefined) {
    return false;
  }

  return (
    entry.platform === expectation.platform &&
    entry.capability === expectation.capability &&
    entry.capabilityStatus === expectation.capabilityStatus &&
    entry.evidenceKind === expectation.evidenceKind &&
    entry.productClaimState === expectation.productClaimState &&
    entry.adapterExecutionState === expectation.adapterExecutionState &&
    browserDomainAdapterEntryMatchesEvidenceExpectation(entry, expectation.evidenceExpectation)
  );
}

function browserDomainAdapterEntryMatchesEvidenceExpectation(
  entry: V08BrowserDomainAdapterProofEntryCandidate,
  evidenceExpectation: BrowserDomainAdapterEvidenceExpectation
): boolean {
  switch (evidenceExpectation) {
    case 'linked-proof':
      return (
        entry.linkedProofCommands.length > 0 &&
        entry.linkedProofArtifacts.length > 0 &&
        entry.manualProofRequirements.length === 0
      );
    case 'linked-degraded-proof':
      return (
        entry.linkedProofCommands.length > 0 &&
        entry.linkedProofArtifacts.length > 0 &&
        entry.manualProofRequirements.length > 0
      );
    case 'manual-proof':
      return (
        entry.linkedProofCommands.length === 0 &&
        entry.linkedProofArtifacts.length === 0 &&
        entry.manualProofRequirements.length > 0
      );
  }
}

type BrowserDomainAdapterEvidenceExpectation = 'linked-proof' | 'linked-degraded-proof' | 'manual-proof';

type BrowserDomainAdapterSurfaceExpectation = {
  surface: V08BrowserDomainAdapterProofSurface;
  platform: ParentControlPlatform;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind;
  productClaimState: V08BrowserDomainAdapterProofClaimState;
  adapterExecutionState: V08BrowserDomainAdapterExecutionState;
  evidenceExpectation: BrowserDomainAdapterEvidenceExpectation;
};

const browserDomainAdapterSurfaceExpectations: readonly BrowserDomainAdapterSurfaceExpectation[] = [
  {
    surface: 'windows-managed-browser-intervention-state',
    platform: 'windows',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind: 'managed-browser',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'windows-managed-browser-exact-url-manual',
    platform: 'windows',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind: 'managed-browser',
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'windows-unmanaged-browser-terminate-boundary',
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind: 'unmanaged-browser',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'windows-unmanaged-browser-warn-noop',
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    evidenceKind: 'unmanaged-browser',
    productClaimState: 'degraded-boundary',
    adapterExecutionState: 'returns-degraded-noop',
    evidenceExpectation: 'linked-degraded-proof',
  },
  {
    surface: 'windows-unmanaged-browser-exact-evidence-not-claimed',
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    capabilityStatus: ParentControlCapabilityStatus.NotImplemented,
    evidenceKind: 'unmanaged-browser',
    productClaimState: 'not-claimed',
    adapterExecutionState: 'not-invoked',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'windows-network-domain-filter-manual',
    platform: 'windows',
    capability: ParentControlCapabilityName.NetworkDomainBlocking,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind: 'network-domain',
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'windows-network-domain-adapter-unavailable',
    platform: 'windows',
    capability: ParentControlCapabilityName.NetworkDomainBlocking,
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    evidenceKind: 'network-domain',
    productClaimState: 'unavailable',
    adapterExecutionState: 'returns-unavailable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'windows-audit-visibility-boundary',
    platform: 'windows',
    capability: ParentControlCapabilityName.LocalStorage,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind: 'audit',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'windows-restart-recovery-visibility-boundary',
    platform: 'windows',
    capability: ParentControlCapabilityName.AppTimeLimit,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind: 'restart-recovery',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'windows-browser-policy-rollback-visibility',
    platform: 'windows',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind: 'rollback',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'linux-browser-domain-adapter-unavailable',
    platform: 'linux',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    evidenceKind: 'unsupported-target',
    productClaimState: 'unavailable',
    adapterExecutionState: 'returns-unavailable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'macos-browser-domain-adapter-unavailable',
    platform: 'macos',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    evidenceKind: 'unsupported-target',
    productClaimState: 'unavailable',
    adapterExecutionState: 'returns-unavailable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'android-browser-domain-adapter-manual',
    platform: 'android',
    capability: ParentControlCapabilityName.VpnDnsFiltering,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind: 'unsupported-target',
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'ios-browser-domain-adapter-manual',
    platform: 'ios',
    capability: ParentControlCapabilityName.NetworkExtension,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind: 'unsupported-target',
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    evidenceExpectation: 'manual-proof',
  },
];

export type V08BrowserDomainAdapterProofReadModelId = typeof V08BrowserDomainAdapterProofReadModelIdSchema.Type;
export type V08BrowserDomainAdapterProofEntryId = typeof V08BrowserDomainAdapterProofEntryIdSchema.Type;
export type V08BrowserDomainAdapterProofReference = typeof V08BrowserDomainAdapterProofReferenceSchema.Type;
export type V08BrowserDomainAdapterProofRequirement = typeof V08BrowserDomainAdapterProofRequirementSchema.Type;
export type V08BrowserDomainAdapterProofClaimBoundary = typeof V08BrowserDomainAdapterProofClaimBoundarySchema.Type;
export type V08BrowserDomainAdapterProofFallback = typeof V08BrowserDomainAdapterProofFallbackSchema.Type;
export type V08BrowserDomainAdapterProofSurface = Infer<typeof V08BrowserDomainAdapterProofSurfaceSchema>;
export type V08BrowserDomainAdapterProofEvidenceKind = Infer<typeof V08BrowserDomainAdapterProofEvidenceKindSchema>;
export type V08BrowserDomainAdapterProofClaimState = Infer<typeof V08BrowserDomainAdapterProofClaimStateSchema>;
export type V08BrowserDomainAdapterExecutionState = Infer<typeof V08BrowserDomainAdapterExecutionStateSchema>;
export type V08WindowsAppControlProofStateId = typeof V08WindowsAppControlProofStateIdSchema.Type;
export type V08WindowsAppControlReadinessState = Infer<typeof V08WindowsAppControlReadinessStateSchema>;
export type V08WindowsAppControlPolicyMutationState = Infer<typeof V08WindowsAppControlPolicyMutationStateSchema>;
export type V08WindowsAppControlRuleIdentityKind = Infer<typeof V08WindowsAppControlRuleIdentityKindSchema>;
export type V08WindowsAppControlAdminRequirement = Infer<typeof V08WindowsAppControlAdminRequirementSchema>;
export type V08WindowsAppControlEventState = Infer<typeof V08WindowsAppControlEventStateSchema>;
export type V08WindowsAppControlProofState = Infer<typeof V08WindowsAppControlProofStateSchema>;
export type V08BrowserDomainAdapterProofEntry = Infer<typeof V08BrowserDomainAdapterProofEntrySchema>;
export type V08BrowserDomainAdapterProofReadModel = Infer<typeof V08BrowserDomainAdapterProofReadModelSchema>;

type V08BrowserDomainAdapterProofEntryInput = {
  proofEntryId: string;
  surface: V08BrowserDomainAdapterProofSurface;
  platform: ParentControlPlatform;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind;
  productClaimState: V08BrowserDomainAdapterProofClaimState;
  adapterExecutionState: V08BrowserDomainAdapterExecutionState;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08BrowserDomainAdapterProofSurface = {
  WindowsManagedBrowserInterventionState: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-managed-browser-intervention-state'
  ),
  WindowsManagedBrowserExactUrlManual: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-managed-browser-exact-url-manual'
  ),
  WindowsUnmanagedBrowserTerminateBoundary: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-unmanaged-browser-terminate-boundary'
  ),
  WindowsUnmanagedBrowserWarnNoop: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-unmanaged-browser-warn-noop'
  ),
  WindowsUnmanagedBrowserExactEvidenceNotClaimed: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-unmanaged-browser-exact-evidence-not-claimed'
  ),
  WindowsNetworkDomainFilterManual: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-network-domain-filter-manual'
  ),
  WindowsNetworkDomainAdapterUnavailable: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-network-domain-adapter-unavailable'
  ),
  WindowsAuditVisibilityBoundary: V08BrowserDomainAdapterProofSurfaceSchema.parse('windows-audit-visibility-boundary'),
  WindowsRestartRecoveryVisibilityBoundary: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-restart-recovery-visibility-boundary'
  ),
  WindowsBrowserPolicyRollbackVisibility: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'windows-browser-policy-rollback-visibility'
  ),
  LinuxBrowserDomainAdapterUnavailable: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'linux-browser-domain-adapter-unavailable'
  ),
  MacosBrowserDomainAdapterUnavailable: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'macos-browser-domain-adapter-unavailable'
  ),
  AndroidBrowserDomainAdapterManual: V08BrowserDomainAdapterProofSurfaceSchema.parse(
    'android-browser-domain-adapter-manual'
  ),
  IosBrowserDomainAdapterManual: V08BrowserDomainAdapterProofSurfaceSchema.parse('ios-browser-domain-adapter-manual'),
} as const;

const documentedAt = '2026-05-30T20:20:00.000Z';
const windowsAppControlRuleIdentityKinds = ['publisher', 'path', 'hash', 'package'] as const;

export const V08BrowserDomainAdapterProofReadModel = V08BrowserDomainAdapterProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-browser-domain-adapter-proof',
  generatedAt: documentedAt,
  sourceReadModelIds: [
    'v0-8-broad-os-adapter-proof',
    'v0-8-cross-platform-enforcement-capability-proof',
    'v0-8-os-adapter-product-proof',
    'browser-policy-runtime',
  ],
  windowsAppControlStates: [
    windowsAppControlState(
      'v0-8-windows-app-control-readiness-detect-only',
      'readiness-check',
      'detect-only',
      windowsAppControlRuleIdentityKinds,
      'administrator-required',
      ['manual-proof-required'],
      ['Windows edition and AppLocker or WDAC availability proof', 'administrator permission proof'],
      'Windows AppLocker/App Control readiness can be represented only as a detect/manual setup state until host policy artifacts exist.',
      'Return manual-required when edition, permission, policy provider, or identity-target proof is missing.'
    ),
    windowsAppControlState(
      'v0-8-windows-app-control-audit-only-visible',
      'audit-only',
      'audit-only-visible',
      windowsAppControlRuleIdentityKinds,
      'administrator-required',
      ['audit-visible'],
      ['audit-mode policy artifact', 'AppLocker or WDAC audit event query proof'],
      'Audit-only AppLocker/App Control state is visible as a readiness/audit proof state and does not block launch.',
      'Keep prevention false and surface audit-only status until enforce-mode apply and rollback proof exists.'
    ),
    windowsAppControlState(
      'v0-8-windows-app-control-enforced-manual-required',
      'enforced',
      'create-update-manual-required',
      windowsAppControlRuleIdentityKinds,
      'administrator-required',
      ['audit-visible', 'rollback-visible'],
      ['AppLocker or WDAC enforced policy apply artifact', 'policy refresh result', 'rollback result'],
      'Enforced AppLocker/App Control mode remains manual-required until real policy create/update, refresh, audit, and rollback artifacts prove launch prevention.',
      'Do not claim launch blocking from requested policy state; require host apply and rollback proof.'
    ),
    windowsAppControlState(
      'v0-8-windows-app-control-manual-required',
      'manual-required',
      'manual-setup-required',
      windowsAppControlRuleIdentityKinds,
      'manual-operator-required',
      ['manual-proof-required'],
      ['parent-visible manual setup state', 'operator confirmation path', 'identity target review proof'],
      'Manual-required AppLocker/App Control setup is represented separately from unavailable and enforced states.',
      'Show manual setup rather than silently downgrading to process termination or browser-domain blocking.'
    ),
    windowsAppControlState(
      'v0-8-windows-app-control-unavailable',
      'unavailable',
      'unavailable',
      [],
      'service-permission-required',
      ['unavailable'],
      ['unsupported Windows edition or missing policy provider proof', 'service permission denial event'],
      'Unavailable AppLocker/App Control state records when the host cannot provide a policy adapter or permission path.',
      'Return unavailable and keep unmanaged fallback manual when policy provider or service permission is absent.'
    ),
    windowsAppControlState(
      'v0-8-windows-app-control-policy-failed',
      'failed',
      'failed',
      windowsAppControlRuleIdentityKinds,
      'administrator-required',
      ['failure-visible'],
      ['policy create or update failure event', 'policy target identity failure event', 'audit failure event'],
      'Failed AppLocker/App Control state is parent-visible as a policy/audit failure without claiming a blocking result.',
      'Record failure and rollback/manual setup requirements; do not treat failed apply as enforcement.'
    ),
  ],
  entries: [
    implementedEntry(
      'v0-8-browser-domain-managed-intervention-state',
      V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserInterventionState,
      ParentControlCapabilityName.ManagedBrowserControl,
      'managed-browser',
      ['node scripts/test/managed-browser-intervention-proof.mjs'],
      ['test-results/managed-browser-intervention-proof/proof.json'],
      'Managed browser intervention state is limited to the Ocentra-owned managed-session boundary and does not prove exact active-tab URL enforcement.',
      'Return manual-required when managed browser launch, active-tab, exact URL, rollback, or audit proof is missing.'
    ),
    manualRequiredEntry(
      'v0-8-browser-domain-managed-exact-url-manual',
      V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserExactUrlManual,
      'windows',
      ParentControlCapabilityName.ManagedBrowserControl,
      'managed-browser',
      ['managed active-tab evidence', 'exact URL apply result', 'rollback result', 'audit custody artifact'],
      'Managed exact URL enforcement remains manual-required because a command target string is not foreground active-tab proof.',
      'Return manual-required until live active-tab, exact URL apply, rollback, and custody artifacts exist.'
    ),
    implementedEntry(
      'v0-8-browser-domain-unmanaged-terminate-boundary',
      V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserTerminateBoundary,
      ParentControlCapabilityName.UnmanagedBrowserDetection,
      'unmanaged-browser',
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
      'Unmanaged browser terminate proof is process-only with pid/name guardrails and is not exact URL, tab, title, page, download, or intent evidence.',
      'Reject missing pid or process-name mismatch; keep URL certainty unclaimed without browser integration.'
    ),
    degradedEntry(
      'v0-8-browser-domain-unmanaged-warn-noop',
      V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserWarnNoop,
      ParentControlCapabilityName.UnmanagedBrowserDetection,
      'unmanaged-browser',
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['unmanaged browser warning no-op service event'],
      ['parent-visible warning delivery proof', 'browser integration proof'],
      'Unmanaged browser warn behavior is a degraded no-op boundary until notification delivery and browser integration exist.',
      'Return a degraded no-op instead of claiming warning delivery or URL-aware browser control.'
    ),
    notClaimedEntry(
      'v0-8-browser-domain-unmanaged-exact-evidence-not-claimed',
      V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserExactEvidenceNotClaimed,
      'windows',
      ParentControlCapabilityName.UnmanagedBrowserDetection,
      'unmanaged-browser',
      ['managed profile integration', 'browser extension or protocol integration', 'active tab custody evidence'],
      'Unmanaged browser exact URL, active tab, title, page, download source, HTTPS content, and intent evidence remain not-claimed.',
      'Do not infer browser content from process names or command targets.'
    ),
    manualRequiredEntry(
      'v0-8-browser-domain-network-filter-manual',
      V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainFilterManual,
      'windows',
      ParentControlCapabilityName.NetworkDomainBlocking,
      'network-domain',
      ['host network filter adapter', 'DNS or VPN apply result', 'rollback result', 'audit custody artifact'],
      'Network/domain blocking remains manual-required and is not proved by domain observation or browser policy records.',
      'Return manual-required until host DNS/VPN/filter apply, rollback, and custody evidence exists.'
    ),
    unavailableEntry(
      'v0-8-browser-domain-network-adapter-unavailable',
      V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainAdapterUnavailable,
      'windows',
      ParentControlCapabilityName.NetworkDomainBlocking,
      'network-domain',
      ['service unavailable event', 'adapter install evidence', 'operator retry path'],
      'The current Windows service boundary can report network/domain adapter unavailable states but does not perform host filtering.',
      'Return unavailable when the host filter adapter is absent or unsupported.'
    ),
    implementedEntry(
      'v0-8-browser-domain-audit-visibility',
      V08BrowserDomainAdapterProofSurface.WindowsAuditVisibilityBoundary,
      ParentControlCapabilityName.LocalStorage,
      'audit',
      ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
      ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      'Audit visibility is limited to existing enforcement journal and browser policy event seams; it is not proof of broad app/domain enforcement.',
      'Return unavailable when the local audit store or event payload cannot be read.'
    ),
    implementedEntry(
      'v0-8-browser-domain-restart-recovery-visibility',
      V08BrowserDomainAdapterProofSurface.WindowsRestartRecoveryVisibilityBoundary,
      ParentControlCapabilityName.AppTimeLimit,
      'restart-recovery',
      ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
      ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      'Restart recovery visibility is limited to app time-limit state recovery and cannot upgrade browser/domain blocking support.',
      'Return unavailable when persisted timer state is missing or incompatible.'
    ),
    implementedEntry(
      'v0-8-browser-domain-browser-policy-rollback-visibility',
      V08BrowserDomainAdapterProofSurface.WindowsBrowserPolicyRollbackVisibility,
      ParentControlCapabilityName.ManagedBrowserControl,
      'rollback',
      ['cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision'],
      ['crates/agent-service/src/browser_policy_api_tests.rs'],
      'Browser policy rollback visibility proves stored policy revision rollback only and does not prove host-level browser/domain enforcement rollback.',
      'Return manual-required for managed exact URL, network/domain, or unmanaged browser rollback until host artifacts exist.'
    ),
    unavailableEntry(
      'v0-8-browser-domain-linux-adapter-unavailable',
      V08BrowserDomainAdapterProofSurface.LinuxBrowserDomainAdapterUnavailable,
      'linux',
      ParentControlCapabilityName.ManagedBrowserControl,
      'unsupported-target',
      ['Linux service-manager proof', 'Linux browser/domain adapter proof'],
      'Linux browser/domain adapter behavior is unavailable in this proof and cannot inherit Windows managed browser behavior.',
      'Report unavailable until Linux-specific browser/domain apply, rollback, and audit proof exists.'
    ),
    unavailableEntry(
      'v0-8-browser-domain-macos-adapter-unavailable',
      V08BrowserDomainAdapterProofSurface.MacosBrowserDomainAdapterUnavailable,
      'macos',
      ParentControlCapabilityName.ManagedBrowserControl,
      'unsupported-target',
      ['macOS permission proof', 'macOS browser/domain adapter proof'],
      'macOS browser/domain adapter behavior is unavailable in this proof and cannot inherit Windows managed browser behavior.',
      'Report unavailable until macOS-specific browser/domain permissions, apply, rollback, and audit proof exists.'
    ),
    manualRequiredEntry(
      'v0-8-browser-domain-android-adapter-manual',
      V08BrowserDomainAdapterProofSurface.AndroidBrowserDomainAdapterManual,
      'android',
      ParentControlCapabilityName.VpnDnsFiltering,
      'unsupported-target',
      ['Android VPN or DNS filtering proof', 'device-owner or managed-profile proof', 'package lifecycle proof'],
      'Android browser/domain control is manual-required and is not implied by desktop managed-browser or network-domain proof.',
      'Return manual-required until real Android package, permission, VPN/DNS, device-owner, and lifecycle artifacts exist.'
    ),
    manualRequiredEntry(
      'v0-8-browser-domain-ios-adapter-manual',
      V08BrowserDomainAdapterProofSurface.IosBrowserDomainAdapterManual,
      'ios',
      ParentControlCapabilityName.NetworkExtension,
      'unsupported-target',
      [
        'Network Extension entitlement proof',
        'Family Controls or DeviceActivity proof',
        'TestFlight or device artifact',
      ],
      'iOS browser/domain control is manual-required and cannot be inferred from desktop or Android proofs.',
      'Return manual-required until approved entitlement, signing, install, and device evidence exists.'
    ),
  ],
});

function windowsAppControlState(
  proofStateId: string,
  readinessState: V08WindowsAppControlReadinessState,
  policyMutationState: V08WindowsAppControlPolicyMutationState,
  ruleIdentityKinds: readonly V08WindowsAppControlRuleIdentityKind[],
  adminRequirement: V08WindowsAppControlAdminRequirement,
  eventStates: readonly V08WindowsAppControlEventState[],
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08WindowsAppControlProofState {
  return V08WindowsAppControlProofStateSchema.parse({
    proofStateId,
    readinessState,
    policyMutationState,
    ruleIdentityKinds: [...ruleIdentityKinds],
    adminRequirement,
    eventStates: [...eventStates],
    manualProofRequirements: [...manualProofRequirements],
    claimBoundary,
    fallbackBehavior,
    appControlPreventionClaimed: false,
    policyCreationClaimed: false,
    policyUpdateClaimed: false,
    rollbackClaimed: false,
    lastCheckedAt: documentedAt,
  });
}

function implementedEntry(
  proofEntryId: string,
  surface: V08BrowserDomainAdapterProofSurface,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind,
  linkedProofCommands: readonly string[],
  linkedProofArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BrowserDomainAdapterProofEntry {
  return entry({
    proofEntryId,
    surface,
    platform: 'windows',
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    evidenceKind,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    linkedProofCommands,
    linkedProofArtifacts,
    manualProofRequirements: [],
    claimBoundary,
    fallbackBehavior,
  });
}

function degradedEntry(
  proofEntryId: string,
  surface: V08BrowserDomainAdapterProofSurface,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind,
  linkedProofCommands: readonly string[],
  linkedProofArtifacts: readonly string[],
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BrowserDomainAdapterProofEntry {
  return entry({
    proofEntryId,
    surface,
    platform: 'windows',
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    evidenceKind,
    productClaimState: 'degraded-boundary',
    adapterExecutionState: 'returns-degraded-noop',
    linkedProofCommands,
    linkedProofArtifacts,
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function manualRequiredEntry(
  proofEntryId: string,
  surface: V08BrowserDomainAdapterProofSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BrowserDomainAdapterProofEntry {
  return entry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function unavailableEntry(
  proofEntryId: string,
  surface: V08BrowserDomainAdapterProofSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BrowserDomainAdapterProofEntry {
  return entry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    evidenceKind,
    productClaimState: 'unavailable',
    adapterExecutionState: 'returns-unavailable',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function notClaimedEntry(
  proofEntryId: string,
  surface: V08BrowserDomainAdapterProofSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08BrowserDomainAdapterProofEvidenceKind,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08BrowserDomainAdapterProofEntry {
  return entry({
    proofEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.NotImplemented,
    evidenceKind,
    productClaimState: 'not-claimed',
    adapterExecutionState: 'not-invoked',
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function entry(input: V08BrowserDomainAdapterProofEntryInput): V08BrowserDomainAdapterProofEntry {
  return V08BrowserDomainAdapterProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofEntryId: input.proofEntryId,
    surface: input.surface,
    platform: input.platform,
    capability: input.capability,
    capabilityStatus: input.capabilityStatus,
    evidenceKind: input.evidenceKind,
    productClaimState: input.productClaimState,
    adapterExecutionState: input.adapterExecutionState,
    linkedProofCommands: [...input.linkedProofCommands],
    linkedProofArtifacts: [...input.linkedProofArtifacts],
    manualProofRequirements: [...input.manualProofRequirements],
    claimBoundary: input.claimBoundary,
    fallbackBehavior: input.fallbackBehavior,
    managedExactUrlClaimed: false,
    unmanagedExactUrlClaimed: false,
    networkDomainBlockingClaimed: false,
    broadBrowserControlClaimed: false,
    unsupportedOsClaimed: false,
    lastCheckedAt: documentedAt,
  });
}

