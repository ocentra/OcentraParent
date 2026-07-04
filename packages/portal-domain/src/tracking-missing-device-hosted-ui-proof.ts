import { PortalDetails } from './details';
import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

export type TrackingMissingDeviceHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly state: PortalDetailValue;
  readonly primaryBadge: PortalDetailValue;
  readonly contactState: PortalDetailValue;
  readonly lastKnownEvidenceRef: PortalDetailValue;
  readonly deviceStatusEvidenceRef: PortalDetailValue;
  readonly actionRefs: PortalDetailValue;
  readonly manualProofRequirements: PortalDetailValue;
};

export type TrackingMissingDeviceHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly sourceProofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly renderedMissingDeviceRows: PortalDetailValue;
  readonly lastKnownOnlyRows: PortalDetailValue;
  readonly offlineRows: PortalDetailValue;
  readonly contactRequestedRows: PortalDetailValue;
  readonly manualRequiredRows: PortalDetailValue;
  readonly currentLocationRuntimeClaimedRows: PortalDetailValue;
  readonly poweredOffTrackingClaimedRows: PortalDetailValue;
  readonly remoteSyncRuntimeClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly physicalDeviceProofClaimedRows: PortalDetailValue;
  readonly osLostModeApiClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingMissingDeviceHostedUiRow[];
};

type TrackingMissingDeviceHostedDefinition = {
  readonly titleToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly stateToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly primaryBadgeToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly contactStateToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly lastKnownEvidenceToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly deviceStatusEvidenceToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly actionRefsToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly manualProofToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
};

const MissingDeviceDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingMissingDeviceLastKnownOnly,
    stateToken: PortalDevTextToken.TrackingMissingDeviceLastKnownState,
    primaryBadgeToken: PortalDevTextToken.TrackingMissingDeviceLastKnownBadge,
    contactStateToken: PortalDevTextToken.TrackingMissingDeviceOfflineContact,
    lastKnownEvidenceToken: PortalDevTextToken.TrackingMissingDeviceLastKnownEvidence,
    deviceStatusEvidenceToken: PortalDevTextToken.TrackingMissingDeviceOfflineStatusEvidence,
    actionRefsToken: PortalDevTextToken.TrackingMissingDeviceReviewCheckInAction,
    manualProofToken: PortalDevTextToken.TrackingMissingDeviceHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingMissingDevicePoweredOff,
    stateToken: PortalDevTextToken.TrackingMissingDeviceOfflineState,
    primaryBadgeToken: PortalDevTextToken.TrackingMissingDeviceOfflineBadge,
    contactStateToken: PortalDevTextToken.TrackingMissingDevicePoweredOffContact,
    lastKnownEvidenceToken: PortalDevTextToken.TrackingMissingDevicePoweredOffEvidence,
    deviceStatusEvidenceToken: PortalDevTextToken.TrackingMissingDevicePoweredOffStatusEvidence,
    actionRefsToken: PortalDevTextToken.TrackingMissingDeviceReviewCheckInAction,
    manualProofToken: PortalDevTextToken.TrackingMissingDevicePoweredOffManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingMissingDeviceContactRequested,
    stateToken: PortalDevTextToken.TrackingMissingDeviceContactRequestedState,
    primaryBadgeToken: PortalDevTextToken.TrackingMissingDeviceContactRequestedBadge,
    contactStateToken: PortalDevTextToken.TrackingMissingDeviceOnlineContact,
    lastKnownEvidenceToken: PortalDevTextToken.TrackingMissingDeviceContactRequestedEvidence,
    deviceStatusEvidenceToken: PortalDevTextToken.TrackingMissingDeviceContactStatusEvidence,
    actionRefsToken: PortalDevTextToken.TrackingMissingDeviceCallMarkFoundAction,
    manualProofToken: PortalDevTextToken.TrackingMissingDeviceHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingMissingDeviceManualRequired,
    stateToken: PortalDevTextToken.TrackingMissingDeviceManualRequiredState,
    primaryBadgeToken: PortalDevTextToken.TrackingMissingDeviceManualRequiredBadge,
    contactStateToken: PortalDevTextToken.TrackingMissingDeviceUnknownContact,
    lastKnownEvidenceToken: PortalDevTextToken.TrackingMissingDeviceManualEvidence,
    deviceStatusEvidenceToken: PortalDevTextToken.TrackingMissingDevicePlatformProofEvidence,
    actionRefsToken: PortalDevTextToken.TrackingMissingDeviceManualPlatformAction,
    manualProofToken: PortalDevTextToken.TrackingMissingDevicePlatformManualProof,
  },
] as const satisfies readonly TrackingMissingDeviceHostedDefinition[];

export function trackingMissingDeviceHostedUiProof(): TrackingMissingDeviceHostedUiProof {
  const rows = MissingDeviceDefinitions.map((definition) => hostedRow(definition));
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingMissingDeviceHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingMissingDeviceHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    sourceProofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingMissingDeviceHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
    renderedMissingDeviceRows: detailFromValue(rows.length),
    lastKnownOnlyRows: detailFromValue(1),
    offlineRows: detailFromValue(1),
    contactRequestedRows: detailFromValue(1),
    manualRequiredRows: detailFromValue(1),
    currentLocationRuntimeClaimedRows: zero(),
    poweredOffTrackingClaimedRows: zero(),
    remoteSyncRuntimeClaimedRows: zero(),
    providerDeliveryClaimedRows: zero(),
    physicalDeviceProofClaimedRows: zero(),
    osLostModeApiClaimedRows: zero(),
    productClaimReadyRows: zero(),
    rows,
  };
}

export const TrackingMissingDeviceHostedUiDetails = {
  ContactState: PortalDetails.RuntimeReference,
  MissingDeviceProof: PortalDetails.PolicyReadiness,
  PrimaryBadge: PortalDetails.DecisionAction,
} as const;

function hostedRow(definition: TrackingMissingDeviceHostedDefinition): TrackingMissingDeviceHostedUiRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    state: detailFromText(definition.stateToken),
    primaryBadge: detailFromText(definition.primaryBadgeToken),
    contactState: detailFromText(definition.contactStateToken),
    lastKnownEvidenceRef: detailFromText(definition.lastKnownEvidenceToken),
    deviceStatusEvidenceRef: detailFromText(definition.deviceStatusEvidenceToken),
    actionRefs: detailFromText(definition.actionRefsToken),
    manualProofRequirements: detailFromText(definition.manualProofToken),
  };
}

function detailFromText(token: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken]): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(token));
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function zero(): PortalDetailValue {
  return detailFromValue(0);
}
