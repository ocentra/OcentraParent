import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
  AgentAppGameAdapterHostCapabilityState,
} from './app-game-adapter-execution-readiness';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const DispatchPreflightCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameAdapterDispatchPreflightPayloadField = 'appGameAdapterDispatchPreflightReadModel' as const;

export const AgentAppGameAdapterDispatchPreflightState = {
  DispatchEligible: 'dispatch-eligible',
  BlockedBeforeDispatch: 'blocked-before-dispatch',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unsupported: 'unsupported',
  Degraded: 'degraded',
} as const;

export const AgentAppGameAdapterDispatchDecision = {
  DispatchEligible: 'dispatch-eligible',
  BlockedBeforeDispatch: 'blocked-before-dispatch',
} as const;

export const AgentAppGameAdapterDispatchOutcomeState = {
  DispatchReady: 'dispatch-ready',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unsupported: 'unsupported',
  Degraded: 'degraded',
  NotDispatched: 'not-dispatched',
} as const;

const AgentAppGameAdapterDispatchPreflightRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  rowId: NonEmptyStringSchema,
  sourceExecutionReadinessRowId: NonEmptyStringSchema,
  sourceProofEntryId: NonEmptyStringSchema,
  platform: NonEmptyStringSchema,
  productMeanings: Schema.Array(Schema.Literal('native-app', 'native-game')),
  adapterCapability: NonEmptyStringSchema,
  adapterExecutionState: Schema.Literal(
    AgentAppGameAdapterExecutionState.ProvedScopedExecution,
    AgentAppGameAdapterExecutionState.ManualRequired,
    AgentAppGameAdapterExecutionState.Unavailable,
    AgentAppGameAdapterExecutionState.Unsupported,
    AgentAppGameAdapterExecutionState.Degraded,
    AgentAppGameAdapterExecutionState.NotClaimed
  ),
  executionDecision: Schema.Literal(
    AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
    AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution
  ),
  dispatchPreflightState: Schema.Literal(
    AgentAppGameAdapterDispatchPreflightState.DispatchEligible,
    AgentAppGameAdapterDispatchPreflightState.BlockedBeforeDispatch,
    AgentAppGameAdapterDispatchPreflightState.ManualRequired,
    AgentAppGameAdapterDispatchPreflightState.Unavailable,
    AgentAppGameAdapterDispatchPreflightState.Unsupported,
    AgentAppGameAdapterDispatchPreflightState.Degraded
  ),
  dispatchDecision: Schema.Literal(
    AgentAppGameAdapterDispatchDecision.DispatchEligible,
    AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch
  ),
  dispatchIntentId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchOutcomeState: Schema.Literal(
    AgentAppGameAdapterDispatchOutcomeState.DispatchReady,
    AgentAppGameAdapterDispatchOutcomeState.ManualRequired,
    AgentAppGameAdapterDispatchOutcomeState.Unavailable,
    AgentAppGameAdapterDispatchOutcomeState.Unsupported,
    AgentAppGameAdapterDispatchOutcomeState.Degraded,
    AgentAppGameAdapterDispatchOutcomeState.NotDispatched
  ),
  dispatchEvidenceRefs: Schema.Array(NonEmptyStringSchema),
  hostCapabilityState: Schema.Literal(
    AgentAppGameAdapterHostCapabilityState.Available,
    AgentAppGameAdapterHostCapabilityState.NotDetected,
    AgentAppGameAdapterHostCapabilityState.NotApplicable
  ),
  hostCapabilityEvidenceRefs: Schema.Array(NonEmptyStringSchema),
  hostCapabilityProbeRefs: Schema.Array(NonEmptyStringSchema),
  dispatchAuditRefs: Schema.Array(NonEmptyStringSchema),
  dispatchTimerRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  claimBoundary: NonEmptyStringSchema,
  fallbackBehavior: NonEmptyStringSchema,
  adapterDispatchEligible: Schema.Boolean,
  adapterDispatchExecutedClaimed: Schema.Literal(false),
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  lastCheckedAt: NonEmptyStringSchema,
});

type AgentAppGameAdapterDispatchPreflightRowCandidate = Infer<typeof AgentAppGameAdapterDispatchPreflightRowBaseSchema>;

export const AgentAppGameAdapterDispatchPreflightRowSchema = withParser(
  AgentAppGameAdapterDispatchPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGameAdapterDispatchPreflightRowCandidate) =>
        dispatchPreflightRowIsHonest(row) ||
        'Expected only the scoped Windows owned-process time-limit row to be dispatch-eligible without ' +
          'claiming adapter execution'
    )
  )
);

const AgentAppGameAdapterDispatchPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  readModelId: NonEmptyStringSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
  custodyLabel: NonEmptyStringSchema,
  capabilityStatus: NonEmptyStringSchema,
  returned: DispatchPreflightCount,
  dispatchEligibleCount: DispatchPreflightCount,
  blockedBeforeDispatchCount: DispatchPreflightCount,
  adapterDispatchEligibleCount: DispatchPreflightCount,
  adapterDispatchExecutedClaimedCount: Schema.Literal(0),
  hostCapabilityAvailableCount: DispatchPreflightCount,
  hostCapabilityNotDetectedCount: DispatchPreflightCount,
  hostCapabilityNotApplicableCount: DispatchPreflightCount,
  hostCapabilityProbeRefCount: DispatchPreflightCount,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  rows: Schema.Array(AgentAppGameAdapterDispatchPreflightRowSchema),
});

type AgentAppGameAdapterDispatchPreflightReadModelCandidate = Infer<
  typeof AgentAppGameAdapterDispatchPreflightReadModelBaseSchema
>;

export const AgentAppGameAdapterDispatchPreflightReadModelSchema = withParser(
  AgentAppGameAdapterDispatchPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGameAdapterDispatchPreflightReadModelCandidate) =>
        (readModel.returned === readModel.rows.length &&
          readModel.dispatchEligibleCount ===
            readModel.rows.filter(
              (row) => row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible
            ).length &&
          readModel.blockedBeforeDispatchCount ===
            readModel.rows.filter(
              (row) => row.dispatchDecision === AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch
            ).length &&
          readModel.adapterDispatchEligibleCount ===
            readModel.rows.filter((row) => row.adapterDispatchEligible).length &&
          readModel.hostCapabilityAvailableCount ===
            readModel.rows.filter((row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available)
              .length &&
          readModel.hostCapabilityNotDetectedCount ===
            readModel.rows.filter(
              (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotDetected
            ).length &&
          readModel.hostCapabilityNotApplicableCount ===
            readModel.rows.filter(
              (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable
            ).length &&
          readModel.hostCapabilityProbeRefCount ===
            readModel.rows.reduce((count, row) => count + row.hostCapabilityProbeRefs.length, 0) &&
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length) ||
        'Expected app/game adapter dispatch preflight counts and row ids to match the rows'
    )
  )
);

export type AgentAppGameAdapterDispatchPreflightRow = Infer<typeof AgentAppGameAdapterDispatchPreflightRowSchema>;
export type AgentAppGameAdapterDispatchPreflightReadModel = Infer<
  typeof AgentAppGameAdapterDispatchPreflightReadModelSchema
>;

export type AgentAppGameAdapterDispatchPreflightFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameAdapterDispatchPreflightResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameAdapterDispatchPreflightReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameAdapterDispatchPreflightFailureReason;
    };

export function parseAgentAppGameAdapterDispatchPreflightEvent(
  event: AgentEventEnvelope
): AgentAppGameAdapterDispatchPreflightResult {
  if (event.event !== AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported) {
    return dispatchPreflightFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGameAdapterDispatchPreflightPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return dispatchPreflightFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return dispatchPreflightFailure('invalid-json');
  }

  const parsed = AgentAppGameAdapterDispatchPreflightReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return dispatchPreflightFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function dispatchPreflightRowIsHonest(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  if (row.dispatchPreflightState === AgentAppGameAdapterDispatchPreflightState.DispatchEligible) {
    return (
      row.platform === 'windows' &&
      row.sourceProofEntryId === 'windows-app-game-owned-process-time-limit' &&
      row.executionDecision === AgentAppGameAdapterExecutionDecision.ExecutionAllowed &&
      row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible &&
      row.dispatchOutcomeState === AgentAppGameAdapterDispatchOutcomeState.DispatchReady &&
      row.dispatchIntentId !== null &&
      row.dispatchEvidenceRefs.length > 0 &&
      row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available &&
      row.hostCapabilityEvidenceRefs.length > 0 &&
      row.hostCapabilityProbeRefs.length > 0 &&
      row.dispatchAuditRefs.length > 0 &&
      row.dispatchTimerRefs.length > 0 &&
      row.manualProofRequirements.length === 0 &&
      row.adapterDispatchEligible &&
      !row.adapterDispatchExecutedClaimed
    );
  }

  return (
    row.dispatchDecision === AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch &&
    row.dispatchIntentId === null &&
    row.dispatchOutcomeState !== AgentAppGameAdapterDispatchOutcomeState.DispatchReady &&
    hostCapabilityStateMatchesEvidence(row) &&
    hostCapabilityProbeRefsAreParentSafe(row) &&
    row.manualProofRequirements.length > 0 &&
    !row.adapterDispatchEligible &&
    !row.adapterDispatchExecutedClaimed
  );
}

function hostCapabilityStateMatchesEvidence(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available) {
    return row.hostCapabilityEvidenceRefs.length > 0;
  }
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable) {
    return row.hostCapabilityEvidenceRefs.length === 0;
  }
  return true;
}

function hostCapabilityProbeRefsAreParentSafe(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable) {
    return row.hostCapabilityProbeRefs.length === 0;
  }
  return row.hostCapabilityProbeRefs.every((ref) => ref.endsWith('-probe-ref'));
}

function dispatchPreflightFailure(
  reason: AgentAppGameAdapterDispatchPreflightFailureReason
): AgentAppGameAdapterDispatchPreflightResult {
  return {
    ok: false,
    reason,
  };
}
