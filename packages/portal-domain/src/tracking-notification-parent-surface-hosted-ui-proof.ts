import { PortalDetails } from './details';
import type { DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { TrackingStatusProofArtifacts, type TrackingStatusProofArtifact } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

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
  readonly titleToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly statusToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly policyDecisionToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly evidenceToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly providerAttemptToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly receiptRequirementToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly preferenceRequirementToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly manualProofToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly redactedSummaryToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
};

const TrackingNotificationParentSurfaceHostedDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntent,
    statusToken: PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntentReady,
    policyDecisionToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomeDecision,
    evidenceToken: PortalDevTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomeAttempt,
    receiptRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomeReceiptRequirement,
    preferenceRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomePreferenceRequirement,
    manualProofToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomeManualProof,
    redactedSummaryToken: PortalDevTextToken.TrackingNotificationParentSurfaceHomeSummary,
  },
  {
    titleToken: PortalDevTextToken.TrackingNotificationParentSurfaceManualAction,
    statusToken: PortalDevTextToken.TrackingNotificationParentSurfaceManualActionRequired,
    policyDecisionToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolDecision,
    evidenceToken: PortalDevTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolAttempt,
    receiptRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolReceiptRequirement,
    preferenceRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolPreferenceRequirement,
    manualProofToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolManualProof,
    redactedSummaryToken: PortalDevTextToken.TrackingNotificationParentSurfaceSchoolSummary,
  },
  {
    titleToken: PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailable,
    statusToken: PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailableStatus,
    policyDecisionToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableDecision,
    evidenceToken: PortalDevTextToken.TrackingNotificationParentSurfaceLocationEvidence,
    providerAttemptToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableAttempt,
    receiptRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableReceiptRequirement,
    preferenceRequirementToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailablePreferenceRequirement,
    manualProofToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableManualProof,
    redactedSummaryToken: PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableSummary,
  },
] as const satisfies readonly TrackingNotificationParentSurfaceHostedDefinition[];

export function trackingNotificationParentSurfaceHostedUiProof(): TrackingNotificationParentSurfaceHostedUiProof {
  const rows = TrackingNotificationParentSurfaceHostedDefinitions.map((definition) => hostedRow(definition));
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.NotificationParentSurfaceHistory,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    title: resolvePortalDevText(definition.titleToken),
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

function detailFromText(token: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken]): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(token));
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function zero(): PortalDetailValue {
  return detailFromValue(0);
}
