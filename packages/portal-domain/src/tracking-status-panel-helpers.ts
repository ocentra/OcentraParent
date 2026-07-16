import { PortalDevTextToken, resolvePortalDevText } from './display-text';
import { type TrackingStatusProofArtifact } from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';
import { GeneratedPortalTrackingContracts as GeneratedPortalTrackingContractsValue } from './generated-portal-contracts';
import {
  detailFromValue,
  familyDashboardRollupRow,
  notReported,
  renderedStateCount,
  row,
  unsupportedManualRow,
  preferredActiveSummaryDetail,
} from './tracking-status-panel-helpers-core';
import {
  activeReadModelEvidenceReferences,
  liveCitation,
  readModelActivityKindCoverage,
  readModelCapabilityCoverage,
  readModelDeviceCoverage,
  readModelDeletedEvidenceReferences,
  sequenceDetail,
} from './tracking-status-panel-helpers-read-model';
import type {
  TrackingFamilyDashboardHostedRollupProof,
  TrackingStatusLiveProjectionInput,
  TrackingStatusLiveSummary,
  TrackingStatusProofRow,
  TrackingStatusServiceDataCoverage,
  TrackingUnsupportedManualPlatformProof,
} from './tracking-status-panel';

void GeneratedPortalTrackingContractsValue;
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
