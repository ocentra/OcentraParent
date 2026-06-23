import { AppGameSchemaVersion } from './app-game-primitives';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from './app-game-adapter-dispatch-preflight';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const DispatchResultCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const AgentEnforcementExecuteCommand = 'agent.enforcement.execute' as const;
const AgentEnforcementAuditReportedEvent = 'agent.enforcement.audit.reported' as const;
const AgentActivityAppGameAdapterDispatchResultReadModelGetCommand =
  'agent.activity.app-game.adapter-dispatch-result.read-model.get' as const;

export const AgentAppGameAdapterDispatchResultPayloadField = 'appGameAdapterDispatchResultReadModel' as const;
export const AgentAppGameAdapterDispatchExecuteResultPayloadField = 'appGameAdapterDispatchExecuteResult' as const;

export const AgentAppGameAdapterDispatchCommandResultState = {
  CommandAccepted: 'command-accepted',
  BlockedBeforeCommand: 'blocked-before-command',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unsupported: 'unsupported',
  Degraded: 'degraded',
} as const;

export const AgentAppGameAdapterDispatchCommandResultDecision = {
  CommandAccepted: 'command-accepted',
  BlockedBeforeCommand: 'blocked-before-command',
} as const;

export const AgentAppGameAdapterDispatchExecutionAuditState = {
  ServiceLocalAuditRecorded: 'service-local-audit-recorded',
  BlockedBeforeExecutionAudit: 'blocked-before-execution-audit',
} as const;

export const AgentAppGameAdapterDispatchExecutionAuditDecision = {
  ServiceLocalAuditRecorded: 'service-local-audit-recorded',
  BlockedBeforeExecutionAudit: 'blocked-before-execution-audit',
} as const;

export const AgentAppGameAdapterDispatchAdapterExecutionState = {
  AdapterExecutionReported: 'adapter-execution-reported',
  AdapterExecutionEvidenceMissing: 'adapter-execution-evidence-missing',
  BlockedBeforeAdapterExecution: 'blocked-before-adapter-execution',
} as const;

export const AgentAppGameAdapterDispatchAdapterExecutionDecision = {
  AdapterExecutionReported: 'adapter-execution-reported',
  AdapterExecutionEvidenceMissing: 'adapter-execution-evidence-missing',
  BlockedBeforeAdapterExecution: 'blocked-before-adapter-execution',
} as const;

const AgentAppGameAdapterDispatchResultRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  rowId: NonEmptyStringSchema,
  sourceDispatchPreflightRowId: NonEmptyStringSchema,
  sourceProofEntryId: NonEmptyStringSchema,
  platform: NonEmptyStringSchema,
  productMeanings: Schema.Array(Schema.Literal('native-app', 'native-game')),
  adapterCapability: NonEmptyStringSchema,
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
  dispatchCommandResultState: Schema.Literal(
    AgentAppGameAdapterDispatchCommandResultState.CommandAccepted,
    AgentAppGameAdapterDispatchCommandResultState.BlockedBeforeCommand,
    AgentAppGameAdapterDispatchCommandResultState.ManualRequired,
    AgentAppGameAdapterDispatchCommandResultState.Unavailable,
    AgentAppGameAdapterDispatchCommandResultState.Unsupported,
    AgentAppGameAdapterDispatchCommandResultState.Degraded
  ),
  dispatchCommandResultDecision: Schema.Literal(
    AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted,
    AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand
  ),
  enforcementCommandName: Schema.Union(Schema.Literal(AgentEnforcementExecuteCommand), Schema.Null),
  enforcementEventName: Schema.Union(Schema.Literal(AgentEnforcementAuditReportedEvent), Schema.Null),
  enforcementActionMode: Schema.Union(Schema.Literal('terminate-process'), Schema.Null),
  dispatchCommandResultId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchCommandAuditRefs: Schema.Array(NonEmptyStringSchema),
  dispatchCommandTimerRefs: Schema.Array(NonEmptyStringSchema),
  dispatchExecutionAuditState: Schema.Literal(
    AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded,
    AgentAppGameAdapterDispatchExecutionAuditState.BlockedBeforeExecutionAudit
  ),
  dispatchExecutionAuditDecision: Schema.Literal(
    AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded,
    AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit
  ),
  dispatchExecutionAuditId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchExecutionAuditRefs: Schema.Array(NonEmptyStringSchema),
  dispatchAdapterExecutionState: Schema.Literal(
    AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported,
    AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionEvidenceMissing,
    AgentAppGameAdapterDispatchAdapterExecutionState.BlockedBeforeAdapterExecution
  ),
  dispatchAdapterExecutionDecision: Schema.Literal(
    AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported,
    AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionEvidenceMissing,
    AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution
  ),
  dispatchAdapterExecutionResultId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchAdapterExecutionStatus: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchAdapterExecutionAdapterResultCode: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchAdapterExecutionAuditEventId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  dispatchAdapterExecutionRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  claimBoundary: NonEmptyStringSchema,
  fallbackBehavior: NonEmptyStringSchema,
  adapterDispatchCommandResultClaimed: Schema.Boolean,
  adapterDispatchExecutedClaimed: Schema.Boolean,
  serviceLocalExecutionAuditClaimed: Schema.Boolean,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  lastCheckedAt: NonEmptyStringSchema,
});

type AgentAppGameAdapterDispatchResultRowCandidate = Infer<typeof AgentAppGameAdapterDispatchResultRowBaseSchema>;

export const AgentAppGameAdapterDispatchResultRowSchema = withParser(
  AgentAppGameAdapterDispatchResultRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGameAdapterDispatchResultRowCandidate) =>
        dispatchResultRowIsHonest(row) ||
        'Expected only the scoped Windows owned-process time-limit row to claim an accepted dispatch ' +
          'command result without claiming adapter execution'
    )
  )
);

const AgentAppGameAdapterDispatchResultReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  readModelId: NonEmptyStringSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
  custodyLabel: NonEmptyStringSchema,
  capabilityStatus: NonEmptyStringSchema,
  returned: DispatchResultCount,
  commandAcceptedCount: DispatchResultCount,
  blockedBeforeCommandCount: DispatchResultCount,
  executionAuditRecordedCount: DispatchResultCount,
  blockedBeforeExecutionAuditCount: DispatchResultCount,
  adapterExecutionReportedCount: DispatchResultCount,
  adapterExecutionEvidenceMissingCount: DispatchResultCount,
  blockedBeforeAdapterExecutionCount: DispatchResultCount,
  adapterDispatchCommandResultClaimedCount: DispatchResultCount,
  serviceLocalExecutionAuditClaimedCount: DispatchResultCount,
  adapterDispatchExecutedClaimedCount: DispatchResultCount,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  rows: Schema.Array(AgentAppGameAdapterDispatchResultRowSchema),
});

type AgentAppGameAdapterDispatchResultReadModelCandidate = Infer<
  typeof AgentAppGameAdapterDispatchResultReadModelBaseSchema
>;

export const AgentAppGameAdapterDispatchResultReadModelSchema = withParser(
  AgentAppGameAdapterDispatchResultReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGameAdapterDispatchResultReadModelCandidate) =>
        dispatchResultReadModelIsConsistent(readModel) ||
        'Expected app/game adapter dispatch result counts and row ids to match the rows'
    )
  )
);

const AgentAppGameAdapterDispatchExecuteResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  commandId: NonEmptyStringSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelId: Schema.Literal('app-game-adapter-dispatch-result'),
  sourceDispatchRowId: Schema.Literal('app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit'),
  sourceProofEntryId: Schema.Literal('windows-app-game-owned-process-time-limit'),
  executionCommandName: Schema.Literal(AgentEnforcementExecuteCommand),
  executionEventName: Schema.Literal(AgentEnforcementAuditReportedEvent),
  executionResultId: NonEmptyStringSchema,
  executionStatus: NonEmptyStringSchema,
  executionAdapterResultCode: NonEmptyStringSchema,
  executionAuditEventId: NonEmptyStringSchema,
  readbackCommandName: Schema.Literal(AgentActivityAppGameAdapterDispatchResultReadModelGetCommand),
  adapterDispatchExecutedClaimed: Schema.Boolean,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
});

type AgentAppGameAdapterDispatchExecuteResultCandidate = Infer<
  typeof AgentAppGameAdapterDispatchExecuteResultBaseSchema
>;

export const AgentAppGameAdapterDispatchExecuteResultSchema = withParser(
  AgentAppGameAdapterDispatchExecuteResultBaseSchema.pipe(
    Schema.filter(
      (result: AgentAppGameAdapterDispatchExecuteResultCandidate) =>
        result.executionStatus === 'actually-enforced' ||
        !result.adapterDispatchExecutedClaimed ||
        'Expected scoped adapter dispatch execution to claim execution only when enforcement reports actually-enforced'
    )
  )
);

export type AgentAppGameAdapterDispatchResultRow = Infer<typeof AgentAppGameAdapterDispatchResultRowSchema>;
export type AgentAppGameAdapterDispatchResultReadModel = Infer<typeof AgentAppGameAdapterDispatchResultReadModelSchema>;
export type AgentAppGameAdapterDispatchExecuteResult = Infer<typeof AgentAppGameAdapterDispatchExecuteResultSchema>;

function dispatchResultRowIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return row.dispatchCommandResultState === AgentAppGameAdapterDispatchCommandResultState.CommandAccepted
    ? acceptedDispatchResultRowIsHonest(row)
    : blockedDispatchResultRowIsHonest(row);
}

function acceptedExecutionEvidenceIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return row.dispatchAdapterExecutionDecision ===
    AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported
    ? reportedExecutionEvidenceIsHonest(row)
    : missingExecutionEvidenceIsHonest(row);
}

function dispatchResultReadModelIsConsistent(readModel: AgentAppGameAdapterDispatchResultReadModelCandidate): boolean {
  const countExpectations = [
    {
      expected: readModel.commandAcceptedCount,
      actual: readModel.rows.filter(
        (row) => row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted
      ).length,
    },
    {
      expected: readModel.blockedBeforeCommandCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand
      ).length,
    },
    {
      expected: readModel.executionAuditRecordedCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchExecutionAuditDecision ===
          AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded
      ).length,
    },
    {
      expected: readModel.blockedBeforeExecutionAuditCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchExecutionAuditDecision ===
          AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit
      ).length,
    },
    {
      expected: readModel.adapterExecutionReportedCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchAdapterExecutionDecision ===
          AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported
      ).length,
    },
    {
      expected: readModel.adapterExecutionEvidenceMissingCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchAdapterExecutionDecision ===
          AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionEvidenceMissing
      ).length,
    },
    {
      expected: readModel.blockedBeforeAdapterExecutionCount,
      actual: readModel.rows.filter(
        (row) =>
          row.dispatchAdapterExecutionDecision ===
          AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution
      ).length,
    },
    {
      expected: readModel.adapterDispatchCommandResultClaimedCount,
      actual: readModel.rows.filter((row) => row.adapterDispatchCommandResultClaimed).length,
    },
    {
      expected: readModel.serviceLocalExecutionAuditClaimedCount,
      actual: readModel.rows.filter((row) => row.serviceLocalExecutionAuditClaimed).length,
    },
    {
      expected: readModel.adapterDispatchExecutedClaimedCount,
      actual: readModel.rows.filter((row) => row.adapterDispatchExecutedClaimed).length,
    },
  ] as const;

  return (
    readModel.returned === readModel.rows.length &&
    countExpectations.every(({ expected, actual }) => expected === actual) &&
    new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length
  );
}

function acceptedDispatchResultRowIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    acceptedDispatchResultMatchesBoundary(row) &&
    acceptedDispatchResultTracksCommand(row) &&
    acceptedDispatchResultTracksExecutionAudit(row) &&
    acceptedDispatchResultKeepsClaimsScoped(row)
  );
}

function blockedDispatchResultRowIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    blockedDispatchResultTracksCommand(row) &&
    blockedDispatchResultTracksExecutionAudit(row) &&
    blockedDispatchResultTracksAdapterExecution(row) &&
    blockedDispatchResultKeepsClaimsScoped(row)
  );
}

function reportedExecutionEvidenceIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchAdapterExecutionState === AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported &&
    row.dispatchAdapterExecutionResultId !== null &&
    row.dispatchAdapterExecutionStatus === 'actually-enforced' &&
    row.dispatchAdapterExecutionAdapterResultCode !== null &&
    row.dispatchAdapterExecutionAuditEventId !== null &&
    row.dispatchAdapterExecutionRefs.length > 0 &&
    row.adapterDispatchExecutedClaimed
  );
}

function missingExecutionEvidenceIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchAdapterExecutionState ===
      AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionEvidenceMissing &&
    row.dispatchAdapterExecutionDecision ===
      AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionEvidenceMissing &&
    row.dispatchAdapterExecutionResultId === null &&
    row.dispatchAdapterExecutionStatus === null &&
    row.dispatchAdapterExecutionAdapterResultCode === null &&
    row.dispatchAdapterExecutionAuditEventId === null &&
    row.dispatchAdapterExecutionRefs.length === 0 &&
    !row.adapterDispatchExecutedClaimed
  );
}

function acceptedDispatchResultMatchesBoundary(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.platform === 'windows' &&
    row.sourceProofEntryId === 'windows-app-game-owned-process-time-limit' &&
    row.dispatchPreflightState === AgentAppGameAdapterDispatchPreflightState.DispatchEligible &&
    row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible &&
    row.dispatchOutcomeState === AgentAppGameAdapterDispatchOutcomeState.DispatchReady &&
    row.dispatchIntentId !== null
  );
}

function acceptedDispatchResultTracksCommand(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted &&
    row.enforcementCommandName === AgentEnforcementExecuteCommand &&
    row.enforcementEventName === AgentEnforcementAuditReportedEvent &&
    row.enforcementActionMode === 'terminate-process' &&
    row.dispatchCommandResultId !== null &&
    row.dispatchCommandAuditRefs.length > 0 &&
    row.dispatchCommandTimerRefs.length > 0
  );
}

function acceptedDispatchResultTracksExecutionAudit(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchExecutionAuditState === AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded &&
    row.dispatchExecutionAuditDecision ===
      AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded &&
    row.dispatchExecutionAuditId !== null &&
    row.dispatchExecutionAuditRefs.length > 0
  );
}

function acceptedDispatchResultKeepsClaimsScoped(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.manualProofRequirements.length === 0 &&
    row.adapterDispatchCommandResultClaimed &&
    row.serviceLocalExecutionAuditClaimed &&
    acceptedExecutionEvidenceIsHonest(row)
  );
}

function blockedDispatchResultTracksCommand(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand &&
    row.enforcementCommandName === null &&
    row.enforcementEventName === null &&
    row.enforcementActionMode === null &&
    row.dispatchCommandResultId === null
  );
}

function blockedDispatchResultTracksExecutionAudit(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchExecutionAuditState === AgentAppGameAdapterDispatchExecutionAuditState.BlockedBeforeExecutionAudit &&
    row.dispatchExecutionAuditDecision ===
      AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit &&
    row.dispatchExecutionAuditId === null &&
    row.dispatchExecutionAuditRefs.length === 0
  );
}

function blockedDispatchResultTracksAdapterExecution(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.dispatchAdapterExecutionState ===
      AgentAppGameAdapterDispatchAdapterExecutionState.BlockedBeforeAdapterExecution &&
    row.dispatchAdapterExecutionDecision ===
      AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution &&
    row.dispatchAdapterExecutionResultId === null &&
    row.dispatchAdapterExecutionStatus === null &&
    row.dispatchAdapterExecutionAdapterResultCode === null &&
    row.dispatchAdapterExecutionAuditEventId === null &&
    row.dispatchAdapterExecutionRefs.length === 0
  );
}

function blockedDispatchResultKeepsClaimsScoped(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  return (
    row.manualProofRequirements.length > 0 &&
    !row.adapterDispatchCommandResultClaimed &&
    !row.adapterDispatchExecutedClaimed &&
    !row.serviceLocalExecutionAuditClaimed
  );
}
