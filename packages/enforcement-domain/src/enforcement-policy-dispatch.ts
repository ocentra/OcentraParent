import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { EnforcementAdapterKindSchema, EnforcementCapabilityStateSchema, EnforcementModeSchema } from './enforcement';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  type PolicyAction,
  PolicyActionSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyTargetSchema,
  type PolicyTargetType,
} from '@ocentra-parent/policy-domain/policy';
import {
  V08EnforcementProductControlSurfaceSchema,
  V08EnforcementProductControlParentActionSchema,
} from './v0-8-enforcement-product-control-spine';

export const EnforcementPolicyDispatchReadModelIdSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchReadModelId');
export const EnforcementPolicyDispatchIntentIdSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchIntentId');
export const EnforcementPolicyDispatchDecisionRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchDecisionRef');
export const EnforcementPolicyDispatchScheduleRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchScheduleRef');
export const EnforcementPolicyDispatchRouteRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchRouteRef');
export const EnforcementPolicyDispatchTimerRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchTimerRef');
export const EnforcementPolicyDispatchAuditRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchAuditRef');
export const EnforcementPolicyDispatchChildReasonRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchChildReasonRef');
export const EnforcementPolicyDispatchCapabilityMatrixIdSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchCapabilityMatrixId');

export const EnforcementPolicyDispatchSourceStateSchema = withParser(
  Schema.Literal('ready', 'stale', 'offline', 'missing', 'wrong-device', 'wrong-route', 'unavailable')
);

export const EnforcementPolicyDispatchProofLevelSchema = withParser(
  Schema.Literal('implemented', 'report-only', 'degraded', 'unavailable', 'manual-required', 'scaffold')
);

export const EnforcementPolicyDispatchOutcomeStateSchema = withParser(
  Schema.Literal(
    'dispatch-ready',
    'dry-run-only',
    'report-only',
    'manual-required',
    'degraded',
    'unavailable',
    'rejected'
  )
);

export const EnforcementPolicyDispatchRejectionReasonSchema = withParser(
  Schema.Literal(
    'none',
    'missing-actor',
    'wrong-device',
    'missing-policy-decision',
    'stale-policy-version',
    'missing-schedule-or-budget',
    'missing-evidence',
    'adapter-manual-required',
    'adapter-unavailable',
    'source-not-ready',
    'route-not-authorized',
    'broad-claim-not-proved'
  )
);

export const EnforcementPolicyDispatchApprovalStateSchema = withParser(
  Schema.Literal('not-required', 'pending', 'approved', 'denied', 'expired', 'override-active', 'manual-required')
);

export const EnforcementPolicyDispatchTimerStateSchema = withParser(
  Schema.Literal(
    'not-required',
    'active',
    'restart-recovered',
    'expired',
    'cancelled',
    'rollback-completed',
    'recovery-needed'
  )
);

const DispatchCapabilityMatrixRowBaseSchema = Schema.Struct({
  matrixId: EnforcementPolicyDispatchCapabilityMatrixIdSchema,
  surface: V08EnforcementProductControlSurfaceSchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  requestedAction: V08EnforcementProductControlParentActionSchema,
  mode: EnforcementModeSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  proofLevel: EnforcementPolicyDispatchProofLevelSchema,
  outcomeState: EnforcementPolicyDispatchOutcomeStateSchema,
  rejectionReason: EnforcementPolicyDispatchRejectionReasonSchema,
  sourceState: EnforcementPolicyDispatchSourceStateSchema,
  childReasonCode: EnforcementPolicyDispatchChildReasonRefSchema,
});

type DispatchCapabilityMatrixRowCandidate = Infer<typeof DispatchCapabilityMatrixRowBaseSchema>;

export const EnforcementPolicyDispatchCapabilityMatrixRowSchema = withParser(
  DispatchCapabilityMatrixRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        dispatchMatrixRowPreservesClaimBoundary(row) ||
        'Expected policy dispatch matrix rows to keep implemented, report-only, degraded, unavailable, manual-required, and scaffold states distinct'
    )
  )
);

export const EnforcementPolicyDispatchIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    intentId: EnforcementPolicyDispatchIntentIdSchema,
    actor: ParentActorReferenceSchema,
    device: ParentDeviceReferenceSchema,
    policyDecisionId: PolicyDecisionIdSchema,
    policyDecisionRef: EnforcementPolicyDispatchDecisionRefSchema,
    policyVersion: ParentPolicyVersionSchema,
    target: PolicyTargetSchema,
    requestedPolicyAction: PolicyActionSchema,
    requestedParentAction: V08EnforcementProductControlParentActionSchema,
    scheduleRef: EnforcementPolicyDispatchScheduleRefSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    approvalRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    routeRef: EnforcementPolicyDispatchRouteRefSchema,
    sourceState: EnforcementPolicyDispatchSourceStateSchema,
    dryRun: Schema.Boolean,
    requestedAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter(
        (intent) => intent.evidenceReferences.length > 0 || 'Expected dispatch intents to include evidence references'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          hasDispatchReferencePrefix(intent.policyDecisionId, 'policy-') &&
          hasDispatchReferencePrefix(intent.policyDecisionRef, 'decision-') &&
          hasDispatchReferencePrefix(intent.scheduleRef, 'schedule-') ||
          'Expected dispatch intents to keep stable policy decision and schedule references'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          intent.requestedParentAction !== 'ask-parent' ||
          intent.dryRun ||
          'Expected ask-parent dispatch intents to stay dry-run only until approval exists'
      )
    )
);

export const EnforcementPolicyDispatchReadModelEntrySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    intent: EnforcementPolicyDispatchIntentSchema,
    matrixRow: EnforcementPolicyDispatchCapabilityMatrixRowSchema,
    approvalState: EnforcementPolicyDispatchApprovalStateSchema,
    timerState: EnforcementPolicyDispatchTimerStateSchema,
    auditRefs: Schema.Array(EnforcementPolicyDispatchAuditRefSchema),
    timerRefs: Schema.Array(EnforcementPolicyDispatchTimerRefSchema),
    childReasonCode: EnforcementPolicyDispatchChildReasonRefSchema,
    reasonCodes: Schema.Array(PolicyReasonCodeSchema),
    dispatchedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    nextCheckAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (entry) =>
        entry.childReasonCode === entry.matrixRow.childReasonCode ||
        'Expected dispatch child reason code to match the matrix row reason'
    )
  )
);

export const EnforcementPolicyDispatchReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: EnforcementPolicyDispatchReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    entries: Schema.Array(EnforcementPolicyDispatchReadModelEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.intent.intentId)).size === readModel.entries.length ||
        'Expected policy dispatch read model intent ids to be unique'
    )
  )
);

function dispatchMatrixRowPreservesClaimBoundary(row: DispatchCapabilityMatrixRowCandidate): boolean {
  switch (row.proofLevel) {
    case 'implemented':
      return dispatchMatrixRowIsImplemented(row);
    case 'report-only':
      return dispatchMatrixRowIsReportOnly(row);
    case 'degraded':
      return dispatchMatrixRowIsDegraded(row);
    case 'unavailable':
      return dispatchMatrixRowIsUnavailable(row);
    case 'manual-required':
      return dispatchMatrixRowIsManualRequired(row);
    case 'scaffold':
      return dispatchMatrixRowIsScaffold(row);
  }
}

function dispatchMatrixRowIsImplemented(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.capabilityState === 'supported' && row.outcomeState === 'dispatch-ready' && row.rejectionReason === 'none';
}

function dispatchMatrixRowIsReportOnly(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.outcomeState === 'report-only' && row.rejectionReason === 'none';
}

function dispatchMatrixRowIsDegraded(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.capabilityState === 'degraded' && row.outcomeState === 'degraded';
}

function dispatchMatrixRowIsUnavailable(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.capabilityState === 'unavailable' && row.outcomeState === 'unavailable';
}

function dispatchMatrixRowIsManualRequired(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.capabilityState === 'manual-required' && row.outcomeState === 'manual-required';
}

function dispatchMatrixRowIsScaffold(row: DispatchCapabilityMatrixRowCandidate): boolean {
  return row.outcomeState === 'rejected' || row.outcomeState === 'dry-run-only';
}

function hasDispatchReferencePrefix(value: string, prefix: string): boolean {
  return value.startsWith(prefix) && value.length > prefix.length;
}

export type EnforcementPolicyDispatchReadModelId = typeof EnforcementPolicyDispatchReadModelIdSchema.Type;
export type EnforcementPolicyDispatchIntentId = typeof EnforcementPolicyDispatchIntentIdSchema.Type;
export type EnforcementPolicyDispatchDecisionRef = typeof EnforcementPolicyDispatchDecisionRefSchema.Type;
export type EnforcementPolicyDispatchScheduleRef = typeof EnforcementPolicyDispatchScheduleRefSchema.Type;
export type EnforcementPolicyDispatchRouteRef = typeof EnforcementPolicyDispatchRouteRefSchema.Type;
export type EnforcementPolicyDispatchTimerRef = typeof EnforcementPolicyDispatchTimerRefSchema.Type;
export type EnforcementPolicyDispatchAuditRef = typeof EnforcementPolicyDispatchAuditRefSchema.Type;
export type EnforcementPolicyDispatchChildReasonRef = typeof EnforcementPolicyDispatchChildReasonRefSchema.Type;
export type EnforcementPolicyDispatchCapabilityMatrixId = typeof EnforcementPolicyDispatchCapabilityMatrixIdSchema.Type;
export type EnforcementPolicyDispatchSourceState = Infer<typeof EnforcementPolicyDispatchSourceStateSchema>;
export type EnforcementPolicyDispatchProofLevel = Infer<typeof EnforcementPolicyDispatchProofLevelSchema>;
export type EnforcementPolicyDispatchOutcomeState = Infer<typeof EnforcementPolicyDispatchOutcomeStateSchema>;
export type EnforcementPolicyDispatchRejectionReason = Infer<typeof EnforcementPolicyDispatchRejectionReasonSchema>;
export type EnforcementPolicyDispatchApprovalState = Infer<typeof EnforcementPolicyDispatchApprovalStateSchema>;
export type EnforcementPolicyDispatchTimerState = Infer<typeof EnforcementPolicyDispatchTimerStateSchema>;
export type EnforcementPolicyDispatchCapabilityMatrixRow = Infer<
  typeof EnforcementPolicyDispatchCapabilityMatrixRowSchema
>;
export type EnforcementPolicyDispatchIntent = Infer<typeof EnforcementPolicyDispatchIntentSchema>;
export type EnforcementPolicyDispatchReadModelEntry = Infer<typeof EnforcementPolicyDispatchReadModelEntrySchema>;
export type EnforcementPolicyDispatchReadModel = Infer<typeof EnforcementPolicyDispatchReadModelSchema>;

type DispatchEntryInput = {
  intentId: string;
  matrixId: string;
  surface: typeof V08EnforcementProductControlSurfaceSchema.Type;
  platform: typeof ParentPlatformSchema.Type;
  adapterKind: typeof EnforcementAdapterKindSchema.Type;
  requestedAction: typeof V08EnforcementProductControlParentActionSchema.Type;
  mode: typeof EnforcementModeSchema.Type;
  capabilityState: typeof EnforcementCapabilityStateSchema.Type;
  proofLevel: EnforcementPolicyDispatchProofLevel;
  outcomeState: EnforcementPolicyDispatchOutcomeState;
  rejectionReason: EnforcementPolicyDispatchRejectionReason;
  sourceState: EnforcementPolicyDispatchSourceState;
  approvalState: EnforcementPolicyDispatchApprovalState;
  timerState: EnforcementPolicyDispatchTimerState;
  childReasonCode: string;
  targetType: PolicyTargetType;
  targetValue: string;
  evidenceReferences: readonly string[];
  auditRefs: readonly string[];
  timerRefs: readonly string[];
  policyVersion?: string;
  dryRun?: boolean;
};

const generatedAt = '2026-06-02T05:45:00.000Z';

export const EnforcementPolicyDispatchReadModel = EnforcementPolicyDispatchReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-enforcement-policy-dispatch',
  generatedAt,
  entries: [
    dispatchEntry({
      intentId: 'dispatch-owned-process-time-limit',
      matrixId: 'matrix-owned-process-implemented',
      surface: 'windows-owned-process-time-limit',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'block-scoped-process',
      mode: 'terminate-process',
      capabilityState: 'supported',
      proofLevel: 'implemented',
      outcomeState: 'dispatch-ready',
      rejectionReason: 'none',
      sourceState: 'ready',
      approvalState: 'not-required',
      timerState: 'active',
      childReasonCode: 'child-reason-time-limit-reached',
      targetType: 'app',
      targetValue: 'owned-process:ocentra-child-demo.exe',
      evidenceReferences: ['evidence-app-session-owned-process'],
      auditRefs: ['audit-owned-process-dispatch-accepted'],
      timerRefs: ['timer-owned-process-active'],
    }),
    dispatchEntry({
      intentId: 'dispatch-app-game-session-handoff',
      matrixId: 'matrix-app-game-time-limit-implemented',
      surface: 'windows-app-time-limit-lifecycle',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'time-limit',
      mode: 'time-limit',
      capabilityState: 'supported',
      proofLevel: 'implemented',
      outcomeState: 'dispatch-ready',
      rejectionReason: 'none',
      sourceState: 'ready',
      approvalState: 'pending',
      timerState: 'restart-recovered',
      childReasonCode: 'child-reason-parent-approval-bonus-time',
      targetType: 'app',
      targetValue: 'app-session:game-launcher',
      evidenceReferences: ['evidence-app-game-session-summary'],
      auditRefs: ['audit-app-game-session-handoff'],
      timerRefs: ['timer-app-game-recovered'],
    }),
    dispatchEntry({
      intentId: 'dispatch-ask-parent-dry-run',
      matrixId: 'matrix-ask-parent-dry-run',
      surface: 'windows-app-time-limit-lifecycle',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'ask-parent',
      mode: 'observe-only',
      capabilityState: 'supported',
      proofLevel: 'scaffold',
      outcomeState: 'dry-run-only',
      rejectionReason: 'none',
      sourceState: 'ready',
      approvalState: 'pending',
      timerState: 'not-required',
      childReasonCode: 'child-reason-ask-parent-review-required',
      targetType: 'app',
      targetValue: 'app-session:ask-parent-review',
      evidenceReferences: ['evidence-app-game-session-summary'],
      auditRefs: ['audit-ask-parent-dry-run'],
      timerRefs: [],
      dryRun: true,
    }),
    dispatchEntry({
      intentId: 'dispatch-unmanaged-browser-report-only',
      matrixId: 'matrix-unmanaged-browser-report-only',
      surface: 'windows-unmanaged-browser-process-fallback',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'report-only',
      mode: 'observe-only',
      capabilityState: 'degraded',
      proofLevel: 'report-only',
      outcomeState: 'report-only',
      rejectionReason: 'none',
      sourceState: 'ready',
      approvalState: 'not-required',
      timerState: 'not-required',
      childReasonCode: 'child-reason-browser-process-report-only',
      targetType: 'site',
      targetValue: 'unmanaged-browser-process',
      evidenceReferences: ['evidence-unmanaged-browser-process'],
      auditRefs: ['audit-unmanaged-browser-report-only'],
      timerRefs: [],
      dryRun: true,
    }),
    dispatchEntry({
      intentId: 'dispatch-network-domain-manual-required',
      matrixId: 'matrix-network-domain-manual-required',
      surface: 'windows-network-domain-blocking',
      platform: 'windows',
      adapterKind: 'network-control',
      requestedAction: 'report-only',
      mode: 'temporary-block',
      capabilityState: 'manual-required',
      proofLevel: 'manual-required',
      outcomeState: 'manual-required',
      rejectionReason: 'adapter-manual-required',
      sourceState: 'ready',
      approvalState: 'manual-required',
      timerState: 'not-required',
      childReasonCode: 'child-reason-adapter-manual-required',
      targetType: 'domain',
      targetValue: 'example.invalid',
      evidenceReferences: ['evidence-network-flow-domain-summary'],
      auditRefs: ['audit-network-domain-manual-required'],
      timerRefs: [],
    }),
    dispatchEntry({
      intentId: 'dispatch-stale-policy-version-rejected',
      matrixId: 'matrix-stale-policy-version-rejected',
      surface: 'windows-app-time-limit-lifecycle',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'time-limit',
      mode: 'time-limit',
      capabilityState: 'supported',
      proofLevel: 'scaffold',
      outcomeState: 'rejected',
      rejectionReason: 'stale-policy-version',
      sourceState: 'stale',
      approvalState: 'not-required',
      timerState: 'not-required',
      childReasonCode: 'child-reason-policy-version-stale',
      targetType: 'app',
      targetValue: 'app-session:game-launcher',
      evidenceReferences: ['evidence-policy-decision-stale'],
      auditRefs: ['audit-stale-policy-version-rejected'],
      timerRefs: [],
    }),
    dispatchEntry({
      intentId: 'dispatch-missing-source-rejected',
      matrixId: 'matrix-missing-source-rejected',
      surface: 'windows-app-time-limit-lifecycle',
      platform: 'windows',
      adapterKind: 'process-control',
      requestedAction: 'time-limit',
      mode: 'time-limit',
      capabilityState: 'supported',
      proofLevel: 'scaffold',
      outcomeState: 'rejected',
      rejectionReason: 'source-not-ready',
      sourceState: 'missing',
      approvalState: 'not-required',
      timerState: 'not-required',
      childReasonCode: 'child-reason-source-not-ready',
      targetType: 'app',
      targetValue: 'policy-source:missing',
      evidenceReferences: ['evidence-policy-source-missing'],
      auditRefs: ['audit-missing-source-rejected'],
      timerRefs: [],
    }),
    dispatchEntry({
      intentId: 'dispatch-tamper-alert-scaffold',
      matrixId: 'matrix-tamper-scaffold',
      surface: 'windows-tamper-uninstall-alerts',
      platform: 'windows',
      adapterKind: 'timer-control',
      requestedAction: 'observe',
      mode: 'observe-only',
      capabilityState: 'unavailable',
      proofLevel: 'scaffold',
      outcomeState: 'rejected',
      rejectionReason: 'broad-claim-not-proved',
      sourceState: 'unavailable',
      approvalState: 'not-required',
      timerState: 'recovery-needed',
      childReasonCode: 'child-reason-integrity-proof-required',
      targetType: 'device',
      targetValue: 'local-dev-agent',
      evidenceReferences: ['evidence-integrity-heartbeat-gap'],
      auditRefs: ['audit-tamper-non-claim'],
      timerRefs: ['timer-integrity-recovery-needed'],
    }),
  ],
});

function dispatchEntry(input: DispatchEntryInput): EnforcementPolicyDispatchReadModelEntry {
  return EnforcementPolicyDispatchReadModelEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intent: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      intentId: input.intentId,
      actor: {
        actorId: 'parent-actor-primary',
        role: 'parent',
      },
      device: {
        deviceId: 'local-dev-agent',
        childProfileId: 'child-profile-v0-8-dispatch',
        label: 'Local dev child device',
        platform: input.platform,
      },
      policyDecisionId: `policy-${input.intentId}`,
      policyDecisionRef: `decision-${input.intentId}`,
      policyVersion: input.policyVersion ?? 'policy-version-v0-8-dispatch',
      target: {
        targetId: `target-${input.intentId}`,
        targetType: input.targetType,
        targetValue: input.targetValue,
      },
      requestedPolicyAction: policyActionForParentAction(input.requestedAction),
      requestedParentAction: input.requestedAction,
      scheduleRef: `schedule-${input.intentId}`,
      evidenceReferences: input.evidenceReferences.map((referenceId) => ({
        evidenceReferenceId: referenceId,
        kind: 'activity-event',
        observedAt: generatedAt,
      })),
      approvalRef:
        input.approvalState === 'not-required'
          ? null
          : {
              actionReferenceId: `approval-${input.intentId}`,
              actor: {
                actorId: 'parent-actor-primary',
                role: 'parent',
              },
              policyVersion: 'policy-version-v0-8-dispatch',
              createdAt: generatedAt,
            },
      routeRef: 'route-localhost-agent-service',
      sourceState: input.sourceState,
      dryRun: input.dryRun ?? false,
      requestedAt: generatedAt,
    },
    matrixRow: {
      matrixId: input.matrixId,
      surface: input.surface,
      platform: input.platform,
      adapterKind: input.adapterKind,
      requestedAction: input.requestedAction,
      mode: input.mode,
      capabilityState: input.capabilityState,
      proofLevel: input.proofLevel,
      outcomeState: input.outcomeState,
      rejectionReason: input.rejectionReason,
      sourceState: input.sourceState,
      childReasonCode: input.childReasonCode,
    },
    approvalState: input.approvalState,
    timerState: input.timerState,
    auditRefs: input.auditRefs,
    timerRefs: input.timerRefs,
    childReasonCode: input.childReasonCode,
    reasonCodes: [input.childReasonCode],
    dispatchedAt: input.outcomeState === 'dispatch-ready' ? generatedAt : null,
    nextCheckAt: input.timerState === 'active' || input.timerState === 'restart-recovered' ? generatedAt : null,
  });
}

function policyActionForParentAction(action: typeof V08EnforcementProductControlParentActionSchema.Type): PolicyAction {
  switch (action) {
    case 'warn':
      return 'warn';
    case 'time-limit':
      return 'time-limit';
    case 'block-scoped-process':
      return 'block';
    case 'ask-parent':
      return 'ask-parent';
    case 'observe':
    case 'dry-run-preview':
    case 'report-only':
      return 'allow';
  }
}

