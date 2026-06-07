import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema,
  AppGameChildUxLocalOutboxProviderPreflightStatus,
  AppGameChildUxLocalOutboxProviderPreflightStatusSchema,
  type AppGameChildUxLocalOutboxProviderPreflightReadModel,
  type AppGameChildUxLocalOutboxProviderPreflightRow,
} from './app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';

const ChildUxProviderStatusHandoffText = Schema.String.pipe(Schema.minLength(1));

export const RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims = [
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
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxLocalOutboxProviderStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims)
);
export const AppGameChildUxLocalOutboxProviderStatusHandoffIdSchema = ChildUxProviderStatusHandoffText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxProviderStatusHandoffId')
);
export const AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema = ChildUxProviderStatusHandoffText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxProviderStatusHandoffReference')
);

const AppGameChildUxLocalOutboxProviderStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema,
  sourcePreflightRowId: AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema,
  sourcePreflightStatus: AppGameChildUxLocalOutboxProviderPreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema, Schema.Null),
  providerStatusBoundaryEntry: V08NotificationProviderStatusBoundaryEntrySchema,
  manualProofRequirements: Schema.Array(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema),
});

export const AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema = withParser(
  AppGameChildUxLocalOutboxProviderStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childUxProviderStatusHandoffRowIsHonest(row) ||
        'Expected child UX provider status handoff rows to map provider preflight rows into manual-required or unavailable provider status boundary entries without claiming delivery'
    )
  )
);

const AppGameChildUxLocalOutboxProviderStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameChildUxLocalOutboxProviderStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderPreflightId: AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema),
  providerStatusBoundaryReadModelRef: AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema,
  providerStatusBoundaryCoverageRefs: Schema.Array(AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema),
  rows: Schema.Array(AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema),
  providerStatusManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(AppGameChildUxLocalOutboxProviderStatusHandoffNonClaimSchema),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema = withParser(
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childUxProviderStatusHandoffReadModelIsHonest(readModel) ||
        'Expected child UX provider status handoff counts and non-claims to match manual-required and unavailable boundary rows'
    )
  )
);

export type AppGameChildUxLocalOutboxProviderStatusHandoffRow = Infer<
  typeof AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema
>;
export type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel = Infer<
  typeof AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema
>;

type ProviderStatusHandoffRowInput = Infer<typeof AppGameChildUxLocalOutboxProviderStatusHandoffRowBaseSchema>;
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

export type AppGameChildUxLocalOutboxProviderStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  sourceReadModel: AppGameChildUxLocalOutboxProviderPreflightReadModel
): AppGameChildUxLocalOutboxProviderStatusHandoffReadModel {
  const parsedSource = AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => providerStatusHandoffRowForPreflightRow(options, row));

  return AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse({
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
    handoffNonClaims: RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims,
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function providerStatusHandoffRowForPreflightRow(
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxProviderPreflightRow
): AppGameChildUxLocalOutboxProviderStatusHandoffRow {
  return AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema.parse({
    handoffRowId: `app-game-child-ux-provider-status-handoff-${row.preflightRowId}`,
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
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxProviderPreflightRow
) {
  const unavailable = row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable;

  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    statusEntryId: `app-game-child-ux-provider-status-${row.preflightRowId}`,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
    quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
    escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
    deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
    notificationIntentRef: `app-game-child-ux-provider-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    notificationStatusRef: `app-game-child-ux-provider-status-ref-${row.preflightRowId}`,
    providerAttemptRef: `app-game-child-ux-provider-attempt-not-started-${row.preflightRowId}`,
    auditRefs: [`app-game-child-ux-provider-status-audit-${row.preflightRowId}`],
    preferenceRefs: providerPreferenceRefsForRow(row),
    readinessRefs: providerReadinessRefsForRow(row),
    providerReceiptRefs: [],
    manualProofRequirements: row.manualProofRequirements,
    minimalPayloadBoundary: unavailable
      ? 'Provider unavailable keeps child UX local outbox delivery unclaimed and visible for manual review.'
      : 'Provider manual-required keeps child UX local outbox delivery blocked until adapter, credentials, preferences, and smoke proof exist.',
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function providerPreferenceRefsForRow(row: AppGameChildUxLocalOutboxProviderPreflightRow): readonly string[] {
  return row.providerChannelRef === null
    ? ['app-game-child-ux-provider-preference-manual-review']
    : [`app-game-child-ux-provider-preference-${row.providerChannelRef}`];
}

function providerReadinessRefsForRow(row: AppGameChildUxLocalOutboxProviderPreflightRow): readonly string[] {
  if (row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable) {
    return ['app-game-child-ux-provider-readiness-unavailable'];
  }
  return row.adapterRequirementRefs.length === 0
    ? ['app-game-child-ux-provider-readiness-manual-required']
    : row.adapterRequirementRefs;
}

function childUxProviderStatusHandoffRowIsHonest(row: ProviderStatusHandoffRowInput): boolean {
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
  status: AppGameChildUxLocalOutboxProviderPreflightStatus
): ProviderStatusBoundaryExpectation {
  if (status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable) {
    return UnavailableProviderStatusBoundaryExpectation;
  }
  return ManualRequiredProviderStatusBoundaryExpectation;
}

function providerStatusBoundaryKeepsDeliveryUnclaimed(entry: ProviderStatusBoundaryEntry): boolean {
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

function childUxProviderStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxProviderStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.providerStatusManualRequiredCount === countProviderStatus(readModel.rows, 'manual-required') &&
    readModel.providerStatusUnavailableCount === countProviderStatus(readModel.rows, 'unavailable') &&
    RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.providerStatusBoundaryCoverageRefs.length ===
      V08NotificationProviderStatusBoundaryReadModel.entries.length &&
    !readModel.childDeliveryRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.retryExecutionRuntimeClaimed &&
    !readModel.quietHoursTimerRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

const countProviderStatus = (
  rows: ReadonlyArray<{
    readonly providerStatusBoundaryEntry: { readonly providerStatus: V08NotificationProviderStatus };
  }>,
  providerStatus: V08NotificationProviderStatus
): number => rows.filter((row) => row.providerStatusBoundaryEntry.providerStatus === providerStatus).length;
