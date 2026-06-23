import {
  deriveAgentBrowserRuntimeSocialProviderReceiptStatus,
  type AgentBrowserRuntimeSocialProviderReceiptStatus,
} from '@ocentra-parent/agent-protocol-domain/browser-runtime-events';
import { type AgentBrowserRuntimeEventChainStream } from '@ocentra-parent/schema-domain/agent-browser-runtime-events';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { decodePortalDetailValue, type PortalDetailValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDetails } from './details';

export type BrowserSocialProviderReceiptIngestionReadinessStatusDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type BrowserSocialProviderReceiptIngestionReadinessStatusIntent = {
  readonly title: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly details: readonly BrowserSocialProviderReceiptIngestionReadinessStatusDetail[];
  readonly productClaim: DisplayText;
};

const ReceiptIngestionReadinessValues = {
  IngestionContractRequired: 'ingestion-contract-required',
  ManualReceiptRequired: 'manual-receipt-required',
  NoProviderReceiptObserved: '0 provider receipts observed',
  NotClaimed: 'not-claimed',
  NotObserved: 'not-observed',
  Ready: 'ready',
  ReadinessRows: ' readiness rows',
  RefsSeparator: ', ',
  Unavailable: 'unavailable',
} as const;

const ReceiptIngestionReadinessCopy = {
  ProductClaim: decodeDisplayText(
    'Browser runtime social provider receipt ingestion readiness status only; provider delivery, receipt ingestion runtime, webhook runtime, credentials, observed provider receipts, parent notification delivery, report delivery, final policy execution, connector/native runtime, browser mutation, child intervention, unmanaged exact URL support, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social provider receipt ingestion readiness'),
} as const;

export function createBrowserSocialProviderReceiptIngestionReadinessStatusIntent(
  stream: AgentBrowserRuntimeEventChainStream
): BrowserSocialProviderReceiptIngestionReadinessStatusIntent {
  return populatedIntent(deriveAgentBrowserRuntimeSocialProviderReceiptStatus(stream));
}

function populatedIntent(
  status: AgentBrowserRuntimeSocialProviderReceiptStatus
): BrowserSocialProviderReceiptIngestionReadinessStatusIntent {
  const readinessRows = status.providerDispatchRequiredRows + status.manualReceiptRequiredRows;

  return {
    title: ReceiptIngestionReadinessCopy.Title,
    state: detailValue(
      readinessRows > 0 ? ReceiptIngestionReadinessValues.Ready : ReceiptIngestionReadinessValues.Unavailable
    ),
    summary: detailValue(String(readinessRows) + ReceiptIngestionReadinessValues.ReadinessRows),
    details: [
      detail(PortalDetails.RowsReturned, readinessRows),
      detail(PortalDetails.PolicyReadiness, readinessStateSummary(status)),
      detail(PortalDetails.Capability, status.manualReceiptRequiredRows),
      detail(PortalDetails.EvidenceReferences, refsValue(status.providerAttemptRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(status.providerReceiptProofRefs)),
      detail(PortalDetails.PolicyEvaluation, refsValue(status.durableStoreRefs)),
      detail(PortalDetails.Provider, refsValue(status.supportStatusRefs)),
      detail(PortalDetails.Status, ReceiptIngestionReadinessValues.NoProviderReceiptObserved),
      detail(PortalDetails.ProductClaim, ReceiptIngestionReadinessCopy.ProductClaim),
      detail(PortalDetails.Status, noClaimSummary(status)),
    ],
    productClaim: ReceiptIngestionReadinessCopy.ProductClaim,
  };
}

function readinessStateSummary(status: AgentBrowserRuntimeSocialProviderReceiptStatus): PortalDetailValue {
  if (status.providerDispatchRequiredRows > 0) {
    return detailValue(ReceiptIngestionReadinessValues.IngestionContractRequired);
  }
  if (status.manualReceiptRequiredRows > 0) {
    return detailValue(ReceiptIngestionReadinessValues.ManualReceiptRequired);
  }
  return detailValue(ReceiptIngestionReadinessValues.Unavailable);
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
    claims.every((claim) => !claim) ? ReceiptIngestionReadinessValues.NotClaimed : ReceiptIngestionReadinessValues.Ready
  );
}

function detail(label: DisplayText, value: unknown): BrowserSocialProviderReceiptIngestionReadinessStatusDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0
      ? refs.join(ReceiptIngestionReadinessValues.RefsSeparator)
      : ReceiptIngestionReadinessValues.NotObserved
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : String(value);
  return decodePortalDetailValue(text);
}
