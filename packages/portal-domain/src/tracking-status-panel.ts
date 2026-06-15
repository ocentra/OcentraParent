import type { DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import type {
  AgentActivityTrackingEvidenceReferenceIds,
  AgentActivityTrackingReadModelCount,
  AgentActivityTrackingReadModel,
  AgentActivityTrackingReadModelResult,
  AgentActivityTrackingReadModelRow,
} from '@ocentra-parent/agent-protocol-domain/tracking-read-model';

import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalFormatting } from './formatting';
import { TrackingStatusProofArtifacts, type TrackingStatusProofArtifact } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;
type PortalTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

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
  readonly deviceCounts: PortalDetailValue;
  readonly capability: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly activityKinds: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly productClaim: PortalDisplayText;
};

export type TrackingStatusLiveProjectionInput = {
  readonly activityTrackingReadModelEvent: AgentEventEnvelope | null;
  readonly activityTrackingReadModel: AgentActivityTrackingReadModelResult | null;
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

const TrackingStatusProofRowDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingStateDisabled,
    evidenceToken: PortalDevTextToken.TrackingEvidenceContracts,
    proofArtifact: TrackingStatusProofArtifacts.ContractBoundary,
  },
  {
    titleToken: PortalDevTextToken.TrackingStatePermissionRequired,
    evidenceToken: PortalDevTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.PermissionCapability,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateStale,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateOffline,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.DeviceStatus,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateLowAccuracy,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateAmbiguousNearby,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.NearbyPlace,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateAlert,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.AlertSeverity,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateAcknowledged,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgement,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateException,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgement,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateChildCheckIn,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateTemporaryLive,
    evidenceToken: PortalDevTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.TemporaryLiveMode,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateMissingDevice,
    evidenceToken: PortalDevTextToken.TrackingEvidencePhysicalMissing,
    proofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
  },
  {
    titleToken: PortalDevTextToken.TrackingStateRetentionDeleted,
    evidenceToken: PortalDevTextToken.TrackingEvidenceUiFixture,
    proofArtifact: TrackingStatusProofArtifacts.RetentionDelete,
    retentionProof: {
      historyVisibility: PortalDevTextToken.TrackingRetentionHistoryHidden,
      deletedEvidence: PortalDevTextToken.TrackingDeletedEvidenceNotRendered,
    },
  },
] as const satisfies readonly TrackingStatusProofRowDefinition[];

const TrackingUnsupportedManualPlatformDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualAndroidBackground,
    supportStateToken: PortalDevTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualAndroidGeofence,
    supportStateToken: PortalDevTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualIosBackground,
    supportStateToken: PortalDevTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualIosGeofence,
    supportStateToken: PortalDevTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualDesktopOs,
    supportStateToken: PortalDevTextToken.TrackingSupportManualRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedManualRequired,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualWebChildAgent,
    supportStateToken: PortalDevTextToken.TrackingSupportPlatformUnsupported,
    renderedStateToken: PortalDevTextToken.TrackingRenderedUnavailable,
  },
  {
    titleToken: PortalDevTextToken.TrackingUnsupportedManualAuthorityHardControl,
    supportStateToken: PortalDevTextToken.TrackingSupportRealDeviceRequired,
    renderedStateToken: PortalDevTextToken.TrackingRenderedAuthorityRequired,
  },
] as const satisfies readonly TrackingUnsupportedManualPlatformDefinition[];

const TrackingFamilyDashboardHostedRollupDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingFamilyDashboardActiveSummary,
    evidenceToken: PortalDevTextToken.TrackingFamilyDashboardActiveEvidence,
    visibleChildren: 2,
    attentionItems: 1,
    retainedAuditItems: 0,
  },
  {
    titleToken: PortalDevTextToken.TrackingFamilyDashboardChildAttention,
    evidenceToken: PortalDevTextToken.TrackingFamilyDashboardChildAttentionEvidence,
    visibleChildren: 1,
    attentionItems: 2,
    retainedAuditItems: 0,
  },
  {
    titleToken: PortalDevTextToken.TrackingFamilyDashboardRetentionAudit,
    evidenceToken: PortalDevTextToken.TrackingFamilyDashboardRetentionAuditEvidence,
    visibleChildren: 0,
    attentionItems: 0,
    retainedAuditItems: 2,
  },
] as const satisfies readonly TrackingFamilyDashboardHostedRollupDefinition[];

export function trackingStatusProofRows(): readonly TrackingStatusProofRow[] {
  return TrackingStatusProofRowDefinitions.map((definition) => row(definition));
}

export function trackingStatusLiveSummary(input: TrackingStatusLiveProjectionInput): TrackingStatusLiveSummary {
  const event = input.activityTrackingReadModelEvent;
  const readModelResult = input.activityTrackingReadModel;
  const baseSummary = {
    title: resolvePortalDevText(PortalDevTextToken.TrackingServiceReadModel),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: notReported(),
    lastObserved: notReported(),
    eventId: notReported(),
    capability: notReported(),
    custody: notReported(),
    evidenceReferences: notReported(),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    lastObserved: preferredActiveSummaryDetail(readModel.latestActiveObservedAt, readModel.latestObservedAt),
    eventId: preferredActiveSummaryDetail(readModel.latestActiveEventId, readModel.latestEventId),
    capability: detailFromValue(readModel.capabilityStatus),
    custody: detailFromValue(readModel.custodyLabel),
    evidenceReferences: activeReadModelEvidenceReferences(readModel),
    parserReason: null,
    citations: readModel.rows.map((readModelRow) => liveCitation(readModelRow)),
  };
}

export function trackingStatusServiceDataCoverage(
  input: TrackingStatusLiveProjectionInput
): TrackingStatusServiceDataCoverage {
  const event = input.activityTrackingReadModelEvent;
  const readModelResult = input.activityTrackingReadModel;
  const baseCoverage = {
    title: resolvePortalDevText(PortalDevTextToken.TrackingServiceDataCoverage),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: notReported(),
    rowVisibility: notReported(),
    lastObserved: notReported(),
    eventId: notReported(),
    deviceCounts: notReported(),
    capability: notReported(),
    custody: notReported(),
    activityKinds: notReported(),
    evidenceReferences: notReported(),
    deletedEvidence: notReported(),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    deviceCounts: readModelDeviceCoverage(readModel),
    capability: readModelCapabilityCoverage(readModel),
    custody: detailFromValue(readModel.custodyLabel),
    activityKinds: readModelActivityKindCoverage(readModel),
    evidenceReferences: activeReadModelEvidenceReferences(readModel),
    deletedEvidence: readModelDeletedEvidenceReferences(readModel),
  };
}

export function trackingFamilyDashboardHostedRollupProof(): TrackingFamilyDashboardHostedRollupProof {
  const rows = TrackingFamilyDashboardHostedRollupDefinitions.map((definition) => familyDashboardRollupRow(definition));
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingFamilyDashboardRollup),
    body: resolvePortalDevText(PortalDevTextToken.TrackingFamilyDashboardRollupBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.FamilyDashboardRollup,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingFamilyDashboardHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    title: resolvePortalDevText(PortalDevTextToken.TrackingUnsupportedManualProofTitle),
    body: resolvePortalDevText(PortalDevTextToken.TrackingUnsupportedManualProofBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofFixture),
    rowsReturned: detailFromValue(rows.length),
    manualRequiredRows: renderedStateCount(rows, PortalDevTextToken.TrackingRenderedManualRequired),
    unavailableRows: renderedStateCount(rows, PortalDevTextToken.TrackingRenderedUnavailable),
    authorityRequiredRows: renderedStateCount(rows, PortalDevTextToken.TrackingRenderedAuthorityRequired),
    fakeCapabilityRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    evidence: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.UnsupportedManualPlatform,
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingUnsupportedManualBoundary),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
    rows,
  };
}

function row(definition: TrackingStatusProofRowDefinition): TrackingStatusProofRow {
  const { titleToken, evidenceToken, proofArtifact } = definition;
  const baseRow = {
    title: resolvePortalDevText(titleToken),
    state: resolvePortalDevText(titleToken),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofFixture),
    evidence: resolvePortalDevText(evidenceToken),
    proofArtifact,
    missingProof: missingProofForEvidence(evidenceToken),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
  const retentionProof = definition.retentionProof;
  if (retentionProof === undefined) {
    return baseRow;
  }
  return {
    ...baseRow,
    historyVisibility: resolvePortalDevText(retentionProof.historyVisibility),
    deletedEvidence: resolvePortalDevText(retentionProof.deletedEvidence),
  };
}

function unsupportedManualRow(
  definition: TrackingUnsupportedManualPlatformDefinition
): TrackingUnsupportedManualPlatformRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    supportState: resolvePortalDevText(definition.supportStateToken),
    renderedState: resolvePortalDevText(definition.renderedStateToken),
  };
}

function familyDashboardRollupRow(
  definition: TrackingFamilyDashboardHostedRollupDefinition
): TrackingFamilyDashboardHostedRollupRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    status: resolvePortalDevText(PortalDevTextToken.TrackingFamilyDashboardRollupReady),
    visibleChildren: detailFromValue(definition.visibleChildren),
    attentionItems: detailFromValue(definition.attentionItems),
    retainedAuditItems: detailFromValue(definition.retainedAuditItems),
    evidence: resolvePortalDevText(definition.evidenceToken),
  };
}

function renderedStateCount(
  rows: readonly TrackingUnsupportedManualPlatformRow[],
  renderedStateToken: PortalTextTokenValue
): PortalDetailValue {
  const renderedState = resolvePortalDevText(renderedStateToken);
  return detailFromValue(rows.filter((rowValue) => rowValue.renderedState === renderedState).length);
}

function missingProofForEvidence(evidenceToken: PortalTextTokenValue): PortalDisplayText {
  if (evidenceToken === PortalDevTextToken.TrackingEvidencePhysicalMissing) {
    return resolvePortalDevText(PortalDevTextToken.TrackingPhysicalDeviceRequired);
  }
  return resolvePortalDevText(PortalDevTextToken.TrackingManualRequired);
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}

function preferredActiveSummaryDetail(activeValue: unknown, legacyValue: unknown): PortalDetailValue {
  return detailFromValue(activeValue ?? legacyValue);
}

function evidenceReferenceDetail(
  evidenceReferenceIds: readonly AgentActivityTrackingEvidenceReferenceIds[number][] | undefined
): PortalDetailValue {
  if (evidenceReferenceIds === undefined || evidenceReferenceIds.length === 0) {
    return notReported();
  }
  return detailFromValue(evidenceReferenceIds.join(PortalFormatting.EventDetailSeparator));
}

function activeReadModelRows(readModel: AgentActivityTrackingReadModel): readonly AgentActivityTrackingReadModelRow[] {
  return readModel.rows.filter((rowValue) => rowValue.deletedAt === null);
}

function activeReadModelEvidenceReferences(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const references = new Set<AgentActivityTrackingEvidenceReferenceIds[number]>();
  for (const rowValue of activeReadModelRows(readModel)) {
    for (const evidenceReferenceId of rowValue.evidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  return evidenceReferenceDetail([...references]);
}

function readModelDeletedEvidenceReferences(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const references = new Set<AgentActivityTrackingEvidenceReferenceIds[number]>();
  for (const rowValue of readModel.rows) {
    for (const evidenceReferenceId of rowValue.deletedEvidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  for (const evidenceReferenceId of readModel.deletedEvidenceReferenceIds) {
    references.add(evidenceReferenceId);
  }
  return evidenceReferenceDetail([...references]);
}

function liveCitation(rowValue: AgentActivityTrackingReadModelRow): TrackingStatusLiveCitation {
  return {
    title: detailFromValue(rowValue.subjectDisplayName ?? rowValue.kind),
    eventId: detailFromValue(rowValue.eventId),
    observedAt: detailFromValue(rowValue.observedAt),
    device: detailFromValue(rowValue.deviceId),
    platform: detailFromValue(rowValue.platform),
    observer: detailFromValue(rowValue.observer),
    activityKind: detailFromValue(rowValue.kind),
    subject: detailFromValue([rowValue.subjectKind, rowValue.subjectId].join(PortalFormatting.EventDetailSeparator)),
    status: detailFromValue(
      [rowValue.queryVisibility, rowValue.capabilityStatus]
        .filter((part) => part !== null && part !== undefined)
        .join(PortalFormatting.EventDetailSeparator)
    ),
    evidenceReferences: evidenceReferenceDetail(rowValue.evidenceReferenceIds),
    deletedEvidence: evidenceReferenceDetail(rowValue.deletedEvidenceReferenceIds),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
}

function listDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  return evidenceReferenceDetail([...new Set(normalizedValues)]);
}

function countDetail(values: readonly AgentActivityTrackingReadModelCount[]): PortalDetailValue {
  if (values.length === 0) {
    return notReported();
  }
  return detailFromValue(
    values.map((value) => `${value.value} (${String(value.count)})`).join(PortalFormatting.EventDetailSeparator)
  );
}

function readModelActivityKindCoverage(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const activeKindCounts = readModel.activeKindCounts ?? [];
  if (activeKindCounts.length > 0) {
    return countDetail(activeKindCounts);
  }
  return listDetail(activeReadModelRows(readModel).map((rowValue) => rowValue.kind));
}

function readModelDeviceCoverage(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const activeDeviceCounts = readModel.activeDeviceCounts ?? [];
  if (activeDeviceCounts.length > 0) {
    return countDetail(activeDeviceCounts);
  }
  return listDetail(activeReadModelRows(readModel).map((rowValue) => rowValue.deviceId));
}

function readModelCapabilityCoverage(readModel: AgentActivityTrackingReadModel): PortalDetailValue {
  const activeCapabilityStatusCounts = readModel.activeCapabilityStatusCounts ?? [];
  if (activeCapabilityStatusCounts.length > 0) {
    return countDetail(activeCapabilityStatusCounts);
  }
  return detailFromValue(readModel.capabilityStatus);
}

function sequenceDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  if (normalizedValues.length === 0) {
    return notReported();
  }
  return detailFromValue(normalizedValues.join(PortalFormatting.EventDetailSeparator));
}
