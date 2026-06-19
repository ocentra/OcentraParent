import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-notification-request-queue-proof')
);

export const AppGameAndroidChildRuntimeLocalNotificationRequestQueueStateSchema = withParser(
  Schema.Literal('local-notification-request-queue-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationRequestReadbackStateSchema = withParser(
  Schema.Literal('local-notification-request-readback-observed')
);

export const AppGameAndroidChildRuntimeLocalNotificationRequestDrainStateSchema = withParser(
  Schema.Literal('local-notification-request-drain-recorded')
);

export const AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-child-runtime-local-notification-request-action-ref',
    'android-child-runtime-local-notification-request-queue-ref',
    'android-child-runtime-local-notification-request-readback-ref',
    'android-child-runtime-local-notification-request-drain-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalNotificationRequestQueueGapSchema = withParser(
  Schema.Literal(
    'service-request-ingestion-not-proved',
    'parent-approval-round-trip-not-proved',
    'provider-delivery-not-proved',
    'platform-delivery-outside-package-not-proved',
    'adapter-dispatch-not-proved',
    'platform-enforcement-not-proved'
  )
);

const LocalRequestQueueLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofLabel');

const AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchemaVersionSchema,
  packageId: LocalRequestQueueLabelSchema,
  notificationRequestQueueState: AppGameAndroidChildRuntimeLocalNotificationRequestQueueStateSchema,
  notificationRequestReadbackState: AppGameAndroidChildRuntimeLocalNotificationRequestReadbackStateSchema,
  notificationRequestDrainState: AppGameAndroidChildRuntimeLocalNotificationRequestDrainStateSchema,
  requestQueueReadbackObserved: Schema.Literal(true),
  requestDrainReadbackObserved: Schema.Literal(true),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalNotificationRequestQueueGapSchema),
  serviceRequestIngestionClaimed: Schema.Literal(false),
  parentApprovalRoundTripClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  platformDeliveryOutsidePackageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsStored: Schema.Literal(false),
  parentVisibleSummary: LocalRequestQueueLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type LocalRequestQueueCandidate = Infer<typeof AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofBaseSchema>;

export const AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localRequestQueueProofIsHonest(proof) ||
        'Expected Android child runtime local notification request queue proof to require queue/readback/drain refs and no service ingestion, approval, provider, external platform, dispatch, enforcement, or raw-row claims'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalNotificationRequestQueueProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema
);

export function createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof(input: {
  readonly checkedAt: AppGameAndroidChildRuntimeLocalNotificationRequestQueueProof['checkedAt'];
}): AppGameAndroidChildRuntimeLocalNotificationRequestQueueProof {
  return decodeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof({
    schemaVersion: 'app-game-android-child-runtime-local-notification-request-queue-proof',
    packageId: 'ca.ocentra.parent.agent',
    notificationRequestQueueState: 'local-notification-request-queue-recorded',
    notificationRequestReadbackState: 'local-notification-request-readback-observed',
    notificationRequestDrainState: 'local-notification-request-drain-recorded',
    requestQueueReadbackObserved: true,
    requestDrainReadbackObserved: true,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-child-runtime-local-notification-request-action-ref',
      'android-child-runtime-local-notification-request-queue-ref',
      'android-child-runtime-local-notification-request-readback-ref',
      'android-child-runtime-local-notification-request-drain-ref',
    ],
    openGaps: [
      'service-request-ingestion-not-proved',
      'parent-approval-round-trip-not-proved',
      'provider-delivery-not-proved',
      'platform-delivery-outside-package-not-proved',
      'adapter-dispatch-not-proved',
      'platform-enforcement-not-proved',
    ],
    serviceRequestIngestionClaimed: false,
    parentApprovalRoundTripClaimed: false,
    providerDeliveryClaimed: false,
    platformDeliveryOutsidePackageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsStored: false,
    parentVisibleSummary:
      'Android child app records package-local ask-parent request queue/readback/drain evidence; service ingestion, approval round trip, provider delivery, external platform delivery, dispatch, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof(
  proof: AppGameAndroidChildRuntimeLocalNotificationRequestQueueProof
) {
  return {
    notificationRequestQueueState: proof.notificationRequestQueueState,
    notificationRequestReadbackState: proof.notificationRequestReadbackState,
    notificationRequestDrainState: proof.notificationRequestDrainState,
    openGapCount: proof.openGaps.length,
  } as const;
}

function localRequestQueueProofIsHonest(proof: LocalRequestQueueCandidate): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.notificationRequestQueueState === 'local-notification-request-queue-recorded' &&
    proof.notificationRequestReadbackState === 'local-notification-request-readback-observed' &&
    proof.notificationRequestDrainState === 'local-notification-request-drain-recorded' &&
    proof.requestQueueReadbackObserved &&
    proof.requestDrainReadbackObserved &&
    proof.proofRefs.includes('android-physical-adb-device-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-action-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-queue-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-readback-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-notification-request-drain-ref') &&
    proof.openGaps.includes('service-request-ingestion-not-proved') &&
    proof.openGaps.includes('parent-approval-round-trip-not-proved') &&
    proof.openGaps.includes('provider-delivery-not-proved') &&
    proof.openGaps.includes('platform-delivery-outside-package-not-proved') &&
    proof.openGaps.includes('adapter-dispatch-not-proved') &&
    proof.openGaps.includes('platform-enforcement-not-proved')
  );
}

