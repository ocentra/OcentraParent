import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const LocalNotificationActionText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidChildRuntimeLocalNotificationActionProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-notification-action-proof')
);

export const AppGameAndroidChildRuntimeLocalNotificationActionChannelStateSchema = withParser(
  Schema.Literal('local-notification-channel-declared')
);

export const AppGameAndroidChildRuntimeLocalNotificationActionPostStateSchema = withParser(
  Schema.Literal('local-notification-post-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationActionStateSchema = withParser(
  Schema.Literal('local-notification-request-action-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationActionMarkerStateSchema = withParser(
  Schema.Literal('local-notification-request-action-marker-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationActionProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-child-runtime-local-notification-channel-ref',
    'android-child-runtime-local-notification-post-ref',
    'android-child-runtime-local-notification-request-action-ref',
    'android-child-runtime-local-notification-request-action-marker-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalNotificationActionGapSchema = withParser(
  Schema.Literal(
    'provider-delivery-not-proved',
    'platform-delivery-outside-package-not-proved',
    'service-request-ingestion-not-proved',
    'parent-approval-round-trip-not-proved',
    'adapter-dispatch-not-proved',
    'platform-enforcement-not-proved'
  )
);

const LocalNotificationActionLabelSchema = LocalNotificationActionText.pipe(
  Schema.brand('AppGameAndroidChildRuntimeLocalNotificationActionProofLabel')
);

const AppGameAndroidChildRuntimeLocalNotificationActionProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalNotificationActionProofSchemaVersionSchema,
  packageId: LocalNotificationActionLabelSchema,
  notificationChannelState: AppGameAndroidChildRuntimeLocalNotificationActionChannelStateSchema,
  notificationPostState: AppGameAndroidChildRuntimeLocalNotificationActionPostStateSchema,
  notificationRequestActionState: AppGameAndroidChildRuntimeLocalNotificationActionStateSchema,
  notificationRequestActionMarkerState: AppGameAndroidChildRuntimeLocalNotificationActionMarkerStateSchema,
  notificationSeenInSystemUi: Schema.Boolean,
  requestActionReadbackObserved: Schema.Literal(true),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationActionProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationActionGapSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  platformDeliveryOutsidePackageClaimed: Schema.Literal(false),
  serviceRequestIngestionClaimed: Schema.Literal(false),
  parentApprovalRoundTripClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsStored: Schema.Literal(false),
  parentVisibleSummary: LocalNotificationActionLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type LocalNotificationActionCandidate = Infer<typeof AppGameAndroidChildRuntimeLocalNotificationActionProofBaseSchema>;

export const AppGameAndroidChildRuntimeLocalNotificationActionProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalNotificationActionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localNotificationActionProofIsHonest(proof) ||
        'Expected Android child runtime local notification action proof to require channel, post, package-local action marker readback, opaque proof refs, and no provider, service ingestion, approval round trip, dispatch, enforcement, or raw-row claims'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalNotificationActionProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalNotificationActionProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalNotificationActionProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalNotificationActionProofSchema
);

export function createAppGameAndroidChildRuntimeLocalNotificationActionProof(input: {
  readonly notificationSeenInSystemUi: AppGameAndroidChildRuntimeLocalNotificationActionProof['notificationSeenInSystemUi'];
  readonly checkedAt: AppGameAndroidChildRuntimeLocalNotificationActionProof['checkedAt'];
}): AppGameAndroidChildRuntimeLocalNotificationActionProof {
  return decodeAppGameAndroidChildRuntimeLocalNotificationActionProof({
    schemaVersion: 'app-game-android-child-runtime-local-notification-action-proof',
    packageId: 'ca.ocentra.parent.agent',
    notificationChannelState: 'local-notification-channel-declared',
    notificationPostState: 'local-notification-post-recorded',
    notificationRequestActionState: 'local-notification-request-action-recorded',
    notificationRequestActionMarkerState: 'local-notification-request-action-marker-recorded',
    notificationSeenInSystemUi: input.notificationSeenInSystemUi,
    requestActionReadbackObserved: true,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-child-runtime-local-notification-channel-ref',
      'android-child-runtime-local-notification-post-ref',
      'android-child-runtime-local-notification-request-action-ref',
      'android-child-runtime-local-notification-request-action-marker-ref',
    ],
    openGaps: [
      'provider-delivery-not-proved',
      'platform-delivery-outside-package-not-proved',
      'service-request-ingestion-not-proved',
      'parent-approval-round-trip-not-proved',
      'adapter-dispatch-not-proved',
      'platform-enforcement-not-proved',
    ],
    providerDeliveryClaimed: false,
    platformDeliveryOutsidePackageClaimed: false,
    serviceRequestIngestionClaimed: false,
    parentApprovalRoundTripClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsStored: false,
    parentVisibleSummary:
      'Android child app notification exposes a package-local ask-parent action and marker; service ingestion, approval round trip, provider delivery, external platform delivery, dispatch, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalNotificationActionProof(
  proof: AppGameAndroidChildRuntimeLocalNotificationActionProof
) {
  return {
    notificationChannelState: proof.notificationChannelState,
    notificationPostState: proof.notificationPostState,
    notificationRequestActionState: proof.notificationRequestActionState,
    notificationRequestActionMarkerState: proof.notificationRequestActionMarkerState,
    notificationSeenInSystemUi: proof.notificationSeenInSystemUi,
    openGapCount: proof.openGaps.length,
  } as const;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function localNotificationActionProofIsHonest(proof: LocalNotificationActionCandidate): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.notificationChannelState === 'local-notification-channel-declared' &&
    proof.notificationPostState === 'local-notification-post-recorded' &&
    proof.notificationRequestActionState === 'local-notification-request-action-recorded' &&
    proof.notificationRequestActionMarkerState === 'local-notification-request-action-marker-recorded' &&
    proof.requestActionReadbackObserved &&
    proof.proofRefs.includes('android-physical-adb-device-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-channel-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-post-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-action-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-action-marker-ref') &&
    proof.openGaps.includes('provider-delivery-not-proved') &&
    proof.openGaps.includes('platform-delivery-outside-package-not-proved') &&
    proof.openGaps.includes('service-request-ingestion-not-proved') &&
    proof.openGaps.includes('parent-approval-round-trip-not-proved') &&
    proof.openGaps.includes('adapter-dispatch-not-proved') &&
    proof.openGaps.includes('platform-enforcement-not-proved')
  );
}
