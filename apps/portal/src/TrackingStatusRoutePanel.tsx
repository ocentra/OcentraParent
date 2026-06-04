import type { ReactElement } from 'react';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  type PortalDetailValue,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import type { PortalLiveActivityState } from './live-activity-state';
import {
  trackingStatusLiveSummary,
  trackingStatusProofRows,
  type TrackingStatusLiveSummary,
  type TrackingStatusProofRow,
} from './tracking-status-panel';

export function shouldRenderTrackingStatusRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.PolicyTracking;
}

export function TrackingStatusRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const liveSummary = trackingStatusLiveSummary(liveActivity);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.TrackingStatusSurface)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalText.Resolve(PortalTextToken.TrackingFirstTarget)}</p>
          <h2>{PortalText.Resolve(PortalTextToken.TrackingStatusSurface)}</h2>
          <p>{PortalText.Resolve(PortalTextToken.TrackingStatusSurfaceBody)}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => {
              actions.selectCommandResult(AgentEvent.ActivityTrackingReadModelReported);
              actions.sendCommand(AgentCommand.ActivityTrackingReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityTrackingReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <TrackingStatusLiveSummaryCard summary={liveSummary} />
          {trackingStatusProofRows().map((proofRow) => (
            <TrackingStatusRouteRow key={String(proofRow.title)} proofRow={proofRow} />
          ))}
        </div>
      </div>
    </section>
  );
}

function TrackingStatusLiveSummaryCard({ summary }: { readonly summary: TrackingStatusLiveSummary }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{summary.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.LoadState} value={summary.loadState} />
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={summary.proofTier} />
        <TrackingStatusDetail label={PortalDetails.GeneratedAt} value={summary.generatedAt} />
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={summary.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.RetentionTombstones} value={summary.retentionTombstones} />
        <TrackingStatusDetail label={PortalDetails.LastObserved} value={summary.lastObserved} />
        <TrackingStatusDetail label={PortalDetails.EventId} value={summary.eventId} />
        <TrackingStatusDetail label={PortalDetails.Capability} value={summary.capability} />
        <TrackingStatusDetail label={PortalDetails.Custody} value={summary.custody} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={summary.evidenceReferences} />
        <TrackingStatusDetail label={PortalDetails.DeletedEvidence} value={summary.deletedEvidence} />
        <TrackingStatusDetail label={PortalDetails.ActivityKind} value={summary.latestRowKind} />
        <TrackingStatusDetail label={PortalDetails.Subject} value={summary.latestRowSubject} />
        <TrackingStatusDetail label={PortalDetails.SubjectKind} value={summary.latestRowSubjectKind} />
        <TrackingStatusDetail label={PortalDetails.SubjectId} value={summary.latestRowSubjectId} />
        <TrackingStatusDetail label={PortalDetails.Device} value={summary.latestRowDevice} />
        <TrackingStatusDetail label={PortalDetails.Platform} value={summary.latestRowPlatform} />
        <TrackingStatusDetail label={PortalDetails.Observer} value={summary.latestRowObserver} />
        <TrackingStatusDetail
          label={PortalDetails.LatestRowEvidenceReferences}
          value={summary.latestRowEvidenceReferences}
        />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={summary.productClaim} />
        {summary.parserReason === null ? null : (
          <TrackingStatusDetail label={PortalDetails.Reason} value={summary.parserReason} />
        )}
      </dl>
    </article>
  );
}

function TrackingStatusRouteRow({ proofRow }: { readonly proofRow: TrackingStatusProofRow }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{proofRow.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.Status} value={proofRow.state} />
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proofRow.proofTier} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proofRow.evidence} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proofRow.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proofRow.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proofRow.productClaim} />
        {proofRow.historyVisibility === undefined ? null : (
          <TrackingStatusDetail label={PortalDetails.HistoryVisibility} value={proofRow.historyVisibility} />
        )}
        {proofRow.deletedEvidence === undefined ? null : (
          <TrackingStatusDetail label={PortalDetails.DeletedEvidence} value={proofRow.deletedEvidence} />
        )}
      </dl>
    </article>
  );
}

function TrackingStatusDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDisplayText | PortalDetailValue | TrackingStatusProofArtifact;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
