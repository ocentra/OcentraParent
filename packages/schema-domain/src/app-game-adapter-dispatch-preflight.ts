import { AppGameSchemaVersion } from './app-game-primitives';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
  AgentAppGameAdapterHostCapabilityState,
} from './app-game-adapter-execution-readiness';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

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
        dispatchPreflightReadModelIsConsistent(readModel) ||
        'Expected app/game adapter dispatch preflight counts and row ids to match the rows'
    )
  )
);

export type AgentAppGameAdapterDispatchPreflightRow = Infer<typeof AgentAppGameAdapterDispatchPreflightRowSchema>;
export type AgentAppGameAdapterDispatchPreflightReadModel = Infer<
  typeof AgentAppGameAdapterDispatchPreflightReadModelSchema
>;

function dispatchPreflightRowIsHonest(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  return row.dispatchPreflightState === AgentAppGameAdapterDispatchPreflightState.DispatchEligible
    ? dispatchEligiblePreflightRowIsHonest(row)
    : blockedPreflightRowIsHonest(row);
}

function dispatchPreflightReadModelIsConsistent(
  readModel: AgentAppGameAdapterDispatchPreflightReadModelCandidate
): boolean {
  const countExpectations = [
    {
      expected: readModel.dispatchEligibleCount,
      actual: readModel.rows.filter(
        (row) => row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible
      ).length,
    },
    {
      expected: readModel.blockedBeforeDispatchCount,
      actual: readModel.rows.filter(
        (row) => row.dispatchDecision === AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch
      ).length,
    },
    {
      expected: readModel.adapterDispatchEligibleCount,
      actual: readModel.rows.filter((row) => row.adapterDispatchEligible).length,
    },
    {
      expected: readModel.hostCapabilityAvailableCount,
      actual: readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available
      ).length,
    },
    {
      expected: readModel.hostCapabilityNotDetectedCount,
      actual: readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotDetected
      ).length,
    },
    {
      expected: readModel.hostCapabilityNotApplicableCount,
      actual: readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable
      ).length,
    },
    {
      expected: readModel.hostCapabilityProbeRefCount,
      actual: readModel.rows.reduce((count, row) => count + row.hostCapabilityProbeRefs.length, 0),
    },
  ] as const;

  return (
    readModel.returned === readModel.rows.length &&
    countExpectations.every(({ expected, actual }) => expected === actual) &&
    new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length
  );
}

function dispatchEligiblePreflightRowIsHonest(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  return (
    dispatchEligiblePreflightRowMatchesBoundary(row) &&
    dispatchEligiblePreflightRowTracksDispatch(row) &&
    dispatchEligiblePreflightRowTracksHostCapability(row) &&
    dispatchEligiblePreflightRowKeepsClaimsScoped(row)
  );
}

function blockedPreflightRowIsHonest(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
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

function dispatchEligiblePreflightRowMatchesBoundary(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  return (
    row.platform === 'windows' &&
    row.sourceProofEntryId === 'windows-app-game-owned-process-time-limit' &&
    row.executionDecision === AgentAppGameAdapterExecutionDecision.ExecutionAllowed
  );
}

function dispatchEligiblePreflightRowTracksDispatch(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  return (
    row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible &&
    row.dispatchOutcomeState === AgentAppGameAdapterDispatchOutcomeState.DispatchReady &&
    row.dispatchIntentId !== null &&
    row.dispatchEvidenceRefs.length > 0 &&
    row.dispatchAuditRefs.length > 0 &&
    row.dispatchTimerRefs.length > 0
  );
}

function dispatchEligiblePreflightRowTracksHostCapability(
  row: AgentAppGameAdapterDispatchPreflightRowCandidate
): boolean {
  return (
    row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available &&
    row.hostCapabilityEvidenceRefs.length > 0 &&
    row.hostCapabilityProbeRefs.length > 0
  );
}

function dispatchEligiblePreflightRowKeepsClaimsScoped(row: AgentAppGameAdapterDispatchPreflightRowCandidate): boolean {
  return row.manualProofRequirements.length === 0 && row.adapterDispatchEligible && !row.adapterDispatchExecutedClaimed;
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
