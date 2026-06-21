import {
  AgentAppGameNotificationReadinessReadModelSchema,
  type AgentAppGameNotificationReadinessReadModel,
} from '@ocentra-parent/schema-domain/app-game-notification-readiness';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export type AgentAppGameNotificationReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameNotificationReadinessResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameNotificationReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameNotificationReadinessFailureReason;
    };

export function parseAgentAppGameNotificationReadinessEvent(
  event: AgentEventEnvelope
): AgentAppGameNotificationReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGameNotificationReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameNotificationReadinessReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGameNotificationReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentAppGameNotificationReadinessFailureReason
): AgentAppGameNotificationReadinessResult {
  return {
    ok: false,
    reason,
  };
}
