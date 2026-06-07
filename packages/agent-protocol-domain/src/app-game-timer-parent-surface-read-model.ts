import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const TimerParentSurfaceText = Schema.String.pipe(Schema.minLength(1));
const TimerParentSurfaceCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

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
    rowId: TimerParentSurfaceText,
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
    evidenceReferenceIds: Schema.Array(TimerParentSurfaceText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameTimerParentSurfaceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: TimerParentSurfaceText,
    custodyLabel: TimerParentSurfaceText,
    capabilityStatus: TimerParentSurfaceText,
    returned: TimerParentSurfaceCount,
    readyForParentSurfaceCount: TimerParentSurfaceCount,
    blockedBySourceFreshnessCount: TimerParentSurfaceCount,
    blockedByCompilerDecisionCount: TimerParentSurfaceCount,
    runtimeManualRequiredCount: TimerParentSurfaceCount,
    controlActionResultCount: TimerParentSurfaceCount,
    controlActionResultReferenceIds: Schema.Array(TimerParentSurfaceText),
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
