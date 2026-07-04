import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;
type PortalDevTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

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
  readonly titleToken: PortalDevTextTokenValue;
  readonly evidenceToken: PortalDevTextTokenValue;
  readonly storedJournalToken: PortalDevTextTokenValue;
  readonly storedReadModelToken: PortalDevTextTokenValue;
  readonly reportSurfaceToken: PortalDevTextTokenValue;
};

const TrackingReportPolicyConsumerHostedUiDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingReportPolicyConsumerParentReport,
    evidenceToken: PortalDevTextToken.TrackingReportPolicyConsumerReportEvidence,
    storedJournalToken: PortalDevTextToken.TrackingReportPolicyConsumerReportJournal,
    storedReadModelToken: PortalDevTextToken.TrackingReportPolicyConsumerReportReadModel,
    reportSurfaceToken: PortalDevTextToken.TrackingReportPolicyConsumerReportSurface,
  },
  {
    titleToken: PortalDevTextToken.TrackingReportPolicyConsumerPolicyDrillIn,
    evidenceToken: PortalDevTextToken.TrackingReportPolicyConsumerPolicyEvidence,
    storedJournalToken: PortalDevTextToken.TrackingReportPolicyConsumerPolicyJournal,
    storedReadModelToken: PortalDevTextToken.TrackingReportPolicyConsumerPolicyReadModel,
    reportSurfaceToken: PortalDevTextToken.TrackingReportPolicyConsumerPolicySurface,
  },
  {
    titleToken: PortalDevTextToken.TrackingReportPolicyConsumerRetentionAudit,
    evidenceToken: PortalDevTextToken.TrackingReportPolicyConsumerRetentionEvidence,
    storedJournalToken: PortalDevTextToken.TrackingReportPolicyConsumerRetentionJournal,
    storedReadModelToken: PortalDevTextToken.TrackingReportPolicyConsumerRetentionReadModel,
    reportSurfaceToken: PortalDevTextToken.TrackingReportPolicyConsumerRetentionSurface,
  },
] as const satisfies readonly TrackingReportPolicyConsumerHostedUiDefinition[];

export function trackingReportPolicyConsumerHostedUiProof(): TrackingReportPolicyConsumerHostedUiProof {
  const rows = TrackingReportPolicyConsumerHostedUiDefinitions.map((definition) =>
    reportPolicyConsumerHostedUiRow(definition)
  );
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingReportPolicyConsumerHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingReportPolicyConsumerHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.ReportPolicyConsumer,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingReportPolicyConsumerHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
    title: resolvePortalDevText(definition.titleToken),
    status: resolvePortalDevText(PortalDevTextToken.TrackingReportPolicyConsumerReady),
    storedJournalRef: detailFromValue(resolvePortalDevText(definition.storedJournalToken)),
    storedReadModelRef: detailFromValue(resolvePortalDevText(definition.storedReadModelToken)),
    evidence: resolvePortalDevText(definition.evidenceToken),
    reportSurface: resolvePortalDevText(definition.reportSurfaceToken),
  };
}

function detailFromValue(value: PortalDisplayText | number): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}
