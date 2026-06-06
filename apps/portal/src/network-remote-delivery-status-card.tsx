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
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.RuntimeReference}
          value={summary.remoteDeliveryStatusRef}
        />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.BrokerDelivery} value={summary.remoteBrokerStatus} />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.FamilyHubDelivery}
          value={summary.remoteFamilyHubStatus}
        />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.Custody} value={summary.remoteCustodyProofRef} />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.ParentRuleContextReferences}
          value={summary.remoteAuthRefs}
        />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.RemoteTransport} value={summary.remoteTransportRefs} />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.RemoteLifecycle} value={summary.remoteLifecycleRefs} />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.MissingProof}
          value={summary.remoteMissingArtifactCounts}
        />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.Events} value={summary.remoteAcceptedEventTypeCount} />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.LocalQueue} value={summary.remoteLocalQueueProof} />
        <NetworkRemoteDeliveryStatusDetail label={PortalDetails.DuplicateEvents} value={summary.remoteDuplicateProof} />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.DeletedEvidence}
          value={summary.remoteDeadLetterCount}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.Transport}
          value={summary.remoteExternalTransportImplemented}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.ChildDelivery}
          value={summary.remoteFamilyHubDeliveryImplemented}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.EventHistory}
          value={summary.remoteCrossProcessReplayImplemented}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.RemoteSync}
          value={summary.remoteRetentionDeleteExportImplemented}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.PolicyAuthority}
          value={summary.remotePolicyAuthority}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.SideEffectAuthority}
          value={summary.remoteSideEffectAuthority}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.EnforcementCommandPublished}
          value={summary.remoteEnforcementCommandEventCount}
        />
        <NetworkRemoteDeliveryStatusDetail
          label={PortalDetails.AdapterDispatch}
          value={summary.remoteAdapterActionExecutedCount}
        />
      </dl>
    </article>
  );
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
