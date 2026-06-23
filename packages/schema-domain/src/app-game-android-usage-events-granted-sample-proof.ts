import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidUsageEventsGrantedSampleProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-usage-events-granted-sample-proof')
);

export const AppGameAndroidUsageEventsGrantedSamplePermissionStateSchema = withParser(
  Schema.Literal('usage-stats-granted')
);

export const AppGameAndroidUsageEventsGrantedSampleStateSchema = withParser(Schema.Literal('sample-observed'));

export const AppGameAndroidUsageEventsGrantedSampleProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-usage-stats-appops-grant-ref',
    'android-package-launch-ui-ref',
    'android-usage-events-count-only-sample-ref'
  )
);

export const AppGameAndroidUsageEventsGrantedSampleGapSchema = withParser(
  Schema.Literal(
    'android-device-owner-authority-not-proved',
    'android-play-policy-not-proved',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const GrantedSampleLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidUsageEventsGrantedSampleProofLabel');

const AppGameAndroidUsageEventsGrantedSampleProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidUsageEventsGrantedSampleProofSchemaVersionSchema,
  packageId: GrantedSampleLabelSchema,
  permissionCheckState: AppGameAndroidUsageEventsGrantedSamplePermissionStateSchema,
  sampleState: AppGameAndroidUsageEventsGrantedSampleStateSchema,
  appOpsGrantObserved: Schema.Literal(true),
  uiStateObserved: Schema.Literal(true),
  sampleEventCount: Schema.Number.pipe(Schema.int(), Schema.greaterThan(0)),
  foregroundEventCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  proofRefs: Schema.Array(AppGameAndroidUsageEventsGrantedSampleProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidUsageEventsGrantedSampleGapSchema),
  rawDeviceSerialStored: Schema.Literal(false),
  rawPackageNamesStored: Schema.Literal(false),
  rawUsageEventsStored: Schema.Literal(false),
  rawActivityRowsStored: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: GrantedSampleLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type GrantedSampleCandidate = Infer<typeof AppGameAndroidUsageEventsGrantedSampleProofBaseSchema>;

export const AppGameAndroidUsageEventsGrantedSampleProofSchema = withParser(
  AppGameAndroidUsageEventsGrantedSampleProofBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        grantedSampleProofIsHonest(readModel) ||
        'Expected Android UsageEvents granted sample proof to require granted AppOps, UI sample counts, opaque proof refs, and no raw-event, dispatch, delivery, or enforcement claims'
    )
  )
);

export type AppGameAndroidUsageEventsGrantedSampleProof = Infer<
  typeof AppGameAndroidUsageEventsGrantedSampleProofSchema
>;

export const decodeAppGameAndroidUsageEventsGrantedSampleProof = Schema.decodeUnknownSync(
  AppGameAndroidUsageEventsGrantedSampleProofSchema
);

export function createAppGameAndroidUsageEventsGrantedSampleProof(input: {
  readonly sampleEventCount: AppGameAndroidUsageEventsGrantedSampleProof['sampleEventCount'];
  readonly foregroundEventCount: AppGameAndroidUsageEventsGrantedSampleProof['foregroundEventCount'];
  readonly checkedAt: AppGameAndroidUsageEventsGrantedSampleProof['checkedAt'];
}): AppGameAndroidUsageEventsGrantedSampleProof {
  return decodeAppGameAndroidUsageEventsGrantedSampleProof({
    schemaVersion: 'app-game-android-usage-events-granted-sample-proof',
    packageId: 'ca.ocentra.parent.agent',
    permissionCheckState: 'usage-stats-granted',
    sampleState: 'sample-observed',
    appOpsGrantObserved: true,
    uiStateObserved: true,
    sampleEventCount: input.sampleEventCount,
    foregroundEventCount: input.foregroundEventCount,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-usage-stats-appops-grant-ref',
      'android-package-launch-ui-ref',
      'android-usage-events-count-only-sample-ref',
    ],
    openGaps: [
      'android-device-owner-authority-not-proved',
      'android-play-policy-not-proved',
      'android-child-runtime-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    rawDeviceSerialStored: false,
    rawPackageNamesStored: false,
    rawUsageEventsStored: false,
    rawActivityRowsStored: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary:
      'Android UsageStats is granted and the package reports a count-only UsageEvents sample; raw events, package names, delivery, dispatch, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidUsageEventsGrantedSampleProof(
  readModel: AppGameAndroidUsageEventsGrantedSampleProof
) {
  return {
    permissionCheckState: readModel.permissionCheckState,
    sampleState: readModel.sampleState,
    sampleEventCount: readModel.sampleEventCount,
    foregroundEventCount: readModel.foregroundEventCount,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function grantedSampleProofIsHonest(readModel: GrantedSampleCandidate): boolean {
  return (
    grantedSampleStateIsHonest(readModel) &&
    grantedSampleProofRefsArePresent(readModel) &&
    grantedSampleOpenGapsArePresent(readModel)
  );
}

function grantedSampleStateIsHonest(readModel: GrantedSampleCandidate): boolean {
  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.permissionCheckState === 'usage-stats-granted' &&
    readModel.sampleState === 'sample-observed' &&
    readModel.sampleEventCount > 0 &&
    readModel.foregroundEventCount >= 0
  );
}

function grantedSampleProofRefsArePresent(readModel: GrantedSampleCandidate): boolean {
  return includesAll(readModel.proofRefs, [
    'android-physical-adb-device-ref',
    'android-usage-stats-appops-grant-ref',
    'android-package-launch-ui-ref',
    'android-usage-events-count-only-sample-ref',
  ] as const);
}

function grantedSampleOpenGapsArePresent(readModel: GrantedSampleCandidate): boolean {
  return includesAll(readModel.openGaps, [
    'android-device-owner-authority-not-proved',
    'android-play-policy-not-proved',
    'android-child-runtime-delivery-not-proved',
    'android-platform-enforcement-not-proved',
  ] as const);
}

function includesAll<T extends string>(values: readonly T[], required: readonly T[]): boolean {
  return required.every((value) => values.includes(value));
}
