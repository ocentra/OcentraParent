import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from './app-game-adapter-dispatch-preflight';
import { AgentCommand, AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const DispatchResultCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

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
  enforcementCommandName: Schema.Union(Schema.Literal(AgentCommand.EnforcementExecute), Schema.Null),
  enforcementEventName: Schema.Union(Schema.Literal(AgentEvent.EnforcementAuditReported), Schema.Null),
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
      // eslint-disable-next-line complexity -- dispatch-result read-model honesty checks enumerate required counters.
      (readModel: AgentAppGameAdapterDispatchResultReadModelCandidate) =>
        (readModel.returned === readModel.rows.length &&
          readModel.commandAcceptedCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted
            ).length &&
          readModel.blockedBeforeCommandCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchCommandResultDecision ===
                AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand
            ).length &&
          readModel.executionAuditRecordedCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchExecutionAuditDecision ===
                AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded
            ).length &&
          readModel.blockedBeforeExecutionAuditCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchExecutionAuditDecision ===
                AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit
            ).length &&
          readModel.adapterExecutionReportedCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchAdapterExecutionDecision ===
                AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported
            ).length &&
          readModel.adapterExecutionEvidenceMissingCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchAdapterExecutionDecision ===
                AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionEvidenceMissing
            ).length &&
          readModel.blockedBeforeAdapterExecutionCount ===
            readModel.rows.filter(
              (row) =>
                row.dispatchAdapterExecutionDecision ===
                AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution
            ).length &&
          readModel.adapterDispatchCommandResultClaimedCount ===
            readModel.rows.filter((row) => row.adapterDispatchCommandResultClaimed).length &&
          readModel.serviceLocalExecutionAuditClaimedCount ===
            readModel.rows.filter((row) => row.serviceLocalExecutionAuditClaimed).length &&
          readModel.adapterDispatchExecutedClaimedCount ===
            readModel.rows.filter((row) => row.adapterDispatchExecutedClaimed).length &&
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length) ||
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
  executionCommandName: Schema.Literal(AgentCommand.EnforcementExecute),
  executionEventName: Schema.Literal(AgentEvent.EnforcementAuditReported),
  executionResultId: NonEmptyStringSchema,
  executionStatus: NonEmptyStringSchema,
  executionAdapterResultCode: NonEmptyStringSchema,
  executionAuditEventId: NonEmptyStringSchema,
  readbackCommandName: Schema.Literal(AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet),
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

export type AgentAppGameAdapterDispatchResultFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameAdapterDispatchResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameAdapterDispatchResultReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameAdapterDispatchResultFailureReason;
    };

export type AgentAppGameAdapterDispatchExecute =
  | {
      readonly ok: true;
      readonly value: AgentAppGameAdapterDispatchExecuteResult;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameAdapterDispatchResultFailureReason;
    };

export function parseAgentAppGameAdapterDispatchResultEvent(
  event: AgentEventEnvelope
): AgentAppGameAdapterDispatchResult {
  if (event.event !== AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported) {
    return dispatchResultFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGameAdapterDispatchResultPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return dispatchResultFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return dispatchResultFailure('invalid-json');
  }

  const parsed = AgentAppGameAdapterDispatchResultReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return dispatchResultFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

export function parseAgentAppGameAdapterDispatchExecuteEvent(
  event: AgentEventEnvelope
): AgentAppGameAdapterDispatchExecute {
  if (event.event !== AgentEvent.ActivityAppGameAdapterDispatchExecuted) {
    return dispatchExecuteFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGameAdapterDispatchExecuteResultPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return dispatchExecuteFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return dispatchExecuteFailure('invalid-json');
  }

  const parsed = AgentAppGameAdapterDispatchExecuteResultSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return dispatchExecuteFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function dispatchResultRowIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  if (row.dispatchCommandResultState === AgentAppGameAdapterDispatchCommandResultState.CommandAccepted) {
    return (
      row.platform === 'windows' &&
      row.sourceProofEntryId === 'windows-app-game-owned-process-time-limit' &&
      row.dispatchPreflightState === AgentAppGameAdapterDispatchPreflightState.DispatchEligible &&
      row.dispatchDecision === AgentAppGameAdapterDispatchDecision.DispatchEligible &&
      row.dispatchOutcomeState === AgentAppGameAdapterDispatchOutcomeState.DispatchReady &&
      row.dispatchIntentId !== null &&
      row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted &&
      row.enforcementCommandName === AgentCommand.EnforcementExecute &&
      row.enforcementEventName === AgentEvent.EnforcementAuditReported &&
      row.enforcementActionMode === 'terminate-process' &&
      row.dispatchCommandResultId !== null &&
      row.dispatchCommandAuditRefs.length > 0 &&
      row.dispatchCommandTimerRefs.length > 0 &&
      row.dispatchExecutionAuditState === AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded &&
      row.dispatchExecutionAuditDecision ===
        AgentAppGameAdapterDispatchExecutionAuditDecision.ServiceLocalAuditRecorded &&
      row.dispatchExecutionAuditId !== null &&
      row.dispatchExecutionAuditRefs.length > 0 &&
      row.manualProofRequirements.length === 0 &&
      row.adapterDispatchCommandResultClaimed &&
      row.serviceLocalExecutionAuditClaimed &&
      acceptedExecutionEvidenceIsHonest(row)
    );
  }

  return (
    row.dispatchCommandResultDecision === AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand &&
    row.enforcementCommandName === null &&
    row.enforcementEventName === null &&
    row.enforcementActionMode === null &&
    row.dispatchCommandResultId === null &&
    row.dispatchExecutionAuditState === AgentAppGameAdapterDispatchExecutionAuditState.BlockedBeforeExecutionAudit &&
    row.dispatchExecutionAuditDecision ===
      AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit &&
    row.dispatchExecutionAuditId === null &&
    row.dispatchExecutionAuditRefs.length === 0 &&
    row.dispatchAdapterExecutionState ===
      AgentAppGameAdapterDispatchAdapterExecutionState.BlockedBeforeAdapterExecution &&
    row.dispatchAdapterExecutionDecision ===
      AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution &&
    row.dispatchAdapterExecutionResultId === null &&
    row.dispatchAdapterExecutionStatus === null &&
    row.dispatchAdapterExecutionAdapterResultCode === null &&
    row.dispatchAdapterExecutionAuditEventId === null &&
    row.dispatchAdapterExecutionRefs.length === 0 &&
    row.manualProofRequirements.length > 0 &&
    !row.adapterDispatchCommandResultClaimed &&
    !row.adapterDispatchExecutedClaimed &&
    !row.serviceLocalExecutionAuditClaimed
  );
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function acceptedExecutionEvidenceIsHonest(row: AgentAppGameAdapterDispatchResultRowCandidate): boolean {
  if (
    row.dispatchAdapterExecutionDecision ===
    AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionReported
  ) {
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

function dispatchResultFailure(
  reason: AgentAppGameAdapterDispatchResultFailureReason
): AgentAppGameAdapterDispatchResult {
  return {
    ok: false,
    reason,
  };
}

function dispatchExecuteFailure(
  reason: AgentAppGameAdapterDispatchResultFailureReason
): AgentAppGameAdapterDispatchExecute {
  return {
    ok: false,
    reason,
  };
}
