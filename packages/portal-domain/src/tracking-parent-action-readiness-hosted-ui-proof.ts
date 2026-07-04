import { PortalDetails } from './details';
import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

export type TrackingParentActionReadinessHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDetailValue;
  readonly primaryActionRef: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly evidenceRefs: PortalDetailValue;
  readonly uiSurfaceRef: PortalDetailValue;
  readonly manualProofRequirements: PortalDetailValue;
};

export type TrackingParentActionReadinessHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly expectedPlaceProofArtifact: TrackingStatusProofArtifact;
  readonly acknowledgementProofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly expectedPlaceRows: PortalDetailValue;
  readonly acknowledgementActionRows: PortalDetailValue;
  readonly renderedParentActionRows: PortalDetailValue;
  readonly liveServiceMutationRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly childDeviceRuntimeClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productionWorkerClaimedRows: PortalDetailValue;
  readonly adapterDispatchClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingParentActionReadinessHostedUiRow[];
};

type TrackingParentActionReadinessHostedDefinition = {
  readonly titleToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly statusToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly primaryActionToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly policyDecisionToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly evidenceToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly uiSurfaceToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
  readonly manualProofToken: (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];
};

const ExpectedPlaceDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingParentActionExpectedPlaceAlert,
    statusToken: PortalDevTextToken.TrackingParentActionAlertPolicyReady,
    primaryActionToken: PortalDevTextToken.TrackingParentActionNotifyParent,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionExpectedPlaceCheckIn,
    statusToken: PortalDevTextToken.TrackingParentActionCheckInPolicyReady,
    primaryActionToken: PortalDevTextToken.TrackingParentActionAskChildCheckIn,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionExpectedPlaceSuppressed,
    statusToken: PortalDevTextToken.TrackingParentActionSuppressedNoAction,
    primaryActionToken: PortalDevTextToken.TrackingParentActionNoAction,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionExpectedPlaceHolidayDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceHolidayEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceHolidaySurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionExpectedPlaceManual,
    statusToken: PortalDevTextToken.TrackingParentActionManualRequired,
    primaryActionToken: PortalDevTextToken.TrackingParentActionManualReview,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracyDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracyEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracySurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionExpectedPlaceManualProof,
  },
] as const satisfies readonly TrackingParentActionReadinessHostedDefinition[];

const AcknowledgementDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingParentActionAcknowledgementRecorded,
    statusToken: PortalDevTextToken.TrackingParentActionAcknowledgementRecordedStatus,
    primaryActionToken: PortalDevTextToken.TrackingParentActionAcknowledgeSafe,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionSafeDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionSafeEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionSafeSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionExceptionActive,
    statusToken: PortalDevTextToken.TrackingParentActionExceptionActiveStatus,
    primaryActionToken: PortalDevTextToken.TrackingParentActionMarkExpected,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionExpectedDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionExpectedEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionExpectedSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionFalseAlarmRecorded,
    statusToken: PortalDevTextToken.TrackingParentActionFalseAlarmRecordedStatus,
    primaryActionToken: PortalDevTextToken.TrackingParentActionMarkFalseAlarm,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionFalseAlarmDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionFalseAlarmEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionFalseAlarmSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionChildCheckInReady,
    statusToken: PortalDevTextToken.TrackingParentActionChildCheckInRequestReady,
    primaryActionToken: PortalDevTextToken.TrackingParentActionRequestChildCheckIn,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionChildCheckInDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionChildCheckInEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionChildCheckInSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionChildRuntimeManualProof,
  },
  {
    titleToken: PortalDevTextToken.TrackingParentActionCriticalReviewReady,
    statusToken: PortalDevTextToken.TrackingParentActionEscalationReviewReady,
    primaryActionToken: PortalDevTextToken.TrackingParentActionEscalateManualReview,
    policyDecisionToken: PortalDevTextToken.TrackingParentActionCriticalReviewDecision,
    evidenceToken: PortalDevTextToken.TrackingParentActionCriticalReviewEvidence,
    uiSurfaceToken: PortalDevTextToken.TrackingParentActionCriticalReviewSurface,
    manualProofToken: PortalDevTextToken.TrackingParentActionEscalationManualProof,
  },
] as const satisfies readonly TrackingParentActionReadinessHostedDefinition[];

export function trackingParentActionReadinessHostedUiProof(): TrackingParentActionReadinessHostedUiProof {
  const expectedPlaceRows = ExpectedPlaceDefinitions.map((definition) => hostedRow(definition));
  const acknowledgementActionRows = AcknowledgementDefinitions.map((definition) => hostedRow(definition));
  const rows = [...expectedPlaceRows, ...acknowledgementActionRows];
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingParentActionReadinessHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingParentActionReadinessHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    expectedPlaceProofArtifact: TrackingStatusProofArtifacts.ExpectedPlaceAlertPolicy,
    acknowledgementProofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgementActionReadiness,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingParentActionReadinessHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
    expectedPlaceRows: detailFromValue(expectedPlaceRows.length),
    acknowledgementActionRows: detailFromValue(acknowledgementActionRows.length),
    renderedParentActionRows: detailFromValue(rows.length),
    liveServiceMutationRows: zero(),
    providerDeliveryClaimedRows: zero(),
    notificationReceiptClaimedRows: zero(),
    childDeviceRuntimeClaimedRows: zero(),
    physicalDeviceClaimedRows: zero(),
    authorityClaimedRows: zero(),
    productionWorkerClaimedRows: zero(),
    adapterDispatchClaimedRows: zero(),
    productClaimReadyRows: zero(),
    rows,
  };
}

export const TrackingParentActionReadinessHostedUiDetails = {
  AcknowledgementProof: PortalDetails.ParentRuleContextReferences,
  ExpectedPlaceProof: PortalDetails.PolicyReadiness,
  PrimaryAction: PortalDetails.DecisionAction,
  UiSurface: PortalDetails.RuntimeReference,
} as const;

function hostedRow(
  definition: TrackingParentActionReadinessHostedDefinition
): TrackingParentActionReadinessHostedUiRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    status: detailFromText(definition.statusToken),
    primaryActionRef: detailFromText(definition.primaryActionToken),
    policyDecisionRef: detailFromText(definition.policyDecisionToken),
    evidenceRefs: detailFromText(definition.evidenceToken),
    uiSurfaceRef: detailFromText(definition.uiSurfaceToken),
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
