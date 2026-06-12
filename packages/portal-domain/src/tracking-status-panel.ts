import type { DisplayText } from '@ocentra-parent/text-domain/contracts';

import type { PortalDetailValue } from './detail-values';
import type { TrackingStatusProofArtifact } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

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
