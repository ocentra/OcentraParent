import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import {
  AppGameNotificationProviderPreflightReadModelSchema,
  AppGameNotificationProviderPreflightStatus,
  AppGameNotificationProviderPreflightStatusSchema,
  type AppGameNotificationProviderPreflightReadModel,
  type AppGameNotificationProviderPreflightRow,
} from './app-game-notification-provider-preflight';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';

export const RequiredAppGameNotificationProviderStatusHandoffNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-quiet-hours-timer-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const AppGameNotificationProviderStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationProviderStatusHandoffNonClaims)
);

export const AppGameNotificationProviderStatusHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationProviderStatusHandoffId'
);
export const AppGameNotificationProviderStatusHandoffReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationProviderStatusHandoffReference'
);

const AppGameNotificationProviderStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: AppGameNotificationProviderStatusHandoffReferenceSchema,
  sourcePreflightRowId: AppGameNotificationProviderStatusHandoffReferenceSchema,
  sourcePreflightStatus: AppGameNotificationProviderPreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationProviderStatusHandoffReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationProviderStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(AppGameNotificationProviderStatusHandoffReferenceSchema, Schema.Null),
  providerStatusBoundaryEntry: V08NotificationProviderStatusBoundaryEntrySchema,
  manualProofRequirements: Schema.Array(AppGameNotificationProviderStatusHandoffReferenceSchema),
});

export const AppGameNotificationProviderStatusHandoffRowSchema = withParser(
  AppGameNotificationProviderStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStatusHandoffRowIsHonest(row) ||
        'Expected app/game notification provider status handoff rows to map provider preflight rows into manual-required or unavailable provider status boundary entries without claiming delivery'
    )
  )
);

const AppGameNotificationProviderStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameNotificationProviderStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderPreflightId: AppGameNotificationProviderStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameNotificationProviderStatusHandoffReferenceSchema),
  providerStatusBoundaryReadModelRef: AppGameNotificationProviderStatusHandoffReferenceSchema,
  providerStatusBoundaryCoverageRefs: Schema.Array(AppGameNotificationProviderStatusHandoffReferenceSchema),
  rows: Schema.Array(AppGameNotificationProviderStatusHandoffRowSchema),
  providerStatusManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(AppGameNotificationProviderStatusHandoffNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const AppGameNotificationProviderStatusHandoffReadModelSchema = withParser(
  AppGameNotificationProviderStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        providerStatusHandoffReadModelIsHonest(readModel) ||
        'Expected app/game notification provider status handoff counts and non-claims to match manual-required and unavailable boundary rows'
    )
  )
);

export type AppGameNotificationProviderStatusHandoffRow = Infer<
  typeof AppGameNotificationProviderStatusHandoffRowSchema
>;
export type AppGameNotificationProviderStatusHandoffReadModel = Infer<
  typeof AppGameNotificationProviderStatusHandoffReadModelSchema
>;

type ProviderStatusHandoffRowInput = Infer<typeof AppGameNotificationProviderStatusHandoffRowBaseSchema>;
type ProviderStatusBoundaryEntry = ProviderStatusHandoffRowInput['providerStatusBoundaryEntry'];
type ProviderStatusBoundaryExpectation = Pick<
  ProviderStatusBoundaryEntry,
  'providerStatus' | 'statusProofState' | 'quietHoursReadiness' | 'escalationReadiness'
>;

const UnavailableProviderStatusBoundaryExpectation: ProviderStatusBoundaryExpectation = {
  providerStatus: 'unavailable',
  statusProofState: 'provider-unavailable-contract',
  quietHoursReadiness: 'unavailable',
  escalationReadiness: 'unavailable',
};

const ManualRequiredProviderStatusBoundaryExpectation: ProviderStatusBoundaryExpectation = {
  providerStatus: 'manual-required',
  statusProofState: 'manual-action-required',
  quietHoursReadiness: 'manual-required',
  escalationReadiness: 'manual-required',
};

export type AppGameNotificationProviderStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationProviderStatusHandoffReadModel(
  options: AppGameNotificationProviderStatusHandoffOptions,
  sourceReadModel: AppGameNotificationProviderPreflightReadModel
): AppGameNotificationProviderStatusHandoffReadModel {
  const parsedSource = AppGameNotificationProviderPreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => providerStatusHandoffRowForPreflightRow(options, row));

  return AppGameNotificationProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceProviderPreflightId: parsedSource.providerPreflightId,
    sourceContractRefs: options.sourceContractRefs,
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerStatusBoundaryCoverageRefs: V08NotificationProviderStatusBoundaryReadModel.entries.map(
      (entry) => entry.statusEntryId
    ),
    rows,
    providerStatusManualRequiredCount: countProviderStatus(rows, 'manual-required'),
    providerStatusUnavailableCount: countProviderStatus(rows, 'unavailable'),
    handoffNonClaims: RequiredAppGameNotificationProviderStatusHandoffNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function providerStatusHandoffRowForPreflightRow(
  options: AppGameNotificationProviderStatusHandoffOptions,
  row: AppGameNotificationProviderPreflightRow
): AppGameNotificationProviderStatusHandoffRow {
  return AppGameNotificationProviderStatusHandoffRowSchema.parse({
    handoffRowId: `app-game-notification-provider-status-handoff-${row.preflightRowId}`,
    sourcePreflightRowId: row.preflightRowId,
    sourcePreflightStatus: row.status,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    providerStatusBoundaryEntry: providerStatusBoundaryEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function providerStatusBoundaryEntryForPreflightRow(
  options: AppGameNotificationProviderStatusHandoffOptions,
  row: AppGameNotificationProviderPreflightRow
) {
  const unavailable = row.status === AppGameNotificationProviderPreflightStatus.Unavailable;
  const entryId = `app-game-notification-provider-status-${row.preflightRowId}`;

  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    statusEntryId: entryId,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
    quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
    escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
    deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
    notificationIntentRef: `app-game-notification-provider-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    notificationStatusRef: `app-game-notification-provider-status-ref-${row.preflightRowId}`,
    providerAttemptRef: `app-game-notification-provider-attempt-not-started-${row.preflightRowId}`,
    auditRefs: [`app-game-notification-provider-status-audit-${row.preflightRowId}`],
    preferenceRefs: providerPreferenceRefsForRow(row),
    readinessRefs: providerReadinessRefsForRow(row),
    providerReceiptRefs: [],
    manualProofRequirements: row.manualProofRequirements,
    minimalPayloadBoundary: unavailable
      ? 'Provider unavailable keeps app/game notification delivery unclaimed and visible for manual review.'
      : 'Provider manual-required keeps app/game notification delivery blocked until adapter, credentials, preferences, and smoke proof exist.',
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function providerPreferenceRefsForRow(row: AppGameNotificationProviderPreflightRow): readonly string[] {
  return row.providerChannelRef === null
    ? ['app-game-notification-provider-preference-manual-review']
    : [`app-game-notification-provider-preference-${row.providerChannelRef}`];
}

function providerReadinessRefsForRow(row: AppGameNotificationProviderPreflightRow): readonly string[] {
  if (row.status === AppGameNotificationProviderPreflightStatus.Unavailable) {
    return ['app-game-notification-provider-readiness-unavailable'];
  }
  return row.adapterRequirementRefs.length === 0
    ? ['app-game-notification-provider-readiness-manual-required']
    : row.adapterRequirementRefs;
}

function providerStatusHandoffRowIsHonest(row: ProviderStatusHandoffRowInput): boolean {
  const entry = row.providerStatusBoundaryEntry;

  return (
    providerStatusBoundaryMatchesPreflight(row) &&
    providerStatusBoundaryKeepsDeliveryUnclaimed(entry) &&
    row.manualProofRequirements.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function providerStatusBoundaryMatchesPreflight(row: ProviderStatusHandoffRowInput): boolean {
  const entry = row.providerStatusBoundaryEntry;
  const expected = providerStatusBoundaryExpectationFor(row.sourcePreflightStatus);

  return (
    entry.providerStatus === expected.providerStatus &&
    entry.statusProofState === expected.statusProofState &&
    entry.quietHoursReadiness === expected.quietHoursReadiness &&
    entry.escalationReadiness === expected.escalationReadiness
  );
}

function providerStatusBoundaryExpectationFor(
  status: AppGameNotificationProviderPreflightStatus
): ProviderStatusBoundaryExpectation {
  if (status === AppGameNotificationProviderPreflightStatus.Unavailable) {
    return UnavailableProviderStatusBoundaryExpectation;
  }
  return ManualRequiredProviderStatusBoundaryExpectation;
}

function providerStatusBoundaryKeepsDeliveryUnclaimed(entry: ProviderStatusBoundaryEntry): boolean {
  const deliveryClaims = [
    entry.providerDeliveryImplemented,
    entry.providerDeliveryObserved,
    entry.deliveredNotificationClaimed,
    entry.sensitiveProviderPayloadClaimed,
    entry.providerStoresChildEvidenceClaimed,
  ];

  return entry.providerReceiptRefs.length === 0 && deliveryClaims.every((claim) => claim === false);
}

function providerStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof AppGameNotificationProviderStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.providerStatusManualRequiredCount === countProviderStatus(readModel.rows, 'manual-required') &&
    readModel.providerStatusUnavailableCount === countProviderStatus(readModel.rows, 'unavailable') &&
    RequiredAppGameNotificationProviderStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.providerStatusBoundaryCoverageRefs.length ===
      V08NotificationProviderStatusBoundaryReadModel.entries.length
  );
}

const countProviderStatus = (
  rows: ReadonlyArray<{
    readonly providerStatusBoundaryEntry: { readonly providerStatus: V08NotificationProviderStatus };
  }>,
  providerStatus: V08NotificationProviderStatus
): number => rows.filter((row) => row.providerStatusBoundaryEntry.providerStatus === providerStatus).length;

