import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentControlCapabilityName,
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatus,
  ParentControlCapabilityStatusSchema,
  type ParentControlPlatform,
  ParentControlPlatformSchema,
} from './capabilities';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyProductControlSpineText = Schema.String.pipe(Schema.minLength(1));

export const V08EnforcementProductControlSpineReadModelIdSchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineReadModelId')
);
export const V08EnforcementProductControlSpineEntryIdSchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineEntryId')
);
export const V08EnforcementProductControlSpineReferenceSchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineReference')
);
export const V08EnforcementProductControlSpineRequirementSchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineRequirement')
);
export const V08EnforcementProductControlSpineClaimBoundarySchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineClaimBoundary')
);
export const V08EnforcementProductControlSpineFallbackSchema = NonEmptyProductControlSpineText.pipe(
  Schema.brand('V08EnforcementProductControlSpineFallback')
);

export const V08EnforcementProductControlSurfaceSchema = withParser(
  Schema.Literal(
    'windows-owned-process-time-limit',
    'windows-app-time-limit-lifecycle',
    'windows-managed-browser-session-intervention',
    'windows-unmanaged-browser-process-fallback',
    'windows-policy-dry-run-preview',
    'windows-approval-override-audit',
    'windows-restart-recovery-timer',
    'windows-rollback-audit-boundary',
    'windows-child-facing-explanation',
    'windows-broad-app-blocking',
    'windows-network-domain-blocking',
    'windows-managed-exact-url-control',
    'windows-unmanaged-exact-url-not-claimed',
    'windows-permission-loss-alerts',
    'windows-tamper-uninstall-alerts'
  )
);

export const V08EnforcementProductControlSurfaceKindSchema = withParser(
  Schema.Literal(
    'process',
    'app-game',
    'managed-browser',
    'unmanaged-browser',
    'network-domain',
    'policy',
    'recovery',
    'audit',
    'child-explanation',
    'integrity'
  )
);

export const V08EnforcementProductControlClaimStateSchema = withParser(
  Schema.Literal(
    'implemented-boundary',
    'degraded-boundary',
    'dry-run-only',
    'manual-required',
    'unavailable',
    'not-claimed'
  )
);

export const V08EnforcementProductControlExecutionStateSchema = withParser(
  Schema.Literal(
    'executes-real-service',
    'returns-dry-run-preview',
    'returns-degraded-noop',
    'returns-manual-required',
    'returns-unavailable',
    'not-invoked'
  )
);

export const V08EnforcementProductControlParentActionSchema = withParser(
  Schema.Literal(
    'observe',
    'warn',
    'time-limit',
    'block-scoped-process',
    'ask-parent',
    'dry-run-preview',
    'report-only'
  )
);

export const V08EnforcementProductControlDevicePolicyStateSchema = withParser(
  Schema.Literal('control-capable', 'preview-only', 'report-only', 'manual-required', 'unavailable', 'not-claimed')
);

const V08EnforcementProductControlSpineEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  entryId: V08EnforcementProductControlSpineEntryIdSchema,
  surface: V08EnforcementProductControlSurfaceSchema,
  surfaceKind: V08EnforcementProductControlSurfaceKindSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  productClaimState: V08EnforcementProductControlClaimStateSchema,
  adapterExecutionState: V08EnforcementProductControlExecutionStateSchema,
  devicePolicyState: V08EnforcementProductControlDevicePolicyStateSchema,
  parentVisibleActions: Schema.Array(V08EnforcementProductControlParentActionSchema),
  linkedProofCommands: Schema.Array(V08EnforcementProductControlSpineReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08EnforcementProductControlSpineReferenceSchema),
  manualProofRequirements: Schema.Array(V08EnforcementProductControlSpineRequirementSchema),
  claimBoundary: V08EnforcementProductControlSpineClaimBoundarySchema,
  fallbackBehavior: V08EnforcementProductControlSpineFallbackSchema,
  broadAppBlockingClaimed: Schema.Boolean,
  networkDomainBlockingClaimed: Schema.Boolean,
  managedExactUrlBlockingClaimed: Schema.Boolean,
  unmanagedExactUrlClaimed: Schema.Boolean,
  tamperResistanceClaimed: Schema.Boolean,
  notificationDeliveryClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08EnforcementProductControlSpineEntryCandidate = Infer<typeof V08EnforcementProductControlSpineEntryBaseSchema>;

export const V08EnforcementProductControlSpineEntrySchema = withParser(
  V08EnforcementProductControlSpineEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        productControlSpineEntryMatchesSurfaceExpectation(entry) ||
        'Expected V0.8 enforcement product-control entries to preserve exact surface state, action, and no-claim boundaries'
    )
  )
);

export const V08EnforcementProductControlSpineReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08EnforcementProductControlSpineReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08EnforcementProductControlSpineReferenceSchema),
    entries: Schema.Array(V08EnforcementProductControlSpineEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.entryId)).size === readModel.entries.length ||
        'Expected V0.8 enforcement product-control spine entry ids to be unique'
    )
  )
);

function productControlSpineEntryMatchesSurfaceExpectation(
  entry: V08EnforcementProductControlSpineEntryCandidate
): boolean {
  if (productControlSpineEntryHasClaimUpgrade(entry)) {
    return false;
  }

  const expectation = productControlSurfaceExpectations.find((candidate) => candidate.surface === entry.surface);
  if (expectation === undefined) {
    return false;
  }

  return (
    entry.surfaceKind === expectation.surfaceKind &&
    entry.platform === expectation.platform &&
    entry.capability === expectation.capability &&
    entry.capabilityStatus === expectation.capabilityStatus &&
    entry.productClaimState === expectation.productClaimState &&
    entry.adapterExecutionState === expectation.adapterExecutionState &&
    entry.devicePolicyState === expectation.devicePolicyState &&
    productControlActionsMatch(entry.parentVisibleActions, expectation.parentVisibleActions) &&
    productControlEvidenceMatches(entry, expectation.evidenceExpectation)
  );
}

function productControlSpineEntryHasClaimUpgrade(entry: V08EnforcementProductControlSpineEntryCandidate): boolean {
  return [
    entry.broadAppBlockingClaimed,
    entry.networkDomainBlockingClaimed,
    entry.managedExactUrlBlockingClaimed,
    entry.unmanagedExactUrlClaimed,
    entry.tamperResistanceClaimed,
    entry.notificationDeliveryClaimed,
  ].some(Boolean);
}

function productControlActionsMatch(
  actualActions: readonly V08EnforcementProductControlParentAction[],
  expectedActions: readonly V08EnforcementProductControlParentAction[]
): boolean {
  return (
    actualActions.length === expectedActions.length &&
    expectedActions.every((expectedAction, index) => actualActions[index] === expectedAction)
  );
}

function productControlEvidenceMatches(
  entry: V08EnforcementProductControlSpineEntryCandidate,
  evidenceExpectation: ProductControlEvidenceExpectation
): boolean {
  switch (evidenceExpectation) {
    case 'linked-proof':
      return (
        entry.linkedProofCommands.length > 0 &&
        entry.linkedProofArtifacts.length > 0 &&
        entry.manualProofRequirements.length === 0
      );
    case 'linked-degraded-proof':
      return entry.linkedProofCommands.length > 0 && entry.linkedProofArtifacts.length > 0;
    case 'manual-proof':
      return (
        entry.linkedProofCommands.length === 0 &&
        entry.linkedProofArtifacts.length === 0 &&
        entry.manualProofRequirements.length > 0
      );
    case 'not-claimed-proof':
      return (
        entry.linkedProofCommands.length === 0 &&
        entry.linkedProofArtifacts.length === 0 &&
        entry.manualProofRequirements.length > 0
      );
  }
}

export type V08EnforcementProductControlSpineReadModelId =
  typeof V08EnforcementProductControlSpineReadModelIdSchema.Type;
export type V08EnforcementProductControlSpineEntryId = typeof V08EnforcementProductControlSpineEntryIdSchema.Type;
export type V08EnforcementProductControlSpineReference = typeof V08EnforcementProductControlSpineReferenceSchema.Type;
export type V08EnforcementProductControlSpineRequirement =
  typeof V08EnforcementProductControlSpineRequirementSchema.Type;
export type V08EnforcementProductControlSpineClaimBoundary =
  typeof V08EnforcementProductControlSpineClaimBoundarySchema.Type;
export type V08EnforcementProductControlSpineFallback = typeof V08EnforcementProductControlSpineFallbackSchema.Type;
export type V08EnforcementProductControlSurface = Infer<typeof V08EnforcementProductControlSurfaceSchema>;
export type V08EnforcementProductControlSurfaceKind = Infer<typeof V08EnforcementProductControlSurfaceKindSchema>;
export type V08EnforcementProductControlClaimState = Infer<typeof V08EnforcementProductControlClaimStateSchema>;
export type V08EnforcementProductControlExecutionState = Infer<typeof V08EnforcementProductControlExecutionStateSchema>;
export type V08EnforcementProductControlParentAction = Infer<typeof V08EnforcementProductControlParentActionSchema>;
export type V08EnforcementProductControlDevicePolicyState = Infer<
  typeof V08EnforcementProductControlDevicePolicyStateSchema
>;
export type V08EnforcementProductControlSpineEntry = Infer<typeof V08EnforcementProductControlSpineEntrySchema>;
export type V08EnforcementProductControlSpineReadModel = Infer<typeof V08EnforcementProductControlSpineReadModelSchema>;

type ProductControlEvidenceExpectation =
  | 'linked-proof'
  | 'linked-degraded-proof'
  | 'manual-proof'
  | 'not-claimed-proof';

type ProductControlSurfaceExpectation = {
  surface: V08EnforcementProductControlSurface;
  surfaceKind: V08EnforcementProductControlSurfaceKind;
  platform: ParentControlPlatform;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  productClaimState: V08EnforcementProductControlClaimState;
  adapterExecutionState: V08EnforcementProductControlExecutionState;
  devicePolicyState: V08EnforcementProductControlDevicePolicyState;
  parentVisibleActions: readonly V08EnforcementProductControlParentAction[];
  evidenceExpectation: ProductControlEvidenceExpectation;
};

type ProductControlEntryInput = ProductControlSurfaceExpectation & {
  entryId: string;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08EnforcementProductControlSurface = {
  WindowsOwnedProcessTimeLimit: V08EnforcementProductControlSurfaceSchema.parse('windows-owned-process-time-limit'),
  WindowsAppTimeLimitLifecycle: V08EnforcementProductControlSurfaceSchema.parse('windows-app-time-limit-lifecycle'),
  WindowsManagedBrowserSessionIntervention: V08EnforcementProductControlSurfaceSchema.parse(
    'windows-managed-browser-session-intervention'
  ),
  WindowsUnmanagedBrowserProcessFallback: V08EnforcementProductControlSurfaceSchema.parse(
    'windows-unmanaged-browser-process-fallback'
  ),
  WindowsPolicyDryRunPreview: V08EnforcementProductControlSurfaceSchema.parse('windows-policy-dry-run-preview'),
  WindowsApprovalOverrideAudit: V08EnforcementProductControlSurfaceSchema.parse('windows-approval-override-audit'),
  WindowsRestartRecoveryTimer: V08EnforcementProductControlSurfaceSchema.parse('windows-restart-recovery-timer'),
  WindowsRollbackAuditBoundary: V08EnforcementProductControlSurfaceSchema.parse('windows-rollback-audit-boundary'),
  WindowsChildFacingExplanation: V08EnforcementProductControlSurfaceSchema.parse('windows-child-facing-explanation'),
  WindowsBroadAppBlocking: V08EnforcementProductControlSurfaceSchema.parse('windows-broad-app-blocking'),
  WindowsNetworkDomainBlocking: V08EnforcementProductControlSurfaceSchema.parse('windows-network-domain-blocking'),
  WindowsManagedExactUrlControl: V08EnforcementProductControlSurfaceSchema.parse('windows-managed-exact-url-control'),
  WindowsUnmanagedExactUrlNotClaimed: V08EnforcementProductControlSurfaceSchema.parse(
    'windows-unmanaged-exact-url-not-claimed'
  ),
  WindowsPermissionLossAlerts: V08EnforcementProductControlSurfaceSchema.parse('windows-permission-loss-alerts'),
  WindowsTamperUninstallAlerts: V08EnforcementProductControlSurfaceSchema.parse('windows-tamper-uninstall-alerts'),
} as const;

const observedAt = '2026-06-01T21:20:00.000Z';

const productControlSurfaceExpectations: readonly ProductControlSurfaceExpectation[] = [
  {
    surface: V08EnforcementProductControlSurface.WindowsOwnedProcessTimeLimit,
    surfaceKind: 'process',
    platform: 'windows',
    capability: ParentControlCapabilityName.OwnedProcessTerminate,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['observe', 'time-limit', 'block-scoped-process'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsAppTimeLimitLifecycle,
    surfaceKind: 'app-game',
    platform: 'windows',
    capability: ParentControlCapabilityName.AppTimeLimit,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['observe', 'time-limit', 'ask-parent'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsManagedBrowserSessionIntervention,
    surfaceKind: 'managed-browser',
    platform: 'windows',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['observe', 'warn', 'time-limit'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsUnmanagedBrowserProcessFallback,
    surfaceKind: 'unmanaged-browser',
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'degraded-boundary',
    adapterExecutionState: 'returns-degraded-noop',
    devicePolicyState: 'report-only',
    parentVisibleActions: ['observe', 'warn', 'report-only'],
    evidenceExpectation: 'linked-degraded-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsPolicyDryRunPreview,
    surfaceKind: 'policy',
    platform: 'windows',
    capability: ParentControlCapabilityName.TypedProtocolBridge,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'dry-run-only',
    adapterExecutionState: 'returns-dry-run-preview',
    devicePolicyState: 'preview-only',
    parentVisibleActions: ['dry-run-preview', 'ask-parent'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsApprovalOverrideAudit,
    surfaceKind: 'policy',
    platform: 'windows',
    capability: ParentControlCapabilityName.TypedProtocolBridge,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['ask-parent', 'report-only'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsRestartRecoveryTimer,
    surfaceKind: 'recovery',
    platform: 'windows',
    capability: ParentControlCapabilityName.AppTimeLimit,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['time-limit', 'report-only'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsRollbackAuditBoundary,
    surfaceKind: 'audit',
    platform: 'windows',
    capability: ParentControlCapabilityName.TypedProtocolBridge,
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
    devicePolicyState: 'report-only',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsChildFacingExplanation,
    surfaceKind: 'child-explanation',
    platform: 'windows',
    capability: ParentControlCapabilityName.TypedProtocolBridge,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsBroadAppBlocking,
    surfaceKind: 'app-game',
    platform: 'windows',
    capability: ParentControlCapabilityName.AppBlocking,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsNetworkDomainBlocking,
    surfaceKind: 'network-domain',
    platform: 'windows',
    capability: ParentControlCapabilityName.NetworkDomainBlocking,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsManagedExactUrlControl,
    surfaceKind: 'managed-browser',
    platform: 'windows',
    capability: ParentControlCapabilityName.ManagedBrowserControl,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsUnmanagedExactUrlNotClaimed,
    surfaceKind: 'unmanaged-browser',
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    capabilityStatus: ParentControlCapabilityStatus.NotImplemented,
    productClaimState: 'not-claimed',
    adapterExecutionState: 'not-invoked',
    devicePolicyState: 'not-claimed',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'not-claimed-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsPermissionLossAlerts,
    surfaceKind: 'integrity',
    platform: 'windows',
    capability: ParentControlCapabilityName.Notifications,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: V08EnforcementProductControlSurface.WindowsTamperUninstallAlerts,
    surfaceKind: 'integrity',
    platform: 'windows',
    capability: ParentControlCapabilityName.PackageLifecycle,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
    evidenceExpectation: 'manual-proof',
  },
];

export const V08EnforcementProductControlSpineReadModel = V08EnforcementProductControlSpineReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-enforcement-product-control-spine',
  generatedAt: observedAt,
  sourceReadModelIds: [
    'v0-8-cross-platform-enforcement-capability-proof',
    'v0-8-browser-domain-adapter-proof',
    'v0-8-os-adapter-product-proof',
    'browser-policy-preview',
  ],
  entries: [
    linkedEntry(
      'v0-8-product-control-owned-process-time-limit',
      V08EnforcementProductControlSurface.WindowsOwnedProcessTimeLimit,
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
      'Owned-process control is limited to pid/name guarded local process behavior and is not broad installed-app blocking.',
      'Reject missing pid or process name mismatch; report unavailable on unsupported hosts.'
    ),
    linkedEntry(
      'v0-8-product-control-app-time-limit',
      V08EnforcementProductControlSurface.WindowsAppTimeLimitLifecycle,
      ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
      ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      'App time-limit lifecycle covers timer create, expiry, parent cancel, restart recovery, and audit for scoped app evidence.',
      'Return unavailable when persisted timer state, adapter state, or target evidence is missing.'
    ),
    linkedEntry(
      'v0-8-product-control-managed-browser-session',
      V08EnforcementProductControlSurface.WindowsManagedBrowserSessionIntervention,
      ['node scripts/test/managed-browser-intervention-proof.mjs'],
      ['test-results/managed-browser-intervention-proof/proof.json'],
      'Managed browser intervention is limited to the Ocentra-owned managed session boundary.',
      'Return manual-required before exact active-tab URL apply, rollback, and custody proof exists.'
    ),
    linkedEntry(
      'v0-8-product-control-unmanaged-browser-process',
      V08EnforcementProductControlSurface.WindowsUnmanagedBrowserProcessFallback,
      ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
      ['test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json'],
      'Unmanaged browser handling is process-observation fallback only; exact URL, title, page, and intent remain unproved.',
      'Warn/report process fallback and keep exact unmanaged URL evidence not claimed.'
    ),
    linkedEntry(
      'v0-8-product-control-policy-dry-run-preview',
      V08EnforcementProductControlSurface.WindowsPolicyDryRunPreview,
      ['cargo test -p ocentra-parent-agent-service policy_preview'],
      ['test-results/policy-preview-proof/proof.json'],
      'Policy preview is dry-run-only and must not execute adapter behavior from portal-authored rules.',
      'Return preview-only state until a child-device agent validates and executes a typed policy decision.'
    ),
    linkedEntry(
      'v0-8-product-control-approval-override-audit',
      V08EnforcementProductControlSurface.WindowsApprovalOverrideAudit,
      ['cargo test -p ocentra-parent-agent-protocol enforcement'],
      ['test-results/enforcement-lan-mobile-product-proof/proof.json'],
      'Approval and override audit references are typed control state, not portal-side enforcement authority.',
      'Reject stale or missing approval references and preserve audit-only state when execution is unavailable.'
    ),
    linkedEntry(
      'v0-8-product-control-restart-recovery-timer',
      V08EnforcementProductControlSurface.WindowsRestartRecoveryTimer,
      ['node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs'],
      ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      'Restart recovery is scoped to persisted timer and enforcement state that the Rust service can recover.',
      'Emit recovery-needed or unavailable state when persisted timer evidence is missing.'
    ),
    linkedEntry(
      'v0-8-product-control-rollback-audit',
      V08EnforcementProductControlSurface.WindowsRollbackAuditBoundary,
      ['cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision'],
      ['test-results/v0-8-browser-domain-adapter-proof/proof.json'],
      'Rollback and audit visibility are service-backed status boundaries, not proof of broad blocking.',
      'Report rollback unavailable when previous revision, custody, or adapter rollback evidence is missing.'
    ),
    manualEntry(
      'v0-8-product-control-child-facing-explanation',
      V08EnforcementProductControlSurface.WindowsChildFacingExplanation,
      ['child-facing status copy contract', 'delivery surface proof', 'audit link from explanation to policy decision'],
      'Child-facing explanation remains manual-required until the child device can show policy reason and request flow state.',
      'Report parent-visible audit state only until child delivery and acknowledgement are proved.'
    ),
    manualEntry(
      'v0-8-product-control-broad-app-blocking',
      V08EnforcementProductControlSurface.WindowsBroadAppBlocking,
      ['OS-approved installed-app identity', 'block apply result', 'rollback result', 'audit custody artifact'],
      'Broad installed-app blocking remains manual-required beyond owned-process terminate and app time-limit proof.',
      'Return manual-required until app/package identity, block apply, rollback, and audit custody artifacts exist.'
    ),
    manualEntry(
      'v0-8-product-control-network-domain-blocking',
      V08EnforcementProductControlSurface.WindowsNetworkDomainBlocking,
      ['host DNS/VPN/filter adapter', 'domain filter apply result', 'rollback result', 'audit custody artifact'],
      'Network/domain blocking remains manual-required and is not proved by network observation metadata.',
      'Return manual-required until DNS/VPN/filter apply, rollback, and custody evidence exists.'
    ),
    manualEntry(
      'v0-8-product-control-managed-exact-url-control',
      V08EnforcementProductControlSurface.WindowsManagedExactUrlControl,
      ['active-tab evidence proof', 'exact URL apply result', 'rollback result', 'audit custody artifact'],
      'Managed exact URL control remains manual-required until active-tab apply/rollback proof exists.',
      'Report managed-session control only; keep exact URL enforcement manual-required.'
    ),
    manualEntry(
      'v0-8-product-control-unmanaged-exact-url-not-claimed',
      V08EnforcementProductControlSurface.WindowsUnmanagedExactUrlNotClaimed,
      ['explicit unmanaged browser integration proof before exact URL evidence'],
      'Unmanaged exact URL evidence is not claimed from process/window or network metadata.',
      'Keep unmanaged exact URL state not claimed unless a browser integration produces typed exact evidence.'
    ),
    manualEntry(
      'v0-8-product-control-permission-loss-alerts',
      V08EnforcementProductControlSurface.WindowsPermissionLossAlerts,
      ['notification delivery provider', 'permission-loss detector', 'parent-visible delivery receipt'],
      'Permission-loss alerts remain manual-required until detector and delivery status are proved.',
      'Report local status only until notification delivery and acknowledgement proof exist.'
    ),
    manualEntry(
      'v0-8-product-control-tamper-uninstall-alerts',
      V08EnforcementProductControlSurface.WindowsTamperUninstallAlerts,
      ['explicit tamper/uninstall product design', 'service removal detector', 'non-stealth parent alert proof'],
      'Tamper/uninstall alerts remain manual-required and do not imply stealth or persistence hardening.',
      'Report manual-required until product/security design and non-stealth alert proof exist.'
    ),
  ],
});

function linkedEntry(
  entryId: string,
  surface: V08EnforcementProductControlSurface,
  linkedProofCommands: readonly string[],
  linkedProofArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08EnforcementProductControlSpineEntry {
  const expectation = productControlExpectationFor(surface);
  return productControlEntry({
    ...expectation,
    entryId,
    linkedProofCommands,
    linkedProofArtifacts,
    manualProofRequirements: [],
    claimBoundary,
    fallbackBehavior,
  });
}

function manualEntry(
  entryId: string,
  surface: V08EnforcementProductControlSurface,
  manualProofRequirements: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08EnforcementProductControlSpineEntry {
  const expectation = productControlExpectationFor(surface);
  return productControlEntry({
    ...expectation,
    entryId,
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    manualProofRequirements,
    claimBoundary,
    fallbackBehavior,
  });
}

function productControlExpectationFor(surface: V08EnforcementProductControlSurface): ProductControlSurfaceExpectation {
  const expectation = productControlSurfaceExpectations.find((candidate) => candidate.surface === surface);
  if (expectation === undefined) {
    throw new Error(`Missing V0.8 product-control surface expectation: ${surface}`);
  }
  return expectation;
}

function productControlEntry(entry: ProductControlEntryInput): V08EnforcementProductControlSpineEntry {
  return V08EnforcementProductControlSpineEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    entryId: entry.entryId,
    surface: entry.surface,
    surfaceKind: entry.surfaceKind,
    platform: entry.platform,
    capability: entry.capability,
    capabilityStatus: entry.capabilityStatus,
    productClaimState: entry.productClaimState,
    adapterExecutionState: entry.adapterExecutionState,
    devicePolicyState: entry.devicePolicyState,
    parentVisibleActions: [...entry.parentVisibleActions],
    linkedProofCommands: [...entry.linkedProofCommands],
    linkedProofArtifacts: [...entry.linkedProofArtifacts],
    manualProofRequirements: [...entry.manualProofRequirements],
    claimBoundary: entry.claimBoundary,
    fallbackBehavior: entry.fallbackBehavior,
    broadAppBlockingClaimed: false,
    networkDomainBlockingClaimed: false,
    managedExactUrlBlockingClaimed: false,
    unmanagedExactUrlClaimed: false,
    tamperResistanceClaimed: false,
    notificationDeliveryClaimed: false,
    lastCheckedAt: observedAt,
  });
}
