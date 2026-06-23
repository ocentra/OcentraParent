import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-delivery-intake-proof')
);

export const AppGameAndroidChildRuntimeLocalDeliveryIntakeStateSchema = withParser(
  Schema.Literal('package-local-delivery-intake-recorded', 'package-local-delivery-intake-unavailable')
);

export const AppGameAndroidChildRuntimeLocalDeliveryReadbackStateSchema = withParser(
  Schema.Literal('package-local-delivery-readback-observed', 'package-local-delivery-readback-unavailable')
);

export const AppGameAndroidChildRuntimeLocalDeliveryProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-package-local-delivery-intake-ref',
    'android-child-runtime-package-local-delivery-readback-ref',
    'android-child-runtime-package-local-delivery-receiver-ref',
    'android-child-runtime-package-local-delivery-activity-trigger-ref',
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalDeliveryGapSchema = withParser(
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

const AndroidChildRuntimeLocalDeliveryProofIdSchema = brandedNonEmptyStringSchema(
  'AppGameAndroidChildRuntimeLocalDeliveryProofId'
);

const AppGameAndroidChildRuntimeLocalDeliveryIntakeProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeLocalDeliveryProofIdSchema,
  deliveryIntakeState: AppGameAndroidChildRuntimeLocalDeliveryIntakeStateSchema,
  deliveryReadbackState: AppGameAndroidChildRuntimeLocalDeliveryReadbackStateSchema,
  receiptChannelState: Schema.Literal('package-local-receipt-channel-recorded'),
  receiptAppendState: Schema.Literal('local-receipt-append-recorded'),
  receiptLocalAckState: Schema.Literal('local-receipt-ack-recorded'),
  packageLocalDeliveryReceiverDeclared: Schema.Boolean,
  packageLocalDeliveryTriggeredByActivity: Schema.Boolean,
  packageLocalDeliveryRecordCount: Schema.Literal(1),
  packageLocalChannelRecordCount: Schema.Literal(1),
  localReceiptRecordCount: Schema.Literal(1),
  localReceiptAckRecordCount: Schema.Literal(1),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalDeliveryProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalDeliveryGapSchema),
  packageLocalDeliveryExecuted: Schema.Literal(true),
  serviceDeliveryIngested: Schema.Literal(false),
  serviceReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimedOutsidePackage: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeLocalDeliveryProofIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeLocalDeliveryCandidate = Infer<
  typeof AppGameAndroidChildRuntimeLocalDeliveryIntakeProofBaseSchema
>;
type AndroidChildRuntimeLocalDeliveryProofRef = Infer<typeof AppGameAndroidChildRuntimeLocalDeliveryProofRefSchema>;

export const AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalDeliveryIntakeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeLocalDeliveryProofIsHonest(proof) ||
        'Expected Android child runtime local delivery intake proof to prove only package-local delivery intake, receipt channel, receipt, and ack marker custody while keeping service ingestion, provider delivery, external platform channel, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalDeliveryIntakeProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchema
);

export function createAppGameAndroidChildRuntimeLocalDeliveryIntakeProof(input: {
  readonly deliveryIntakeState: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof['deliveryIntakeState'];
  readonly deliveryReadbackState: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof['deliveryReadbackState'];
  readonly receiptChannelState: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof['receiptChannelState'];
  readonly receiptAppendState: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof['receiptAppendState'];
  readonly receiptLocalAckState: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof['receiptLocalAckState'];
  readonly packageLocalDeliveryReceiverDeclared: boolean;
  readonly packageLocalDeliveryTriggeredByActivity: boolean;
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeLocalDeliveryIntakeProof {
  return decodeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof({
    schemaVersion: 'app-game-android-child-runtime-local-delivery-intake-proof',
    proofId: 'android-child-runtime-local-delivery-intake-proof-ref',
    deliveryIntakeState: input.deliveryIntakeState,
    deliveryReadbackState: input.deliveryReadbackState,
    receiptChannelState: input.receiptChannelState,
    receiptAppendState: input.receiptAppendState,
    receiptLocalAckState: input.receiptLocalAckState,
    packageLocalDeliveryReceiverDeclared: input.packageLocalDeliveryReceiverDeclared,
    packageLocalDeliveryTriggeredByActivity: input.packageLocalDeliveryTriggeredByActivity,
    packageLocalDeliveryRecordCount: 1,
    packageLocalChannelRecordCount: 1,
    localReceiptRecordCount: 1,
    localReceiptAckRecordCount: 1,
    proofRefs: androidChildRuntimeLocalDeliveryProofRefs(input),
    openGaps: [
      'android-child-runtime-service-delivery-ingestion-not-proved',
      'android-child-runtime-service-receipt-ingestion-not-proved',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved-outside-package',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
      'android-raw-private-source-rows-not-included',
    ],
    packageLocalDeliveryExecuted: true,
    serviceDeliveryIngested: false,
    serviceReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimedOutsidePackage: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime package-local delivery intake executed in the child package only; service ingestion, provider delivery, external platform delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof(
  proof: AppGameAndroidChildRuntimeLocalDeliveryIntakeProof
) {
  return {
    deliveryIntakeState: proof.deliveryIntakeState,
    deliveryReadbackState: proof.deliveryReadbackState,
    packageLocalDeliveryReceiverDeclared: proof.packageLocalDeliveryReceiverDeclared,
    packageLocalDeliveryTriggeredByActivity: proof.packageLocalDeliveryTriggeredByActivity,
    packageLocalDeliveryExecuted: proof.packageLocalDeliveryExecuted,
    serviceDeliveryIngested: proof.serviceDeliveryIngested,
    providerDeliveryExecuted: proof.providerDeliveryExecuted,
    openGapCount: proof.openGaps.length,
  } as const;
}

function androidChildRuntimeLocalDeliveryProofRefs(input: {
  readonly packageLocalDeliveryReceiverDeclared: boolean;
  readonly packageLocalDeliveryTriggeredByActivity: boolean;
}): ReadonlyArray<AndroidChildRuntimeLocalDeliveryProofRef> {
  const refs: Array<AndroidChildRuntimeLocalDeliveryProofRef> = [
    'android-child-runtime-package-local-delivery-intake-ref',
    'android-child-runtime-package-local-delivery-readback-ref',
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref',
  ];
  if (input.packageLocalDeliveryReceiverDeclared) {
    refs.push('android-child-runtime-package-local-delivery-receiver-ref');
  }
  if (input.packageLocalDeliveryTriggeredByActivity) {
    refs.push('android-child-runtime-package-local-delivery-activity-trigger-ref');
  }
  return refs;
}

function androidChildRuntimeLocalDeliveryProofIsHonest(proof: AndroidChildRuntimeLocalDeliveryCandidate): boolean {
  return (
    androidChildRuntimeLocalDeliveryStateIsHonest(proof) &&
    androidChildRuntimeLocalDeliveryProofRefsArePresent(proof) &&
    androidChildRuntimeLocalDeliveryOpenGapsArePresent(proof) &&
    androidChildRuntimeLocalDeliveryClaimsRemainScoped(proof)
  );
}

function androidChildRuntimeLocalDeliveryStateIsHonest(proof: AndroidChildRuntimeLocalDeliveryCandidate): boolean {
  return (
    proof.deliveryIntakeState === 'package-local-delivery-intake-recorded' &&
    proof.deliveryReadbackState === 'package-local-delivery-readback-observed' &&
    proof.receiptChannelState === 'package-local-receipt-channel-recorded' &&
    proof.receiptAppendState === 'local-receipt-append-recorded' &&
    proof.receiptLocalAckState === 'local-receipt-ack-recorded' &&
    proof.packageLocalDeliveryReceiverDeclared &&
    proof.packageLocalDeliveryTriggeredByActivity &&
    proof.packageLocalDeliveryRecordCount === 1 &&
    proof.packageLocalChannelRecordCount === 1 &&
    proof.localReceiptRecordCount === 1 &&
    proof.localReceiptAckRecordCount === 1 &&
    proof.packageLocalDeliveryExecuted
  );
}

function androidChildRuntimeLocalDeliveryProofRefsArePresent(
  proof: AndroidChildRuntimeLocalDeliveryCandidate
): boolean {
  return includesAll(proof.proofRefs, [
    'android-child-runtime-package-local-delivery-intake-ref',
    'android-child-runtime-package-local-delivery-readback-ref',
    'android-child-runtime-package-local-delivery-receiver-ref',
    'android-child-runtime-package-local-delivery-activity-trigger-ref',
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref',
  ] as const);
}

function androidChildRuntimeLocalDeliveryOpenGapsArePresent(proof: AndroidChildRuntimeLocalDeliveryCandidate): boolean {
  return includesAll(proof.openGaps, [
    'android-child-runtime-service-delivery-ingestion-not-proved',
    'android-child-runtime-service-receipt-ingestion-not-proved',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved-outside-package',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved',
    'android-raw-private-source-rows-not-included',
  ] as const);
}

function androidChildRuntimeLocalDeliveryClaimsRemainScoped(proof: AndroidChildRuntimeLocalDeliveryCandidate): boolean {
  return (
    !proof.serviceDeliveryIngested &&
    !proof.serviceReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimedOutsidePackage &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}

function includesAll<T extends string>(values: readonly T[], required: readonly T[]): boolean {
  return required.every((value) => values.includes(value));
}
