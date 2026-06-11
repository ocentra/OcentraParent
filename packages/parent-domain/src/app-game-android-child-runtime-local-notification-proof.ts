import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const LocalNotificationText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidChildRuntimeLocalNotificationProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-notification-proof')
);

export const AppGameAndroidChildRuntimeLocalNotificationChannelStateSchema = withParser(
  Schema.Literal('local-notification-channel-declared')
);

export const AppGameAndroidChildRuntimeLocalNotificationPostStateSchema = withParser(
  Schema.Literal('local-notification-post-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationMarkerStateSchema = withParser(
  Schema.Literal('local-notification-marker-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-child-runtime-local-notification-channel-ref',
    'android-child-runtime-local-notification-post-ref',
    'android-child-runtime-local-notification-marker-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalNotificationGapSchema = withParser(
  Schema.Literal(
    'provider-delivery-not-proved',
    'platform-delivery-outside-package-not-proved',
    'adapter-dispatch-not-proved',
    'platform-enforcement-not-proved'
  )
);

const LocalNotificationLabelSchema = LocalNotificationText.pipe(
  Schema.brand('AppGameAndroidChildRuntimeLocalNotificationProofLabel')
);

const AppGameAndroidChildRuntimeLocalNotificationProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalNotificationProofSchemaVersionSchema,
  packageId: LocalNotificationLabelSchema,
  notificationChannelState: AppGameAndroidChildRuntimeLocalNotificationChannelStateSchema,
  notificationPostState: AppGameAndroidChildRuntimeLocalNotificationPostStateSchema,
  notificationMarkerState: AppGameAndroidChildRuntimeLocalNotificationMarkerStateSchema,
  notificationSeenInSystemUi: Schema.Boolean,
  markerReadbackObserved: Schema.Literal(true),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationGapSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  platformDeliveryOutsidePackageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsStored: Schema.Literal(false),
  parentVisibleSummary: LocalNotificationLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type LocalNotificationCandidate = Infer<typeof AppGameAndroidChildRuntimeLocalNotificationProofBaseSchema>;

export const AppGameAndroidChildRuntimeLocalNotificationProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalNotificationProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localNotificationProofIsHonest(proof) ||
        'Expected Android child runtime local notification proof to require channel, post, marker readback, opaque proof refs, and no provider, external platform, dispatch, enforcement, or raw-row claims'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalNotificationProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalNotificationProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalNotificationProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalNotificationProofSchema
);

export function createAppGameAndroidChildRuntimeLocalNotificationProof(input: {
  readonly notificationSeenInSystemUi: AppGameAndroidChildRuntimeLocalNotificationProof['notificationSeenInSystemUi'];
  readonly checkedAt: AppGameAndroidChildRuntimeLocalNotificationProof['checkedAt'];
}): AppGameAndroidChildRuntimeLocalNotificationProof {
  return decodeAppGameAndroidChildRuntimeLocalNotificationProof({
    schemaVersion: 'app-game-android-child-runtime-local-notification-proof',
    packageId: 'ca.ocentra.parent.agent',
    notificationChannelState: 'local-notification-channel-declared',
    notificationPostState: 'local-notification-post-recorded',
    notificationMarkerState: 'local-notification-marker-recorded',
    notificationSeenInSystemUi: input.notificationSeenInSystemUi,
    markerReadbackObserved: true,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-child-runtime-local-notification-channel-ref',
      'android-child-runtime-local-notification-post-ref',
      'android-child-runtime-local-notification-marker-ref',
    ],
    openGaps: [
      'provider-delivery-not-proved',
      'platform-delivery-outside-package-not-proved',
      'adapter-dispatch-not-proved',
      'platform-enforcement-not-proved',
    ],
    providerDeliveryClaimed: false,
    platformDeliveryOutsidePackageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsStored: false,
    parentVisibleSummary:
      'Android child app posted a package-local app/game notification and marker; provider delivery, external platform delivery, dispatch, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalNotificationProof(
  proof: AppGameAndroidChildRuntimeLocalNotificationProof
) {
  return {
    notificationChannelState: proof.notificationChannelState,
    notificationPostState: proof.notificationPostState,
    notificationMarkerState: proof.notificationMarkerState,
    notificationSeenInSystemUi: proof.notificationSeenInSystemUi,
    openGapCount: proof.openGaps.length,
  } as const;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function localNotificationProofIsHonest(proof: LocalNotificationCandidate): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.notificationChannelState === 'local-notification-channel-declared' &&
    proof.notificationPostState === 'local-notification-post-recorded' &&
    proof.notificationMarkerState === 'local-notification-marker-recorded' &&
    proof.markerReadbackObserved &&
    proof.proofRefs.includes('android-physical-adb-device-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-channel-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-post-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-marker-ref') &&
    proof.openGaps.includes('provider-delivery-not-proved') &&
    proof.openGaps.includes('platform-delivery-outside-package-not-proved') &&
    proof.openGaps.includes('adapter-dispatch-not-proved') &&
    proof.openGaps.includes('platform-enforcement-not-proved')
  );
}
