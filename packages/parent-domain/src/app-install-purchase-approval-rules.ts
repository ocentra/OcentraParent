type AppInstallPurchaseApprovalPlatform = 'windows' | 'macos' | 'linux' | 'android' | 'ios';
type AppInstallPurchaseApprovalRequestKind = 'install' | 'purchase' | 'subscription';
type AppInstallPurchaseApprovalFreshness = 'fresh' | 'stale' | 'unknown' | 'manual-required' | 'unavailable';
type AppInstallPurchaseApprovalSupportState = 'supported' | 'manual-required' | 'unavailable';
type AppInstallPurchaseApprovalState =
  | 'pending-parent-review'
  | 'approved'
  | 'denied'
  | 'time-box-active'
  | 'expired'
  | 'review-needed';
type AppInstallPurchaseApprovalExpiryState = 'not-expiring' | 'time-box-active' | 'expired' | 'review-needed';
type AppInstallPurchaseApprovalDecisionAction = 'approve' | 'deny' | 'time-box' | 'review-needed';
type AppInstallPurchaseApprovalPurchaseKind = 'one-time-purchase' | 'in-app-purchase' | 'subscription';
type AppInstallPurchaseApprovalNonClaim =
  | 'no-store-integration'
  | 'no-billing-entitlement-logic'
  | 'no-portal-ui'
  | 'no-platform-adapter'
  | 'no-store-policy-bypass'
  | 'no-real-install-or-purchase-interception'
  | 'not-generic-app-blocking';

interface AppInstallPurchaseApprovalStoreMetadataRuleInput {
  readonly freshness: AppInstallPurchaseApprovalFreshness;
  readonly sourceState: AppInstallPurchaseApprovalSupportState;
  readonly listingId: unknown | null;
  readonly appTitle: unknown | null;
  readonly publisherName: unknown | null;
  readonly category: unknown | null;
  readonly ageRating: unknown | null;
  readonly refreshedAt: unknown | null;
  readonly staleAt: unknown | null;
}

interface AppInstallPurchaseApprovalStateSnapshotRuleInput {
  readonly state: AppInstallPurchaseApprovalState;
  readonly expiryState: AppInstallPurchaseApprovalExpiryState;
  readonly expiresAt: unknown | null;
  readonly reviewReason: unknown | null;
}

interface PurchaseRequestRuleInput {
  readonly requestKind: AppInstallPurchaseApprovalRequestKind;
  readonly purchaseKind: AppInstallPurchaseApprovalPurchaseKind;
  readonly subscriptionPeriod: unknown | null;
  readonly billingEntitlementClaim: 'not-claimed';
}

interface AppInstallPurchaseApprovalDecisionRuleInput {
  readonly decisionAction: AppInstallPurchaseApprovalDecisionAction;
  readonly resultingState: AppInstallPurchaseApprovalStateSnapshotRuleInput;
  readonly parentAction: unknown | null;
  readonly auditEventRefs: readonly unknown[];
}

interface AppInstallPurchaseApprovalPlatformSupportRowRuleInput {
  readonly platform: AppInstallPurchaseApprovalPlatform;
  readonly contractRequestState: AppInstallPurchaseApprovalSupportState;
  readonly storeMetadataState: AppInstallPurchaseApprovalSupportState;
  readonly installInterceptionState: AppInstallPurchaseApprovalSupportState;
  readonly purchaseInterceptionState: AppInstallPurchaseApprovalSupportState;
  readonly subscriptionInterceptionState: AppInstallPurchaseApprovalSupportState;
  readonly childPendingState: AppInstallPurchaseApprovalSupportState;
  readonly approvalDeliveryState: AppInstallPurchaseApprovalSupportState;
  readonly manualRequirement: unknown | null;
  readonly unavailableReason: unknown | null;
  readonly claimBoundary: { readonly includes: (needle: 'contract proof') => boolean };
}

interface AppInstallPurchaseApprovalContractProofRuleInput {
  readonly installRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly purchaseRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly subscriptionRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly approvalDecisions: readonly AppInstallPurchaseApprovalDecisionRuleInput[];
  readonly platformSupportMatrix: readonly AppInstallPurchaseApprovalPlatformSupportRowRuleInput[];
  readonly nonClaims: readonly AppInstallPurchaseApprovalNonClaim[];
  readonly storeIntegrationClaim: 'not-claimed';
  readonly billingEntitlementClaim: 'not-claimed';
  readonly portalUiClaim: 'not-implemented';
  readonly platformAdapterClaim: 'not-implemented';
  readonly interceptionClaim: 'not-claimed';
  readonly runtimeBlockingSeparation: 'separate-from-generic-app-blocking';
}

const RequiredPlatformRows = ['windows', 'macos', 'linux', 'android', 'ios'] as const;
const RequiredDecisionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredSupportStates = ['supported', 'manual-required', 'unavailable'] as const;
const RequiredNonClaims = [
  'no-store-integration',
  'no-billing-entitlement-logic',
  'no-portal-ui',
  'no-platform-adapter',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'not-generic-app-blocking',
] as const;

export function storeMetadataFreshnessIsConsistent(
  metadata: AppInstallPurchaseApprovalStoreMetadataRuleInput
): boolean {
  if (metadata.freshness === 'fresh') {
    return metadata.sourceState === 'supported' && metadataFieldsArePresent(metadata);
  }
  if (metadata.freshness === 'stale') {
    return metadata.sourceState === 'supported' && metadataFieldsArePresent(metadata);
  }
  if (metadata.freshness === 'manual-required') {
    return metadata.sourceState === 'manual-required' && metadataFieldsAreAbsent(metadata);
  }
  if (metadata.freshness === 'unavailable') {
    return metadata.sourceState === 'unavailable' && metadataFieldsAreAbsent(metadata);
  }
  return metadata.sourceState !== 'supported' && metadataFieldsAreAbsent(metadata);
}

export function approvalStateSnapshotIsConsistent(snapshot: AppInstallPurchaseApprovalStateSnapshotRuleInput): boolean {
  if (snapshot.state === 'time-box-active') {
    return snapshot.expiryState === 'time-box-active' && snapshot.expiresAt !== null && snapshot.reviewReason === null;
  }
  if (snapshot.state === 'expired') {
    return snapshot.expiryState === 'expired' && snapshot.expiresAt !== null && snapshot.reviewReason === null;
  }
  if (snapshot.state === 'review-needed') {
    return snapshot.expiryState === 'review-needed' && snapshot.expiresAt === null && snapshot.reviewReason !== null;
  }
  return snapshot.expiryState === 'not-expiring' && snapshot.expiresAt === null && snapshot.reviewReason === null;
}

export function purchaseRequestKindIsConsistent(request: PurchaseRequestRuleInput): boolean {
  if (request.requestKind === 'subscription') {
    return request.purchaseKind === 'subscription' && request.subscriptionPeriod !== null;
  }
  return request.purchaseKind !== 'subscription' && request.subscriptionPeriod === null;
}

export function approvalDecisionIsConsistent(decision: AppInstallPurchaseApprovalDecisionRuleInput): boolean {
  if (decision.auditEventRefs.length === 0) {
    return false;
  }
  if (decision.decisionAction === 'approve') {
    return decision.resultingState.state === 'approved' && decision.parentAction !== null;
  }
  if (decision.decisionAction === 'deny') {
    return decision.resultingState.state === 'denied' && decision.parentAction !== null;
  }
  if (decision.decisionAction === 'time-box') {
    return decision.resultingState.state === 'time-box-active' && decision.parentAction !== null;
  }
  return decision.resultingState.state === 'review-needed' && decision.parentAction === null;
}

export function platformSupportRowIsHonest(row: AppInstallPurchaseApprovalPlatformSupportRowRuleInput): boolean {
  const gatedStates = [
    row.storeMetadataState,
    row.installInterceptionState,
    row.purchaseInterceptionState,
    row.subscriptionInterceptionState,
    row.childPendingState,
    row.approvalDeliveryState,
  ];
  const manualRequirementIsPresent = !gatedStates.includes('manual-required') || row.manualRequirement !== null;
  const unavailableReasonIsPresent = !gatedStates.includes('unavailable') || row.unavailableReason !== null;

  return (
    row.contractRequestState === 'supported' &&
    row.installInterceptionState !== 'supported' &&
    row.purchaseInterceptionState !== 'supported' &&
    row.subscriptionInterceptionState !== 'supported' &&
    row.approvalDeliveryState !== 'supported' &&
    manualRequirementIsPresent &&
    unavailableReasonIsPresent &&
    row.claimBoundary.includes('contract proof')
  );
}

export function appInstallPurchaseApprovalContractProofIsHonest(
  proof: AppInstallPurchaseApprovalContractProofRuleInput
): boolean {
  return (
    requestKindsArePresent(proof) &&
    decisionActionsArePresent(proof.approvalDecisions) &&
    platformMatrixIsComplete(proof.platformSupportMatrix) &&
    platformMatrixContainsAllSupportStates(proof.platformSupportMatrix) &&
    nonClaimsAreComplete(proof.nonClaims) &&
    proof.storeIntegrationClaim === 'not-claimed' &&
    proof.billingEntitlementClaim === 'not-claimed' &&
    proof.portalUiClaim === 'not-implemented' &&
    proof.platformAdapterClaim === 'not-implemented' &&
    proof.interceptionClaim === 'not-claimed' &&
    proof.runtimeBlockingSeparation === 'separate-from-generic-app-blocking'
  );
}

function metadataFieldsArePresent(metadata: AppInstallPurchaseApprovalStoreMetadataRuleInput): boolean {
  return (
    metadata.listingId !== null &&
    metadata.appTitle !== null &&
    metadata.publisherName !== null &&
    metadata.category !== null &&
    metadata.ageRating !== null &&
    metadata.refreshedAt !== null &&
    metadata.staleAt !== null
  );
}

function metadataFieldsAreAbsent(metadata: AppInstallPurchaseApprovalStoreMetadataRuleInput): boolean {
  return (
    metadata.listingId === null &&
    metadata.appTitle === null &&
    metadata.publisherName === null &&
    metadata.category === null &&
    metadata.ageRating === null &&
    metadata.refreshedAt === null &&
    metadata.staleAt === null
  );
}

function requestKindsArePresent(proof: AppInstallPurchaseApprovalContractProofRuleInput): boolean {
  return (
    proof.installRequest.requestKind === 'install' &&
    proof.purchaseRequest.requestKind === 'purchase' &&
    proof.subscriptionRequest.requestKind === 'subscription'
  );
}

function decisionActionsArePresent(decisions: readonly AppInstallPurchaseApprovalDecisionRuleInput[]): boolean {
  const actions = new Set(decisions.map((decision) => decision.decisionAction));
  return RequiredDecisionActions.every((action) => actions.has(action));
}

function platformMatrixIsComplete(rows: readonly AppInstallPurchaseApprovalPlatformSupportRowRuleInput[]): boolean {
  const byPlatform = new Map(rows.map((row) => [row.platform, row] as const));
  return byPlatform.size === rows.length && RequiredPlatformRows.every((platform) => byPlatform.has(platform));
}

function platformMatrixContainsAllSupportStates(
  rows: readonly AppInstallPurchaseApprovalPlatformSupportRowRuleInput[]
): boolean {
  const states = new Set<AppInstallPurchaseApprovalSupportState>();
  for (const row of rows) {
    states.add(row.contractRequestState);
    states.add(row.storeMetadataState);
    states.add(row.installInterceptionState);
    states.add(row.purchaseInterceptionState);
    states.add(row.subscriptionInterceptionState);
    states.add(row.childPendingState);
    states.add(row.approvalDeliveryState);
  }
  return RequiredSupportStates.every((state) => states.has(state));
}

function nonClaimsAreComplete(nonClaims: readonly AppInstallPurchaseApprovalNonClaim[]): boolean {
  const nonClaimSet = new Set(nonClaims);
  return RequiredNonClaims.every((nonClaim) => nonClaimSet.has(nonClaim));
}
