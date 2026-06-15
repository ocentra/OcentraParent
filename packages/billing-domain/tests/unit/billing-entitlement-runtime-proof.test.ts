import { describe, expect, it } from 'vitest';
import {
  BillingEntitlementRuntimeDeviceLimitConsumptionSchema,
  BillingEntitlementRuntimeFailureConsumptionSchema,
  BillingEntitlementRuntimeProofReadModel,
  BillingEntitlementRuntimeProofSchema,
  BillingEntitlementRuntimeSnapshotConsumptionSchema,
  summarizeBillingEntitlementRuntimeConsumptionStates,
  summarizeBillingEntitlementRuntimeSnapshotStates,
} from '../../src/billing-entitlement-runtime-proof';

describe('billing entitlement runtime proof', () => {
  acceptsRuntimeStatusConsumptionProof();
  rejectsDegradedSnapshotConsumptionWithoutFailureState();
  rejectsDeniedDeviceLimitConsumptionThatDoesNotBlockNewDevice();
  rejectsFailureConsumptionThatDropsLocalSafetyOrCustodyBoundary();
  rejectsRuntimeProofOverclaims();
});

function acceptsRuntimeStatusConsumptionProof(): void {
  it('accepts entitlement snapshot device-limit and failure consumption without provider or production claims', () => {
    const proof = BillingEntitlementRuntimeProofSchema.parse(BillingEntitlementRuntimeProofReadModel);

    expect(summarizeBillingEntitlementRuntimeSnapshotStates(proof.snapshotConsumptions)).toEqual({
      'snapshot-active': 1,
      'snapshot-stale': 1,
      'payment-required': 1,
      'provider-unavailable': 1,
      'manual-review': 0,
    });
    expect(summarizeBillingEntitlementRuntimeConsumptionStates(proof.deviceLimitConsumptions)).toEqual({
      'accepted-local': 1,
      'accepted-grace': 1,
      'blocked-new-device': 1,
      'manual-required': 1,
      'unavailable-local-safety': 0,
    });
    expect(proof.failureConsumptions.map((row) => row.failureState.failureKind)).toEqual([
      'provider-unavailable',
      'stale-snapshot',
      'payment-required',
      'validation-failed',
    ]);
    expect(proof.nonClaims).toEqual([
      'no-stripe-sdk',
      'no-live-provider-execution',
      'no-provider-contact',
      'no-refund-credit-runtime',
      'no-child-activity-custody',
      'no-production-billing-claim',
      'no-portal-ui',
    ]);
  });
}

function rejectsDegradedSnapshotConsumptionWithoutFailureState(): void {
  it('rejects degraded entitlement snapshot runtime rows without consumed failure state', () => {
    const staleRow = requiredSnapshotRow('snapshot-stale');

    expect(
      BillingEntitlementRuntimeSnapshotConsumptionSchema.safeParse({
        ...staleRow,
        failureState: null,
      }).success
    ).toBe(false);
  });
}

function rejectsDeniedDeviceLimitConsumptionThatDoesNotBlockNewDevice(): void {
  it('rejects denied device-limit decisions consumed as accepted runtime state', () => {
    const deniedRow = requiredDeviceLimitRow('blocked-new-device');

    expect(
      BillingEntitlementRuntimeDeviceLimitConsumptionSchema.safeParse({
        ...deniedRow,
        consumptionState: 'accepted-local',
      }).success
    ).toBe(false);
  });
}

function rejectsFailureConsumptionThatDropsLocalSafetyOrCustodyBoundary(): void {
  it('rejects billing failure consumption that drops local safety continuation or adds child custody', () => {
    const providerFailure = requiredFailureConsumption('provider-unavailable');

    expect(
      BillingEntitlementRuntimeFailureConsumptionSchema.safeParse({
        ...providerFailure,
        failureState: {
          ...providerFailure.failureState,
          existingLocalSafetyContinues: false,
        },
      }).success
    ).toBe(false);
    expect(
      BillingEntitlementRuntimeFailureConsumptionSchema.safeParse({
        ...providerFailure,
        childActivityCustody: 'included',
      }).success
    ).toBe(false);
  });
}

function rejectsRuntimeProofOverclaims(): void {
  it('rejects proof overclaims for provider execution contact refund credit production billing or portal UI', () => {
    const proof = BillingEntitlementRuntimeProofReadModel;

    for (const invalidProof of [
      { ...proof, providerExecutionClaim: 'implemented' },
      { ...proof, providerContactClaim: 'implemented' },
      { ...proof, refundCreditClaim: 'implemented' },
      { ...proof, productionBillingClaim: 'claimed' },
      { ...proof, portalUiClaim: 'implemented' },
      { ...proof, nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-contact') },
    ]) {
      expect(BillingEntitlementRuntimeProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function requiredSnapshotRow(runtimeState: 'snapshot-stale') {
  const row = BillingEntitlementRuntimeProofReadModel.snapshotConsumptions.find(
    (entry) => entry.runtimeState === runtimeState
  );
  if (row === undefined) {
    throw new Error(`missing snapshot runtime state: ${runtimeState}`);
  }
  return row;
}

function requiredDeviceLimitRow(consumptionState: 'blocked-new-device') {
  const row = BillingEntitlementRuntimeProofReadModel.deviceLimitConsumptions.find(
    (entry) => entry.consumptionState === consumptionState
  );
  if (row === undefined) {
    throw new Error(`missing device-limit runtime state: ${consumptionState}`);
  }
  return row;
}

function requiredFailureConsumption(failureKind: 'provider-unavailable') {
  const row = BillingEntitlementRuntimeProofReadModel.failureConsumptions.find(
    (entry) => entry.failureState.failureKind === failureKind
  );
  if (row === undefined) {
    throw new Error(`missing failure consumption: ${failureKind}`);
  }
  return row;
}
