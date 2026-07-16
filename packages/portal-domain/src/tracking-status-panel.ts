import { type PortalRouteEventRecord } from './portal-contract-adapter';
import { type PortalDetailValue, type TrackingStatusProofArtifact } from './portal-contract-text-contracts';
import type { PortalActivityTrackingReadModelResult } from './live-activity-state';
import {
  trackingFamilyDashboardHostedRollupProof as trackingFamilyDashboardHostedRollupProofImpl,
  trackingStatusLiveSummary as trackingStatusLiveSummaryImpl,
  trackingStatusProofRows as trackingStatusProofRowsImpl,
  trackingStatusServiceDataCoverage as trackingStatusServiceDataCoverageImpl,
  trackingUnsupportedManualPlatformProof as trackingUnsupportedManualPlatformProofImpl,
} from './tracking-status-panel-helpers';

type PortalDisplayText = string;

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
  readonly activityTrackingReadModelEvent: PortalRouteEventRecord | null;
  readonly activityTrackingReadModel: PortalActivityTrackingReadModelResult | null;
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

export function trackingStatusProofRows(): readonly TrackingStatusProofRow[] {
  return trackingStatusProofRowsImpl();
}

export function trackingStatusLiveSummary(input: TrackingStatusLiveProjectionInput): TrackingStatusLiveSummary {
  return trackingStatusLiveSummaryImpl(input);
}

export function trackingStatusServiceDataCoverage(
  input: TrackingStatusLiveProjectionInput
): TrackingStatusServiceDataCoverage {
  return trackingStatusServiceDataCoverageImpl(input);
}

export function trackingFamilyDashboardHostedRollupProof(): TrackingFamilyDashboardHostedRollupProof {
  return trackingFamilyDashboardHostedRollupProofImpl();
}

export function trackingUnsupportedManualPlatformProof(): TrackingUnsupportedManualPlatformProof {
  return trackingUnsupportedManualPlatformProofImpl();
}
