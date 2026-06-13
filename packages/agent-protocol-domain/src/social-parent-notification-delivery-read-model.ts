import {
  SocialParentNotificationDeliveryReadinessReadModelSchema,
  type SocialParentNotificationDeliveryReadinessReadModel,
  type SocialParentNotificationDeliveryReadinessRow,
} from '@ocentra-parent/browser-domain/social-parent-notification-delivery-readiness';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export const SocialParentNotificationDeliveryReadModelSnapshotSchema =
  SocialParentNotificationDeliveryReadinessReadModelSchema;

export type SocialParentNotificationDeliveryReadModelSnapshot =
  SocialParentNotificationDeliveryReadinessReadModel;
export type SocialParentNotificationDeliveryReadModelRow = SocialParentNotificationDeliveryReadinessRow;

export type AgentSocialParentNotificationDeliveryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialParentNotificationDeliveryReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialParentNotificationDeliveryReadModelSnapshot;
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

  const parsed = SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse(decoded);
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
