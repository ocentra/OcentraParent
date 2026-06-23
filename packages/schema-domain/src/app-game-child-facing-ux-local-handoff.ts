import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGameChildUxCardSchema } from './app-game-child-facing-ux';
import {
  AppGameChildUxHandoffReadModelSchema,
  AppGameChildUxHandoffStatus,
  type AppGameChildUxHandoffReadModel,
  type AppGameChildUxHandoffRow,
} from './app-game-child-facing-ux-handoff';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';

export const AppGameChildUxLocalHandoffArtifactRecordIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalHandoffArtifactRecordId'
);
export const AppGameChildUxLocalHandoffArtifactReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalHandoffArtifactReference'
);

const AppGameChildUxLocalHandoffArtifactRecordBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  recordId: AppGameChildUxLocalHandoffArtifactRecordIdSchema,
  sourceHandoffId: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  sourceHandoffReferenceId: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  localArtifactRootRef: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  localArtifactFileRef: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  card: AppGameChildUxCardSchema,
  childReasonReferences: Schema.Array(AppGameChildUxLocalHandoffArtifactReferenceSchema),
  childStatusReferences: Schema.Array(AppGameChildUxLocalHandoffArtifactReferenceSchema),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  privateDiagnosticsIncluded: Schema.Literal(false),
  writtenAt: ParentTimestampSchema,
});

export const AppGameChildUxLocalHandoffArtifactRecordSchema = withParser(
  AppGameChildUxLocalHandoffArtifactRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        appGameChildUxLocalHandoffArtifactRecordIsHonest(record) ||
        'Expected child UX local handoff artifact records to include only ready child-safe rows without delivery adapter platform or diagnostics claims'
    )
  )
);

const AppGameChildUxLocalHandoffArtifactReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  sourceHandoffId: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  generatedAt: ParentTimestampSchema,
  localArtifactRootRef: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  localArtifactFileRef: AppGameChildUxLocalHandoffArtifactReferenceSchema,
  records: Schema.Array(AppGameChildUxLocalHandoffArtifactRecordSchema),
  writtenRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  skippedBlockedRowCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  privateDiagnosticsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalHandoffArtifactReadModelSchema = withParser(
  AppGameChildUxLocalHandoffArtifactReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildUxLocalHandoffArtifactCountsMatch(readModel) ||
        'Expected child UX local handoff artifact counts to match written records and skipped blocked rows'
    )
  )
);

export type AppGameChildUxLocalHandoffArtifactRecord = Infer<typeof AppGameChildUxLocalHandoffArtifactRecordSchema>;
export type AppGameChildUxLocalHandoffArtifactReadModel = Infer<
  typeof AppGameChildUxLocalHandoffArtifactReadModelSchema
>;

export type AppGameChildUxLocalHandoffArtifactOptions = {
  readonly generatedAt: string;
  readonly localArtifactRootRef: string;
  readonly localArtifactFileRef: string;
};

export function buildAppGameChildUxLocalHandoffArtifactReadModel(
  options: AppGameChildUxLocalHandoffArtifactOptions,
  handoffReadModel: AppGameChildUxHandoffReadModel
): AppGameChildUxLocalHandoffArtifactReadModel {
  const source = AppGameChildUxHandoffReadModelSchema.parse(handoffReadModel);
  const records = source.rows.flatMap((row) =>
    row.status === AppGameChildUxHandoffStatus.Ready
      ? [appGameChildUxHandoffRowToLocalArtifactRecord(options, source.handoffId, row)]
      : []
  );

  return AppGameChildUxLocalHandoffArtifactReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    sourceHandoffId: source.handoffId,
    generatedAt: options.generatedAt,
    localArtifactRootRef: options.localArtifactRootRef,
    localArtifactFileRef: options.localArtifactFileRef,
    records,
    writtenRecordCount: records.length,
    skippedBlockedRowCount: source.rows.length - records.length,
    childDeliveryRuntimeClaimed: false,
    notificationDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    privateDiagnosticsIncluded: false,
  });
}

export function serializeAppGameChildUxLocalHandoffJsonl(
  readModel: AppGameChildUxLocalHandoffArtifactReadModel
): string {
  return `${readModel.records.map((record) => JSON.stringify(record)).join('\n')}\n`;
}

export function parseAppGameChildUxLocalHandoffJsonl(jsonl: string): AppGameChildUxLocalHandoffArtifactRecord[] {
  return jsonl
    .split('\n')
    .filter((line) => line.trim().length > 0)
    .map((line) => AppGameChildUxLocalHandoffArtifactRecordSchema.parse(JSON.parse(line)));
}

function appGameChildUxHandoffRowToLocalArtifactRecord(
  options: AppGameChildUxLocalHandoffArtifactOptions,
  sourceHandoffId: string,
  row: AppGameChildUxHandoffRow
): AppGameChildUxLocalHandoffArtifactRecord {
  return AppGameChildUxLocalHandoffArtifactRecordSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    recordId: `app-game-child-ux-local-handoff-${row.card.childUxStateId}`,
    sourceHandoffId,
    sourceHandoffReferenceId: row.handoffReferenceId,
    localArtifactRootRef: options.localArtifactRootRef,
    localArtifactFileRef: options.localArtifactFileRef,
    card: row.card,
    childReasonReferences: row.card.childReasonReferences,
    childStatusReferences: row.card.childStatusReferences,
    childDeliveryRuntimeClaimed: false,
    notificationDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    privateDiagnosticsIncluded: false,
    writtenAt: options.generatedAt,
  });
}

function appGameChildUxLocalHandoffArtifactRecordIsHonest(
  record: Infer<typeof AppGameChildUxLocalHandoffArtifactRecordBaseSchema>
): boolean {
  return (
    record.childReasonReferences.length > 0 &&
    record.childStatusReferences.length > 0 &&
    record.card.childReasonReferences.length === record.childReasonReferences.length &&
    record.card.childStatusReferences.length === record.childStatusReferences.length &&
    record.card.privateDiagnosticReferences.length === 0 &&
    record.childDeliveryRuntimeClaimed === false &&
    record.notificationDeliveryClaimed === false &&
    record.adapterDispatchClaimed === false &&
    record.platformEnforcementClaimed === false &&
    record.privateDiagnosticsIncluded === false
  );
}

function appGameChildUxLocalHandoffArtifactCountsMatch(
  readModel: Infer<typeof AppGameChildUxLocalHandoffArtifactReadModelBaseSchema>
): boolean {
  return (
    readModel.writtenRecordCount === readModel.records.length &&
    readModel.childDeliveryRuntimeClaimed === false &&
    readModel.notificationDeliveryClaimed === false &&
    readModel.adapterDispatchClaimed === false &&
    readModel.platformEnforcementClaimed === false &&
    readModel.privateDiagnosticsIncluded === false
  );
}
