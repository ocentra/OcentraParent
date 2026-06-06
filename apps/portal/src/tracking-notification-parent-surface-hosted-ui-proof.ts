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

export type TrackingNotificationParentSurfaceHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly evidenceRefs: PortalDetailValue;
  readonly providerAttemptRef: PortalDetailValue;
  readonly receiptRequirementRefs: PortalDetailValue;
  readonly preferenceRequirementRefs: PortalDetailValue;
  readonly manualProofRequirements: PortalDetailValue;
  readonly redactedSummaryRef: PortalDetailValue;
};

export type TrackingNotificationParentSurfaceHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly renderedParentNotificationUiRows: PortalDetailValue;
  readonly parentPreferenceMutationRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly receiptIngestionClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productionStorageClaimedRows: PortalDetailValue;
  readonly adapterDispatchClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingNotificationParentSurfaceHostedUiRow[];
};

type TrackingNotificationParentSurfaceHostedDefinition = {
  readonly titleToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly statusToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly policyDecisionToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly evidenceToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly providerAttemptToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly receiptRequirementToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly preferenceRequirementToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly manualProofToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly redactedSummaryToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
};

const TrackingNotificationParentSurfaceHostedDefinitions = [
  {
    titleToken: PortalTextToken.TrackingNotificationParentSurfaceHistoryIntent,
    statusToken: PortalTextToken.TrackingNotificationParentSurfaceHistoryIntentReady,
    policyDecisionToken: PortalTextToken.TrackingNotificationParentSurfaceHomeDecision,
    evidenceToken: PortalTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalTextToken.TrackingNotificationParentSurfaceHomeAttempt,
    receiptRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceHomeReceiptRequirement,
    preferenceRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceHomePreferenceRequirement,
    manualProofToken: PortalTextToken.TrackingNotificationParentSurfaceHomeManualProof,
    redactedSummaryToken: PortalTextToken.TrackingNotificationParentSurfaceHomeSummary,
  },
  {
    titleToken: PortalTextToken.TrackingNotificationParentSurfaceManualAction,
    statusToken: PortalTextToken.TrackingNotificationParentSurfaceManualActionRequired,
    policyDecisionToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolDecision,
    evidenceToken: PortalTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolAttempt,
    receiptRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolReceiptRequirement,
    preferenceRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolPreferenceRequirement,
    manualProofToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolManualProof,
    redactedSummaryToken: PortalTextToken.TrackingNotificationParentSurfaceSchoolSummary,
  },
  {
    titleToken: PortalTextToken.TrackingNotificationParentSurfaceProviderUnavailable,
    statusToken: PortalTextToken.TrackingNotificationParentSurfaceProviderUnavailableStatus,
    policyDecisionToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailableDecision,
    evidenceToken: PortalTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailableAttempt,
    receiptRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailableReceiptRequirement,
    preferenceRequirementToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailablePreferenceRequirement,
    manualProofToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailableManualProof,
    redactedSummaryToken: PortalTextToken.TrackingNotificationParentSurfaceUnavailableSummary,
  },
] as const satisfies readonly TrackingNotificationParentSurfaceHostedDefinition[];

export function trackingNotificationParentSurfaceHostedUiProof(): TrackingNotificationParentSurfaceHostedUiProof {
  const rows = TrackingNotificationParentSurfaceHostedDefinitions.map((definition) => hostedRow(definition));
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingNotificationParentSurfaceHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingNotificationParentSurfaceHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.NotificationParentSurfaceHistory,
    boundary: PortalText.Resolve(PortalTextToken.TrackingNotificationParentSurfaceHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    renderedParentNotificationUiRows: detailFromValue(rows.length),
    parentPreferenceMutationRows: zero(),
    providerDeliveryClaimedRows: zero(),
    receiptIngestionClaimedRows: zero(),
    childDeviceDeliveryClaimedRows: zero(),
    physicalDeviceClaimedRows: zero(),
    authorityClaimedRows: zero(),
    productionStorageClaimedRows: zero(),
    adapterDispatchClaimedRows: zero(),
    productClaimReadyRows: zero(),
    rows,
  };
}

export const TrackingNotificationParentSurfaceHostedUiDetails = {
  PreferenceRequirement: PortalDetails.ParentRuleContextReferences,
  ProviderAttempt: PortalDetails.ProviderSource,
  ReceiptRequirement: PortalDetails.AdapterDispatch,
  RedactedSummary: PortalDetails.PrivacyMode,
} as const;

function hostedRow(
  definition: TrackingNotificationParentSurfaceHostedDefinition
): TrackingNotificationParentSurfaceHostedUiRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    status: detailFromText(definition.statusToken),
    policyDecisionRef: detailFromText(definition.policyDecisionToken),
    evidenceRefs: detailFromText(definition.evidenceToken),
    providerAttemptRef: detailFromText(definition.providerAttemptToken),
    receiptRequirementRefs: detailFromText(definition.receiptRequirementToken),
    preferenceRequirementRefs: detailFromText(definition.preferenceRequirementToken),
    manualProofRequirements: detailFromText(definition.manualProofToken),
    redactedSummaryRef: detailFromText(definition.redactedSummaryToken),
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
