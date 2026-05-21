import {
  ActivityNetworkFlowReadModelSchema,
  type ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';

export function parseNetworkFlowReadModel(payload: AgentProtocolLogFields): ActivityNetworkFlowReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const row = returned === 0 ? [] : [networkFlowObservation(payload)];
  const parsed = ActivityNetworkFlowReadModelSchema.safeParse({
    schemaVersion: ActivityQuerySchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    custody: payload[AgentProtocolDefaults.Field.Custody],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned,
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    rows: row,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function networkFlowObservation(payload: AgentProtocolLogFields) {
  return {
    schemaVersion: ActivityQuerySchemaVersion,
    eventId: payload[AgentProtocolDefaults.Field.LatestEventId],
    observedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    observer: payload[AgentProtocolDefaults.Field.Observer],
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    adapterId: payload[AgentProtocolDefaults.Field.AdapterId],
    protocol: nullIfMissing(payload[AgentProtocolDefaults.Field.NetworkProtocol]),
    tcpState: nullIfMissing(payload[AgentProtocolDefaults.Field.TcpState]),
    localEndpoint: {
      ip: nullIfMissing(payload[AgentProtocolDefaults.Field.LocalIp]),
      port: nullIfMissing(payload[AgentProtocolDefaults.Field.LocalPort]),
    },
    destinationEndpoint: {
      ip: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationIp]),
      port: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationPort]),
    },
    destinationDomain: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationDomain]),
    domainAttributionStatus: payload[AgentProtocolDefaults.Field.DomainAttributionStatus],
    processAttributionStatus: payload[AgentProtocolDefaults.Field.ProcessAttributionStatus],
    processId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessId]),
    processName: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessName]),
    counters: {
      connectionCount: payload[AgentProtocolDefaults.Field.ConnectionCount],
      bytesSent: nullIfMissing(payload[AgentProtocolDefaults.Field.BytesSent]),
      bytesReceived: nullIfMissing(payload[AgentProtocolDefaults.Field.BytesReceived]),
      firstSeenAt: nullIfMissing(payload[AgentProtocolDefaults.Field.FirstSeenAt]),
      lastSeenAt: nullIfMissing(payload[AgentProtocolDefaults.Field.LastSeenAt]),
    },
    evidence: [],
  };
}

function nullIfMissing(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  return value === undefined ? null : value;
}
