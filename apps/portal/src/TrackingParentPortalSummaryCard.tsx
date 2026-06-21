import type { ReactElement } from 'react';
import {
  PortalDom,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/portal-domain/routes';
import {
  trackingStatusLiveSummary,
  trackingStatusServiceDataCoverage,
} from '@ocentra-parent/portal-domain/tracking-status-panel';
import { type PortalDetailValue } from '@ocentra-parent/portal-domain/detail-values';
import {
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/tracking-status-proof-artifacts';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderTrackingParentPortalSummary(route: PortalRouteValue): boolean {
  return route === PortalRoute.Overview || route === PortalRoute.Devices;
}

export function TrackingParentPortalSummaryCard({
  liveActivity,
}: {
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const summary = trackingStatusLiveSummary(liveActivity);
  const coverage = trackingStatusServiceDataCoverage(liveActivity);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.TrackingStatusSurface)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalText.Resolve(PortalTextToken.TrackingFirstTarget)}</p>
          <h2>{summary.title}</h2>
          <p>{PortalText.Resolve(PortalTextToken.TrackingStatusSurfaceBody)}</p>
        </header>
        <div className={PortalDom.Classes.TrackingStatusOverlayGrid}>
          <TrackingParentPortalStatusCard
            details={[
              { label: PortalDetails.LoadState, value: summary.loadState },
              { label: PortalDetails.ProofTier, value: summary.proofTier },
              { label: PortalDetails.RowsReturned, value: summary.rowsReturned },
              { label: PortalDetails.LastObserved, value: summary.lastObserved },
              { label: PortalDetails.EventId, value: summary.eventId },
              { label: PortalDetails.Capability, value: summary.capability },
              { label: PortalDetails.Custody, value: summary.custody },
              { label: PortalDetails.EvidenceReferences, value: summary.evidenceReferences },
              { label: PortalDetails.ProductClaim, value: summary.productClaim },
            ]}
            title={summary.title}
          />
          <TrackingParentPortalStatusCard
            details={[
              { label: PortalDetails.LoadState, value: coverage.loadState },
              { label: PortalDetails.ProofTier, value: coverage.proofTier },
              { label: PortalDetails.RowsReturned, value: coverage.rowsReturned },
              { label: PortalDetails.HistoryVisibility, value: coverage.rowVisibility },
              { label: PortalDetails.LastObserved, value: coverage.lastObserved },
              { label: PortalDetails.Device, value: coverage.deviceCounts },
              { label: PortalDetails.Capability, value: coverage.capability },
              { label: PortalDetails.Custody, value: coverage.custody },
              { label: PortalDetails.ActivityKind, value: coverage.activityKinds },
              { label: PortalDetails.DeletedEvidence, value: coverage.deletedEvidence },
              { label: PortalDetails.ProductClaim, value: coverage.productClaim },
            ]}
            title={coverage.title}
          />
        </div>
      </div>
    </section>
  );
}

function TrackingParentPortalStatusCard({
  details,
  title,
}: {
  readonly details: readonly TrackingParentPortalStatusDetail[];
  readonly title: PortalDisplayText;
}): ReactElement {
  return (
    <article className={trackingParentPortalStatusCardClassName()}>
      <h2>{title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((detail) => (
          <TrackingParentPortalStatusDetail key={`${detail.label}:${detail.value}`} detail={detail} />
        ))}
      </dl>
    </article>
  );
}

function TrackingParentPortalStatusDetail({
  detail,
}: {
  readonly detail: TrackingParentPortalStatusDetail;
}): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function trackingParentPortalStatusCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

type TrackingParentPortalStatusDetail = {
  readonly label: PortalDisplayText;
  readonly value: PortalDisplayText | PortalDetailValue | TrackingStatusProofArtifact;
};
