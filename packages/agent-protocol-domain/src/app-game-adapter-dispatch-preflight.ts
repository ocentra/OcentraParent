import {
  AgentAppGameAdapterDispatchPreflightPayloadField,
  AgentAppGameAdapterDispatchPreflightReadModelSchema,
  type AgentAppGameAdapterDispatchPreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-dispatch-preflight';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

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

function dispatchPreflightFailure(
  reason: AgentAppGameAdapterDispatchPreflightFailureReason
): AgentAppGameAdapterDispatchPreflightResult {
  return {
    ok: false,
    reason,
  };
}
