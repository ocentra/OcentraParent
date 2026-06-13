import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  AppGameAndroidUsageEventsCommandName,
  AppGameAndroidUsageEventsEventName,
} from './app-game-android-usage-events-contracts';

export const AppGameAndroidUsageEventsRuntimePreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-runtime-preflight')
);

export const AppGameAndroidUsageEventsRuntimePermissionStateSchema = withParser(
  Schema.Literal('usage-stats-granted', 'settings-grant-required', 'permission-check-unavailable')
);

export const AppGameAndroidUsageEventsRuntimeCollectionStateSchema = withParser(
  Schema.Literal('collection-ready-for-proof', 'collection-blocked-before-runtime-proof')
);

export const AppGameAndroidUsageEventsRuntimeServiceStateSchema = withParser(
  Schema.Literal('service-visible', 'service-unavailable')
);

export const AppGameAndroidUsageEventsRuntimePreflightCommandSchema = withParser(
  Schema.Literal(AppGameAndroidUsageEventsCommandName.RuntimePreflightGet)
);

export const AppGameAndroidUsageEventsRuntimePreflightEventSchema = withParser(
  Schema.Literal(AppGameAndroidUsageEventsEventName.RuntimePreflightReported)
);

export const AppGameAndroidUsageEventsRuntimePreflightRefSchema = withParser(
  Schema.Literal('android-usage-events-runtime-preflight-ref', 'android-usage-stats-appops-preflight-ref')
);

export const AppGameAndroidUsageEventsRuntimePreflightGapSchema = withParser(
  Schema.Literal(
    'android-usage-events-runtime-sample-not-proved',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const RuntimePreflightLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidUsageEventsRuntimePreflightLabel');

const AppGameAndroidUsageEventsRuntimePreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsRuntimePreflightSchemaVersionSchema,
  packageId: RuntimePreflightLabelSchema,
  nativeBridgeClass: RuntimePreflightLabelSchema,
  permissionCheckState: AppGameAndroidUsageEventsRuntimePermissionStateSchema,
  runtimeCollectionState: AppGameAndroidUsageEventsRuntimeCollectionStateSchema,
  usageStatsServiceState: AppGameAndroidUsageEventsRuntimeServiceStateSchema,
  commands: Schema.Array(AppGameAndroidUsageEventsRuntimePreflightCommandSchema),
  events: Schema.Array(AppGameAndroidUsageEventsRuntimePreflightEventSchema),
  proofRefs: Schema.Array(AppGameAndroidUsageEventsRuntimePreflightRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsRuntimePreflightGapSchema),
  rawUsageEventsStored: Schema.Literal(false),
  packageNamesStored: Schema.Literal(false),
  runtimeCollectionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: RuntimePreflightLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type RuntimePreflightCandidate = Infer<typeof AppGameAndroidUsageEventsRuntimePreflightReadModelBaseSchema>;

export const AppGameAndroidUsageEventsRuntimePreflightReadModelSchema = withParser(
  AppGameAndroidUsageEventsRuntimePreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidUsageEventsRuntimePreflightIsHonest(readModel) ||
        'Expected Android UsageEvents runtime preflight to expose permission/service readiness without claiming collection, raw storage, dispatch, enforcement, or child delivery'
    )
  )
);

export type AppGameAndroidUsageEventsRuntimePreflightReadModel = Infer<
  typeof AppGameAndroidUsageEventsRuntimePreflightReadModelSchema
>;

export const decodeAppGameAndroidUsageEventsRuntimePreflightReadModel = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsRuntimePreflightReadModelSchema
);

export function createAppGameAndroidUsageEventsRuntimePreflightReadModel(input: {
  readonly permissionCheckState: AppGameAndroidUsageEventsRuntimePreflightReadModel['permissionCheckState'];
  readonly usageStatsServiceState: AppGameAndroidUsageEventsRuntimePreflightReadModel['usageStatsServiceState'];
  readonly checkedAt: AppGameAndroidUsageEventsRuntimePreflightReadModel['checkedAt'];
}): AppGameAndroidUsageEventsRuntimePreflightReadModel {
  const collectionState =
    input.permissionCheckState === 'usage-stats-granted'
      ? 'collection-ready-for-proof'
      : 'collection-blocked-before-runtime-proof';

  return decodeAppGameAndroidUsageEventsRuntimePreflightReadModel({
    schemaVersion: 'app-game-android-usage-events-runtime-preflight',
    packageId: 'ca.ocentra.parent.agent',
    nativeBridgeClass: 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight',
    permissionCheckState: input.permissionCheckState,
    runtimeCollectionState: collectionState,
    usageStatsServiceState: input.usageStatsServiceState,
    commands: [AppGameAndroidUsageEventsCommandName.RuntimePreflightGet],
    events: [AppGameAndroidUsageEventsEventName.RuntimePreflightReported],
    proofRefs: ['android-usage-events-runtime-preflight-ref', 'android-usage-stats-appops-preflight-ref'],
    openGaps: [
      'android-usage-events-runtime-sample-not-proved',
      'android-child-runtime-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    rawUsageEventsStored: false,
    packageNamesStored: false,
    runtimeCollectionClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary: runtimePreflightSummary(input.permissionCheckState, collectionState),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidUsageEventsRuntimePreflightReadModel(
  readModel: AppGameAndroidUsageEventsRuntimePreflightReadModel
) {
  return {
    permissionCheckState: readModel.permissionCheckState,
    runtimeCollectionState: readModel.runtimeCollectionState,
    usageStatsServiceState: readModel.usageStatsServiceState,
    runtimeCollectionClaimed: readModel.runtimeCollectionClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function runtimePreflightSummary(
  permissionState: RuntimePreflightCandidate['permissionCheckState'],
  collectionState: RuntimePreflightCandidate['runtimeCollectionState']
) {
  if (permissionState === 'usage-stats-granted' && collectionState === 'collection-ready-for-proof') {
    return 'Android package can see UsageStats permission state as granted, but runtime UsageEvents samples still require proof before collection or delivery is claimed.';
  }

  return 'Android package UsageEvents runtime remains blocked until UsageStats settings grant and runtime sample proof are attached.';
}

function androidUsageEventsRuntimePreflightIsHonest(readModel: RuntimePreflightCandidate): boolean {
  const collectionStateMatchesPermission =
    readModel.permissionCheckState === 'usage-stats-granted'
      ? readModel.runtimeCollectionState === 'collection-ready-for-proof'
      : readModel.runtimeCollectionState === 'collection-blocked-before-runtime-proof';

  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.nativeBridgeClass === 'ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight' &&
    collectionStateMatchesPermission &&
    readModel.commands.includes(AppGameAndroidUsageEventsCommandName.RuntimePreflightGet) &&
    readModel.events.includes(AppGameAndroidUsageEventsEventName.RuntimePreflightReported) &&
    readModel.proofRefs.includes('android-usage-events-runtime-preflight-ref') &&
    readModel.proofRefs.includes('android-usage-stats-appops-preflight-ref') &&
    readModel.openGaps.includes('android-usage-events-runtime-sample-not-proved') &&
    readModel.openGaps.includes('android-child-runtime-delivery-not-proved') &&
    readModel.openGaps.includes('android-platform-enforcement-not-proved')
  );
}

