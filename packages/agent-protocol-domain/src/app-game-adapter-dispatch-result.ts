import {
  AgentAppGameAdapterDispatchExecuteResultPayloadField,
  AgentAppGameAdapterDispatchExecuteResultSchema,
  AgentAppGameAdapterDispatchResultPayloadField,
  AgentAppGameAdapterDispatchResultReadModelSchema,
  type AgentAppGameAdapterDispatchExecuteResult,
  type AgentAppGameAdapterDispatchResultReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-result';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';

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
