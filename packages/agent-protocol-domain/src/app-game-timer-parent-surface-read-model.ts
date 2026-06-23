import {
  AgentAppGameTimerParentSurfaceReadModelSchema,
  type AgentAppGameTimerParentSurfaceReadModel,
} from '@ocentra-parent/schema-domain/app-game-timer-parent-surface-read-model';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

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
