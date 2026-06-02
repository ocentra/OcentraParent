import {
  V08SupportedAdapterRuntimeProofReadModelSchema,
  type V08SupportedAdapterRuntimeProofReadModel,
} from '@ocentra-parent/parent-domain/v0-8-supported-adapter-runtime-proof';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from './contracts';

export type EnforcementSupportedAdapterRuntimeProofParseResult =
  | {
      readonly status: 'accepted';
      readonly readModel: V08SupportedAdapterRuntimeProofReadModel;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'unexpected-event' | 'missing-read-model' | 'invalid-read-model-json' | 'invalid-read-model';
    };

export function parseEnforcementSupportedAdapterRuntimeProofEvent(
  event: AgentEventEnvelope
): EnforcementSupportedAdapterRuntimeProofParseResult {
  if (event.event !== AgentEvent.EnforcementSupportedAdapterRuntimeProofReported) {
    return { status: 'rejected', reason: 'unexpected-event' };
  }

  const rawReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel];
  if (typeof rawReadModel !== 'string' || rawReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-read-model' };
  }

  const decoded = parseJson(rawReadModel);
  if (decoded.status === 'rejected') {
    return decoded;
  }

  const parsed = V08SupportedAdapterRuntimeProofReadModelSchema.safeParse(decoded.value);
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
