import { AgentEvent, isAgentProtocolLogText } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
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
import type { PortalRouteEventRecord } from './portal-contract-adapter';

const SocialReadModelFailureReason = {
  WrongEvent: 'wrong-event',
  MissingJsonField: 'missing-json-field',
  InvalidJson: 'invalid-json',
  InvalidPayload: 'invalid-payload',
} as const;

export type SocialReadModelFailureReason =
  (typeof SocialReadModelFailureReason)[keyof typeof SocialReadModelFailureReason];

export type SocialReadModelResult<T> =
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

export type SocialParentNotificationDeliveryReadModelSnapshot = SocialParentNotificationDeliveryReadinessReadModel;
export type AgentSocialParentNotificationDeliveryReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialParentNotificationDeliveryReadModelResult =
  SocialReadModelResult<SocialParentNotificationDeliveryReadModelSnapshot>;

export type AgentSocialDashboardReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialDashboardReadModelResult = SocialReadModelResult<SocialDashboardUxSnapshot>;

export function parseAgentSocialAlertReportReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialAlertReportReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialAlertReportReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel,
    SocialAlertReportReadModelSnapshotSchema
  );
}

export function parseAgentSocialAlertReportParentSurfaceReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialAlertReportParentSurfaceReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel,
    SocialAlertReportParentSurfaceReadModelSnapshotSchema
  );
}

export function parseAgentSocialParentNotificationDeliveryReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialParentNotificationDeliveryReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel,
    SocialParentNotificationDeliveryReadinessReadModelSchema
  );
}

export function parseAgentSocialDashboardReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialDashboardReadModelResult {
  return parseAgentReadModelEvent(
    event,
    AgentEvent.BrowserSocialDashboardReadModelReported,
    AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel,
    SocialDashboardUxSnapshotSchema
  );
}

function parseAgentReadModelEvent<T>(
  event: PortalRouteEventRecord,
  expectedEvent: PortalRouteEventRecord['event'],
  payloadField: string,
  schema: SafeParseSchema<T>
): SocialReadModelResult<T> {
  if (event.event !== expectedEvent) {
    return failure(SocialReadModelFailureReason.WrongEvent);
  }

  const raw = event.payload?.[payloadField];
  if (!isAgentProtocolLogText(raw)) {
    return failure(SocialReadModelFailureReason.MissingJsonField);
  }

  const decoded = parseJson(raw);
  if (decoded === null) {
    return failure(SocialReadModelFailureReason.InvalidJson);
  }

  const parsed = schema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return failure(SocialReadModelFailureReason.InvalidPayload);
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function parseJson(raw: string): unknown | null {
  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

function failure(reason: SocialReadModelFailureReason): SocialReadModelResult<never> {
  return {
    ok: false,
    reason,
  };
}
