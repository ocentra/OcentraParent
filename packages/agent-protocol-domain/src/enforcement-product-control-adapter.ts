import {
  V08EnforcementProductControlSpineReadModelSchema,
  type V08EnforcementProductControlSpineReadModel,
} from '@ocentra-parent/enforcement-domain/v0-8-enforcement-product-control-spine';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from './contracts';

export type EnforcementProductControlParseResult =
  | {
      readonly status: 'accepted';
      readonly readModel: V08EnforcementProductControlSpineReadModel;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'unexpected-event' | 'missing-read-model' | 'invalid-read-model-json' | 'invalid-read-model';
    };

export function parseEnforcementProductControlSpineEvent(
  event: AgentEventEnvelope
): EnforcementProductControlParseResult {
  if (event.event !== AgentEvent.EnforcementProductControlSpineReported) {
    return { status: 'rejected', reason: 'unexpected-event' };
  }

  const rawReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel];
  if (typeof rawReadModel !== 'string' || rawReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-read-model' };
  }

  const decoded = parseJson(rawReadModel);
  if (decoded.status === 'rejected') {
    return decoded;
  }

  const parsed = V08EnforcementProductControlSpineReadModelSchema.safeParse(decoded.value);
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
