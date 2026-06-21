import {
  AppGamePlatformProofStatusReadModelSchema,
  type AppGamePlatformProofStatusReadModel,
} from '@ocentra-parent/schema-domain/app-game-platform-proof-status';
import { AgentProtocolDefaults } from './defaults';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export type AgentAppGamePlatformProofStatusFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePlatformProofStatusResult =
  | {
      readonly ok: true;
      readonly value: AppGamePlatformProofStatusReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePlatformProofStatusFailureReason;
    };

export function parseAgentAppGamePlatformProofStatusEvent(
  event: AgentEventEnvelope
): AgentAppGamePlatformProofStatusResult {
  if (event.event !== AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported) {
    return platformStatusFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGamePlatformProofStatusReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return platformStatusFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return platformStatusFailure('invalid-json');
  }

  const parsed = AppGamePlatformProofStatusReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return platformStatusFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function platformStatusFailure(
  reason: AgentAppGamePlatformProofStatusFailureReason
): AgentAppGamePlatformProofStatusResult {
  return {
    ok: false,
    reason,
  };
}
