import {
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  TrackingEvidenceDrawerHostedUiProofDetails,
  trackingEvidenceDrawerHostedUiProof,
  type TrackingEvidenceDrawerHostedUiProof,
} from '@ocentra-parent/portal-domain/tracking-evidence-drawer-hosted-ui-proof';
import {
  trackingFamilyDashboardHostedRollupProof,
  trackingStatusLiveSummary,
  trackingStatusProofRows,
  trackingStatusServiceDataCoverage,
  trackingUnsupportedManualPlatformProof,
  type TrackingFamilyDashboardHostedRollupProof,
  type TrackingStatusLiveCitation,
  type TrackingStatusLiveSummary,
  type TrackingStatusProofRow,
  type TrackingStatusServiceDataCoverage,
  type TrackingUnsupportedManualPlatformProof,
} from '@ocentra-parent/portal-domain/tracking-status-panel';
import { trackingRetentionSettingsHostedUiProof } from '@ocentra-parent/portal-domain/tracking-retention-settings-hosted-ui-proof';
import {
  trackingChildCheckInProof,
  trackingChildRuntimeUiProof,
  type TrackingChildCheckInProof,
  type TrackingChildRuntimeUiProof,
} from '@ocentra-parent/portal-domain/tracking-child-check-in-proof';
import { renderTrackingRetentionSettingsHostedUiProof } from './tracking-retention-settings-hosted-ui-proof';
import { appendDetail, portalDetailFromSequence, portalDetailFromValue } from './detail-list';
import type { PortalLiveActivityState } from './live-activity-state';
import { renderDashboard } from './portal-dashboard';

export function renderTrackingStatusSurface(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const intro = document.createElement(PortalDom.Tags.Section);
  intro.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.TrackingStatusSurface);

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = resolvePortalDevText(PortalDevTextToken.TrackingStatusSurfaceBody);

  intro.append(title, body);
  container.append(intro);

  renderDashboard(container, (dashboard) => {
    const liveSummary = trackingStatusLiveSummary(liveActivity);
    dashboard.append(renderTrackingStatusLiveSummary(liveSummary));
    dashboard.append(renderTrackingStatusServiceDataCoverage(trackingStatusServiceDataCoverage(liveActivity)));
    dashboard.append(renderTrackingFamilyDashboardHostedRollupProof(trackingFamilyDashboardHostedRollupProof()));
    dashboard.append(
      renderTrackingRetentionSettingsHostedUiProof(
        trackingRetentionSettingsHostedUiProof(liveActivity.activityTrackingRetentionSettingsWriteResult)
      )
    );
    dashboard.append(
      renderTrackingEvidenceDrawerHostedUiProof(trackingEvidenceDrawerHostedUiProof(liveSummary.citations[0] ?? null))
    );
    for (const citation of liveSummary.citations) {
      dashboard.append(renderTrackingStatusLiveCitation(citation));
    }
    dashboard.append(renderTrackingChildCheckInProof(trackingChildCheckInProof()));
    dashboard.append(renderTrackingChildRuntimeUiProof(trackingChildRuntimeUiProof()));
    dashboard.append(renderTrackingUnsupportedManualPlatformProof(trackingUnsupportedManualPlatformProof()));
    for (const proofRow of trackingStatusProofRows()) {
      dashboard.append(renderTrackingStatusRow(proofRow));
    }
  });
}

function renderTrackingStatusRow(proofRow: TrackingStatusProofRow): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proofRow.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.Status, toDetail(proofRow.state));
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proofRow.proofTier));
  appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proofRow.evidence));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proofRow.proofArtifact));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proofRow.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proofRow.productClaim));
  if (proofRow.historyVisibility !== undefined) {
    appendDetail(metadata, PortalDetails.HistoryVisibility, toDetail(proofRow.historyVisibility));
  }
  if (proofRow.deletedEvidence !== undefined) {
    appendDetail(metadata, PortalDetails.DeletedEvidence, toDetail(proofRow.deletedEvidence));
  }

  panel.append(title, metadata);
  return panel;
}

function renderTrackingUnsupportedManualPlatformProof(proof: TrackingUnsupportedManualPlatformProof): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendUnsupportedManualSummary(metadata, proof);
  for (const proofRow of proof.rows) {
    appendDetail(metadata, PortalDetails.Title, toDetail(proofRow.title));
    appendDetail(metadata, PortalDetails.ReadinessKind, toDetail(proofRow.supportState));
    appendDetail(metadata, PortalDetails.Status, toDetail(proofRow.renderedState));
  }

  panel.append(title, body, metadata);
  return panel;
}

function appendUnsupportedManualSummary(
  metadata: HTMLDListElement,
  proof: TrackingUnsupportedManualPlatformProof
): void {
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, proof.rowsReturned);
  appendDetail(metadata, PortalDetails.Status, sequenceDetail([proof.manualRequiredRows, proof.unavailableRows]));
  appendDetail(metadata, PortalDetails.AdapterBoundary, toDetail(proof.boundary));
  appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proof.evidence));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proof.productClaim));
  appendDetail(metadata, PortalDetails.Provider, proof.fakeCapabilityRows);
  appendDetail(metadata, PortalDetails.PolicyReadiness, proof.productClaimReadyRows);
  appendDetail(metadata, PortalDetails.Device, proof.physicalDeviceClaimedRows);
  appendDetail(metadata, PortalDetails.Enforcement, proof.authorityClaimedRows);
  appendDetail(metadata, PortalDetails.ManualReview, proof.authorityRequiredRows);
}

function renderTrackingFamilyDashboardHostedRollupProof(proof: TrackingFamilyDashboardHostedRollupProof): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofFamilyDashboard);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendFamilyDashboardRollupSummary(metadata, proof);
  for (const proofRow of proof.rows) {
    appendDetail(metadata, PortalDetails.Title, toDetail(proofRow.title));
    appendDetail(metadata, PortalDetails.Status, toDetail(proofRow.status));
    appendDetail(metadata, PortalDetails.Device, proofRow.visibleChildren);
    appendDetail(metadata, PortalDetails.RowCount, proofRow.attentionItems);
    appendDetail(metadata, PortalDetails.HistoryVisibility, proofRow.retainedAuditItems);
    appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proofRow.evidence));
  }

  panel.append(title, body, metadata);
  return panel;
}

function appendFamilyDashboardRollupSummary(
  metadata: HTMLDListElement,
  proof: TrackingFamilyDashboardHostedRollupProof
): void {
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, proof.rowsReturned);
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
  appendDetail(metadata, PortalDetails.AdapterBoundary, toDetail(proof.boundary));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proof.productClaim));
  appendDetail(metadata, PortalDetails.ChildDelivery, proof.childDeviceDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.Provider, proof.providerDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.Events, proof.notificationReceiptClaimedRows);
  appendDetail(metadata, PortalDetails.Device, proof.physicalDeviceClaimedRows);
  appendDetail(metadata, PortalDetails.Enforcement, proof.authorityClaimedRows);
  appendDetail(metadata, PortalDetails.PolicyReadiness, proof.productClaimReadyRows);
}

function renderTrackingStatusLiveSummary(summary: TrackingStatusLiveSummary): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = summary.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.LoadState, summary.loadState);
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(summary.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, summary.rowsReturned);
  appendDetail(metadata, PortalDetails.LastObserved, summary.lastObserved);
  appendDetail(metadata, PortalDetails.EventId, summary.eventId);
  appendDetail(metadata, PortalDetails.Capability, summary.capability);
  appendDetail(metadata, PortalDetails.Custody, summary.custody);
  appendDetail(metadata, PortalDetails.EvidenceReferences, summary.evidenceReferences);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(summary.productClaim));
  if (summary.parserReason !== null) {
    appendDetail(metadata, PortalDetails.Reason, summary.parserReason);
  }

  panel.append(title, metadata);
  return panel;
}

function renderTrackingStatusServiceDataCoverage(coverage: TrackingStatusServiceDataCoverage): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = coverage.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.LoadState, coverage.loadState);
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(coverage.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, coverage.rowsReturned);
  appendDetail(metadata, PortalDetails.HistoryVisibility, coverage.rowVisibility);
  appendDetail(metadata, PortalDetails.LastObserved, coverage.lastObserved);
  appendDetail(metadata, PortalDetails.EventId, coverage.eventId);
  appendDetail(metadata, PortalDetails.Device, coverage.deviceCounts);
  appendDetail(metadata, PortalDetails.Capability, coverage.capability);
  appendDetail(metadata, PortalDetails.Custody, coverage.custody);
  appendDetail(metadata, PortalDetails.ActivityKind, coverage.activityKinds);
  appendDetail(metadata, PortalDetails.EvidenceReferences, coverage.evidenceReferences);
  appendDetail(metadata, PortalDetails.DeletedEvidence, coverage.deletedEvidence);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(coverage.productClaim));

  panel.append(title, metadata);
  return panel;
}

function renderTrackingEvidenceDrawerHostedUiProof(proof: TrackingEvidenceDrawerHostedUiProof): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofEvidenceDrawer);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendEvidenceDrawerDetails(metadata, proof);

  panel.append(title, body, metadata);
  return panel;
}

function appendEvidenceDrawerDetails(metadata: HTMLDListElement, proof: TrackingEvidenceDrawerHostedUiProof): void {
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.DrawerMode, toDetail(proof.drawerMode));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.SourceEvent, proof.sourceEventId);
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.EvidenceReferences, proof.evidenceReferences);
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.DeletedEvidence, proof.deletedEvidence);
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.ProofArtifact, toDetail(proof.proofArtifact));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.Boundary, toDetail(proof.boundary));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.ProductClaim, toDetail(proof.productClaim));
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.PolicyEvaluator, proof.policyEvaluatorClaimedRows);
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.ActionDispatch, proof.actionDispatchClaimedRows);
  appendDetail(
    metadata,
    TrackingEvidenceDrawerHostedUiProofDetails.ChildDelivery,
    proof.childDeviceDeliveryClaimedRows
  );
  appendDetail(
    metadata,
    TrackingEvidenceDrawerHostedUiProofDetails.ProviderDelivery,
    proof.providerDeliveryClaimedRows
  );
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.PhysicalDevice, proof.physicalDeviceClaimedRows);
  appendDetail(metadata, TrackingEvidenceDrawerHostedUiProofDetails.Authority, proof.authorityClaimedRows);
}

function renderTrackingStatusLiveCitation(citation: TrackingStatusLiveCitation): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofCitationDetail);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = citation.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.EventId, citation.eventId);
  appendDetail(metadata, PortalDetails.LastObserved, citation.observedAt);
  appendDetail(metadata, PortalDetails.Device, citation.device);
  appendDetail(metadata, PortalDetails.Platform, citation.platform);
  appendDetail(metadata, PortalDetails.Observer, citation.observer);
  appendDetail(metadata, PortalDetails.ActivityKind, citation.activityKind);
  appendDetail(metadata, PortalDetails.Subject, citation.subject);
  appendDetail(metadata, PortalDetails.Status, citation.status);
  appendDetail(metadata, PortalDetails.EvidenceReferences, citation.evidenceReferences);
  appendDetail(metadata, PortalDetails.DeletedEvidence, citation.deletedEvidence);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(citation.productClaim));

  panel.append(title, metadata);
  return panel;
}

function renderTrackingChildCheckInProof(proof: TrackingChildCheckInProof): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofChildCheckIn);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.ChildCopy, toDetail(proof.copyBoundary));
  appendDetail(metadata, PortalDetails.ChildSafeAction, toDetail(proof.safeAction));
  appendDetail(metadata, PortalDetails.ChildHelpAction, toDetail(proof.helpAction));
  appendDetail(metadata, PortalDetails.ChildShareLocationAction, toDetail(proof.shareLocationAction));
  appendDetail(metadata, PortalDetails.ChildCallParentAction, toDetail(proof.callParentAction));
  appendDetail(metadata, PortalDetails.ChildDelivery, toDetail(proof.deliveryBoundary));
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proof.evidence));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proof.productClaim));

  panel.append(title, body, metadata);
  return panel;
}

function renderTrackingChildRuntimeUiProof(proof: TrackingChildRuntimeUiProof): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofChildRuntimeUi);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.ChildCopy, toDetail(proof.disclosure));
  appendDetail(metadata, PortalDetails.ChildSafeAction, toDetail(proof.safeResponse));
  appendDetail(metadata, PortalDetails.ChildHelpAction, toDetail(proof.helpResponse));
  appendDetail(metadata, PortalDetails.ChildShareLocationAction, toDetail(proof.locationShareConsent));
  appendDetail(metadata, PortalDetails.ChildDelivery, toDetail(proof.deliveryBoundary));
  appendDetail(metadata, PortalDetails.AdapterBoundary, toDetail(proof.runtimeBoundary));
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proof.evidence));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proof.productClaim));

  panel.append(title, body, metadata);
  return panel;
}

function toDetail(value: PortalDisplayText | TrackingStatusProofArtifact): PortalDetailValue {
  return portalDetailFromValue(value);
}

function sequenceDetail(values: readonly unknown[]): PortalDetailValue {
  return portalDetailFromSequence(values);
}
