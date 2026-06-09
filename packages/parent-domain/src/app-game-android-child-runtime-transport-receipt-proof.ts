import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const AndroidChildRuntimeReceiptText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidChildRuntimeTransportReceiptProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-child-runtime-transport-receipt-proof')
);

export const AppGameAndroidChildRuntimeTransportChannelStateSchema = withParser(
  Schema.Literal('activity-visible-transport-channel', 'activity-unavailable-transport-channel')
);

export const AppGameAndroidChildRuntimeReceiptStoreStateSchema = withParser(
  Schema.Literal('internal-receipt-store-available', 'internal-receipt-store-unavailable')
);

export const AppGameAndroidChildRuntimeReceiptAckStateSchema = withParser(
  Schema.Literal('receipt-ack-waiting-for-runtime')
);

export const AppGameAndroidChildRuntimeTransportReceiptProofRefSchema = withParser(
  Schema.Literal(
    'android-child-runtime-activity-transport-ref',
    'android-child-runtime-internal-receipt-store-ref',
    'android-child-runtime-status-ui-ref'
  )
);

export const AppGameAndroidChildRuntimeTransportReceiptGapSchema = withParser(
  Schema.Literal(
    'android-child-runtime-transport-not-executed',
    'android-child-runtime-receipt-not-ingested',
    'android-provider-delivery-not-executed',
    'android-platform-delivery-channel-not-proved',
    'android-adapter-dispatch-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const AndroidChildRuntimeReceiptIdSchema = AndroidChildRuntimeReceiptText.pipe(
  Schema.brand('AppGameAndroidChildRuntimeTransportReceiptProofId')
);

const AppGameAndroidChildRuntimeTransportReceiptProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidChildRuntimeTransportReceiptProofSchemaVersionSchema,
  proofId: AndroidChildRuntimeReceiptIdSchema,
  transportChannelState: AppGameAndroidChildRuntimeTransportChannelStateSchema,
  receiptStoreState: AppGameAndroidChildRuntimeReceiptStoreStateSchema,
  receiptAckState: AppGameAndroidChildRuntimeReceiptAckStateSchema,
  packageActivityVisible: Schema.Boolean,
  uiTransportStateObserved: Schema.Boolean,
  uiReceiptStateObserved: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidChildRuntimeTransportReceiptProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidChildRuntimeTransportReceiptGapSchema),
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  parentVisibleSummary: AndroidChildRuntimeReceiptIdSchema,
  checkedAt: ParentTimestampSchema,
});

type AndroidChildRuntimeReceiptCandidate = Infer<
  typeof AppGameAndroidChildRuntimeTransportReceiptProofBaseSchema
>;

export const AppGameAndroidChildRuntimeTransportReceiptProofSchema = withParser(
  AppGameAndroidChildRuntimeTransportReceiptProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidChildRuntimeTransportReceiptProofIsHonest(proof) ||
        'Expected Android child runtime transport receipt proof to expose only parent-safe transport/store readiness and keep transport execution, receipt ingestion, provider delivery, platform delivery, adapter dispatch, platform enforcement, and raw rows unclaimed'
    )
  )
);

export type AppGameAndroidChildRuntimeTransportReceiptProof = Infer<
  typeof AppGameAndroidChildRuntimeTransportReceiptProofSchema
>;

export const decodeAppGameAndroidChildRuntimeTransportReceiptProof = Schema.decodeUnknownSync(
  AppGameAndroidChildRuntimeTransportReceiptProofSchema
);

export function createAppGameAndroidChildRuntimeTransportReceiptProof(input: {
  readonly transportChannelState: AppGameAndroidChildRuntimeTransportReceiptProof['transportChannelState'];
  readonly receiptStoreState: AppGameAndroidChildRuntimeTransportReceiptProof['receiptStoreState'];
  readonly receiptAckState: AppGameAndroidChildRuntimeTransportReceiptProof['receiptAckState'];
  readonly packageActivityVisible: boolean;
  readonly uiTransportStateObserved: boolean;
  readonly uiReceiptStateObserved: boolean;
  readonly checkedAt: string;
}): AppGameAndroidChildRuntimeTransportReceiptProof {
  return decodeAppGameAndroidChildRuntimeTransportReceiptProof({
    schemaVersion: 'app-game-android-child-runtime-transport-receipt-proof',
    proofId: 'android-child-runtime-transport-receipt-proof-ref',
    transportChannelState: input.transportChannelState,
    receiptStoreState: input.receiptStoreState,
    receiptAckState: input.receiptAckState,
    packageActivityVisible: input.packageActivityVisible,
    uiTransportStateObserved: input.uiTransportStateObserved,
    uiReceiptStateObserved: input.uiReceiptStateObserved,
    proofRefs: androidChildRuntimeTransportReceiptProofRefs(input),
    openGaps: [
      'android-child-runtime-transport-not-executed',
      'android-child-runtime-receipt-not-ingested',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
    parentVisibleSummary:
      'Android child runtime transport and receipt readiness is visible in the child app, but transport execution, receipt ingestion, provider delivery, platform delivery, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidChildRuntimeTransportReceiptProof(
  proof: AppGameAndroidChildRuntimeTransportReceiptProof
) {
  return {
    transportChannelState: proof.transportChannelState,
    receiptStoreState: proof.receiptStoreState,
    receiptAckState: proof.receiptAckState,
    packageActivityVisible: proof.packageActivityVisible,
    uiTransportStateObserved: proof.uiTransportStateObserved,
    uiReceiptStateObserved: proof.uiReceiptStateObserved,
    openGapCount: proof.openGaps.length,
  } as const;
}

function androidChildRuntimeTransportReceiptProofRefs(input: {
  readonly packageActivityVisible: boolean;
  readonly uiTransportStateObserved: boolean;
  readonly uiReceiptStateObserved: boolean;
}) {
  const refs = [];
  if (input.packageActivityVisible) {
    refs.push('android-child-runtime-activity-transport-ref');
  }
  if (input.uiReceiptStateObserved) {
    refs.push('android-child-runtime-internal-receipt-store-ref');
  }
  if (input.uiTransportStateObserved && input.uiReceiptStateObserved) {
    refs.push('android-child-runtime-status-ui-ref');
  }
  return refs;
}

function androidChildRuntimeTransportReceiptProofIsHonest(
  proof: AndroidChildRuntimeReceiptCandidate
): boolean {
  return (
    proof.transportChannelState === 'activity-visible-transport-channel' &&
    proof.receiptStoreState === 'internal-receipt-store-available' &&
    proof.receiptAckState === 'receipt-ack-waiting-for-runtime' &&
    proof.packageActivityVisible &&
    proof.uiTransportStateObserved &&
    proof.uiReceiptStateObserved &&
    proof.proofRefs.includes('android-child-runtime-activity-transport-ref') &&
    proof.proofRefs.includes('android-child-runtime-internal-receipt-store-ref') &&
    proof.proofRefs.includes('android-child-runtime-status-ui-ref') &&
    proof.openGaps.includes('android-child-runtime-transport-not-executed') &&
    proof.openGaps.includes('android-child-runtime-receipt-not-ingested') &&
    proof.openGaps.includes('android-provider-delivery-not-executed') &&
    proof.openGaps.includes('android-platform-delivery-channel-not-proved') &&
    proof.openGaps.includes('android-adapter-dispatch-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved') &&
    !proof.runtimeTransportExecuted &&
    !proof.runtimeReceiptIngested &&
    !proof.providerDeliveryExecuted &&
    !proof.platformDeliveryChannelClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.rawPrivateSourceRowsIncluded
  );
}
