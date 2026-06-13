import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { type AppGameAndroidPhysicalDeviceProof } from './app-game-android-physical-device-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const AppGameAndroidUsageEventsReplaySchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-replay')
);

export const AppGameAndroidUsageEventsReplayStateSchema = withParser(
  Schema.Literal('durable-replay-ready', 'replay-not-ready')
);

export const AppGameAndroidUsageEventsReplaySourceStateSchema = withParser(
  Schema.Literal('foreground-events-observed', 'foreground-events-unavailable')
);

export const AppGameAndroidUsageEventsReplayCustodyStateSchema = withParser(Schema.Literal('redacted-counts-only'));

export const AppGameAndroidUsageEventsReplayRefSchema = withParser(
  Schema.Literal(
    'android-usage-events-replay-ref',
    'android-physical-usage-events-dump-ref',
    'android-runtime-visibility-read-model-ref'
  )
);

export const AppGameAndroidUsageEventsReplayGapSchema = withParser(
  Schema.Literal(
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-child-runtime-replay-consumer-not-attached',
    'android-hide-suspend-not-proved',
    'android-platform-enforcement-not-proved',
    'android-child-device-delivery-not-proved'
  )
);

const AndroidUsageReplayLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidUsageEventsReplayLabel');

const AndroidUsageReplayCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGameAndroidUsageEventsReplayReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsReplaySchemaVersionSchema,
  readModelId: AndroidUsageReplayLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofId: AndroidUsageReplayLabelSchema,
  replayState: AppGameAndroidUsageEventsReplayStateSchema,
  sourceState: AppGameAndroidUsageEventsReplaySourceStateSchema,
  custodyState: AppGameAndroidUsageEventsReplayCustodyStateSchema,
  usageEventsSampleCount: AndroidUsageReplayCountSchema,
  foregroundActivityEventCount: AndroidUsageReplayCountSchema,
  runtimeVisibilityReady: Schema.Boolean,
  durableReplayReady: Schema.Boolean,
  rawPackageNamesClaimed: Schema.Boolean,
  rawActivityRowsClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidUsageEventsReplayRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsReplayGapSchema),
  parentVisibleSummary: AndroidUsageReplayLabelSchema,
});

type AppGameAndroidUsageEventsReplayReadModelCandidate = Infer<
  typeof AppGameAndroidUsageEventsReplayReadModelBaseSchema
>;

export const AppGameAndroidUsageEventsReplayReadModelSchema = withParser(
  AppGameAndroidUsageEventsReplayReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidUsageEventsReplayReadModelIsHonest(readModel) ||
        'Expected Android UsageEvents replay readiness to use redacted foreground counts and keep raw rows, adapter dispatch, enforcement, and child delivery unclaimed'
    )
  )
);

export type AppGameAndroidUsageEventsReplayReadModel = Infer<typeof AppGameAndroidUsageEventsReplayReadModelSchema>;

export const decodeAppGameAndroidUsageEventsReplayReadModel = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsReplayReadModelSchema
);

export function createAppGameAndroidUsageEventsReplayReadModel(input: {
  readonly androidProof: AppGameAndroidPhysicalDeviceProof;
  readonly generatedAt: AppGameAndroidUsageEventsReplayReadModel['generatedAt'];
}): AppGameAndroidUsageEventsReplayReadModel {
  const runtimeVisibilityReady =
    input.androidProof.usageEventsDumpState === 'usage-events-dump-observed' &&
    input.androidProof.foregroundEvidenceObserved &&
    input.androidProof.usageEventsPackageNamesRedacted;
  const readModel = {
    schemaVersion: 'app-game-android-usage-events-replay',
    readModelId: 'android-runtime-visibility-read-model-ref',
    generatedAt: input.generatedAt,
    sourceProofId: input.androidProof.proofId,
    replayState: runtimeVisibilityReady ? 'durable-replay-ready' : 'replay-not-ready',
    sourceState: runtimeVisibilityReady ? 'foreground-events-observed' : 'foreground-events-unavailable',
    custodyState: 'redacted-counts-only',
    usageEventsSampleCount: input.androidProof.usageEventsSampleCount,
    foregroundActivityEventCount: input.androidProof.foregroundActivityEventCount,
    runtimeVisibilityReady,
    durableReplayReady: runtimeVisibilityReady,
    rawPackageNamesClaimed: false,
    rawActivityRowsClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: androidUsageEventsReplayRefs(runtimeVisibilityReady),
    openGaps: androidUsageEventsReplayOpenGaps(input.androidProof),
    parentVisibleSummary:
      'Android UsageEvents foreground samples are durable-replay ready as redacted counts only; child runtime delivery and platform enforcement remain unclaimed.',
  };

  return decodeAppGameAndroidUsageEventsReplayReadModel(readModel);
}

export function summarizeAppGameAndroidUsageEventsReplayReadModel(readModel: AppGameAndroidUsageEventsReplayReadModel) {
  return {
    replayState: readModel.replayState,
    runtimeVisibilityReady: readModel.runtimeVisibilityReady,
    durableReplayReady: readModel.durableReplayReady,
    usageEventsSampleCount: readModel.usageEventsSampleCount,
    foregroundActivityEventCount: readModel.foregroundActivityEventCount,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function androidUsageEventsReplayRefs(runtimeVisibilityReady: boolean) {
  const refs = ['android-runtime-visibility-read-model-ref'];
  if (runtimeVisibilityReady) {
    refs.push('android-usage-events-replay-ref', 'android-physical-usage-events-dump-ref');
  }
  return refs;
}

function androidUsageEventsReplayOpenGaps(proof: AppGameAndroidPhysicalDeviceProof) {
  const gaps = [
    'android-child-runtime-replay-consumer-not-attached',
    'android-hide-suspend-not-proved',
    'android-platform-enforcement-not-proved',
    'android-child-device-delivery-not-proved',
  ];
  if (proof.deviceOwnerState === 'not-device-owner') {
    gaps.push('android-device-owner-not-proved');
  }
  if (proof.profileOwnerState === 'not-profile-owner') {
    gaps.push('android-profile-owner-not-proved');
  }
  return gaps;
}

function androidUsageEventsReplayReadModelIsHonest(
  readModel: AppGameAndroidUsageEventsReplayReadModelCandidate
): boolean {
  return (
    androidUsageEventsReplayStateIsConsistent(readModel) &&
    readModel.custodyState === 'redacted-counts-only' &&
    !readModel.rawPackageNamesClaimed &&
    !readModel.rawActivityRowsClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed &&
    readModel.openGaps.includes('android-child-device-delivery-not-proved') &&
    readModel.openGaps.includes('android-platform-enforcement-not-proved')
  );
}

function androidUsageEventsReplayStateIsConsistent(
  readModel: AppGameAndroidUsageEventsReplayReadModelCandidate
): boolean {
  if (!readModel.runtimeVisibilityReady || !readModel.durableReplayReady) {
    return readModel.replayState === 'replay-not-ready';
  }

  return (
    readModel.replayState === 'durable-replay-ready' &&
    readModel.sourceState === 'foreground-events-observed' &&
    readModel.usageEventsSampleCount > 0 &&
    readModel.foregroundActivityEventCount > 0 &&
    readModel.proofRefs.includes('android-usage-events-replay-ref') &&
    readModel.proofRefs.includes('android-physical-usage-events-dump-ref')
  );
}

