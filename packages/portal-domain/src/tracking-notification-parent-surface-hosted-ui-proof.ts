import { PortalDetails } from './details';
import {
  RequiredTrackingNotificationParentSurfaceHistoryNonClaims,
  TrackingNotificationParentSurfaceHistoryReadModelSchema,
  TrackingNotificationParentSurfaceHistoryStatus,
  type TrackingNotificationParentSurfaceHistoryReadModel,
  type TrackingNotificationParentSurfaceHistoryRow,
} from '@ocentra-parent/schema-domain/tracking-notification-parent-surface-history-proof';
import type { DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { TrackingStatusProofArtifacts, type TrackingStatusProofArtifact } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

const TrackingNotificationParentSurfaceHostedUiValues = {
  NotReported: 'not reported',
  ReferenceSeparator: ' | ',
  SchemaVersion: 'v0.6',
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

const DefaultTrackingNotificationParentSurfaceHistoryReadModel =
  TrackingNotificationParentSurfaceHistoryReadModelSchema.parse({
    schemaVersion: TrackingNotificationParentSurfaceHostedUiValues.SchemaVersion,
    proofId: 'tracking-notification-parent-surface-history-proof',
    generatedAt: '2026-06-06T16:16:00.000Z',
    family: {
      familyId: 'family-tracking-notification-history',
    },
    sourceProviderNotificationProofRef: 'tracking-provider-notification-proof-for-parent-surface-history',
    sourceReceiptBoundaryProofRef: 'tracking-notification-receipt-boundary-proof-for-parent-surface-history',
    sourcePreferencePreflightProofRef: 'tracking-notification-preference-preflight-proof-for-parent-surface-history',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'tracking-notification-receipt-boundary-proof',
      'tracking-notification-preference-preflight-proof',
      'notifications-expectations',
      'location-geofence-device-status',
    ],
    rows: [
      {
        historyRowId: 'tracking-notification-history-tracking-alert-home-arrival',
        sourceAlertId: 'tracking-alert-home-arrival',
        sourceProviderNotificationRowId: 'tracking-provider-notification-tracking-alert-home-arrival',
        sourceReceiptBoundaryRowId: 'tracking-notification-receipt-tracking-alert-home-arrival',
        sourcePreferencePreflightRowId: 'tracking-notification-preference-preflight-tracking-alert-home-arrival',
        status: TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady,
        sourcePolicyDecisionId: 'tracking-decision-home-arrival',
        evidenceRefs: ['location-evidence-geofence-entry'],
        notificationStatusRefs: ['tracking-notification-intent-home-arrival'],
        reasonCodeRefs: ['home-arrival-notification'],
        providerStatusEntryRef: 'tracking-provider-status-entry-home-arrival',
        providerAttemptRef: 'tracking-provider-attempt-home-arrival',
        auditRefs: ['tracking-provider-notification-audit-tracking-alert-home-arrival'],
        providerPreferenceRefs: ['tracking-parent-provider-preference-home-arrival'],
        parentPreferenceRequirementRefs: ['parent-notification-preference-required-home-arrival'],
        quietHoursRequirementRefs: ['tracking-quiet-hours-policy-required-tracking-alert-home-arrival'],
        receiptRequirementRefs: ['receipt-ingestion-required-home-arrival'],
        manualProofRequirements: ['provider-delivery-runtime-required', 'receipt-webhook-runtime-required'],
        drillInRefs: ['tracking-notification-history-drill-in-tracking-alert-home-arrival'],
        redactedParentSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-home-arrival',
        renderedParentNotificationUiClaimed: false,
        parentPreferenceMutationRuntimeClaimed: false,
        providerDeliveryClaimed: false,
        receiptIngestionRuntimeClaimed: false,
        childDeviceDeliveryClaimed: false,
        mobilePhysicalDeviceProofClaimed: false,
        authorityProofClaimed: false,
      },
      {
        historyRowId: 'tracking-notification-history-tracking-alert-left-expected-place',
        sourceAlertId: 'tracking-alert-left-expected-place',
        sourceProviderNotificationRowId: 'tracking-provider-notification-tracking-alert-left-expected-place',
        sourceReceiptBoundaryRowId: 'tracking-notification-receipt-tracking-alert-left-expected-place',
        sourcePreferencePreflightRowId: 'tracking-notification-preference-preflight-tracking-alert-left-expected-place',
        status: TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired,
        sourcePolicyDecisionId: 'tracking-decision-left-expected-place',
        evidenceRefs: ['location-evidence-geofence-entry'],
        notificationStatusRefs: ['tracking-notification-intent-left-school'],
        reasonCodeRefs: ['left-expected-place'],
        providerStatusEntryRef: 'tracking-provider-status-entry-left-school',
        providerAttemptRef: 'tracking-provider-attempt-left-school',
        auditRefs: ['tracking-provider-notification-audit-tracking-alert-left-expected-place'],
        providerPreferenceRefs: ['tracking-parent-provider-preference-left-school'],
        parentPreferenceRequirementRefs: [
          'tracking-parent-notification-preference-required-tracking-alert-left-school',
        ],
        quietHoursRequirementRefs: ['quiet-hours-requirement-left-school'],
        receiptRequirementRefs: ['manual-receipt-required-left-school'],
        manualProofRequirements: ['manual-provider-review-required', 'quiet-hours-runtime-required'],
        drillInRefs: ['tracking-notification-history-drill-in-tracking-alert-left-expected-place'],
        redactedParentSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-left-expected-place',
        renderedParentNotificationUiClaimed: false,
        parentPreferenceMutationRuntimeClaimed: false,
        providerDeliveryClaimed: false,
        receiptIngestionRuntimeClaimed: false,
        childDeviceDeliveryClaimed: false,
        mobilePhysicalDeviceProofClaimed: false,
        authorityProofClaimed: false,
      },
      {
        historyRowId: 'tracking-notification-history-tracking-alert-provider-unavailable',
        sourceAlertId: 'tracking-alert-provider-unavailable',
        sourceProviderNotificationRowId: 'tracking-provider-notification-tracking-alert-provider-unavailable',
        sourceReceiptBoundaryRowId: 'tracking-notification-receipt-tracking-alert-provider-unavailable',
        sourcePreferencePreflightRowId:
          'tracking-notification-preference-preflight-tracking-alert-provider-unavailable',
        status: TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable,
        sourcePolicyDecisionId: 'tracking-decision-provider-unavailable',
        evidenceRefs: ['location-evidence-geofence-entry'],
        notificationStatusRefs: ['tracking-notification-intent-provider-unavailable'],
        reasonCodeRefs: ['provider-unavailable'],
        providerStatusEntryRef: 'tracking-provider-status-entry-provider-unavailable',
        providerAttemptRef: 'tracking-provider-attempt-unavailable',
        auditRefs: ['tracking-provider-notification-audit-tracking-alert-provider-unavailable'],
        providerPreferenceRefs: ['tracking-parent-provider-preference-provider-unavailable'],
        parentPreferenceRequirementRefs: ['source-unavailable-preference-required'],
        quietHoursRequirementRefs: [],
        receiptRequirementRefs: ['provider-receipt-unavailable'],
        manualProofRequirements: ['provider-adapter-unavailable', 'manual-parent-history-review-required'],
        drillInRefs: ['tracking-notification-history-drill-in-tracking-alert-provider-unavailable'],
        redactedParentSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-provider-unavailable',
        renderedParentNotificationUiClaimed: false,
        parentPreferenceMutationRuntimeClaimed: false,
        providerDeliveryClaimed: false,
        receiptIngestionRuntimeClaimed: false,
        childDeviceDeliveryClaimed: false,
        mobilePhysicalDeviceProofClaimed: false,
        authorityProofClaimed: false,
      },
    ],
    historyIntentReadyCount: 1,
    manualActionRequiredCount: 1,
    providerUnavailableCount: 1,
    proofNonClaims: RequiredTrackingNotificationParentSurfaceHistoryNonClaims,
    renderedParentNotificationUiClaimed: false,
    parentPreferenceMutationRuntimeClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableHistoryStorageClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });

export function trackingNotificationParentSurfaceHostedUiProof(): TrackingNotificationParentSurfaceHostedUiProof {
  return trackingNotificationParentSurfaceHostedUiProofFromReadModel(
    DefaultTrackingNotificationParentSurfaceHistoryReadModel
  );
}

export function trackingNotificationParentSurfaceHostedUiProofFromReadModel(
  readModelInput: unknown
): TrackingNotificationParentSurfaceHostedUiProof {
  const parsed = TrackingNotificationParentSurfaceHistoryReadModelSchema.safeParse(readModelInput);
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
  readModel: TrackingNotificationParentSurfaceHistoryReadModel
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
  row: TrackingNotificationParentSurfaceHistoryRow
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

function titleForHistoryStatus(status: TrackingNotificationParentSurfaceHistoryRow['status']): PortalDisplayText {
  switch (status) {
    case TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntent);
    case TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceManualAction);
    case TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable:
      return resolvePortalDevText(PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailable);
  }
}

function preferenceRequirementRefsFor(row: TrackingNotificationParentSurfaceHistoryRow): readonly string[] {
  if (
    row.status === TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired &&
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
