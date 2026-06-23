import type { ReactElement } from 'react';
import {
  type PortalDetailValue,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  networkEvidenceDrawerSummary,
  type NetworkEvidenceDrawerSummary,
} from '@ocentra-parent/portal-domain/network-evidence-drawer';
import {
  isPortalInlineNetworkEvidenceDrawerRoute,
  isPortalNetworkEvidenceDrawerRoute,
} from '@ocentra-parent/portal-domain/routes';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderNetworkEvidenceDrawerRoute(route: PortalRouteValue): boolean {
  return isPortalNetworkEvidenceDrawerRoute(route);
}

export function NetworkEvidenceDrawerRoutePanel({
  liveActivity,
  route,
}: {
  readonly liveActivity: PortalLiveActivityState;
  readonly route: PortalRouteValue;
}): ReactElement {
  const summary = networkEvidenceDrawerSummary(liveActivity.networkFlowReadModel, {
    networkFlowEventPayload: liveActivity.networkFlowEvent?.payload ?? null,
    policyPreviewReadModel: liveActivity.policyPreviewReadModel,
    networkRuntimeEventChainStream: liveActivity.networkRuntimeEventChainStream,
  });
  const inlineOnActivityRoute = isPortalInlineNetworkEvidenceDrawerRoute(route);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.NetworkFlow)}
      className={PortalDom.Classes.TrackingStatusOverlay}
      style={inlineOnActivityRoute ? inlineActivityRoutePanelStyle : undefined}
    >
      <div
        className={PortalDom.Classes.TrackingStatusOverlayContent}
        style={inlineOnActivityRoute ? inlineActivityRoutePanelContentStyle : undefined}
      >
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</p>
          <h2>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</h2>
          {liveActivity.networkFlowReadModel === null ? (
            <p>{resolvePortalDevText(PortalDevTextToken.NoNetworkFlow)}</p>
          ) : null}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <NetworkEvidenceDrawerCard summary={summary} />
          <NetworkEvidenceUnsupportedClaimCard summary={summary} />
        </div>
      </div>
    </section>
  );
}

function NetworkEvidenceDrawerCard({ summary }: { readonly summary: NetworkEvidenceDrawerSummary }): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.EventId} value={summary.evidenceId} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastObserved} value={summary.observedAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.FirstObserved} value={summary.firstSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastChecked} value={summary.lastSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Device} value={summary.deviceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Profile} value={summary.childProfileRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Source} value={summary.sourceAdapter} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Capability} value={summary.sourceQuality} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PlatformState} value={summary.platformState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReadModelRows} value={summary.readModelRows} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Destination} value={summary.remoteEndpoint} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.NetworkProtocol} value={summary.protocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.TcpState} value={summary.applicationProtocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Process} value={summary.processRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Domain} value={summary.domainEvidenceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Connections} value={summary.byteSummary} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EvidenceReferences} value={summary.evidenceReferences} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReasonCodes} value={summary.uncertaintyReasonCodes} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Custody} value={summary.custody} />
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
        <NetworkEvidenceDrawerDetail label={PortalDetails.AnalyzerAlerts} value={summary.analyzerAlertRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.DetectionResults} value={summary.detectionResultRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LocalAiResult} value={summary.aiAuditRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.RiskBudget} value={summary.riskBudgetRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PolicyPreview} value={summary.policyDecisionRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EnforcementHandoff} value={summary.interventionResultRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.DeletedEvidence} value={summary.retentionState} />
        <NetworkEvidenceDrawerDetail
          label={PortalDetails.DeletedEvidenceReferences}
          value={summary.deletedEvidenceReferences}
        />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PerformanceState} value={summary.degradedState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Level} value={summary.evidenceGrade} />
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

const inlineActivityRoutePanelStyle = {
  position: 'relative',
  inset: 'auto',
  zIndex: 'auto',
  marginTop: '16px',
  overflow: 'visible',
} as const;

const inlineActivityRoutePanelContentStyle = {
  height: 'auto',
} as const;
