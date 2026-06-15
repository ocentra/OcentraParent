import {
  V08BroadOsAdapterRuntimeProofReadModelSchema,
  type V08BroadOsAdapterRuntimeProofReadModel,
} from '@ocentra-parent/enforcement-domain/v0-8-broad-os-adapter-runtime-proof';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from './contracts';

export type EnforcementBroadAdapterProofParseResult =
  | {
      readonly status: 'accepted';
      readonly readModel: V08BroadOsAdapterRuntimeProofReadModel;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'unexpected-event' | 'missing-read-model' | 'invalid-read-model-json' | 'invalid-read-model';
    };

export function parseEnforcementBroadAdapterProofEvent(
  event: AgentEventEnvelope
): EnforcementBroadAdapterProofParseResult {
  if (event.event !== AgentEvent.EnforcementBroadAdapterProofReported) {
    return { status: 'rejected', reason: 'unexpected-event' };
  }

  const rawReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementBroadAdapterProofReadModel];
  if (typeof rawReadModel !== 'string' || rawReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-read-model' };
  }

  const decoded = parseJson(rawReadModel);
  if (decoded.status === 'rejected') {
    return decoded;
  }

  const parsed = V08BroadOsAdapterRuntimeProofReadModelSchema.safeParse(decoded.value);
  if (!parsed.success) {
    return { status: 'rejected', reason: 'invalid-read-model' };
  }

  return { status: 'accepted', readModel: parsed.data };
}

function parseJson(value: string):
  | {
      readonly status: 'accepted';
      readonly value: unknown;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'invalid-read-model-json';
    } {
  try {
    return { status: 'accepted', value: JSON.parse(value) as unknown };
  } catch {
    return { status: 'rejected', reason: 'invalid-read-model-json' };
  }
}
