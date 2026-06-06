import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  type PortalDetailValue,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalLiveActivityState } from './live-activity-state';
import {
  emptyNetworkAdapterCapabilityStatusSummary,
  type NetworkAdapterCapabilityStatusSummary,
} from './network-adapter-capability-status';
import { networkEvidenceDrawerSummary, type NetworkEvidenceDrawerSummary } from './network-evidence-drawer';

export function shouldRenderNetworkEvidenceDrawerRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Activity;
}

export function NetworkEvidenceDrawerRoutePanel({
  liveActivity,
}: {
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const summary = networkEvidenceDrawerSummary(
    liveActivity.networkFlowReadModel,
    liveActivity.networkRuntimeEventChain,
    liveActivity.networkFlowDigest
  );
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.NetworkFlow)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalText.Resolve(PortalTextToken.NetworkFlow)}</p>
          <h2>{PortalText.Resolve(PortalTextToken.NetworkFlow)}</h2>
          {liveActivity.networkFlowReadModel === null ? (
            <p>{PortalText.Resolve(PortalTextToken.NoNetworkFlow)}</p>
          ) : null}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <NetworkEvidenceDrawerCard summary={summary} />
          <NetworkEvidenceUnsupportedClaimCard summary={summary} />
          <NetworkAdapterCapabilityStatusCard status={liveActivity.networkAdapterCapabilityStatus} />
        </div>
      </div>
    </section>
  );
}

function NetworkEvidenceDrawerCard({ summary }: { readonly summary: NetworkEvidenceDrawerSummary }): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{PortalText.Resolve(PortalTextToken.NetworkFlow)}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.EventId} value={summary.evidenceId} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastObserved} value={summary.observedAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.FirstObserved} value={summary.firstSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastChecked} value={summary.lastSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Device} value={summary.deviceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Profile} value={summary.childProfileRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Source} value={summary.sourceAdapter} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Capability} value={summary.sourceQuality} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Destination} value={summary.remoteEndpoint} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.NetworkProtocol} value={summary.protocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.TcpState} value={summary.applicationProtocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Process} value={summary.processRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Domain} value={summary.domainEvidenceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Connections} value={summary.byteSummary} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EvidenceReferences} value={summary.evidenceReferences} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReasonCodes} value={summary.uncertaintyReasonCodes} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReasonCodes} value={summary.digestIndicators} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EvidenceReferences} value={summary.digestIndicatorEvidence} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Custody} value={summary.custody} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EventHistory} value={summary.eventHistoryRef} />
      </dl>
    </article>
  );
}

function NetworkEvidenceUnsupportedClaimCard({
  summary,
}: {
  readonly summary: NetworkEvidenceDrawerSummary;
}): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{PortalDetails.MissingProof}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.BrowserEvidence} value={summary.browserRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ExactUrlClaim} value={summary.exactUrlClaim} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LocalAiResult} value={summary.aiAuditRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Audit} value={summary.auditRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.UnknownState} value={summary.riskBudgetRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PolicyPreview} value={summary.policyDecisionRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EnforcementHandoff} value={summary.interventionResultRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.DeletedEvidence} value={summary.retentionState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Level} value={summary.evidenceGrade} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Confidence} value={summary.confidence} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ManualRequired} value={summary.manualRequiredState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.UnavailableState} value={summary.unavailableState} />
      </dl>
    </article>
  );
}

function NetworkAdapterCapabilityStatusCard({
  status,
}: {
  readonly status: NetworkAdapterCapabilityStatusSummary | null;
}): ReactElement {
  const summary = status ?? emptyNetworkAdapterCapabilityStatusSummary();
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{PortalDetails.AdapterBoundary}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.Source} value={summary.sourceReadModel} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastChecked} value={summary.generatedAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Capability} value={summary.observePolicyHandoff} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Platform} value={summary.platformMatrix} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PolicyPreview} value={summary.hostDomainManualGate} />
        <NetworkEvidenceDrawerDetail
          label={PortalDetails.EnforcementHandoff}
          value={summary.hostDomainArtifactStatus}
        />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ExactUrlClaim} value={summary.exactUrlCapability} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReasonCodes} value={summary.degradedState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ManualRequired} value={summary.manualProofRequirements} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.UnavailableState} value={summary.unavailableState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.MissingProof} value={summary.unsupportedState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.AdapterDispatch} value={summary.noClaimBoundary} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Audit} value={summary.proofArtifacts} />
      </dl>
    </article>
  );
}

function NetworkEvidenceDrawerDetail({
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

function networkEvidenceDrawerCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
