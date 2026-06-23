import {
  SocialParentNotificationDeliveryReadinessReadModelSchema,
  type SocialParentNotificationDeliveryReadinessReadModel,
  type SocialParentNotificationDeliveryReadinessRow,
} from '@ocentra-parent/schema-domain/social-parent-notification-delivery-readiness';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export { SocialParentNotificationDeliveryReadinessReadModelSchema as SocialParentNotificationDeliveryReadModelSnapshotSchema };
export type {
  SocialParentNotificationDeliveryReadinessReadModel as SocialParentNotificationDeliveryReadModelSnapshot,
  SocialParentNotificationDeliveryReadinessRow as SocialParentNotificationDeliveryReadModelRow,
};

export type AgentSocialParentNotificationDeliveryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialParentNotificationDeliveryReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialParentNotificationDeliveryReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialParentNotificationDeliveryReadModelFailureReason;
    };

export function parseAgentSocialParentNotificationDeliveryReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialParentNotificationDeliveryReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialParentNotificationDeliveryReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentSocialParentNotificationDeliveryReadModelFailureReason
): AgentSocialParentNotificationDeliveryReadModelResult {
  return {
    ok: false,
    reason,
  };
}
