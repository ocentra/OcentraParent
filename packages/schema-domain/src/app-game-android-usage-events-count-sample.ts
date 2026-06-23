import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidUsageEventsCountSampleSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-count-sample')
);

export const AppGameAndroidUsageEventsCountSampleStateSchema = withParser(
  Schema.Literal('sample-permission-required', 'sample-observed', 'sample-empty', 'sample-unavailable')
);

export const AppGameAndroidUsageEventsCountSampleRefSchema = withParser(
  Schema.Literal('android-usage-events-count-sample-ref', 'android-usage-events-runtime-preflight-ref')
);

export const AppGameAndroidUsageEventsCountSampleGapSchema = withParser(
  Schema.Literal(
    'android-usage-stats-settings-grant-not-proved',
    'android-usage-events-runtime-sample-not-observed',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const CountSampleLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidUsageEventsCountSampleLabel');

const AppGameAndroidUsageEventsCountSampleReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsCountSampleSchemaVersionSchema,
  packageId: CountSampleLabelSchema,
  nativeBridgeClass: CountSampleLabelSchema,
  sampleState: AppGameAndroidUsageEventsCountSampleStateSchema,
  sampleLookbackMillis: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  sampleEventCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  foregroundEventCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  proofRefs: Schema.Array(AppGameAndroidUsageEventsCountSampleRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsCountSampleGapSchema),
  rawUsageEventsStored: Schema.Literal(false),
  packageNamesStored: Schema.Literal(false),
  rawActivityRowsStored: Schema.Literal(false),
  runtimeCollectionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: CountSampleLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type CountSampleCandidate = Infer<typeof AppGameAndroidUsageEventsCountSampleReadModelBaseSchema>;

export const AppGameAndroidUsageEventsCountSampleReadModelSchema = withParser(
  AppGameAndroidUsageEventsCountSampleReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        countSampleReadModelIsHonest(readModel) ||
        'Expected Android UsageEvents count sample to stay count-only without raw rows, package names, dispatch, enforcement, or child delivery claims'
    )
  )
);

export type AppGameAndroidUsageEventsCountSampleReadModel = Infer<
  typeof AppGameAndroidUsageEventsCountSampleReadModelSchema
>;

export const decodeAppGameAndroidUsageEventsCountSampleReadModel = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsCountSampleReadModelSchema
);

export function createAppGameAndroidUsageEventsCountSampleReadModel(input: {
  readonly sampleState: AppGameAndroidUsageEventsCountSampleReadModel['sampleState'];
  readonly sampleLookbackMillis: AppGameAndroidUsageEventsCountSampleReadModel['sampleLookbackMillis'];
  readonly sampleEventCount: AppGameAndroidUsageEventsCountSampleReadModel['sampleEventCount'];
  readonly foregroundEventCount: AppGameAndroidUsageEventsCountSampleReadModel['foregroundEventCount'];
  readonly checkedAt: AppGameAndroidUsageEventsCountSampleReadModel['checkedAt'];
}): AppGameAndroidUsageEventsCountSampleReadModel {
  return decodeAppGameAndroidUsageEventsCountSampleReadModel({
    schemaVersion: 'app-game-android-usage-events-count-sample',
    packageId: 'ca.ocentra.parent.agent',
    nativeBridgeClass: 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight',
    sampleState: input.sampleState,
    sampleLookbackMillis: input.sampleLookbackMillis,
    sampleEventCount: input.sampleEventCount,
    foregroundEventCount: input.foregroundEventCount,
    proofRefs: ['android-usage-events-count-sample-ref', 'android-usage-events-runtime-preflight-ref'],
    openGaps: countSampleOpenGaps(input.sampleState),
    rawUsageEventsStored: false,
    packageNamesStored: false,
    rawActivityRowsStored: false,
    runtimeCollectionClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary: countSampleSummary(input.sampleState),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidUsageEventsCountSampleReadModel(
  readModel: AppGameAndroidUsageEventsCountSampleReadModel
) {
  return {
    sampleState: readModel.sampleState,
    sampleEventCount: readModel.sampleEventCount,
    foregroundEventCount: readModel.foregroundEventCount,
    openGapCount: readModel.openGaps.length,
    runtimeCollectionClaimed: readModel.runtimeCollectionClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
  } as const;
}

function countSampleOpenGaps(sampleState: CountSampleCandidate['sampleState']) {
  const gaps = ['android-child-runtime-delivery-not-proved', 'android-platform-enforcement-not-proved'];
  if (sampleState === 'sample-permission-required') {
    gaps.unshift('android-usage-stats-settings-grant-not-proved');
  }
  if (sampleState !== 'sample-observed') {
    gaps.unshift('android-usage-events-runtime-sample-not-observed');
  }
  return gaps;
}

function countSampleSummary(sampleState: CountSampleCandidate['sampleState']) {
  return sampleState === 'sample-observed'
    ? 'Android package observed count-only UsageEvents samples, but raw rows, package names, child delivery, and enforcement remain unclaimed.'
    : 'Android package UsageEvents sample is not observed yet; settings grant, runtime sample proof, child delivery, and enforcement remain open.';
}

function countSampleReadModelIsHonest(readModel: CountSampleCandidate): boolean {
  const countsMatchState =
    readModel.sampleState === 'sample-observed'
      ? readModel.sampleEventCount > 0
      : readModel.sampleEventCount === 0 && readModel.foregroundEventCount === 0;
  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.nativeBridgeClass === 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight' &&
    countsMatchState &&
    readModel.foregroundEventCount <= readModel.sampleEventCount &&
    readModel.proofRefs.includes('android-usage-events-count-sample-ref') &&
    readModel.proofRefs.includes('android-usage-events-runtime-preflight-ref') &&
    readModel.openGaps.includes('android-child-runtime-delivery-not-proved') &&
    readModel.openGaps.includes('android-platform-enforcement-not-proved')
  );
}
