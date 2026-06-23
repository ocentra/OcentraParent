import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidChildRuntimeLocalReceiptProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-local-receipt-proof')
);

export const AppGameAndroidChildRuntimeLocalReceiptAppendStateSchema = withParser(
  Schema.Literal('local-receipt-append-recorded', 'local-receipt-append-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptReadbackStateSchema = withParser(
  Schema.Literal('local-receipt-readback-observed', 'local-receipt-readback-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptStoreStateSchema = withParser(
  Schema.Literal('internal-receipt-store-available', 'internal-receipt-store-unavailable')
);

export const AppGameAndroidChildRuntimeLocalReceiptProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-internal-receipt-store-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-readback-ref',
    'android-child-runtime-status-ui-ref'
  )
);

export const AppGameAndroidChildRuntimeLocalReceiptGapSchema = withParser(
  Schema.Literal(
    'android-child-runtime-transport-not-executed',
    'android-child-runtime-receipt-not-ingested-by-service',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const AndroidChildRuntimeLocalReceiptProofIdSchema = brandedNonEmptyStringSchema(
  'AppGameAndroidChildRuntimeLocalReceiptProofId'
);

const AppGameAndroidChildRuntimeLocalReceiptProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeLocalReceiptProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeLocalReceiptProofIdSchema,
  receiptStoreState: AppGameAndroidChildRuntimeLocalReceiptStoreStateSchema,
  receiptAppendState: AppGameAndroidChildRuntimeLocalReceiptAppendStateSchema,
  receiptReadbackState: AppGameAndroidChildRuntimeLocalReceiptReadbackStateSchema,
  localReceiptRecordCount: Schema.Literal(1),
  packageActivityVisible: Schema.Boolean,
  uiReceiptAppendStateObserved: Schema.Boolean,
  uiReceiptReadbackStateObserved: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeLocalReceiptGapSchema),
  localReceiptAppendExecuted: Schema.Literal(true),
  localReceiptReadbackObserved: Schema.Literal(true),
  runtimeTransportExecuted: Schema.Literal(false),
  serviceReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeLocalReceiptProofIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeLocalReceiptCandidate = Infer<typeof AppGameAndroidChildRuntimeLocalReceiptProofBaseSchema>;

export const AppGameAndroidChildRuntimeLocalReceiptProofSchema = withParser(
  AppGameAndroidChildRuntimeLocalReceiptProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeLocalReceiptProofIsHonest(proof) ||
        'Expected Android child runtime local receipt proof to prove only package-local receipt append/readback while keeping runtime transport, service ingestion, provider delivery, platform channel, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeLocalReceiptProof = Infer<
  typeof AppGameAndroidChildRuntimeLocalReceiptProofSchema
>;

export const decodeAppGameAndroidChildRuntimeLocalReceiptProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeLocalReceiptProofSchema
);

export function createAppGameAndroidChildRuntimeLocalReceiptProof(input: {
  readonly receiptStoreState: AppGameAndroidChildRuntimeLocalReceiptProof['receiptStoreState'];
  readonly receiptAppendState: AppGameAndroidChildRuntimeLocalReceiptProof['receiptAppendState'];
  readonly receiptReadbackState: AppGameAndroidChildRuntimeLocalReceiptProof['receiptReadbackState'];
  readonly packageActivityVisible: boolean;
  readonly uiReceiptAppendStateObserved: boolean;
  readonly uiReceiptReadbackStateObserved: boolean;
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeLocalReceiptProof {
  return decodeAppGameAndroidChildRuntimeLocalReceiptProof({
    schemaVersion: 'app-game-android-child-runtime-local-receipt-proof',
    proofId: 'android-child-runtime-local-receipt-proof-ref',
    receiptStoreState: input.receiptStoreState,
    receiptAppendState: input.receiptAppendState,
    receiptReadbackState: input.receiptReadbackState,
    localReceiptRecordCount: 1,
    packageActivityVisible: input.packageActivityVisible,
    uiReceiptAppendStateObserved: input.uiReceiptAppendStateObserved,
    uiReceiptReadbackStateObserved: input.uiReceiptReadbackStateObserved,
    proofRefs: androidChildRuntimeLocalReceiptProofRefs(input),
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
    runtimeTransportExecuted: false,
    serviceReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime local receipt append and readback are package-local only; runtime transport, service receipt ingestion, provider delivery, platform channel delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeLocalReceiptProof(
  proof: AppGameAndroidChildRuntimeLocalReceiptProof
) {
  return {
    receiptStoreState: proof.receiptStoreState,
    receiptAppendState: proof.receiptAppendState,
    receiptReadbackState: proof.receiptReadbackState,
    localReceiptRecordCount: proof.localReceiptRecordCount,
    localReceiptAppendExecuted: proof.localReceiptAppendExecuted,
    localReceiptReadbackObserved: proof.localReceiptReadbackObserved,
    serviceReceiptIngested: proof.serviceReceiptIngested,
    openGapCount: proof.openGaps.length,
  } as const;
}

function androidChildRuntimeLocalReceiptProofRefs(input: {
  readonly packageActivityVisible: boolean;
  readonly uiReceiptAppendStateObserved: boolean;
  readonly uiReceiptReadbackStateObserved: boolean;
}) {
  const refs = ['android-child-runtime-internal-receipt-store-ref'];
  if (input.uiReceiptAppendStateObserved) {
    refs.push('android-child-runtime-local-receipt-write-ref');
  }
  if (input.uiReceiptReadbackStateObserved) {
    refs.push('android-child-runtime-local-receipt-readback-ref');
  }
  if (input.packageActivityVisible && input.uiReceiptAppendStateObserved && input.uiReceiptReadbackStateObserved) {
    refs.push('android-child-runtime-status-ui-ref');
  }
  return refs;
}

function androidChildRuntimeLocalReceiptProofIsHonest(proof: AndroidChildRuntimeLocalReceiptCandidate): boolean {
  return (
    androidChildRuntimeLocalReceiptStateIsHonest(proof) &&
    androidChildRuntimeLocalReceiptProofRefsArePresent(proof) &&
    androidChildRuntimeLocalReceiptOpenGapsArePresent(proof) &&
    androidChildRuntimeLocalReceiptClaimsRemainScoped(proof)
  );
}

function androidChildRuntimeLocalReceiptStateIsHonest(proof: AndroidChildRuntimeLocalReceiptCandidate): boolean {
  return (
    proof.receiptStoreState === 'internal-receipt-store-available' &&
    proof.receiptAppendState === 'local-receipt-append-recorded' &&
    proof.receiptReadbackState === 'local-receipt-readback-observed' &&
    proof.localReceiptRecordCount === 1 &&
    proof.packageActivityVisible &&
    proof.uiReceiptAppendStateObserved &&
    proof.uiReceiptReadbackStateObserved &&
    proof.localReceiptAppendExecuted &&
    proof.localReceiptReadbackObserved
  );
}

function androidChildRuntimeLocalReceiptProofRefsArePresent(proof: AndroidChildRuntimeLocalReceiptCandidate): boolean {
  return includesAll(proof.proofRefs, [
    'android-child-runtime-internal-receipt-store-ref',
    'android-child-runtime-local-receipt-write-ref',
    'android-child-runtime-local-receipt-readback-ref',
    'android-child-runtime-status-ui-ref',
  ] as const);
}

function androidChildRuntimeLocalReceiptOpenGapsArePresent(proof: AndroidChildRuntimeLocalReceiptCandidate): boolean {
  return includesAll(proof.openGaps, [
    'android-child-runtime-transport-not-executed',
    'android-child-runtime-receipt-not-ingested-by-service',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved',
  ] as const);
}

function androidChildRuntimeLocalReceiptClaimsRemainScoped(proof: AndroidChildRuntimeLocalReceiptCandidate): boolean {
  return (
    !proof.runtimeTransportExecuted &&
    !proof.serviceReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}

function includesAll<T extends string>(values: readonly T[], required: readonly T[]): boolean {
  return required.every((value) => values.includes(value));
}
