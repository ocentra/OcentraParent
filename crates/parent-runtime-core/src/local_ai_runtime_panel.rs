#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalAiRuntimePanelProjection {
    pub summary_status: String,
    pub summary_read_model_rows: String,
    pub summary_product_claim: String,
    pub cards: Vec<LocalAiRuntimePanelCardProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalAiRuntimePanelCardProjection {
    pub kind: String,
    pub details: Vec<LocalAiRuntimePanelDetailProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalAiRuntimePanelDetailProjection {
    pub field_key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalAiRuntimePanelInput<'a> {
    pub runtime_status: Option<LocalAiRuntimeStatusInput<'a>>,
    pub household_job: Option<LocalAiHouseholdJobInput<'a>>,
    pub memory_graph: Option<LocalAiMemoryGraphInput<'a>>,
    pub remote_assistant_boundary: Option<LocalAiRemoteAssistantBoundaryInput<'a>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalAiRuntimeStatusInput<'a> {
    pub event_id: Option<&'a str>,
    pub sent_at: Option<&'a str>,
    pub runtime_reference: Option<&'a str>,
    pub provider: Option<&'a str>,
    pub model: Option<&'a str>,
    pub load_state: Option<&'a str>,
    pub capability: Option<&'a str>,
    pub resource_class: Option<&'a str>,
    pub degraded_state: Option<&'a str>,
    pub privacy_mode: Option<&'a str>,
    pub execution_state: Option<&'a str>,
    pub reason: Option<&'a str>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalAiHouseholdJobInput<'a> {
    pub event_id: Option<&'a str>,
    pub sent_at: Option<&'a str>,
    pub request_id: Option<&'a str>,
    pub status: Option<&'a str>,
    pub state: Option<&'a str>,
    pub provider: Option<&'a str>,
    pub provider_source: Option<&'a str>,
    pub capability: Option<&'a str>,
    pub resource_class: Option<&'a str>,
    pub load_state: Option<&'a str>,
    pub privacy_mode: Option<&'a str>,
    pub custody: Option<&'a str>,
    pub policy_readiness: Option<&'a str>,
    pub adapter_boundary: Option<&'a str>,
    pub lease_id: Option<&'a str>,
    pub last_checked: Option<&'a str>,
    pub last_observed: Option<&'a str>,
    pub decision_source: Option<&'a str>,
    pub execution_state: Option<&'a str>,
    pub reason: Option<&'a str>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalAiMemoryGraphInput<'a> {
    pub custody: Option<&'a str>,
    pub capability_status: Option<&'a str>,
    pub generated_at: Option<&'a str>,
    pub returned_node_count: u32,
    pub returned_edge_count: u32,
    pub omitted_edge_count: u32,
    pub degraded_reasons: Vec<&'a str>,
    pub evidence_reference_ids: Vec<&'a str>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalAiRemoteAssistantBoundaryInput<'a> {
    pub event_id: Option<&'a str>,
    pub sent_at: Option<&'a str>,
    pub request_id: Option<&'a str>,
    pub state: Option<&'a str>,
    pub provider: Option<&'a str>,
    pub adapter_boundary: Option<&'a str>,
    pub policy_readiness: Option<&'a str>,
    pub custody: Option<&'a str>,
    pub deleted_evidence: Option<&'a str>,
    pub privacy_mode: Option<&'a str>,
    pub evidence_references: Option<&'a str>,
    pub row_count: Option<&'a str>,
}

const NOT_REPORTED: &str = "not-reported";
const CARD_RUNTIME_STATUS: &str = "runtime-status";
const CARD_HOUSEHOLD_JOB: &str = "household-job";
const CARD_MEMORY_GRAPH: &str = "memory-graph";
const CARD_REMOTE_ASSISTANT_BOUNDARY: &str = "remote-assistant-boundary";

const FIELD_EVENT_ID: &str = "eventId";
const FIELD_SENT_AT: &str = "sentAt";
const FIELD_RUNTIME_REFERENCE: &str = "runtimeReference";
const FIELD_PROVIDER: &str = "provider";
const FIELD_MODEL: &str = "model";
const FIELD_LOAD_STATE: &str = "loadState";
const FIELD_CAPABILITY: &str = "capability";
const FIELD_RESOURCE_CLASS: &str = "resourceClass";
const FIELD_DEGRADED_STATE: &str = "degradedState";
const FIELD_PRIVACY_MODE: &str = "privacyMode";
const FIELD_EXECUTION_STATE: &str = "executionState";
const FIELD_REASON: &str = "reason";
const FIELD_REQUEST_ID: &str = "requestId";
const FIELD_STATUS: &str = "status";
const FIELD_STATE: &str = "state";
const FIELD_PROVIDER_SOURCE: &str = "providerSource";
const FIELD_CUSTODY: &str = "custody";
const FIELD_POLICY_READINESS: &str = "policyReadiness";
const FIELD_ADAPTER_BOUNDARY: &str = "adapterBoundary";
const FIELD_LAST_CHECKED: &str = "lastChecked";
const FIELD_LAST_OBSERVED: &str = "lastObserved";
const FIELD_DECISION_SOURCE: &str = "decisionSource";
const FIELD_PRODUCT_CLAIM: &str = "productClaim";
const FIELD_GENERATED_AT: &str = "generatedAt";
const FIELD_GRAPH_NODES: &str = "graphNodes";
const FIELD_GRAPH_EDGES: &str = "graphEdges";
const FIELD_GRAPH_OMITTED_EDGES: &str = "graphOmittedEdges";
const FIELD_EVIDENCE_REFERENCES: &str = "evidenceReferences";
const FIELD_DELETED_EVIDENCE: &str = "deletedEvidence";
const FIELD_ROW_COUNT: &str = "rowCount";

const SUMMARY_PRODUCT_CLAIM: &str = "no-model-quality-or-enforcement-claim";
const MEMORY_GRAPH_PRODUCT_CLAIM: &str = "source-cited-memory-graph-read-model-only";
const HOUSEHOLD_JOB_PRODUCT_CLAIM: &str = "worker-only-child-agent-authority";
const REMOTE_ASSISTANT_PRODUCT_CLAIM: &str = "remote-assistant-report-only-local-policy-authority";

pub fn project_local_ai_runtime_panel(
    input: &LocalAiRuntimePanelInput<'_>,
) -> LocalAiRuntimePanelProjection {
    let cards = [
        runtime_status_card(input.runtime_status.as_ref()),
        household_job_card(input.household_job.as_ref()),
        memory_graph_card(input.memory_graph.as_ref()),
        remote_assistant_boundary_card(input.remote_assistant_boundary.as_ref()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    LocalAiRuntimePanelProjection {
        summary_status: if cards.is_empty() {
            NOT_REPORTED.to_string()
        } else {
            "reported".to_string()
        },
        summary_read_model_rows: cards.len().to_string(),
        summary_product_claim: SUMMARY_PRODUCT_CLAIM.to_string(),
        cards,
    }
}

pub fn local_ai_runtime_panel_typescript() -> String {
    LOCAL_AI_RUNTIME_PANEL_TYPESCRIPT.to_string()
}

fn runtime_status_card(
    input: Option<&LocalAiRuntimeStatusInput<'_>>,
) -> Option<LocalAiRuntimePanelCardProjection> {
    input.map(|value| LocalAiRuntimePanelCardProjection {
        kind: CARD_RUNTIME_STATUS.to_string(),
        details: vec![
            detail(FIELD_EVENT_ID, value.event_id.map(ToString::to_string)),
            detail(FIELD_SENT_AT, value.sent_at.map(ToString::to_string)),
            detail(
                FIELD_RUNTIME_REFERENCE,
                value.runtime_reference.map(ToString::to_string),
            ),
            detail(FIELD_PROVIDER, value.provider.map(ToString::to_string)),
            detail(FIELD_MODEL, value.model.map(ToString::to_string)),
            detail(FIELD_LOAD_STATE, value.load_state.map(ToString::to_string)),
            detail(FIELD_CAPABILITY, value.capability.map(ToString::to_string)),
            detail(
                FIELD_RESOURCE_CLASS,
                value.resource_class.map(ToString::to_string),
            ),
            detail(
                FIELD_DEGRADED_STATE,
                value.degraded_state.map(ToString::to_string),
            ),
            detail(
                FIELD_PRIVACY_MODE,
                value.privacy_mode.map(ToString::to_string),
            ),
            detail(
                FIELD_EXECUTION_STATE,
                value.execution_state.map(ToString::to_string),
            ),
            detail(FIELD_REASON, value.reason.map(ToString::to_string)),
        ],
    })
}

fn household_job_card(
    input: Option<&LocalAiHouseholdJobInput<'_>>,
) -> Option<LocalAiRuntimePanelCardProjection> {
    input.map(|value| LocalAiRuntimePanelCardProjection {
        kind: CARD_HOUSEHOLD_JOB.to_string(),
        details: vec![
            detail(FIELD_EVENT_ID, value.event_id.map(ToString::to_string)),
            detail(FIELD_SENT_AT, value.sent_at.map(ToString::to_string)),
            detail(FIELD_REQUEST_ID, value.request_id.map(ToString::to_string)),
            detail(FIELD_STATUS, value.status.map(ToString::to_string)),
            detail(FIELD_STATE, value.state.map(ToString::to_string)),
            detail(FIELD_PROVIDER, value.provider.map(ToString::to_string)),
            detail(
                FIELD_PROVIDER_SOURCE,
                value.provider_source.map(ToString::to_string),
            ),
            detail(FIELD_CAPABILITY, value.capability.map(ToString::to_string)),
            detail(
                FIELD_RESOURCE_CLASS,
                value.resource_class.map(ToString::to_string),
            ),
            detail(FIELD_LOAD_STATE, value.load_state.map(ToString::to_string)),
            detail(
                FIELD_PRIVACY_MODE,
                value.privacy_mode.map(ToString::to_string),
            ),
            detail(FIELD_CUSTODY, value.custody.map(ToString::to_string)),
            detail(
                FIELD_POLICY_READINESS,
                value.policy_readiness.map(ToString::to_string),
            ),
            detail(
                FIELD_ADAPTER_BOUNDARY,
                value.adapter_boundary.map(ToString::to_string),
            ),
            detail(FIELD_REQUEST_ID, value.lease_id.map(ToString::to_string)),
            detail(
                FIELD_LAST_CHECKED,
                value.last_checked.map(ToString::to_string),
            ),
            detail(
                FIELD_LAST_OBSERVED,
                value.last_observed.map(ToString::to_string),
            ),
            detail(
                FIELD_DECISION_SOURCE,
                value.decision_source.map(ToString::to_string),
            ),
            detail(
                FIELD_EXECUTION_STATE,
                value.execution_state.map(ToString::to_string),
            ),
            detail(FIELD_REASON, value.reason.map(ToString::to_string)),
            detail(
                FIELD_PRODUCT_CLAIM,
                Some(HOUSEHOLD_JOB_PRODUCT_CLAIM.to_string()),
            ),
        ],
    })
}

fn memory_graph_card(
    input: Option<&LocalAiMemoryGraphInput<'_>>,
) -> Option<LocalAiRuntimePanelCardProjection> {
    input.map(|value| LocalAiRuntimePanelCardProjection {
        kind: CARD_MEMORY_GRAPH.to_string(),
        details: vec![
            detail(FIELD_CUSTODY, value.custody.map(ToString::to_string)),
            detail(
                FIELD_CAPABILITY,
                value.capability_status.map(ToString::to_string),
            ),
            detail(
                FIELD_GENERATED_AT,
                value.generated_at.map(ToString::to_string),
            ),
            detail(
                FIELD_GRAPH_NODES,
                Some(value.returned_node_count.to_string()),
            ),
            detail(
                FIELD_GRAPH_EDGES,
                Some(value.returned_edge_count.to_string()),
            ),
            detail(
                FIELD_GRAPH_OMITTED_EDGES,
                Some(value.omitted_edge_count.to_string()),
            ),
            detail(
                FIELD_EVIDENCE_REFERENCES,
                Some(unique_list(value.evidence_reference_ids.iter().copied())),
            ),
            detail(
                FIELD_DEGRADED_STATE,
                Some(comma_list(value.degraded_reasons.iter().copied())),
            ),
            detail(
                FIELD_PRODUCT_CLAIM,
                Some(MEMORY_GRAPH_PRODUCT_CLAIM.to_string()),
            ),
        ],
    })
}

fn remote_assistant_boundary_card(
    input: Option<&LocalAiRemoteAssistantBoundaryInput<'_>>,
) -> Option<LocalAiRuntimePanelCardProjection> {
    input.map(|value| LocalAiRuntimePanelCardProjection {
        kind: CARD_REMOTE_ASSISTANT_BOUNDARY.to_string(),
        details: vec![
            detail(FIELD_EVENT_ID, value.event_id.map(ToString::to_string)),
            detail(FIELD_SENT_AT, value.sent_at.map(ToString::to_string)),
            detail(FIELD_REQUEST_ID, value.request_id.map(ToString::to_string)),
            detail(FIELD_STATE, value.state.map(ToString::to_string)),
            detail(FIELD_PROVIDER, value.provider.map(ToString::to_string)),
            detail(
                FIELD_ADAPTER_BOUNDARY,
                value.adapter_boundary.map(ToString::to_string),
            ),
            detail(
                FIELD_POLICY_READINESS,
                value.policy_readiness.map(ToString::to_string),
            ),
            detail(FIELD_CUSTODY, value.custody.map(ToString::to_string)),
            detail(
                FIELD_DELETED_EVIDENCE,
                value.deleted_evidence.map(ToString::to_string),
            ),
            detail(
                FIELD_PRIVACY_MODE,
                value.privacy_mode.map(ToString::to_string),
            ),
            detail(
                FIELD_EVIDENCE_REFERENCES,
                value.evidence_references.map(ToString::to_string),
            ),
            detail(FIELD_ROW_COUNT, value.row_count.map(ToString::to_string)),
            detail(
                FIELD_PRODUCT_CLAIM,
                Some(REMOTE_ASSISTANT_PRODUCT_CLAIM.to_string()),
            ),
        ],
    })
}

fn detail(field_key: &str, value: Option<String>) -> LocalAiRuntimePanelDetailProjection {
    LocalAiRuntimePanelDetailProjection {
        field_key: field_key.to_string(),
        value: normalize_value(value),
    }
}

fn normalize_value(value: Option<String>) -> String {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
    {
        Some(candidate) => candidate.to_string(),
        None => NOT_REPORTED.to_string(),
    }
}

fn comma_list<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let normalized = values
        .into_iter()
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
        .collect::<Vec<_>>();
    if normalized.is_empty() {
        return NOT_REPORTED.to_string();
    }
    normalized.join(",")
}

fn unique_list<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let mut ordered = Vec::new();
    for value in values {
        let candidate = value.trim();
        if candidate.is_empty() || ordered.contains(&candidate) {
            continue;
        }
        ordered.push(candidate);
    }
    if ordered.is_empty() {
        return NOT_REPORTED.to_string();
    }
    ordered.join(",")
}

const LOCAL_AI_RUNTIME_PANEL_TYPESCRIPT: &str = r#"/* generated from crates/parent-runtime-core/src/local_ai_runtime_panel.rs */

export type LocalAiRuntimePanelCardKind =
  | 'runtime-status'
  | 'household-job'
  | 'memory-graph'
  | 'remote-assistant-boundary';

export type LocalAiRuntimePanelFieldKey =
  | 'eventId'
  | 'sentAt'
  | 'runtimeReference'
  | 'provider'
  | 'model'
  | 'loadState'
  | 'capability'
  | 'resourceClass'
  | 'degradedState'
  | 'privacyMode'
  | 'executionState'
  | 'reason'
  | 'requestId'
  | 'status'
  | 'state'
  | 'providerSource'
  | 'custody'
  | 'policyReadiness'
  | 'adapterBoundary'
  | 'lastChecked'
  | 'lastObserved'
  | 'decisionSource'
  | 'productClaim'
  | 'generatedAt'
  | 'graphNodes'
  | 'graphEdges'
  | 'graphOmittedEdges'
  | 'evidenceReferences'
  | 'deletedEvidence'
  | 'rowCount';

export type LocalAiRuntimePanelTemplate = {
  readonly summaryStatus: string;
  readonly summaryReadModelRows: string;
  readonly summaryProductClaim: string;
  readonly cards: readonly LocalAiRuntimePanelCardTemplate[];
};

export type LocalAiRuntimePanelCardTemplate = {
  readonly kind: LocalAiRuntimePanelCardKind;
  readonly details: readonly LocalAiRuntimePanelDetailTemplate[];
};

export type LocalAiRuntimePanelDetailTemplate = {
  readonly fieldKey: LocalAiRuntimePanelFieldKey;
  readonly value: string;
};

export type LocalAiRuntimePanelInput = {
  readonly runtimeStatus?: LocalAiRuntimeStatusInput | null;
  readonly householdJob?: LocalAiHouseholdJobInput | null;
  readonly memoryGraph?: LocalAiMemoryGraphInput | null;
  readonly remoteAssistantBoundary?: LocalAiRemoteAssistantBoundaryInput | null;
};

export type LocalAiRuntimeStatusInput = {
  readonly eventId?: string | null;
  readonly sentAt?: string | null;
  readonly runtimeReference?: string | null;
  readonly provider?: string | null;
  readonly model?: string | null;
  readonly loadState?: string | null;
  readonly capability?: string | null;
  readonly resourceClass?: string | null;
  readonly degradedState?: string | null;
  readonly privacyMode?: string | null;
  readonly executionState?: string | null;
  readonly reason?: string | null;
};

export type LocalAiHouseholdJobInput = {
  readonly eventId?: string | null;
  readonly sentAt?: string | null;
  readonly requestId?: string | null;
  readonly status?: string | null;
  readonly state?: string | null;
  readonly provider?: string | null;
  readonly providerSource?: string | null;
  readonly capability?: string | null;
  readonly resourceClass?: string | null;
  readonly loadState?: string | null;
  readonly privacyMode?: string | null;
  readonly custody?: string | null;
  readonly policyReadiness?: string | null;
  readonly adapterBoundary?: string | null;
  readonly leaseId?: string | null;
  readonly lastChecked?: string | null;
  readonly lastObserved?: string | null;
  readonly decisionSource?: string | null;
  readonly executionState?: string | null;
  readonly reason?: string | null;
};

export type LocalAiMemoryGraphInput = {
  readonly custody?: string | null;
  readonly capabilityStatus?: string | null;
  readonly generatedAt?: string | null;
  readonly returnedNodeCount: number;
  readonly returnedEdgeCount: number;
  readonly omittedEdgeCount: number;
  readonly degradedReasons: readonly string[];
  readonly evidenceReferenceIds: readonly string[];
};

export type LocalAiRemoteAssistantBoundaryInput = {
  readonly eventId?: string | null;
  readonly sentAt?: string | null;
  readonly requestId?: string | null;
  readonly state?: string | null;
  readonly provider?: string | null;
  readonly adapterBoundary?: string | null;
  readonly policyReadiness?: string | null;
  readonly custody?: string | null;
  readonly deletedEvidence?: string | null;
  readonly privacyMode?: string | null;
  readonly evidenceReferences?: string | null;
  readonly rowCount?: string | null;
};

const NOT_REPORTED = 'not-reported';
const SUMMARY_PRODUCT_CLAIM = 'no-model-quality-or-enforcement-claim';
const MEMORY_GRAPH_PRODUCT_CLAIM = 'source-cited-memory-graph-read-model-only';
const HOUSEHOLD_JOB_PRODUCT_CLAIM = 'worker-only-child-agent-authority';
const REMOTE_ASSISTANT_PRODUCT_CLAIM = 'remote-assistant-report-only-local-policy-authority';

export function localAiRuntimePanelTemplate(
  input: LocalAiRuntimePanelInput = {}
): LocalAiRuntimePanelTemplate {
  const cards = [
    runtimeStatusCard(input.runtimeStatus ?? null),
    householdJobCard(input.householdJob ?? null),
    memoryGraphCard(input.memoryGraph ?? null),
    remoteAssistantBoundaryCard(input.remoteAssistantBoundary ?? null),
  ].filter((card): card is LocalAiRuntimePanelCardTemplate => card !== null);

  return {
    summaryStatus: cards.length === 0 ? NOT_REPORTED : 'reported',
    summaryReadModelRows: String(cards.length),
    summaryProductClaim: SUMMARY_PRODUCT_CLAIM,
    cards,
  };
}

function runtimeStatusCard(input: LocalAiRuntimeStatusInput | null): LocalAiRuntimePanelCardTemplate | null {
  return input === null
    ? null
    : {
        kind: 'runtime-status',
        details: [
          detail('eventId', input.eventId),
          detail('sentAt', input.sentAt),
          detail('runtimeReference', input.runtimeReference),
          detail('provider', input.provider),
          detail('model', input.model),
          detail('loadState', input.loadState),
          detail('capability', input.capability),
          detail('resourceClass', input.resourceClass),
          detail('degradedState', input.degradedState),
          detail('privacyMode', input.privacyMode),
          detail('executionState', input.executionState),
          detail('reason', input.reason),
        ],
      };
}

function householdJobCard(input: LocalAiHouseholdJobInput | null): LocalAiRuntimePanelCardTemplate | null {
  return input === null
    ? null
    : {
        kind: 'household-job',
        details: [
          detail('eventId', input.eventId),
          detail('sentAt', input.sentAt),
          detail('requestId', input.requestId),
          detail('status', input.status),
          detail('state', input.state),
          detail('provider', input.provider),
          detail('providerSource', input.providerSource),
          detail('capability', input.capability),
          detail('resourceClass', input.resourceClass),
          detail('loadState', input.loadState),
          detail('privacyMode', input.privacyMode),
          detail('custody', input.custody),
          detail('policyReadiness', input.policyReadiness),
          detail('adapterBoundary', input.adapterBoundary),
          detail('requestId', input.leaseId),
          detail('lastChecked', input.lastChecked),
          detail('lastObserved', input.lastObserved),
          detail('decisionSource', input.decisionSource),
          detail('executionState', input.executionState),
          detail('reason', input.reason),
          detail('productClaim', HOUSEHOLD_JOB_PRODUCT_CLAIM),
        ],
      };
}

function memoryGraphCard(input: LocalAiMemoryGraphInput | null): LocalAiRuntimePanelCardTemplate | null {
  return input === null
    ? null
    : {
        kind: 'memory-graph',
        details: [
          detail('custody', input.custody),
          detail('capability', input.capabilityStatus),
          detail('generatedAt', input.generatedAt),
          detail('graphNodes', String(input.returnedNodeCount)),
          detail('graphEdges', String(input.returnedEdgeCount)),
          detail('graphOmittedEdges', String(input.omittedEdgeCount)),
          detail('evidenceReferences', uniqueList(input.evidenceReferenceIds)),
          detail('degradedState', commaList(input.degradedReasons)),
          detail('productClaim', MEMORY_GRAPH_PRODUCT_CLAIM),
        ],
      };
}

function remoteAssistantBoundaryCard(
  input: LocalAiRemoteAssistantBoundaryInput | null
): LocalAiRuntimePanelCardTemplate | null {
  return input === null
    ? null
    : {
        kind: 'remote-assistant-boundary',
        details: [
          detail('eventId', input.eventId),
          detail('sentAt', input.sentAt),
          detail('requestId', input.requestId),
          detail('state', input.state),
          detail('provider', input.provider),
          detail('adapterBoundary', input.adapterBoundary),
          detail('policyReadiness', input.policyReadiness),
          detail('custody', input.custody),
          detail('deletedEvidence', input.deletedEvidence),
          detail('privacyMode', input.privacyMode),
          detail('evidenceReferences', input.evidenceReferences),
          detail('rowCount', input.rowCount),
          detail('productClaim', REMOTE_ASSISTANT_PRODUCT_CLAIM),
        ],
      };
}

function detail(fieldKey: LocalAiRuntimePanelFieldKey, value: string | null | undefined): LocalAiRuntimePanelDetailTemplate {
  return {
    fieldKey,
    value: normalizeValue(value),
  };
}

function normalizeValue(value: string | null | undefined): string {
  return value === undefined || value === null || value.trim().length === 0 ? NOT_REPORTED : value;
}

function commaList(values: readonly string[]): string {
  const normalized = values.map((value) => value.trim()).filter((value) => value.length > 0);
  return normalized.length === 0 ? NOT_REPORTED : normalized.join(',');
}

function uniqueList(values: readonly string[]): string {
  const normalized: string[] = [];
  for (const value of values) {
    const candidate = value.trim();
    if (candidate.length === 0 || normalized.includes(candidate)) {
      continue;
    }
    normalized.push(candidate);
  }
  return normalized.length === 0 ? NOT_REPORTED : normalized.join(',');
}
"#;
