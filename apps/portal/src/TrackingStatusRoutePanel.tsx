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
  trackingChildCheckInProof,
  trackingChildRuntimeUiProof,
  type TrackingChildCheckInProof,
  type TrackingChildRuntimeUiProof,
} from './tracking-child-check-in-proof';
import {
  trackingRetentionSettingsHostedUiProof,
  type TrackingRetentionSettingsHostedUiProof,
} from './tracking-retention-settings-hosted-ui-proof';
import {
  TrackingEvidenceDrawerHostedUiProofDetails,
  trackingEvidenceDrawerHostedUiProof,
  type TrackingEvidenceDrawerHostedUiProof,
} from './tracking-evidence-drawer-hosted-ui-proof';
import {
  trackingFamilyDashboardHostedRollupProof,
  trackingStatusLiveSummary,
  trackingStatusServiceDataCoverage,
  trackingStatusProofRows,
  trackingUnsupportedManualPlatformProof,
  type TrackingFamilyDashboardHostedRollupProof,
  type TrackingStatusServiceDataCoverage,
  type TrackingStatusLiveCitation,
  type TrackingStatusLiveSummary,
  type TrackingStatusProofRow,
  type TrackingUnsupportedManualPlatformProof,
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
  const serviceDataCoverage = trackingStatusServiceDataCoverage(liveActivity);
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
          <TrackingStatusServiceDataCoverageCard coverage={serviceDataCoverage} />
          <TrackingFamilyDashboardHostedRollupProofCard proof={trackingFamilyDashboardHostedRollupProof()} />
          <TrackingRetentionSettingsHostedUiProofCard
            actions={actions}
            commandEnabled={commandEnabled}
            proof={trackingRetentionSettingsHostedUiProof(liveActivity.activityTrackingRetentionSettingsWriteResult)}
          />
          <TrackingEvidenceDrawerHostedUiProofCard
            proof={trackingEvidenceDrawerHostedUiProof(liveSummary.citations[0] ?? null)}
          />
          {liveSummary.citations.map((citation) => (
            <TrackingStatusLiveCitationCard key={String(citation.eventId)} citation={citation} />
          ))}
          <TrackingChildCheckInProofCard proof={trackingChildCheckInProof()} />
          <TrackingChildRuntimeUiProofCard proof={trackingChildRuntimeUiProof()} />
          <TrackingUnsupportedManualPlatformProofCard proof={trackingUnsupportedManualPlatformProof()} />
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
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={summary.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.LastObserved} value={summary.lastObserved} />
        <TrackingStatusDetail label={PortalDetails.EventId} value={summary.eventId} />
        <TrackingStatusDetail label={PortalDetails.Capability} value={summary.capability} />
        <TrackingStatusDetail label={PortalDetails.Custody} value={summary.custody} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={summary.evidenceReferences} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={summary.productClaim} />
        {summary.parserReason === null ? null : (
          <TrackingStatusDetail label={PortalDetails.Reason} value={summary.parserReason} />
        )}
      </dl>
    </article>
  );
}

function TrackingStatusServiceDataCoverageCard({
  coverage,
}: {
  readonly coverage: TrackingStatusServiceDataCoverage;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{coverage.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.LoadState} value={coverage.loadState} />
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={coverage.proofTier} />
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={coverage.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.HistoryVisibility} value={coverage.rowVisibility} />
        <TrackingStatusDetail label={PortalDetails.LastObserved} value={coverage.lastObserved} />
        <TrackingStatusDetail label={PortalDetails.EventId} value={coverage.eventId} />
        <TrackingStatusDetail label={PortalDetails.Capability} value={coverage.capability} />
        <TrackingStatusDetail label={PortalDetails.Custody} value={coverage.custody} />
        <TrackingStatusDetail label={PortalDetails.ActivityKind} value={coverage.activityKinds} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={coverage.evidenceReferences} />
        <TrackingStatusDetail label={PortalDetails.DeletedEvidence} value={coverage.deletedEvidence} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={coverage.productClaim} />
      </dl>
    </article>
  );
}

function TrackingFamilyDashboardHostedRollupProofCard({
  proof,
}: {
  readonly proof: TrackingFamilyDashboardHostedRollupProof;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofFamilyDashboard }}
    >
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={proof.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proof.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.AdapterBoundary} value={proof.boundary} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proof.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proof.productClaim} />
        <TrackingStatusDetail label={PortalDetails.ChildDelivery} value={proof.childDeviceDeliveryClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Provider} value={proof.providerDeliveryClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Events} value={proof.notificationReceiptClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Device} value={proof.physicalDeviceClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Enforcement} value={proof.authorityClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.PolicyReadiness} value={proof.productClaimReadyRows} />
        {proof.rows.map((proofRow) => (
          <TrackingFamilyDashboardHostedRollupRow key={String(proofRow.title)} proofRow={proofRow} />
        ))}
      </dl>
    </article>
  );
}

function TrackingFamilyDashboardHostedRollupRow({
  proofRow,
}: {
  readonly proofRow: TrackingFamilyDashboardHostedRollupProof['rows'][number];
}): ReactElement {
  return (
    <>
      <TrackingStatusDetail label={PortalDetails.Title} value={proofRow.title} />
      <TrackingStatusDetail label={PortalDetails.Status} value={proofRow.status} />
      <TrackingStatusDetail label={PortalDetails.Device} value={proofRow.visibleChildren} />
      <TrackingStatusDetail label={PortalDetails.RowCount} value={proofRow.attentionItems} />
      <TrackingStatusDetail label={PortalDetails.HistoryVisibility} value={proofRow.retainedAuditItems} />
      <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proofRow.evidence} />
    </>
  );
}

function TrackingRetentionSettingsHostedUiProofCard({
  actions,
  commandEnabled,
  proof,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly proof: TrackingRetentionSettingsHostedUiProof;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofRetentionSettings }}
    >
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <button
        className={PortalDom.Classes.CommandResultTab}
        disabled={!commandEnabled}
        type={PortalDom.ButtonType.Button}
        onClick={() => {
          actions.selectCommandResult(AgentEvent.ActivityTrackingRetentionSettingsWriteReported);
          actions.sendCommand(AgentCommand.ActivityTrackingRetentionSettingsWrite, {});
        }}
      >
        {PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsWritePreflightButton)}
      </button>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={proof.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proof.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.AdapterBoundary} value={proof.boundary} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proof.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proof.productClaim} />
        <TrackingStatusDetail label={PortalDetails.Events} value={proof.serviceMutationClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Provider} value={proof.providerDeliveryClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.ChildDelivery} value={proof.childDeviceDeliveryClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Device} value={proof.physicalDeviceClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Enforcement} value={proof.authorityClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.PolicyReadiness} value={proof.productClaimReadyRows} />
        <TrackingStatusDetail label={PortalDetails.Status} value={proof.platformRuntimeClaimedRows} />
        {proof.rows.map((proofRow) => (
          <TrackingRetentionSettingsHostedUiRow key={String(proofRow.title)} proofRow={proofRow} />
        ))}
        <TrackingRetentionSettingsWritePreflightRow proof={proof} />
      </dl>
    </article>
  );
}

function TrackingRetentionSettingsWritePreflightRow({
  proof,
}: {
  readonly proof: TrackingRetentionSettingsHostedUiProof;
}): ReactElement {
  return (
    <>
      <TrackingStatusDetail label={PortalDetails.Title} value={proof.writePreflight.title} />
      <TrackingStatusDetail label={PortalDetails.SubjectId} value={proof.writePreflight.commandId} />
      <TrackingStatusDetail label={PortalDetails.ActivityKind} value={proof.writePreflight.settingsKind} />
      <TrackingStatusDetail label={PortalDetails.ExecutionState} value={proof.writePreflight.writeState} />
      <TrackingStatusDetail label={PortalDetails.LastObserved} value={proof.writePreflight.acceptedAt} />
      <TrackingStatusDetail label={PortalDetails.Source} value={proof.writePreflight.sourceMutationProofRefs} />
      <TrackingStatusDetail label={PortalDetails.Transport} value={proof.writePreflight.commandTransportClaimedRows} />
      <TrackingStatusDetail
        label={PortalDetails.Events}
        value={proof.writePreflight.serviceWritePreflightClaimedRows}
      />
      <TrackingStatusDetail label={PortalDetails.Database} value={proof.writePreflight.serviceMutationExecutedRows} />
      <TrackingStatusDetail label={PortalDetails.Status} value={proof.writePreflight.platformRuntimeClaimedRows} />
      <TrackingStatusDetail
        label={PortalDetails.ChildDelivery}
        value={proof.writePreflight.childDeviceDeliveryClaimedRows}
      />
      <TrackingStatusDetail label={PortalDetails.Provider} value={proof.writePreflight.providerDeliveryClaimedRows} />
      <TrackingStatusDetail
        label={PortalDetails.AdapterDispatch}
        value={proof.writePreflight.notificationReceiptClaimedRows}
      />
      <TrackingStatusDetail label={PortalDetails.Device} value={proof.writePreflight.physicalDeviceClaimedRows} />
      <TrackingStatusDetail label={PortalDetails.Enforcement} value={proof.writePreflight.authorityClaimedRows} />
      <TrackingStatusDetail label={PortalDetails.PolicyReadiness} value={proof.writePreflight.productClaimReadyRows} />
      <TrackingStatusDetail label={PortalDetails.Reason} value={proof.writePreflight.parserReason} />
      <TrackingStatusDetail label={PortalDetails.AdapterBoundary} value={proof.writePreflight.boundary} />
    </>
  );
}

function TrackingRetentionSettingsHostedUiRow({
  proofRow,
}: {
  readonly proofRow: TrackingRetentionSettingsHostedUiProof['rows'][number];
}): ReactElement {
  return (
    <>
      <TrackingStatusDetail label={PortalDetails.Title} value={proofRow.title} />
      <TrackingStatusDetail label={PortalDetails.Status} value={proofRow.status} />
      <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proofRow.evidence} />
    </>
  );
}

function TrackingEvidenceDrawerHostedUiProofCard({
  proof,
}: {
  readonly proof: TrackingEvidenceDrawerHostedUiProof;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofEvidenceDrawer }}
    >
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={TrackingEvidenceDrawerHostedUiProofDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={TrackingEvidenceDrawerHostedUiProofDetails.DrawerMode} value={proof.drawerMode} />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.SourceEvent}
          value={proof.sourceEventId}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.EvidenceReferences}
          value={proof.evidenceReferences}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.DeletedEvidence}
          value={proof.deletedEvidence}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.ProofArtifact}
          value={proof.proofArtifact}
        />
        <TrackingStatusDetail label={TrackingEvidenceDrawerHostedUiProofDetails.Boundary} value={proof.boundary} />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.MissingProof}
          value={proof.missingProof}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.ProductClaim}
          value={proof.productClaim}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.PolicyEvaluator}
          value={proof.policyEvaluatorClaimedRows}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.ActionDispatch}
          value={proof.actionDispatchClaimedRows}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.ChildDelivery}
          value={proof.childDeviceDeliveryClaimedRows}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.ProviderDelivery}
          value={proof.providerDeliveryClaimedRows}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.PhysicalDevice}
          value={proof.physicalDeviceClaimedRows}
        />
        <TrackingStatusDetail
          label={TrackingEvidenceDrawerHostedUiProofDetails.Authority}
          value={proof.authorityClaimedRows}
        />
      </dl>
    </article>
  );
}

function TrackingStatusLiveCitationCard({ citation }: { readonly citation: TrackingStatusLiveCitation }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofCitationDetail }}
    >
      <h2>{citation.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.EventId} value={citation.eventId} />
        <TrackingStatusDetail label={PortalDetails.LastObserved} value={citation.observedAt} />
        <TrackingStatusDetail label={PortalDetails.Device} value={citation.device} />
        <TrackingStatusDetail label={PortalDetails.Platform} value={citation.platform} />
        <TrackingStatusDetail label={PortalDetails.Observer} value={citation.observer} />
        <TrackingStatusDetail label={PortalDetails.ActivityKind} value={citation.activityKind} />
        <TrackingStatusDetail label={PortalDetails.Subject} value={citation.subject} />
        <TrackingStatusDetail label={PortalDetails.Status} value={citation.status} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={citation.evidenceReferences} />
        <TrackingStatusDetail label={PortalDetails.DeletedEvidence} value={citation.deletedEvidence} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={citation.productClaim} />
      </dl>
    </article>
  );
}

function TrackingChildCheckInProofCard({ proof }: { readonly proof: TrackingChildCheckInProof }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofChildCheckIn }}
    >
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.ChildCopy} value={proof.copyBoundary} />
        <TrackingStatusDetail label={PortalDetails.ChildSafeAction} value={proof.safeAction} />
        <TrackingStatusDetail label={PortalDetails.ChildHelpAction} value={proof.helpAction} />
        <TrackingStatusDetail label={PortalDetails.ChildShareLocationAction} value={proof.shareLocationAction} />
        <TrackingStatusDetail label={PortalDetails.ChildCallParentAction} value={proof.callParentAction} />
        <TrackingStatusDetail label={PortalDetails.ChildDelivery} value={proof.deliveryBoundary} />
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proof.evidence} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proof.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proof.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proof.productClaim} />
      </dl>
    </article>
  );
}

function TrackingChildRuntimeUiProofCard({ proof }: { readonly proof: TrackingChildRuntimeUiProof }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article
      className={className}
      {...{ [PortalDom.Attributes.DataTrackingProof]: PortalDom.Attributes.TrackingProofChildRuntimeUi }}
    >
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.ChildCopy} value={proof.disclosure} />
        <TrackingStatusDetail label={PortalDetails.ChildSafeAction} value={proof.safeResponse} />
        <TrackingStatusDetail label={PortalDetails.ChildHelpAction} value={proof.helpResponse} />
        <TrackingStatusDetail label={PortalDetails.ChildShareLocationAction} value={proof.locationShareConsent} />
        <TrackingStatusDetail label={PortalDetails.ChildDelivery} value={proof.deliveryBoundary} />
        <TrackingStatusDetail label={PortalDetails.AdapterBoundary} value={proof.runtimeBoundary} />
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proof.evidence} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proof.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proof.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proof.productClaim} />
      </dl>
    </article>
  );
}

function TrackingUnsupportedManualPlatformProofCard({
  proof,
}: {
  readonly proof: TrackingUnsupportedManualPlatformProof;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{proof.title}</h2>
      <p>{proof.body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <TrackingStatusDetail label={PortalDetails.ProofTier} value={proof.proofTier} />
        <TrackingStatusDetail label={PortalDetails.RowsReturned} value={proof.rowsReturned} />
        <TrackingStatusDetail label={PortalDetails.AdapterBoundary} value={proof.boundary} />
        <TrackingStatusDetail label={PortalDetails.EvidenceReferences} value={proof.evidence} />
        <TrackingStatusDetail label={PortalDetails.RuntimeReference} value={proof.proofArtifact} />
        <TrackingStatusDetail label={PortalDetails.MissingProof} value={proof.missingProof} />
        <TrackingStatusDetail label={PortalDetails.ProductClaim} value={proof.productClaim} />
        <TrackingStatusDetail label={PortalDetails.Provider} value={proof.fakeCapabilityRows} />
        <TrackingStatusDetail label={PortalDetails.PolicyReadiness} value={proof.productClaimReadyRows} />
        <TrackingStatusDetail label={PortalDetails.Device} value={proof.physicalDeviceClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.Enforcement} value={proof.authorityClaimedRows} />
        <TrackingStatusDetail label={PortalDetails.ManualReview} value={proof.authorityRequiredRows} />
        {proof.rows.map((proofRow) => (
          <TrackingUnsupportedManualPlatformRow key={String(proofRow.title)} proofRow={proofRow} />
        ))}
      </dl>
    </article>
  );
}

function TrackingUnsupportedManualPlatformRow({
  proofRow,
}: {
  readonly proofRow: TrackingUnsupportedManualPlatformProof['rows'][number];
}): ReactElement {
  return (
    <>
      <TrackingStatusDetail label={PortalDetails.Title} value={proofRow.title} />
      <TrackingStatusDetail label={PortalDetails.ReadinessKind} value={proofRow.supportState} />
      <TrackingStatusDetail label={PortalDetails.Status} value={proofRow.renderedState} />
    </>
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
