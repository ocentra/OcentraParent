/* generated from crates/parent-runtime-core/src/network_evidence_drawer.rs */

type NetworkEvidenceDrawerSummaryTemplate = {
  readonly evidenceId: string;
  readonly observedAt: string;
  readonly firstSeenAt: string;
  readonly lastSeenAt: string;
  readonly deviceRef: string;
  readonly childProfileRef: string;
  readonly sourceAdapter: string;
  readonly sourceQuality: string;
  readonly localEndpoint: string;
  readonly remoteEndpoint: string;
  readonly protocolCandidate: string;
  readonly applicationProtocolCandidate: string;
  readonly processRef: string;
  readonly browserRef: string;
  readonly domainEvidenceRef: string;
  readonly byteSummary: string;
  readonly aiAuditRef: string;
  readonly policyDecisionRef: string;
  readonly interventionResultRef: string;
  readonly eventHistoryRef: string;
  readonly retentionState: string;
  readonly custody: string;
  readonly evidenceGrade: string;
  readonly confidence: string;
  readonly uncertaintyReasonCodes: string;
  readonly evidenceReferences: string;
  readonly exactUrlClaim: string;
  readonly platformState: string;
  readonly readModelRows: string;
  readonly degradedState: string;
  readonly deletedEvidenceReferences: string;
};

type NetworkEvidenceDrawerSummaryContext = {
  readonly networkEvidenceSummary?: NetworkEvidenceDrawerEvidenceSummary | null;
};

type NetworkEvidenceDrawerEvidenceSummary = {
  readonly aiAuditRef?: string | null;
  readonly policyDecisionRef?: string | null;
  readonly networkEvidenceGrade?: string | null;
  readonly interventionResultRef?: string | null;
};

type NetworkEvidenceDrawerEndpoint = {
  readonly ip: string | null;
  readonly port: number | null;
};

type NetworkEvidenceDrawerEvidenceRef = {
  readonly evidenceId: string;
};

type NetworkEvidenceDrawerFlowCounters = {
  readonly connectionCount: number | null;
  readonly bytesSent: number | null;
  readonly bytesReceived: number | null;
  readonly firstSeenAt: string | null;
  readonly lastSeenAt: string | null;
};

type NetworkEvidenceDrawerObservation = {
  readonly eventId: string;
  readonly observedAt: string | null;
  readonly adapterId: string | null;
  readonly localEndpoint: NetworkEvidenceDrawerEndpoint;
  readonly destinationEndpoint: NetworkEvidenceDrawerEndpoint;
  readonly protocol: string | null;
  readonly tcpState: string | null;
  readonly processName: string | null;
  readonly processId: number | null;
  readonly processAttributionStatus: string | null;
  readonly destinationDomain: string | null;
  readonly domainAttributionStatus: string | null;
  readonly capabilityStatus: string | null;
  readonly counters: NetworkEvidenceDrawerFlowCounters;
  readonly evidence: readonly NetworkEvidenceDrawerEvidenceRef[];
};

type NetworkEvidenceDrawerReadModel = {
  readonly rows: readonly NetworkEvidenceDrawerObservation[];
  readonly capabilityStatus: string | null;
  readonly custody: string | null;
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly exportableRows: number;
  readonly latestTombstoneEventId: string | null;
  readonly latestTombstoneObservedAt: string | null;
  readonly deletedEvidenceReferenceIds: readonly string[];
};

const EVENT_DETAIL_SEPARATOR = ' | ';
const NOT_REPORTED = 'Not reported';

export function networkEvidenceDrawerSummaryTemplate(
  readModel: NetworkEvidenceDrawerReadModel | null,
  context?: NetworkEvidenceDrawerSummaryContext
): NetworkEvidenceDrawerSummaryTemplate {
  const row = firstNetworkFlowRow(readModel);
  const summary = context?.networkEvidenceSummary ?? null;

  return {
    evidenceId: detailFromRowValue(row, (value) => value.eventId),
    observedAt: detailFromRowValue(row, (value) => value.observedAt),
    firstSeenAt: detailFromRowValue(row, (value) => value.counters.firstSeenAt),
    lastSeenAt: detailFromRowValue(row, (value) => value.counters.lastSeenAt),
    deviceRef: NOT_REPORTED,
    childProfileRef: NOT_REPORTED,
    sourceAdapter: detailFromRowValue(row, (value) => value.adapterId),
    sourceQuality: sourceQualityDetail(row, readModel),
    localEndpoint: endpointDetail(row?.localEndpoint),
    remoteEndpoint: endpointDetail(row?.destinationEndpoint),
    protocolCandidate: detailFromRowValue(row, (value) => value.protocol),
    applicationProtocolCandidate: detailFromRowValue(row, (value) => value.tcpState),
    processRef: processDetail(row),
    browserRef: NOT_REPORTED,
    domainEvidenceRef: domainDetail(row),
    byteSummary: byteSummary(row),
    aiAuditRef: detailFromOptionalString(summary?.aiAuditRef),
    policyDecisionRef: detailFromOptionalString(summary?.policyDecisionRef),
    interventionResultRef: detailFromOptionalString(summary?.interventionResultRef),
    eventHistoryRef: detailFromRowValue(row, (value) => value.eventId),
    retentionState: retentionState(readModel),
    custody: readModelCustodyDetail(readModel),
    evidenceGrade: detailFromOptionalString(summary?.networkEvidenceGrade),
    confidence: NOT_REPORTED,
    uncertaintyReasonCodes: uncertaintyReasonCodes(row),
    evidenceReferences: evidenceReferenceDetail(row),
    exactUrlClaim: NOT_REPORTED,
    platformState: readModelPlatformState(readModel),
    readModelRows: readModelRows(readModel),
    degradedState: degradedState(row, readModel),
    deletedEvidenceReferences: deletedEvidenceReferences(readModel),
  };
}

function firstNetworkFlowRow(
  readModel: NetworkEvidenceDrawerReadModel | null
): NetworkEvidenceDrawerObservation | null {
  return readModel?.rows[0] ?? null;
}

function detailFromRowValue(
  row: NetworkEvidenceDrawerObservation | null,
  select: (row: NetworkEvidenceDrawerObservation) => string | null | undefined
): string {
  return row === null ? NOT_REPORTED : detailFromOptionalString(select(row));
}

function sourceQualityDetail(
  row: NetworkEvidenceDrawerObservation | null,
  readModel: NetworkEvidenceDrawerReadModel | null
): string {
  if (row?.capabilityStatus) {
    return row.capabilityStatus;
  }
  if (readModel?.capabilityStatus) {
    return readModel.capabilityStatus;
  }
  return NOT_REPORTED;
}

function readModelCustodyDetail(readModel: NetworkEvidenceDrawerReadModel | null): string {
  return detailFromOptionalString(readModel?.custody);
}

function readModelPlatformState(readModel: NetworkEvidenceDrawerReadModel | null): string {
  return readModel === null ? NOT_REPORTED : joinedDetail([readModel.custody, readModel.capabilityStatus]);
}

function readModelRows(readModel: NetworkEvidenceDrawerReadModel | null): string {
  return readModel === null
    ? NOT_REPORTED
    : joinedDetail([readModel.returned, readModel.activeRows, readModel.tombstoneRows, readModel.exportableRows]);
}

function degradedState(
  row: NetworkEvidenceDrawerObservation | null,
  readModel: NetworkEvidenceDrawerReadModel | null
): string {
  if (row !== null) {
    return joinedDetail([row.capabilityStatus, row.domainAttributionStatus, row.processAttributionStatus]);
  }
  return detailFromOptionalString(readModel?.capabilityStatus);
}

function domainDetail(row: NetworkEvidenceDrawerObservation | null): string {
  return row === null ? NOT_REPORTED : joinedDetail([row.destinationDomain, row.domainAttributionStatus]);
}

function processDetail(row: NetworkEvidenceDrawerObservation | null): string {
  return row === null ? NOT_REPORTED : joinedDetail([row.processName, row.processId, row.processAttributionStatus]);
}

function byteSummary(row: NetworkEvidenceDrawerObservation | null): string {
  return row === null
    ? NOT_REPORTED
    : joinedDetail([row.counters.connectionCount, row.counters.bytesSent, row.counters.bytesReceived]);
}

function uncertaintyReasonCodes(row: NetworkEvidenceDrawerObservation | null): string {
  return row === null
    ? NOT_REPORTED
    : joinedDetail([row.domainAttributionStatus, row.processAttributionStatus, row.capabilityStatus]);
}

function evidenceReferenceDetail(row: NetworkEvidenceDrawerObservation | null): string {
  return row === null || row.evidence.length === 0
    ? NOT_REPORTED
    : joinedDetail(row.evidence.map((evidence) => evidence.evidenceId));
}

function retentionState(readModel: NetworkEvidenceDrawerReadModel | null): string {
  if (readModel === null) {
    return NOT_REPORTED;
  }
  return readModel.latestTombstoneEventId !== null || readModel.latestTombstoneObservedAt !== null
    ? joinedDetail([
        readModel.latestTombstoneEventId,
        readModel.latestTombstoneObservedAt,
        ...readModel.deletedEvidenceReferenceIds,
      ])
    : joinedDetail([readModel.tombstoneRows, readModel.exportableRows]);
}

function deletedEvidenceReferences(readModel: NetworkEvidenceDrawerReadModel | null): string {
  return readModel === null || readModel.deletedEvidenceReferenceIds.length === 0
    ? NOT_REPORTED
    : joinedDetail(readModel.deletedEvidenceReferenceIds);
}

function endpointDetail(endpoint: NetworkEvidenceDrawerEndpoint | null | undefined): string {
  return endpoint?.ip === null || endpoint?.ip === undefined
    ? NOT_REPORTED
    : joinedDetail([endpoint.ip, endpoint.port]);
}

function joinedDetail(values: readonly unknown[]): string {
  const normalized = values
    .filter((value) => value !== undefined && value !== null && `${value}`.length > 0)
    .map((value) => String(value));
  return normalized.length === 0 ? NOT_REPORTED : normalized.join(EVENT_DETAIL_SEPARATOR);
}

function detailFromOptionalString(value: string | null | undefined): string {
  return value === undefined || value === null || value.trim().length === 0 ? NOT_REPORTED : value;
}
