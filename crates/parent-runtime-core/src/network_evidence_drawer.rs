#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerSummaryProjection {
    pub evidence_id: String,
    pub observed_at: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub device_ref: String,
    pub child_profile_ref: String,
    pub source_adapter: String,
    pub source_quality: String,
    pub local_endpoint: String,
    pub remote_endpoint: String,
    pub protocol_candidate: String,
    pub application_protocol_candidate: String,
    pub process_ref: String,
    pub browser_ref: String,
    pub domain_evidence_ref: String,
    pub byte_summary: String,
    pub analyzer_alert_ref: String,
    pub detection_result_ref: String,
    pub ai_audit_ref: String,
    pub risk_budget_ref: String,
    pub policy_decision_ref: String,
    pub intervention_result_ref: String,
    pub event_history_ref: String,
    pub retention_state: String,
    pub custody: String,
    pub evidence_grade: String,
    pub confidence: String,
    pub uncertainty_reason_codes: String,
    pub evidence_references: String,
    pub exact_url_claim: String,
    pub platform_state: String,
    pub read_model_rows: String,
    pub degraded_state: String,
    pub deleted_evidence_references: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerSummaryContext<'a> {
    pub network_evidence_summary: Option<NetworkEvidenceDrawerEvidenceSummary<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerEvidenceSummary<'a> {
    pub analyzer_alert_ref: Option<&'a str>,
    pub detection_result_ref: Option<&'a str>,
    pub ai_audit_ref: Option<&'a str>,
    pub risk_budget_ref: Option<&'a str>,
    pub policy_decision_ref: Option<&'a str>,
    pub network_evidence_grade: Option<&'a str>,
    pub intervention_result_ref: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerEndpoint<'a> {
    pub ip: Option<&'a str>,
    pub port: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerEvidenceRef<'a> {
    pub evidence_id: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerFlowCounters<'a> {
    pub connection_count: Option<u32>,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub first_seen_at: Option<&'a str>,
    pub last_seen_at: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerObservation<'a> {
    pub event_id: &'a str,
    pub observed_at: Option<&'a str>,
    pub adapter_id: Option<&'a str>,
    pub local_endpoint: NetworkEvidenceDrawerEndpoint<'a>,
    pub destination_endpoint: NetworkEvidenceDrawerEndpoint<'a>,
    pub protocol: Option<&'a str>,
    pub tcp_state: Option<&'a str>,
    pub process_name: Option<&'a str>,
    pub process_id: Option<u32>,
    pub process_attribution_status: Option<&'a str>,
    pub destination_domain: Option<&'a str>,
    pub domain_attribution_status: Option<&'a str>,
    pub capability_status: Option<&'a str>,
    pub counters: NetworkEvidenceDrawerFlowCounters<'a>,
    pub evidence: Vec<NetworkEvidenceDrawerEvidenceRef<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkEvidenceDrawerReadModel<'a> {
    pub rows: Vec<NetworkEvidenceDrawerObservation<'a>>,
    pub capability_status: Option<&'a str>,
    pub custody: Option<&'a str>,
    pub returned: u32,
    pub active_rows: u32,
    pub tombstone_rows: u32,
    pub exportable_rows: u32,
    pub latest_tombstone_event_id: Option<&'a str>,
    pub latest_tombstone_observed_at: Option<&'a str>,
    pub deleted_evidence_reference_ids: Vec<&'a str>,
}

const EVENT_DETAIL_SEPARATOR: &str = " | ";
const NOT_REPORTED: &str = "Not reported";

#[path = "network_evidence_drawer_details.rs"]
mod network_evidence_drawer_details;

use self::network_evidence_drawer_details::*;

pub fn project_network_evidence_drawer_summary(
    read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>,
    context: Option<&NetworkEvidenceDrawerSummaryContext<'_>>,
) -> NetworkEvidenceDrawerSummaryProjection {
    let row = first_network_flow_row(read_model);
    let summary = context.and_then(|value| value.network_evidence_summary.as_ref());

    NetworkEvidenceDrawerSummaryProjection {
        evidence_id: detail_from_row_value(row, |value| Some(value.event_id)),
        observed_at: detail_from_row_value(row, |value| value.observed_at),
        first_seen_at: detail_from_row_value(row, |value| value.counters.first_seen_at),
        last_seen_at: detail_from_row_value(row, |value| value.counters.last_seen_at),
        device_ref: not_reported(),
        child_profile_ref: not_reported(),
        source_adapter: detail_from_row_value(row, |value| value.adapter_id),
        source_quality: source_quality_detail(row, read_model),
        local_endpoint: endpoint_detail(row.map(|value| &value.local_endpoint)),
        remote_endpoint: endpoint_detail(row.map(|value| &value.destination_endpoint)),
        protocol_candidate: detail_from_row_value(row, |value| value.protocol),
        application_protocol_candidate: detail_from_row_value(row, |value| value.tcp_state),
        process_ref: process_detail(row),
        browser_ref: not_reported(),
        domain_evidence_ref: domain_detail(row),
        byte_summary: byte_summary(row),
        analyzer_alert_ref: detail_from_optional_str(summary.and_then(|value| value.analyzer_alert_ref)),
        detection_result_ref: detail_from_optional_str(summary.and_then(|value| value.detection_result_ref)),
        ai_audit_ref: detail_from_optional_str(summary.and_then(|value| value.ai_audit_ref)),
        risk_budget_ref: detail_from_optional_str(summary.and_then(|value| value.risk_budget_ref)),
        policy_decision_ref: detail_from_optional_str(summary.and_then(|value| value.policy_decision_ref)),
        intervention_result_ref: detail_from_optional_str(summary.and_then(|value| value.intervention_result_ref)),
        event_history_ref: detail_from_row_value(row, |value| Some(value.event_id)),
        retention_state: retention_state(read_model),
        custody: read_model_custody_detail(read_model),
        evidence_grade: detail_from_optional_str(summary.and_then(|value| value.network_evidence_grade)),
        confidence: not_reported(),
        uncertainty_reason_codes: uncertainty_reason_codes(row),
        evidence_references: evidence_reference_detail(row),
        exact_url_claim: not_reported(),
        platform_state: read_model_platform_state(read_model),
        read_model_rows: read_model_rows(read_model),
        degraded_state: degraded_state(row, read_model),
        deleted_evidence_references: deleted_evidence_references(read_model),
    }
}

pub fn network_evidence_drawer_typescript() -> String {
    NETWORK_EVIDENCE_DRAWER_TYPESCRIPT.to_string()
}

fn first_network_flow_row<'a>(
    read_model: Option<&'a NetworkEvidenceDrawerReadModel<'a>>,
) -> Option<&'a NetworkEvidenceDrawerObservation<'a>> {
    read_model.and_then(|value| value.rows.first())
}

fn detail_from_row_value<'a>(
    row: Option<&'a NetworkEvidenceDrawerObservation<'a>>,
    select: impl Fn(&'a NetworkEvidenceDrawerObservation<'a>) -> Option<&'a str>,
) -> String {
    row.map_or_else(not_reported, |value| detail_from_optional_str(select(value)))
}

fn source_quality_detail(
    row: Option<&NetworkEvidenceDrawerObservation<'_>>,
    read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>,
) -> String {
    if let Some(value) = row.and_then(|candidate| candidate.capability_status) {
        return value.to_string();
    }
    if let Some(value) = read_model.and_then(|candidate| candidate.capability_status) {
        return value.to_string();
    }
    not_reported()
}

fn read_model_custody_detail(read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>) -> String {
    detail_from_optional_str(read_model.and_then(|value| value.custody))
}

fn read_model_platform_state(read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>) -> String {
    match read_model {
        Some(value) => joined_detail([
            value.custody.map(ToString::to_string),
            value.capability_status.map(ToString::to_string),
        ]),
        None => not_reported(),
    }
}

fn read_model_rows(read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>) -> String {
    match read_model {
        Some(value) => joined_detail([
            Some(value.returned.to_string()),
            Some(value.active_rows.to_string()),
            Some(value.tombstone_rows.to_string()),
            Some(value.exportable_rows.to_string()),
        ]),
        None => not_reported(),
    }
}

fn degraded_state(
    row: Option<&NetworkEvidenceDrawerObservation<'_>>,
    read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>,
) -> String {
    if let Some(value) = row {
        return joined_detail([
            value.capability_status.map(ToString::to_string),
            value.domain_attribution_status.map(ToString::to_string),
            value.process_attribution_status.map(ToString::to_string),
        ]);
    }
    detail_from_optional_str(read_model.and_then(|value| value.capability_status))
}

fn domain_detail(row: Option<&NetworkEvidenceDrawerObservation<'_>>) -> String {
    match row {
        Some(value) => joined_detail([
            value.destination_domain.map(ToString::to_string),
            value.domain_attribution_status.map(ToString::to_string),
        ]),
        None => not_reported(),
    }
}

fn process_detail(row: Option<&NetworkEvidenceDrawerObservation<'_>>) -> String {
    match row {
        Some(value) => joined_detail([
            value.process_name.map(ToString::to_string),
            value.process_id.map(|candidate| candidate.to_string()),
            value.process_attribution_status.map(ToString::to_string),
        ]),
        None => not_reported(),
    }
}

fn byte_summary(row: Option<&NetworkEvidenceDrawerObservation<'_>>) -> String {
    match row {
        Some(value) => joined_detail([
            value.counters.connection_count.map(|candidate| candidate.to_string()),
            value.counters.bytes_sent.map(|candidate| candidate.to_string()),
            value.counters.bytes_received.map(|candidate| candidate.to_string()),
        ]),
        None => not_reported(),
    }
}

fn uncertainty_reason_codes(row: Option<&NetworkEvidenceDrawerObservation<'_>>) -> String {
    match row {
        Some(value) => joined_detail([
            value.domain_attribution_status.map(ToString::to_string),
            value.process_attribution_status.map(ToString::to_string),
            value.capability_status.map(ToString::to_string),
        ]),
        None => not_reported(),
    }
}

fn evidence_reference_detail(row: Option<&NetworkEvidenceDrawerObservation<'_>>) -> String {
    match row {
        Some(value) if !value.evidence.is_empty() => joined_detail(
            value
                .evidence
                .iter()
                .map(|evidence| Some(evidence.evidence_id.to_string())),
        ),
        _ => not_reported(),
    }
}

fn retention_state(read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>) -> String {
    match read_model {
        Some(value)
            if value.latest_tombstone_event_id.is_some()
                || value.latest_tombstone_observed_at.is_some() =>
        {
            let mut details = vec![
                value.latest_tombstone_event_id.map(ToString::to_string),
                value.latest_tombstone_observed_at.map(ToString::to_string),
            ];
            details.extend(
                value
                    .deleted_evidence_reference_ids
                    .iter()
                    .map(|candidate| Some((*candidate).to_string())),
            );
            joined_detail(details)
        }
        Some(value) => joined_detail([
            Some(value.tombstone_rows.to_string()),
            Some(value.exportable_rows.to_string()),
        ]),
        None => not_reported(),
    }
}

fn deleted_evidence_references(read_model: Option<&NetworkEvidenceDrawerReadModel<'_>>) -> String {
    match read_model {
        Some(value) if !value.deleted_evidence_reference_ids.is_empty() => joined_detail(
            value
                .deleted_evidence_reference_ids
                .iter()
                .map(|candidate| Some((*candidate).to_string())),
        ),
        _ => not_reported(),
    }
}

fn endpoint_detail(endpoint: Option<&NetworkEvidenceDrawerEndpoint<'_>>) -> String {
    match endpoint {
        Some(value) if value.ip.is_some() => joined_detail([
            value.ip.map(ToString::to_string),
            value.port.map(|candidate| candidate.to_string()),
        ]),
        _ => not_reported(),
    }
}

const NETWORK_EVIDENCE_DRAWER_TYPESCRIPT: &str = r#"/* generated from crates/parent-runtime-core/src/network_evidence_drawer.rs */

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
  readonly analyzerAlertRef: string;
  readonly detectionResultRef: string;
  readonly aiAuditRef: string;
  readonly riskBudgetRef: string;
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
  readonly analyzerAlertRef?: string | null;
  readonly detectionResultRef?: string | null;
  readonly aiAuditRef?: string | null;
  readonly riskBudgetRef?: string | null;
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
    analyzerAlertRef: detailFromOptionalString(summary?.analyzerAlertRef),
    detectionResultRef: detailFromOptionalString(summary?.detectionResultRef),
    aiAuditRef: detailFromOptionalString(summary?.aiAuditRef),
    riskBudgetRef: detailFromOptionalString(summary?.riskBudgetRef),
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

function firstNetworkFlowRow(readModel: NetworkEvidenceDrawerReadModel | null): NetworkEvidenceDrawerObservation | null {
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
  return endpoint?.ip === null || endpoint?.ip === undefined ? NOT_REPORTED : joinedDetail([endpoint.ip, endpoint.port]);
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
"#;
