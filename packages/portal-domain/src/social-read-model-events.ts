import { PortalAgentEvent, type PortalRouteEventRecord } from './portal-contract-adapter';
import {
  type GeneratedPortalAgentActivitySurfaceAdapterFailureReason,
  GeneratedPortalSocialReadModelPayloadField,
  type GeneratedPortalSocialAlertReportParentSurfaceReadModelSnapshot,
  type GeneratedPortalSocialAlertReportReadModelSnapshot,
  type GeneratedPortalSocialDashboardUxSnapshot,
  type GeneratedPortalSocialParentNotificationDeliveryReadModelSnapshot,
  type GeneratedPortalSocialReadModelPayloadFieldName,
} from './generated-portal-contracts';
import type { ReadModelResult } from './read-model-result';

export type SocialReadModelFailureReason = GeneratedPortalAgentActivitySurfaceAdapterFailureReason;

export type SocialReadModelResult<T> = ReadModelResult<T, SocialReadModelFailureReason>;

type JsonPayloadParseResult =
  | {
      readonly state: 'parsed';
      readonly value: unknown;
    }
  | {
      readonly state: 'invalid-json';
    }
  | {
      readonly state: 'missing-json-field';
    };

type JsonParseResult =
  | {
      readonly ok: true;
      readonly value: unknown;
    }
  | {
      readonly ok: false;
    };

export type SocialAlertReportReadModelSnapshot = GeneratedPortalSocialAlertReportReadModelSnapshot;
export type SocialAlertReportParentSurfaceReadModelSnapshot =
  GeneratedPortalSocialAlertReportParentSurfaceReadModelSnapshot;
export type SocialParentNotificationDeliveryReadModelSnapshot =
  GeneratedPortalSocialParentNotificationDeliveryReadModelSnapshot;
export type SocialDashboardUxSnapshot = GeneratedPortalSocialDashboardUxSnapshot;

export type AgentSocialAlertReportReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialAlertReportReadModelResult = SocialReadModelResult<SocialAlertReportReadModelSnapshot>;

export type AgentSocialAlertReportParentSurfaceReadModelFailureReason = SocialReadModelFailureReason;
export type AgentSocialAlertReportParentSurfaceReadModelResult =
  SocialReadModelResult<SocialAlertReportParentSurfaceReadModelSnapshot>;

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
    PortalAgentEvent.BrowserSocialAlertReportReadModelReported,
    GeneratedPortalSocialReadModelPayloadField.AlertReport
  );
}

export function parseAgentSocialAlertReportParentSurfaceReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialAlertReportParentSurfaceReadModelResult {
  return parseAgentReadModelEvent(
    event,
    PortalAgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
    GeneratedPortalSocialReadModelPayloadField.AlertReportParentSurface
  );
}

export function parseAgentSocialParentNotificationDeliveryReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialParentNotificationDeliveryReadModelResult {
  return parseAgentReadModelEvent(
    event,
    PortalAgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
    GeneratedPortalSocialReadModelPayloadField.ParentNotificationDelivery
  );
}

export function parseAgentSocialDashboardReadModelEvent(
  event: PortalRouteEventRecord
): AgentSocialDashboardReadModelResult {
  return parseAgentReadModelEvent(
    event,
    PortalAgentEvent.BrowserSocialDashboardReadModelReported,
    GeneratedPortalSocialReadModelPayloadField.Dashboard
  );
}

function parseAgentReadModelEvent<T extends Readonly<Record<string, unknown>>>(
  event: PortalRouteEventRecord,
  expectedEvent: PortalRouteEventRecord['event'],
  payloadField: GeneratedPortalSocialReadModelPayloadFieldName
): SocialReadModelResult<T> {
  if (event.event !== expectedEvent) {
    return failure('wrong-event');
  }

  const decoded = parseJsonPayloadField(event.payload, payloadField);
  if (decoded.state === 'missing-json-field') {
    return failure('missing-json-field');
  }
  if (decoded.state === 'invalid-json') {
    return failure('invalid-json');
  }

  if (!isRecord(decoded.value)) {
    return failure('invalid-payload');
  }

  return {
    ok: true,
    value: decoded.value as T,
  };
}

function parseJsonPayloadField(
  payload: PortalRouteEventRecord['payload'],
  payloadField: GeneratedPortalSocialReadModelPayloadFieldName
): JsonPayloadParseResult {
  if (payload === undefined) {
    return { state: 'missing-json-field' };
  }

  const value = payload[payloadField];
  if (typeof value !== 'string') {
    return { state: 'missing-json-field' };
  }

  const parsed = parseJson(value);
  return parsed.ok
    ? {
        state: 'parsed',
        value: parsed.value,
      }
    : { state: 'invalid-json' };
}

function parseJson(raw: string): JsonParseResult {
  try {
    return {
      ok: true,
      value: JSON.parse(raw),
    };
  } catch {
    return { ok: false };
  }
}

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function failure(reason: SocialReadModelFailureReason): SocialReadModelResult<never> {
  return {
    ok: false,
    reason,
  };
}
