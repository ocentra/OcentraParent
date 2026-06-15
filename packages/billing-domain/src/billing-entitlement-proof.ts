import { BillingEntitlementContractProofSchema, type BillingFailureState } from './billing-entitlement';

const Timestamp = '2026-06-03T09:57:32.000Z';
const ExpiryTimestamp = '2026-06-10T09:57:32.000Z';
const SnapshotId = 'entitlement-snapshot-family-1-active';

const ProviderUnavailableFailure = billingFailureState(
  'provider-unavailable',
  'unavailable',
  'local-only',
  'wait-for-provider',
  true,
  null
);
const NetworkUnavailableFailure = billingFailureState(
  'network-unavailable',
  'unavailable',
  'local-only',
  'wait-for-provider',
  true,
  null
);
const StaleSnapshotFailure = billingFailureState(
  'stale-snapshot',
  'stale',
  'grace-with-local-safety',
  'wait-for-provider',
  true,
  ExpiryTimestamp
);
const PaymentRequiredFailure = billingFailureState(
  'payment-required',
  'past-due',
  'grace-with-local-safety',
  'payment-update',
  true,
  null
);
const AccountMismatchFailure = billingFailureState(
  'account-mismatch',
  'manual-review',
  'manual-review-with-local-safety',
  'manual-support-review',
  false,
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

export const BillingEntitlementContractProofReadModel = BillingEntitlementContractProofSchema.parse({
  schemaVersion: 'billing-entitlement-contract-proof',
  plan: {
    schemaVersion: 'billing-entitlement-contract-proof',
    planId: 'family-plus-monthly',
    displayTextToken: 'billing.plan.familyPlusMonthly',
    activeState: 'active',
    deviceLimit: 5,
    featureEntitlements: [
      featureEntitlement('multi-device-sync', true, true, false),
      featureEntitlement('advanced-reports', true, true, false),
      featureEntitlement('local-evidence-capture', true, false, true),
      featureEntitlement('evidence-export-access', true, false, true),
    ],
    retentionExportAllowance: {
      advancedReportDays: 30,
      exportAllowed: true,
      parentOwnedStorageRequired: true,
    },
    priceReference: 'price-family-plus-monthly-backend-ref',
    updatedAt: Timestamp,
  },
  entitlementSnapshot: {
    schemaVersion: 'billing-entitlement-contract-proof',
    snapshotId: SnapshotId,
    family: {
      familyId: 'family-billing-entitlement-proof-1',
    },
    parentAccount: {
      parentAccountId: 'parent-account-billing-entitlement-proof-1',
    },
    planId: 'family-plus-monthly',
    subscriptionStatus: 'active',
    source: 'signed-local-snapshot',
    signatureState: 'schema-valid-local',
    generatedAt: Timestamp,
    expiresAt: ExpiryTimestamp,
    deviceLimit: 5,
    baseChildDeviceLimit: 1,
    activeReferralCredits: 2,
    paidExtraChildDeviceSeats: 2,
    effectiveChildDeviceLimit: 5,
    featureDecisions: [
      featureDecision('multi-device-sync', 'available', 'within-plan', false, 'unchanged'),
      featureDecision('advanced-reports', 'available', 'within-plan', false, 'unchanged'),
      featureDecision('cloud-relay', 'grace', 'snapshot-stale', false, 'grace-with-local-safety'),
      featureDecision('local-evidence-capture', 'local-only', 'within-plan', true, 'local-only'),
      featureDecision('evidence-export-access', 'local-only', 'within-plan', true, 'local-only'),
    ],
    failureState: null,
  },
  referralCreditSummary: {
    activeQualifiedReferralParents: 2,
    activeReferralCredits: 2,
    pendingReferralInvites: 1,
    revokedReferralCredits: 1,
  },
  subscriptionStatusProofRows: [
    subscriptionStatusProofRow('trialing', 'billing-backend', 'available', 'unchanged', 'allow-new-device', null),
    subscriptionStatusProofRow('active', 'signed-local-snapshot', 'available', 'unchanged', 'allow-new-device', null),
    subscriptionStatusProofRow(
      'past-due',
      'signed-local-snapshot',
      'past-due',
      'grace-with-local-safety',
      'grace-existing-devices',
      PaymentRequiredFailure
    ),
    subscriptionStatusProofRow(
      'expired',
      'signed-local-snapshot',
      'stale',
      'grace-with-local-safety',
      'deny-new-device',
      StaleSnapshotFailure
    ),
    subscriptionStatusProofRow(
      'grace',
      'signed-local-snapshot',
      'grace',
      'grace-with-local-safety',
      'grace-existing-devices',
      null
    ),
    subscriptionStatusProofRow(
      'unavailable',
      'unavailable',
      'unavailable',
      'local-only',
      'deny-new-device',
      NetworkUnavailableFailure
    ),
  ],
  billingSyncEvents: [
    billingSyncEvent(
      'billing-sync-active-1',
      'trialing',
      'active',
      'billing-backend',
      'stripe-subscription-backend-ref',
      null
    ),
    billingSyncEvent(
      'billing-sync-provider-down-1',
      'active',
      'unavailable',
      'unavailable',
      null,
      ProviderUnavailableFailure
    ),
  ],
  deviceLimitDecisions: [
    deviceLimitDecision('device-limit-allowed-1', 4, 5, false, 'allowed', 'within-plan', 'windows-child-device-1'),
    deviceLimitDecision('device-limit-denied-1', 5, 5, false, 'denied', 'limit-exceeded', 'android-child-device-6'),
    deviceLimitDecision('device-limit-grace-1', 5, 5, true, 'grace', 'snapshot-stale', 'ios-child-device-2'),
    deviceLimitDecision(
      'device-limit-manual-1',
      5,
      5,
      false,
      'manual-review',
      'manual-review',
      'android-child-device-7'
    ),
  ],
  failureStates: [
    ProviderUnavailableFailure,
    NetworkUnavailableFailure,
    StaleSnapshotFailure,
    PaymentRequiredFailure,
    AccountMismatchFailure,
    ValidationFailedFailure,
  ],
  nonClaims: [
    'no-stripe-sdk',
    'no-billing-provider-backend',
    'no-provider-token-custody',
    'no-child-activity-custody',
    'no-safety-shutdown',
    'no-portal-ui',
  ],
  stripeSdkClaim: 'not-included',
  billingProviderBackendClaim: 'not-implemented',
  portalUiClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-included',
  updatedAt: Timestamp,
});

export const BillingEntitlementContractProof = BillingEntitlementContractProofReadModel;

export const BillingEntitlementKnownGaps = [
  'Billing provider integration and Stripe customer/subscription storage remain unimplemented.',
  'Account backend, entitlement signing runtime, and subscription sync delivery remain unimplemented.',
  'Portal billing UI and account-management flows remain unimplemented.',
  'Child-device safety modules do not consume these entitlement snapshots yet.',
] as const;

export function summarizeBillingFailureStates(
  failureStates: ReadonlyArray<BillingFailureState>
): Record<
  | 'provider-unavailable'
  | 'network-unavailable'
  | 'stale-snapshot'
  | 'payment-required'
  | 'account-mismatch'
  | 'validation-failed',
  number
> {
  const counts = {
    'provider-unavailable': 0,
    'network-unavailable': 0,
    'stale-snapshot': 0,
    'payment-required': 0,
    'account-mismatch': 0,
    'validation-failed': 0,
  };
  for (const failureState of failureStates) {
    if (failureState.failureKind in counts) {
      counts[failureState.failureKind as keyof typeof counts] += 1;
    }
  }
  return counts;
}

function featureEntitlement(
  featureCode: 'multi-device-sync' | 'advanced-reports' | 'local-evidence-capture' | 'evidence-export-access',
  included: true,
  gateable: boolean,
  safetyCritical: boolean
) {
  return {
    featureCode,
    included,
    gateable,
    safetyCritical,
    localSafetyBehavior: safetyCritical ? 'local-only' : 'unchanged',
    childActivityCustody: 'not-included',
  } as const;
}

function featureDecision(
  featureCode:
    | 'multi-device-sync'
    | 'advanced-reports'
    | 'cloud-relay'
    | 'local-evidence-capture'
    | 'evidence-export-access',
  decision: 'available' | 'grace' | 'local-only',
  reasonCode: 'within-plan' | 'snapshot-stale',
  safetyCritical: boolean,
  localSafetyBehavior: 'unchanged' | 'local-only' | 'grace-with-local-safety'
) {
  return {
    featureCode,
    decision,
    reasonCode,
    safetyCritical,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
  } as const;
}

function subscriptionStatusProofRow(
  subscriptionStatus: 'trialing' | 'active' | 'past-due' | 'expired' | 'grace' | 'unavailable',
  source: 'billing-backend' | 'signed-local-snapshot' | 'unavailable',
  parentVisibleState: 'available' | 'past-due' | 'stale' | 'grace' | 'unavailable',
  localSafetyBehavior: 'unchanged' | 'local-only' | 'grace-with-local-safety',
  deviceActivationBehavior: 'allow-new-device' | 'deny-new-device' | 'grace-existing-devices',
  failureState: ReturnType<typeof billingFailureState> | null
) {
  return {
    schemaVersion: 'billing-entitlement-contract-proof',
    subscriptionStatus,
    source,
    parentVisibleState,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    deviceActivationBehavior,
    failureState,
  } as const;
}

function billingSyncEvent(
  syncEventId: 'billing-sync-active-1' | 'billing-sync-provider-down-1',
  previousStatus: 'trialing' | 'active',
  nextStatus: 'active' | 'unavailable',
  source: 'billing-backend' | 'unavailable',
  providerReference: 'stripe-subscription-backend-ref' | null,
  failureState: ReturnType<typeof billingFailureState> | null
) {
  return {
    schemaVersion: 'billing-entitlement-contract-proof',
    syncEventId,
    previousStatus,
    nextStatus,
    source,
    actor: {
      actorId: source === 'billing-backend' ? 'billing-sync-system' : 'billing-outage-monitor',
      role: 'system',
    },
    recordedAt: Timestamp,
    providerReference,
    providerBoundary: providerReference === null ? 'none' : 'backend-reference-only',
    failureState,
  } as const;
}

function deviceLimitDecision(
  decisionId: 'device-limit-allowed-1' | 'device-limit-denied-1' | 'device-limit-grace-1' | 'device-limit-manual-1',
  activeDeviceCount: 4 | 5,
  planDeviceLimit: 5,
  requestedDeviceAlreadyTrusted: boolean,
  decision: 'allowed' | 'denied' | 'grace' | 'manual-review',
  reasonCode: 'within-plan' | 'limit-exceeded' | 'snapshot-stale' | 'manual-review',
  deviceId: 'windows-child-device-1' | 'android-child-device-6' | 'ios-child-device-2' | 'android-child-device-7'
) {
  return {
    schemaVersion: 'billing-entitlement-contract-proof',
    decisionId,
    requestedDevice: {
      deviceId,
      childProfileId: 'child-billing-entitlement-proof-1',
      label: `${deviceId} activation`,
      platform: deviceId.startsWith('windows') ? 'windows' : deviceId.startsWith('android') ? 'android' : 'ios',
    },
    entitlementSnapshotId: SnapshotId,
    activeDeviceCount,
    planDeviceLimit,
    requestedDeviceAlreadyTrusted,
    decision,
    reasonCode,
    deviceActivationBehavior:
      decision === 'allowed'
        ? 'allow-new-device'
        : decision === 'denied'
          ? 'deny-new-device'
          : decision === 'grace'
            ? 'grace-existing-devices'
            : 'manual-review-required',
    auditReference: `audit-${decisionId}`,
    existingLocalSafetyBehavior: decision === 'allowed' ? 'unchanged' : 'grace-with-local-safety',
  } as const;
}

function billingFailureState(
  failureKind:
    | 'provider-unavailable'
    | 'network-unavailable'
    | 'stale-snapshot'
    | 'payment-required'
    | 'account-mismatch'
    | 'validation-failed',
  parentVisibleState: 'unavailable' | 'stale' | 'past-due' | 'manual-review',
  localSafetyBehavior: 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  parentResolution: 'payment-update' | 'manual-support-review' | 'wait-for-provider',
  retryAllowed: boolean,
  retryAfter: typeof ExpiryTimestamp | null
) {
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
