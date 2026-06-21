import {
  AgentActivityTrackingReadModelSchema,
  type AgentActivityTrackingReadModel,
} from '@ocentra-parent/schema-domain/agent-tracking-read-model';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export type AgentActivityTrackingReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentActivityTrackingReadModelResult =
  | {
      readonly ok: true;
      readonly value: AgentActivityTrackingReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentActivityTrackingReadModelFailureReason;
    };

export function parseAgentActivityTrackingReadModelEvent(
  event: AgentEventEnvelope
): AgentActivityTrackingReadModelResult {
  if (event.event !== AgentEvent.ActivityTrackingReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityTrackingReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentActivityTrackingReadModelSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentActivityTrackingReadModelFailureReason): AgentActivityTrackingReadModelResult {
  return {
    ok: false,
    reason,
  };
}
