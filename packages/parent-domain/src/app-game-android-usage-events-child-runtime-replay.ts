import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameAndroidUsageEventsReplayReadModelSchema,
  type AppGameAndroidUsageEventsReplayReadModel,
} from './app-game-android-usage-events-replay';
import { ParentTimestampSchema } from './reference-primitives';

const AndroidRuntimeReplayText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidUsageEventsChildRuntimeReplaySchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-child-runtime-replay')
);

export const AppGameAndroidUsageEventsChildRuntimeReplayStateSchema = withParser(
  Schema.Literal('consumer-attached-redacted-replay', 'consumer-not-ready')
);

export const AppGameAndroidUsageEventsChildRuntimeReplayCustodySchema = withParser(
  Schema.Literal('redacted-runtime-counters-only')
);

export const AppGameAndroidUsageEventsChildRuntimeReplayRefSchema = withParser(
  Schema.Literal(
    'android-usage-events-child-runtime-replay-ref',
    'android-usage-events-replay-ref',
    'android-runtime-visibility-read-model-ref'
  )
);

export const AppGameAndroidUsageEventsChildRuntimeReplayGapSchema = withParser(
  Schema.Literal(
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-hide-suspend-not-proved',
    'android-platform-enforcement-not-proved',
    'android-child-device-delivery-not-proved'
  )
);

const AndroidRuntimeReplayLabelSchema = AndroidRuntimeReplayText.pipe(
  Schema.brand('AppGameAndroidUsageEventsChildRuntimeReplayLabel')
);
const AndroidRuntimeReplayCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGameAndroidUsageEventsChildRuntimeReplayBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsChildRuntimeReplaySchemaVersionSchema,
  readModelId: AndroidRuntimeReplayLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceReplayReadModelId: AndroidRuntimeReplayLabelSchema,
  replayState: AppGameAndroidUsageEventsChildRuntimeReplayStateSchema,
  custodyState: AppGameAndroidUsageEventsChildRuntimeReplayCustodySchema,
  replayedForegroundEventCount: AndroidRuntimeReplayCountSchema,
  replayedUsageEventSampleCount: AndroidRuntimeReplayCountSchema,
  childRuntimeReplayConsumerAttached: Schema.Boolean,
  rawPackageNamesClaimed: Schema.Literal(false),
  rawActivityRowsClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameAndroidUsageEventsChildRuntimeReplayRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsChildRuntimeReplayGapSchema),
  parentVisibleSummary: AndroidRuntimeReplayLabelSchema,
});

type AppGameAndroidUsageEventsChildRuntimeReplayCandidate = Infer<
  typeof AppGameAndroidUsageEventsChildRuntimeReplayBaseSchema
>;

export const AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema = withParser(
  AppGameAndroidUsageEventsChildRuntimeReplayBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidRuntimeReplayReadModelIsHonest(readModel) ||
        'Expected Android child runtime replay to attach only redacted UsageEvents counters and keep delivery, raw rows, adapter dispatch, and enforcement unclaimed'
    )
  )
);

export type AppGameAndroidUsageEventsChildRuntimeReplayReadModel = Infer<
  typeof AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema
>;

export const decodeAppGameAndroidUsageEventsChildRuntimeReplayReadModel = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema
);

export function createAppGameAndroidUsageEventsChildRuntimeReplayReadModel(input: {
  readonly replayReadModel: AppGameAndroidUsageEventsReplayReadModel;
  readonly generatedAt: AppGameAndroidUsageEventsChildRuntimeReplayReadModel['generatedAt'];
}): AppGameAndroidUsageEventsChildRuntimeReplayReadModel {
  const source = AppGameAndroidUsageEventsReplayReadModelSchema.parse(input.replayReadModel);
  const consumerAttached = source.durableReplayReady && source.runtimeVisibilityReady;

  return decodeAppGameAndroidUsageEventsChildRuntimeReplayReadModel({
    schemaVersion: 'app-game-android-usage-events-child-runtime-replay',
    readModelId: 'android-usage-events-child-runtime-replay-ref',
    generatedAt: input.generatedAt,
    sourceReplayReadModelId: source.readModelId,
    replayState: consumerAttached ? 'consumer-attached-redacted-replay' : 'consumer-not-ready',
    custodyState: 'redacted-runtime-counters-only',
    replayedForegroundEventCount: consumerAttached ? source.foregroundActivityEventCount : 0,
    replayedUsageEventSampleCount: consumerAttached ? source.usageEventsSampleCount : 0,
    childRuntimeReplayConsumerAttached: consumerAttached,
    rawPackageNamesClaimed: false,
    rawActivityRowsClaimed: false,
    childDeviceDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    proofRefs: androidRuntimeReplayRefs(consumerAttached),
    openGaps: androidRuntimeReplayOpenGaps(source),
    parentVisibleSummary: consumerAttached
      ? 'Android UsageEvents replay is attached to a child-runtime replay consumer as redacted counters only; delivery and platform enforcement remain unclaimed.'
      : 'Android UsageEvents replay is not ready for child-runtime replay; delivery and platform enforcement remain unclaimed.',
  });
}

export function summarizeAppGameAndroidUsageEventsChildRuntimeReplayReadModel(
  readModel: AppGameAndroidUsageEventsChildRuntimeReplayReadModel
) {
  return {
    replayState: readModel.replayState,
    childRuntimeReplayConsumerAttached: readModel.childRuntimeReplayConsumerAttached,
    replayedForegroundEventCount: readModel.replayedForegroundEventCount,
    replayedUsageEventSampleCount: readModel.replayedUsageEventSampleCount,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function androidRuntimeReplayRefs(consumerAttached: boolean) {
  const refs = ['android-runtime-visibility-read-model-ref'];
  if (consumerAttached) {
    refs.push('android-usage-events-replay-ref', 'android-usage-events-child-runtime-replay-ref');
  }
  return refs;
}

function androidRuntimeReplayOpenGaps(source: AppGameAndroidUsageEventsReplayReadModel) {
  return source.openGaps.filter(
    (gap) => gap !== 'android-child-runtime-replay-consumer-not-attached'
  );
}

function androidRuntimeReplayReadModelIsHonest(
  readModel: AppGameAndroidUsageEventsChildRuntimeReplayCandidate
): boolean {
  return (
    androidRuntimeReplayStateIsConsistent(readModel) &&
    readModel.custodyState === 'redacted-runtime-counters-only' &&
    !readModel.rawPackageNamesClaimed &&
    !readModel.rawActivityRowsClaimed &&
    !readModel.childDeviceDeliveryClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    readModel.openGaps.includes('android-child-device-delivery-not-proved') &&
    readModel.openGaps.includes('android-platform-enforcement-not-proved')
  );
}

function androidRuntimeReplayStateIsConsistent(
  readModel: AppGameAndroidUsageEventsChildRuntimeReplayCandidate
): boolean {
  if (!readModel.childRuntimeReplayConsumerAttached) {
    return (
      readModel.replayState === 'consumer-not-ready' &&
      readModel.replayedForegroundEventCount === 0 &&
      readModel.replayedUsageEventSampleCount === 0
    );
  }

  return (
    readModel.replayState === 'consumer-attached-redacted-replay' &&
    readModel.replayedForegroundEventCount > 0 &&
    readModel.replayedUsageEventSampleCount > 0 &&
    readModel.proofRefs.includes('android-usage-events-child-runtime-replay-ref') &&
    !readModel.openGaps.includes('android-child-runtime-replay-consumer-not-attached' as never)
  );
}
