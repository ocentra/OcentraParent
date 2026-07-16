/* generated from crates/parent-runtime-core/src/local_ai_runtime_panel.rs */

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

export function localAiRuntimePanelTemplate(input: LocalAiRuntimePanelInput = {}): LocalAiRuntimePanelTemplate {
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

function detail(
  fieldKey: LocalAiRuntimePanelFieldKey,
  value: string | null | undefined
): LocalAiRuntimePanelDetailTemplate {
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
