import type {
  ActivityNetworkEndpoint,
  ActivityNetworkFlowDigest,
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
import type { NetworkRuntimeEventChainSummary } from './network-runtime-event-chain';

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
  readonly auditRef: PortalDetailValue;
  readonly riskBudgetRef: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly interventionResultRef: PortalDetailValue;
  readonly eventHistoryRef: PortalDetailValue;
  readonly retentionState: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly evidenceGrade: PortalDetailValue;
  readonly confidence: PortalDetailValue;
  readonly manualRequiredState: PortalDetailValue;
  readonly unavailableState: PortalDetailValue;
  readonly uncertaintyReasonCodes: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly digestIndicators: PortalDetailValue;
  readonly digestIndicatorEvidence: PortalDetailValue;
  readonly exactUrlClaim: PortalDetailValue;
};

export function networkEvidenceDrawerSummary(
  readModel: ActivityNetworkFlowReadModel | null,
  eventChain: NetworkRuntimeEventChainSummary | null = null,
  digest: ActivityNetworkFlowDigest | null = null
): NetworkEvidenceDrawerSummary {
  const row = firstNetworkFlowRow(readModel);
  const runtime = runtimeEventChainDetails(eventChain, row);
  return {
    evidenceId: detailFromRowValue(row, (value) => value.eventId),
    observedAt: detailFromRowValue(row, (value) => value.observedAt),
    firstSeenAt: detailFromRowValue(row, (value) => value.counters.firstSeenAt),
    lastSeenAt: detailFromRowValue(row, (value) => value.counters.lastSeenAt),
    deviceRef: notReported(),
    childProfileRef: notReported(),
    sourceAdapter: detailFromRowValue(row, (value) => value.adapterId),
    sourceQuality: sourceQualityDetail(row, readModel),
    localEndpoint: endpointDetail(row?.localEndpoint),
    remoteEndpoint: endpointDetail(row?.destinationEndpoint),
    protocolCandidate: detailFromRowValue(row, (value) => value.protocol),
    applicationProtocolCandidate: detailFromRowValue(row, (value) => value.tcpState),
    processRef: processDetail(row),
    browserRef: notReported(),
    domainEvidenceRef: domainDetail(row),
    byteSummary: byteSummary(row),
    analyzerAlertRef: notReported(),
    detectionResultRef: notReported(),
    aiAuditRef: runtime.aiAuditRef,
    auditRef: runtime.auditRef,
    riskBudgetRef: notReported(),
    policyDecisionRef: runtime.policyDecisionRef,
    interventionResultRef: runtime.interventionResultRef,
    eventHistoryRef: runtime.eventHistoryRef,
    retentionState: runtime.retentionState,
    custody: readModelCustodyDetail(readModel),
    evidenceGrade: runtime.evidenceGrade,
    confidence: runtime.confidence,
    manualRequiredState: runtime.manualRequiredState,
    unavailableState: runtime.unavailableState,
    uncertaintyReasonCodes: uncertaintyReasonCodes(row),
    evidenceReferences: evidenceReferenceDetail(row),
    digestIndicators: digestIndicatorDetail(digest),
    digestIndicatorEvidence: digestIndicatorEvidenceDetail(digest),
    exactUrlClaim: notReported(),
  };
}

function runtimeEventChainDetails(
  eventChain: NetworkRuntimeEventChainSummary | null,
  row: ActivityNetworkFlowObservation | null
): NetworkRuntimeEventChainSummary {
  if (eventChain !== null) {
    return eventChain;
  }

  return {
    aiAuditRef: notReported(),
    auditRef: notReported(),
    policyDecisionRef: notReported(),
    interventionResultRef: notReported(),
    eventHistoryRef: detailFromValue(row?.eventId),
    retentionState: notReported(),
    evidenceGrade: notReported(),
    confidence: notReported(),
    manualRequiredState: notReported(),
    unavailableState: notReported(),
  };
}

function firstNetworkFlowRow(readModel: ActivityNetworkFlowReadModel | null): ActivityNetworkFlowObservation | null {
  if (readModel === null) {
    return null;
  }
  return readModel.rows[0] ?? null;
}

function detailFromRowValue(
  row: ActivityNetworkFlowObservation | null,
  getValue: (row: ActivityNetworkFlowObservation) => LogFieldValue | undefined
): PortalDetailValue {
  if (row === null) {
    return notReported();
  }
  return detailFromValue(getValue(row));
}

function sourceQualityDetail(
  row: ActivityNetworkFlowObservation | null,
  readModel: ActivityNetworkFlowReadModel | null
): PortalDetailValue {
  if (row !== null) {
    return detailFromValue(row.capabilityStatus);
  }
  if (readModel !== null) {
    return detailFromValue(readModel.capabilityStatus);
  }
  return notReported();
}

function readModelCustodyDetail(readModel: ActivityNetworkFlowReadModel | null): PortalDetailValue {
  if (readModel === null) {
    return notReported();
  }
  return detailFromValue(readModel.custody);
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

function digestIndicatorDetail(digest: ActivityNetworkFlowDigest | null): PortalDetailValue {
  if (digest === null || digest.unusualIndicators.length === 0) {
    return notReported();
  }
  return joinedDetail(digest.unusualIndicators.map((indicator) => indicator.kind));
}

function digestIndicatorEvidenceDetail(digest: ActivityNetworkFlowDigest | null): PortalDetailValue {
  if (digest === null || digest.unusualIndicators.length === 0) {
    return notReported();
  }
  const evidenceIds = digest.unusualIndicators.flatMap((indicator) => indicator.evidenceIds);
  return joinedDetail([...new Set(evidenceIds)]);
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
