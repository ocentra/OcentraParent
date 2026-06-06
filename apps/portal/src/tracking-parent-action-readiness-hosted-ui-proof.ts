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
  readonly titleToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly statusToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly primaryActionToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly policyDecisionToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly evidenceToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly uiSurfaceToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
  readonly manualProofToken: (typeof PortalTextToken)[keyof typeof PortalTextToken];
};

const ExpectedPlaceDefinitions = [
  {
    titleToken: PortalTextToken.TrackingParentActionExpectedPlaceAlert,
    statusToken: PortalTextToken.TrackingParentActionAlertPolicyReady,
    primaryActionToken: PortalTextToken.TrackingParentActionNotifyParent,
    policyDecisionToken: PortalTextToken.TrackingParentActionExpectedPlaceSchoolDecision,
    evidenceToken: PortalTextToken.TrackingParentActionExpectedPlaceSchoolEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionExpectedPlaceSchoolSurface,
    manualProofToken: PortalTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionExpectedPlaceCheckIn,
    statusToken: PortalTextToken.TrackingParentActionCheckInPolicyReady,
    primaryActionToken: PortalTextToken.TrackingParentActionAskChildCheckIn,
    policyDecisionToken: PortalTextToken.TrackingParentActionExpectedPlaceLateBusDecision,
    evidenceToken: PortalTextToken.TrackingParentActionExpectedPlaceLateBusEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionExpectedPlaceLateBusSurface,
    manualProofToken: PortalTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionExpectedPlaceSuppressed,
    statusToken: PortalTextToken.TrackingParentActionSuppressedNoAction,
    primaryActionToken: PortalTextToken.TrackingParentActionNoAction,
    policyDecisionToken: PortalTextToken.TrackingParentActionExpectedPlaceHolidayDecision,
    evidenceToken: PortalTextToken.TrackingParentActionExpectedPlaceHolidayEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionExpectedPlaceHolidaySurface,
    manualProofToken: PortalTextToken.TrackingParentActionHostedReadOnlyManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionExpectedPlaceManual,
    statusToken: PortalTextToken.TrackingParentActionManualRequired,
    primaryActionToken: PortalTextToken.TrackingParentActionManualReview,
    policyDecisionToken: PortalTextToken.TrackingParentActionExpectedPlaceLowAccuracyDecision,
    evidenceToken: PortalTextToken.TrackingParentActionExpectedPlaceLowAccuracyEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionExpectedPlaceLowAccuracySurface,
    manualProofToken: PortalTextToken.TrackingParentActionExpectedPlaceManualProof,
  },
] as const satisfies readonly TrackingParentActionReadinessHostedDefinition[];

const AcknowledgementDefinitions = [
  {
    titleToken: PortalTextToken.TrackingParentActionAcknowledgementRecorded,
    statusToken: PortalTextToken.TrackingParentActionAcknowledgementRecordedStatus,
    primaryActionToken: PortalTextToken.TrackingParentActionAcknowledgeSafe,
    policyDecisionToken: PortalTextToken.TrackingParentActionSafeDecision,
    evidenceToken: PortalTextToken.TrackingParentActionSafeEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionSafeSurface,
    manualProofToken: PortalTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionExceptionActive,
    statusToken: PortalTextToken.TrackingParentActionExceptionActiveStatus,
    primaryActionToken: PortalTextToken.TrackingParentActionMarkExpected,
    policyDecisionToken: PortalTextToken.TrackingParentActionExpectedDecision,
    evidenceToken: PortalTextToken.TrackingParentActionExpectedEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionExpectedSurface,
    manualProofToken: PortalTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionFalseAlarmRecorded,
    statusToken: PortalTextToken.TrackingParentActionFalseAlarmRecordedStatus,
    primaryActionToken: PortalTextToken.TrackingParentActionMarkFalseAlarm,
    policyDecisionToken: PortalTextToken.TrackingParentActionFalseAlarmDecision,
    evidenceToken: PortalTextToken.TrackingParentActionFalseAlarmEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionFalseAlarmSurface,
    manualProofToken: PortalTextToken.TrackingParentActionServiceMutationManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionChildCheckInReady,
    statusToken: PortalTextToken.TrackingParentActionChildCheckInRequestReady,
    primaryActionToken: PortalTextToken.TrackingParentActionRequestChildCheckIn,
    policyDecisionToken: PortalTextToken.TrackingParentActionChildCheckInDecision,
    evidenceToken: PortalTextToken.TrackingParentActionChildCheckInEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionChildCheckInSurface,
    manualProofToken: PortalTextToken.TrackingParentActionChildRuntimeManualProof,
  },
  {
    titleToken: PortalTextToken.TrackingParentActionCriticalReviewReady,
    statusToken: PortalTextToken.TrackingParentActionEscalationReviewReady,
    primaryActionToken: PortalTextToken.TrackingParentActionEscalateManualReview,
    policyDecisionToken: PortalTextToken.TrackingParentActionCriticalReviewDecision,
    evidenceToken: PortalTextToken.TrackingParentActionCriticalReviewEvidence,
    uiSurfaceToken: PortalTextToken.TrackingParentActionCriticalReviewSurface,
    manualProofToken: PortalTextToken.TrackingParentActionEscalationManualProof,
  },
] as const satisfies readonly TrackingParentActionReadinessHostedDefinition[];

export function trackingParentActionReadinessHostedUiProof(): TrackingParentActionReadinessHostedUiProof {
  const expectedPlaceRows = ExpectedPlaceDefinitions.map((definition) => hostedRow(definition));
  const acknowledgementActionRows = AcknowledgementDefinitions.map((definition) => hostedRow(definition));
  const rows = [...expectedPlaceRows, ...acknowledgementActionRows];
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingParentActionReadinessHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingParentActionReadinessHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    expectedPlaceProofArtifact: TrackingStatusProofArtifacts.ExpectedPlaceAlertPolicy,
    acknowledgementProofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgementActionReadiness,
    boundary: PortalText.Resolve(PortalTextToken.TrackingParentActionReadinessHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
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
    title: PortalText.Resolve(definition.titleToken),
    status: detailFromText(definition.statusToken),
    primaryActionRef: detailFromText(definition.primaryActionToken),
    policyDecisionRef: detailFromText(definition.policyDecisionToken),
    evidenceRefs: detailFromText(definition.evidenceToken),
    uiSurfaceRef: detailFromText(definition.uiSurfaceToken),
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
