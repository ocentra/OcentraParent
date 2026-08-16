import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { type ParentPortalDetailValue, type ParentTrackingStatusProofArtifact } from '../generated/parent-ui-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  type TrackingRetentionSettingsHostedUiProof,
  type TrackingRetentionSettingsWritePreflight,
} from '@ocentra-parent/portal-domain/tracking-retention-settings-hosted-ui-proof';
import { appendDetail, portalDetailFromValue } from './detail-list';

export function renderTrackingRetentionSettingsHostedUiProof(
  proof: TrackingRetentionSettingsHostedUiProof
): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;
  panel.setAttribute(PortalDom.Attributes.DataTrackingProof, PortalDom.Attributes.TrackingProofRetentionSettings);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = proof.title;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = proof.body;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendRetentionSettingsHostedUiSummary(metadata, proof);
  for (const proofRow of proof.rows) {
    appendDetail(metadata, PortalDetails.Title, toDetail(proofRow.title));
    appendDetail(metadata, PortalDetails.Status, toDetail(proofRow.status));
    appendDetail(metadata, PortalDetails.EvidenceReferences, toDetail(proofRow.evidence));
  }

  panel.append(title, body, metadata);
  return panel;
}

function appendRetentionSettingsHostedUiSummary(
  metadata: HTMLDListElement,
  proof: TrackingRetentionSettingsHostedUiProof
): void {
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, proof.rowsReturned);
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.writeCommandProofArtifact));
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.localStateProofArtifact));
  appendDetail(metadata, PortalDetails.AdapterBoundary, toDetail(proof.boundary));
  appendDetail(metadata, PortalDetails.MissingProof, toDetail(proof.missingProof));
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(proof.productClaim));
  appendDetail(metadata, PortalDetails.Events, proof.serviceMutationClaimedRows);
  appendDetail(metadata, PortalDetails.Provider, proof.providerDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.ChildDelivery, proof.childDeviceDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.Device, proof.physicalDeviceClaimedRows);
  appendDetail(metadata, PortalDetails.Enforcement, proof.authorityClaimedRows);
  appendDetail(metadata, PortalDetails.PolicyReadiness, proof.productClaimReadyRows);
  appendDetail(metadata, PortalDetails.Status, proof.platformRuntimeClaimedRows);
  appendRetentionSettingsWritePreflight(metadata, proof.writePreflight);
}

function appendRetentionSettingsWritePreflight(
  metadata: HTMLDListElement,
  writePreflight: TrackingRetentionSettingsWritePreflight
): void {
  appendDetail(metadata, PortalDetails.Title, toDetail(writePreflight.title));
  appendDetail(metadata, PortalDetails.SubjectId, writePreflight.commandId);
  appendDetail(metadata, PortalDetails.ActivityKind, writePreflight.settingsKind);
  appendDetail(metadata, PortalDetails.ExecutionState, writePreflight.writeState);
  appendDetail(metadata, PortalDetails.LastObserved, writePreflight.acceptedAt);
  appendDetail(metadata, PortalDetails.Source, writePreflight.sourceMutationProofRefs);
  appendDetail(metadata, PortalDetails.EntryId, writePreflight.sourceWriterIntentRefs);
  appendDetail(metadata, PortalDetails.RuntimeReference, writePreflight.sourceReadModelProofRefs);
  appendDetail(metadata, PortalDetails.State, writePreflight.appliedRetentionWindowHours);
  appendDetail(metadata, PortalDetails.PolicyEvaluation, writePreflight.appliedDeleteAfterAlertResolved);
  appendDetail(metadata, PortalDetails.Custody, writePreflight.parentExportPrepared);
  appendDetail(metadata, PortalDetails.Destination, writePreflight.remoteSyncEnabled);
  appendDetail(metadata, PortalDetails.LocalAiResult, writePreflight.remoteAiEnabled);
  appendDetail(metadata, PortalDetails.RowCount, writePreflight.localServiceStateRevision);
  appendDetail(metadata, PortalDetails.RuntimeReference, writePreflight.localServiceStateSnapshotRef);
  appendDetail(metadata, PortalDetails.Database, writePreflight.durableSettingsPersistedRows);
  appendDetail(metadata, PortalDetails.Transport, writePreflight.commandTransportClaimedRows);
  appendDetail(metadata, PortalDetails.Events, writePreflight.serviceWritePreflightClaimedRows);
  appendDetail(metadata, PortalDetails.Database, writePreflight.serviceMutationExecutedRows);
  appendDetail(metadata, PortalDetails.Status, writePreflight.platformRuntimeClaimedRows);
  appendDetail(metadata, PortalDetails.ChildDelivery, writePreflight.childDeviceDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.Provider, writePreflight.providerDeliveryClaimedRows);
  appendDetail(metadata, PortalDetails.AdapterDispatch, writePreflight.notificationReceiptClaimedRows);
  appendDetail(metadata, PortalDetails.Device, writePreflight.physicalDeviceClaimedRows);
  appendDetail(metadata, PortalDetails.Enforcement, writePreflight.authorityClaimedRows);
  appendDetail(metadata, PortalDetails.PolicyReadiness, writePreflight.productClaimReadyRows);
  appendDetail(metadata, PortalDetails.Reason, writePreflight.parserReason);
  appendDetail(metadata, PortalDetails.AdapterBoundary, toDetail(writePreflight.boundary));
}

function toDetail(value: PortalDisplayText | ParentTrackingStatusProofArtifact): ParentPortalDetailValue {
  return portalDetailFromValue(value);
}
