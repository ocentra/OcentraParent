import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;
type PortalDevTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

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
  readonly titleToken: PortalDevTextTokenValue;
  readonly evidenceToken: PortalDevTextTokenValue;
  readonly exportedRows: number;
  readonly redactedEvidenceRefs: number;
  readonly custodyToken: PortalDevTextTokenValue;
};

const TrackingReportExportHostedUiDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingReportExportRedactedReport,
    evidenceToken: PortalDevTextToken.TrackingReportExportRedactedReportEvidence,
    exportedRows: 6,
    redactedEvidenceRefs: 6,
    custodyToken: PortalDevTextToken.TrackingReportExportRedactedCustody,
  },
  {
    titleToken: PortalDevTextToken.TrackingReportExportRetentionAudit,
    evidenceToken: PortalDevTextToken.TrackingReportExportRetentionAuditEvidence,
    exportedRows: 5,
    redactedEvidenceRefs: 5,
    custodyToken: PortalDevTextToken.TrackingReportExportLocalCustody,
  },
  {
    titleToken: PortalDevTextToken.TrackingReportExportFamilySummary,
    evidenceToken: PortalDevTextToken.TrackingReportExportFamilySummaryEvidence,
    exportedRows: 3,
    redactedEvidenceRefs: 3,
    custodyToken: PortalDevTextToken.TrackingReportExportRedactedCustody,
  },
  {
    titleToken: PortalDevTextToken.TrackingReportExportPolicyDrillIn,
    evidenceToken: PortalDevTextToken.TrackingReportExportPolicyDrillInEvidence,
    exportedRows: 2,
    redactedEvidenceRefs: 2,
    custodyToken: PortalDevTextToken.TrackingReportExportRedactedCustody,
  },
] as const satisfies readonly TrackingReportExportHostedUiDefinition[];

export function trackingReportExportHostedUiProof(): TrackingReportExportHostedUiProof {
  const rows = TrackingReportExportHostedUiDefinitions.map((definition) => reportExportHostedUiRow(definition));
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingReportExportHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingReportExportHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.ReportExportReadModel,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingReportExportHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    title: resolvePortalDevText(definition.titleToken),
    status: resolvePortalDevText(PortalDevTextToken.TrackingReportExportReadModelReady),
    exportedRows: detailFromValue(definition.exportedRows),
    redactedEvidenceRefs: detailFromValue(definition.redactedEvidenceRefs),
    custody: detailFromValue(resolvePortalDevText(definition.custodyToken)),
    evidence: resolvePortalDevText(definition.evidenceToken),
  };
}

function detailFromValue(value: PortalDisplayText | number): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}
