import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { FamilyReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  AppGameChildUxLocalOutboxProviderPreflightStatus,
  AppGameChildUxLocalOutboxProviderPreflightStatusSchema,
} from './app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';

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
export const AppGameChildUxLocalOutboxProviderStatusHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxProviderStatusHandoffId'
);
export const AppGameChildUxLocalOutboxProviderStatusHandoffReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxProviderStatusHandoffReference'
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

export const decodeAppGameChildUxLocalOutboxProviderStatusHandoffReadModel = Schema.decodeUnknownSync(
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema
);
