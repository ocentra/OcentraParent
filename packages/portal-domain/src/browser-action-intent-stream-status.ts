import {
  deriveAgentBrowserRuntimeActionIntentStatus,
  type AgentBrowserRuntimeActionIntentStatus,
  type AgentBrowserRuntimeEventChainStream,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type BrowserActionIntentStreamStatusDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type BrowserActionIntentStreamStatusIntent = {
  readonly title: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly details: readonly BrowserActionIntentStreamStatusDetail[];
  readonly productClaim: DisplayText;
};

const ActionIntentStreamStatusValues = {
  ActionCandidates: ' action candidates',
  NoExecutionClaimed: 'no-execution-claimed',
  NotClaimed: 'not-claimed',
  NotObserved: 'not-observed',
  Ready: 'ready',
  RefsSeparator: ', ',
  Unavailable: 'unavailable',
} as const;

const ActionIntentStreamStatusCopy = {
  ProductClaim: decodeDisplayText(
    'Browser runtime action-intent stream status only; local outbox handoff, child accepted refs, and parent read-model refs may be visible, while adapter dispatch, child intervention execution, browser mutation, final policy execution, unmanaged exact URL support, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Browser action-intent stream status'),
} as const;

export function createBrowserActionIntentStreamStatusIntent(
  stream: AgentBrowserRuntimeEventChainStream
): BrowserActionIntentStreamStatusIntent {
  return populatedIntent(deriveAgentBrowserRuntimeActionIntentStatus(stream));
}

function populatedIntent(status: AgentBrowserRuntimeActionIntentStatus): BrowserActionIntentStreamStatusIntent {
  return {
    title: ActionIntentStreamStatusCopy.Title,
    state: detailValue(
      status.candidateCount > 0 ? ActionIntentStreamStatusValues.Ready : ActionIntentStreamStatusValues.Unavailable
    ),
    summary: detailValue(String(status.candidateCount) + ActionIntentStreamStatusValues.ActionCandidates),
    details: [
      detail(PortalDetails.RowsReturned, status.candidateCount),
      detail(PortalDetails.Status, String(status.candidateCount) + ActionIntentStreamStatusValues.ActionCandidates),
      detail(PortalDetails.PolicyReadiness, status.handoffCandidateCount),
      detail(PortalDetails.EvidenceReferences, refsValue(status.handoffOutboxRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(status.handoffRefs)),
      detail(PortalDetails.Capability, status.childAcceptedRows),
      detail(PortalDetails.PolicyEvaluation, refsValue(status.parentReadModelRefs)),
      detail(PortalDetails.Status, refsValue(status.childCommandRefs)),
      detail(PortalDetails.Status, refsValue(status.childAcceptedEventRefs)),
      detail(PortalDetails.ProductClaim, ActionIntentStreamStatusCopy.ProductClaim),
      detail(PortalDetails.Status, noClaimSummary(status)),
    ],
    productClaim: ActionIntentStreamStatusCopy.ProductClaim,
  };
}

function noClaimSummary(status: AgentBrowserRuntimeActionIntentStatus): PortalDetailValue {
  const claims = [
    status.dispatchAttemptCount,
    status.adapterExecutionCount,
    status.childInterventionExecutionCount,
    status.enforcementExecutionCount,
  ];
  return detailValue(
    claims.every((claim) => claim === 0)
      ? ActionIntentStreamStatusValues.NotClaimed
      : ActionIntentStreamStatusValues.NoExecutionClaimed
  );
}

function detail(label: DisplayText, value: unknown): BrowserActionIntentStreamStatusDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0
      ? refs.join(ActionIntentStreamStatusValues.RefsSeparator)
      : ActionIntentStreamStatusValues.NotObserved
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : String(value);
  return decodePortalDetailValue(text);
}
