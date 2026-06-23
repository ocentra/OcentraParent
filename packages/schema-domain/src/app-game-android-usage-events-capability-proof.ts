import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppGameAndroidUsageEventsCommandName,
  AppGameAndroidUsageEventsEventName,
} from './app-game-android-usage-events-contracts';

export const AppGameAndroidUsageEventsCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-capability-proof')
);

export const AppGameAndroidUsageEventsCapabilityStateSchema = withParser(
  Schema.Literal('package-local-scaffold', 'settings-grant-required', 'runtime-grant-not-proved')
);

export const AppGameAndroidUsageEventsReplayConsumerStateSchema = withParser(
  Schema.Literal('parent-domain-boundary-only')
);

export const AppGameAndroidUsageEventsCapabilityCommandSchema = withParser(
  Schema.Literal(
    AppGameAndroidUsageEventsCommandName.CapabilityGet,
    AppGameAndroidUsageEventsCommandName.ReplayBoundaryGet
  )
);

export const AppGameAndroidUsageEventsCapabilityEventSchema = withParser(
  Schema.Literal(
    AppGameAndroidUsageEventsEventName.CapabilityReported,
    AppGameAndroidUsageEventsEventName.ReplayBoundaryReported
  )
);

export const AppGameAndroidUsageEventsCapabilityRefSchema = withParser(
  Schema.Literal('android-usage-events-capability-bridge-ref', 'android-package-local-usage-events-proof-ref')
);

export const AppGameAndroidUsageEventsCapabilityGapSchema = withParser(
  Schema.Literal(
    'android-usage-stats-settings-grant-not-proved',
    'android-usage-events-runtime-collection-not-proved',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const AndroidUsageEventsLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidUsageEventsCapabilityProofLabel');

const AppGameAndroidUsageEventsCapabilityReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsCapabilityProofSchemaVersionSchema,
  packageId: AndroidUsageEventsLabelSchema,
  nativeBridgeClass: AndroidUsageEventsLabelSchema,
  usageEventsBridgeState: AppGameAndroidUsageEventsCapabilityStateSchema,
  permissionState: AppGameAndroidUsageEventsCapabilityStateSchema,
  eventCollectionState: AppGameAndroidUsageEventsCapabilityStateSchema,
  replayConsumerState: AppGameAndroidUsageEventsReplayConsumerStateSchema,
  commands: Schema.Array(AppGameAndroidUsageEventsCapabilityCommandSchema),
  events: Schema.Array(AppGameAndroidUsageEventsCapabilityEventSchema),
  proofRefs: Schema.Array(AppGameAndroidUsageEventsCapabilityRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsCapabilityGapSchema),
  rawUsageEventsStored: Schema.Literal(false),
  packageNamesStored: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: AndroidUsageEventsLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type AppGameAndroidUsageEventsCapabilityCandidate = Infer<
  typeof AppGameAndroidUsageEventsCapabilityReadModelBaseSchema
>;

export const AppGameAndroidUsageEventsCapabilityReadModelSchema = withParser(
  AppGameAndroidUsageEventsCapabilityReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidUsageEventsCapabilityReadModelIsHonest(readModel) ||
        'Expected Android app/game UsageEvents capability proof to stay package-local and keep settings grant, runtime collection, adapter dispatch, enforcement, and child delivery unclaimed'
    )
  )
);

export type AppGameAndroidUsageEventsCapabilityReadModel = Infer<
  typeof AppGameAndroidUsageEventsCapabilityReadModelSchema
>;

export const decodeAppGameAndroidUsageEventsCapabilityReadModel = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsCapabilityReadModelSchema
);

export function createAppGameAndroidUsageEventsCapabilityReadModel(input: {
  readonly checkedAt: AppGameAndroidUsageEventsCapabilityReadModel['checkedAt'];
}): AppGameAndroidUsageEventsCapabilityReadModel {
  return decodeAppGameAndroidUsageEventsCapabilityReadModel({
    schemaVersion: 'app-game-android-usage-events-capability-proof',
    packageId: 'ca.ocentra.parent.agent',
    nativeBridgeClass: 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsCapabilityProof',
    usageEventsBridgeState: 'package-local-scaffold',
    permissionState: 'settings-grant-required',
    eventCollectionState: 'runtime-grant-not-proved',
    replayConsumerState: 'parent-domain-boundary-only',
    commands: [
      AppGameAndroidUsageEventsCommandName.CapabilityGet,
      AppGameAndroidUsageEventsCommandName.ReplayBoundaryGet,
    ],
    events: [
      AppGameAndroidUsageEventsEventName.CapabilityReported,
      AppGameAndroidUsageEventsEventName.ReplayBoundaryReported,
    ],
    proofRefs: ['android-usage-events-capability-bridge-ref', 'android-package-local-usage-events-proof-ref'],
    openGaps: [
      'android-usage-stats-settings-grant-not-proved',
      'android-usage-events-runtime-collection-not-proved',
      'android-child-runtime-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    rawUsageEventsStored: false,
    packageNamesStored: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary:
      'Android package-local app/game UsageEvents bridge is present, but UsageStats settings grant, runtime collection, replay delivery, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidUsageEventsCapabilityReadModel(
  readModel: AppGameAndroidUsageEventsCapabilityReadModel
) {
  return {
    usageEventsBridgeState: readModel.usageEventsBridgeState,
    permissionState: readModel.permissionState,
    eventCollectionState: readModel.eventCollectionState,
    replayConsumerState: readModel.replayConsumerState,
    openGapCount: readModel.openGaps.length,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
  } as const;
}

function androidUsageEventsCapabilityReadModelIsHonest(
  readModel: AppGameAndroidUsageEventsCapabilityCandidate
): boolean {
  return (
    androidUsageEventsCapabilityStateIsHonest(readModel) &&
    androidUsageEventsCapabilityCommandsAndEventsArePresent(readModel) &&
    androidUsageEventsCapabilityProofRefsArePresent(readModel) &&
    androidUsageEventsCapabilityOpenGapsArePresent(readModel)
  );
}

function androidUsageEventsCapabilityStateIsHonest(readModel: AppGameAndroidUsageEventsCapabilityCandidate): boolean {
  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.nativeBridgeClass === 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsCapabilityProof' &&
    readModel.usageEventsBridgeState === 'package-local-scaffold' &&
    readModel.permissionState === 'settings-grant-required' &&
    readModel.eventCollectionState === 'runtime-grant-not-proved' &&
    readModel.replayConsumerState === 'parent-domain-boundary-only'
  );
}

function androidUsageEventsCapabilityCommandsAndEventsArePresent(
  readModel: AppGameAndroidUsageEventsCapabilityCandidate
): boolean {
  return (
    includesAll(readModel.commands, [
      AppGameAndroidUsageEventsCommandName.CapabilityGet,
      AppGameAndroidUsageEventsCommandName.ReplayBoundaryGet,
    ] as const) &&
    includesAll(readModel.events, [
      AppGameAndroidUsageEventsEventName.CapabilityReported,
      AppGameAndroidUsageEventsEventName.ReplayBoundaryReported,
    ] as const)
  );
}

function androidUsageEventsCapabilityProofRefsArePresent(
  readModel: AppGameAndroidUsageEventsCapabilityCandidate
): boolean {
  return includesAll(readModel.proofRefs, [
    'android-usage-events-capability-bridge-ref',
    'android-package-local-usage-events-proof-ref',
  ] as const);
}

function androidUsageEventsCapabilityOpenGapsArePresent(
  readModel: AppGameAndroidUsageEventsCapabilityCandidate
): boolean {
  return includesAll(readModel.openGaps, [
    'android-usage-stats-settings-grant-not-proved',
    'android-usage-events-runtime-collection-not-proved',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved',
  ] as const);
}

function includesAll<T extends string>(values: readonly T[], required: readonly T[]): boolean {
  return required.every((value) => values.includes(value));
}
