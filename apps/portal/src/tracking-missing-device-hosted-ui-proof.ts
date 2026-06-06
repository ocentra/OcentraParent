import {
  PortalDetails,
  PortalText,
  PortalTextToken,
  TrackingStatusProofArtifacts,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';

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
  readonly titleToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly stateToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly primaryBadgeToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly contactStateToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly lastKnownEvidenceToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly deviceStatusEvidenceToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly actionRefsToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly manualProofToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
};

const MissingDeviceDefinitions = [
  {
    titleToken: PortalTextToken.TrackingMissingDeviceLastKnownOnly,
    stateToken: PortalTextToken.TrackingMissingDeviceLastKnownState,
    primaryBadgeToken: PortalTextToken.TrackingMissingDeviceLastKnownBadge,
    contactStateToken: PortalTextToken.TrackingMissingDeviceOfflineContact,
    lastKnownEvidenceToken: PortalTextToken.TrackingMissingDeviceLastKnownEvidence,
    deviceStatusEvidenceToken: PortalTextToken.TrackingMissingDeviceOfflineStatusEvidence,
    actionRefsToken: PortalTextToken.TrackingMissingDeviceReviewCheckInAction,
    manualProofToken: PortalTextToken.TrackingMissingDeviceHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingMissingDevicePoweredOff,
    stateToken: PortalTextToken.TrackingMissingDeviceOfflineState,
    primaryBadgeToken: PortalTextToken.TrackingMissingDeviceOfflineBadge,
    contactStateToken: PortalTextToken.TrackingMissingDevicePoweredOffContact,
    lastKnownEvidenceToken: PortalTextToken.TrackingMissingDevicePoweredOffEvidence,
    deviceStatusEvidenceToken: PortalTextToken.TrackingMissingDevicePoweredOffStatusEvidence,
    actionRefsToken: PortalTextToken.TrackingMissingDeviceReviewCheckInAction,
    manualProofToken: PortalTextToken.TrackingMissingDevicePoweredOffManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingMissingDeviceContactRequested,
    stateToken: PortalTextToken.TrackingMissingDeviceContactRequestedState,
    primaryBadgeToken: PortalTextToken.TrackingMissingDeviceContactRequestedBadge,
    contactStateToken: PortalTextToken.TrackingMissingDeviceOnlineContact,
    lastKnownEvidenceToken: PortalTextToken.TrackingMissingDeviceContactRequestedEvidence,
    deviceStatusEvidenceToken: PortalTextToken.TrackingMissingDeviceContactStatusEvidence,
    actionRefsToken: PortalTextToken.TrackingMissingDeviceCallMarkFoundAction,
    manualProofToken: PortalTextToken.TrackingMissingDeviceHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingMissingDeviceManualRequired,
    stateToken: PortalTextToken.TrackingMissingDeviceManualRequiredState,
    primaryBadgeToken: PortalTextToken.TrackingMissingDeviceManualRequiredBadge,
    contactStateToken: PortalTextToken.TrackingMissingDeviceUnknownContact,
    lastKnownEvidenceToken: PortalTextToken.TrackingMissingDeviceManualEvidence,
    deviceStatusEvidenceToken: PortalTextToken.TrackingMissingDevicePlatformProofEvidence,
    actionRefsToken: PortalTextToken.TrackingMissingDeviceManualPlatformAction,
    manualProofToken: PortalTextToken.TrackingMissingDevicePlatformManualProof,
  },
] as const satisfies readonly TrackingMissingDeviceHostedDefinition[];

export function trackingMissingDeviceHostedUiProof(): TrackingMissingDeviceHostedUiProof {
  const rows = MissingDeviceDefinitions.map((definition) => hostedRow(definition));
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingMissingDeviceHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingMissingDeviceHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    sourceProofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
    boundary: PortalText.Resolve(PortalTextToken.TrackingMissingDeviceHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
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
    title: PortalText.Resolve(definition.titleToken),
    state: detailFromText(definition.stateToken),
    primaryBadge: detailFromText(definition.primaryBadgeToken),
    contactState: detailFromText(definition.contactStateToken),
    lastKnownEvidenceRef: detailFromText(definition.lastKnownEvidenceToken),
    deviceStatusEvidenceRef: detailFromText(definition.deviceStatusEvidenceToken),
    actionRefs: detailFromText(definition.actionRefsToken),
    manualProofRequirements: detailFromText(definition.manualProofToken),
  };
}

function detailFromText(token: (typeof PortalTextToken)[keyof typeof PortalTextToken]): PortalDetailValue {
  return detailFromValue(PortalText.Resolve(token));
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function zero(): PortalDetailValue {
  return detailFromValue(0);
}
