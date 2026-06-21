import {
  AgentAppGamePolicyReadinessReadModelSchema,
  type AgentAppGamePolicyReadinessReadModel,
} from '@ocentra-parent/schema-domain/app-game-policy-readiness';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export type AgentAppGamePolicyReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePolicyReadinessResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGamePolicyReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePolicyReadinessFailureReason;
    };

export function parseAgentAppGamePolicyReadinessEvent(
  event: AgentEventEnvelope
): AgentAppGamePolicyReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGamePolicyReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGamePolicyReadinessReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGamePolicyReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentAppGamePolicyReadinessFailureReason): AgentAppGamePolicyReadinessResult {
  return {
    ok: false,
    reason,
  };
}
