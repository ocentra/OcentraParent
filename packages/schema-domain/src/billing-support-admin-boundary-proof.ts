import { Schema } from './effect';
import {
  BillingSupportAdminBoundaryProofSchema,
  type BillingSupportAdminFailureState,
} from './billing-support-admin-boundary';
import type {
  BillingSupportAdminAction,
  BillingSupportAdminRuntimeState,
} from './billing-support-admin-boundary-values';
import { ParentTimestampSchema } from './family-reference-primitives';

const decodeParentTimestamp = Schema.decodeUnknownSync(ParentTimestampSchema);
const Timestamp = decodeParentTimestamp('2026-06-04T12:19:16.000Z');
const RetryTimestamp = decodeParentTimestamp('2026-06-04T13:19:16.000Z');

const ManualSupportFailure = billingSupportFailureState(
  'validation-failed',
  'manual-review',
  'manual-review-with-local-safety',
  'manual-support-review',
  false,
  null
);
const PaymentRequiredFailure = billingSupportFailureState(
  'payment-required',
  'past-due',
  'grace-with-local-safety',
  'payment-update',
  true,
  null
);
const ProviderUnavailableFailure = billingSupportFailureState(
  'provider-unavailable',
  'unavailable',
  'local-only',
  'wait-for-provider',
  true,
  RetryTimestamp
);

export const BillingSupportAdminBoundaryProofReadModel = BillingSupportAdminBoundaryProofSchema.parse({
  schemaVersion: 'billing-support-admin-boundary-proof',
  rows: [
    billingSupportAdminRow(
      'billing-support-admin-triage',
      'support-case-triage',
      'read-only-local-proof',
      'available',
      'unchanged',
      'none',
      false,
      null
    ),
    billingSupportAdminRow(
      'billing-support-admin-account-review',
      'account-status-review',
      'read-only-local-proof',
      'available',
      'unchanged',
      'backend-reference-only',
      false,
      null
    ),
    billingSupportAdminRow(
      'billing-support-admin-escalation-request',
      'billing-escalation-request',
      'manual-required',
      'manual-review',
      'manual-review-with-local-safety',
      'backend-reference-only',
      true,
      ManualSupportFailure
    ),
    billingSupportAdminRow(
      'billing-support-admin-provider-contact',
      'provider-contact-manual-required',
      'manual-required',
      'past-due',
      'grace-with-local-safety',
      'backend-reference-only',
      true,
      PaymentRequiredFailure
    ),
    billingSupportAdminRow(
      'billing-support-admin-entitlement-override',
      'entitlement-admin-override-manual-required',
      'not-implemented',
      'manual-review',
      'manual-review-with-local-safety',
      'none',
      true,
      ManualSupportFailure
    ),
    billingSupportAdminRow(
      'billing-support-admin-refund-credit',
      'refund-credit-manual-required',
      'not-implemented',
      'unavailable',
      'local-only',
      'backend-reference-only',
      true,
      ProviderUnavailableFailure
    ),
  ],
  nonClaims: [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact',
    'no-account-backend-admin-runtime',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ],
  portalUiClaim: 'not-implemented',
  providerContactClaim: 'not-executed',
  backendUploadClaim: 'not-executed',
  childActivityCustodyClaim: 'not-included',
  updatedAt: Timestamp,
});

export const BillingSupportAdminBoundaryProof = BillingSupportAdminBoundaryProofReadModel;

export const BillingSupportAdminBoundaryKnownGaps = [
  'Billing provider contact remains manual-required and not executed.',
  'Account backend admin runtime and entitlement override runtime remain unimplemented.',
  'Refund and credit issuance runtime remains unimplemented.',
  'Portal admin UI and production support backend upload remain unimplemented.',
  'Child activity custody remains excluded from billing support admin rows.',
] as const;

function billingSupportAdminRow(
  boundaryId:
    | 'billing-support-admin-triage'
    | 'billing-support-admin-account-review'
    | 'billing-support-admin-escalation-request'
    | 'billing-support-admin-provider-contact'
    | 'billing-support-admin-entitlement-override'
    | 'billing-support-admin-refund-credit',
  action: BillingSupportAdminAction,
  runtimeState: BillingSupportAdminRuntimeState,
  parentVisibleState: 'available' | 'past-due' | 'unavailable' | 'manual-review',
  localSafetyBehavior: 'unchanged' | 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  providerBoundary: 'backend-reference-only' | 'none',
  manualRequired: boolean,
  failureState: ReturnType<typeof billingSupportFailureState> | null
) {
  return {
    schemaVersion: 'billing-support-admin-boundary-proof',
    boundaryId,
    supportCase: `support-case-${boundaryId}`,
    parentAccount: {
      parentAccountId: 'parent-account-billing-support-admin-proof-1',
    },
    family: {
      familyId: 'family-billing-support-admin-proof-1',
    },
    action,
    runtimeState,
    parentVisibleState,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    providerBoundary,
    providerSecretCustody: 'not-present',
    disclosedDataClasses: dataClassesFor(action),
    manualRequired,
    providerContacted: false,
    accountLookupExecuted: false,
    entitlementOverrideApplied: false,
    refundCreditIssued: false,
    supportBackendUploadExecuted: false,
    failureState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function dataClassesFor(action: BillingSupportAdminAction) {
  const shared = ['support-case-status-ref', 'account-status-ref', 'redaction-audit-ref'] as const;
  if (action === 'support-case-triage') {
    return shared;
  }
  if (action === 'account-status-review') {
    return [...shared, 'subscription-status-ref', 'entitlement-snapshot-ref'] as const;
  }
  if (action === 'billing-escalation-request') {
    return [...shared, 'billing-failure-state-ref', 'subscription-status-ref'] as const;
  }
  if (action === 'entitlement-admin-override-manual-required') {
    return [...shared, 'entitlement-snapshot-ref', 'device-limit-decision-ref'] as const;
  }
  return [...shared, 'billing-failure-state-ref'] as const;
}

function billingSupportFailureState(
  failureKind: 'provider-unavailable' | 'payment-required' | 'validation-failed',
  parentVisibleState: 'unavailable' | 'past-due' | 'manual-review',
  localSafetyBehavior: 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  parentResolution: 'payment-update' | 'manual-support-review' | 'wait-for-provider',
  retryAllowed: boolean,
  retryAfter: typeof RetryTimestamp | null
): BillingSupportAdminFailureState {
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
