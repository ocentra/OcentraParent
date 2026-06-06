import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  TrackingStatusProofArtifacts,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';
import type { AgentTrackingRetentionSettingsWriteResultParseResult } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import { appendDetail } from './detail-list';

type PortalTextTokenValue = (typeof PortalTextToken)[keyof typeof PortalTextToken];

export type TrackingRetentionSettingsWritePreflight = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly commandId: PortalDetailValue;
  readonly settingsKind: PortalDetailValue;
  readonly writeState: PortalDetailValue;
  readonly acceptedAt: PortalDetailValue;
  readonly sourceMutationProofRefs: PortalDetailValue;
  readonly commandTransportClaimedRows: PortalDetailValue;
  readonly serviceWritePreflightClaimedRows: PortalDetailValue;
  readonly serviceMutationExecutedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly parserReason: PortalDetailValue;
  readonly boundary: PortalDisplayText;
};

export type TrackingRetentionSettingsHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly evidence: PortalDisplayText;
};

export type TrackingRetentionSettingsHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly serviceMutationClaimedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly writePreflight: TrackingRetentionSettingsWritePreflight;
  readonly rows: readonly TrackingRetentionSettingsHostedUiRow[];
};

type TrackingRetentionSettingsHostedUiDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
};

const TrackingRetentionSettingsHostedUiDefinitions = [
  {
    titleToken: PortalTextToken.TrackingRetentionSettingsWindow,
    evidenceToken: PortalTextToken.TrackingRetentionSettingsWindowEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingRetentionSettingsDeleteAfterAlert,
    evidenceToken: PortalTextToken.TrackingRetentionSettingsDeleteAfterAlertEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingRetentionSettingsParentExport,
    evidenceToken: PortalTextToken.TrackingRetentionSettingsParentExportEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingRetentionSettingsRemoteSyncDisabled,
    evidenceToken: PortalTextToken.TrackingRetentionSettingsRemoteSyncEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingRetentionSettingsRemoteAiDisabled,
    evidenceToken: PortalTextToken.TrackingRetentionSettingsRemoteAiEvidence,
  },
] as const satisfies readonly TrackingRetentionSettingsHostedUiDefinition[];

export function trackingRetentionSettingsHostedUiProof(
  writeResult: AgentTrackingRetentionSettingsWriteResultParseResult | null = null
): TrackingRetentionSettingsHostedUiProof {
  const rows = TrackingRetentionSettingsHostedUiDefinitions.map((definition) =>
    retentionSettingsHostedUiRow(definition)
  );
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.RetentionSettingsReadModel,
    boundary: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    serviceMutationClaimedRows: detailFromValue(0),
    platformRuntimeClaimedRows: detailFromValue(0),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    writePreflight: retentionSettingsWritePreflight(writeResult),
    rows,
  };
}

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

function retentionSettingsHostedUiRow(
  definition: TrackingRetentionSettingsHostedUiDefinition
): TrackingRetentionSettingsHostedUiRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    status: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsReadModelReady),
    evidence: PortalText.Resolve(definition.evidenceToken),
  };
}

function appendRetentionSettingsHostedUiSummary(
  metadata: HTMLDListElement,
  proof: TrackingRetentionSettingsHostedUiProof
): void {
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(proof.proofTier));
  appendDetail(metadata, PortalDetails.RowsReturned, proof.rowsReturned);
  appendDetail(metadata, PortalDetails.RuntimeReference, toDetail(proof.proofArtifact));
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

function retentionSettingsWritePreflight(
  writeResult: AgentTrackingRetentionSettingsWriteResultParseResult | null
): TrackingRetentionSettingsWritePreflight {
  const base = emptyRetentionSettingsWritePreflight();
  if (writeResult === null) {
    return base;
  }
  if (!writeResult.ok) {
    return {
      ...base,
      parserReason: detailFromValue(writeResult.reason),
    };
  }
  const value = writeResult.value;
  return {
    ...base,
    commandId: detailFromValue(value.commandId),
    settingsKind: detailFromValue(value.settingsKind),
    writeState: detailFromValue(value.writeState),
    acceptedAt: detailFromValue(value.acceptedAt),
    sourceMutationProofRefs: detailFromValue(value.sourceMutationProofRefs.join(PortalFormatting.EventDetailSeparator)),
    commandTransportClaimedRows: detailFromFlag(value.commandTransportClaimed),
    serviceWritePreflightClaimedRows: detailFromFlag(value.serviceWritePreflightClaimed),
    serviceMutationExecutedRows: detailFromFlag(value.serviceMutationExecuted),
    platformRuntimeClaimedRows: detailFromFlag(value.platformRuntimeClaimed),
    childDeviceDeliveryClaimedRows: detailFromFlag(value.childDeviceDeliveryClaimed),
    providerDeliveryClaimedRows: detailFromFlag(value.providerDeliveryClaimed),
    notificationReceiptClaimedRows: detailFromFlag(value.notificationReceiptClaimed),
    physicalDeviceClaimedRows: detailFromFlag(value.physicalDeviceClaimed),
    authorityClaimedRows: detailFromFlag(value.authorityClaimed),
    productClaimReadyRows: detailFromFlag(value.productClaimReady),
  };
}

function emptyRetentionSettingsWritePreflight(): TrackingRetentionSettingsWritePreflight {
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsWritePreflight),
    body: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsWritePreflightBody),
    commandId: notReported(),
    settingsKind: notReported(),
    writeState: notReported(),
    acceptedAt: notReported(),
    sourceMutationProofRefs: notReported(),
    commandTransportClaimedRows: detailFromValue(0),
    serviceWritePreflightClaimedRows: detailFromValue(0),
    serviceMutationExecutedRows: detailFromValue(0),
    platformRuntimeClaimedRows: detailFromValue(0),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    notificationReceiptClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    parserReason: notReported(),
    boundary: PortalText.Resolve(PortalTextToken.TrackingRetentionSettingsWritePreflightBoundary),
  };
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

function detailFromFlag(value: boolean): PortalDetailValue {
  return detailFromValue(value ? 1 : 0);
}

function notReported(): PortalDetailValue {
  return toDetail(PortalText.Resolve(PortalTextToken.NotReported));
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function toDetail(value: PortalDisplayText | TrackingStatusProofArtifact): PortalDetailValue {
  return decodePortalDetailValue(value);
}
