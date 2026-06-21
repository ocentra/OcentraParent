import {
  AppGameTimerParentPreferenceSetupRequestResultSchema,
  type AppGameTimerParentPreferenceSetupRequestResult,
} from '@ocentra-parent/schema-domain/app-game-timer-parent-preference-setup-request';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export type AgentAppGameTimerParentPreferenceSetupRequestFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameTimerParentPreferenceSetupRequestResult =
  | {
      readonly ok: true;
      readonly value: AppGameTimerParentPreferenceSetupRequestResult;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameTimerParentPreferenceSetupRequestFailureReason;
    };

export function parseAgentAppGameTimerParentPreferenceSetupRequestEvent(
  event: AgentEventEnvelope
): AgentAppGameTimerParentPreferenceSetupRequestResult {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested) {
    return requestFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest];
  if (!isAgentProtocolLogText(raw)) {
    return requestFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return requestFailure('invalid-json');
  }

  const parsed = AppGameTimerParentPreferenceSetupRequestResultSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return requestFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function requestFailure(
  reason: AgentAppGameTimerParentPreferenceSetupRequestFailureReason
): AgentAppGameTimerParentPreferenceSetupRequestResult {
  return {
    ok: false,
    reason,
  };
}
