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
  TrackingProviderNotificationProofReadModelSchema,
  type TrackingProviderNotificationProofReadModel,
  type TrackingProviderNotificationProofRow,
  type TrackingProviderNotificationStatusKind,
} from './tracking-provider-notification-proof';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './notification-v3-provider-retry';

export const TrackingNotificationPreferencePreflightStatus = {
  ParentPreferenceRequired: 'parent-preference-required',
  SourceManualRequired: 'source-manual-required',
  SourceUnavailable: 'source-unavailable',
} as const;

export const RequiredTrackingNotificationPreferencePreflightNonClaims = [
  'no-parent-notification-preference-ui',
  'no-parent-notification-history-ui',
  'no-parent-frequency-control-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const TrackingNotificationPreferencePreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(TrackingNotificationPreferencePreflightStatus))
);
export const TrackingNotificationPreferencePreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationPreferencePreflightNonClaims)
);
export const TrackingNotificationPreferencePreflightIdSchema = brandedNonEmptyStringSchema('TrackingNotificationPreferencePreflightId');
export const TrackingNotificationPreferencePreflightReferenceSchema = brandedNonEmptyStringSchema('TrackingNotificationPreferencePreflightReference');

const TrackingNotificationPreferencePreflightRowBaseSchema = Schema.Struct({
  preferenceRowId: TrackingNotificationPreferencePreflightReferenceSchema,
  sourceProviderNotificationRowId: TrackingNotificationPreferencePreflightReferenceSchema,
  sourceAlertId: TrackingNotificationPreferencePreflightReferenceSchema,
  providerStatusKind: Schema.Literal('provider-adapter-required', 'manual-required', 'unavailable'),
  status: TrackingNotificationPreferencePreflightStatusSchema,
  sourcePolicyDecisionId: TrackingNotificationPreferencePreflightReferenceSchema,
  evidenceRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  notificationStatusRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  providerAttemptRef: Schema.Union(TrackingNotificationPreferencePreflightReferenceSchema, Schema.Null),
  providerPreferenceRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  parentPreferenceState: Schema.Union(V3NotificationParentPreferenceStateSchema, Schema.Null),
  quietHoursDecision: Schema.Union(V3NotificationQuietHoursDecisionSchema, Schema.Null),
  parentPreferenceRequirementRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
});

export const TrackingNotificationPreferencePreflightRowSchema = withParser(
  TrackingNotificationPreferencePreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingPreferencePreflightRowIsHonest(row) ||
        'Expected tracking notification preference rows to preserve source refs and block delivery until parent preferences and quiet-hours proof exist'
    )
  )
);

const TrackingNotificationPreferencePreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  preferencePreflightId: TrackingNotificationPreferencePreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderNotificationProofId: TrackingNotificationPreferencePreflightReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationPreferencePreflightReferenceSchema),
  rows: Schema.Array(TrackingNotificationPreferencePreflightRowSchema),
  parentPreferenceRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  sourceManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  sourceUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(TrackingNotificationPreferencePreflightNonClaimSchema),
  parentNotificationPreferenceUiClaimed: Schema.Literal(false),
  parentNotificationHistoryUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const TrackingNotificationPreferencePreflightReadModelSchema = withParser(
  TrackingNotificationPreferencePreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingPreferencePreflightReadModelIsHonest(readModel) ||
        'Expected tracking notification preference preflight counts and non-claims to match provider-derived rows'
    )
  )
);

export type TrackingNotificationPreferencePreflightStatus = Infer<
  typeof TrackingNotificationPreferencePreflightStatusSchema
>;
export type TrackingNotificationPreferencePreflightRow = Infer<typeof TrackingNotificationPreferencePreflightRowSchema>;
export type TrackingNotificationPreferencePreflightReadModel = Infer<
  typeof TrackingNotificationPreferencePreflightReadModelSchema
>;

export type TrackingNotificationPreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

type PreferenceRowInput = Infer<typeof TrackingNotificationPreferencePreflightRowBaseSchema>;
type PreferenceReadModelInput = Infer<typeof TrackingNotificationPreferencePreflightReadModelBaseSchema>;

export function buildTrackingNotificationPreferencePreflightReadModel(
  options: TrackingNotificationPreferencePreflightOptions,
  sourceReadModel: TrackingProviderNotificationProofReadModel
): TrackingNotificationPreferencePreflightReadModel {
  const parsedSource = TrackingProviderNotificationProofReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(trackingPreferencePreflightRowForProviderRow);

  return TrackingNotificationPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceProviderNotificationProofId: parsedSource.proofId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    parentPreferenceRequiredCount: countRows(
      rows,
      TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired
    ),
    sourceManualRequiredCount: countRows(rows, TrackingNotificationPreferencePreflightStatus.SourceManualRequired),
    sourceUnavailableCount: countRows(rows, TrackingNotificationPreferencePreflightStatus.SourceUnavailable),
    preflightNonClaims: RequiredTrackingNotificationPreferencePreflightNonClaims,
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function trackingPreferencePreflightRowForProviderRow(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationPreferencePreflightRow {
  const status = preferenceStatusForProviderKind(row.providerStatusKind);
  const preferenceRefs = preferenceRefsFor(row);
  const quietHoursRefs = quietHoursRefsFor(row);
  const sourceManualRequirements = row.manualProofRequirements;

  return TrackingNotificationPreferencePreflightRowSchema.parse({
    preferenceRowId: `tracking-notification-preference-preflight-${row.sourceAlertId}`,
    sourceProviderNotificationRowId: row.rowId,
    sourceAlertId: row.sourceAlertId,
    providerStatusKind: row.providerStatusKind,
    status,
    sourcePolicyDecisionId: row.sourcePolicyDecisionId,
    evidenceRefs: row.evidenceRefs,
    notificationStatusRefs: row.notificationStatusRefs,
    reasonCodeRefs: row.reasonCodeRefs,
    providerAttemptRef: row.providerStatusBoundaryEntry.providerAttemptRef,
    providerPreferenceRefs: row.providerStatusBoundaryEntry.preferenceRefs,
    parentPreferenceState:
      status === TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired
        ? 'manual-setup-required'
        : null,
    quietHoursDecision:
      status === TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired ? 'manual-required' : null,
    parentPreferenceRequirementRefs: preferenceRefs,
    quietHoursRequirementRefs: quietHoursRefs,
    manualProofRequirements: [...sourceManualRequirements, ...preferenceRefs, ...quietHoursRefs],
  });
}

function preferenceStatusForProviderKind(
  kind: TrackingProviderNotificationStatusKind
): TrackingNotificationPreferencePreflightStatus {
  if (kind === 'provider-adapter-required') {
    return TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired;
  }
  return kind === 'unavailable'
    ? TrackingNotificationPreferencePreflightStatus.SourceUnavailable
    : TrackingNotificationPreferencePreflightStatus.SourceManualRequired;
}

function preferenceRefsFor(row: TrackingProviderNotificationProofRow): readonly string[] {
  return row.providerStatusKind === 'provider-adapter-required'
    ? [
        `tracking-parent-notification-preference-required-${row.sourceAlertId}`,
        `tracking-notification-frequency-control-required-${row.sourceAlertId}`,
      ]
    : row.manualProofRequirements;
}

function quietHoursRefsFor(row: TrackingProviderNotificationProofRow): readonly string[] {
  return row.providerStatusKind === 'provider-adapter-required'
    ? [`tracking-quiet-hours-policy-required-${row.sourceAlertId}`]
    : row.manualProofRequirements;
}

function trackingPreferencePreflightRowIsHonest(row: PreferenceRowInput): boolean {
  const sourceRefsPresent =
    row.evidenceRefs.length > 0 && row.reasonCodeRefs.length > 0 && row.manualProofRequirements.length > 0;
  if (!sourceRefsPresent) {
    return false;
  }
  if (row.status === TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired) {
    return (
      row.parentPreferenceState === 'manual-setup-required' &&
      row.quietHoursDecision === 'manual-required' &&
      row.providerAttemptRef !== null &&
      row.providerPreferenceRefs.length > 0 &&
      row.parentPreferenceRequirementRefs.length >= 2 &&
      row.quietHoursRequirementRefs.length >= 1
    );
  }
  return row.parentPreferenceState === null && row.quietHoursDecision === null;
}

function trackingPreferencePreflightReadModelIsHonest(readModel: PreferenceReadModelInput): boolean {
  return (
    readModel.parentPreferenceRequiredCount ===
      countRows(readModel.rows, TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired) &&
    readModel.sourceManualRequiredCount ===
      countRows(readModel.rows, TrackingNotificationPreferencePreflightStatus.SourceManualRequired) &&
    readModel.sourceUnavailableCount ===
      countRows(readModel.rows, TrackingNotificationPreferencePreflightStatus.SourceUnavailable) &&
    RequiredTrackingNotificationPreferencePreflightNonClaims.every((claim) =>
      readModel.preflightNonClaims.includes(claim)
    )
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: TrackingNotificationPreferencePreflightStatus }>,
  status: TrackingNotificationPreferencePreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

