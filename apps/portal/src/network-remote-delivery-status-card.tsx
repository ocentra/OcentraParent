import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import {
  emptyNetworkProductReadinessStatusSummary,
  type NetworkProductReadinessStatusSummary,
} from './network-product-readiness-status';

export function NetworkRemoteDeliveryStatusCard({
  status,
}: {
  readonly status: NetworkProductReadinessStatusSummary | null;
}): ReactElement {
  const summary = status ?? emptyNetworkProductReadinessStatusSummary();
  return (
    <article className={networkRemoteDeliveryStatusCardClassName()}>
      <h2>{PortalDetails.RemoteDeliveryStatus}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkRemoteDeliveryStatusDetails summary={summary} />
      </dl>
    </article>
  );
}

type NetworkRemoteDeliveryStatusDetailEntry = {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
};

function NetworkRemoteDeliveryStatusDetails({
  summary,
}: {
  readonly summary: NetworkProductReadinessStatusSummary;
}): ReactElement {
  return (
    <>
      {networkRemoteDeliveryStatusDetails(summary).map((detail) => (
        <NetworkRemoteDeliveryStatusDetail key={detail.label} label={detail.label} value={detail.value} />
      ))}
    </>
  );
}

function networkRemoteDeliveryStatusDetails(
  summary: NetworkProductReadinessStatusSummary
): readonly NetworkRemoteDeliveryStatusDetailEntry[] {
  return [...remoteDeliveryEvidenceDetails(summary), ...remoteDeliveryFalseClaimDetails(summary)];
}

function remoteDeliveryEvidenceDetails(
  summary: NetworkProductReadinessStatusSummary
): readonly NetworkRemoteDeliveryStatusDetailEntry[] {
  return [
    { label: PortalDetails.RuntimeReference, value: summary.remoteDeliveryStatusRef },
    { label: PortalDetails.BrokerDelivery, value: summary.remoteBrokerStatus },
    { label: PortalDetails.FamilyHubDelivery, value: summary.remoteFamilyHubStatus },
    { label: PortalDetails.Custody, value: summary.remoteCustodyProofRef },
    { label: PortalDetails.ParentRuleContextReferences, value: summary.remoteAuthRefs },
    { label: PortalDetails.RemoteTransport, value: summary.remoteTransportRefs },
    { label: PortalDetails.RemoteLifecycle, value: summary.remoteLifecycleRefs },
    { label: PortalDetails.MissingProof, value: summary.remoteMissingArtifactCounts },
    { label: PortalDetails.Events, value: summary.remoteAcceptedEventTypeCount },
    { label: PortalDetails.LocalQueue, value: summary.remoteLocalQueueProof },
    { label: PortalDetails.DuplicateEvents, value: summary.remoteDuplicateProof },
    { label: PortalDetails.DeletedEvidence, value: summary.remoteDeadLetterCount },
    { label: PortalDetails.Audit, value: summary.remoteLifecycleFollowupRef },
    { label: PortalDetails.ManualRequired, value: summary.remoteLifecycleManualRequired },
  ];
}

function remoteDeliveryFalseClaimDetails(
  summary: NetworkProductReadinessStatusSummary
): readonly NetworkRemoteDeliveryStatusDetailEntry[] {
  return [
    { label: PortalDetails.Transport, value: summary.remoteExternalTransportImplemented },
    { label: PortalDetails.ChildDelivery, value: summary.remoteFamilyHubDeliveryImplemented },
    { label: PortalDetails.EventHistory, value: summary.remoteCrossProcessReplayImplemented },
    { label: PortalDetails.RemoteSync, value: summary.remoteRetentionDeleteExportImplemented },
    { label: PortalDetails.PolicyAuthority, value: summary.remotePolicyAuthority },
    { label: PortalDetails.SideEffectAuthority, value: summary.remoteSideEffectAuthority },
    { label: PortalDetails.EnforcementCommandPublished, value: summary.remoteEnforcementCommandEventCount },
    { label: PortalDetails.AdapterDispatch, value: summary.remoteAdapterActionExecutedCount },
  ];
}

function NetworkRemoteDeliveryStatusDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function networkRemoteDeliveryStatusCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
