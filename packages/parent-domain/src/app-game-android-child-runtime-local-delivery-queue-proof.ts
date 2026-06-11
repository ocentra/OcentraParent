import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const AndroidChildRuntimeLocalDeliveryQueueText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-delivery-queue-proof')
);

export const AppGameAndroidChildRuntimeLocalDeliveryQueueStateSchema = withParser(
  Schema.Literal('package-local-delivery-queue-recorded', 'package-local-delivery-queue-unavailable')
);

export const AppGameAndroidChildRuntimeLocalDeliveryDrainStateSchema = withParser(
  Schema.Literal('package-local-delivery-drain-recorded', 'package-local-delivery-drain-unavailable')
);

export const AppGameAndroidChildRuntimeLocalDeliveryQueueProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-package-local-delivery-intake-ref',
    'android-child-runtime-package-local-delivery-readback-ref',
    'android-child-runtime-package-local-delivery-queue-ref',
    'android-child-runtime-package-local-delivery-drain-ref',
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalDeliveryQueueGapSchema = withParser(
  Schema.Literal(
    'android-child-runtime-service-delivery-ingestion-not-proved',
    'android-child-runtime-service-receipt-ingestion-not-proved',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved-outside-package',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved',
    'android-raw-private-source-rows-not-included'
  )
);

const AndroidChildRuntimeLocalDeliveryQueueProofIdSchema = AndroidChildRuntimeLocalDeliveryQueueText.pipe(
  Schema.brand('AppGameAndroidChildRuntimeLocalDeliveryQueueProofId')
);

const AppGameAndroidChildRuntimeLocalDeliveryQueueProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeLocalDeliveryQueueProofIdSchema,
  deliveryIntakeState: Schema.Literal('package-local-delivery-intake-recorded'),
  deliveryReadbackState: Schema.Literal('package-local-delivery-readback-observed'),
  deliveryQueueState: AppGameAndroidChildRuntimeLocalDeliveryQueueStateSchema,
  deliveryDrainState: AppGameAndroidChildRuntimeLocalDeliveryDrainStateSchema,
  receiptChannelState: Schema.Literal('package-local-receipt-channel-recorded'),
  receiptAppendState: Schema.Literal('local-receipt-append-recorded'),
  receiptLocalAckState: Schema.Literal('local-receipt-ack-recorded'),
  packageLocalDeliveryRecordCount: Schema.Literal(1),
  packageLocalDeliveryQueueRecordCount: Schema.Literal(1),
  packageLocalDeliveryDrainRecordCount: Schema.Literal(1),
  packageLocalChannelRecordCount: Schema.Literal(1),
  localReceiptRecordCount: Schema.Literal(1),
  localReceiptAckRecordCount: Schema.Literal(1),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalDeliveryQueueProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalDeliveryQueueGapSchema),
  packageLocalDeliveryQueued: Schema.Literal(true),
  packageLocalDeliveryDrained: Schema.Literal(true),
  serviceDeliveryIngested: Schema.Literal(false),
  serviceReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimedOutsidePackage: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeLocalDeliveryQueueProofIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeLocalDeliveryQueueCandidate = Infer<
  typeof AppGameAndroidChildRuntimeLocalDeliveryQueueProofBaseSchema
>;

export const AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalDeliveryQueueProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeLocalDeliveryQueueProofIsHonest(proof) ||
        'Expected Android child runtime local delivery queue proof to prove only package-local queue and drain custody while keeping service ingestion, provider delivery, external platform channel, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalDeliveryQueueProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalDeliveryQueueProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchema
);

export function createAppGameAndroidChildRuntimeLocalDeliveryQueueProof(input: {
  readonly deliveryIntakeState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['deliveryIntakeState'];
  readonly deliveryReadbackState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['deliveryReadbackState'];
  readonly deliveryQueueState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['deliveryQueueState'];
  readonly deliveryDrainState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['deliveryDrainState'];
  readonly receiptChannelState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['receiptChannelState'];
  readonly receiptAppendState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['receiptAppendState'];
  readonly receiptLocalAckState: AppGameAndroidChildRuntimeLocalDeliveryQueueProof['receiptLocalAckState'];
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeLocalDeliveryQueueProof {
  return decodeAppGameAndroidChildRuntimeLocalDeliveryQueueProof({
    schemaVersion: 'app-game-android-child-runtime-local-delivery-queue-proof',
    proofId: 'android-child-runtime-local-delivery-queue-proof-ref',
    deliveryIntakeState: input.deliveryIntakeState,
    deliveryReadbackState: input.deliveryReadbackState,
    deliveryQueueState: input.deliveryQueueState,
    deliveryDrainState: input.deliveryDrainState,
    receiptChannelState: input.receiptChannelState,
    receiptAppendState: input.receiptAppendState,
    receiptLocalAckState: input.receiptLocalAckState,
    packageLocalDeliveryRecordCount: 1,
    packageLocalDeliveryQueueRecordCount: 1,
    packageLocalDeliveryDrainRecordCount: 1,
    packageLocalChannelRecordCount: 1,
    localReceiptRecordCount: 1,
    localReceiptAckRecordCount: 1,
    proofRefs: [
      'android-child-runtime-package-local-delivery-intake-ref',
      'android-child-runtime-package-local-delivery-readback-ref',
      'android-child-runtime-package-local-delivery-queue-ref',
      'android-child-runtime-package-local-delivery-drain-ref',
      'android-child-runtime-package-local-receipt-channel-ref',
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-ack-write-ref',
    ],
    openGaps: [
      'android-child-runtime-service-delivery-ingestion-not-proved',
      'android-child-runtime-service-receipt-ingestion-not-proved',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved-outside-package',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
      'android-raw-private-source-rows-not-included',
    ],
    packageLocalDeliveryQueued: true,
    packageLocalDeliveryDrained: true,
    serviceDeliveryIngested: false,
    serviceReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimedOutsidePackage: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime package-local delivery queue and drain markers were recorded in the child package only; service ingestion, provider delivery, external platform delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalDeliveryQueueProof(
  proof: AppGameAndroidChildRuntimeLocalDeliveryQueueProof
) {
  return {
    deliveryQueueState: proof.deliveryQueueState,
    deliveryDrainState: proof.deliveryDrainState,
    packageLocalDeliveryQueued: proof.packageLocalDeliveryQueued,
    packageLocalDeliveryDrained: proof.packageLocalDeliveryDrained,
    serviceDeliveryIngested: proof.serviceDeliveryIngested,
    providerDeliveryExecuted: proof.providerDeliveryExecuted,
    openGapCount: proof.openGaps.length,
  } as const;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function androidChildRuntimeLocalDeliveryQueueProofIsHonest(
  proof: AndroidChildRuntimeLocalDeliveryQueueCandidate
): boolean {
  return (
    proof.deliveryIntakeState === 'package-local-delivery-intake-recorded' &&
    proof.deliveryReadbackState === 'package-local-delivery-readback-observed' &&
    proof.deliveryQueueState === 'package-local-delivery-queue-recorded' &&
    proof.deliveryDrainState === 'package-local-delivery-drain-recorded' &&
    proof.receiptChannelState === 'package-local-receipt-channel-recorded' &&
    proof.receiptAppendState === 'local-receipt-append-recorded' &&
    proof.receiptLocalAckState === 'local-receipt-ack-recorded' &&
    proof.packageLocalDeliveryRecordCount === 1 &&
    proof.packageLocalDeliveryQueueRecordCount === 1 &&
    proof.packageLocalDeliveryDrainRecordCount === 1 &&
    proof.packageLocalChannelRecordCount === 1 &&
    proof.localReceiptRecordCount === 1 &&
    proof.localReceiptAckRecordCount === 1 &&
    proof.proofRefs.includes('android-child-runtime-package-local-delivery-intake-ref') &&
    proof.proofRefs.includes('android-child-runtime-package-local-delivery-readback-ref') &&
    proof.proofRefs.includes('android-child-runtime-package-local-delivery-queue-ref') &&
    proof.proofRefs.includes('android-child-runtime-package-local-delivery-drain-ref') &&
    proof.proofRefs.includes('android-child-runtime-package-local-receipt-channel-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-write-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-ack-write-ref') &&
    proof.openGaps.includes('android-child-runtime-service-delivery-ingestion-not-proved') &&
    proof.openGaps.includes('android-child-runtime-service-receipt-ingestion-not-proved') &&
    proof.openGaps.includes('android-provider-delivery-not-executed') &&
    proof.openGaps.includes('android-platform-delivery-channel-not-proved-outside-package') &&
    proof.openGaps.includes('android-adapter-dispatch-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved') &&
    proof.openGaps.includes('android-raw-private-source-rows-not-included') &&
    proof.packageLocalDeliveryQueued &&
    proof.packageLocalDeliveryDrained &&
    !proof.serviceDeliveryIngested &&
    !proof.serviceReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimedOutsidePackage &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}
