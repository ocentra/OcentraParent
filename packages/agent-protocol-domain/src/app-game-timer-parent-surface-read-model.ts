import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const TimerParentSurfaceCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const TimerParentSurfaceParentPreferenceSetupRequestStatus = Schema.Literal('request-ready', 'unavailable-visible');

export const AgentAppGameTimerParentSurfaceTargetDomain = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AgentAppGameTimerParentSurfaceState = {
  ReadyForParentSurface: 'ready-for-parent-surface',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  RuntimeManualRequired: 'runtime-manual-required',
} as const;

export const AgentAppGameTimerParentSurfaceRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rowId: NonEmptyStringSchema,
    targetDomain: Schema.Literal(
      AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      AgentAppGameTimerParentSurfaceTargetDomain.NativeGame
    ),
    timerSurfaceState: Schema.Literal(
      AgentAppGameTimerParentSurfaceState.ReadyForParentSurface,
      AgentAppGameTimerParentSurfaceState.BlockedBySourceFreshness,
      AgentAppGameTimerParentSurfaceState.BlockedByCompilerDecision,
      AgentAppGameTimerParentSurfaceState.RuntimeManualRequired
    ),
    rowCount: TimerParentSurfaceCount,
    evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameTimerParentSurfaceChildUxLocalArtifactRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    artifactReferenceId: NonEmptyStringSchema,
    sourceResultId: NonEmptyStringSchema,
    targetDomain: Schema.Literal(
      AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      AgentAppGameTimerParentSurfaceTargetDomain.NativeGame
    ),
    childReasonReferenceIds: Schema.Array(NonEmptyStringSchema),
    childStatusReferenceIds: Schema.Array(NonEmptyStringSchema),
    childDeliveryClaimed: Schema.Literal(false),
    notificationDeliveryClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
  })
);

export const AgentAppGameTimerParentSurfaceChildUxParentSurfaceIntentRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    parentSurfaceIntentReferenceId: NonEmptyStringSchema,
    sourceResultId: NonEmptyStringSchema,
    sourceArtifactReferenceId: NonEmptyStringSchema,
    targetDomain: Schema.Literal(
      AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      AgentAppGameTimerParentSurfaceTargetDomain.NativeGame
    ),
    historyVisibility: Schema.Literal('history-row-visible'),
    parentSurfaceStatus: Schema.Literal('manual-action-required'),
    preferenceVisibility: Schema.Literal('preference-setup-required'),
    drillInReferenceIds: Schema.Array(NonEmptyStringSchema),
    manualProofReferenceIds: Schema.Array(NonEmptyStringSchema),
    sensitiveDetailIncluded: Schema.Literal(false),
    parentNotificationUiRendered: Schema.Literal(false),
    parentPreferenceMutationClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
  })
);

export const AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    parentPreferenceSetupReferenceId: NonEmptyStringSchema,
    sourceParentSurfaceIntentReferenceId: NonEmptyStringSchema,
    sourceResultId: NonEmptyStringSchema,
    sourceArtifactReferenceId: NonEmptyStringSchema,
    targetDomain: Schema.Literal(
      AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      AgentAppGameTimerParentSurfaceTargetDomain.NativeGame
    ),
    draftStatus: Schema.Literal('draft-ready', 'unavailable-visible'),
    parentPreferenceSetupRequestStatus: TimerParentSurfaceParentPreferenceSetupRequestStatus,
    parentPreferenceSetupRequestReferenceIds: Schema.Array(NonEmptyStringSchema),
    drillInReferenceIds: Schema.Array(NonEmptyStringSchema),
    manualProofReferenceIds: Schema.Array(NonEmptyStringSchema),
    parentPreferenceUiRendered: Schema.Literal(false),
    parentFrequencyControlUiRendered: Schema.Literal(false),
    parentPreferenceMutationClaimed: Schema.Literal(false),
    notificationRuleMutationClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
  })
);

export const AgentAppGameTimerParentSurfaceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: NonEmptyStringSchema,
    custodyLabel: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
    returned: TimerParentSurfaceCount,
    readyForParentSurfaceCount: TimerParentSurfaceCount,
    blockedBySourceFreshnessCount: TimerParentSurfaceCount,
    blockedByCompilerDecisionCount: TimerParentSurfaceCount,
    runtimeManualRequiredCount: TimerParentSurfaceCount,
    controlActionResultCount: TimerParentSurfaceCount,
    controlActionResultReferenceIds: Schema.Array(NonEmptyStringSchema),
    controlActionResultStatuses: Schema.Array(NonEmptyStringSchema),
    controlActionResultCapabilityStates: Schema.Array(NonEmptyStringSchema),
    controlActionResultEnforcementStatuses: Schema.Array(NonEmptyStringSchema),
    childFacingReasonReferenceIds: Schema.Array(NonEmptyStringSchema),
    childFacingStatusReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxHandoffReadyCount: TimerParentSurfaceCount,
    childUxHandoffBlockedCount: TimerParentSurfaceCount,
    childUxHandoffReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxLocalHandoffArtifactRecordCount: TimerParentSurfaceCount,
    childUxLocalHandoffArtifactSkippedCount: TimerParentSurfaceCount,
    childUxLocalHandoffArtifactReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxLocalHandoffArtifactRecords: Schema.Array(AgentAppGameTimerParentSurfaceChildUxLocalArtifactRecordSchema),
    childUxParentSurfaceIntentManualActionRequiredCount: TimerParentSurfaceCount,
    childUxParentSurfaceIntentUnavailableVisibleCount: TimerParentSurfaceCount,
    childUxParentSurfaceIntentHistoryVisibleCount: TimerParentSurfaceCount,
    childUxParentSurfaceIntentPreferenceSetupRequiredCount: TimerParentSurfaceCount,
    childUxParentSurfaceIntentReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxParentSurfaceIntentRecords: Schema.Array(
      AgentAppGameTimerParentSurfaceChildUxParentSurfaceIntentRecordSchema
    ),
    childUxParentPreferenceSetupDraftReadyCount: TimerParentSurfaceCount,
    childUxParentPreferenceSetupUnavailableVisibleCount: TimerParentSurfaceCount,
    childUxParentPreferenceSetupReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxParentPreferenceSetupRequestReadyCount: TimerParentSurfaceCount,
    childUxParentPreferenceSetupRequestUnavailableVisibleCount: TimerParentSurfaceCount,
    childUxParentPreferenceSetupRequestReferenceIds: Schema.Array(NonEmptyStringSchema),
    childUxParentPreferenceSetupRecords: Schema.Array(
      AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecordSchema
    ),
    timerRuntimeClaimed: Schema.Boolean,
    schedulerPersistenceClaimed: Schema.Boolean,
    durableSchedulerStorageClaimed: Schema.Boolean,
    auditRuntimeClaimed: Schema.Boolean,
    rollbackRuntimeClaimed: Schema.Boolean,
    adapterDispatchClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rawPrivateSourceRowsIncluded: Schema.Literal(false),
    rows: Schema.Array(AgentAppGameTimerParentSurfaceRowSchema),
  })
);

export type AgentAppGameTimerParentSurfaceTargetDomain = Infer<
  typeof AgentAppGameTimerParentSurfaceRowSchema
>['targetDomain'];
export type AgentAppGameTimerParentSurfaceState = Infer<
  typeof AgentAppGameTimerParentSurfaceRowSchema
>['timerSurfaceState'];
export type AgentAppGameTimerParentSurfaceRow = Infer<typeof AgentAppGameTimerParentSurfaceRowSchema>;
export type AgentAppGameTimerParentSurfaceChildUxLocalArtifactRecord = Infer<
  typeof AgentAppGameTimerParentSurfaceChildUxLocalArtifactRecordSchema
>;
export type AgentAppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord = Infer<
  typeof AgentAppGameTimerParentSurfaceChildUxParentSurfaceIntentRecordSchema
>;
export type AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord = Infer<
  typeof AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecordSchema
>;
export type AgentAppGameTimerParentSurfaceReadModel = Infer<typeof AgentAppGameTimerParentSurfaceReadModelSchema>;

export type AgentAppGameTimerParentSurfaceFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameTimerParentSurfaceResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameTimerParentSurfaceReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameTimerParentSurfaceFailureReason;
    };

export function parseAgentAppGameTimerParentSurfaceEvent(
  event: AgentEventEnvelope
): AgentAppGameTimerParentSurfaceResult {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameTimerParentSurfaceReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGameTimerParentSurfaceReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentAppGameTimerParentSurfaceFailureReason): AgentAppGameTimerParentSurfaceResult {
  return {
    ok: false,
    reason,
  };
}
