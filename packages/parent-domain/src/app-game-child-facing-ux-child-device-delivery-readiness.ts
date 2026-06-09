import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
  type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxProviderStatusHandoffRow,
} from './app-game-child-facing-ux-local-outbox-provider-status-handoff';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';

const ChildDeviceDeliveryReadinessText = Schema.String.pipe(Schema.minLength(1));

export const AppGameChildDeviceDeliveryReadinessStatus = {
  TransportRequired: 'child-transport-required',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildDeviceDeliveryReadinessStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildDeviceDeliveryReadinessStatus))
);

export const AppGameChildDeviceDeliveryReadinessGapSchema = withParser(
  Schema.Literal(
    'child-runtime-transport-not-attached',
    'child-runtime-receipt-not-ingested',
    'provider-delivery-not-executed',
    'platform-delivery-channel-not-proved',
    'manual-proof-required',
    'source-unavailable'
  )
);

export const AppGameChildDeviceDeliveryReadinessNonClaimSchema = withParser(
  Schema.Literal(
    'no-child-runtime-transport',
    'no-child-runtime-receipt-ingestion',
    'no-provider-delivery-execution',
    'no-platform-delivery-channel',
    'no-adapter-dispatch',
    'no-platform-enforcement',
    'no-raw-private-source-rows'
  )
);

export const RequiredAppGameChildDeviceDeliveryReadinessNonClaims = [
  'no-child-runtime-transport',
  'no-child-runtime-receipt-ingestion',
  'no-provider-delivery-execution',
  'no-platform-delivery-channel',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

const ChildDeviceDeliveryReadinessIdSchema = ChildDeviceDeliveryReadinessText.pipe(
  Schema.brand('AppGameChildDeviceDeliveryReadinessId')
);
const ChildDeviceDeliveryReadinessReferenceSchema = ChildDeviceDeliveryReadinessText.pipe(
  Schema.brand('AppGameChildDeviceDeliveryReadinessReference')
);

const AppGameChildDeviceDeliveryReadinessRowBaseSchema = Schema.Struct({
  deliveryReadinessRowId: ChildDeviceDeliveryReadinessReferenceSchema,
  sourceProviderStatusHandoffRowId: ChildDeviceDeliveryReadinessReferenceSchema,
  sourceProviderStatus: ChildDeviceDeliveryReadinessReferenceSchema,
  sourceOutboxRecordRef: Schema.Union(ChildDeviceDeliveryReadinessReferenceSchema, Schema.Null),
  sourceSchedulerEntryRef: Schema.Union(ChildDeviceDeliveryReadinessReferenceSchema, Schema.Null),
  deliveryReadinessStatus: AppGameChildDeviceDeliveryReadinessStatusSchema,
  requiredTransportRefs: Schema.Array(ChildDeviceDeliveryReadinessReferenceSchema),
  openGaps: Schema.Array(AppGameChildDeviceDeliveryReadinessGapSchema),
  childRuntimeTransportClaimed: Schema.Literal(false),
  childRuntimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
});

export const AppGameChildDeviceDeliveryReadinessRowSchema = withParser(
  AppGameChildDeviceDeliveryReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildDeviceDeliveryReadinessRowIsHonest(row) ||
        'Expected app/game child-device delivery rows to keep runtime transport, receipt ingestion, provider delivery, and platform channel claims false'
    )
  )
);

const AppGameChildDeviceDeliveryReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: ChildDeviceDeliveryReadinessIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderStatusHandoffId: ChildDeviceDeliveryReadinessReferenceSchema,
  rows: Schema.Array(AppGameChildDeviceDeliveryReadinessRowSchema),
  transportRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  nonClaims: Schema.Array(AppGameChildDeviceDeliveryReadinessNonClaimSchema),
  childRuntimeTransportClaimed: Schema.Literal(false),
  childRuntimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildDeviceDeliveryReadinessReadModelSchema = withParser(
  AppGameChildDeviceDeliveryReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildDeviceDeliveryReadinessReadModelIsHonest(readModel) ||
        'Expected app/game child-device delivery readiness counts and non-claims to match child transport required, manual-required, and unavailable rows'
    )
  )
);

export type AppGameChildDeviceDeliveryReadinessStatus = Infer<
  typeof AppGameChildDeviceDeliveryReadinessStatusSchema
>;
export type AppGameChildDeviceDeliveryReadinessRow = Infer<
  typeof AppGameChildDeviceDeliveryReadinessRowSchema
>;
export type AppGameChildDeviceDeliveryReadinessReadModel = Infer<
  typeof AppGameChildDeviceDeliveryReadinessReadModelSchema
>;

export type AppGameChildDeviceDeliveryReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly requiredTransportRefs: readonly string[];
};

export function buildAppGameChildDeviceDeliveryReadinessReadModel(
  options: AppGameChildDeviceDeliveryReadinessOptions,
  sourceReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel
): AppGameChildDeviceDeliveryReadinessReadModel {
  const source = AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => childDeviceDeliveryReadinessRow(options, row));

  return AppGameChildDeviceDeliveryReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceProviderStatusHandoffId: source.handoffId,
    rows,
    transportRequiredCount: countRows(rows, AppGameChildDeviceDeliveryReadinessStatus.TransportRequired),
    manualRequiredCount: countRows(rows, AppGameChildDeviceDeliveryReadinessStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildDeviceDeliveryReadinessStatus.Unavailable),
    nonClaims: RequiredAppGameChildDeviceDeliveryReadinessNonClaims,
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function summarizeAppGameChildDeviceDeliveryReadiness(
  readModel: AppGameChildDeviceDeliveryReadinessReadModel
) {
  return {
    transportRequiredCount: readModel.transportRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    rowCount: readModel.rows.length,
  } as const;
}

function childDeviceDeliveryReadinessRow(
  options: AppGameChildDeviceDeliveryReadinessOptions,
  row: AppGameChildUxLocalOutboxProviderStatusHandoffRow
): AppGameChildDeviceDeliveryReadinessRow {
  const status = childDeviceDeliveryStatusFor(row);

  return AppGameChildDeviceDeliveryReadinessRowSchema.parse({
    deliveryReadinessRowId: `app-game-child-device-delivery-readiness-${row.handoffRowId}`,
    sourceProviderStatusHandoffRowId: row.handoffRowId,
    sourceProviderStatus: row.providerStatusBoundaryEntry.providerStatus,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    deliveryReadinessStatus: status,
    requiredTransportRefs:
      status === AppGameChildDeviceDeliveryReadinessStatus.TransportRequired
        ? options.requiredTransportRefs
        : row.manualProofRequirements,
    openGaps: childDeviceDeliveryGapsFor(status),
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  });
}

function childDeviceDeliveryStatusFor(
  row: AppGameChildUxLocalOutboxProviderStatusHandoffRow
): AppGameChildDeviceDeliveryReadinessStatus {
  if (row.providerStatusBoundaryEntry.providerStatus === 'unavailable') {
    return AppGameChildDeviceDeliveryReadinessStatus.Unavailable;
  }
  if (row.sourceSchedulerEntryRef === null || row.sourceOutboxRecordRef === null) {
    return AppGameChildDeviceDeliveryReadinessStatus.ManualRequired;
  }
  return AppGameChildDeviceDeliveryReadinessStatus.TransportRequired;
}

function childDeviceDeliveryGapsFor(status: AppGameChildDeviceDeliveryReadinessStatus) {
  if (status === AppGameChildDeviceDeliveryReadinessStatus.Unavailable) {
    return ['source-unavailable', 'child-runtime-transport-not-attached'] as const;
  }
  if (status === AppGameChildDeviceDeliveryReadinessStatus.ManualRequired) {
    return ['manual-proof-required', 'child-runtime-transport-not-attached'] as const;
  }
  return [
    'child-runtime-transport-not-attached',
    'child-runtime-receipt-not-ingested',
    'provider-delivery-not-executed',
    'platform-delivery-channel-not-proved',
  ] as const;
}

function appGameChildDeviceDeliveryReadinessRowIsHonest(
  row: Infer<typeof AppGameChildDeviceDeliveryReadinessRowBaseSchema>
): boolean {
  return (
    row.openGaps.includes('child-runtime-transport-not-attached') &&
    row.requiredTransportRefs.length > 0 &&
    !row.childRuntimeTransportClaimed &&
    !row.childRuntimeReceiptIngested &&
    !row.providerDeliveryExecuted &&
    !row.platformDeliveryChannelClaimed
  );
}

function appGameChildDeviceDeliveryReadinessReadModelIsHonest(
  readModel: Infer<typeof AppGameChildDeviceDeliveryReadinessReadModelBaseSchema>
): boolean {
  return (
    readModel.transportRequiredCount ===
      countRows(readModel.rows, AppGameChildDeviceDeliveryReadinessStatus.TransportRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameChildDeviceDeliveryReadinessStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameChildDeviceDeliveryReadinessStatus.Unavailable) &&
    RequiredAppGameChildDeviceDeliveryReadinessNonClaims.every((claim) => readModel.nonClaims.includes(claim)) &&
    !readModel.childRuntimeTransportClaimed &&
    !readModel.childRuntimeReceiptIngested &&
    !readModel.providerDeliveryExecuted &&
    !readModel.platformDeliveryChannelClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly deliveryReadinessStatus: AppGameChildDeviceDeliveryReadinessStatus }>,
  status: AppGameChildDeviceDeliveryReadinessStatus
): number {
  return rows.filter((row) => row.deliveryReadinessStatus === status).length;
}
