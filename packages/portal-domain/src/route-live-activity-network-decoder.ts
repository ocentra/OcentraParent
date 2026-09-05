import type { PortalLiveActivityState } from './live-activity-state';

type PortalNetworkFlowReadModel = NonNullable<PortalLiveActivityState['networkFlowReadModel']>;
type PortalNetworkFlowObservation = PortalNetworkFlowReadModel['rows'][number];

interface NetworkEndpointSnapshot {
  readonly ip?: string | null;
  readonly port?: number | null;
}

interface NetworkFlowCountersSnapshot {
  readonly connectionCount: number;
  readonly bytesSent?: number | null;
  readonly bytesReceived?: number | null;
  readonly firstSeenAt?: string | null;
  readonly lastSeenAt?: string | null;
}

interface NetworkEvidenceReferenceSnapshot {
  readonly evidenceId: string;
}

export interface PortalNetworkFlowObservationSnapshot {
  readonly eventId: string;
  readonly observedAt: string;
  readonly adapterId: string;
  readonly localEndpoint: NetworkEndpointSnapshot;
  readonly destinationEndpoint: NetworkEndpointSnapshot;
  readonly protocol?: string | null;
  readonly tcpState?: string | null;
  readonly processName?: string | null;
  readonly processId?: number | null;
  readonly processAttributionStatus: string;
  readonly destinationDomain?: string | null;
  readonly domainAttributionStatus: string;
  readonly capabilityStatus: string;
  readonly counters: NetworkFlowCountersSnapshot;
  readonly evidence: readonly NetworkEvidenceReferenceSnapshot[];
}

export interface PortalNetworkFlowReadModelSnapshot {
  readonly rows: readonly PortalNetworkFlowObservationSnapshot[];
  readonly capabilityStatus: string;
  readonly custody: string;
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly exportableRows: number;
  readonly latestTombstoneEventId?: string | null;
  readonly latestTombstoneObservedAt?: string | null;
  readonly deletedEvidenceReferenceIds: readonly string[];
}

export interface PortalNetworkRuntimeEventValueSnapshot {
  readonly aiAnalysisRef?: string | null;
  readonly policyDecisionRef?: string | null;
  readonly enforcementResultRef?: string | null;
}

export interface PortalNetworkRuntimeEventResultSnapshot {
  readonly ok: boolean;
  readonly reason?: string | null;
  readonly eventType?: string | null;
  readonly value?: PortalNetworkRuntimeEventValueSnapshot | null;
}

export interface PortalNetworkRuntimeEventChainStreamSnapshot {
  readonly streamedEventCount?: number | null;
  readonly events: readonly PortalNetworkRuntimeEventResultSnapshot[];
  readonly invalidEventCount: number;
}

export type DecodedPortalNetworkRuntimeEventResult =
  | {
      readonly ok: true;
      readonly eventType: string;
      readonly value: Readonly<Record<string, unknown>>;
    }
  | {
      readonly ok: false;
      readonly reason: string;
    };

export interface DecodedPortalNetworkRuntimeEventChainStream {
  readonly streamedEventCount: number | null;
  readonly events: readonly DecodedPortalNetworkRuntimeEventResult[];
  readonly invalidEventCount: number;
}

export function decodeNetworkFlowReadModel(
  value: PortalNetworkFlowReadModelSnapshot | null | undefined
): PortalNetworkFlowReadModel | null {
  if (value === null || value === undefined) return null;

  return {
    rows: value.rows.map(decodeNetworkFlowObservation),
    capabilityStatus: value.capabilityStatus,
    custody: value.custody,
    returned: value.returned,
    activeRows: value.activeRows,
    tombstoneRows: value.tombstoneRows,
    exportableRows: value.exportableRows,
    latestTombstoneEventId: value.latestTombstoneEventId ?? null,
    latestTombstoneObservedAt: value.latestTombstoneObservedAt ?? null,
    deletedEvidenceReferenceIds: value.deletedEvidenceReferenceIds,
  };
}

export function decodeNetworkRuntimeEventChainStream(
  value: PortalNetworkRuntimeEventChainStreamSnapshot | null | undefined
): DecodedPortalNetworkRuntimeEventChainStream | null {
  if (value === null || value === undefined) return null;
  const streamedEventCount = value.streamedEventCount ?? null;
  if (streamedEventCount !== null && streamedEventCount < 0) return null;

  const events: DecodedPortalNetworkRuntimeEventResult[] = [];
  for (const event of value.events) {
    const decoded = decodeNetworkRuntimeEvent(event);
    if (decoded === null) return null;
    events.push(decoded);
  }

  return { streamedEventCount, events, invalidEventCount: value.invalidEventCount };
}

function decodeNetworkFlowObservation(value: PortalNetworkFlowObservationSnapshot): PortalNetworkFlowObservation {
  return {
    eventId: value.eventId,
    observedAt: value.observedAt,
    adapterId: value.adapterId,
    localEndpoint: nullableEndpoint(value.localEndpoint),
    destinationEndpoint: nullableEndpoint(value.destinationEndpoint),
    protocol: value.protocol ?? null,
    tcpState: value.tcpState ?? null,
    processName: value.processName ?? null,
    processId: value.processId ?? null,
    processAttributionStatus: value.processAttributionStatus,
    destinationDomain: value.destinationDomain ?? null,
    domainAttributionStatus: value.domainAttributionStatus,
    capabilityStatus: value.capabilityStatus,
    counters: {
      connectionCount: value.counters.connectionCount,
      bytesSent: value.counters.bytesSent ?? null,
      bytesReceived: value.counters.bytesReceived ?? null,
      firstSeenAt: value.counters.firstSeenAt ?? null,
      lastSeenAt: value.counters.lastSeenAt ?? null,
    },
    evidence: value.evidence.map((evidence) => ({ evidenceId: evidence.evidenceId })),
  };
}

function nullableEndpoint(value: NetworkEndpointSnapshot): PortalNetworkFlowObservation['localEndpoint'] {
  return {
    ip: value.ip ?? null,
    port: value.port ?? null,
  };
}

function decodeNetworkRuntimeEvent(
  value: PortalNetworkRuntimeEventResultSnapshot
): DecodedPortalNetworkRuntimeEventResult | null {
  if (value.ok) return decodeSuccessfulNetworkRuntimeEvent(value);
  if (
    typeof value.reason !== 'string' ||
    value.reason.length === 0 ||
    (value.eventType !== null && value.eventType !== undefined) ||
    (value.value !== null && value.value !== undefined)
  ) {
    return null;
  }
  return { ok: false, reason: value.reason };
}

function decodeSuccessfulNetworkRuntimeEvent(
  value: PortalNetworkRuntimeEventResultSnapshot
): DecodedPortalNetworkRuntimeEventResult | null {
  if (
    typeof value.eventType !== 'string' ||
    value.eventType.length === 0 ||
    value.value === null ||
    value.value === undefined ||
    (value.reason !== null && value.reason !== undefined)
  ) {
    return null;
  }
  return {
    ok: true,
    eventType: value.eventType,
    value: decodeNetworkRuntimeEventValue(value.value),
  };
}

function decodeNetworkRuntimeEventValue(
  value: PortalNetworkRuntimeEventValueSnapshot
): Readonly<Record<string, unknown>> {
  return {
    ...(value.aiAnalysisRef === null || value.aiAnalysisRef === undefined
      ? {}
      : { aiAnalysisRef: value.aiAnalysisRef }),
    ...(value.policyDecisionRef === null || value.policyDecisionRef === undefined
      ? {}
      : { policyDecisionRef: value.policyDecisionRef }),
    ...(value.enforcementResultRef === null || value.enforcementResultRef === undefined
      ? {}
      : { enforcementResultRef: value.enforcementResultRef }),
  };
}
