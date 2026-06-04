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
import type {
  AgentActivityTrackingEvidenceReferenceIds,
  AgentActivityTrackingReadModelRow,
} from '@ocentra-parent/agent-protocol-domain/tracking-read-model';
import { appendDetail } from './detail-list';
import type { PortalLiveActivityState } from './live-activity-state';
import { renderDashboard } from './portal-dashboard';

type PortalTextTokenValue = (typeof PortalTextToken)[keyof typeof PortalTextToken];

export type TrackingStatusProofRow = {
  readonly title: PortalDisplayText;
  readonly state: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly evidence: PortalDisplayText;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly historyVisibility?: PortalDisplayText;
  readonly deletedEvidence?: PortalDisplayText;
};

export type TrackingStatusLiveSummary = {
  readonly title: PortalDisplayText;
  readonly loadState: PortalDetailValue;
  readonly proofTier: PortalDisplayText;
  readonly generatedAt: PortalDetailValue;
  readonly rowsReturned: PortalDetailValue;
  readonly retentionTombstones: PortalDetailValue;
  readonly lastObserved: PortalDetailValue;
  readonly eventId: PortalDetailValue;
  readonly capability: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly latestRowKind: PortalDetailValue;
  readonly latestRowSubject: PortalDetailValue;
  readonly latestRowSubjectKind: PortalDetailValue;
  readonly latestRowSubjectId: PortalDetailValue;
  readonly latestRowDevice: PortalDetailValue;
  readonly latestRowPlatform: PortalDetailValue;
  readonly latestRowObserver: PortalDetailValue;
  readonly latestRowEvidenceReferences: PortalDetailValue;
  readonly parserReason: PortalDetailValue | null;
  readonly productClaim: PortalDisplayText;
};

export type TrackingStatusEvidenceDrawerRow = {
  readonly title: PortalDetailValue;
  readonly proofTier: PortalDisplayText;
  readonly eventId: PortalDetailValue;
  readonly observedAt: PortalDetailValue;
  readonly activityKind: PortalDetailValue;
  readonly subject: PortalDetailValue;
  readonly subjectKind: PortalDetailValue;
  readonly subjectId: PortalDetailValue;
  readonly device: PortalDetailValue;
  readonly platform: PortalDetailValue;
  readonly observer: PortalDetailValue;
  readonly capability: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly productClaim: PortalDisplayText;
};

type TrackingStatusRetentionProofDefinition = {
  readonly historyVisibility: PortalTextTokenValue;
  readonly deletedEvidence: PortalTextTokenValue;
};

type TrackingStatusProofRowDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly retentionProof?: TrackingStatusRetentionProofDefinition;
};

const TrackingStatusProofRowDefinitions = [
  {
    titleToken: PortalTextToken.TrackingStateDisabled,
    evidenceToken: PortalTextToken.TrackingEvidenceContracts,
    proofArtifact: TrackingStatusProofArtifacts.ContractBoundary,
  },
  {
    titleToken: PortalTextToken.TrackingStatePermissionRequired,
    evidenceToken: PortalTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.PermissionCapability,
  },
  {
    titleToken: PortalTextToken.TrackingStateStale,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingStateOffline,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.DeviceStatus,
  },
  {
    titleToken: PortalTextToken.TrackingStateLowAccuracy,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  },
  {
    titleToken: PortalTextToken.TrackingStateAmbiguousNearby,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.NearbyPlace,
  },
  {
    titleToken: PortalTextToken.TrackingStateAlert,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.AlertSeverity,
  },
  {
    titleToken: PortalTextToken.TrackingStateAcknowledged,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgement,
  },
  {
    titleToken: PortalTextToken.TrackingStateException,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgement,
  },
  {
    titleToken: PortalTextToken.TrackingStateChildCheckIn,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
  },
  {
    titleToken: PortalTextToken.TrackingStateTemporaryLive,
    evidenceToken: PortalTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.TemporaryLiveMode,
  },
  {
    titleToken: PortalTextToken.TrackingStateMissingDevice,
    evidenceToken: PortalTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
  },
  {
    titleToken: PortalTextToken.TrackingStateRetentionDeleted,
    evidenceToken: PortalTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RetentionDelete,
    retentionProof: {
      historyVisibility: PortalTextToken.TrackingRetentionHistoryHidden,
      deletedEvidence: PortalTextToken.TrackingDeletedEvidenceNotRendered,
    },
  },
] as const satisfies readonly TrackingStatusProofRowDefinition[];

export function trackingStatusProofRows(): readonly TrackingStatusProofRow[] {
  return TrackingStatusProofRowDefinitions.map((definition) => row(definition));
}

export function trackingStatusLiveSummary(liveActivity: PortalLiveActivityState): TrackingStatusLiveSummary {
  const event = liveActivity.activityTrackingReadModelEvent;
  const readModelResult = liveActivity.activityTrackingReadModel;
  const baseSummary = {
    title: PortalText.Resolve(PortalTextToken.TrackingServiceReadModel),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    generatedAt: notReported(),
    rowsReturned: notReported(),
    retentionTombstones: notReported(),
    lastObserved: notReported(),
    eventId: notReported(),
    capability: notReported(),
    custody: notReported(),
    evidenceReferences: notReported(),
    deletedEvidence: notReported(),
    latestRowKind: notReported(),
    latestRowSubject: notReported(),
    latestRowSubjectKind: notReported(),
    latestRowSubjectId: notReported(),
    latestRowDevice: notReported(),
    latestRowPlatform: notReported(),
    latestRowObserver: notReported(),
    latestRowEvidenceReferences: notReported(),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };

  if (event === null || readModelResult === null) {
    return {
      ...baseSummary,
      loadState: notReported(),
      parserReason: null,
    };
  }

  if (!readModelResult.ok) {
    return {
      ...baseSummary,
      loadState: detailFromValue(event.severity),
      parserReason: detailFromValue(readModelResult.reason),
    };
  }

  const readModel = readModelResult.value;
  const latestRow = readModel.rows[0];
  return {
    ...baseSummary,
    loadState: detailFromValue(event.severity),
    generatedAt: detailFromValue(readModel.generatedAt),
    rowsReturned: detailFromValue(readModel.returned),
    retentionTombstones: detailFromValue(readModel.retentionTombstoneCount),
    lastObserved: detailFromValue(readModel.latestObservedAt),
    eventId: detailFromValue(readModel.latestEventId),
    capability: detailFromValue(readModel.capabilityStatus),
    custody: detailFromValue(readModel.custodyLabel),
    evidenceReferences: evidenceReferenceDetail(readModel.evidenceReferenceIds),
    deletedEvidence: evidenceReferenceDetail(readModel.retentionTombstoneEvidenceReferenceIds),
    latestRowKind: readModelRowDetail(latestRow, (rowValue) => rowValue.kind),
    latestRowSubject: readModelRowDetail(latestRow, (rowValue) => rowValue.subjectDisplayName ?? rowValue.subjectId),
    latestRowSubjectKind: readModelRowDetail(latestRow, (rowValue) => rowValue.subjectKind),
    latestRowSubjectId: readModelRowDetail(latestRow, (rowValue) => rowValue.subjectId),
    latestRowDevice: readModelRowDetail(latestRow, (rowValue) => rowValue.deviceId),
    latestRowPlatform: readModelRowDetail(latestRow, (rowValue) => rowValue.platform),
    latestRowObserver: readModelRowDetail(latestRow, (rowValue) => rowValue.observer),
    latestRowEvidenceReferences: evidenceReferenceDetail(latestRow?.evidenceReferenceIds),
    parserReason: null,
  };
}

export function trackingStatusEvidenceDrawerRows(
  liveActivity: PortalLiveActivityState
): readonly TrackingStatusEvidenceDrawerRow[] {
  const readModelResult = liveActivity.activityTrackingReadModel;
  if (readModelResult === null || !readModelResult.ok) {
    return [];
  }
  return readModelResult.value.rows.map((rowValue) => ({
    title: readModelRowDetail(rowValue, (value) => value.subjectDisplayName ?? value.subjectId),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    eventId: readModelRowDetail(rowValue, (value) => value.eventId),
    observedAt: readModelRowDetail(rowValue, (value) => value.observedAt),
    activityKind: readModelRowDetail(rowValue, (value) => value.kind),
    subject: readModelRowDetail(rowValue, (value) => value.subjectDisplayName ?? value.subjectId),
    subjectKind: readModelRowDetail(rowValue, (value) => value.subjectKind),
    subjectId: readModelRowDetail(rowValue, (value) => value.subjectId),
    device: readModelRowDetail(rowValue, (value) => value.deviceId),
    platform: readModelRowDetail(rowValue, (value) => value.platform),
    observer: readModelRowDetail(rowValue, (value) => value.observer),
    capability: readModelRowDetail(rowValue, (value) => value.capabilityStatus),
    evidenceReferences: evidenceReferenceDetail(rowValue.evidenceReferenceIds),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  }));
}

export function renderTrackingStatusSurface(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const intro = document.createElement(PortalDom.Tags.Section);
  intro.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.TrackingStatusSurface);

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.CommandResultEmpty;
  body.textContent = PortalText.Resolve(PortalTextToken.TrackingStatusSurfaceBody);

  intro.append(title, body);
  container.append(intro);

  renderDashboard(container, (dashboard) => {
    dashboard.append(renderTrackingStatusLiveSummary(trackingStatusLiveSummary(liveActivity)));
    for (const evidenceDrawerRow of trackingStatusEvidenceDrawerRows(liveActivity)) {
      dashboard.append(renderTrackingStatusEvidenceDrawerRow(evidenceDrawerRow));
    }
    for (const proofRow of trackingStatusProofRows()) {
      dashboard.append(renderTrackingStatusRow(proofRow));
    }
  });
}

function row(definition: TrackingStatusProofRowDefinition): TrackingStatusProofRow {
  const { titleToken, evidenceToken, proofArtifact } = definition;
  const baseRow = {
    title: PortalText.Resolve(titleToken),
    state: PortalText.Resolve(titleToken),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofFixture),
    evidence: PortalText.Resolve(evidenceToken),
    proofArtifact,
    missingProof: missingProofForEvidence(evidenceToken),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };
  const retentionProof = definition.retentionProof;
  if (retentionProof === undefined) {
    return baseRow;
  }
  return {
    ...baseRow,
    historyVisibility: PortalText.Resolve(retentionProof.historyVisibility),
    deletedEvidence: PortalText.Resolve(retentionProof.deletedEvidence),
  };
}

function missingProofForEvidence(evidenceToken: PortalTextTokenValue): PortalDisplayText {
  if (evidenceToken === PortalTextToken.TrackingEvidencePhysicalMissing) {
    return PortalText.Resolve(PortalTextToken.TrackingPhysicalDeviceRequired);
  }
  return PortalText.Resolve(PortalTextToken.TrackingManualRequired);
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

function renderTrackingStatusLiveSummary(summary: TrackingStatusLiveSummary): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = summary.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.LoadState, summary.loadState);
  appendDetail(metadata, PortalDetails.ProofTier, toDetail(summary.proofTier));
  appendDetail(metadata, PortalDetails.GeneratedAt, summary.generatedAt);
  appendDetail(metadata, PortalDetails.RowsReturned, summary.rowsReturned);
  appendDetail(metadata, PortalDetails.RetentionTombstones, summary.retentionTombstones);
  appendDetail(metadata, PortalDetails.LastObserved, summary.lastObserved);
  appendDetail(metadata, PortalDetails.EventId, summary.eventId);
  appendDetail(metadata, PortalDetails.Capability, summary.capability);
  appendDetail(metadata, PortalDetails.Custody, summary.custody);
  appendDetail(metadata, PortalDetails.EvidenceReferences, summary.evidenceReferences);
  appendDetail(metadata, PortalDetails.DeletedEvidence, summary.deletedEvidence);
  appendDetail(metadata, PortalDetails.ActivityKind, summary.latestRowKind);
  appendDetail(metadata, PortalDetails.Subject, summary.latestRowSubject);
  appendDetail(metadata, PortalDetails.SubjectKind, summary.latestRowSubjectKind);
  appendDetail(metadata, PortalDetails.SubjectId, summary.latestRowSubjectId);
  appendDetail(metadata, PortalDetails.Device, summary.latestRowDevice);
  appendDetail(metadata, PortalDetails.Platform, summary.latestRowPlatform);
  appendDetail(metadata, PortalDetails.Observer, summary.latestRowObserver);
  appendDetail(metadata, PortalDetails.LatestRowEvidenceReferences, summary.latestRowEvidenceReferences);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(summary.productClaim));
  if (summary.parserReason !== null) {
    appendDetail(metadata, PortalDetails.Reason, summary.parserReason);
  }

  panel.append(title, metadata);
  return panel;
}

function renderTrackingStatusEvidenceDrawerRow(evidenceDrawerRow: TrackingStatusEvidenceDrawerRow): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = evidenceDrawerRow.title;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.ServiceEvidenceDrawer, toDetail(evidenceDrawerRow.proofTier));
  appendDetail(metadata, PortalDetails.EventId, evidenceDrawerRow.eventId);
  appendDetail(metadata, PortalDetails.ObservedAt, evidenceDrawerRow.observedAt);
  appendDetail(metadata, PortalDetails.ActivityKind, evidenceDrawerRow.activityKind);
  appendDetail(metadata, PortalDetails.Subject, evidenceDrawerRow.subject);
  appendDetail(metadata, PortalDetails.SubjectKind, evidenceDrawerRow.subjectKind);
  appendDetail(metadata, PortalDetails.SubjectId, evidenceDrawerRow.subjectId);
  appendDetail(metadata, PortalDetails.Device, evidenceDrawerRow.device);
  appendDetail(metadata, PortalDetails.Platform, evidenceDrawerRow.platform);
  appendDetail(metadata, PortalDetails.Observer, evidenceDrawerRow.observer);
  appendDetail(metadata, PortalDetails.Capability, evidenceDrawerRow.capability);
  appendDetail(metadata, PortalDetails.RowEvidenceReferences, evidenceDrawerRow.evidenceReferences);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(evidenceDrawerRow.productClaim));

  panel.append(title, metadata);
  return panel;
}

function toDetail(value: PortalDisplayText | TrackingStatusProofArtifact): PortalDetailValue {
  return detailFromValue(value);
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function evidenceReferenceDetail(
  evidenceReferenceIds: AgentActivityTrackingEvidenceReferenceIds | undefined
): PortalDetailValue {
  if (evidenceReferenceIds === undefined || evidenceReferenceIds.length === 0) {
    return notReported();
  }
  return detailFromValue(evidenceReferenceIds.join(PortalFormatting.EventDetailSeparator));
}

function readModelRowDetail(
  rowValue: AgentActivityTrackingReadModelRow | undefined,
  select: (rowValue: AgentActivityTrackingReadModelRow) => unknown
): PortalDetailValue {
  if (rowValue === undefined) {
    return notReported();
  }
  return detailFromValue(select(rowValue));
}

function notReported(): PortalDetailValue {
  return toDetail(PortalText.Resolve(PortalTextToken.NotReported));
}
