import {
  PortalText,
  PortalTextToken,
  TrackingStatusProofArtifacts,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';

type PortalTextTokenValue = (typeof PortalTextToken)[keyof typeof PortalTextToken];

export type TrackingReportPolicyConsumerHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly storedJournalRef: PortalDetailValue;
  readonly storedReadModelRef: PortalDetailValue;
  readonly evidence: PortalDisplayText;
  readonly reportSurface: PortalDisplayText;
};

export type TrackingReportPolicyConsumerHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly aiExecutionClaimedRows: PortalDetailValue;
  readonly policyMutationClaimedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingReportPolicyConsumerHostedUiRow[];
};

type TrackingReportPolicyConsumerHostedUiDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly storedJournalToken: PortalTextTokenValue;
  readonly storedReadModelToken: PortalTextTokenValue;
  readonly reportSurfaceToken: PortalTextTokenValue;
};

const TrackingReportPolicyConsumerHostedUiDefinitions = [
  {
    titleToken: PortalTextToken.TrackingReportPolicyConsumerParentReport,
    evidenceToken: PortalTextToken.TrackingReportPolicyConsumerReportEvidence,
    storedJournalToken: PortalTextToken.TrackingReportPolicyConsumerReportJournal,
    storedReadModelToken: PortalTextToken.TrackingReportPolicyConsumerReportReadModel,
    reportSurfaceToken: PortalTextToken.TrackingReportPolicyConsumerReportSurface,
  },
  {
    titleToken: PortalTextToken.TrackingReportPolicyConsumerPolicyDrillIn,
    evidenceToken: PortalTextToken.TrackingReportPolicyConsumerPolicyEvidence,
    storedJournalToken: PortalTextToken.TrackingReportPolicyConsumerPolicyJournal,
    storedReadModelToken: PortalTextToken.TrackingReportPolicyConsumerPolicyReadModel,
    reportSurfaceToken: PortalTextToken.TrackingReportPolicyConsumerPolicySurface,
  },
  {
    titleToken: PortalTextToken.TrackingReportPolicyConsumerRetentionAudit,
    evidenceToken: PortalTextToken.TrackingReportPolicyConsumerRetentionEvidence,
    storedJournalToken: PortalTextToken.TrackingReportPolicyConsumerRetentionJournal,
    storedReadModelToken: PortalTextToken.TrackingReportPolicyConsumerRetentionReadModel,
    reportSurfaceToken: PortalTextToken.TrackingReportPolicyConsumerRetentionSurface,
  },
] as const satisfies readonly TrackingReportPolicyConsumerHostedUiDefinition[];

export function trackingReportPolicyConsumerHostedUiProof(): TrackingReportPolicyConsumerHostedUiProof {
  const rows = TrackingReportPolicyConsumerHostedUiDefinitions.map((definition) =>
    reportPolicyConsumerHostedUiRow(definition)
  );
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingReportPolicyConsumerHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingReportPolicyConsumerHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.ReportPolicyConsumer,
    boundary: PortalText.Resolve(PortalTextToken.TrackingReportPolicyConsumerHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    aiExecutionClaimedRows: detailFromValue(0),
    policyMutationClaimedRows: detailFromValue(0),
    platformRuntimeClaimedRows: detailFromValue(0),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    notificationReceiptClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    rows,
  };
}

function reportPolicyConsumerHostedUiRow(
  definition: TrackingReportPolicyConsumerHostedUiDefinition
): TrackingReportPolicyConsumerHostedUiRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    status: PortalText.Resolve(PortalTextToken.TrackingReportPolicyConsumerReady),
    storedJournalRef: detailFromValue(PortalText.Resolve(definition.storedJournalToken)),
    storedReadModelRef: detailFromValue(PortalText.Resolve(definition.storedReadModelToken)),
    evidence: PortalText.Resolve(definition.evidenceToken),
    reportSurface: PortalText.Resolve(definition.reportSurfaceToken),
  };
}

function detailFromValue(value: PortalDisplayText | number): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}
