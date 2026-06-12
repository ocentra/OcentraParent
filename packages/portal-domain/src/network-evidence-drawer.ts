import type {
  ActivityNetworkEndpoint,
  ActivityNetworkFlowObservation,
  ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import { PortalFormatting } from './formatting';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';

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
  readonly platformState: PortalDetailValue;
  readonly readModelRows: PortalDetailValue;
  readonly degradedState: PortalDetailValue;
  readonly deletedEvidenceReferences: PortalDetailValue;
};

export function networkEvidenceDrawerSummary(
  readModel: ActivityNetworkFlowReadModel | null
): NetworkEvidenceDrawerSummary {
  const row = firstNetworkFlowRow(readModel);
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
    aiAuditRef: notReported(),
    riskBudgetRef: notReported(),
    policyDecisionRef: notReported(),
    interventionResultRef: notReported(),
    eventHistoryRef: detailFromValue(row?.eventId),
    retentionState: retentionState(readModel),
    custody: readModelCustodyDetail(readModel),
    evidenceGrade: notReported(),
    confidence: notReported(),
    uncertaintyReasonCodes: uncertaintyReasonCodes(row),
    evidenceReferences: evidenceReferenceDetail(row),
    exactUrlClaim: notReported(),
    platformState: readModelPlatformState(readModel),
    readModelRows: readModelRows(readModel),
    degradedState: degradedState(row, readModel),
    deletedEvidenceReferences: deletedEvidenceReferences(readModel),
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

function readModelPlatformState(readModel: ActivityNetworkFlowReadModel | null): PortalDetailValue {
  if (readModel === null) {
    return notReported();
  }
  return joinedDetail([readModel.custody, readModel.capabilityStatus]);
}

function readModelRows(readModel: ActivityNetworkFlowReadModel | null): PortalDetailValue {
  if (readModel === null) {
    return notReported();
  }
  return joinedDetail([readModel.returned, readModel.activeRows, readModel.tombstoneRows, readModel.exportableRows]);
}

function degradedState(
  row: ActivityNetworkFlowObservation | null,
  readModel: ActivityNetworkFlowReadModel | null
): PortalDetailValue {
  if (row !== null) {
    return joinedDetail([row.capabilityStatus, row.domainAttributionStatus, row.processAttributionStatus]);
  }
  if (readModel !== null) {
    return detailFromValue(readModel.capabilityStatus);
  }
  return notReported();
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

function retentionState(readModel: ActivityNetworkFlowReadModel | null): PortalDetailValue {
  if (readModel === null) {
    return notReported();
  }
  if (readModel.latestTombstoneEventId !== null || readModel.latestTombstoneObservedAt !== null) {
    return joinedDetail([
      readModel.latestTombstoneEventId,
      readModel.latestTombstoneObservedAt,
      ...readModel.deletedEvidenceReferenceIds,
    ]);
  }
  return joinedDetail([readModel.tombstoneRows, readModel.exportableRows]);
}

function deletedEvidenceReferences(readModel: ActivityNetworkFlowReadModel | null): PortalDetailValue {
  if (readModel === null || readModel.deletedEvidenceReferenceIds.length === 0) {
    return notReported();
  }
  return joinedDetail(readModel.deletedEvidenceReferenceIds);
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
  return decodePortalDetailValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}
