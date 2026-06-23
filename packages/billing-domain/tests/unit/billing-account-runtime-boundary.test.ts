import { describe, expect, it } from 'vitest';
import {
  BillingAccountRuntimeBoundaryProofSchema,
  BillingAccountRuntimeEntitlementSigningBoundarySchema,
  BillingAccountRuntimeOperationRowSchema,
  BillingAccountRuntimeStatusRowSchema,
} from '@ocentra-parent/schema-domain/billing-account-runtime-boundary';
import { BillingAccountRuntimeBoundaryProofReadModel } from '@ocentra-parent/schema-domain/billing-account-runtime-boundary-proof';

describe('billing account runtime boundary', () => {
  acceptsBillingAccountRuntimeBoundaryProof();
  rejectsUnavailableAccountRowsWithoutFailureState();
  rejectsAvailableRuntimeRowsFromUnavailableSources();
  rejectsProviderLifecycleMismatches();
  rejectsEntitlementSigningGapsWithoutManualRequiredContext();
  rejectsRuntimeOperationOverclaims();
  rejectsProofWithoutSignedChildConsumptionClaim();
});

function acceptsBillingAccountRuntimeBoundaryProof(): void {
  it('accepts account runtime proof with signed child snapshot consumption and no provider secrets or portal UI claim', () => {
    const proof = BillingAccountRuntimeBoundaryProofSchema.parse(BillingAccountRuntimeBoundaryProofReadModel);
    const activeRow = requiredAccountStatusRow('active');
    const manualReviewRow = requiredAccountStatusRow('manual-review');

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
      'no-child-activity-custody',
    ]);
    expect(requiredRuntimeOperation('entitlement-snapshot-read').childDeviceConsumption).toBe(
      'signed-snapshot-consumed'
    );
    expect(proof.childDeviceConsumptionClaim).toBe('signed-snapshot-consumption-contract');
    expect(proof.entitlementSigningBoundary.failureState).toEqual(requiredFailure('validation-failed'));
    expect(activeRow.providerMode).toBe('stripe-hosted');
    expect(activeRow.nextRenewalAt).toBe('2026-07-14T00:00:00.000Z');
    expect(activeRow.manualInvoiceState).toEqual({
      visible: false,
      invoiceState: null,
    });
    expect(manualReviewRow.providerMode).toBe('manual-invoice');
    expect(manualReviewRow.nextRenewalAt).toBeNull();
    expect(manualReviewRow.manualInvoiceState).toEqual({
      visible: true,
      invoiceState: 'manual-support-required',
    });
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

function rejectsProviderLifecycleMismatches(): void {
  it('rejects manual invoice and Stripe-hosted rows that advertise the wrong renewal or manual invoice state', () => {
    const activeRow = requiredAccountStatusRow('active');
    const manualReviewRow = requiredAccountStatusRow('manual-review');

    expect(
      BillingAccountRuntimeStatusRowSchema.safeParse({
        ...activeRow,
        nextRenewalAt: null,
      }).success
    ).toBe(false);
    expect(
      BillingAccountRuntimeStatusRowSchema.safeParse({
        ...activeRow,
        manualInvoiceState: {
          visible: true,
          invoiceState: 'manual-support-required',
        },
      }).success
    ).toBe(false);
    expect(
      BillingAccountRuntimeStatusRowSchema.safeParse({
        ...manualReviewRow,
        nextRenewalAt: '2026-07-14T00:00:00.000Z',
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
        childDeviceConsumption: 'signed-snapshot-consumed',
      }).success
    ).toBe(false);
    expect(
      BillingAccountRuntimeOperationRowSchema.safeParse({
        ...requiredRuntimeOperation('entitlement-snapshot-read'),
        childDeviceConsumption: 'not-implemented',
      }).success
    ).toBe(false);
  });
}

function rejectsProofWithoutSignedChildConsumptionClaim(): void {
  it('rejects billing runtime proof that omits the signed child consumption contract claim', () => {
    expect(
      BillingAccountRuntimeBoundaryProofSchema.safeParse({
        ...BillingAccountRuntimeBoundaryProofReadModel,
        childDeviceConsumptionClaim: 'not-supported',
      }).success
    ).toBe(false);
  });
}

function requiredAccountStatusRow(
  accountStatus: 'active' | 'backend-unavailable' | 'provider-unavailable' | 'manual-review'
) {
  const row = BillingAccountRuntimeBoundaryProofReadModel.accountStatusRows.find(
    (entry) => entry.accountStatus === accountStatus
  );
  if (row === undefined) {
    throw new Error(`missing billing account status row: ${accountStatus}`);
  }
  return row;
}

function requiredRuntimeOperation(operation: 'provider-webhook-sync' | 'entitlement-snapshot-read') {
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

function summarizeBillingAccountRuntimeStatuses(
  rows: ReadonlyArray<{ readonly accountStatus: string }>
): Record<
  'trialing' | 'active' | 'past-due' | 'backend-unavailable' | 'provider-unavailable' | 'manual-review',
  number
> {
  return countKnownValues(
    ['trialing', 'active', 'past-due', 'backend-unavailable', 'provider-unavailable', 'manual-review'],
    rows,
    'accountStatus'
  );
}

function summarizeBillingAccountRuntimeOperations(
  rows: ReadonlyArray<{ readonly operation: string }>
): Record<
  | 'account-status-read'
  | 'subscription-status-read'
  | 'entitlement-snapshot-read'
  | 'device-limit-decision-read'
  | 'download-status-read'
  | 'provider-webhook-sync',
  number
> {
  return countKnownValues(
    [
      'account-status-read',
      'subscription-status-read',
      'entitlement-snapshot-read',
      'device-limit-decision-read',
      'download-status-read',
      'provider-webhook-sync',
    ],
    rows,
    'operation'
  );
}

function countKnownValues<const Value extends string, const Key extends string>(
  values: readonly Value[],
  rows: ReadonlyArray<{ readonly [Field in Key]: Value }>,
  key: Key
): Record<Value, number> {
  const counts = {} as Record<Value, number>;
  for (const value of values) {
    counts[value] = 0;
  }
  for (const row of rows) {
    counts[row[key]] += 1;
  }
  return counts;
}
