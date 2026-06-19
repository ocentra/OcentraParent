import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidChildRuntimeLocalReceiptAckProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-receipt-ack-proof')
);

export const AppGameAndroidChildRuntimeLocalReceiptAckStateSchema = withParser(
  Schema.Literal('local-receipt-ack-recorded', 'local-receipt-ack-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptAckReadbackStateSchema = withParser(
  Schema.Literal('local-receipt-ack-readback-observed', 'local-receipt-ack-readback-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptAckProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-readback-ref',
    'android-child-runtime-local-receipt-ack-write-ref',
    'android-child-runtime-local-receipt-ack-readback-ref',
    'android-child-runtime-status-ui-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalReceiptAckGapSchema = withParser(
  Schema.Literal(
    'android-child-runtime-transport-not-executed',
    'android-child-runtime-receipt-not-ingested-by-service',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const AndroidChildRuntimeLocalReceiptAckProofIdSchema = brandedNonEmptyStringSchema('AppGameAndroidChildRuntimeLocalReceiptAckProofId');

const AppGameAndroidChildRuntimeLocalReceiptAckProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalReceiptAckProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeLocalReceiptAckProofIdSchema,
  receiptAppendState: Schema.Literal('local-receipt-append-recorded'),
  receiptReadbackState: Schema.Literal('local-receipt-readback-observed'),
  receiptLocalAckState: AppGameAndroidChildRuntimeLocalReceiptAckStateSchema,
  receiptLocalAckReadbackState: AppGameAndroidChildRuntimeLocalReceiptAckReadbackStateSchema,
  localReceiptRecordCount: Schema.Literal(1),
  localReceiptAckRecordCount: Schema.Literal(1),
  packageActivityVisible: Schema.Boolean,
  uiReceiptAckStateObserved: Schema.Boolean,
  uiReceiptAckReadbackStateObserved: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptAckProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptAckGapSchema),
  localReceiptAppendExecuted: Schema.Literal(true),
  localReceiptReadbackObserved: Schema.Literal(true),
  localReceiptAckExecuted: Schema.Literal(true),
  localReceiptAckReadbackObserved: Schema.Literal(true),
  runtimeTransportExecuted: Schema.Literal(false),
  serviceReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeLocalReceiptAckProofIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeLocalReceiptAckCandidate = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptAckProofBaseSchema
>;

export const AppGameAndroidChildRuntimeLocalReceiptAckProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalReceiptAckProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeLocalReceiptAckProofIsHonest(proof) ||
        'Expected Android child runtime local receipt ack proof to prove only package-local receipt and ack write/readback while keeping runtime transport, service ingestion, provider delivery, platform channel, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalReceiptAckProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptAckProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalReceiptAckProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalReceiptAckProofSchema
);

export function createAppGameAndroidChildRuntimeLocalReceiptAckProof(input: {
  readonly receiptAppendState: AppGameAndroidChildRuntimeLocalReceiptAckProof['receiptAppendState'];
  readonly receiptReadbackState: AppGameAndroidChildRuntimeLocalReceiptAckProof['receiptReadbackState'];
  readonly receiptLocalAckState: AppGameAndroidChildRuntimeLocalReceiptAckProof['receiptLocalAckState'];
  readonly receiptLocalAckReadbackState: AppGameAndroidChildRuntimeLocalReceiptAckProof['receiptLocalAckReadbackState'];
  readonly packageActivityVisible: boolean;
  readonly uiReceiptAckStateObserved: boolean;
  readonly uiReceiptAckReadbackStateObserved: boolean;
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeLocalReceiptAckProof {
  return decodeAppGameAndroidChildRuntimeLocalReceiptAckProof({
    schemaVersion: 'app-game-android-child-runtime-local-receipt-ack-proof',
    proofId: 'android-child-runtime-local-receipt-ack-proof-ref',
    receiptAppendState: input.receiptAppendState,
    receiptReadbackState: input.receiptReadbackState,
    receiptLocalAckState: input.receiptLocalAckState,
    receiptLocalAckReadbackState: input.receiptLocalAckReadbackState,
    localReceiptRecordCount: 1,
    localReceiptAckRecordCount: 1,
    packageActivityVisible: input.packageActivityVisible,
    uiReceiptAckStateObserved: input.uiReceiptAckStateObserved,
    uiReceiptAckReadbackStateObserved: input.uiReceiptAckReadbackStateObserved,
    proofRefs: androidChildRuntimeLocalReceiptAckProofRefs(input),
    openGaps: [
      'android-child-runtime-transport-not-executed',
      'android-child-runtime-receipt-not-ingested-by-service',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    localReceiptAppendExecuted: true,
    localReceiptReadbackObserved: true,
    localReceiptAckExecuted: true,
    localReceiptAckReadbackObserved: true,
    runtimeTransportExecuted: false,
    serviceReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime local receipt ack is package-local only; runtime transport, service receipt ingestion, provider delivery, platform channel delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalReceiptAckProof(
  proof: AppGameAndroidChildRuntimeLocalReceiptAckProof
) {
  return {
    receiptAppendState: proof.receiptAppendState,
    receiptReadbackState: proof.receiptReadbackState,
    receiptLocalAckState: proof.receiptLocalAckState,
    receiptLocalAckReadbackState: proof.receiptLocalAckReadbackState,
    localReceiptRecordCount: proof.localReceiptRecordCount,
    localReceiptAckRecordCount: proof.localReceiptAckRecordCount,
    serviceReceiptIngested: proof.serviceReceiptIngested,
    openGapCount: proof.openGaps.length,
  } as const;
}

function androidChildRuntimeLocalReceiptAckProofRefs(input: {
  readonly packageActivityVisible: boolean;
  readonly uiReceiptAckStateObserved: boolean;
  readonly uiReceiptAckReadbackStateObserved: boolean;
}) {
  const refs = ['android-child-runtime-local-receipt-write-ref', 'android-child-runtime-local-receipt-readback-ref'];
  if (input.uiReceiptAckStateObserved) {
    refs.push('android-child-runtime-local-receipt-ack-write-ref');
  }
  if (input.uiReceiptAckReadbackStateObserved) {
    refs.push('android-child-runtime-local-receipt-ack-readback-ref');
  }
  if (input.packageActivityVisible && input.uiReceiptAckStateObserved && input.uiReceiptAckReadbackStateObserved) {
    refs.push('android-child-runtime-status-ui-ref');
  }
  return refs;
}

function androidChildRuntimeLocalReceiptAckProofIsHonest(proof: AndroidChildRuntimeLocalReceiptAckCandidate): boolean {
  return (
    proof.receiptAppendState === 'local-receipt-append-recorded' &&
    proof.receiptReadbackState === 'local-receipt-readback-observed' &&
    proof.receiptLocalAckState === 'local-receipt-ack-recorded' &&
    proof.receiptLocalAckReadbackState === 'local-receipt-ack-readback-observed' &&
    proof.localReceiptRecordCount === 1 &&
    proof.localReceiptAckRecordCount === 1 &&
    proof.packageActivityVisible &&
    proof.uiReceiptAckStateObserved &&
    proof.uiReceiptAckReadbackStateObserved &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-write-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-readback-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-ack-write-ref') &&
    proof.proofRefs.includes('android-child-runtime-local-receipt-ack-readback-ref') &&
    proof.proofRefs.includes('android-child-runtime-status-ui-ref') &&
    proof.openGaps.includes('android-child-runtime-transport-not-executed') &&
    proof.openGaps.includes('android-child-runtime-receipt-not-ingested-by-service') &&
    proof.openGaps.includes('android-provider-delivery-not-executed') &&
    proof.openGaps.includes('android-platform-delivery-channel-not-proved') &&
    proof.openGaps.includes('android-adapter-dispatch-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved') &&
    proof.localReceiptAppendExecuted &&
    proof.localReceiptReadbackObserved &&
    proof.localReceiptAckExecuted &&
    proof.localReceiptAckReadbackObserved &&
    !proof.runtimeTransportExecuted &&
    !proof.serviceReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}

