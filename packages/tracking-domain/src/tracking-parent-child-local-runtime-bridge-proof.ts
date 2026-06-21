import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
const TrackingParentChildLocalRuntimeBridgeCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingParentChildLocalRuntimeBridgeRefSchema = brandedNonEmptyStringSchema('TrackingParentChildLocalRuntimeBridgeRef');

export const TrackingParentChildLocalRuntimeBridgeRowIdSchema = brandedNonEmptyStringSchema('TrackingParentChildLocalRuntimeBridgeRowId');

export const TrackingParentChildLocalRuntimeBridgeStatusSchema = Schema.Literal(
  'local-parent-child-runtime-observed-physical-child-runtime-required'
);

export const RequiredTrackingParentChildLocalRuntimeBridgeSourceRefs = [
  'test-results/eventing-parent-child-runtime-proof/proof.json',
  'output/eventing-plan-proof/51-54-parent-child-runtime/proof-summary.json',
] as const;

export const RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs = [
  'parent-action-received',
  'parent-command-validated',
  'parent-child-command-forward-requested',
  'parent-child-command-forwarded',
  'child-command-received',
  'child-command-accepted',
  'child-capability-state-updated',
  'child-runtime-health-updated',
  'parent-read-model-projected',
] as const;

export const TrackingParentChildLocalRuntimeBridgeInputSchema = withParser(
  Schema.Struct({
    eventingProofRef: TrackingParentChildLocalRuntimeBridgeRefSchema,
    eventingRowProofRef: TrackingParentChildLocalRuntimeBridgeRefSchema,
    runtimeSourceRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(3)),
    phaseRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(9)),
    publishReportCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
    storedEventCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
    deadLetterCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
    childAgentPhaseCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
    parentReadModelProjectionObserved: Schema.Boolean,
    typedLocalServiceTransportObserved: Schema.Boolean,
  })
);

const TrackingParentChildLocalRuntimeBridgeRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingParentChildLocalRuntimeBridgeRowIdSchema,
  generatedAt: ParentTimestampSchema,
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  status: TrackingParentChildLocalRuntimeBridgeStatusSchema,
  sourceProofRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(2)),
  eventingProofRef: TrackingParentChildLocalRuntimeBridgeRefSchema,
  eventingRowProofRef: TrackingParentChildLocalRuntimeBridgeRefSchema,
  runtimeSourceRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(3)),
  phaseRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(9)),
  publishReportCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
  storedEventCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
  deadLetterCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
  childAgentPhaseCount: TrackingParentChildLocalRuntimeBridgeCountSchema,
  parentReadModelProjectionObserved: Schema.Literal(true),
  typedLocalServiceTransportObserved: Schema.Literal(true),
  localParentChildRuntimeObserved: Schema.Literal(true),
  missingProofReasonRefs: Schema.Array(TrackingParentChildLocalRuntimeBridgeRefSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema).pipe(Schema.minItems(1)),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
  parentReceiptRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingParentChildLocalRuntimeBridgeRowSchema = withParser(
  TrackingParentChildLocalRuntimeBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingParentChildLocalRuntimeBridgeRowIsHonest(row) ||
        'Expected tracking parent-child local runtime bridge rows to prove local event handoff without physical child-device/product claims'
    )
  )
);

export const TrackingParentChildLocalRuntimeBridgeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-parent-child-local-runtime-bridge-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingParentChildLocalRuntimeBridgeRowSchema).pipe(Schema.minItems(1)),
    proofClaims: Schema.Struct({
      localParentChildRuntimeObserved: Schema.Literal(true),
      typedLocalServiceTransportObserved: Schema.Literal(true),
      parentReadModelProjectionObserved: Schema.Literal(true),
      physicalChildRuntimeStillRequired: Schema.Literal(true),
      noChildDeviceDeliveryRuntimeClaim: Schema.Literal(true),
      noChildDeviceExecutionRuntimeClaim: Schema.Literal(true),
      noRenderedChildDeviceUiRuntimeClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      localParentChildRuntimeObserved: Schema.Literal(true),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
      renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
      parentReceiptRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.every((row) => row.sourceProofRefs.length >= 2 && row.missingProofReasonRefs.length > 0) ||
        'Expected tracking parent-child local runtime bridge proof to preserve source refs and physical-runtime blockers'
    )
  )
);

export type TrackingParentChildLocalRuntimeBridgeInput = Infer<typeof TrackingParentChildLocalRuntimeBridgeInputSchema>;
export type TrackingParentChildLocalRuntimeBridgeProof = Infer<typeof TrackingParentChildLocalRuntimeBridgeProofSchema>;
type TrackingParentChildLocalRuntimeBridgeRowInput = Infer<typeof TrackingParentChildLocalRuntimeBridgeRowBaseSchema>;

export function buildTrackingParentChildLocalRuntimeBridgeProof(
  generatedAt: string,
  input: TrackingParentChildLocalRuntimeBridgeInput
): TrackingParentChildLocalRuntimeBridgeProof {
  const parsedInput = TrackingParentChildLocalRuntimeBridgeInputSchema.parse(input);

  return TrackingParentChildLocalRuntimeBridgeProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-parent-child-local-runtime-bridge-proof',
    generatedAt,
    rows: [bridgeRow(generatedAt, parsedInput)],
    proofClaims: {
      localParentChildRuntimeObserved: true,
      typedLocalServiceTransportObserved: true,
      parentReadModelProjectionObserved: true,
      physicalChildRuntimeStillRequired: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noChildDeviceExecutionRuntimeClaim: true,
      noRenderedChildDeviceUiRuntimeClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      localParentChildRuntimeObserved: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function bridgeRow(
  generatedAt: string,
  input: TrackingParentChildLocalRuntimeBridgeInput
): TrackingParentChildLocalRuntimeBridgeRowInput {
  return TrackingParentChildLocalRuntimeBridgeRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-parent-child-local-runtime-bridge',
    generatedAt,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'local-parent-child-runtime-observed-physical-child-runtime-required',
    sourceProofRefs: [input.eventingProofRef, input.eventingRowProofRef],
    eventingProofRef: input.eventingProofRef,
    eventingRowProofRef: input.eventingRowProofRef,
    runtimeSourceRefs: input.runtimeSourceRefs,
    phaseRefs: input.phaseRefs,
    publishReportCount: input.publishReportCount,
    storedEventCount: input.storedEventCount,
    deadLetterCount: input.deadLetterCount,
    childAgentPhaseCount: input.childAgentPhaseCount,
    parentReadModelProjectionObserved: input.parentReadModelProjectionObserved,
    typedLocalServiceTransportObserved: input.typedLocalServiceTransportObserved,
    localParentChildRuntimeObserved: true,
    missingProofReasonRefs: [
      'physical-child-device-delivery-runtime-required',
      'rendered-child-device-ui-runtime-required',
      'parent-receipt-runtime-required',
      'physical-device-observation-required',
    ],
    auditRefs: ['tracking-parent-child-local-runtime-bridge-audit'],
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    parentReceiptRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function trackingParentChildLocalRuntimeBridgeRowIsHonest(row: TrackingParentChildLocalRuntimeBridgeRowInput): boolean {
  return (
    row.sourceProofRefs.includes(row.eventingProofRef) &&
    row.sourceProofRefs.includes(row.eventingRowProofRef) &&
    RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs.every((phaseRef) =>
      row.phaseRefs.some((rowPhaseRef) => rowPhaseRef === phaseRef)
    ) &&
    row.publishReportCount >= row.phaseRefs.length &&
    row.storedEventCount >= row.phaseRefs.length &&
    row.deadLetterCount === 0 &&
    row.childAgentPhaseCount >= 4 &&
    row.parentReadModelProjectionObserved === true &&
    row.typedLocalServiceTransportObserved === true &&
    row.localParentChildRuntimeObserved === true &&
    row.missingProofReasonRefs.length > 0 &&
    trackingParentChildLocalRuntimeBridgeNonClaimsAreHonest(row)
  );
}

function trackingParentChildLocalRuntimeBridgeNonClaimsAreHonest(
  row: TrackingParentChildLocalRuntimeBridgeRowInput
): boolean {
  return (
    row.childDeviceDeliveryRuntimeClaimed === false &&
    row.childDeviceExecutionRuntimeClaimed === false &&
    row.renderedChildDeviceUiRuntimeClaimed === false &&
    row.parentReceiptRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.productionWorkerClaimed === false &&
    row.productClaimReady === false
  );
}

