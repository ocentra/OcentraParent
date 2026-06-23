import {
  AgentAppGameChildRuntimeTransportReceiptPayloadField,
  AgentAppGameChildRuntimeTransportReceiptReadModelSchema,
  type AgentAppGameChildRuntimeTransportReceiptReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-runtime-transport-receipt';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';

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

function childRuntimeTransportReceiptFailure(
  reason: AgentAppGameChildRuntimeTransportReceiptFailureReason
): AgentAppGameChildRuntimeTransportReceiptResult {
  return {
    ok: false,
    reason,
  };
}
