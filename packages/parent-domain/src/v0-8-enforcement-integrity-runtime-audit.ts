import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './reference-primitives';

const NonEmptyIntegrityAuditText = Schema.String.pipe(Schema.minLength(1));

export const V08EnforcementIntegrityRuntimeAuditReadModelIdSchema = NonEmptyIntegrityAuditText.pipe(
  Schema.brand('V08EnforcementIntegrityRuntimeAuditReadModelId')
);
export const V08EnforcementIntegrityRuntimeAuditEntryIdSchema = NonEmptyIntegrityAuditText.pipe(
  Schema.brand('V08EnforcementIntegrityRuntimeAuditEntryId')
);
export const V08EnforcementIntegrityRuntimeAuditReferenceSchema = NonEmptyIntegrityAuditText.pipe(
  Schema.brand('V08EnforcementIntegrityRuntimeAuditReference')
);
export const V08EnforcementIntegrityRuntimeAuditRequirementSchema = NonEmptyIntegrityAuditText.pipe(
  Schema.brand('V08EnforcementIntegrityRuntimeAuditRequirement')
);
export const V08EnforcementIntegrityRuntimeAuditBoundarySchema = NonEmptyIntegrityAuditText.pipe(
  Schema.brand('V08EnforcementIntegrityRuntimeAuditBoundary')
);

export const V08EnforcementIntegrityRuntimeAuditSurfaceSchema = withParser(
  Schema.Literal(
    'app-game-time-limit',
    'managed-browser-session',
    'unmanaged-browser-process-fallback',
    'network-domain-observe-only',
    'host-network-domain-filter',
    'notification-delivery',
    'integrity-heartbeat',
    'tamper-uninstall-signal',
    'mobile-child-control'
  )
);

export const V08EnforcementIntegrityRuntimeAuditResultSchema = withParser(
  Schema.Literal(
    'succeeded',
    'failed',
    'unavailable',
    'expired',
    'rolled-back',
    'superseded',
    'no-op',
    'manual-required',
    'unsupported',
    'observe-only'
  )
);

export const V08EnforcementIntegrityRuntimeAuditExecutionSchema = withParser(
  Schema.Literal(
    'executed-supported-boundary',
    'dry-run-no-adapter-execution',
    'rejected-before-adapter',
    'manual-required-no-execution',
    'observe-only-no-execution',
    'unavailable-no-execution',
    'unsupported-no-execution',
    'recovery-needed-no-execution'
  )
);

export const V08EnforcementIntegrityRuntimeAuditIntentStateSchema = withParser(
  Schema.Literal(
    'validated',
    'observe-only',
    'rejected-invalid',
    'rejected-stale',
    'rejected-wrong-device',
    'rejected-unsupported'
  )
);

export const V08EnforcementIntegrityRuntimeAuditTimerStateSchema = withParser(
  Schema.Literal(
    'active-timer-backed',
    'expired-backed',
    'cancelled-backed',
    'rollback-backed',
    'recovery-needed',
    'not-applicable',
    'unavailable'
  )
);

export const V08EnforcementIntegrityRuntimeAuditRollbackStateSchema = withParser(
  Schema.Literal(
    'not-needed',
    'rollback-token-backed',
    'rollback-completed',
    'rollback-required',
    'manual-required',
    'unavailable'
  )
);

export const V08EnforcementIntegrityRuntimeAuditChildStateSchema = withParser(
  Schema.Literal(
    'status-ref-backed',
    'reason-ref-backed',
    'approval-intent-backed',
    'manual-required',
    'unsupported',
    'not-claimed'
  )
);

export const V08EnforcementIntegrityRuntimeAuditIntegrityStateSchema = withParser(
  Schema.Literal(
    'running',
    'permission-missing',
    'adapter-unavailable',
    'stale-heartbeat',
    'service-stopped',
    'uninstall-detection-manual-required',
    'tamper-signal-manual-required',
    'anti-tamper-not-claimed',
    'not-applicable'
  )
);

export const V08EnforcementIntegrityRuntimeAuditAuditStateSchema = withParser(
  Schema.Literal('audit-backed', 'audit-required', 'manual-required', 'unavailable', 'not-claimed')
);

const V08EnforcementIntegrityRuntimeAuditEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  auditEntryId: V08EnforcementIntegrityRuntimeAuditEntryIdSchema,
  surface: V08EnforcementIntegrityRuntimeAuditSurfaceSchema,
  platform: ParentPlatformSchema,
  result: V08EnforcementIntegrityRuntimeAuditResultSchema,
  execution: V08EnforcementIntegrityRuntimeAuditExecutionSchema,
  intentState: V08EnforcementIntegrityRuntimeAuditIntentStateSchema,
  timerState: V08EnforcementIntegrityRuntimeAuditTimerStateSchema,
  rollbackState: V08EnforcementIntegrityRuntimeAuditRollbackStateSchema,
  childState: V08EnforcementIntegrityRuntimeAuditChildStateSchema,
  integrityState: V08EnforcementIntegrityRuntimeAuditIntegrityStateSchema,
  auditState: V08EnforcementIntegrityRuntimeAuditAuditStateSchema,
  policyDecisionRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  evidenceRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  adapterOutcomeRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  auditRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  rollbackRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  timerRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  childStatusRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  integrityRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  parentIntentRefs: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
  manualProofRequirements: Schema.Array(V08EnforcementIntegrityRuntimeAuditRequirementSchema),
  boundary: V08EnforcementIntegrityRuntimeAuditBoundarySchema,
  broadInstalledAppBlockingClaimed: Schema.Boolean,
  hostNetworkDomainBlockingClaimed: Schema.Boolean,
  exactActiveTabEnforcementClaimed: Schema.Boolean,
  notificationDeliveryClaimed: Schema.Boolean,
  tamperHardeningClaimed: Schema.Boolean,
  mobilePrivilegeClaimed: Schema.Boolean,
  stealthPersistenceClaimed: Schema.Boolean,
  privilegeEscalationClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08EnforcementIntegrityRuntimeAuditEntryCandidate = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditEntryBaseSchema
>;

export const V08EnforcementIntegrityRuntimeAuditEntrySchema = withParser(
  V08EnforcementIntegrityRuntimeAuditEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        integrityAuditEntryIsHonest(entry) ||
        'Expected V0.8 enforcement integrity runtime audit entries to preserve supported execution, no-execution, rollback, child-status, permission-loss, heartbeat, tamper, and unsupported states without broad blocking or anti-tamper claim upgrades'
    )
  )
);

export const V08EnforcementIntegrityRuntimeAuditReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08EnforcementIntegrityRuntimeAuditReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08EnforcementIntegrityRuntimeAuditReferenceSchema),
    entries: Schema.Array(V08EnforcementIntegrityRuntimeAuditEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.auditEntryId)).size === readModel.entries.length ||
        'Expected V0.8 enforcement integrity runtime audit entry ids to be unique'
    )
  )
);

function integrityAuditEntryIsHonest(entry: V08EnforcementIntegrityRuntimeAuditEntryCandidate): boolean {
  if (integrityAuditEntryHasClaimUpgrade(entry)) {
    return false;
  }

  if (!integrityAuditExecutionMatchesResult(entry)) {
    return false;
  }

  if (!integrityAuditStateHasRequiredRefs(entry)) {
    return false;
  }

  return integrityAuditNonClaimStateIsExplicit(entry);
}

function integrityAuditEntryHasClaimUpgrade(entry: V08EnforcementIntegrityRuntimeAuditEntryCandidate): boolean {
  return [
    entry.broadInstalledAppBlockingClaimed,
    entry.hostNetworkDomainBlockingClaimed,
    entry.exactActiveTabEnforcementClaimed,
    entry.notificationDeliveryClaimed,
    entry.tamperHardeningClaimed,
    entry.mobilePrivilegeClaimed,
    entry.stealthPersistenceClaimed,
    entry.privilegeEscalationClaimed,
  ].some(Boolean);
}

function integrityAuditExecutionMatchesResult(entry: V08EnforcementIntegrityRuntimeAuditEntryCandidate): boolean {
  switch (entry.result) {
    case 'succeeded':
    case 'expired':
    case 'rolled-back':
    case 'superseded':
      return entry.execution === 'executed-supported-boundary';
    case 'no-op':
      return entry.execution === 'dry-run-no-adapter-execution' || entry.execution === 'rejected-before-adapter';
    case 'failed':
      return entry.execution === 'rejected-before-adapter';
    case 'observe-only':
      return entry.execution === 'observe-only-no-execution';
    case 'manual-required':
      return entry.execution === 'manual-required-no-execution';
    case 'unavailable':
      return entry.execution === 'unavailable-no-execution' || entry.execution === 'recovery-needed-no-execution';
    case 'unsupported':
      return entry.execution === 'unsupported-no-execution';
  }
}

function integrityAuditStateHasRequiredRefs(entry: V08EnforcementIntegrityRuntimeAuditEntryCandidate): boolean {
  if (entry.auditState === 'audit-backed' && entry.auditRefs.length === 0) {
    return false;
  }

  if (entry.intentState === 'validated' && entry.policyDecisionRefs.length === 0) {
    return false;
  }

  if (entry.execution === 'executed-supported-boundary' && entry.adapterOutcomeRefs.length === 0) {
    return false;
  }

  if (entry.timerState !== 'not-applicable' && entry.timerState !== 'unavailable' && entry.timerRefs.length === 0) {
    return false;
  }

  if (entry.childState.endsWith('-backed') && entry.childStatusRefs.length === 0) {
    return false;
  }

  return true;
}

function integrityAuditNonClaimStateIsExplicit(entry: V08EnforcementIntegrityRuntimeAuditEntryCandidate): boolean {
  if (entry.result === 'manual-required' || entry.result === 'unavailable' || entry.result === 'unsupported') {
    return entry.manualProofRequirements.length > 0;
  }

  if (entry.integrityState.includes('manual-required') || entry.integrityState === 'anti-tamper-not-claimed') {
    return entry.manualProofRequirements.length > 0 && entry.auditState !== 'audit-backed';
  }

  return true;
}

export type V08EnforcementIntegrityRuntimeAuditReadModelId =
  typeof V08EnforcementIntegrityRuntimeAuditReadModelIdSchema.Type;
export type V08EnforcementIntegrityRuntimeAuditEntryId =
  typeof V08EnforcementIntegrityRuntimeAuditEntryIdSchema.Type;
export type V08EnforcementIntegrityRuntimeAuditReference =
  typeof V08EnforcementIntegrityRuntimeAuditReferenceSchema.Type;
export type V08EnforcementIntegrityRuntimeAuditRequirement =
  typeof V08EnforcementIntegrityRuntimeAuditRequirementSchema.Type;
export type V08EnforcementIntegrityRuntimeAuditBoundary =
  typeof V08EnforcementIntegrityRuntimeAuditBoundarySchema.Type;
export type V08EnforcementIntegrityRuntimeAuditSurface = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditSurfaceSchema
>;
export type V08EnforcementIntegrityRuntimeAuditResult = Infer<typeof V08EnforcementIntegrityRuntimeAuditResultSchema>;
export type V08EnforcementIntegrityRuntimeAuditExecution = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditExecutionSchema
>;
export type V08EnforcementIntegrityRuntimeAuditIntentState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditIntentStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditTimerState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditTimerStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditRollbackState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditRollbackStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditChildState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditChildStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditIntegrityState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditIntegrityStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditAuditState = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditAuditStateSchema
>;
export type V08EnforcementIntegrityRuntimeAuditEntry = Infer<typeof V08EnforcementIntegrityRuntimeAuditEntrySchema>;
export type V08EnforcementIntegrityRuntimeAuditReadModel = Infer<
  typeof V08EnforcementIntegrityRuntimeAuditReadModelSchema
>;

type V08EnforcementIntegrityRuntimeAuditEntryInput = {
  auditEntryId: string;
  surface: V08EnforcementIntegrityRuntimeAuditSurface;
  platform: typeof ParentPlatformSchema.Type;
  result: V08EnforcementIntegrityRuntimeAuditResult;
  execution: V08EnforcementIntegrityRuntimeAuditExecution;
  intentState: V08EnforcementIntegrityRuntimeAuditIntentState;
  timerState: V08EnforcementIntegrityRuntimeAuditTimerState;
  rollbackState: V08EnforcementIntegrityRuntimeAuditRollbackState;
  childState: V08EnforcementIntegrityRuntimeAuditChildState;
  integrityState: V08EnforcementIntegrityRuntimeAuditIntegrityState;
  auditState: V08EnforcementIntegrityRuntimeAuditAuditState;
  policyDecisionRefs: readonly string[];
  evidenceRefs: readonly string[];
  adapterOutcomeRefs: readonly string[];
  auditRefs: readonly string[];
  rollbackRefs: readonly string[];
  timerRefs: readonly string[];
  childStatusRefs: readonly string[];
  integrityRefs: readonly string[];
  parentIntentRefs: readonly string[];
  manualProofRequirements: readonly string[];
  boundary: string;
};

const generatedAt = '2026-06-02T11:25:05.000Z';

const SourceReadModelIds = {
  SupportedAdapterRuntimeProof: 'v0-8-supported-adapter-runtime-proof',
  PolicyDispatchProof: 'v0-8-enforcement-policy-dispatch-proof',
  ProductControlSpine: 'v0-8-enforcement-product-control-spine',
  EnforcementAuditJournal: 'enforcement-audit-journal',
  TimerRecoveryState: 'enforcement-timer-recovery-state',
} as const;

export const V08EnforcementIntegrityRuntimeAuditReadModel =
  V08EnforcementIntegrityRuntimeAuditReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'v0-8-enforcement-integrity-runtime-audit',
    generatedAt,
    sourceReadModelIds: Object.values(SourceReadModelIds),
    entries: [
      supportedEntry({
        auditEntryId: 'app-time-limit-action-succeeded',
        result: 'succeeded',
        timerState: 'active-timer-backed',
        rollbackState: 'rollback-token-backed',
        childState: 'reason-ref-backed',
        boundary:
          'Owned-process app/game time-limit actions can execute only with policy, evidence, timer, rollback, child-reason, and audit references.',
      }),
      supportedEntry({
        auditEntryId: 'app-time-limit-action-expired',
        result: 'expired',
        timerState: 'expired-backed',
        rollbackState: 'not-needed',
        childState: 'status-ref-backed',
        boundary:
          'Expiry is audit-backed by timer state and child-facing status refs; it does not imply broad app blocking.',
      }),
      supportedEntry({
        auditEntryId: 'app-time-limit-action-rolled-back',
        result: 'rolled-back',
        timerState: 'rollback-backed',
        rollbackState: 'rollback-completed',
        childState: 'status-ref-backed',
        boundary:
          'Rollback is a typed supported-boundary state with rollback refs and audit refs, not an implicit unblock claim for unproved adapters.',
      }),
      supportedEntry({
        auditEntryId: 'parent-override-superseded-action',
        result: 'superseded',
        timerState: 'cancelled-backed',
        rollbackState: 'rollback-token-backed',
        childState: 'approval-intent-backed',
        parentIntentRefs: ['parent-override-intent-ref'],
        boundary:
          'Parent override supersedes a validated action through auditable intent refs owned by the agent runtime.',
      }),
      noExecutionEntry({
        auditEntryId: 'dry-run-preview-no-op',
        result: 'no-op',
        execution: 'dry-run-no-adapter-execution',
        intentState: 'observe-only',
        timerState: 'not-applicable',
        rollbackState: 'not-needed',
        childState: 'not-claimed',
        boundary: 'Dry-run and observe previews are audit-visible no-ops and must not execute adapters.',
      }),
      noExecutionEntry({
        auditEntryId: 'stale-policy-decision-rejected',
        result: 'failed',
        execution: 'rejected-before-adapter',
        intentState: 'rejected-stale',
        timerState: 'not-applicable',
        rollbackState: 'not-needed',
        childState: 'reason-ref-backed',
        childStatusRefs: ['child-status-stale-policy-ref'],
        boundary:
          'Stale policy decisions reject before adapter execution and keep a child-facing reason/status ref.',
      }),
      noExecutionEntry({
        auditEntryId: 'wrong-device-intent-rejected',
        result: 'failed',
        execution: 'rejected-before-adapter',
        intentState: 'rejected-wrong-device',
        timerState: 'not-applicable',
        rollbackState: 'not-needed',
        childState: 'reason-ref-backed',
        childStatusRefs: ['child-status-wrong-device-ref'],
        boundary: 'Wrong-device intents reject before adapter execution and remain auditable.',
      }),
      noExecutionEntry({
        auditEntryId: 'network-domain-observe-only',
        surface: 'network-domain-observe-only',
        result: 'observe-only',
        execution: 'observe-only-no-execution',
        intentState: 'observe-only',
        timerState: 'not-applicable',
        rollbackState: 'not-needed',
        childState: 'not-claimed',
        evidenceRefs: ['network-flow-summary-ref'],
        boundary:
          'Network/domain runtime state is observe-only over stored flow evidence; host DNS/filter enforcement is not executed.',
      }),
      manualEntry({
        auditEntryId: 'host-network-domain-filter-manual-required',
        surface: 'host-network-domain-filter',
        manualProofRequirements: ['host DNS or filter apply artifact', 'host filter rollback artifact'],
        boundary:
          'Host network/domain filtering remains manual-required until apply, rollback, and audit artifacts exist.',
      }),
      unavailableEntry({
        auditEntryId: 'permission-loss-unavailable',
        integrityState: 'permission-missing',
        manualProofRequirements: ['permission restoration artifact', 'operator-visible permission state'],
        boundary: 'Permission loss is explicit unavailable state and must not be reported as enforcement success.',
      }),
      unavailableEntry({
        auditEntryId: 'adapter-unavailable-recovery-needed',
        execution: 'recovery-needed-no-execution',
        timerState: 'recovery-needed',
        integrityState: 'adapter-unavailable',
        manualProofRequirements: ['adapter recovery artifact', 'service restart recovery proof'],
        boundary:
          'Adapter recovery is explicit recovery-needed/unavailable state where persisted restart recovery is not proved.',
      }),
      unavailableEntry({
        auditEntryId: 'stale-integrity-heartbeat',
        surface: 'integrity-heartbeat',
        integrityState: 'stale-heartbeat',
        manualProofRequirements: ['fresh heartbeat proof', 'parent-visible stale agent alert'],
        boundary: 'Stale heartbeat is parent-visible degraded integrity state, not anti-tamper hardening.',
      }),
      unsupportedEntry({
        auditEntryId: 'mobile-child-control-unsupported',
        surface: 'mobile-child-control',
        platform: 'ios',
        manualProofRequirements: ['Family Controls entitlement artifact', 'DeviceActivity proof artifact'],
        boundary: 'Mobile child control remains unsupported without platform entitlement and device proof.',
      }),
      manualEntry({
        auditEntryId: 'tamper-uninstall-detection-manual-required',
        surface: 'tamper-uninstall-signal',
        integrityState: 'tamper-signal-manual-required',
        auditState: 'manual-required',
        manualProofRequirements: [
          'service-manager stop proof',
          'uninstall detection artifact',
          'security review before hardening',
        ],
        boundary:
          'Tamper/uninstall is represented as detectable/manual-required state only; no stealth, persistence, or anti-tamper hardening is claimed.',
      }),
    ],
  });

function supportedEntry(
  input: Pick<
    V08EnforcementIntegrityRuntimeAuditEntryInput,
    'auditEntryId' | 'result' | 'timerState' | 'rollbackState' | 'childState' | 'boundary'
  > &
    Partial<Pick<V08EnforcementIntegrityRuntimeAuditEntryInput, 'parentIntentRefs'>>
): V08EnforcementIntegrityRuntimeAuditEntry {
  return entry({
    auditEntryId: input.auditEntryId,
    surface: 'app-game-time-limit',
    platform: 'windows',
    result: input.result,
    execution: 'executed-supported-boundary',
    intentState: 'validated',
    timerState: input.timerState,
    rollbackState: input.rollbackState,
    childState: input.childState,
    integrityState: 'running',
    auditState: 'audit-backed',
    policyDecisionRefs: ['policy-decision-ref'],
    evidenceRefs: ['app-session-evidence-ref', 'owned-process-identity-ref'],
    adapterOutcomeRefs: ['adapter-outcome-ref'],
    auditRefs: ['enforcement-audit-ref'],
    rollbackRefs: ['rollback-token-ref'],
    timerRefs: ['timer-state-ref'],
    childStatusRefs: ['child-status-ref'],
    integrityRefs: ['integrity-heartbeat-ref'],
    parentIntentRefs: input.parentIntentRefs ?? [],
    manualProofRequirements: [],
    boundary: input.boundary,
  });
}

function noExecutionEntry(
  input: Pick<
    V08EnforcementIntegrityRuntimeAuditEntryInput,
    | 'auditEntryId'
    | 'result'
    | 'execution'
    | 'intentState'
    | 'timerState'
    | 'rollbackState'
    | 'childState'
    | 'boundary'
  > &
    Partial<Pick<V08EnforcementIntegrityRuntimeAuditEntryInput, 'surface' | 'evidenceRefs' | 'childStatusRefs'>>
): V08EnforcementIntegrityRuntimeAuditEntry {
  return entry({
    auditEntryId: input.auditEntryId,
    surface: input.surface ?? 'app-game-time-limit',
    platform: 'windows',
    result: input.result,
    execution: input.execution,
    intentState: input.intentState,
    timerState: input.timerState,
    rollbackState: input.rollbackState,
    childState: input.childState,
    integrityState: 'running',
    auditState: 'audit-backed',
    policyDecisionRefs: [],
    evidenceRefs: input.evidenceRefs ?? ['policy-preview-ref'],
    adapterOutcomeRefs: [],
    auditRefs: ['enforcement-audit-ref'],
    rollbackRefs: [],
    timerRefs: [],
    childStatusRefs: input.childStatusRefs ?? [],
    integrityRefs: ['integrity-heartbeat-ref'],
    parentIntentRefs: [],
    manualProofRequirements: [],
    boundary: input.boundary,
  });
}

function manualEntry(
  input: Pick<V08EnforcementIntegrityRuntimeAuditEntryInput, 'auditEntryId' | 'surface' | 'manualProofRequirements' | 'boundary'> &
    Partial<Pick<V08EnforcementIntegrityRuntimeAuditEntryInput, 'integrityState' | 'auditState'>>
): V08EnforcementIntegrityRuntimeAuditEntry {
  return entry({
    auditEntryId: input.auditEntryId,
    surface: input.surface,
    platform: 'windows',
    result: 'manual-required',
    execution: 'manual-required-no-execution',
    intentState: 'rejected-unsupported',
    timerState: 'unavailable',
    rollbackState: 'manual-required',
    childState: 'manual-required',
    integrityState: input.integrityState ?? 'not-applicable',
    auditState: input.auditState ?? 'manual-required',
    policyDecisionRefs: [],
    evidenceRefs: [],
    adapterOutcomeRefs: [],
    auditRefs: [],
    rollbackRefs: [],
    timerRefs: [],
    childStatusRefs: [],
    integrityRefs: [],
    parentIntentRefs: [],
    manualProofRequirements: input.manualProofRequirements,
    boundary: input.boundary,
  });
}

function unavailableEntry(
  input: Pick<
    V08EnforcementIntegrityRuntimeAuditEntryInput,
    'auditEntryId' | 'integrityState' | 'manualProofRequirements' | 'boundary'
  > &
    Partial<Pick<V08EnforcementIntegrityRuntimeAuditEntryInput, 'surface' | 'execution' | 'timerState'>>
): V08EnforcementIntegrityRuntimeAuditEntry {
  return entry({
    auditEntryId: input.auditEntryId,
    surface: input.surface ?? 'integrity-heartbeat',
    platform: 'windows',
    result: 'unavailable',
    execution: input.execution ?? 'unavailable-no-execution',
    intentState: 'validated',
    timerState: input.timerState ?? 'unavailable',
    rollbackState: 'unavailable',
    childState: 'reason-ref-backed',
    integrityState: input.integrityState,
    auditState: 'audit-backed',
    policyDecisionRefs: ['policy-decision-ref'],
    evidenceRefs: [],
    adapterOutcomeRefs: [],
    auditRefs: ['enforcement-audit-ref'],
    rollbackRefs: [],
    timerRefs: input.timerState === 'recovery-needed' ? ['timer-recovery-needed-ref'] : [],
    childStatusRefs: ['child-status-unavailable-ref'],
    integrityRefs: ['integrity-state-ref'],
    parentIntentRefs: [],
    manualProofRequirements: input.manualProofRequirements,
    boundary: input.boundary,
  });
}

function unsupportedEntry(
  input: Pick<
    V08EnforcementIntegrityRuntimeAuditEntryInput,
    'auditEntryId' | 'surface' | 'platform' | 'manualProofRequirements' | 'boundary'
  >
): V08EnforcementIntegrityRuntimeAuditEntry {
  return entry({
    auditEntryId: input.auditEntryId,
    surface: input.surface,
    platform: input.platform,
    result: 'unsupported',
    execution: 'unsupported-no-execution',
    intentState: 'rejected-unsupported',
    timerState: 'unavailable',
    rollbackState: 'unavailable',
    childState: 'unsupported',
    integrityState: 'not-applicable',
    auditState: 'unavailable',
    policyDecisionRefs: [],
    evidenceRefs: [],
    adapterOutcomeRefs: [],
    auditRefs: [],
    rollbackRefs: [],
    timerRefs: [],
    childStatusRefs: [],
    integrityRefs: [],
    parentIntentRefs: [],
    manualProofRequirements: input.manualProofRequirements,
    boundary: input.boundary,
  });
}

function entry(input: V08EnforcementIntegrityRuntimeAuditEntryInput): V08EnforcementIntegrityRuntimeAuditEntry {
  return V08EnforcementIntegrityRuntimeAuditEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    broadInstalledAppBlockingClaimed: false,
    hostNetworkDomainBlockingClaimed: false,
    exactActiveTabEnforcementClaimed: false,
    notificationDeliveryClaimed: false,
    tamperHardeningClaimed: false,
    mobilePrivilegeClaimed: false,
    stealthPersistenceClaimed: false,
    privilegeEscalationClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeV08EnforcementIntegrityRuntimeAuditEntry = Schema.decodeUnknownSync(
  V08EnforcementIntegrityRuntimeAuditEntrySchema
);
export const decodeV08EnforcementIntegrityRuntimeAuditReadModel = Schema.decodeUnknownSync(
  V08EnforcementIntegrityRuntimeAuditReadModelSchema
);
