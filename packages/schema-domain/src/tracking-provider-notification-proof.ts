import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  type TrackingAlertIntent,
  TrackingLocationPolicyReadModelSchema,
  type TrackingLocationPolicyReadModel,
} from './tracking-location-policy';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';

export const RequiredTrackingProviderNotificationProofNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-retry-worker-runtime',
  'no-quiet-hours-timer-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const TrackingProviderNotificationProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingProviderNotificationProofNonClaims)
);

export const TrackingProviderNotificationProofIdSchema = brandedNonEmptyStringSchema('TrackingProviderNotificationProofId');
export const TrackingProviderNotificationProofReferenceSchema = brandedNonEmptyStringSchema('TrackingProviderNotificationProofReference');

export const TrackingProviderNotificationStatusKindSchema = withParser(
  Schema.Literal('provider-adapter-required', 'manual-required', 'unavailable')
);

const TrackingProviderNotificationProofRowBaseSchema = Schema.Struct({
  rowId: TrackingProviderNotificationProofReferenceSchema,
  sourceAlertId: TrackingProviderNotificationProofReferenceSchema,
  sourcePolicyDecisionId: TrackingProviderNotificationProofReferenceSchema,
  severity: TrackingProviderNotificationProofReferenceSchema,
  sensitiveDetailMode: TrackingProviderNotificationProofReferenceSchema,
  evidenceRefs: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
  notificationStatusRefs: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
  providerStatusKind: TrackingProviderNotificationStatusKindSchema,
  providerStatusBoundaryEntry: V08NotificationProviderStatusBoundaryEntrySchema,
  manualProofRequirements: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
});

export const TrackingProviderNotificationProofRowSchema = withParser(
  TrackingProviderNotificationProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingProviderNotificationProofRowIsHonest(row) ||
        'Expected tracking provider notification rows to preserve alert evidence/policy refs and map to manual-required or unavailable provider status without claiming delivery'
    )
  )
);

const TrackingProviderNotificationProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingProviderNotificationProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceTrackingReadModelRef: TrackingProviderNotificationProofReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
  providerStatusBoundaryReadModelRef: TrackingProviderNotificationProofReferenceSchema,
  providerStatusBoundaryCoverageRefs: Schema.Array(TrackingProviderNotificationProofReferenceSchema),
  rows: Schema.Array(TrackingProviderNotificationProofRowSchema),
  providerAdapterRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingProviderNotificationProofNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const TrackingProviderNotificationProofReadModelSchema = withParser(
  TrackingProviderNotificationProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingProviderNotificationProofReadModelIsHonest(readModel) ||
        'Expected tracking provider notification proof counts and non-claims to match provider-boundary rows'
    )
  )
);

export type TrackingProviderNotificationStatusKind = Infer<typeof TrackingProviderNotificationStatusKindSchema>;
export type TrackingProviderNotificationProofRow = Infer<typeof TrackingProviderNotificationProofRowSchema>;
export type TrackingProviderNotificationProofReadModel = Infer<typeof TrackingProviderNotificationProofReadModelSchema>;

export type TrackingProviderNotificationProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly sourceTrackingReadModelRef: string;
  readonly sourceContractRefs: readonly string[];
};

type TrackingProviderNotificationProofRowInput = Infer<typeof TrackingProviderNotificationProofRowBaseSchema>;
type TrackingProviderNotificationProofReadModelInput = Infer<
  typeof TrackingProviderNotificationProofReadModelBaseSchema
>;
type ProviderStatusBoundaryEntry = TrackingProviderNotificationProofRowInput['providerStatusBoundaryEntry'];

export function buildTrackingProviderNotificationProofReadModel(
  options: TrackingProviderNotificationProofOptions,
  sourceReadModel: TrackingLocationPolicyReadModel
): TrackingProviderNotificationProofReadModel {
  const parsedSource = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.alerts.map((alert) => trackingProviderNotificationRowForAlert(options, alert));

  return TrackingProviderNotificationProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    sourceTrackingReadModelRef: options.sourceTrackingReadModelRef,
    sourceContractRefs: options.sourceContractRefs,
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerStatusBoundaryCoverageRefs: V08NotificationProviderStatusBoundaryReadModel.entries.map(
      (entry) => entry.statusEntryId
    ),
    rows,
    providerAdapterRequiredCount: countStatusKind(rows, 'provider-adapter-required'),
    manualRequiredCount: countStatusKind(rows, 'manual-required'),
    unavailableCount: countStatusKind(rows, 'unavailable'),
    proofNonClaims: RequiredTrackingProviderNotificationProofNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function trackingProviderNotificationRowForAlert(
  options: TrackingProviderNotificationProofOptions,
  alert: TrackingAlertIntent
): TrackingProviderNotificationProofRow {
  const providerStatusKind = providerStatusKindForAlert(alert);
  const manualProofRequirements = manualProofRequirementsFor(providerStatusKind, alert);

  return TrackingProviderNotificationProofRowSchema.parse({
    rowId: `tracking-provider-notification-${alert.alertId}`,
    sourceAlertId: alert.alertId,
    sourcePolicyDecisionId: alert.policyDecisionId,
    severity: alert.severity,
    sensitiveDetailMode: alert.sensitiveDetailMode,
    evidenceRefs: alert.evidenceReferences.map((evidence) => evidence.evidenceReferenceId),
    notificationStatusRefs: alert.notificationStatusRefs,
    reasonCodeRefs: alert.reasonCodes,
    providerStatusKind,
    providerStatusBoundaryEntry: providerStatusBoundaryEntryForAlert(
      options,
      alert,
      providerStatusKind,
      manualProofRequirements
    ),
    manualProofRequirements,
  });
}

function providerStatusKindForAlert(alert: TrackingAlertIntent): TrackingProviderNotificationStatusKind {
  if (alert.notificationStatusRefs.length === 0) {
    return 'unavailable';
  }
  if (alert.severity === 'critical' || alert.severity === 'urgent') {
    return 'manual-required';
  }
  return 'provider-adapter-required';
}

function manualProofRequirementsFor(
  statusKind: TrackingProviderNotificationStatusKind,
  alert: TrackingAlertIntent
): readonly string[] {
  if (statusKind === 'unavailable') {
    return [`tracking-provider-unavailable-${alert.alertId}`];
  }
  if (statusKind === 'manual-required') {
    return [
      `tracking-provider-parent-preference-required-${alert.alertId}`,
      `tracking-provider-critical-escalation-review-${alert.alertId}`,
      `tracking-provider-smoke-proof-required-${alert.alertId}`,
    ];
  }
  return [
    `tracking-provider-adapter-required-${alert.alertId}`,
    `tracking-provider-credentials-required-${alert.alertId}`,
    `tracking-provider-smoke-proof-required-${alert.alertId}`,
  ];
}

function providerStatusBoundaryEntryForAlert(
  options: TrackingProviderNotificationProofOptions,
  alert: TrackingAlertIntent,
  statusKind: TrackingProviderNotificationStatusKind,
  manualProofRequirements: readonly string[]
): ProviderStatusBoundaryEntry {
  const unavailable = statusKind === 'unavailable';
  const manualRequired = statusKind === 'manual-required';

  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    statusEntryId: `tracking-provider-status-${alert.alertId}`,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
    quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
    escalationReadiness: unavailable || manualRequired ? 'manual-required' : 'manual-required',
    deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
    notificationIntentRef: alert.alertId,
    notificationStatusRef: alert.notificationStatusRefs[0] ?? `tracking-provider-status-unavailable-${alert.alertId}`,
    providerAttemptRef: `tracking-provider-attempt-not-started-${alert.alertId}`,
    auditRefs: [`tracking-provider-notification-audit-${alert.alertId}`],
    preferenceRefs: [`tracking-provider-notification-preference-${alert.alertId}`],
    readinessRefs: manualProofRequirements,
    providerReceiptRefs: [],
    manualProofRequirements,
    minimalPayloadBoundary:
      alert.sensitiveDetailMode === 'authenticated-drill-in-only'
        ? 'Tracking notification provider payload must use authenticated drill-in only and keep location detail out of third-party preview.'
        : 'Tracking notification provider payload is limited to minimal parent alert copy with evidence detail behind authenticated parent surfaces.',
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function trackingProviderNotificationProofRowIsHonest(row: TrackingProviderNotificationProofRowInput): boolean {
  const entry = row.providerStatusBoundaryEntry;

  return (
    row.evidenceRefs.length > 0 &&
    row.reasonCodeRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    providerStatusMatchesKind(row.providerStatusKind, entry.providerStatus) &&
    providerBoundaryKeepsDeliveryUnclaimed(entry)
  );
}

function providerStatusMatchesKind(
  statusKind: TrackingProviderNotificationStatusKind,
  providerStatus: V08NotificationProviderStatus
): boolean {
  return statusKind === 'unavailable' ? providerStatus === 'unavailable' : providerStatus === 'manual-required';
}

function providerBoundaryKeepsDeliveryUnclaimed(entry: ProviderStatusBoundaryEntry): boolean {
  return (
    entry.providerReceiptRefs.length === 0 &&
    [
      entry.providerDeliveryImplemented,
      entry.providerDeliveryObserved,
      entry.deliveredNotificationClaimed,
      entry.sensitiveProviderPayloadClaimed,
      entry.providerStoresChildEvidenceClaimed,
    ].every((claim) => claim === false)
  );
}

function trackingProviderNotificationProofReadModelIsHonest(
  readModel: TrackingProviderNotificationProofReadModelInput
): boolean {
  return (
    readModel.providerAdapterRequiredCount === countStatusKind(readModel.rows, 'provider-adapter-required') &&
    readModel.manualRequiredCount === countStatusKind(readModel.rows, 'manual-required') &&
    readModel.unavailableCount === countStatusKind(readModel.rows, 'unavailable') &&
    RequiredTrackingProviderNotificationProofNonClaims.every((claim) => readModel.proofNonClaims.includes(claim)) &&
    readModel.providerStatusBoundaryCoverageRefs.length ===
      V08NotificationProviderStatusBoundaryReadModel.entries.length
  );
}

const countStatusKind = (
  rows: ReadonlyArray<{ readonly providerStatusKind: TrackingProviderNotificationStatusKind }>,
  statusKind: TrackingProviderNotificationStatusKind
): number => rows.filter((row) => row.providerStatusKind === statusKind).length;

export const decodeTrackingProviderNotificationProofReadModel = (input: unknown) =>
  TrackingProviderNotificationProofReadModelSchema.parse(input);

