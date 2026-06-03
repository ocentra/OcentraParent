import {
  AppInstallPurchaseApprovalContractProofSchema,
  type AppInstallPurchaseApprovalPlatformSupportRow,
} from './app-install-purchase-approval';

const Timestamp = '2026-06-03T07:10:00.000Z';
const ExpiryTimestamp = '2026-06-10T07:10:00.000Z';
const EvidenceReference = {
  evidenceReferenceId: 'evidence-install-purchase-proof-1',
  kind: 'activity-event',
  observedAt: Timestamp,
} as const;
const ParentAction = {
  actionReferenceId: 'parent-action-install-purchase-proof-1',
  actor: {
    actorId: 'parent-install-purchase-proof-1',
    role: 'parent',
  },
  policyVersion: 'install-purchase-approval-policy-v1',
  createdAt: Timestamp,
} as const;
const RequestAuditEvent = auditEvent('audit-request-recorded-1', 'request-recorded');
const DecisionAuditEvent = auditEvent('audit-parent-decision-recorded-1', 'parent-decision-recorded');

export const AppInstallPurchaseApprovalContractProofReadModel = AppInstallPurchaseApprovalContractProofSchema.parse({
  schemaVersion: 'app-install-purchase-approval-contract-proof',
  installRequest: request('install-request-proof-1', 'install', 'android'),
  purchaseRequest: {
    ...request('purchase-request-proof-1', 'purchase', 'android'),
    storeMetadata: storeMetadata('google-play', 'stale'),
    purchaseKind: 'in-app-purchase',
    subscriptionPeriod: null,
    priceDisplay: 'USD 4.99',
    billingEntitlementClaim: 'not-claimed',
  },
  subscriptionRequest: {
    ...request('subscription-request-proof-1', 'subscription', 'android'),
    storeMetadata: storeMetadata('apple-app-store', 'manual-required'),
    purchaseKind: 'subscription',
    subscriptionPeriod: 'monthly',
    priceDisplay: 'USD 9.99 monthly',
    billingEntitlementClaim: 'not-claimed',
  },
  approvalDecisions: [
    decision('approve', 'approved', 'not-expiring', null, null, ParentAction),
    decision('deny', 'denied', 'not-expiring', null, null, ParentAction),
    decision('time-box', 'time-box-active', 'time-box-active', ExpiryTimestamp, null, ParentAction),
    decision('review-needed', 'review-needed', 'review-needed', null, 'age rating changed', null),
  ],
  platformSupportMatrix: [
    platformRow('windows', 'microsoft-store', 'manual-required', 'manual-required'),
    platformRow('macos', 'mac-app-store', 'manual-required', 'manual-required'),
    platformRow('linux', 'linux-package-manager', 'unavailable', 'unavailable'),
    platformRow('android', 'google-play', 'manual-required', 'manual-required'),
    platformRow('ios', 'apple-app-store', 'manual-required', 'manual-required'),
  ],
  nonClaims: [
    'no-store-integration',
    'no-billing-entitlement-logic',
    'no-portal-ui',
    'no-platform-adapter',
    'no-store-policy-bypass',
    'no-real-install-or-purchase-interception',
    'not-generic-app-blocking',
  ],
  storeIntegrationClaim: 'not-claimed',
  billingEntitlementClaim: 'not-claimed',
  portalUiClaim: 'not-implemented',
  platformAdapterClaim: 'not-implemented',
  interceptionClaim: 'not-claimed',
  runtimeBlockingSeparation: 'separate-from-generic-app-blocking',
  updatedAt: Timestamp,
});

export const AppInstallPurchaseApprovalContractProof = AppInstallPurchaseApprovalContractProofReadModel;

export const AppInstallPurchaseApprovalProofKnownGaps = [
  'Google Play, Apple App Store, Microsoft Store, Mac App Store, and package-manager integrations are not implemented.',
  'No billing entitlement state is used as child-safety approval authority.',
  'No portal approval UI exists in this proof.',
  'No platform adapter, store policy bypass, or real install/purchase interception is claimed.',
  'Generic runtime app blocking remains separate from install and purchase approval.',
] as const;

export function summarizeAppInstallPurchaseApprovalSupportStates(
  rows: ReadonlyArray<AppInstallPurchaseApprovalPlatformSupportRow>
): Record<'supported' | 'manual-required' | 'unavailable', number> {
  const counts: Record<'supported' | 'manual-required' | 'unavailable', number> = {
    supported: 0,
    'manual-required': 0,
    unavailable: 0,
  };
  for (const row of rows) {
    for (const state of [
      row.contractRequestState,
      row.storeMetadataState,
      row.installInterceptionState,
      row.purchaseInterceptionState,
      row.subscriptionInterceptionState,
      row.childPendingState,
      row.approvalDeliveryState,
    ]) {
      counts[state] += 1;
    }
  }
  return counts;
}

function request(
  requestId: 'install-request-proof-1' | 'purchase-request-proof-1' | 'subscription-request-proof-1',
  requestKind: 'install' | 'purchase' | 'subscription',
  platform: 'android'
) {
  return {
    schemaVersion: 'app-install-purchase-approval-contract-proof',
    requestId,
    requestKind,
    family: {
      familyId: 'family-install-purchase-proof-1',
    },
    child: {
      childProfileId: 'child-install-purchase-proof-1',
      displayName: 'Avery',
    },
    device: {
      deviceId: `${platform}-child-device-1`,
      childProfileId: 'child-install-purchase-proof-1',
      label: `${platform} child device`,
      platform,
    },
    platform,
    storeMetadata: storeMetadata('parent-manual-entry', 'fresh'),
    approvalState: {
      state: 'pending-parent-review',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    requestedAt: Timestamp,
    evidenceReferences: [EvidenceReference],
    auditEventRefs: [RequestAuditEvent],
  } as const;
}

function storeMetadata(
  storeSurface: 'parent-manual-entry' | 'google-play' | 'apple-app-store',
  freshness: 'fresh' | 'stale' | 'manual-required'
) {
  return {
    storeSurface,
    sourceState: freshness === 'manual-required' ? 'manual-required' : 'supported',
    freshness,
    listingId: freshness === 'manual-required' ? null : `${storeSurface}-listing-minecraft`,
    appTitle: freshness === 'manual-required' ? null : 'Minecraft',
    publisherName: freshness === 'manual-required' ? null : 'Mojang Studios',
    category: freshness === 'manual-required' ? null : 'Games',
    ageRating: freshness === 'manual-required' ? null : 'Everyone 10 plus',
    refreshedAt: freshness === 'manual-required' ? null : Timestamp,
    staleAt: freshness === 'manual-required' ? null : ExpiryTimestamp,
    proofRequirement: `${storeSurface} metadata remains contract proof until platform source artifacts exist`,
  } as const;
}

function decision(
  decisionAction: 'approve' | 'deny' | 'time-box' | 'review-needed',
  state: 'approved' | 'denied' | 'time-box-active' | 'review-needed',
  expiryState: 'not-expiring' | 'time-box-active' | 'review-needed',
  expiresAt: typeof ExpiryTimestamp | null,
  reviewReason: 'age rating changed' | null,
  parentAction: typeof ParentAction | null
) {
  return {
    schemaVersion: 'app-install-purchase-approval-contract-proof',
    decisionId: `decision-${decisionAction}`,
    requestId: decisionAction === 'approve' ? 'install-request-proof-1' : 'purchase-request-proof-1',
    requestKind: decisionAction === 'approve' ? 'install' : 'purchase',
    decisionAction,
    resultingState: {
      state,
      expiryState,
      expiresAt,
      reviewReason,
    },
    parentAction,
    decidedAt: Timestamp,
    auditEventRefs: [DecisionAuditEvent],
  } as const;
}

function platformRow(
  platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios',
  storeSurface: 'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store',
  storeMetadataState: 'manual-required' | 'unavailable',
  platformState: 'manual-required' | 'unavailable'
) {
  return {
    platform,
    storeSurface,
    contractRequestState: 'supported',
    storeMetadataState,
    installInterceptionState: platformState,
    purchaseInterceptionState: platformState,
    subscriptionInterceptionState: platformState,
    childPendingState: platformState,
    approvalDeliveryState: platformState,
    manualRequirement:
      platformState === 'manual-required'
        ? 'real store or OS approval API proof with user-visible parent workflow'
        : null,
    unavailableReason:
      platformState === 'unavailable'
        ? 'no approved platform store interception path is claimed by this contract proof'
        : null,
    proofRequirement: 'contract proof only; platform adapter proof must be added before product support claims',
    claimBoundary: 'contract proof only; no platform adapter no store integration no runtime blocking',
  } as const;
}

function auditEvent(
  auditEventId: 'audit-request-recorded-1' | 'audit-parent-decision-recorded-1',
  eventKind: 'request-recorded' | 'parent-decision-recorded'
) {
  return {
    auditEventId,
    eventKind,
    recordedAt: Timestamp,
    evidenceReferences: [EvidenceReference],
  } as const;
}
