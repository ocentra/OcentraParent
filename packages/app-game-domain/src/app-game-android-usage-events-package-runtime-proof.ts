import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const PackageRuntimeText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidUsageEventsPackageRuntimeProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-package-runtime-proof')
);

export const AppGameAndroidUsageEventsPackageRuntimeStateSchema = withParser(
  Schema.Literal('package-installed', 'package-launch-observed', 'package-ui-not-observed')
);

export const AppGameAndroidUsageEventsPackageRuntimePermissionStateSchema = withParser(
  Schema.Literal('usage-stats-granted', 'settings-grant-required', 'permission-check-unavailable')
);

export const AppGameAndroidUsageEventsPackageRuntimeSampleStateSchema = withParser(
  Schema.Literal('sample-permission-required', 'sample-observed', 'sample-empty', 'sample-unavailable')
);

export const AppGameAndroidUsageEventsPackageRuntimeProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-package-install-ref',
    'android-package-launch-ui-ref',
    'android-usage-stats-appops-preflight-ref'
  )
);

export const AppGameAndroidUsageEventsPackageRuntimeGapSchema = withParser(
  Schema.Literal(
    'android-usage-stats-settings-grant-not-proved',
    'android-usage-events-live-package-sample-not-observed',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const PackageRuntimeLabelSchema = PackageRuntimeText.pipe(
  Schema.brand('AppGameAndroidUsageEventsPackageRuntimeProofLabel')
);

const AppGameAndroidUsageEventsPackageRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsPackageRuntimeProofSchemaVersionSchema,
  packageId: PackageRuntimeLabelSchema,
  installedState: AppGameAndroidUsageEventsPackageRuntimeStateSchema,
  launchState: AppGameAndroidUsageEventsPackageRuntimeStateSchema,
  permissionCheckState: AppGameAndroidUsageEventsPackageRuntimePermissionStateSchema,
  sampleState: AppGameAndroidUsageEventsPackageRuntimeSampleStateSchema,
  appOpsObserved: Schema.Boolean,
  uiStateObserved: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidUsageEventsPackageRuntimeProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsPackageRuntimeGapSchema),
  rawDeviceSerialStored: Schema.Literal(false),
  rawPackageNamesStored: Schema.Literal(false),
  rawUsageEventsStored: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: PackageRuntimeLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type PackageRuntimeCandidate = Infer<typeof AppGameAndroidUsageEventsPackageRuntimeProofBaseSchema>;

export const AppGameAndroidUsageEventsPackageRuntimeProofSchema = withParser(
  AppGameAndroidUsageEventsPackageRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        packageRuntimeProofIsHonest(readModel) ||
        'Expected Android package runtime proof to require install, UI, AppOps refs, and keep raw data, dispatch, enforcement, and child delivery unclaimed'
    )
  )
);

export type AppGameAndroidUsageEventsPackageRuntimeProof = Infer<
  typeof AppGameAndroidUsageEventsPackageRuntimeProofSchema
>;

export const decodeAppGameAndroidUsageEventsPackageRuntimeProof = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsPackageRuntimeProofSchema
);

export function createAppGameAndroidUsageEventsPackageRuntimeProof(input: {
  readonly permissionCheckState: AppGameAndroidUsageEventsPackageRuntimeProof['permissionCheckState'];
  readonly sampleState: AppGameAndroidUsageEventsPackageRuntimeProof['sampleState'];
  readonly uiStateObserved: AppGameAndroidUsageEventsPackageRuntimeProof['uiStateObserved'];
  readonly appOpsObserved: AppGameAndroidUsageEventsPackageRuntimeProof['appOpsObserved'];
  readonly checkedAt: AppGameAndroidUsageEventsPackageRuntimeProof['checkedAt'];
}): AppGameAndroidUsageEventsPackageRuntimeProof {
  return decodeAppGameAndroidUsageEventsPackageRuntimeProof({
    schemaVersion: 'app-game-android-usage-events-package-runtime-proof',
    packageId: 'ca.ocentra.parent.agent',
    installedState: 'package-installed',
    launchState: input.uiStateObserved ? 'package-launch-observed' : 'package-ui-not-observed',
    permissionCheckState: input.permissionCheckState,
    sampleState: input.sampleState,
    appOpsObserved: input.appOpsObserved,
    uiStateObserved: input.uiStateObserved,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-package-install-ref',
      'android-package-launch-ui-ref',
      'android-usage-stats-appops-preflight-ref',
    ],
    openGaps: packageRuntimeOpenGaps(input.permissionCheckState, input.sampleState),
    rawDeviceSerialStored: false,
    rawPackageNamesStored: false,
    rawUsageEventsStored: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary: packageRuntimeSummary(input.permissionCheckState, input.sampleState),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidUsageEventsPackageRuntimeProof(
  readModel: AppGameAndroidUsageEventsPackageRuntimeProof
) {
  return {
    installedState: readModel.installedState,
    launchState: readModel.launchState,
    permissionCheckState: readModel.permissionCheckState,
    sampleState: readModel.sampleState,
    appOpsObserved: readModel.appOpsObserved,
    uiStateObserved: readModel.uiStateObserved,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function packageRuntimeOpenGaps(
  permissionState: PackageRuntimeCandidate['permissionCheckState'],
  sampleState: PackageRuntimeCandidate['sampleState']
) {
  const gaps = ['android-child-runtime-delivery-not-proved', 'android-platform-enforcement-not-proved'];
  if (sampleState !== 'sample-observed') {
    gaps.unshift('android-usage-events-live-package-sample-not-observed');
  }
  if (permissionState !== 'usage-stats-granted') {
    gaps.unshift('android-usage-stats-settings-grant-not-proved');
  }
  return gaps;
}

function packageRuntimeSummary(
  permissionState: PackageRuntimeCandidate['permissionCheckState'],
  sampleState: PackageRuntimeCandidate['sampleState']
) {
  if (permissionState === 'usage-stats-granted' && sampleState === 'sample-observed') {
    return 'Android package install and launch are observed with count-only UsageEvents sample visibility; delivery and enforcement remain unclaimed.';
  }

  return 'Android package install and launch are observed, but UsageStats grant or live package sample proof is still missing.';
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function packageRuntimeProofIsHonest(readModel: PackageRuntimeCandidate): boolean {
  const grantAndSampleConsistent =
    readModel.permissionCheckState === 'usage-stats-granted' ||
    readModel.sampleState === 'sample-permission-required' ||
    readModel.sampleState === 'sample-unavailable';

  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.installedState === 'package-installed' &&
    readModel.launchState === 'package-launch-observed' &&
    readModel.uiStateObserved &&
    readModel.appOpsObserved &&
    grantAndSampleConsistent &&
    readModel.proofRefs.includes('android-physical-adb-device-ref') &&
    readModel.proofRefs.includes('android-package-install-ref') &&
    readModel.proofRefs.includes('android-package-launch-ui-ref') &&
    readModel.proofRefs.includes('android-usage-stats-appops-preflight-ref') &&
    readModel.openGaps.includes('android-child-runtime-delivery-not-proved') &&
    readModel.openGaps.includes('android-platform-enforcement-not-proved')
  );
}
