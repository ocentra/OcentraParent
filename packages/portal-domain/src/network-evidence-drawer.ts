import { decodePortalDetailValue, type PortalDetailValue } from './portal-contract-text-contracts';
import { networkEvidenceDrawerSummaryTemplate } from './network-evidence-drawer.generated';

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
  readonly aiAuditRef: PortalDetailValue;
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

interface NetworkEvidenceDrawerSummaryContext {
  readonly networkEvidenceSummary?: NetworkEvidenceDrawerEvidenceSummary | null;
}

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

export function networkEvidenceDrawerSummary(
  readModel: NetworkEvidenceDrawerReadModel | null,
  context?: NetworkEvidenceDrawerSummaryContext
): NetworkEvidenceDrawerSummary {
  const template = networkEvidenceDrawerSummaryTemplate(readModel, context);

  return {
    evidenceId: decodePortalDetailValue(template.evidenceId),
    observedAt: decodePortalDetailValue(template.observedAt),
    firstSeenAt: decodePortalDetailValue(template.firstSeenAt),
    lastSeenAt: decodePortalDetailValue(template.lastSeenAt),
    deviceRef: decodePortalDetailValue(template.deviceRef),
    childProfileRef: decodePortalDetailValue(template.childProfileRef),
    sourceAdapter: decodePortalDetailValue(template.sourceAdapter),
    sourceQuality: decodePortalDetailValue(template.sourceQuality),
    localEndpoint: decodePortalDetailValue(template.localEndpoint),
    remoteEndpoint: decodePortalDetailValue(template.remoteEndpoint),
    protocolCandidate: decodePortalDetailValue(template.protocolCandidate),
    applicationProtocolCandidate: decodePortalDetailValue(template.applicationProtocolCandidate),
    processRef: decodePortalDetailValue(template.processRef),
    browserRef: decodePortalDetailValue(template.browserRef),
    domainEvidenceRef: decodePortalDetailValue(template.domainEvidenceRef),
    byteSummary: decodePortalDetailValue(template.byteSummary),
    aiAuditRef: decodePortalDetailValue(template.aiAuditRef),
    policyDecisionRef: decodePortalDetailValue(template.policyDecisionRef),
    interventionResultRef: decodePortalDetailValue(template.interventionResultRef),
    eventHistoryRef: decodePortalDetailValue(template.eventHistoryRef),
    retentionState: decodePortalDetailValue(template.retentionState),
    custody: decodePortalDetailValue(template.custody),
    evidenceGrade: decodePortalDetailValue(template.evidenceGrade),
    confidence: decodePortalDetailValue(template.confidence),
    uncertaintyReasonCodes: decodePortalDetailValue(template.uncertaintyReasonCodes),
    evidenceReferences: decodePortalDetailValue(template.evidenceReferences),
    exactUrlClaim: decodePortalDetailValue(template.exactUrlClaim),
    platformState: decodePortalDetailValue(template.platformState),
    readModelRows: decodePortalDetailValue(template.readModelRows),
    degradedState: decodePortalDetailValue(template.degradedState),
    deletedEvidenceReferences: decodePortalDetailValue(template.deletedEvidenceReferences),
  };
}
