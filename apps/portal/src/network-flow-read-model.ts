import {
  ActivityNetworkFlowDigestSchema,
  ActivityNetworkFlowReadModelSchema,
  type ActivityNetworkFlowDigest,
  type ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import type { ActivityEvidenceRef } from '@ocentra-parent/activity-domain/contracts';
import { decodeActivityEvidenceId, type ActivityEvidenceId } from '@ocentra-parent/activity-domain/primitives';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';

export function parseNetworkFlowReadModel(payload: AgentProtocolLogFields): ActivityNetworkFlowReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const digest = networkFlowDigest(payload);
  const row = returned === 0 ? [] : [networkFlowObservation(payload, digest)];
  const parsed = ActivityNetworkFlowReadModelSchema.safeParse({
    schemaVersion: ActivityQuerySchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    custody: payload[AgentProtocolDefaults.Field.Custody],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned,
    activeRows: payload[AgentProtocolDefaults.Field.ActiveRows],
    tombstoneRows: payload[AgentProtocolDefaults.Field.TombstoneRows],
    exportableRows: payload[AgentProtocolDefaults.Field.ExportableRows],
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    latestEventId: nullIfMissing(payload[AgentProtocolDefaults.Field.LatestEventId]),
    latestObservedAt: nullIfMissing(payload[AgentProtocolDefaults.Field.LatestObservedAt]),
    latestTombstoneEventId: nullIfMissing(payload[AgentProtocolDefaults.Field.LatestTombstoneEventId]),
    latestTombstoneObservedAt: nullIfMissing(payload[AgentProtocolDefaults.Field.LatestTombstoneObservedAt]),
    deletedEvidenceReferenceIds: evidenceReferenceIds(payload[AgentProtocolDefaults.Field.DeletedEvidenceReferenceIds]),
    rows: row,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function networkFlowObservation(payload: AgentProtocolLogFields, digest: ActivityNetworkFlowDigest | null) {
  const evidence = digest?.evidence ?? [];
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
    evidence,
  };
}

function nullIfMissing(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  if (value === undefined) {
    return null;
  }
  if (typeof value === AgentProtocolDefaults.Primitive.String && value.trim().length === 0) {
    return null;
  }
  return value;
}

function networkFlowDigest(payload: AgentProtocolLogFields): ActivityNetworkFlowDigest | null {
  const raw = payload[AgentProtocolDefaults.Field.ActivityDigest];
  if (typeof raw !== AgentProtocolDefaults.Primitive.String) {
    return null;
  }

  try {
    const decoded = JSON.parse(String(raw)) as unknown;
    const parsed = ActivityNetworkFlowDigestSchema.safeParse(decoded);
    if (!parsed.success) {
      return null;
    }
    return normalizeDigestEvidence(parsed.data);
  } catch {
    return null;
  }
}

function normalizeDigestEvidence(digest: ActivityNetworkFlowDigest): ActivityNetworkFlowDigest {
  const evidence = digest.evidence.filter(isUsableEvidence);
  return {
    ...digest,
    evidence,
  };
}

function isUsableEvidence(evidence: ActivityEvidenceRef): boolean {
  return evidence.evidenceId.length > 0;
}

function evidenceReferenceIds(
  value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined
): ActivityEvidenceId[] {
  if (typeof value !== AgentProtocolDefaults.Primitive.String) {
    return [];
  }
  return String(value)
    .split(AgentProtocolDefaults.Delimiter.List)
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0)
    .map((entry) => decodeActivityEvidenceId(entry));
}
