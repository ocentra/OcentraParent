import {
  EnforcementPolicyDispatchReadModelSchema,
  type EnforcementPolicyDispatchReadModel,
} from '@ocentra-parent/schema-domain/enforcement-policy-dispatch';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type EnforcementPolicyDispatchParseResult =
  | {
      readonly status: 'accepted';
      readonly readModel: EnforcementPolicyDispatchReadModel;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'unexpected-event' | 'missing-read-model' | 'invalid-read-model-json' | 'invalid-read-model';
    };

export function parseEnforcementPolicyDispatchEvent(event: AgentEventEnvelope): EnforcementPolicyDispatchParseResult {
  if (event.event !== AgentEvent.EnforcementPolicyDispatchReported) {
    return { status: 'rejected', reason: 'unexpected-event' };
  }

  const rawReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementPolicyDispatchReadModel];
  if (typeof rawReadModel !== 'string' || rawReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-read-model' };
  }

  const decoded = parseJson(rawReadModel);
  if (decoded.status === 'rejected') {
    return decoded;
  }

  const parsed = EnforcementPolicyDispatchReadModelSchema.safeParse(decoded.value);
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
