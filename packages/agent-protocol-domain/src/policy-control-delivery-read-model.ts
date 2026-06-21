import {
  PolicyControlDeliveryReadModelPayloadField,
  PolicyControlDeliveryReadModelReportedEventName,
  PolicyControlDeliveryReadModelSnapshotSchema,
  type PolicyControlDeliveryReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-policy-control-delivery-read-model';
export type AgentPolicyControlDeliveryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentPolicyControlDeliveryReadModelResult =
  | {
      readonly ok: true;
      readonly value: PolicyControlDeliveryReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentPolicyControlDeliveryReadModelFailureReason;
    };

export function parseAgentPolicyControlDeliveryReadModelEvent(event: {
  readonly event: string;
  readonly payload: Record<string, unknown>;
}): AgentPolicyControlDeliveryReadModelResult {
  if (event.event !== PolicyControlDeliveryReadModelReportedEventName) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[PolicyControlDeliveryReadModelPayloadField];
  if (typeof raw !== 'string' || raw.trim().length === 0) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = PolicyControlDeliveryReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentPolicyControlDeliveryReadModelFailureReason): AgentPolicyControlDeliveryReadModelResult {
  return {
    ok: false,
    reason,
  };
}
