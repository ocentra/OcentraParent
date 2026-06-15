import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  BillingAccountRuntimeBoundaryProofSchema,
  type BillingAccountRuntimeFailureState,
} from './billing-account-runtime-boundary';
import type {
  BillingAccountBackendRuntimeState,
  BillingAccountRuntimeOperation,
  BillingAccountRuntimeStatus,
} from './billing-account-runtime-boundary-values';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const decodeParentTimestamp = Schema.decodeUnknownSync(ParentTimestampSchema);
const Timestamp = decodeParentTimestamp('2026-06-03T13:39:17.000Z');
const RetryTimestamp = decodeParentTimestamp('2026-06-03T14:39:17.000Z');
const RenewalTimestamp = decodeParentTimestamp('2026-07-14T00:00:00.000Z');

const ProviderUnavailableFailure = unavailableFailure('provider-unavailable');
const NetworkUnavailableFailure = unavailableFailure('network-unavailable');
const PaymentRequiredFailure = billingFailureState(
  'payment-required',
  'past-due',
  'grace-with-local-safety',
  'payment-update',
  true,
  null
);
const ValidationFailedFailure = billingFailureState(
  'validation-failed',
  'manual-review',
  'manual-review-with-local-safety',
  'manual-support-review',
  false,
  null
);

export const BillingAccountRuntimeBoundaryProofReadModel = BillingAccountRuntimeBoundaryProofSchema.parse({
  schemaVersion: 'billing-account-runtime-boundary-proof',
  accountStatusRows: [
    accountStatusRow(
      'billing-account-runtime-active',
      'active',
      'active',
      'account-backend',
      'available',
      'available',
      'unchanged',
      'stripe-hosted',
      RenewalTimestamp,
      false,
      null
    ),
    accountStatusRow(
      'billing-account-runtime-past-due',
      'past-due',
      'past-due',
      'signed-local-snapshot',
      'manual-required',
      'past-due',
      'grace-with-local-safety',
      'stripe-hosted',
      RenewalTimestamp,
      false,
      PaymentRequiredFailure
    ),
    accountStatusRow(
      'billing-account-runtime-provider-unavailable',
      'provider-unavailable',
      'unavailable',
      'unavailable',
      'provider-unavailable',
      'unavailable',
      'local-only',
      'stripe-hosted',
      null,
      false,
      ProviderUnavailableFailure
    ),
    accountStatusRow(
      'billing-account-runtime-backend-unavailable',
      'backend-unavailable',
      'unavailable',
      'unavailable',
      'backend-unavailable',
      'unavailable',
      'local-only',
      'stripe-hosted',
      null,
      false,
      NetworkUnavailableFailure
    ),
    accountStatusRow(
      'billing-account-runtime-manual-review',
      'manual-review',
      'unknown',
      'manual-support-review',
      'manual-required',
      'manual-review',
      'manual-review-with-local-safety',
      'manual-invoice',
      null,
      true,
      ValidationFailedFailure
    ),
  ],
  runtimeOperations: [
    runtimeOperation('account-status-read', 'not-implemented', 'none', 'not-implemented', true, ProviderUnavailableFailure),
    runtimeOperation(
      'subscription-status-read',
      'not-implemented',
      'none',
      'not-implemented',
      true,
      ProviderUnavailableFailure
    ),
    runtimeOperation(
      'entitlement-snapshot-read',
      'manual-required',
      'none',
      'signed-snapshot-consumed',
      true,
      ValidationFailedFailure
    ),
    runtimeOperation(
      'device-limit-decision-read',
      'manual-required',
      'none',
      'manual-required',
      true,
      ValidationFailedFailure
    ),
    runtimeOperation('download-status-read', 'not-implemented', 'none', 'not-implemented', true, NetworkUnavailableFailure),
    runtimeOperation(
      'provider-webhook-sync',
      'not-implemented',
      'backend-reference-only',
      'not-implemented',
      true,
      ProviderUnavailableFailure
    ),
  ],
  entitlementSigningBoundary: {
    schemaVersion: 'billing-account-runtime-boundary-proof',
    signingState: 'manual-required',
    signedSnapshotAccepted: false,
    manualRequired: true,
    signingRuntimeClaim: 'not-implemented',
    failureState: ValidationFailedFailure,
  },
  failureStates: [
    ProviderUnavailableFailure,
    NetworkUnavailableFailure,
    PaymentRequiredFailure,
    ValidationFailedFailure,
  ],
  nonClaims: [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-runtime',
    'no-account-backend',
    'no-entitlement-signing-runtime',
    'no-portal-ui',
    'no-child-activity-custody',
  ],
  stripeSdkClaim: 'not-included',
  providerSecretClaim: 'not-included',
  accountBackendClaim: 'not-implemented',
  portalUiClaim: 'not-implemented',
  childDeviceConsumptionClaim: 'signed-snapshot-consumption-contract',
  childActivityCustodyClaim: 'not-included',
  updatedAt: Timestamp,
});

export const BillingAccountRuntimeBoundaryProof = BillingAccountRuntimeBoundaryProofReadModel;

export const BillingAccountRuntimeBoundaryKnownGaps = [
  'Billing account backend and provider runtime remain unimplemented.',
  'Stripe/provider secrets are not present and provider references stay behind a future backend boundary.',
  'Entitlement signing delivery runtime remains manual-required.',
  'Portal billing UI and account-management flows remain unimplemented.',
  'Child-device entitlement consumption is limited to signed local snapshots and does not contact providers.',
] as const;

function accountStatusRow(
  boundaryId:
    | 'billing-account-runtime-active'
    | 'billing-account-runtime-past-due'
    | 'billing-account-runtime-provider-unavailable'
    | 'billing-account-runtime-backend-unavailable'
    | 'billing-account-runtime-manual-review',
  accountStatus: BillingAccountRuntimeStatus,
  subscriptionStatus: 'active' | 'past-due' | 'unknown' | 'unavailable',
  source: 'account-backend' | 'signed-local-snapshot' | 'manual-support-review' | 'unavailable',
  backendRuntimeState: BillingAccountBackendRuntimeState,
  parentVisibleState: 'available' | 'past-due' | 'unavailable' | 'manual-review',
  localSafetyBehavior: 'unchanged' | 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  providerMode: 'stripe-hosted' | 'manual-invoice',
  nextRenewalAt: typeof RenewalTimestamp | null,
  manualInvoiceVisible: boolean,
  failureState: ReturnType<typeof billingFailureState> | null
) {
  return {
    schemaVersion: 'billing-account-runtime-boundary-proof',
    boundaryId,
    parentAccount: {
      parentAccountId: 'parent-account-runtime-boundary-proof-1',
    },
    family: {
      familyId: 'family-runtime-boundary-proof-1',
    },
    accountStatus,
    subscriptionStatus,
    source,
    backendRuntimeState,
    parentVisibleState,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    providerSecretCustody: 'not-present',
    providerMode,
    nextRenewalAt,
    manualInvoiceState: {
      visible: manualInvoiceVisible,
      invoiceState: manualInvoiceVisible ? 'manual-support-required' : null,
    },
    failureState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function runtimeOperation(
  operation: BillingAccountRuntimeOperation,
  backendRuntimeState: 'manual-required' | 'not-implemented',
  providerBoundary: 'backend-reference-only' | 'none',
  childDeviceConsumption: 'signed-snapshot-consumed' | 'manual-required' | 'not-implemented',
  manualRequired: true,
  failureState: ReturnType<typeof billingFailureState>
) {
  return {
    schemaVersion: 'billing-account-runtime-boundary-proof',
    operation,
    backendRuntimeState,
    providerBoundary,
    providerSecretCustody: 'not-present',
    childDeviceConsumption,
    manualRequired,
    failureState,
  } as const;
}

function unavailableFailure(failureKind: 'provider-unavailable' | 'network-unavailable') {
  return billingFailureState(failureKind, 'unavailable', 'local-only', 'wait-for-provider', true, RetryTimestamp);
}

function billingFailureState(
  failureKind: 'provider-unavailable' | 'network-unavailable' | 'payment-required' | 'validation-failed',
  parentVisibleState: 'unavailable' | 'past-due' | 'manual-review',
  localSafetyBehavior: 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  parentResolution: 'payment-update' | 'manual-support-review' | 'wait-for-provider',
  retryAllowed: boolean,
  retryAfter: typeof RetryTimestamp | null
): BillingAccountRuntimeFailureState {
  return {
    failureKind,
    parentVisibleState,
    localSafetyBehavior,
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution,
    retryAllowed,
    retryAfter,
  } as const;
}
