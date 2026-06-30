import {
  BillingDeviceLimitDecisionSchema,
  type BillingEntitlementSnapshot,
  BillingEntitlementSnapshotSchema,
  BillingFailureStateSchema,
  BillingPlanSchema,
  BillingSubscriptionStatusProofRowSchema,
  BillingSyncEventSchema,
  type BillingFailureState,
  type BillingPlan,
} from './billing-entitlement';
import { BillingPricingMatrixProofSchema } from './billing-pricing-matrix';

const Timestamp = '2026-06-13T22:47:00.000Z';
const RetryTimestamp = '2026-06-14T00:47:00.000Z';
const ExpiryTimestamp = '2026-06-20T22:47:00.000Z';

const ProviderUnavailableFailure = billingFailureState(
  'provider-unavailable',
  'unavailable',
  'local-only',
  'wait-for-provider',
  true,
  RetryTimestamp
);
const TrialGraceFailure = billingFailureState(
  'payment-required',
  'grace',
  'grace-with-local-safety',
  'payment-update',
  true,
  RetryTimestamp
);
const ValidationFailure = billingFailureState(
  'validation-failed',
  'manual-review',
  'manual-review-with-local-safety',
  'manual-support-review',
  false,
  null
);

const FreePlan = pricingPlan('family-safety-free', 1, 7, 'price-family-safety-free-backend-ref', [
  featureEntitlement('local-evidence-capture', true, false, true, 'local-only'),
  featureEntitlement('evidence-export-access', true, false, true, 'local-only'),
  featureEntitlement('multi-device-sync', false, true, false, 'unchanged'),
  featureEntitlement('advanced-reports', false, true, false, 'unchanged'),
]);
const CorePlan = pricingPlan('family-monitor-core', 3, 30, 'price-family-monitor-core-backend-ref', [
  featureEntitlement('local-evidence-capture', true, false, true, 'local-only'),
  featureEntitlement('evidence-export-access', true, false, true, 'local-only'),
  featureEntitlement('multi-device-sync', true, true, false, 'unchanged'),
  featureEntitlement('advanced-reports', true, true, false, 'unchanged'),
]);
const PlusPlan = pricingPlan('family-monitor-plus', 5, 90, 'price-family-monitor-plus-backend-ref', [
  featureEntitlement('local-evidence-capture', true, false, true, 'local-only'),
  featureEntitlement('evidence-export-access', true, false, true, 'local-only'),
  featureEntitlement('multi-device-sync', true, true, false, 'unchanged'),
  featureEntitlement('advanced-reports', true, true, false, 'unchanged'),
]);

const FreeSnapshot = entitlementSnapshot(
  'pricing-free-snapshot',
  FreePlan,
  'active',
  'signed-local-snapshot',
  'schema-valid-local',
  [
    featureDecision('local-evidence-capture', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('evidence-export-access', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('multi-device-sync', 'locked', 'within-plan', false, 'unchanged'),
    featureDecision('advanced-reports', 'locked', 'within-plan', false, 'unchanged'),
  ],
  null
);
const CoreSnapshot = entitlementSnapshot(
  'pricing-core-snapshot',
  CorePlan,
  'active',
  'signed-local-snapshot',
  'signed',
  [
    featureDecision('local-evidence-capture', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('evidence-export-access', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('multi-device-sync', 'available', 'within-plan', false, 'unchanged'),
    featureDecision('advanced-reports', 'available', 'within-plan', false, 'unchanged'),
  ],
  null
);
const PlusSnapshot = entitlementSnapshot(
  'pricing-plus-snapshot',
  PlusPlan,
  'active',
  'signed-local-snapshot',
  'signed',
  [
    featureDecision('local-evidence-capture', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('evidence-export-access', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('multi-device-sync', 'available', 'within-plan', false, 'unchanged'),
    featureDecision('advanced-reports', 'available', 'within-plan', false, 'unchanged'),
  ],
  null
);

const TrialSnapshot = entitlementSnapshot(
  'pricing-core-trial-snapshot',
  CorePlan,
  'trialing',
  'signed-local-snapshot',
  'signed',
  [
    featureDecision('local-evidence-capture', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('evidence-export-access', 'local-only', 'within-plan', true, 'local-only'),
    featureDecision('multi-device-sync', 'available', 'within-plan', false, 'unchanged'),
    featureDecision('advanced-reports', 'available', 'within-plan', false, 'unchanged'),
  ],
  null
);
const GraceSnapshot = entitlementSnapshot(
  'pricing-core-grace-snapshot',
  CorePlan,
  'grace',
  'signed-local-snapshot',
  'signed',
  [
    featureDecision('local-evidence-capture', 'local-only', 'payment-required', true, 'grace-with-local-safety'),
    featureDecision('evidence-export-access', 'local-only', 'payment-required', true, 'grace-with-local-safety'),
    featureDecision('multi-device-sync', 'grace', 'payment-required', false, 'grace-with-local-safety'),
    featureDecision('advanced-reports', 'grace', 'payment-required', false, 'grace-with-local-safety'),
  ],
  TrialGraceFailure
);
const FreeDegradedSnapshot = entitlementSnapshot(
  'pricing-free-degraded-snapshot',
  FreePlan,
  'unavailable',
  'unavailable',
  'unavailable',
  [
    featureDecision('local-evidence-capture', 'local-only', 'billing-unavailable', true, 'local-only'),
    featureDecision('evidence-export-access', 'local-only', 'billing-unavailable', true, 'local-only'),
    featureDecision('multi-device-sync', 'unavailable', 'billing-unavailable', false, 'local-only'),
    featureDecision('advanced-reports', 'unavailable', 'billing-unavailable', false, 'local-only'),
  ],
  ProviderUnavailableFailure
);
const ManualReviewSnapshot = entitlementSnapshot(
  'pricing-core-manual-review-snapshot',
  CorePlan,
  'unknown',
  'manual-admin-review',
  'manual-required',
  [
    featureDecision('local-evidence-capture', 'local-only', 'manual-review', true, 'manual-review-with-local-safety'),
    featureDecision('evidence-export-access', 'local-only', 'manual-review', true, 'manual-review-with-local-safety'),
    featureDecision('multi-device-sync', 'manual-required', 'manual-review', false, 'manual-review-with-local-safety'),
    featureDecision('advanced-reports', 'manual-required', 'manual-review', false, 'manual-review-with-local-safety'),
  ],
  ValidationFailure
);

export const BillingPricingMatrixProofReadModel = BillingPricingMatrixProofSchema.parse({
  schemaVersion: 'billing-entitlement-contract-proof',
  tierMatrix: [
    {
      plan: FreePlan,
      defaultEntitlementSnapshot: FreeSnapshot,
      defaultDeviceLimitDecision: deviceLimitDecision(
        'pricing-free-device-allowed',
        FreeSnapshot.snapshotId,
        0,
        1,
        false,
        'allowed',
        'within-plan',
        'windows-child-device-1'
      ),
    },
    {
      plan: CorePlan,
      defaultEntitlementSnapshot: CoreSnapshot,
      defaultDeviceLimitDecision: deviceLimitDecision(
        'pricing-core-device-allowed',
        CoreSnapshot.snapshotId,
        2,
        3,
        false,
        'allowed',
        'within-plan',
        'android-child-device-2'
      ),
    },
    {
      plan: PlusPlan,
      defaultEntitlementSnapshot: PlusSnapshot,
      defaultDeviceLimitDecision: deviceLimitDecision(
        'pricing-plus-device-allowed',
        PlusSnapshot.snapshotId,
        4,
        5,
        false,
        'allowed',
        'within-plan',
        'ios-child-device-3'
      ),
    },
  ],
  trialGraceBoundary: {
    paidPlan: CorePlan,
    trialSnapshot: TrialSnapshot,
    trialStatusProofRow: subscriptionStatusProofRow(
      'trialing',
      'signed-local-snapshot',
      'available',
      'unchanged',
      'allow-new-device',
      null
    ),
    graceSnapshot: GraceSnapshot,
    graceStatusProofRow: subscriptionStatusProofRow(
      'grace',
      'signed-local-snapshot',
      'grace',
      'grace-with-local-safety',
      'grace-existing-devices',
      TrialGraceFailure
    ),
    graceDeviceLimitDecision: deviceLimitDecision(
      'pricing-core-grace-device',
      GraceSnapshot.snapshotId,
      3,
      3,
      true,
      'grace',
      'payment-required',
      'windows-child-device-2'
    ),
  },
  safetyCriticalFreeBoundary: {
    freePlan: FreePlan,
    degradedSnapshot: FreeDegradedSnapshot,
    safetyCriticalPlanFeatures: FreePlan.featureEntitlements.filter((feature) => feature.safetyCritical),
    safetyCriticalSnapshotDecisions: FreeDegradedSnapshot.featureDecisions.filter(
      (decision) => decision.safetyCritical
    ),
  },
  entitlementSourceOwner: {
    authoritativeSourceOwner: 'billing-backend',
    offlineContinuationSource: 'signed-local-snapshot',
    manualFallbackSource: 'manual-admin-review',
    authoritativeSyncEvent: billingSyncEvent(
      'pricing-sync-core-active',
      'trialing',
      'active',
      'billing-backend',
      'stripe-subscription-core-backend-ref',
      null
    ),
    offlineEntitlementSnapshot: CoreSnapshot,
    offlineDeviceLimitDecision: deviceLimitDecision(
      'pricing-core-device-offline',
      CoreSnapshot.snapshotId,
      2,
      3,
      false,
      'allowed',
      'within-plan',
      'android-child-device-4'
    ),
    manualReviewEntitlementSnapshot: ManualReviewSnapshot,
  },
  updatedAt: Timestamp,
});

export const BillingPricingMatrixProof = BillingPricingMatrixProofReadModel;

export const BillingPricingMatrixKnownGaps = [
  'Exact price amounts, billing cadence variants, and public pricing copy remain unresolved product decisions.',
  'Checkout, portal, webhook, refund, dispute, and cancellation lifecycle slices remain outside this TS-only pricing matrix proof.',
  'Rust mirrors and runtime consumption wiring are intentionally left untouched by this pricing-only contract slice.',
] as const;

function pricingPlan(
  planId: 'family-safety-free' | 'family-monitor-core' | 'family-monitor-plus',
  deviceLimit: 1 | 3 | 5,
  advancedReportDays: 7 | 30 | 90,
  priceReference:
    | 'price-family-safety-free-backend-ref'
    | 'price-family-monitor-core-backend-ref'
    | 'price-family-monitor-plus-backend-ref',
  featureEntitlements: ReadonlyArray<ReturnType<typeof featureEntitlement>>
): BillingPlan {
  return BillingPlanSchema.parse({
    schemaVersion: 'billing-entitlement-contract-proof',
    planId,
    displayTextToken: `billing.plan.${planId}`,
    activeState: 'active',
    parentPortalAccess: {
      includedParentPortalCount: 1,
      extraParentSlotState: 'manual-required',
    },
    deviceLimit,
    featureEntitlements,
    retentionExportAllowance: {
      advancedReportDays,
      exportAllowed: true,
      parentOwnedStorageRequired: true,
    },
    priceReference,
    updatedAt: Timestamp,
  });
}

function entitlementSnapshot(
  snapshotId:
    | 'pricing-free-snapshot'
    | 'pricing-core-snapshot'
    | 'pricing-plus-snapshot'
    | 'pricing-core-trial-snapshot'
    | 'pricing-core-grace-snapshot'
    | 'pricing-free-degraded-snapshot'
    | 'pricing-core-manual-review-snapshot',
  plan: BillingPlan,
  subscriptionStatus: 'active' | 'trialing' | 'grace' | 'unknown' | 'unavailable',
  source: 'signed-local-snapshot' | 'manual-admin-review' | 'unavailable',
  signatureState: 'signed' | 'schema-valid-local' | 'manual-required' | 'unavailable',
  featureDecisions: ReadonlyArray<ReturnType<typeof featureDecision>>,
  failureState: BillingFailureState | null
) {
  return BillingEntitlementSnapshotSchema.parse({
    schemaVersion: 'billing-entitlement-contract-proof',
    snapshotId,
    family: {
      familyId: 'family-billing-pricing-proof-1',
    },
    parentAccount: {
      parentAccountId: 'parent-account-billing-pricing-proof-1',
    },
    planId: plan.planId,
    subscriptionStatus,
    source,
    signatureState,
    generatedAt: Timestamp,
    expiresAt: ExpiryTimestamp,
    deviceLimit: plan.deviceLimit,
    baseChildDeviceLimit: plan.deviceLimit,
    activeReferralCredits: 0,
    paidExtraChildDeviceSeats: 0,
    effectiveChildDeviceLimit: plan.deviceLimit,
    featureDecisions,
    failureState,
  });
}

function subscriptionStatusProofRow(
  subscriptionStatus: 'trialing' | 'grace',
  source: 'signed-local-snapshot',
  parentVisibleState: 'available' | 'grace',
  localSafetyBehavior: 'unchanged' | 'grace-with-local-safety',
  deviceActivationBehavior: 'allow-new-device' | 'grace-existing-devices',
  failureState: BillingFailureState | null
) {
  return BillingSubscriptionStatusProofRowSchema.parse({
    schemaVersion: 'billing-entitlement-contract-proof',
    subscriptionStatus,
    source,
    parentVisibleState,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    deviceActivationBehavior,
    failureState,
  });
}

function deviceLimitDecision(
  decisionId:
    | 'pricing-free-device-allowed'
    | 'pricing-core-device-allowed'
    | 'pricing-plus-device-allowed'
    | 'pricing-core-grace-device'
    | 'pricing-core-device-offline',
  entitlementSnapshotId: BillingEntitlementSnapshot['snapshotId'],
  activeDeviceCount: 0 | 2 | 3 | 4,
  planDeviceLimit: 1 | 3 | 5,
  requestedDeviceAlreadyTrusted: boolean,
  decision: 'allowed' | 'grace',
  reasonCode: 'within-plan' | 'payment-required',
  deviceId:
    | 'windows-child-device-1'
    | 'android-child-device-2'
    | 'ios-child-device-3'
    | 'windows-child-device-2'
    | 'android-child-device-4'
) {
  return BillingDeviceLimitDecisionSchema.parse({
    schemaVersion: 'billing-entitlement-contract-proof',
    decisionId,
    requestedDevice: {
      deviceId,
      childProfileId: 'child-billing-pricing-proof-1',
      label: `${deviceId} activation`,
      platform: deviceId.startsWith('windows') ? 'windows' : deviceId.startsWith('android') ? 'android' : 'ios',
    },
    entitlementSnapshotId,
    activeDeviceCount,
    planDeviceLimit,
    requestedDeviceAlreadyTrusted,
    decision,
    reasonCode,
    deviceActivationBehavior: decision === 'allowed' ? 'allow-new-device' : 'grace-existing-devices',
    auditReference: `audit-${decisionId}`,
    existingLocalSafetyBehavior: decision === 'allowed' ? 'unchanged' : 'grace-with-local-safety',
  });
}

function billingSyncEvent(
  syncEventId: 'pricing-sync-core-active',
  previousStatus: 'trialing',
  nextStatus: 'active',
  source: 'billing-backend',
  providerReference: 'stripe-subscription-core-backend-ref' | null,
  failureState: BillingFailureState | null
) {
  return BillingSyncEventSchema.parse({
    schemaVersion: 'billing-entitlement-contract-proof',
    syncEventId,
    previousStatus,
    nextStatus,
    source,
    actor: {
      actorId: 'billing-sync-system',
      role: 'system',
    },
    recordedAt: Timestamp,
    providerReference,
    providerBoundary: providerReference === null ? 'none' : 'backend-reference-only',
    failureState,
  });
}

function billingFailureState(
  failureKind: 'provider-unavailable' | 'payment-required' | 'validation-failed',
  parentVisibleState: 'unavailable' | 'grace' | 'manual-review',
  localSafetyBehavior: 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety',
  parentResolution: 'wait-for-provider' | 'payment-update' | 'manual-support-review',
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

function featureEntitlement(
  featureCode: 'local-evidence-capture' | 'evidence-export-access' | 'multi-device-sync' | 'advanced-reports',
  included: boolean,
  gateable: boolean,
  safetyCritical: boolean,
  localSafetyBehavior: 'unchanged' | 'local-only'
) {
  return {
    featureCode,
    included,
    gateable,
    safetyCritical,
    localSafetyBehavior,
    childActivityCustody: 'not-included',
  } as const;
}

function featureDecision(
  featureCode: 'local-evidence-capture' | 'evidence-export-access' | 'multi-device-sync' | 'advanced-reports',
  decision: 'available' | 'locked' | 'grace' | 'local-only' | 'manual-required' | 'unavailable',
  reasonCode: 'within-plan' | 'payment-required' | 'billing-unavailable' | 'manual-review',
  safetyCritical: boolean,
  localSafetyBehavior: 'unchanged' | 'local-only' | 'grace-with-local-safety' | 'manual-review-with-local-safety'
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
