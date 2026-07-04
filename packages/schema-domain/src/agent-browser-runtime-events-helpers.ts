import type {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventChainEntry,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
} from './agent-browser-runtime-events';
import { ChildDomainRuntimeEventTypeLiteral } from './child-domain-runtime-events';

export type AgentBrowserRuntimeActionIntentCandidate = {
  readonly eventRef: string;
  readonly policyPreviewId: string;
  readonly assistantActionIntentId: string;
  readonly sourceRef: string;
  readonly evidenceRef: string;
  readonly observedAt: string;
};

type BrowserRuntimePayloadCandidate = {
  readonly phase: AgentBrowserRuntimePhase;
  readonly sourceRef: string;
  readonly evidenceRef: string;
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
  readonly degradedReason: string | null;
  readonly dryRun: boolean;
  readonly policyAuthority: boolean;
  readonly policyPreviewId: string | null;
  readonly assistantActionIntentId: string | null;
  readonly adapterDispatchClaimed: boolean;
  readonly exactUrlClaimed: boolean;
  readonly interventionCommandAllowed: boolean;
  readonly interventionCommandRef: string | null;
  readonly interventionResultRef: string | null;
  readonly observedAt: string;
};

type BrowserRuntimeStreamCandidate = {
  readonly actionIntentCandidates: number;
  readonly actionIntentHandoffCandidates: number;
  readonly actionIntentHandoffOutboxRefs: readonly string[];
  readonly actionIntentHandoffRefs: readonly string[];
  readonly actionIntentChildAcceptedRows: number;
  readonly actionIntentChildCommandRefs: readonly string[];
  readonly actionIntentChildAcceptedEventRefs: readonly string[];
  readonly actionIntentParentReadModelRefs: readonly string[];
  readonly socialProviderReceiptBoundaryRows: number;
  readonly socialProviderDispatchRequiredRows: number;
  readonly socialProviderManualReceiptRequiredRows: number;
  readonly socialProviderAttemptRefs: readonly string[];
  readonly socialProviderReceiptProofRefs: readonly string[];
  readonly socialProviderDurableRows: number;
  readonly socialProviderDurableResultRefs: readonly string[];
  readonly socialProviderDurableStoreRefs: readonly string[];
  readonly socialProviderReadModelRefs: readonly string[];
  readonly socialProviderSupportStatusRefs: readonly string[];
  readonly entries: readonly AgentBrowserRuntimeEventChainEntry[];
};

const AgentBrowserRuntimeEventTypeByPhase: Record<AgentBrowserRuntimePhase, AgentBrowserRuntimeEventType> = {
  EvidenceObserved: 'browser.evidence.observed',
  EvidenceJournaled: 'browser.evidence.journaled',
  AiAnalysisRequested: ChildDomainRuntimeEventTypeLiteral.BrowserAiAnalysisRequested,
  AiAnalysisCompleted: 'browser.ai.analysis.completed',
  PolicyEvaluationRequested: ChildDomainRuntimeEventTypeLiteral.BrowserPolicyEvaluationRequested,
  PolicyDecisionCompleted: 'browser.policy.decision.completed',
  InterventionCommandIssued: 'browser.intervention.command.issued',
  InterventionResultObserved: 'browser.intervention.result.observed',
  AuditEntryCommitted: 'browser.audit.entry.committed',
  ReadModelProjected: 'browser.read-model.projected',
};

export function actionIntentCandidatesFromEntries(
  entries: readonly AgentBrowserRuntimeEventChainEntry[]
): AgentBrowserRuntimeActionIntentCandidate[] {
  return entries.flatMap((entry) => actionIntentCandidateFromEntry(entry));
}

export function browserRuntimeSocialProviderReceiptStateIsHonest(stream: BrowserRuntimeStreamCandidate): boolean {
  if (
    stream.socialProviderReceiptBoundaryRows !==
    stream.socialProviderDispatchRequiredRows + stream.socialProviderManualReceiptRequiredRows
  ) {
    return false;
  }
  if (stream.socialProviderDispatchRequiredRows === 0) {
    return socialProviderReceiptRefsAreEmpty(stream);
  }
  return (
    stream.socialProviderAttemptRefs.length === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderReceiptProofRefs.length === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderDurableRows === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderDurableResultRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderDurableStoreRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderReadModelRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderSupportStatusRefs.length === stream.socialProviderDurableRows
  );
}

export function browserRuntimeActionIntentChildStatusIsHonest(stream: {
  readonly actionIntentChildAcceptedRows: number;
  readonly actionIntentChildCommandRefs: readonly string[];
  readonly actionIntentChildAcceptedEventRefs: readonly string[];
  readonly actionIntentParentReadModelRefs: readonly string[];
}): boolean {
  return (
    stream.actionIntentChildCommandRefs.length === stream.actionIntentChildAcceptedRows &&
    stream.actionIntentChildAcceptedEventRefs.length === stream.actionIntentChildAcceptedRows &&
    stream.actionIntentParentReadModelRefs.length === stream.actionIntentChildAcceptedRows
  );
}

export function browserRuntimePayloadIsHonest(payload: BrowserRuntimePayloadCandidate): boolean {
  if (!browserRuntimeContextSupportsExactUrl(payload) && payload.exactUrlClaimed) {
    return false;
  }
  if (!browserRuntimeUnavailableContextHasReason(payload)) {
    return false;
  }
  if (!payload.exactUrlClaimed && payload.interventionCommandAllowed) {
    return false;
  }
  if (!browserRuntimeDryRunHasNoDispatch(payload)) {
    return false;
  }
  if (payload.adapterDispatchClaimed && !payload.interventionCommandAllowed) {
    return false;
  }
  if (!payload.interventionCommandAllowed) {
    return payload.interventionCommandRef === null && payload.interventionResultRef === null;
  }
  return payload.interventionCommandRef !== null && payload.adapterDispatchClaimed;
}

export function phaseMatchesEventType(phase: AgentBrowserRuntimePhase, eventType: AgentBrowserRuntimeEventType): boolean {
  return eventType === phaseToEventType(phase);
}

function actionIntentCandidateFromEntry(
  entry: AgentBrowserRuntimeEventChainEntry
): AgentBrowserRuntimeActionIntentCandidate[] {
  const payload = entry.payload;
  if (
    payload.phase !== 'PolicyDecisionCompleted' ||
    !payload.dryRun ||
    !payload.policyAuthority ||
    payload.policyPreviewId === null ||
    payload.assistantActionIntentId === null
  ) {
    return [];
  }

  return [
    {
      eventRef: entry.eventRef,
      policyPreviewId: payload.policyPreviewId,
      assistantActionIntentId: payload.assistantActionIntentId,
      sourceRef: payload.sourceRef,
      evidenceRef: payload.evidenceRef,
      observedAt: payload.observedAt,
    },
  ];
}

function socialProviderReceiptRefsAreEmpty(stream: BrowserRuntimeStreamCandidate): boolean {
  return (
    stream.socialProviderAttemptRefs.length === 0 &&
    stream.socialProviderReceiptProofRefs.length === 0 &&
    stream.socialProviderDurableRows === 0 &&
    stream.socialProviderDurableResultRefs.length === 0 &&
    stream.socialProviderDurableStoreRefs.length === 0 &&
    stream.socialProviderReadModelRefs.length === 0 &&
    stream.socialProviderSupportStatusRefs.length === 0
  );
}

function browserRuntimeContextSupportsExactUrl(payload: {
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
}): boolean {
  const capabilityAllowsExactUrl =
    payload.capabilityStatus === 'available' || payload.capabilityStatus === 'tab-list-only';
  const queryAllowsExactUrl = payload.queryVisibility === 'live-local' || payload.queryVisibility === 'live-lan';
  return capabilityAllowsExactUrl && queryAllowsExactUrl && payload.custodyLabel !== 'unavailable';
}

function browserRuntimeUnavailableContextHasReason(payload: {
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
  readonly degradedReason: string | null;
}): boolean {
  if (
    payload.queryVisibility !== 'unavailable' &&
    payload.capabilityStatus !== 'bridge-missing' &&
    payload.capabilityStatus !== 'stale' &&
    payload.capabilityStatus !== 'adapter-error'
  ) {
    return true;
  }
  return payload.degradedReason !== null;
}

function browserRuntimeDryRunHasNoDispatch(payload: {
  readonly dryRun: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly interventionCommandAllowed: boolean;
  readonly interventionCommandRef: string | null;
  readonly interventionResultRef: string | null;
}): boolean {
  if (!payload.dryRun) {
    return true;
  }

  return (
    !payload.adapterDispatchClaimed &&
    !payload.interventionCommandAllowed &&
    payload.interventionCommandRef === null &&
    payload.interventionResultRef === null
  );
}

function phaseToEventType(phase: AgentBrowserRuntimePhase): AgentBrowserRuntimeEventType {
  return AgentBrowserRuntimeEventTypeByPhase[phase];
}
