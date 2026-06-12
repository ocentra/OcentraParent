import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const AndroidChildRuntimeLocalReceiptChannelText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidChildRuntimeLocalReceiptChannelProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-receipt-channel-proof')
);

export const AppGameAndroidChildRuntimeLocalReceiptChannelStateSchema = withParser(
  Schema.Literal('package-local-receipt-channel-recorded', 'package-local-receipt-channel-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptChannelProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref',
    'android-child-runtime-manifest-receiver-ref',
    'android-child-runtime-activity-trigger-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalReceiptChannelGapSchema = withParser(
  Schema.Literal(
    'android-child-runtime-service-receipt-ingestion-not-proved',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved-outside-package',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved',
    'android-raw-private-source-rows-not-included'
  )
);

const AndroidChildRuntimeLocalReceiptChannelProofIdSchema = AndroidChildRuntimeLocalReceiptChannelText.pipe(
  Schema.brand('AppGameAndroidChildRuntimeLocalReceiptChannelProofId')
);

const AppGameAndroidChildRuntimeLocalReceiptChannelProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalReceiptChannelProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeLocalReceiptChannelProofIdSchema,
  receiptChannelState: AppGameAndroidChildRuntimeLocalReceiptChannelStateSchema,
  receiptAppendState: Schema.Literal('local-receipt-append-recorded'),
  receiptLocalAckState: Schema.Literal('local-receipt-ack-recorded'),
  packageLocalBroadcastReceiverDeclared: Schema.Boolean,
  packageLocalBroadcastTriggeredByActivity: Schema.Boolean,
  packageLocalChannelRecordCount: Schema.Literal(1),
  localReceiptRecordCount: Schema.Literal(1),
  localReceiptAckRecordCount: Schema.Literal(1),
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptChannelProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptChannelGapSchema),
  packageLocalChannelExecuted: Schema.Literal(true),
  runtimeTransportExecutedOutsidePackage: Schema.Literal(false),
  serviceReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimedOutsidePackage: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeLocalReceiptChannelProofIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeLocalReceiptChannelCandidate = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptChannelProofBaseSchema
>;
type AndroidChildRuntimeLocalReceiptChannelProofRef = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptChannelProofRefSchema
>;

export const AppGameAndroidChildRuntimeLocalReceiptChannelProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalReceiptChannelProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeLocalReceiptChannelProofIsHonest(proof) ||
        'Expected Android child runtime local receipt channel proof to prove only in-package broadcast channel execution while keeping service ingestion, provider delivery, external platform channel, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalReceiptChannelProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptChannelProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalReceiptChannelProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalReceiptChannelProofSchema
);

export function createAppGameAndroidChildRuntimeLocalReceiptChannelProof(input: {
  readonly receiptChannelState: AppGameAndroidChildRuntimeLocalReceiptChannelProof['receiptChannelState'];
  readonly receiptAppendState: AppGameAndroidChildRuntimeLocalReceiptChannelProof['receiptAppendState'];
  readonly receiptLocalAckState: AppGameAndroidChildRuntimeLocalReceiptChannelProof['receiptLocalAckState'];
  readonly packageLocalBroadcastReceiverDeclared: boolean;
  readonly packageLocalBroadcastTriggeredByActivity: boolean;
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeLocalReceiptChannelProof {
  return decodeAppGameAndroidChildRuntimeLocalReceiptChannelProof({
    schemaVersion: 'app-game-android-child-runtime-local-receipt-channel-proof',
    proofId: 'android-child-runtime-local-receipt-channel-proof-ref',
    receiptChannelState: input.receiptChannelState,
    receiptAppendState: input.receiptAppendState,
    receiptLocalAckState: input.receiptLocalAckState,
    packageLocalBroadcastReceiverDeclared: input.packageLocalBroadcastReceiverDeclared,
    packageLocalBroadcastTriggeredByActivity: input.packageLocalBroadcastTriggeredByActivity,
    packageLocalChannelRecordCount: 1,
    localReceiptRecordCount: 1,
    localReceiptAckRecordCount: 1,
    proofRefs: androidChildRuntimeLocalReceiptChannelProofRefs(input),
    openGaps: [
      'android-child-runtime-service-receipt-ingestion-not-proved',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved-outside-package',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
      'android-raw-private-source-rows-not-included',
    ],
    packageLocalChannelExecuted: true,
    runtimeTransportExecutedOutsidePackage: false,
    serviceReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimedOutsidePackage: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime package-local receipt channel executed in the child package only; service receipt ingestion, provider delivery, external platform delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalReceiptChannelProof(
  proof: AppGameAndroidChildRuntimeLocalReceiptChannelProof
) {
  return {
    receiptChannelState: proof.receiptChannelState,
    packageLocalBroadcastReceiverDeclared: proof.packageLocalBroadcastReceiverDeclared,
    packageLocalBroadcastTriggeredByActivity: proof.packageLocalBroadcastTriggeredByActivity,
    packageLocalChannelExecuted: proof.packageLocalChannelExecuted,
    serviceReceiptIngested: proof.serviceReceiptIngested,
    providerDeliveryExecuted: proof.providerDeliveryExecuted,
    openGapCount: proof.openGaps.length,
  } as const;
}

function androidChildRuntimeLocalReceiptChannelProofRefs(input: {
  readonly packageLocalBroadcastReceiverDeclared: boolean;
  readonly packageLocalBroadcastTriggeredByActivity: boolean;
}): ReadonlyArray<AndroidChildRuntimeLocalReceiptChannelProofRef> {
  const refs: Array<AndroidChildRuntimeLocalReceiptChannelProofRef> = [
    'android-child-runtime-package-local-receipt-channel-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-ack-write-ref',
  ];
  if (input.packageLocalBroadcastReceiverDeclared) {
    refs.push('android-child-runtime-manifest-receiver-ref');
  }
  if (input.packageLocalBroadcastTriggeredByActivity) {
    refs.push('android-child-runtime-activity-trigger-ref');
  }
  return refs;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function androidChildRuntimeLocalReceiptChannelProofIsHonest(
  proof: AndroidChildRuntimeLocalReceiptChannelCandidate
): boolean {
  return (
    proof.receiptChannelState === 'package-local-receipt-channel-recorded' &&
    proof.receiptAppendState === 'local-receipt-append-recorded' &&
    proof.receiptLocalAckState === 'local-receipt-ack-recorded' &&
    proof.packageLocalBroadcastReceiverDeclared &&
    proof.packageLocalBroadcastTriggeredByActivity &&
    proof.packageLocalChannelRecordCount === 1 &&
    proof.localReceiptRecordCount === 1 &&
    proof.localReceiptAckRecordCount === 1 &&
    proof.proofRefs.includes('android-child-runtime-package-local-receipt-channel-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-write-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-ack-write-ref') &&
    proof.proofRefs.includes('android-child-runtime-manifest-receiver-ref') &&
    proof.proofRefs.includes('android-child-runtime-activity-trigger-ref') &&
    proof.openGaps.includes('android-child-runtime-service-receipt-ingestion-not-proved') &&
    proof.openGaps.includes('android-provider-delivery-not-executed') &&
    proof.openGaps.includes('android-platform-delivery-channel-not-proved-outside-package') &&
    proof.openGaps.includes('android-adapter-dispatch-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved') &&
    proof.openGaps.includes('android-raw-private-source-rows-not-included') &&
    proof.packageLocalChannelExecuted &&
    !proof.runtimeTransportExecutedOutsidePackage &&
    !proof.serviceReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimedOutsidePackage &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}
