import { PortalDetails } from './details';
import { GeneratedPortalTrackingContracts } from './generated-portal-contracts';
import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;
type GeneratedTrackingNotificationParentSurfaceHistoryReadModel = NonNullable<
  ReturnType<typeof TrackingNotificationParentSurfaceHistoryContracts.decode>
>;
type GeneratedTrackingNotificationParentSurfaceHistoryRow =
  GeneratedTrackingNotificationParentSurfaceHistoryReadModel['rows'][number];

const TrackingNotificationParentSurfaceHistoryContracts =
  GeneratedPortalTrackingContracts.NotificationParentSurfaceHistory;

const TrackingNotificationParentSurfaceHostedUiValues = {
  NotReported: 'not reported',
  ReferenceSeparator: ' | ',
} as const;

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

export function trackingNotificationParentSurfaceHostedUiProof(): TrackingNotificationParentSurfaceHostedUiProof {
  return trackingNotificationParentSurfaceHostedUiProofFromReadModel(
    TrackingNotificationParentSurfaceHistoryContracts.DefaultReadModel
  );
}

export function trackingNotificationParentSurfaceHostedUiProofFromReadModel(
  readModelInput: unknown
): TrackingNotificationParentSurfaceHostedUiProof {
  const parsed = TrackingNotificationParentSurfaceHistoryContracts.ReadModelSchema.safeParse(readModelInput);
  if (!parsed.success) {
    return unavailableHostedUiProof();
  }

  const readModel = parsed.data;
  return hostedUiProofFromReadModel(readModel);
}

export const TrackingNotificationParentSurfaceHostedUiDetails = {
  PreferenceRequirement: PortalDetails.ParentRuleContextReferences,
  ProviderAttempt: PortalDetails.ProviderSource,
  ReceiptRequirement: PortalDetails.AdapterDispatch,
  RedactedSummary: PortalDetails.PrivacyMode,
} as const;

function hostedUiProofFromReadModel(
  readModel: GeneratedTrackingNotificationParentSurfaceHistoryReadModel
): TrackingNotificationParentSurfaceHostedUiProof {
  const rows = readModel.rows.map((row) => hostedRowFromHistoryRow(row));
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
    parentPreferenceMutationRows: detailFromClaim(readModel.parentPreferenceMutationRuntimeClaimed),
    providerDeliveryClaimedRows: detailFromClaim(readModel.providerDeliveryRuntimeClaimed),
    receiptIngestionClaimedRows: detailFromClaim(readModel.providerReceiptIngestionRuntimeClaimed),
    childDeviceDeliveryClaimedRows: detailFromClaim(readModel.childDeviceDeliveryClaimed),
    physicalDeviceClaimedRows: detailFromClaim(readModel.mobilePhysicalDeviceProofClaimed),
    authorityClaimedRows: detailFromClaim(readModel.authorityProofClaimed),
    productionStorageClaimedRows: detailFromClaim(readModel.productionDurableHistoryStorageClaimed),
    adapterDispatchClaimedRows: detailFromClaim(readModel.adapterDispatchClaimed),
    productClaimReadyRows: zero(),
    rows,
  };
}

function unavailableHostedUiProof(): TrackingNotificationParentSurfaceHostedUiProof {
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: zero(),
    proofArtifact: TrackingStatusProofArtifacts.NotificationParentSurfaceHistory,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
    renderedParentNotificationUiRows: zero(),
    parentPreferenceMutationRows: zero(),
    providerDeliveryClaimedRows: zero(),
    receiptIngestionClaimedRows: zero(),
    childDeviceDeliveryClaimedRows: zero(),
    physicalDeviceClaimedRows: zero(),
    authorityClaimedRows: zero(),
    productionStorageClaimedRows: zero(),
    adapterDispatchClaimedRows: zero(),
    productClaimReadyRows: zero(),
    rows: [],
  };
}

function hostedRowFromHistoryRow(
  row: GeneratedTrackingNotificationParentSurfaceHistoryRow
): TrackingNotificationParentSurfaceHostedUiRow {
  return {
    title: titleForHistoryStatus(row.status),
    status: detailFromValue(row.status),
    policyDecisionRef: detailFromValue(row.sourcePolicyDecisionId),
    evidenceRefs: detailFromReferences(row.evidenceRefs),
    providerAttemptRef: detailFromValue(row.providerAttemptRef),
    receiptRequirementRefs: detailFromReferences(row.receiptRequirementRefs),
    preferenceRequirementRefs: detailFromReferences(preferenceRequirementRefsFor(row)),
    manualProofRequirements: detailFromReferences(row.manualProofRequirements),
    redactedSummaryRef: detailFromValue(row.redactedParentSummaryRef),
  };
}

function titleForHistoryStatus(
  status: GeneratedTrackingNotificationParentSurfaceHistoryRow['status']
): PortalDisplayText {
  switch (status) {
    case TrackingNotificationParentSurfaceHistoryContracts.Status.HistoryIntentReady:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntent);
    case TrackingNotificationParentSurfaceHistoryContracts.Status.ManualActionRequired:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceManualAction);
    case TrackingNotificationParentSurfaceHistoryContracts.Status.ProviderUnavailable:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailable);
  }
}

function preferenceRequirementRefsFor(row: GeneratedTrackingNotificationParentSurfaceHistoryRow): readonly string[] {
  if (
    row.status === TrackingNotificationParentSurfaceHistoryContracts.Status.ManualActionRequired &&
    row.quietHoursRequirementRefs.length > 0
  ) {
    return row.quietHoursRequirementRefs;
  }
  return row.parentPreferenceRequirementRefs.length > 0
    ? row.parentPreferenceRequirementRefs
    : row.quietHoursRequirementRefs;
}

function detailFromClaim(claim: boolean): PortalDetailValue {
  return detailFromValue(Number(claim));
}

function detailFromReferences(refs: readonly string[]): PortalDetailValue {
  return detailFromValue(
    refs.length > 0
      ? refs.join(TrackingNotificationParentSurfaceHostedUiValues.ReferenceSeparator)
      : TrackingNotificationParentSurfaceHostedUiValues.NotReported
  );
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function zero(): PortalDetailValue {
  return detailFromValue(0);
}
