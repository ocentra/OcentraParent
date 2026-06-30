/* compatibility shim over Rust-generated app-install purchase approval rules */

import {
  type GeneratedAppInstallPurchaseApprovalAuditReportSurface,
  type GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim,
  type GeneratedAppInstallPurchaseApprovalChildFacingStatus,
  type GeneratedAppInstallPurchaseApprovalDecisionAction,
  type GeneratedAppInstallPurchaseApprovalExpiryState,
  type GeneratedAppInstallPurchaseApprovalInterceptionClaim,
  type GeneratedAppInstallPurchaseApprovalNonClaim,
  type GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim,
  type GeneratedAppInstallPurchaseApprovalPortalUiClaim,
  type GeneratedAppInstallPurchaseApprovalProofIntegrationState,
  type GeneratedAppInstallPurchaseApprovalPurchaseKind,
  type GeneratedAppInstallPurchaseApprovalRequestKind,
  type GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation,
  type GeneratedAppInstallPurchaseApprovalState,
  type GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim,
  type GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness,
  type GeneratedAppInstallPurchaseApprovalSupportState,
  type GeneratedParentPlatform,
} from './generated/app-install-purchase-approval-contracts';
import {
  appInstallPurchaseApprovalContractProofIsHonestGenerated,
  auditReportIntegrationIsHonestGenerated,
  approvalDecisionIsConsistentGenerated,
  approvalStateSnapshotIsConsistentGenerated,
  childFacingStateIsConsistentGenerated,
  platformSupportRowIsHonestGenerated,
  purchaseRequestKindIsConsistentGenerated,
  storeMetadataFreshnessIsConsistentGenerated,
} from './generated/app-install-purchase-approval-contract-rules';
import type { AppInstallPurchaseApprovalPlatformSourceMetadataRow } from './app-install-purchase-approval-platform-sources';
import type { AppInstallPurchaseApprovalPackageSourceArtifactRow } from './app-install-purchase-approval-package-sources';

type AppInstallPurchaseApprovalPlatform = GeneratedParentPlatform;
type AppInstallPurchaseApprovalRequestKind = GeneratedAppInstallPurchaseApprovalRequestKind;
type AppInstallPurchaseApprovalFreshness = GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness;
type AppInstallPurchaseApprovalSupportState = GeneratedAppInstallPurchaseApprovalSupportState;
type AppInstallPurchaseApprovalState = GeneratedAppInstallPurchaseApprovalState;
type AppInstallPurchaseApprovalExpiryState = GeneratedAppInstallPurchaseApprovalExpiryState;
type AppInstallPurchaseApprovalDecisionAction = GeneratedAppInstallPurchaseApprovalDecisionAction;
type AppInstallPurchaseApprovalPurchaseKind = GeneratedAppInstallPurchaseApprovalPurchaseKind;
type AppInstallPurchaseApprovalChildFacingStatus = GeneratedAppInstallPurchaseApprovalChildFacingStatus;
type AppInstallPurchaseApprovalAuditReportSurface = GeneratedAppInstallPurchaseApprovalAuditReportSurface;
type AppInstallPurchaseApprovalProofIntegrationState = GeneratedAppInstallPurchaseApprovalProofIntegrationState;
type AppInstallPurchaseApprovalNonClaim = GeneratedAppInstallPurchaseApprovalNonClaim;

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
  readonly billingEntitlementClaim: GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim;
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

interface AppInstallPurchaseApprovalChildFacingStateRuleInput {
  readonly childVisibleStatus: AppInstallPurchaseApprovalChildFacingStatus;
  readonly sourceApprovalState: AppInstallPurchaseApprovalStateSnapshotRuleInput;
  readonly deliveryState: AppInstallPurchaseApprovalSupportState;
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: { readonly includes: (needle: 'contract proof' | 'no platform adapter') => boolean };
}

interface AppInstallPurchaseApprovalAuditReportIntegrationRuleInput {
  readonly surface: AppInstallPurchaseApprovalAuditReportSurface;
  readonly integrationState: AppInstallPurchaseApprovalProofIntegrationState;
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly proofRequirement: unknown;
  readonly claimBoundary: { readonly includes: (needle: 'contract proof' | 'no portal runtime') => boolean };
}

interface AppInstallPurchaseApprovalContractProofRuleInput {
  readonly installRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly purchaseRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly subscriptionRequest: { readonly requestKind: AppInstallPurchaseApprovalRequestKind };
  readonly approvalDecisions: readonly AppInstallPurchaseApprovalDecisionRuleInput[];
  readonly platformSupportMatrix: readonly AppInstallPurchaseApprovalPlatformSupportRowRuleInput[];
  readonly platformSourceMetadata: readonly AppInstallPurchaseApprovalPlatformSourceMetadataRow[];
  readonly packageSourceArtifacts: readonly AppInstallPurchaseApprovalPackageSourceArtifactRow[];
  readonly childFacingStates: readonly AppInstallPurchaseApprovalChildFacingStateRuleInput[];
  readonly auditReportIntegration: readonly AppInstallPurchaseApprovalAuditReportIntegrationRuleInput[];
  readonly nonClaims: readonly AppInstallPurchaseApprovalNonClaim[];
  readonly storeIntegrationClaim: GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim;
  readonly billingEntitlementClaim: GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim;
  readonly portalUiClaim: GeneratedAppInstallPurchaseApprovalPortalUiClaim;
  readonly platformAdapterClaim: GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim;
  readonly interceptionClaim: GeneratedAppInstallPurchaseApprovalInterceptionClaim;
  readonly runtimeBlockingSeparation: GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation;
}

export function storeMetadataFreshnessIsConsistent(
  metadata: AppInstallPurchaseApprovalStoreMetadataRuleInput
): boolean {
  return storeMetadataFreshnessIsConsistentGenerated(
    metadata as unknown as Parameters<typeof storeMetadataFreshnessIsConsistentGenerated>[0]
  );
}

export function approvalStateSnapshotIsConsistent(
  snapshot: AppInstallPurchaseApprovalStateSnapshotRuleInput
): boolean {
  return approvalStateSnapshotIsConsistentGenerated(
    snapshot as unknown as Parameters<typeof approvalStateSnapshotIsConsistentGenerated>[0]
  );
}

export function purchaseRequestKindIsConsistent(request: PurchaseRequestRuleInput): boolean {
  return purchaseRequestKindIsConsistentGenerated(
    request as unknown as Parameters<typeof purchaseRequestKindIsConsistentGenerated>[0]
  );
}

export function approvalDecisionIsConsistent(
  decision: AppInstallPurchaseApprovalDecisionRuleInput
): boolean {
  return approvalDecisionIsConsistentGenerated(
    decision as unknown as Parameters<typeof approvalDecisionIsConsistentGenerated>[0]
  );
}

export function platformSupportRowIsHonest(
  row: AppInstallPurchaseApprovalPlatformSupportRowRuleInput
): boolean {
  return platformSupportRowIsHonestGenerated(
    row as unknown as Parameters<typeof platformSupportRowIsHonestGenerated>[0]
  );
}

export function childFacingStateIsConsistent(
  state: AppInstallPurchaseApprovalChildFacingStateRuleInput
): boolean {
  return childFacingStateIsConsistentGenerated(
    state as unknown as Parameters<typeof childFacingStateIsConsistentGenerated>[0]
  );
}

export function auditReportIntegrationIsHonest(
  integration: AppInstallPurchaseApprovalAuditReportIntegrationRuleInput
): boolean {
  return auditReportIntegrationIsHonestGenerated(
    integration as unknown as Parameters<typeof auditReportIntegrationIsHonestGenerated>[0]
  );
}

export function appInstallPurchaseApprovalContractProofIsHonest(
  proof: AppInstallPurchaseApprovalContractProofRuleInput
): boolean {
  return appInstallPurchaseApprovalContractProofIsHonestGenerated(
    proof as unknown as Parameters<typeof appInstallPurchaseApprovalContractProofIsHonestGenerated>[0]
  );
}
