import { AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import type { LogFieldValue } from '@ocentra-parent/schema-domain/logging-contracts';
import { PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { PortalReadableValues } from '@ocentra-parent/portal-domain/details';
import { decodePortalDetailValue, type PortalDetailValue } from '@ocentra-parent/portal-domain/detail-values';

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
