import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGameChildUxCardSchema, type AppGameChildUxCard } from './app-game-child-facing-ux';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

export const AppGameChildUxHandoffStatus = {
  Ready: 'ready-for-local-child-ux-handoff',
  BlockedMissingRefs: 'blocked-missing-child-refs',
} as const;

export const AppGameChildUxHandoffStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxHandoffStatus))
);
export const AppGameChildUxHandoffIdSchema = brandedNonEmptyStringSchema('AppGameChildUxHandoffId');
export const AppGameChildUxHandoffReferenceSchema = brandedNonEmptyStringSchema('AppGameChildUxHandoffReference');

const AppGameChildUxHandoffRowBaseSchema = Schema.Struct({
  handoffReferenceId: AppGameChildUxHandoffReferenceSchema,
  status: AppGameChildUxHandoffStatusSchema,
  card: AppGameChildUxCardSchema,
  blockedReasonRefs: Schema.Array(AppGameChildUxHandoffReferenceSchema),
});

export const AppGameChildUxHandoffRowSchema = withParser(
  AppGameChildUxHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildUxHandoffRowIsHonest(row) ||
        'Expected child UX handoff rows to require child reason and status refs before local handoff readiness'
    )
  )
);

const AppGameChildUxHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameChildUxHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  localHandoffRootRef: AppGameChildUxHandoffReferenceSchema,
  rows: Schema.Array(AppGameChildUxHandoffRowSchema),
  readyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  blockedMissingRefsCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  privateDiagnosticsIncluded: Schema.Literal(false),
});

export const AppGameChildUxHandoffReadModelSchema = withParser(
  AppGameChildUxHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildUxHandoffReadModelCountsMatch(readModel) ||
        'Expected child UX handoff counts to match ready and blocked rows'
    )
  )
);

export type AppGameChildUxHandoffStatus = Infer<typeof AppGameChildUxHandoffStatusSchema>;
export type AppGameChildUxHandoffRow = Infer<typeof AppGameChildUxHandoffRowSchema>;
export type AppGameChildUxHandoffReadModel = Infer<typeof AppGameChildUxHandoffReadModelSchema>;

export type AppGameChildUxHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly localHandoffRootRef: string;
};

export function buildAppGameChildUxHandoffReadModel(
  options: AppGameChildUxHandoffOptions,
  cards: ReadonlyArray<AppGameChildUxCard>
): AppGameChildUxHandoffReadModel {
  const rows = cards.map(appGameChildUxToHandoffRow);

  return AppGameChildUxHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    localHandoffRootRef: options.localHandoffRootRef,
    rows,
    readyCount: countRows(rows, AppGameChildUxHandoffStatus.Ready),
    blockedMissingRefsCount: countRows(rows, AppGameChildUxHandoffStatus.BlockedMissingRefs),
    childDeliveryRuntimeClaimed: false,
    notificationDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    privateDiagnosticsIncluded: false,
  });
}

export function appGameChildUxToHandoffRow(candidate: AppGameChildUxCard): AppGameChildUxHandoffRow {
  const card = AppGameChildUxCardSchema.parse(candidate);
  const ready = card.childReasonReferences.length > 0 && card.childStatusReferences.length > 0;

  return AppGameChildUxHandoffRowSchema.parse({
    handoffReferenceId: `app-game-child-ux-handoff-${card.childUxStateId}`,
    status: ready ? AppGameChildUxHandoffStatus.Ready : AppGameChildUxHandoffStatus.BlockedMissingRefs,
    card,
    blockedReasonRefs: ready ? [] : ['child-reason-and-status-refs-required'],
  });
}

function appGameChildUxHandoffRowIsHonest(row: Infer<typeof AppGameChildUxHandoffRowBaseSchema>): boolean {
  const hasChildRefs = row.card.childReasonReferences.length > 0 && row.card.childStatusReferences.length > 0;

  if (row.status === AppGameChildUxHandoffStatus.Ready) {
    return hasChildRefs && row.blockedReasonRefs.length === 0;
  }

  return !hasChildRefs && row.blockedReasonRefs.length > 0;
}

function appGameChildUxHandoffReadModelCountsMatch(
  readModel: Infer<typeof AppGameChildUxHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.readyCount === countRows(readModel.rows, AppGameChildUxHandoffStatus.Ready) &&
    readModel.blockedMissingRefsCount === countRows(readModel.rows, AppGameChildUxHandoffStatus.BlockedMissingRefs)
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxHandoffStatus }>,
  status: AppGameChildUxHandoffStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

