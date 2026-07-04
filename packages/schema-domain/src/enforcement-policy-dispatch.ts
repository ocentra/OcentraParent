import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { EnforcementAdapterKindSchema, EnforcementCapabilityStateSchema, EnforcementModeSchema } from './enforcement';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyTargetSchema,
} from '@ocentra-parent/schema-domain/policy-contracts';
import {
  V08EnforcementProductControlSurfaceSchema,
  V08EnforcementProductControlParentActionSchema,
} from './v0-8-enforcement-product-control-spine';

export const EnforcementPolicyDispatchReadModelIdSchema = brandedNonEmptyStringSchema(
  'EnforcementPolicyDispatchReadModelId'
);
export const EnforcementPolicyDispatchIntentIdSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchIntentId');
export const EnforcementPolicyDispatchDecisionRefSchema = brandedNonEmptyStringSchema(
  'EnforcementPolicyDispatchDecisionRef'
);
export const EnforcementPolicyDispatchScheduleRefSchema = brandedNonEmptyStringSchema(
  'EnforcementPolicyDispatchScheduleRef'
);
export const EnforcementPolicyDispatchRouteRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchRouteRef');
export const EnforcementPolicyDispatchTimerRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchTimerRef');
export const EnforcementPolicyDispatchAuditRefSchema = brandedNonEmptyStringSchema('EnforcementPolicyDispatchAuditRef');
export const EnforcementPolicyDispatchChildReasonRefSchema = brandedNonEmptyStringSchema(
  'EnforcementPolicyDispatchChildReasonRef'
);
export const EnforcementPolicyDispatchCapabilityMatrixIdSchema = brandedNonEmptyStringSchema(
  'EnforcementPolicyDispatchCapabilityMatrixId'
);

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
          (hasDispatchReferencePrefix(intent.policyDecisionId, 'policy-') &&
            hasDispatchReferencePrefix(intent.policyDecisionRef, 'decision-') &&
            hasDispatchReferencePrefix(intent.scheduleRef, 'schedule-')) ||
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
