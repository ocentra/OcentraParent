import {
  deriveAgentBrowserRuntimeSocialProviderReceiptStatus,
  type AgentBrowserRuntimeEventChainStream,
  type AgentBrowserRuntimeSocialProviderReceiptStatus,
} from '@ocentra-parent/agent-protocol-domain/browser-runtime-events';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type BrowserSocialProviderReceiptStreamStatusDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type BrowserSocialProviderReceiptStreamStatusIntent = {
  readonly title: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly details: readonly BrowserSocialProviderReceiptStreamStatusDetail[];
  readonly productClaim: DisplayText;
};

const ReceiptStreamStatusValues = {
  NotClaimed: 'not-claimed',
  NotObserved: 'not-observed',
  ReceiptBoundaryRows: ' receipt boundary rows',
  Ready: 'ready',
  RefsSeparator: ', ',
  Unavailable: 'unavailable',
} as const;

const ReceiptStreamStatusCopy = {
  ProductClaim: decodeDisplayText(
    'Browser runtime social provider receipt stream status only; provider delivery, receipt ingestion, parent notification delivery, report delivery, final policy execution, connector/native runtime, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social provider receipt stream status'),
} as const;

export function createBrowserSocialProviderReceiptStreamStatusIntent(
  stream: AgentBrowserRuntimeEventChainStream
): BrowserSocialProviderReceiptStreamStatusIntent {
  return populatedIntent(deriveAgentBrowserRuntimeSocialProviderReceiptStatus(stream));
}

function populatedIntent(
  status: AgentBrowserRuntimeSocialProviderReceiptStatus
): BrowserSocialProviderReceiptStreamStatusIntent {
  return {
    title: ReceiptStreamStatusCopy.Title,
    state: detailValue(
      status.receiptBoundaryRows > 0 ? ReceiptStreamStatusValues.Ready : ReceiptStreamStatusValues.Unavailable
    ),
    summary: detailValue(String(status.receiptBoundaryRows) + ReceiptStreamStatusValues.ReceiptBoundaryRows),
    details: [
      detail(PortalDetails.RowsReturned, status.receiptBoundaryRows),
      detail(PortalDetails.Status, status.providerDispatchRequiredRows),
      detail(PortalDetails.Capability, status.manualReceiptRequiredRows),
      detail(PortalDetails.EvidenceReferences, refsValue(status.providerAttemptRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(status.providerReceiptProofRefs)),
      detail(PortalDetails.PolicyEvaluation, refsValue(status.readModelRefs)),
      detail(PortalDetails.ProductClaim, ReceiptStreamStatusCopy.ProductClaim),
      detail(PortalDetails.Status, noClaimSummary(status)),
    ],
    productClaim: ReceiptStreamStatusCopy.ProductClaim,
  };
}

function noClaimSummary(status: AgentBrowserRuntimeSocialProviderReceiptStatus): PortalDetailValue {
  const claims = [
    status.providerDeliveryClaimed,
    status.receiptIngestionClaimed,
    status.parentNotificationDeliveryClaimed,
    status.reportDeliveryClaimed,
    status.finalPolicyExecutionClaimed,
    status.connectorNativeRuntimeClaimed,
    status.enforcementClaimed,
  ];
  return detailValue(
    claims.every((claim) => !claim) ? ReceiptStreamStatusValues.NotClaimed : ReceiptStreamStatusValues.Ready
  );
}

function detail(label: DisplayText, value: unknown): BrowserSocialProviderReceiptStreamStatusDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0 ? refs.join(ReceiptStreamStatusValues.RefsSeparator) : ReceiptStreamStatusValues.NotObserved
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : String(value);
  return decodePortalDetailValue(text);
}
