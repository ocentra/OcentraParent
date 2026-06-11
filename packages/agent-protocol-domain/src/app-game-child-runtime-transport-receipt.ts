import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const ChildRuntimeTransportReceiptText = Schema.String.pipe(Schema.minLength(1));
const ChildRuntimeTransportReceiptCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameChildRuntimeTransportReceiptPayloadField =
  'appGameChildRuntimeTransportReceiptReadModel' as const;

export const AgentAppGameChildRuntimeTransportReceiptState = {
  TransportRequired: 'child-runtime-transport-required',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

const AgentAppGameChildRuntimeTransportReceiptRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  rowId: ChildRuntimeTransportReceiptText,
  sourceRuntimeWriterRowId: ChildRuntimeTransportReceiptText,
  boundaryState: Schema.Literal(
    AgentAppGameChildRuntimeTransportReceiptState.TransportRequired,
    AgentAppGameChildRuntimeTransportReceiptState.ManualRequired,
    AgentAppGameChildRuntimeTransportReceiptState.Unavailable
  ),
  productMeanings: Schema.Array(Schema.Literal('native-app', 'native-game')),
  requiredTransportRefs: Schema.Array(ChildRuntimeTransportReceiptText),
  requiredReceiptRefs: Schema.Array(ChildRuntimeTransportReceiptText),
  openGaps: Schema.Array(ChildRuntimeTransportReceiptText),
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
});

type AgentAppGameChildRuntimeTransportReceiptRowCandidate = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptRowBaseSchema
>;

export const AgentAppGameChildRuntimeTransportReceiptRowSchema = withParser(
  AgentAppGameChildRuntimeTransportReceiptRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGameChildRuntimeTransportReceiptRowCandidate) =>
        childRuntimeTransportReceiptRowIsHonest(row) ||
        'Expected app/game child runtime transport receipt rows to keep delivery and receipt execution unclaimed'
    )
  )
);

const AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  readModelId: ChildRuntimeTransportReceiptText,
  generatedAt: ChildRuntimeTransportReceiptText,
  sourceReadModelIds: Schema.Array(ChildRuntimeTransportReceiptText),
  custodyLabel: ChildRuntimeTransportReceiptText,
  capabilityStatus: ChildRuntimeTransportReceiptText,
  returned: ChildRuntimeTransportReceiptCount,
  transportRequiredCount: ChildRuntimeTransportReceiptCount,
  manualRequiredCount: ChildRuntimeTransportReceiptCount,
  unavailableCount: ChildRuntimeTransportReceiptCount,
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  rows: Schema.Array(AgentAppGameChildRuntimeTransportReceiptRowSchema),
});

type AgentAppGameChildRuntimeTransportReceiptReadModelCandidate = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema
>;
type AgentAppGameChildRuntimeTransportReceiptStateValue =
  (typeof AgentAppGameChildRuntimeTransportReceiptState)[keyof typeof AgentAppGameChildRuntimeTransportReceiptState];

export const AgentAppGameChildRuntimeTransportReceiptReadModelSchema = withParser(
  AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGameChildRuntimeTransportReceiptReadModelCandidate) =>
        childRuntimeTransportReceiptCountsMatch(readModel) ||
        'Expected app/game child runtime transport receipt counts to match status rows'
    )
  )
);

export type AgentAppGameChildRuntimeTransportReceiptRow = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptRowSchema
>;
export type AgentAppGameChildRuntimeTransportReceiptReadModel = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptReadModelSchema
>;

export type AgentAppGameChildRuntimeTransportReceiptFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameChildRuntimeTransportReceiptResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameChildRuntimeTransportReceiptReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameChildRuntimeTransportReceiptFailureReason;
    };

export function parseAgentAppGameChildRuntimeTransportReceiptEvent(
  event: AgentEventEnvelope
): AgentAppGameChildRuntimeTransportReceiptResult {
  if (event.event !== AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported) {
    return childRuntimeTransportReceiptFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGameChildRuntimeTransportReceiptPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return childRuntimeTransportReceiptFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return childRuntimeTransportReceiptFailure('invalid-json');
  }

  const parsed = AgentAppGameChildRuntimeTransportReceiptReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return childRuntimeTransportReceiptFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function childRuntimeTransportReceiptRowIsHonest(row: AgentAppGameChildRuntimeTransportReceiptRowCandidate): boolean {
  return (
    row.productMeanings.includes('native-app') &&
    row.productMeanings.includes('native-game') &&
    row.requiredTransportRefs.length > 0 &&
    row.requiredReceiptRefs.length > 0 &&
    row.openGaps.some((gap) => gap === 'child-runtime-transport-not-executed') &&
    !row.runtimeTransportExecuted &&
    !row.runtimeReceiptIngested &&
    !row.providerDeliveryExecuted &&
    !row.platformDeliveryChannelClaimed
  );
}

function childRuntimeTransportReceiptCountsMatch(
  readModel: AgentAppGameChildRuntimeTransportReceiptReadModelCandidate
): boolean {
  return (
    readModel.returned === readModel.rows.length &&
    readModel.transportRequiredCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.TransportRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.Unavailable) &&
    !readModel.runtimeTransportExecuted &&
    !readModel.runtimeReceiptIngested &&
    !readModel.providerDeliveryExecuted &&
    !readModel.platformDeliveryChannelClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: readonly AgentAppGameChildRuntimeTransportReceiptRowCandidate[],
  state: AgentAppGameChildRuntimeTransportReceiptStateValue
): number {
  return rows.filter((row) => row.boundaryState === state).length;
}

function childRuntimeTransportReceiptFailure(
  reason: AgentAppGameChildRuntimeTransportReceiptFailureReason
): AgentAppGameChildRuntimeTransportReceiptResult {
  return {
    ok: false,
    reason,
  };
}
