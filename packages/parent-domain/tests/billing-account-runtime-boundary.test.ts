import { describe, expect, it } from 'vitest';
import {
  BillingAccountRuntimeBoundaryProofSchema,
  BillingAccountRuntimeEntitlementSigningBoundarySchema,
  BillingAccountRuntimeOperationRowSchema,
  BillingAccountRuntimeStatusRowSchema,
  summarizeBillingAccountRuntimeOperations,
  summarizeBillingAccountRuntimeStatuses,
} from '../src/billing-account-runtime-boundary';
import { BillingAccountRuntimeBoundaryProofReadModel } from '../src/billing-account-runtime-boundary-proof';

describe('billing account runtime boundary', () => {
  acceptsBillingAccountRuntimeBoundaryProof();
  rejectsUnavailableAccountRowsWithoutFailureState();
  rejectsAvailableRuntimeRowsFromUnavailableSources();
  rejectsEntitlementSigningGapsWithoutManualRequiredContext();
  rejectsRuntimeOperationOverclaims();
});

function acceptsBillingAccountRuntimeBoundaryProof(): void {
  it('accepts account backend runtime proof without provider secrets portal UI or child-device claims', () => {
    const proof = BillingAccountRuntimeBoundaryProofSchema.parse(BillingAccountRuntimeBoundaryProofReadModel);

    expect(summarizeBillingAccountRuntimeStatuses(proof.accountStatusRows)).toEqual({
      trialing: 0,
      active: 1,
      'past-due': 1,
      'backend-unavailable': 1,
      'provider-unavailable': 1,
      'manual-review': 1,
    });
    expect(summarizeBillingAccountRuntimeOperations(proof.runtimeOperations)).toEqual({
      'account-status-read': 1,
      'subscription-status-read': 1,
      'entitlement-snapshot-read': 1,
      'device-limit-decision-read': 1,
      'download-status-read': 1,
      'provider-webhook-sync': 1,
    });
    expect(proof.nonClaims).toEqual([
      'no-stripe-sdk',
      'no-provider-secrets',
      'no-billing-provider-runtime',
      'no-account-backend',
      'no-entitlement-signing-runtime',
      'no-portal-ui',
      'no-child-device-consumption',
      'no-child-activity-custody',
    ]);
    expect(proof.entitlementSigningBoundary.failureState).toEqual(requiredFailure('validation-failed'));
  });
}

function rejectsUnavailableAccountRowsWithoutFailureState(): void {
  it('rejects unavailable account runtime rows without parent-visible failure state', () => {
    const providerUnavailableRow = requiredAccountStatusRow('provider-unavailable');

    expect(
      BillingAccountRuntimeStatusRowSchema.safeParse({
        ...providerUnavailableRow,
        failureState: null,
      }).success
    ).toBe(false);
  });
}

function rejectsAvailableRuntimeRowsFromUnavailableSources(): void {
  it('rejects available runtime rows from unavailable source state', () => {
    const backendUnavailableRow = requiredAccountStatusRow('backend-unavailable');

    expect(
      BillingAccountRuntimeStatusRowSchema.safeParse({
        ...backendUnavailableRow,
        backendRuntimeState: 'available',
      }).success
    ).toBe(false);
  });
}

function rejectsEntitlementSigningGapsWithoutManualRequiredContext(): void {
  it('rejects entitlement signing gaps without manual-required state and failure context', () => {
    const signingBoundary = BillingAccountRuntimeBoundaryProofReadModel.entitlementSigningBoundary;

    expect(
      BillingAccountRuntimeEntitlementSigningBoundarySchema.safeParse({
        ...signingBoundary,
        manualRequired: false,
      }).success
    ).toBe(false);
    expect(
      BillingAccountRuntimeEntitlementSigningBoundarySchema.safeParse({
        ...signingBoundary,
        failureState: null,
      }).success
    ).toBe(false);
  });
}

function rejectsRuntimeOperationOverclaims(): void {
  it('rejects billing runtime operation rows that overclaim provider custody or child-device consumption', () => {
    const webhookOperation = requiredRuntimeOperation('provider-webhook-sync');

    expect(
      BillingAccountRuntimeOperationRowSchema.safeParse({
        ...webhookOperation,
        providerSecretCustody: 'stored-in-portal',
      }).success
    ).toBe(false);
    expect(
      BillingAccountRuntimeOperationRowSchema.safeParse({
        ...webhookOperation,
        childDeviceConsumption: 'implemented',
      }).success
    ).toBe(false);
  });
}

function requiredAccountStatusRow(accountStatus: 'backend-unavailable' | 'provider-unavailable') {
  const row = BillingAccountRuntimeBoundaryProofReadModel.accountStatusRows.find(
    (entry) => entry.accountStatus === accountStatus
  );
  if (row === undefined) {
    throw new Error(`missing billing account status row: ${accountStatus}`);
  }
  return row;
}

function requiredRuntimeOperation(operation: 'provider-webhook-sync') {
  const row = BillingAccountRuntimeBoundaryProofReadModel.runtimeOperations.find(
    (entry) => entry.operation === operation
  );
  if (row === undefined) {
    throw new Error(`missing billing runtime operation: ${operation}`);
  }
  return row;
}

function requiredFailure(failureKind: 'validation-failed') {
  const failure = BillingAccountRuntimeBoundaryProofReadModel.failureStates.find(
    (entry) => entry.failureKind === failureKind
  );
  if (failure === undefined) {
    throw new Error(`missing billing failure state: ${failureKind}`);
  }
  return failure;
}
