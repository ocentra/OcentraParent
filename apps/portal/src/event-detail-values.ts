import type { AgentEventEnvelope, AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalReadableValues,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';

export type AgentPayloadField = (typeof AgentProtocolDefaults.Field)[keyof typeof AgentProtocolDefaults.Field];

export function eventStatus(event: AgentEventEnvelope | null): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return decodePortalDetailValue(event.severity);
}

export function payloadDetail(event: AgentEventEnvelope | null, field: AgentPayloadField): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return detailFromValue(event.payload[field]);
}

export function detailFromValue(value: LogFieldValue | undefined): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  const readableValue = PortalReadableValues[String(value)];
  if (readableValue !== undefined) {
    return decodePortalDetailValue(readableValue);
  }
  return decodePortalDetailValue(String(value));
}

export function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
