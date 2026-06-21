import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  SocialAlertReportParentSurfaceReadModelSnapshotSchema,
  type SocialAlertReportParentSurfaceReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-alert-report-parent-surface-read-model';
import {
  SocialAlertReportReadModelSnapshotSchema,
  type SocialAlertReportReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-alert-report-read-model';
import {
  SocialDashboardUxSnapshotSchema,
  type SocialDashboardUxSnapshot,
} from '@ocentra-parent/schema-domain/social-dashboard-ux';
import {
  SocialParentNotificationDeliveryReadinessReadModelSchema,
  type SocialParentNotificationDeliveryReadinessReadModel,
} from '@ocentra-parent/schema-domain/social-parent-notification-delivery-readiness';

type SocialReadModelFailureReason = 'wrong-event' | 'missing-json-field' | 'invalid-json' | 'invalid-payload';
type SocialReadModelResult<T> =
  | {
      readonly ok: true;
      readonly value: T;
    }
  | {
      readonly ok: false;
      readonly reason: SocialReadModelFailureReason;
    };

type SafeParseSchema<T> = {
  safeParse(input: unknown): {
    readonly success: boolean;
    readonly data?: T;
  };
};

export type AgentSocialAlertReportReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialAlertReportReadModelResult = SocialReadModelResult<SocialAlertReportReadModelSnapshot>;

export type AgentSocialAlertReportParentSurfaceReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialAlertReportParentSurfaceReadModelResult =
  SocialReadModelResult<SocialAlertReportParentSurfaceReadModelSnapshot>;

export type SocialParentNotificationDeliveryReadModelSnapshot =
  SocialParentNotificationDeliveryReadinessReadModel;
export type AgentSocialParentNotificationDeliveryReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialParentNotificationDeliveryReadModelResult =
  SocialReadModelResult<SocialParentNotificationDeliveryReadModelSnapshot>;

export type AgentSocialDashboardReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialDashboardReadModelResult = SocialReadModelResult<SocialDashboardUxSnapshot>;

export function parseAgentSocialAlertReportReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialAlertReportReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel,
    SocialAlertReportReadModelSnapshotSchema
  );
}

export function parseAgentSocialAlertReportParentSurfaceReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportParentSurfaceReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel,
    SocialAlertReportParentSurfaceReadModelSnapshotSchema
  );
}

export function parseAgentSocialParentNotificationDeliveryReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialParentNotificationDeliveryReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel,
    SocialParentNotificationDeliveryReadinessReadModelSchema
  );
}

export function parseAgentSocialDashboardReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialDashboardReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialDashboardReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel,
    SocialDashboardUxSnapshotSchema
  );
}

function parseAgentReadModelEvent<T>(
  event: AgentEventEnvelope,
  expectedEvent: AgentEventEnvelope['event'],
  payloadField: string,
  schema: SafeParseSchema<T>
): SocialReadModelResult<T> {
  if (event.event !== expectedEvent) {
    return failure('wrong-event');
  }

  const raw = event.payload[payloadField];
  if (!isAgentProtocolLogText(raw)) {
    return failure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return failure('invalid-json');
  }

  const parsed = schema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return failure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function failure(reason: SocialReadModelFailureReason) {
  return {
    ok: false,
    reason,
  } as const;
}
