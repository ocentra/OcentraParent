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

export type TrackingReportExportHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly exportedRows: PortalDetailValue;
  readonly redactedEvidenceRefs: PortalDetailValue;
  readonly custody: PortalDetailValue;
  readonly evidence: PortalDisplayText;
};

export type TrackingReportExportHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly rawLocationPayloadClaimedRows: PortalDetailValue;
  readonly serviceMutationClaimedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly rows: readonly TrackingReportExportHostedUiRow[];
};

type TrackingReportExportHostedUiDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly exportedRows: number;
  readonly redactedEvidenceRefs: number;
  readonly custodyToken: PortalTextTokenValue;
};

const TrackingReportExportHostedUiDefinitions = [
  {
    titleToken: PortalTextToken.TrackingReportExportRedactedReport,
    evidenceToken: PortalTextToken.TrackingReportExportRedactedReportEvidence,
    exportedRows: 6,
    redactedEvidenceRefs: 6,
    custodyToken: PortalTextToken.TrackingReportExportRedactedCustody,
  },
  {
    titleToken: PortalTextToken.TrackingReportExportRetentionAudit,
    evidenceToken: PortalTextToken.TrackingReportExportRetentionAuditEvidence,
    exportedRows: 5,
    redactedEvidenceRefs: 5,
    custodyToken: PortalTextToken.TrackingReportExportLocalCustody,
  },
  {
    titleToken: PortalTextToken.TrackingReportExportFamilySummary,
    evidenceToken: PortalTextToken.TrackingReportExportFamilySummaryEvidence,
    exportedRows: 3,
    redactedEvidenceRefs: 3,
    custodyToken: PortalTextToken.TrackingReportExportRedactedCustody,
  },
  {
    titleToken: PortalTextToken.TrackingReportExportPolicyDrillIn,
    evidenceToken: PortalTextToken.TrackingReportExportPolicyDrillInEvidence,
    exportedRows: 2,
    redactedEvidenceRefs: 2,
    custodyToken: PortalTextToken.TrackingReportExportRedactedCustody,
  },
] as const satisfies readonly TrackingReportExportHostedUiDefinition[];

export function trackingReportExportHostedUiProof(): TrackingReportExportHostedUiProof {
  const rows = TrackingReportExportHostedUiDefinitions.map((definition) => reportExportHostedUiRow(definition));
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingReportExportHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingReportExportHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.ReportExportReadModel,
    boundary: PortalText.Resolve(PortalTextToken.TrackingReportExportHostedBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    rawLocationPayloadClaimedRows: detailFromValue(0),
    serviceMutationClaimedRows: detailFromValue(0),
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

function reportExportHostedUiRow(definition: TrackingReportExportHostedUiDefinition): TrackingReportExportHostedUiRow {
  return {
    title: PortalText.Resolve(definition.titleToken),
    status: PortalText.Resolve(PortalTextToken.TrackingReportExportReadModelReady),
    exportedRows: detailFromValue(definition.exportedRows),
    redactedEvidenceRefs: detailFromValue(definition.redactedEvidenceRefs),
    custody: detailFromValue(PortalText.Resolve(definition.custodyToken)),
    evidence: PortalText.Resolve(definition.evidenceToken),
  };
}

function detailFromValue(value: PortalDisplayText | number): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}
