import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BillingDeviceLimitDecisionSchema,
  BillingEntitlementSnapshotSchema,
  BillingFailureStateSchema,
  type BillingDeviceLimitDecision,
  type BillingEntitlementSnapshot,
  type BillingFailureState,
} from './billing-entitlement';
import { BillingEntitlementContractProofReadModel } from './billing-entitlement-proof';
import {
  BillingChildActivityCustodyClaimSchema,
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingStripeSdkClaimSchema,
} from './billing-entitlement-values';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingEntitlementRuntimeAuditReferenceSchema,
  BillingEntitlementRuntimeBoundaryIdSchema,
  BillingEntitlementRuntimeChildCustodyClaimSchema,
  BillingEntitlementRuntimeConsumptionStateSchema,
  BillingEntitlementRuntimeNonClaimSchema,
  BillingEntitlementRuntimeOperationSchema,
  BillingEntitlementRuntimePortalUiClaimSchema,
  BillingEntitlementRuntimeProductionBillingClaimSchema,
  BillingEntitlementRuntimeProviderContactClaimSchema,
  BillingEntitlementRuntimeProviderExecutionClaimSchema,
  BillingEntitlementRuntimeRefundCreditClaimSchema,
  BillingEntitlementRuntimeSchemaVersionSchema,
  BillingEntitlementRuntimeSnapshotStateSchema,
  BillingEntitlementRuntimeSourceSchema,
  summarizeBillingEntitlementRuntimeConsumptionStates,
  summarizeBillingEntitlementRuntimeSnapshotStates,
  type BillingEntitlementRuntimeConsumptionState,
  type BillingEntitlementRuntimeNonClaim,
  type BillingEntitlementRuntimeOperation,
  type BillingEntitlementRuntimeSnapshotState,
} from './billing-entitlement-runtime-proof-values';

export * from './billing-entitlement-runtime-proof-values';

const Timestamp = '2026-06-04T23:34:57.000Z';
const ExpiryTimestamp = '2026-06-11T23:34:57.000Z';
const RetryTimestamp = '2026-06-05T00:34:57.000Z';

const RuntimeProviderUnavailableFailure = runtimeFailureState(
  'provider-unavailable',
  'unavailable',
  'local-only',
  'wait-for-provider',
  true,
  RetryTimestamp
);
const RuntimeStaleSnapshotFailure = runtimeFailureState(
  'stale-snapshot',
  'stale',
  'grace-with-local-safety',
  'wait-for-provider',
  true,
  RetryTimestamp
);
const RuntimePaymentRequiredFailure = runtimeFailureState(
  'payment-required',
  'past-due',
  'grace-with-local-safety',
  'payment-update',
  true,
  null
);
const RuntimeValidationFailure = runtimeFailureState(
  'validation-failed',
  'manual-review',
  'manual-review-with-local-safety',
  'manual-support-review',
  false,
  null
);

export const BillingEntitlementRuntimeSnapshotConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    runtimeState: BillingEntitlementRuntimeSnapshotStateSchema,
    source: BillingEntitlementRuntimeSourceSchema,
    entitlementSnapshot: BillingEntitlementSnapshotSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.runtimeState === 'snapshot-active' ||
        row.failureState !== null ||
        'Expected non-active entitlement runtime rows to carry consumed billing failure state'
    ),
    Schema.filter(
      (row) =>
        row.source !== 'unavailable' ||
        row.entitlementSnapshot.source === 'unavailable' ||
        'Expected unavailable runtime source to consume an unavailable entitlement snapshot'
    )
  )
);

export const BillingEntitlementRuntimeDeviceLimitConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    deviceLimitDecision: BillingDeviceLimitDecisionSchema,
    consumptionState: BillingEntitlementRuntimeConsumptionStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.deviceLimitDecision.decision !== 'denied' ||
        row.consumptionState === 'blocked-new-device' ||
        'Expected denied device-limit decisions to be consumed as blocked-new-device runtime state'
    ),
    Schema.filter(
      (row) =>
        row.deviceLimitDecision.decision === 'allowed' ||
        row.failureState !== null ||
        row.consumptionState === 'manual-required' ||
        'Expected non-allowed device-limit decisions to carry failure or manual runtime context'
    )
  )
);

export const BillingEntitlementRuntimeFailureConsumptionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    boundaryId: BillingEntitlementRuntimeBoundaryIdSchema,
    operation: BillingEntitlementRuntimeOperationSchema,
    failureState: BillingFailureStateSchema,
    consumedFor: Schema.Array(BillingEntitlementRuntimeOperationSchema),
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    auditReference: BillingEntitlementRuntimeAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.failureState.existingLocalSafetyContinues ||
        'Expected consumed billing failure state to keep local safety continuation explicit'
    ),
    Schema.filter(
      (row) =>
        row.consumedFor.length > 0 || 'Expected consumed billing failure state to name the runtime boundary using it'
    )
  )
);

export const BillingEntitlementRuntimeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementRuntimeSchemaVersionSchema,
    snapshotConsumptions: Schema.Array(BillingEntitlementRuntimeSnapshotConsumptionSchema),
    deviceLimitConsumptions: Schema.Array(BillingEntitlementRuntimeDeviceLimitConsumptionSchema),
    failureConsumptions: Schema.Array(BillingEntitlementRuntimeFailureConsumptionSchema),
    nonClaims: Schema.Array(BillingEntitlementRuntimeNonClaimSchema),
    stripeSdkClaim: BillingStripeSdkClaimSchema,
    providerExecutionClaim: BillingEntitlementRuntimeProviderExecutionClaimSchema,
    providerContactClaim: BillingEntitlementRuntimeProviderContactClaimSchema,
    refundCreditClaim: BillingEntitlementRuntimeRefundCreditClaimSchema,
    productionBillingClaim: BillingEntitlementRuntimeProductionBillingClaimSchema,
    portalUiClaim: BillingEntitlementRuntimePortalUiClaimSchema,
    childCustodyClaim: BillingEntitlementRuntimeChildCustodyClaimSchema,
    childActivityCustodyClaim: BillingChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingEntitlementRuntimeProofIsHonest(proof) ||
        'Expected billing entitlement runtime proof to keep provider contact refund credit child custody and production claims explicit'
    )
  )
);

export type BillingEntitlementRuntimeSnapshotConsumption = Infer<
  typeof BillingEntitlementRuntimeSnapshotConsumptionSchema
>;
export type BillingEntitlementRuntimeDeviceLimitConsumption = Infer<
  typeof BillingEntitlementRuntimeDeviceLimitConsumptionSchema
>;
export type BillingEntitlementRuntimeFailureConsumption = Infer<
  typeof BillingEntitlementRuntimeFailureConsumptionSchema
>;
export type BillingEntitlementRuntimeProof = Infer<typeof BillingEntitlementRuntimeProofSchema>;

export const decodeBillingEntitlementRuntimeProof = Schema.decodeUnknownSync(BillingEntitlementRuntimeProofSchema);

export const BillingEntitlementRuntimeProofReadModel = BillingEntitlementRuntimeProofSchema.parse({
  schemaVersion: 'billing-entitlement-runtime-proof',
  snapshotConsumptions: [
    snapshotConsumption('runtime-snapshot-active', 'snapshot-active', 'signed-local-snapshot', activeSnapshot(), null),
    snapshotConsumption(
      'runtime-snapshot-stale',
      'snapshot-stale',
      'signed-local-snapshot',
      degradedSnapshot('expired', 'signed-local-snapshot', 'schema-valid-local', RuntimeStaleSnapshotFailure),
      RuntimeStaleSnapshotFailure
    ),
    snapshotConsumption(
      'runtime-snapshot-payment-required',
      'payment-required',
      'signed-local-snapshot',
      degradedSnapshot('past-due', 'signed-local-snapshot', 'schema-valid-local', RuntimePaymentRequiredFailure),
      RuntimePaymentRequiredFailure
    ),
    snapshotConsumption(
      'runtime-snapshot-provider-unavailable',
      'provider-unavailable',
      'unavailable',
      degradedSnapshot('unavailable', 'unavailable', 'unavailable', RuntimeProviderUnavailableFailure),
      RuntimeProviderUnavailableFailure
    ),
  ],
  deviceLimitConsumptions: [
    deviceLimitConsumption('runtime-device-allowed', 'allowed', 'accepted-local', null),
    deviceLimitConsumption('runtime-device-denied', 'denied', 'blocked-new-device', RuntimePaymentRequiredFailure),
    deviceLimitConsumption('runtime-device-grace', 'grace', 'accepted-grace', RuntimeStaleSnapshotFailure),
    deviceLimitConsumption('runtime-device-manual', 'manual-review', 'manual-required', RuntimeValidationFailure),
  ],
  failureConsumptions: [
    failureConsumption(
      'runtime-failure-provider-unavailable',
      RuntimeProviderUnavailableFailure,
      'unavailable-local-safety',
      ['account-entitlement-snapshot-consumption', 'billing-failure-state-consumption']
    ),
    failureConsumption('runtime-failure-stale-snapshot', RuntimeStaleSnapshotFailure, 'accepted-grace', [
      'account-entitlement-snapshot-consumption',
      'device-limit-decision-consumption',
    ]),
    failureConsumption('runtime-failure-payment-required', RuntimePaymentRequiredFailure, 'blocked-new-device', [
      'account-entitlement-snapshot-consumption',
      'device-limit-decision-consumption',
    ]),
    failureConsumption('runtime-failure-validation-failed', RuntimeValidationFailure, 'manual-required', [
      'billing-failure-state-consumption',
    ]),
  ],
  nonClaims: [
    'no-stripe-sdk',
    'no-live-provider-execution',
    'no-provider-contact',
    'no-refund-credit-runtime',
    'no-child-activity-custody',
    'no-production-billing-claim',
    'no-portal-ui',
  ],
  stripeSdkClaim: 'not-included',
  providerExecutionClaim: 'not-implemented',
  providerContactClaim: 'manual-required',
  refundCreditClaim: 'manual-required',
  productionBillingClaim: 'not-claimed',
  portalUiClaim: 'not-implemented',
  childCustodyClaim: 'signed-snapshot-consumption-contract',
  childActivityCustodyClaim: 'not-included',
  updatedAt: Timestamp,
});

export const BillingEntitlementRuntimeProof = BillingEntitlementRuntimeProofReadModel;

export const BillingEntitlementRuntimeKnownGaps = [
  'Stripe/live provider execution remains unimplemented.',
  'Provider contact, refund, and credit actions remain manual-required support/admin states.',
  'Child-device entitlement runtime is limited to signed local snapshot consumption and does not execute live provider delivery.',
  'Portal billing UI and production subscription support remain unclaimed.',
] as const;

export { summarizeBillingEntitlementRuntimeConsumptionStates, summarizeBillingEntitlementRuntimeSnapshotStates };

function billingEntitlementRuntimeProofIsHonest(proof: {
  readonly snapshotConsumptions: ReadonlyArray<{
    readonly runtimeState: BillingEntitlementRuntimeSnapshotState;
    readonly failureState: BillingFailureState | null;
    readonly childActivityCustody: 'not-included';
  }>;
  readonly deviceLimitConsumptions: ReadonlyArray<{
    readonly deviceLimitDecision: { readonly decision: string; readonly reasonCode: string };
    readonly consumptionState: BillingEntitlementRuntimeConsumptionState;
    readonly childActivityCustody: 'not-included';
  }>;
  readonly failureConsumptions: ReadonlyArray<{
    readonly failureState: { readonly failureKind: string; readonly existingLocalSafetyContinues: boolean };
    readonly childActivityCustody: 'not-included';
  }>;
  readonly nonClaims: ReadonlyArray<BillingEntitlementRuntimeNonClaim>;
  readonly childCustodyClaim: 'signed-snapshot-consumption-contract' | 'not-supported';
}): boolean {
  const requiredNonClaims: ReadonlyArray<BillingEntitlementRuntimeNonClaim> = [
    'no-stripe-sdk',
    'no-live-provider-execution',
    'no-provider-contact',
    'no-refund-credit-runtime',
    'no-child-activity-custody',
    'no-production-billing-claim',
    'no-portal-ui',
  ];
  const requiredRuntimeStates: ReadonlyArray<BillingEntitlementRuntimeSnapshotState> = [
    'snapshot-active',
    'snapshot-stale',
    'payment-required',
    'provider-unavailable',
  ];
  const requiredFailureKinds = ['provider-unavailable', 'stale-snapshot', 'payment-required', 'validation-failed'];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    requiredRuntimeStates.every((state) => proof.snapshotConsumptions.some((row) => row.runtimeState === state)) &&
    proof.snapshotConsumptions.every((row) => row.childActivityCustody === 'not-included') &&
    proof.deviceLimitConsumptions.some(
      (row) =>
        row.deviceLimitDecision.decision === 'denied' &&
        row.deviceLimitDecision.reasonCode === 'limit-exceeded' &&
        row.consumptionState === 'blocked-new-device'
    ) &&
    proof.deviceLimitConsumptions.every((row) => row.childActivityCustody === 'not-included') &&
    requiredFailureKinds.every((kind) =>
      proof.failureConsumptions.some((row) => row.failureState.failureKind === kind)
    ) &&
    proof.failureConsumptions.every(
      (row) => row.failureState.existingLocalSafetyContinues && row.childActivityCustody === 'not-included'
    ) &&
    proof.childCustodyClaim === 'signed-snapshot-consumption-contract'
  );
}

function activeSnapshot(): BillingEntitlementSnapshot {
  return BillingEntitlementContractProofReadModel.entitlementSnapshot;
}

function degradedSnapshot(
  subscriptionStatus: 'past-due' | 'expired' | 'unavailable',
  source: 'signed-local-snapshot' | 'unavailable',
  signatureState: 'schema-valid-local' | 'unavailable',
  failureState: BillingFailureState
): BillingEntitlementSnapshot {
  return BillingEntitlementSnapshotSchema.parse({
    ...BillingEntitlementContractProofReadModel.entitlementSnapshot,
    snapshotId: `entitlement-runtime-${subscriptionStatus}`,
    subscriptionStatus,
    source,
    signatureState,
    expiresAt: ExpiryTimestamp,
    failureState,
  });
}

function snapshotConsumption(
  boundaryId:
    | 'runtime-snapshot-active'
    | 'runtime-snapshot-stale'
    | 'runtime-snapshot-payment-required'
    | 'runtime-snapshot-provider-unavailable',
  runtimeState: BillingEntitlementRuntimeSnapshotState,
  source: 'signed-local-snapshot' | 'unavailable',
  entitlementSnapshot: BillingEntitlementSnapshot,
  failureState: BillingFailureState | null
) {
  return {
    schemaVersion: 'billing-entitlement-runtime-proof',
    boundaryId,
    operation: 'account-entitlement-snapshot-consumption',
    runtimeState,
    source,
    entitlementSnapshot,
    localSafetyBehavior: failureState?.localSafetyBehavior ?? 'unchanged',
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    failureState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function deviceLimitConsumption(
  boundaryId: 'runtime-device-allowed' | 'runtime-device-denied' | 'runtime-device-grace' | 'runtime-device-manual',
  decision: 'allowed' | 'denied' | 'grace' | 'manual-review',
  consumptionState: BillingEntitlementRuntimeConsumptionState,
  failureState: BillingFailureState | null
) {
  return {
    schemaVersion: 'billing-entitlement-runtime-proof',
    boundaryId,
    operation: 'device-limit-decision-consumption',
    deviceLimitDecision: requiredDeviceLimitDecision(decision),
    consumptionState,
    localSafetyBehavior: failureState?.localSafetyBehavior ?? 'unchanged',
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    failureState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function failureConsumption(
  boundaryId:
    | 'runtime-failure-provider-unavailable'
    | 'runtime-failure-stale-snapshot'
    | 'runtime-failure-payment-required'
    | 'runtime-failure-validation-failed',
  failureState: BillingFailureState,
  consumptionState: BillingEntitlementRuntimeConsumptionState,
  consumedFor: ReadonlyArray<BillingEntitlementRuntimeOperation>
) {
  return {
    schemaVersion: 'billing-entitlement-runtime-proof',
    boundaryId,
    operation: 'billing-failure-state-consumption',
    failureState,
    consumedFor,
    localSafetyBehavior: failureState.localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    consumptionState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function requiredDeviceLimitDecision(decision: BillingDeviceLimitDecision['decision']): BillingDeviceLimitDecision {
  const row = BillingEntitlementContractProofReadModel.deviceLimitDecisions.find(
    (entry) => entry.decision === decision
  );
  if (row === undefined) {
    throw new Error(`missing billing device-limit decision: ${decision}`);
  }
  return row;
}

function runtimeFailureState(
  failureKind: 'provider-unavailable' | 'stale-snapshot' | 'payment-required' | 'validation-failed',
  parentVisibleState: 'unavailable' | 'stale' | 'past-due' | 'manual-review',
  localSafetyBehavior: 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  parentResolution: 'payment-update' | 'manual-support-review' | 'wait-for-provider',
  retryAllowed: boolean,
  retryAfter: typeof RetryTimestamp | null
): BillingFailureState {
  return BillingFailureStateSchema.parse({
    failureKind,
    parentVisibleState,
    localSafetyBehavior,
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution,
    retryAllowed,
    retryAfter,
  });
}
