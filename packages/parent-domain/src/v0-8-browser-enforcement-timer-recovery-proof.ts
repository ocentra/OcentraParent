import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentControlCapabilityName,
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatus,
  ParentControlCapabilityStatusSchema,
  ParentControlPlatformSchema,
} from './capabilities';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyProofText = Schema.String.pipe(Schema.minLength(1));

export const V08BrowserEnforcementTimerRecoveryProofReadModelIdSchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofReadModelId')
);
export const V08BrowserEnforcementTimerRecoveryProofEntryIdSchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofEntryId')
);
export const V08BrowserEnforcementTimerRecoveryProofReferenceSchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofReference')
);
export const V08BrowserEnforcementTimerRecoveryProofRequirementSchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofRequirement')
);
export const V08BrowserEnforcementTimerRecoveryProofBoundarySchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofBoundary')
);
export const V08BrowserEnforcementTimerRecoveryProofFallbackSchema = NonEmptyProofText.pipe(
  Schema.brand('V08BrowserEnforcementTimerRecoveryProofFallback')
);

export const V08TimerRecoverySurfaceSchema = withParser(
  Schema.Literal(
    'timer-created-active',
    'timer-extended-active',
    'timer-expired-cleared',
    'timer-cancelled-cleared',
    'timer-restart-recovered-active',
    'timer-recovery-needed-missing-state',
    'timer-rollback-completed-cleared',
    'timer-rollback-unavailable-visible'
  )
);

export const V08TimerRecoveryLifecycleStateSchema = withParser(
  Schema.Literal(
    'created',
    'extended',
    'expired',
    'cancelled',
    'restart-recovered',
    'recovery-needed',
    'rollback-completed',
    'rollback-unavailable'
  )
);

export const V08TimerRecoveryEventKindSchema = withParser(
  Schema.Literal(
    'created',
    'extended',
    'expired',
    'cancelled',
    'restart-recovered',
    'rollback-completed',
    'recovery-needed',
    'unavailable'
  )
);

export const V08TimerRecoveryResultStatusSchema = withParser(
  Schema.Literal('no-op', 'expired', 'superseded', 'rolled-back', 'unavailable')
);

export const V08TimerRecoveryRollbackStateSchema = withParser(
  Schema.Literal('not-required', 'available', 'completed', 'unavailable')
);

export const V08TimerRecoveryStatePersistenceSchema = withParser(
  Schema.Literal('active-state-persisted', 'active-state-cleared', 'active-state-missing')
);

export const V08TimerRecoveryParentVisibleStateSchema = withParser(
  Schema.Literal('next-check-visible', 'terminal-state-visible', 'failure-state-visible', 'rollback-state-visible')
);

export const V08UnmanagedBrowserFallbackSurfaceSchema = withParser(
  Schema.Literal(
    'unmanaged-process-identity-required',
    'unmanaged-report-only',
    'unmanaged-warn-child',
    'unmanaged-parent-review',
    'unmanaged-terminate-process',
    'unmanaged-relaunch-managed-browser',
    'unmanaged-manual-required',
    'unmanaged-degraded',
    'unmanaged-unavailable'
  )
);

export const V08UnmanagedBrowserFallbackStateSchema = withParser(
  Schema.Literal(
    'process-identity-required',
    'report-only',
    'warn-child',
    'parent-review',
    'terminate-process',
    'relaunch-managed-browser',
    'manual-required',
    'degraded',
    'unavailable'
  )
);

export const V08UnmanagedBrowserFallbackExecutionStateSchema = withParser(
  Schema.Literal(
    'process-identity-rejected',
    'returns-report-only',
    'returns-degraded-noop',
    'returns-parent-review',
    'executes-real-service',
    'returns-manual-required',
    'returns-unavailable'
  )
);

export const V08UnmanagedBrowserProcessIdentityStateSchema = withParser(
  Schema.Literal('missing-process-identity-rejected', 'pid-name-required', 'pid-name-validated', 'not-applicable')
);

const V08TimerRecoveryProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08BrowserEnforcementTimerRecoveryProofEntryIdSchema,
  surface: V08TimerRecoverySurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  lifecycleState: V08TimerRecoveryLifecycleStateSchema,
  timerEventKind: V08TimerRecoveryEventKindSchema,
  resultStatus: V08TimerRecoveryResultStatusSchema,
  rollbackState: V08TimerRecoveryRollbackStateSchema,
  statePersistence: V08TimerRecoveryStatePersistenceSchema,
  parentVisibleStates: Schema.Array(V08TimerRecoveryParentVisibleStateSchema),
  nextCheckAtVisible: Schema.Boolean,
  failureStateVisible: Schema.Boolean,
  linkedProofCommands: Schema.Array(V08BrowserEnforcementTimerRecoveryProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08BrowserEnforcementTimerRecoveryProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08BrowserEnforcementTimerRecoveryProofRequirementSchema),
  claimBoundary: V08BrowserEnforcementTimerRecoveryProofBoundarySchema,
  fallbackBehavior: V08BrowserEnforcementTimerRecoveryProofFallbackSchema,
  lastCheckedAt: ParentTimestampSchema,
});

const V08UnmanagedBrowserFallbackProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: V08BrowserEnforcementTimerRecoveryProofEntryIdSchema,
  surface: V08UnmanagedBrowserFallbackSurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  fallbackState: V08UnmanagedBrowserFallbackStateSchema,
  adapterExecutionState: V08UnmanagedBrowserFallbackExecutionStateSchema,
  processIdentityRequired: Schema.Boolean,
  processIdentityState: V08UnmanagedBrowserProcessIdentityStateSchema,
  exactUrlClaimed: Schema.Boolean,
  activeTabClaimed: Schema.Boolean,
  titleClaimed: Schema.Boolean,
  contentClaimed: Schema.Boolean,
  notificationDeliveryClaimed: Schema.Boolean,
  broadBrowserBlockingClaimed: Schema.Boolean,
  linkedProofCommands: Schema.Array(V08BrowserEnforcementTimerRecoveryProofReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08BrowserEnforcementTimerRecoveryProofReferenceSchema),
  manualProofRequirements: Schema.Array(V08BrowserEnforcementTimerRecoveryProofRequirementSchema),
  claimBoundary: V08BrowserEnforcementTimerRecoveryProofBoundarySchema,
  fallbackBehavior: V08BrowserEnforcementTimerRecoveryProofFallbackSchema,
  lastCheckedAt: ParentTimestampSchema,
});

type V08TimerRecoveryProofEntryCandidate = Infer<typeof V08TimerRecoveryProofEntryBaseSchema>;
type V08UnmanagedBrowserFallbackProofEntryCandidate = Infer<typeof V08UnmanagedBrowserFallbackProofEntryBaseSchema>;

export const V08TimerRecoveryProofEntrySchema = withParser(
  V08TimerRecoveryProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        timerRecoveryEntryMatchesExpectation(entry) ||
        'Expected V0.8 timer recovery proof entries to preserve create, extend, expire, cancel, restart recovery, recovery-needed, and rollback visibility boundaries'
    )
  )
);

export const V08UnmanagedBrowserFallbackProofEntrySchema = withParser(
  V08UnmanagedBrowserFallbackProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        unmanagedFallbackEntryIsHonest(entry) ||
        'Expected V0.8 unmanaged browser fallback entries to keep process identity, report-only, warn, review, terminate, relaunch, manual, degraded, and unavailable states separate without exact content claim upgrades'
    )
  )
);

export const V08BrowserEnforcementTimerRecoveryProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08BrowserEnforcementTimerRecoveryProofReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08BrowserEnforcementTimerRecoveryProofReferenceSchema),
    timerEntries: Schema.Array(V08TimerRecoveryProofEntrySchema),
    unmanagedFallbackEntries: Schema.Array(V08UnmanagedBrowserFallbackProofEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        (new Set(readModel.timerEntries.map((entry) => entry.proofEntryId)).size === readModel.timerEntries.length &&
          new Set(readModel.unmanagedFallbackEntries.map((entry) => entry.proofEntryId)).size ===
            readModel.unmanagedFallbackEntries.length) ||
        'Expected V0.8 browser enforcement timer recovery proof entry ids to be unique per section'
    ),
    Schema.filter(
      (readModel) =>
        timerRecoveryStatesAreComplete(readModel.timerEntries) ||
        'Expected V0.8 timer recovery proof to cover create, extend, expire, cancel, restart recovery, recovery-needed, rollback-completed, and rollback-unavailable states'
    ),
    Schema.filter(
      (readModel) =>
        unmanagedFallbackStatesAreComplete(readModel.unmanagedFallbackEntries) ||
        'Expected V0.8 unmanaged browser fallback proof to cover process identity, report-only, warn, review, terminate, relaunch, manual-required, degraded, and unavailable states'
    )
  )
);

function timerRecoveryEntryMatchesExpectation(entry: V08TimerRecoveryProofEntryCandidate): boolean {
  const expectation = timerRecoveryExpectations.find((candidate) => candidate.surface === entry.surface);
  if (expectation === undefined) {
    return false;
  }

  return (
    timerRecoveryCoreFieldsMatch(entry, expectation) &&
    parentVisibleStatesMatch(entry.parentVisibleStates, expectation.parentVisibleStates) &&
    proofEvidenceMatches(entry, expectation.evidenceExpectation)
  );
}

function timerRecoveryCoreFieldsMatch(
  entry: V08TimerRecoveryProofEntryCandidate,
  expectation: TimerRecoveryExpectation
): boolean {
  return (
    entry.platform === 'windows' &&
    entry.capability === ParentControlCapabilityName.AppTimeLimit &&
    entry.capabilityStatus === expectation.capabilityStatus &&
    entry.lifecycleState === expectation.lifecycleState &&
    entry.timerEventKind === expectation.timerEventKind &&
    entry.resultStatus === expectation.resultStatus &&
    entry.rollbackState === expectation.rollbackState &&
    entry.statePersistence === expectation.statePersistence &&
    entry.nextCheckAtVisible === expectation.nextCheckAtVisible &&
    entry.failureStateVisible === expectation.failureStateVisible
  );
}

function unmanagedFallbackEntryIsHonest(entry: V08UnmanagedBrowserFallbackProofEntryCandidate): boolean {
  if (unmanagedFallbackEntryHasClaimUpgrade(entry)) {
    return false;
  }

  const expectation = unmanagedFallbackExpectations.find((candidate) => candidate.surface === entry.surface);
  if (expectation === undefined) {
    return false;
  }

  return (
    entry.platform === 'windows' &&
    entry.capability === ParentControlCapabilityName.UnmanagedBrowserDetection &&
    entry.capabilityStatus === expectation.capabilityStatus &&
    entry.fallbackState === expectation.fallbackState &&
    entry.adapterExecutionState === expectation.adapterExecutionState &&
    entry.processIdentityRequired === expectation.processIdentityRequired &&
    entry.processIdentityState === expectation.processIdentityState &&
    proofEvidenceMatches(entry, expectation.evidenceExpectation)
  );
}

function unmanagedFallbackEntryHasClaimUpgrade(entry: V08UnmanagedBrowserFallbackProofEntryCandidate): boolean {
  return [
    entry.exactUrlClaimed,
    entry.activeTabClaimed,
    entry.titleClaimed,
    entry.contentClaimed,
    entry.notificationDeliveryClaimed,
    entry.broadBrowserBlockingClaimed,
  ].some(Boolean);
}

function timerRecoveryStatesAreComplete(entries: readonly V08TimerRecoveryProofEntryCandidate[]): boolean {
  return timerRecoveryExpectations.every((expectation) =>
    entries.some((entry) => entry.surface === expectation.surface)
  );
}

function unmanagedFallbackStatesAreComplete(
  entries: readonly V08UnmanagedBrowserFallbackProofEntryCandidate[]
): boolean {
  return unmanagedFallbackExpectations.every((expectation) =>
    entries.some((entry) => entry.surface === expectation.surface)
  );
}

function parentVisibleStatesMatch(
  actual: readonly V08TimerRecoveryParentVisibleState[],
  expected: readonly V08TimerRecoveryParentVisibleState[]
): boolean {
  return actual.length === expected.length && expected.every((state) => actual.includes(state));
}

function proofEvidenceMatches(entry: ProofEvidenceFields, evidenceExpectation: ProofEvidenceExpectation): boolean {
  switch (evidenceExpectation) {
    case 'linked-proof':
      return (
        entry.linkedProofCommands.length > 0 &&
        entry.linkedProofArtifacts.length > 0 &&
        entry.manualProofRequirements.length === 0
      );
    case 'linked-with-manual-requirements':
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

type ProofEvidenceFields = {
  linkedProofCommands: readonly unknown[];
  linkedProofArtifacts: readonly unknown[];
  manualProofRequirements: readonly unknown[];
};

type ProofEvidenceExpectation = 'linked-proof' | 'linked-with-manual-requirements' | 'manual-proof';

type TimerRecoveryExpectation = {
  surface: V08TimerRecoverySurface;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  lifecycleState: V08TimerRecoveryLifecycleState;
  timerEventKind: V08TimerRecoveryEventKind;
  resultStatus: V08TimerRecoveryResultStatus;
  rollbackState: V08TimerRecoveryRollbackState;
  statePersistence: V08TimerRecoveryStatePersistence;
  parentVisibleStates: readonly V08TimerRecoveryParentVisibleState[];
  nextCheckAtVisible: boolean;
  failureStateVisible: boolean;
  evidenceExpectation: ProofEvidenceExpectation;
};

type UnmanagedFallbackExpectation = {
  surface: V08UnmanagedBrowserFallbackSurface;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  fallbackState: V08UnmanagedBrowserFallbackState;
  adapterExecutionState: V08UnmanagedBrowserFallbackExecutionState;
  processIdentityRequired: boolean;
  processIdentityState: V08UnmanagedBrowserProcessIdentityState;
  evidenceExpectation: ProofEvidenceExpectation;
};

const timerRecoveryExpectations: readonly TimerRecoveryExpectation[] = [
  {
    surface: 'timer-created-active',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    lifecycleState: 'created',
    timerEventKind: 'created',
    resultStatus: 'no-op',
    rollbackState: 'available',
    statePersistence: 'active-state-persisted',
    parentVisibleStates: ['next-check-visible'],
    nextCheckAtVisible: true,
    failureStateVisible: false,
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'timer-extended-active',
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    lifecycleState: 'extended',
    timerEventKind: 'extended',
    resultStatus: 'no-op',
    rollbackState: 'available',
    statePersistence: 'active-state-persisted',
    parentVisibleStates: ['next-check-visible'],
    nextCheckAtVisible: true,
    failureStateVisible: false,
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'timer-expired-cleared',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    lifecycleState: 'expired',
    timerEventKind: 'expired',
    resultStatus: 'expired',
    rollbackState: 'not-required',
    statePersistence: 'active-state-cleared',
    parentVisibleStates: ['terminal-state-visible'],
    nextCheckAtVisible: false,
    failureStateVisible: false,
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'timer-cancelled-cleared',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    lifecycleState: 'cancelled',
    timerEventKind: 'cancelled',
    resultStatus: 'superseded',
    rollbackState: 'not-required',
    statePersistence: 'active-state-cleared',
    parentVisibleStates: ['terminal-state-visible'],
    nextCheckAtVisible: false,
    failureStateVisible: false,
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'timer-restart-recovered-active',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    lifecycleState: 'restart-recovered',
    timerEventKind: 'restart-recovered',
    resultStatus: 'no-op',
    rollbackState: 'not-required',
    statePersistence: 'active-state-persisted',
    parentVisibleStates: ['next-check-visible'],
    nextCheckAtVisible: true,
    failureStateVisible: false,
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'timer-recovery-needed-missing-state',
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    lifecycleState: 'recovery-needed',
    timerEventKind: 'recovery-needed',
    resultStatus: 'unavailable',
    rollbackState: 'unavailable',
    statePersistence: 'active-state-missing',
    parentVisibleStates: ['failure-state-visible'],
    nextCheckAtVisible: false,
    failureStateVisible: true,
    evidenceExpectation: 'linked-with-manual-requirements',
  },
  {
    surface: 'timer-rollback-completed-cleared',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    lifecycleState: 'rollback-completed',
    timerEventKind: 'rollback-completed',
    resultStatus: 'rolled-back',
    rollbackState: 'completed',
    statePersistence: 'active-state-cleared',
    parentVisibleStates: ['terminal-state-visible', 'rollback-state-visible'],
    nextCheckAtVisible: false,
    failureStateVisible: false,
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'timer-rollback-unavailable-visible',
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    lifecycleState: 'rollback-unavailable',
    timerEventKind: 'unavailable',
    resultStatus: 'unavailable',
    rollbackState: 'unavailable',
    statePersistence: 'active-state-cleared',
    parentVisibleStates: ['failure-state-visible', 'rollback-state-visible'],
    nextCheckAtVisible: false,
    failureStateVisible: true,
    evidenceExpectation: 'linked-with-manual-requirements',
  },
];

const unmanagedFallbackExpectations: readonly UnmanagedFallbackExpectation[] = [
  {
    surface: 'unmanaged-process-identity-required',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    fallbackState: 'process-identity-required',
    adapterExecutionState: 'process-identity-rejected',
    processIdentityRequired: true,
    processIdentityState: 'missing-process-identity-rejected',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'unmanaged-report-only',
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    fallbackState: 'report-only',
    adapterExecutionState: 'returns-report-only',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'unmanaged-warn-child',
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    fallbackState: 'warn-child',
    adapterExecutionState: 'returns-degraded-noop',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'linked-with-manual-requirements',
  },
  {
    surface: 'unmanaged-parent-review',
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    fallbackState: 'parent-review',
    adapterExecutionState: 'returns-parent-review',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'unmanaged-terminate-process',
    capabilityStatus: ParentControlCapabilityStatus.Implemented,
    fallbackState: 'terminate-process',
    adapterExecutionState: 'executes-real-service',
    processIdentityRequired: true,
    processIdentityState: 'pid-name-validated',
    evidenceExpectation: 'linked-proof',
  },
  {
    surface: 'unmanaged-relaunch-managed-browser',
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    fallbackState: 'relaunch-managed-browser',
    adapterExecutionState: 'returns-manual-required',
    processIdentityRequired: true,
    processIdentityState: 'pid-name-required',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'unmanaged-manual-required',
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    fallbackState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'manual-proof',
  },
  {
    surface: 'unmanaged-degraded',
    capabilityStatus: ParentControlCapabilityStatus.Supported,
    fallbackState: 'degraded',
    adapterExecutionState: 'returns-degraded-noop',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'linked-with-manual-requirements',
  },
  {
    surface: 'unmanaged-unavailable',
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    fallbackState: 'unavailable',
    adapterExecutionState: 'returns-unavailable',
    processIdentityRequired: false,
    processIdentityState: 'not-applicable',
    evidenceExpectation: 'manual-proof',
  },
];

export type V08BrowserEnforcementTimerRecoveryProofReadModelId =
  typeof V08BrowserEnforcementTimerRecoveryProofReadModelIdSchema.Type;
export type V08BrowserEnforcementTimerRecoveryProofEntryId =
  typeof V08BrowserEnforcementTimerRecoveryProofEntryIdSchema.Type;
export type V08BrowserEnforcementTimerRecoveryProofReference =
  typeof V08BrowserEnforcementTimerRecoveryProofReferenceSchema.Type;
export type V08BrowserEnforcementTimerRecoveryProofRequirement =
  typeof V08BrowserEnforcementTimerRecoveryProofRequirementSchema.Type;
export type V08BrowserEnforcementTimerRecoveryProofBoundary =
  typeof V08BrowserEnforcementTimerRecoveryProofBoundarySchema.Type;
export type V08BrowserEnforcementTimerRecoveryProofFallback =
  typeof V08BrowserEnforcementTimerRecoveryProofFallbackSchema.Type;
export type V08TimerRecoverySurface = Infer<typeof V08TimerRecoverySurfaceSchema>;
export type V08TimerRecoveryLifecycleState = Infer<typeof V08TimerRecoveryLifecycleStateSchema>;
export type V08TimerRecoveryEventKind = Infer<typeof V08TimerRecoveryEventKindSchema>;
export type V08TimerRecoveryResultStatus = Infer<typeof V08TimerRecoveryResultStatusSchema>;
export type V08TimerRecoveryRollbackState = Infer<typeof V08TimerRecoveryRollbackStateSchema>;
export type V08TimerRecoveryStatePersistence = Infer<typeof V08TimerRecoveryStatePersistenceSchema>;
export type V08TimerRecoveryParentVisibleState = Infer<typeof V08TimerRecoveryParentVisibleStateSchema>;
export type V08UnmanagedBrowserFallbackSurface = Infer<typeof V08UnmanagedBrowserFallbackSurfaceSchema>;
export type V08UnmanagedBrowserFallbackState = Infer<typeof V08UnmanagedBrowserFallbackStateSchema>;
export type V08UnmanagedBrowserFallbackExecutionState = Infer<typeof V08UnmanagedBrowserFallbackExecutionStateSchema>;
export type V08UnmanagedBrowserProcessIdentityState = Infer<typeof V08UnmanagedBrowserProcessIdentityStateSchema>;
export type V08TimerRecoveryProofEntry = Infer<typeof V08TimerRecoveryProofEntrySchema>;
export type V08UnmanagedBrowserFallbackProofEntry = Infer<typeof V08UnmanagedBrowserFallbackProofEntrySchema>;
export type V08BrowserEnforcementTimerRecoveryProofReadModel = Infer<
  typeof V08BrowserEnforcementTimerRecoveryProofReadModelSchema
>;

export const V08TimerRecoverySurface = {
  CreatedActive: V08TimerRecoverySurfaceSchema.parse('timer-created-active'),
  ExtendedActive: V08TimerRecoverySurfaceSchema.parse('timer-extended-active'),
  ExpiredCleared: V08TimerRecoverySurfaceSchema.parse('timer-expired-cleared'),
  CancelledCleared: V08TimerRecoverySurfaceSchema.parse('timer-cancelled-cleared'),
  RestartRecoveredActive: V08TimerRecoverySurfaceSchema.parse('timer-restart-recovered-active'),
  RecoveryNeededMissingState: V08TimerRecoverySurfaceSchema.parse('timer-recovery-needed-missing-state'),
  RollbackCompletedCleared: V08TimerRecoverySurfaceSchema.parse('timer-rollback-completed-cleared'),
  RollbackUnavailableVisible: V08TimerRecoverySurfaceSchema.parse('timer-rollback-unavailable-visible'),
} as const;

export const V08UnmanagedBrowserFallbackSurface = {
  ProcessIdentityRequired: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-process-identity-required'),
  ReportOnly: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-report-only'),
  WarnChild: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-warn-child'),
  ParentReview: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-parent-review'),
  TerminateProcess: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-terminate-process'),
  RelaunchManagedBrowser: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-relaunch-managed-browser'),
  ManualRequired: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-manual-required'),
  Degraded: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-degraded'),
  Unavailable: V08UnmanagedBrowserFallbackSurfaceSchema.parse('unmanaged-unavailable'),
} as const;

export const decodeV08TimerRecoveryProofEntry = Schema.decodeUnknownSync(V08TimerRecoveryProofEntrySchema);
export const decodeV08UnmanagedBrowserFallbackProofEntry = Schema.decodeUnknownSync(
  V08UnmanagedBrowserFallbackProofEntrySchema
);
export const decodeV08BrowserEnforcementTimerRecoveryProofReadModel = Schema.decodeUnknownSync(
  V08BrowserEnforcementTimerRecoveryProofReadModelSchema
);
