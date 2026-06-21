import {
  AppGameAdapterExecutionReadinessReadModelSchema,
  type AppGameAdapterExecutionReadinessReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-execution-readiness';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const AppGameAdapterExecutionReadinessPayloadField = 'appGameAdapterExecutionReadinessReadModel' as const;

export type AgentAppGameAdapterExecutionReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameAdapterExecutionReadinessResult =
  | {
      readonly ok: true;
      readonly value: AppGameAdapterExecutionReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameAdapterExecutionReadinessFailureReason;
    };

export function parseAgentAppGameAdapterExecutionReadinessEvent(
  event: AgentEventEnvelope
): AgentAppGameAdapterExecutionReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGameAdapterExecutionReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AppGameAdapterExecutionReadinessPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AppGameAdapterExecutionReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentAppGameAdapterExecutionReadinessFailureReason
): AgentAppGameAdapterExecutionReadinessResult {
  return {
    ok: false,
    reason,
  };
}
