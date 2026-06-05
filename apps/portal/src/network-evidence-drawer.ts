import type {
  ActivityNetworkEndpoint,
  ActivityNetworkFlowObservation,
  ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';

export type NetworkEvidenceDrawerSummary = {
  readonly evidenceId: PortalDetailValue;
  readonly observedAt: PortalDetailValue;
  readonly firstSeenAt: PortalDetailValue;
  readonly lastSeenAt: PortalDetailValue;
  readonly deviceRef: PortalDetailValue;
  readonly childProfileRef: PortalDetailValue;
  readonly sourceAdapter: PortalDetailValue;
  readonly sourceQuality: PortalDetailValue;
  readonly localEndpoint: PortalDetailValue;
  readonly remoteEndpoint: PortalDetailValue;
  readonly protocolCandidate: PortalDetailValue;
  readonly applicationProtocolCandidate: PortalDetailValue;
  readonly processRef: PortalDetailValue;
  readonly browserRef: PortalDetailValue;
  readonly domainEvidenceRef: PortalDetailValue;
  readonly byteSummary: PortalDetailValue;
  readonly analyzerAlertRef: PortalDetailValue;
  readonly detectionResultRef: PortalDetailValue;
  readonly aiAuditRef: PortalDetailValue;
  readonly riskBudgetRef: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly interventionResultRef: PortalDetailValue;
  readonly eventHistoryRef: PortalDetailValue;
  readonly retentionState: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly evidenceGrade: PortalDetailValue;
  readonly confidence: PortalDetailValue;
  readonly uncertaintyReasonCodes: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly exactUrlClaim: PortalDetailValue;
};

export function networkEvidenceDrawerSummary(
  readModel: ActivityNetworkFlowReadModel | null
): NetworkEvidenceDrawerSummary {
  const row = readModel?.rows[0] ?? null;
  return {
    evidenceId: detailFromValue(row?.eventId),
    observedAt: detailFromValue(row?.observedAt),
    firstSeenAt: detailFromValue(row?.counters.firstSeenAt),
    lastSeenAt: detailFromValue(row?.counters.lastSeenAt),
    deviceRef: notReported(),
    childProfileRef: notReported(),
    sourceAdapter: detailFromValue(row?.adapterId),
    sourceQuality: detailFromValue(row?.capabilityStatus ?? readModel?.capabilityStatus),
    localEndpoint: endpointDetail(row?.localEndpoint),
    remoteEndpoint: endpointDetail(row?.destinationEndpoint),
    protocolCandidate: detailFromValue(row?.protocol),
    applicationProtocolCandidate: detailFromValue(row?.tcpState),
    processRef: processDetail(row),
    browserRef: notReported(),
    domainEvidenceRef: domainDetail(row),
    byteSummary: byteSummary(row),
    analyzerAlertRef: notReported(),
    detectionResultRef: notReported(),
    aiAuditRef: notReported(),
    riskBudgetRef: notReported(),
    policyDecisionRef: notReported(),
    interventionResultRef: notReported(),
    eventHistoryRef: detailFromValue(row?.eventId),
    retentionState: notReported(),
    custody: detailFromValue(readModel?.custody),
    evidenceGrade: notReported(),
    confidence: notReported(),
    uncertaintyReasonCodes: uncertaintyReasonCodes(row),
    evidenceReferences: evidenceReferenceDetail(row),
    exactUrlClaim: notReported(),
  };
}

function domainDetail(row: ActivityNetworkFlowObservation | null): PortalDetailValue {
  if (row === null) {
    return notReported();
  }
  return joinedDetail([row.destinationDomain, row.domainAttributionStatus]);
}

function processDetail(row: ActivityNetworkFlowObservation | null): PortalDetailValue {
  if (row === null) {
    return notReported();
  }
  return joinedDetail([row.processName, row.processId, row.processAttributionStatus]);
}

function byteSummary(row: ActivityNetworkFlowObservation | null): PortalDetailValue {
  if (row === null) {
    return notReported();
  }
  return joinedDetail([row.counters.connectionCount, row.counters.bytesSent, row.counters.bytesReceived]);
}

function uncertaintyReasonCodes(row: ActivityNetworkFlowObservation | null): PortalDetailValue {
  if (row === null) {
    return notReported();
  }
  return joinedDetail([row.domainAttributionStatus, row.processAttributionStatus, row.capabilityStatus]);
}

function evidenceReferenceDetail(row: ActivityNetworkFlowObservation | null): PortalDetailValue {
  if (row === null || row.evidence.length === 0) {
    return notReported();
  }
  return joinedDetail(row.evidence.map((evidence) => evidence.evidenceId));
}

function endpointDetail(endpoint: ActivityNetworkEndpoint | null | undefined): PortalDetailValue {
  if (endpoint === null || endpoint === undefined || endpoint.ip === null) {
    return notReported();
  }
  return joinedDetail([endpoint.ip, endpoint.port]);
}

function joinedDetail(values: readonly (LogFieldValue | undefined)[]): PortalDetailValue {
  const normalized = values.filter((value) => value !== undefined && value !== null).map((value) => String(value));
  if (normalized.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(normalized.join(PortalFormatting.EventDetailSeparator));
}

function detailFromValue(value: LogFieldValue | undefined): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
