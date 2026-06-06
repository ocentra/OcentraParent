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
  AgentActivityTrackingReadModel,
  AgentActivityTrackingReadModelRow,
} from '@ocentra-parent/agent-protocol-domain/tracking-read-model';
import {
  trackingChildCheckInProof,
  trackingChildRuntimeUiProof,
  type TrackingChildCheckInProof,
  type TrackingChildRuntimeUiProof,
} from './tracking-child-check-in-proof';
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
  readonly rowsReturned: PortalDetailValue;
  readonly lastObserved: PortalDetailValue;
  readonly eventId: PortalDetailValue;
  readonly capability: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly parserReason: PortalDetailValue | null;
  readonly productClaim: PortalDisplayText;
  readonly citations: readonly TrackingStatusLiveCitation[];
};

export type TrackingStatusLiveCitation = {
  readonly title: PortalDetailValue;
  readonly eventId: PortalDetailValue;
  readonly observedAt: PortalDetailValue;
  readonly device: PortalDetailValue;
  readonly platform: PortalDetailValue;
  readonly observer: PortalDetailValue;
  readonly activityKind: PortalDetailValue;
  readonly subject: PortalDetailValue;
  readonly status: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly productClaim: PortalDisplayText;
};

export type TrackingStatusServiceDataCoverage = {
  readonly title: PortalDisplayText;
  readonly loadState: PortalDetailValue;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly rowVisibility: PortalDetailValue;
  readonly lastObserved: PortalDetailValue;
  readonly eventId: PortalDetailValue;
  readonly capability: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly activityKinds: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly productClaim: PortalDisplayText;
};

export type TrackingUnsupportedManualPlatformRow = {
  readonly title: PortalDisplayText;
  readonly supportState: PortalDisplayText;
  readonly renderedState: PortalDisplayText;
};

export type TrackingUnsupportedManualPlatformProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly manualRequiredRows: PortalDetailValue;
  readonly unavailableRows: PortalDetailValue;
  readonly authorityRequiredRows: PortalDetailValue;
  readonly fakeCapabilityRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly evidence: PortalDisplayText;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly missingProof: PortalDisplayText;
  readonly boundary: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly rows: readonly TrackingUnsupportedManualPlatformRow[];
};

export type TrackingFamilyDashboardHostedRollupRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly visibleChildren: PortalDetailValue;
  readonly attentionItems: PortalDetailValue;
  readonly retainedAuditItems: PortalDetailValue;
  readonly evidence: PortalDisplayText;
};

export type TrackingFamilyDashboardHostedRollupProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingFamilyDashboardHostedRollupRow[];
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

type TrackingUnsupportedManualPlatformDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly supportStateToken: PortalTextTokenValue;
  readonly renderedStateToken: PortalTextTokenValue;
};

type TrackingFamilyDashboardHostedRollupDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly visibleChildren: number;
  readonly attentionItems: number;
  readonly retainedAuditItems: number;
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

const TrackingUnsupportedManualPlatformDefinitions = [
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualAndroidBackground,
    supportStateToken: PortalTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualAndroidGeofence,
    supportStateToken: PortalTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualIosBackground,
    supportStateToken: PortalTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualIosGeofence,
    supportStateToken: PortalTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualDesktopOs,
    supportStateToken: PortalTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualWebChildAgent,
    supportStateToken: PortalTextToken.TrackingSupportPlatformUnsupported,
    renderedStateToken: PortalTextToken.TrackingRenderedUnavailable,
  },
  {
    titleToken: PortalTextToken.TrackingUnsupportedManualAuthorityHardControl,
    supportStateToken: PortalTextToken.TrackingSupportRealDeviceRequired,
    renderedStateToken: PortalTextToken.TrackingRenderedAuthorityRequired,
  },
] as const satisfies readonly TrackingUnsupportedManualPlatformDefinition[];

const TrackingFamilyDashboardHostedRollupDefinitions = [
  {
    titleToken: PortalTextToken.TrackingFamilyDashboardActiveSummary,
    evidenceToken: PortalTextToken.TrackingFamilyDashboardActiveEvidence,
    visibleChildren: 2,
    attentionItems: 1,
    retainedAuditItems: 0,
  },
  {
    titleToken: PortalTextToken.TrackingFamilyDashboardChildAttention,
    evidenceToken: PortalTextToken.TrackingFamilyDashboardChildAttentionEvidence,
    visibleChildren: 1,
    attentionItems: 2,
    retainedAuditItems: 0,
  },
  {
    titleToken: PortalTextToken.TrackingFamilyDashboardRetentionAudit,
    evidenceToken: PortalTextToken.TrackingFamilyDashboardRetentionAuditEvidence,
    visibleChildren: 0,
    attentionItems: 0,
    retainedAuditItems: 2,
  },
] as const satisfies readonly TrackingFamilyDashboardHostedRollupDefinition[];

export function trackingStatusProofRows(): readonly TrackingStatusProofRow[] {
  return TrackingStatusProofRowDefinitions.map((definition) => row(definition));
}

export function trackingFamilyDashboardHostedRollupProof(): TrackingFamilyDashboardHostedRollupProof {
  const rows = TrackingFamilyDashboardHostedRollupDefinitions.map((definition) => familyDashboardRollupRow(definition));
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingFamilyDashboardRollup),
    body: PortalText.Resolve(PortalTextToken.TrackingFamilyDashboardRollupBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.FamilyDashboardRollup,
    boundary: PortalText.Resolve(PortalTextToken.TrackingFamilyDashboardHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    notificationReceiptClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    rows,
  };
}

export function trackingUnsupportedManualPlatformProof(): TrackingUnsupportedManualPlatformProof {
  const rows = TrackingUnsupportedManualPlatformDefinitions.map((definition) => unsupportedManualRow(definition));
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingUnsupportedManualProofTitle),
    body: PortalText.Resolve(PortalTextToken.TrackingUnsupportedManualProofBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofFixture),
    rowsReturned: detailFromValue(rows.length),
    manualRequiredRows: renderedStateCount(rows, PortalTextToken.TrackingRenderedManualRequired),
    unavailableRows: renderedStateCount(rows, PortalTextToken.TrackingRenderedUnavailable),
    authorityRequiredRows: renderedStateCount(rows, PortalTextToken.TrackingRenderedAuthorityRequired),
    fakeCapabilityRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    evidence: PortalText.Resolve(PortalTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.UnsupportedManualPlatform,
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    boundary: PortalText.Resolve(PortalTextToken.TrackingUnsupportedManualBoundary),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    rows,
  };
}

export function trackingStatusLiveSummary(liveActivity: PortalLiveActivityState): TrackingStatusLiveSummary {
  const event = liveActivity.activityTrackingReadModelEvent;
  const readModelResult = liveActivity.activityTrackingReadModel;
  const baseSummary = {
    title: PortalText.Resolve(PortalTextToken.TrackingServiceReadModel),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: notReported(),
    lastObserved: notReported(),
    eventId: notReported(),
    capability: notReported(),
    custody: notReported(),
    evidenceReferences: notReported(),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    citations: [],
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
  return {
    ...baseSummary,
    loadState: detailFromValue(event.severity),
    rowsReturned: detailFromValue(readModel.returned),
    lastObserved: detailFromValue(readModel.latestObservedAt),
    eventId: detailFromValue(readModel.latestEventId),
    capability: detailFromValue(readModel.capabilityStatus),
    custody: detailFromValue(readModel.custodyLabel),
    evidenceReferences: readModelEvidenceReferences(readModel),
    parserReason: null,
    citations: readModel.rows.map((readModelRow) => liveCitation(readModelRow)),
  };
}

export function trackingStatusServiceDataCoverage(
  liveActivity: PortalLiveActivityState
): TrackingStatusServiceDataCoverage {
  const event = liveActivity.activityTrackingReadModelEvent;
  const readModelResult = liveActivity.activityTrackingReadModel;
  const baseCoverage = {
    title: PortalText.Resolve(PortalTextToken.TrackingServiceDataCoverage),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: notReported(),
    rowVisibility: notReported(),
    lastObserved: notReported(),
    eventId: notReported(),
    capability: notReported(),
    custody: notReported(),
    activityKinds: notReported(),
    evidenceReferences: notReported(),
    deletedEvidence: notReported(),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };

  if (event === null || readModelResult === null) {
    return {
      ...baseCoverage,
      loadState: notReported(),
    };
  }

  if (!readModelResult.ok) {
    return {
      ...baseCoverage,
      loadState: detailFromValue(event.severity),
      rowVisibility: detailFromValue(readModelResult.reason),
    };
  }

  const readModel = readModelResult.value;
  return {
    ...baseCoverage,
    loadState: detailFromValue(event.severity),
    rowsReturned: detailFromValue(readModel.returned),
    rowVisibility: sequenceDetail([readModel.activeRows, readModel.tombstoneRows]),
    lastObserved: detailFromValue(readModel.latestTombstoneObservedAt ?? readModel.latestObservedAt),
    eventId: detailFromValue(readModel.latestTombstoneEventId ?? readModel.latestEventId),
    capability: detailFromValue(readModel.capabilityStatus),
    custody: detailFromValue(readModel.custodyLabel),
    activityKinds: listDetail(readModel.rows.map((readModelRow) => readModelRow.kind)),
    evidenceReferences: evidenceReferenceDetail(
      readModel.rows.flatMap((readModelRow) => readModelRow.evidenceReferenceIds)
    ),
    deletedEvidence: readModelDeletedEvidenceReferences(readModel),
  };
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
    const liveSummary = trackingStatusLiveSummary(liveActivity);
    dashboard.append(renderTrackingStatusLiveSummary(liveSummary));
    dashboard.append(renderTrackingStatusServiceDataCoverage(trackingStatusServiceDataCoverage(liveActivity)));
    dashboard.append(renderTrackingFamilyDashboardHostedRollupProof(trackingFamilyDashboardHostedRollupProof()));
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

function unsupportedManualRow(
  definition: TrackingUnsupportedManualPlatformDefinition
): TrackingUnsupportedManualPlatformRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    supportState: PortalText.Resolve(definition.supportStateToken),
    renderedState: PortalText.Resolve(definition.renderedStateToken),
  };
}

function familyDashboardRollupRow(
  definition: TrackingFamilyDashboardHostedRollupDefinition
): TrackingFamilyDashboardHostedRollupRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    status: PortalText.Resolve(PortalTextToken.TrackingFamilyDashboardRollupReady),
    visibleChildren: detailFromValue(definition.visibleChildren),
    attentionItems: detailFromValue(definition.attentionItems),
    retainedAuditItems: detailFromValue(definition.retainedAuditItems),
    evidence: PortalText.Resolve(definition.evidenceToken),
  };
}

function renderedStateCount(
  rows: readonly TrackingUnsupportedManualPlatformRow[],
  renderedStateToken: PortalTextTokenValue
): PortalDetailValue {
  const renderedState = PortalText.Resolve(renderedStateToken);
  return detailFromValue(rows.filter((rowValue) => rowValue.renderedState === renderedState).length);
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

function renderTrackingFamilyDashboardHostedRollupProof(
  proof: TrackingFamilyDashboardHostedRollupProof
): HTMLElement {
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
  appendDetail(metadata, PortalDetails.Capability, coverage.capability);
  appendDetail(metadata, PortalDetails.Custody, coverage.custody);
  appendDetail(metadata, PortalDetails.ActivityKind, coverage.activityKinds);
  appendDetail(metadata, PortalDetails.EvidenceReferences, coverage.evidenceReferences);
  appendDetail(metadata, PortalDetails.DeletedEvidence, coverage.deletedEvidence);
  appendDetail(metadata, PortalDetails.ProductClaim, toDetail(coverage.productClaim));

  panel.append(title, metadata);
  return panel;
}

function renderTrackingStatusLiveCitation(citation: TrackingStatusLiveCitation): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

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
  return detailFromValue(value);
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function evidenceReferenceDetail(
  evidenceReferenceIds: readonly AgentActivityTrackingEvidenceReferenceIds[number][] | undefined
): PortalDetailValue {
  if (evidenceReferenceIds === undefined || evidenceReferenceIds.length === 0) {
    return notReported();
  }
  return detailFromValue(evidenceReferenceIds.join(PortalFormatting.EventDetailSeparator));
}

function readModelEvidenceReferences(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const references = new Set<AgentActivityTrackingEvidenceReferenceIds[number]>();
  for (const row of readModel.rows) {
    for (const evidenceReferenceId of row.evidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
    for (const evidenceReferenceId of row.deletedEvidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  for (const evidenceReferenceId of readModel.deletedEvidenceReferenceIds) {
    references.add(evidenceReferenceId);
  }
  return evidenceReferenceDetail([...references]);
}

function readModelDeletedEvidenceReferences(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const references = new Set<AgentActivityTrackingEvidenceReferenceIds[number]>();
  for (const row of readModel.rows) {
    for (const evidenceReferenceId of row.deletedEvidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  for (const evidenceReferenceId of readModel.deletedEvidenceReferenceIds) {
    references.add(evidenceReferenceId);
  }
  return evidenceReferenceDetail([...references]);
}

function liveCitation(row: AgentActivityTrackingReadModelRow): TrackingStatusLiveCitation {
  return {
    title: detailFromValue(row.subjectDisplayName ?? row.kind),
    eventId: detailFromValue(row.eventId),
    observedAt: detailFromValue(row.observedAt),
    device: detailFromValue(row.deviceId),
    platform: detailFromValue(row.platform),
    observer: detailFromValue(row.observer),
    activityKind: detailFromValue(row.kind),
    subject: detailFromValue([row.subjectKind, row.subjectId].join(PortalFormatting.EventDetailSeparator)),
    status: detailFromValue(
      [row.queryVisibility, row.capabilityStatus]
        .filter((part) => part !== null && part !== undefined)
        .join(PortalFormatting.EventDetailSeparator)
    ),
    evidenceReferences: evidenceReferenceDetail(row.evidenceReferenceIds),
    deletedEvidence: evidenceReferenceDetail(row.deletedEvidenceReferenceIds),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };
}

function notReported(): PortalDetailValue {
  return toDetail(PortalText.Resolve(PortalTextToken.NotReported));
}

function listDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  return evidenceReferenceDetail([...new Set(normalizedValues)]);
}

function sequenceDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  if (normalizedValues.length === 0) {
    return notReported();
  }
  return detailFromValue(normalizedValues.join(PortalFormatting.EventDetailSeparator));
}
