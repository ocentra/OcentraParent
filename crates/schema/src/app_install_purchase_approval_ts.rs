use super::app_install_purchase_approval::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION;
use std::path::Path;

const APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION_PLACEHOLDER: &str =
    "__APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION__";
const SCHEMA_SOURCE_DIRECTORY: &str = "src";
const APP_INSTALL_PURCHASE_PROOF_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_proof_helpers.ts.txt";
const APP_INSTALL_PURCHASE_REPORT_STATUS_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_report_status_helpers.ts.txt";
const APP_INSTALL_PURCHASE_PLATFORM_PROVIDER_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_platform_provider_helpers.ts.txt";
const APP_INSTALL_PURCHASE_PLATFORM_EVIDENCE_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_platform_evidence_helpers.ts.txt";
const APP_INSTALL_PURCHASE_DELIVERY_RUNTIME_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_delivery_runtime_helpers.ts.txt";
const APP_INSTALL_PURCHASE_EXTERNAL_RUNTIME_HELPERS_TYPESCRIPT_PATH: &str =
    "app_install_purchase_external_runtime_helpers.ts.txt";
const APP_INSTALL_PURCHASE_TYPESCRIPT_SIDECAR_READ_ERROR: &str =
    "app install purchase TypeScript sidecar should be readable";

const APP_INSTALL_PURCHASE_APPROVAL_TYPESCRIPT_TEMPLATE: &str = r#"/* generated from crates/schema/src/app_install_purchase_approval.rs */

export const AppInstallPurchaseApprovalContractRuntime = {
  SchemaVersion: '__APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION__',
} as const;

export type GeneratedFamilyId = string;
export type GeneratedChildProfileId = string;
export type GeneratedChildProfileDisplayName = string;
export type GeneratedParentDeviceId = string;
export type GeneratedParentDeviceLabel = string;
export type GeneratedParentActorId = string;
export type GeneratedParentPolicyVersion = string;
export type GeneratedParentEvidenceReferenceId = string;
export type GeneratedParentActionReferenceId = string;
export type GeneratedParentTimestamp = string;
export type GeneratedAppInstallPurchaseApprovalRequestId = string;
export type GeneratedAppInstallPurchaseApprovalDecisionId = string;
export type GeneratedAppInstallPurchaseApprovalAuditEventId = string;
export type GeneratedAppInstallPurchaseApprovalStoreListingId = string;
export type GeneratedAppInstallPurchaseApprovalAppTitle = string;
export type GeneratedAppInstallPurchaseApprovalPublisherName = string;
export type GeneratedAppInstallPurchaseApprovalCategory = string;
export type GeneratedAppInstallPurchaseApprovalAgeRating = string;
export type GeneratedAppInstallPurchaseApprovalReviewReason = string;
export type GeneratedAppInstallPurchaseApprovalProofRequirement = string;
export type GeneratedAppInstallPurchaseApprovalUnavailableReason = string;
export type GeneratedAppInstallPurchaseApprovalManualRequirement = string;
export type GeneratedAppInstallPurchaseApprovalClaimBoundary = string;
export type GeneratedAppInstallPurchaseApprovalPriceDisplay = string;
export type GeneratedAppInstallPurchaseApprovalChildStateId = string;
export type GeneratedAppInstallPurchaseApprovalReportRef = string;
export type GeneratedAppInstallPurchaseApprovalPlatformSourceRowId = string;
export type GeneratedAppInstallPurchaseApprovalPlatformSourceArtifactRequirement = string;
export type GeneratedAppInstallPurchaseApprovalPlatformSourceLimitationReason = string;
export type GeneratedAppInstallPurchaseApprovalPlatformSourceReportRef = string;
export type GeneratedAppInstallPurchaseApprovalPlatformSourceClaimBoundary = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRowId = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceMetadataRowId = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRequirement = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceLimitationReason = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceReportRef = string;
export type GeneratedAppInstallPurchaseApprovalPackageSourceClaimBoundary = string;

export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentActorRole = 'parent' | 'guardian' | 'system';
export type GeneratedParentEvidenceReferenceKind =
  | 'journal-event'
  | 'query-store-summary'
  | 'activity-event'
  | 'policy-decision'
  | 'local-ai-result';

export type GeneratedAppInstallPurchaseApprovalRequestKind = 'install' | 'purchase' | 'subscription';
export type GeneratedAppInstallPurchaseApprovalStoreSurface =
  | 'google-play'
  | 'apple-app-store'
  | 'mac-app-store'
  | 'microsoft-store'
  | 'linux-package-manager'
  | 'parent-manual-entry'
  | 'unknown-store';
export type GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness =
  | 'fresh'
  | 'stale'
  | 'unknown'
  | 'manual-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalSupportState =
  | 'supported'
  | 'manual-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalDecisionAction =
  | 'approve'
  | 'deny'
  | 'time-box'
  | 'review-needed';
export type GeneratedAppInstallPurchaseApprovalState =
  | 'pending-parent-review'
  | 'approved'
  | 'denied'
  | 'time-box-active'
  | 'expired'
  | 'review-needed';
export type GeneratedAppInstallPurchaseApprovalExpiryState =
  | 'not-expiring'
  | 'time-box-active'
  | 'expired'
  | 'review-needed';
export type GeneratedAppInstallPurchaseApprovalPurchaseKind =
  | 'one-time-purchase'
  | 'in-app-purchase'
  | 'subscription';
export type GeneratedAppInstallPurchaseApprovalSubscriptionPeriod =
  | 'weekly'
  | 'monthly'
  | 'annual'
  | 'unknown';
export type GeneratedAppInstallPurchaseApprovalChildFacingStatus =
  | 'pending-parent-review-visible'
  | 'approved-visible'
  | 'denied-visible'
  | 'time-box-visible'
  | 'review-needed-visible';
export type GeneratedAppInstallPurchaseApprovalAuditReportSurface =
  | 'request-audit-history'
  | 'parent-decision-audit-history'
  | 'child-facing-state-report'
  | 'platform-limitation-report';
export type GeneratedAppInstallPurchaseApprovalProofIntegrationState =
  | 'contract-only'
  | 'manual-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalAuditEventKind =
  | 'request-recorded'
  | 'metadata-source-evaluated'
  | 'parent-decision-recorded'
  | 'approval-expired'
  | 'platform-limitation-recorded';
export type GeneratedAppInstallPurchaseApprovalNonClaim =
  | 'no-store-integration'
  | 'no-billing-entitlement-logic'
  | 'no-portal-ui'
  | 'no-platform-adapter'
  | 'no-store-policy-bypass'
  | 'no-real-install-or-purchase-interception'
  | 'not-generic-app-blocking';
export type GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim = 'not-claimed';
export type GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim = 'not-claimed';
export type GeneratedAppInstallPurchaseApprovalPortalUiClaim = 'not-implemented';
export type GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim = 'not-implemented';
export type GeneratedAppInstallPurchaseApprovalInterceptionClaim = 'not-claimed';
export type GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation = 'separate-from-generic-app-blocking';
export type GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority =
  | 'google-play-listing'
  | 'apple-app-store-listing'
  | 'mac-app-store-listing'
  | 'microsoft-store-listing'
  | 'linux-package-manager-index';
export type GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState =
  | 'contract-only'
  | 'manual-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState =
  | 'requires-approved-api-proof'
  | 'requires-store-artifact-proof'
  | 'platform-unavailable';
export type GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataField =
  | 'store-listing-id'
  | 'app-title'
  | 'publisher-name'
  | 'category'
  | 'age-rating'
  | 'price-display'
  | 'subscription-period'
  | 'source-url';
export type GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallback = 'contract-only-parent-review';
export type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus =
  | 'manual-required'
  | 'device-proof-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState =
  | 'manual-required'
  | 'unavailable';
export type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim = 'not-attached';
export type GeneratedAppInstallPurchaseApprovalPackageSourceField =
  | 'package-identifier'
  | 'installer-source'
  | 'publisher-or-developer'
  | 'version-or-build'
  | 'signature-or-receipt'
  | 'source-captured-at';
export type GeneratedAppInstallPurchaseApprovalPackageSourceKind =
  | 'windows-store-package-identity'
  | 'macos-bundle-receipt'
  | 'linux-package-manager-record'
  | 'android-package-source-record'
  | 'ios-app-source-record';
export type GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustody = 'no-child-activity-data';

export const GeneratedParentPlatforms = ['windows', 'linux', 'macos', 'android', 'ios'] as const satisfies readonly GeneratedParentPlatform[];
export const GeneratedParentActorRoles = ['parent', 'guardian', 'system'] as const satisfies readonly GeneratedParentActorRole[];
export const GeneratedParentEvidenceReferenceKinds = [
  'journal-event',
  'query-store-summary',
  'activity-event',
  'policy-decision',
  'local-ai-result',
] as const satisfies readonly GeneratedParentEvidenceReferenceKind[];
export const GeneratedAppInstallPurchaseApprovalRequestKinds = [
  'install',
  'purchase',
  'subscription',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalRequestKind[];
export const GeneratedAppInstallPurchaseApprovalStoreSurfaces = [
  'google-play',
  'apple-app-store',
  'mac-app-store',
  'microsoft-store',
  'linux-package-manager',
  'parent-manual-entry',
  'unknown-store',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalStoreSurface[];
export const GeneratedAppInstallPurchaseApprovalStoreMetadataFreshnessStates = [
  'fresh',
  'stale',
  'unknown',
  'manual-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness[];
export const GeneratedAppInstallPurchaseApprovalSupportStates = [
  'supported',
  'manual-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalSupportState[];
export const GeneratedAppInstallPurchaseApprovalDecisionActions = [
  'approve',
  'deny',
  'time-box',
  'review-needed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalDecisionAction[];
export const GeneratedAppInstallPurchaseApprovalStates = [
  'pending-parent-review',
  'approved',
  'denied',
  'time-box-active',
  'expired',
  'review-needed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalState[];
export const GeneratedAppInstallPurchaseApprovalExpiryStates = [
  'not-expiring',
  'time-box-active',
  'expired',
  'review-needed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalExpiryState[];
export const GeneratedAppInstallPurchaseApprovalPurchaseKinds = [
  'one-time-purchase',
  'in-app-purchase',
  'subscription',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPurchaseKind[];
export const GeneratedAppInstallPurchaseApprovalSubscriptionPeriods = [
  'weekly',
  'monthly',
  'annual',
  'unknown',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalSubscriptionPeriod[];
export const GeneratedAppInstallPurchaseApprovalChildFacingStatuses = [
  'pending-parent-review-visible',
  'approved-visible',
  'denied-visible',
  'time-box-visible',
  'review-needed-visible',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalChildFacingStatus[];
export const GeneratedAppInstallPurchaseApprovalAuditReportSurfaces = [
  'request-audit-history',
  'parent-decision-audit-history',
  'child-facing-state-report',
  'platform-limitation-report',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalAuditReportSurface[];
export const GeneratedAppInstallPurchaseApprovalProofIntegrationStates = [
  'contract-only',
  'manual-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalProofIntegrationState[];
export const GeneratedAppInstallPurchaseApprovalAuditEventKinds = [
  'request-recorded',
  'metadata-source-evaluated',
  'parent-decision-recorded',
  'approval-expired',
  'platform-limitation-recorded',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalAuditEventKind[];
export const GeneratedAppInstallPurchaseApprovalNonClaims = [
  'no-store-integration',
  'no-billing-entitlement-logic',
  'no-portal-ui',
  'no-platform-adapter',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'not-generic-app-blocking',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalNonClaim[];
export const GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims = [
  'not-claimed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim[];
export const GeneratedAppInstallPurchaseApprovalBillingEntitlementClaims = [
  'not-claimed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim[];
export const GeneratedAppInstallPurchaseApprovalPortalUiClaims = [
  'not-implemented',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPortalUiClaim[];
export const GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims = [
  'not-implemented',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim[];
export const GeneratedAppInstallPurchaseApprovalInterceptionClaims = [
  'not-claimed',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalInterceptionClaim[];
export const GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparations = [
  'separate-from-generic-app-blocking',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation[];
export const GeneratedAppInstallPurchaseApprovalPlatformSourceAuthorities = [
  'google-play-listing',
  'apple-app-store-listing',
  'mac-app-store-listing',
  'microsoft-store-listing',
  'linux-package-manager-index',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority[];
export const GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataStates = [
  'contract-only',
  'manual-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState[];
export const GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceStates = [
  'requires-approved-api-proof',
  'requires-store-artifact-proof',
  'platform-unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState[];
export const GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields = [
  'store-listing-id',
  'app-title',
  'publisher-name',
  'category',
  'age-rating',
  'price-display',
  'subscription-period',
  'source-url',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataField[];
export const GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallbacks = [
  'contract-only-parent-review',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallback[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatuses = [
  'manual-required',
  'device-proof-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathStates = [
  'manual-required',
  'unavailable',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaims = [
  'not-attached',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceFields = [
  'package-identifier',
  'installer-source',
  'publisher-or-developer',
  'version-or-build',
  'signature-or-receipt',
  'source-captured-at',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceField[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceKinds = [
  'windows-store-package-identity',
  'macos-bundle-receipt',
  'linux-package-manager-record',
  'android-package-source-record',
  'ios-app-source-record',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceKind[];
export const GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustodyStates = [
  'no-child-activity-data',
] as const satisfies readonly GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustody[];

export interface GeneratedParentActorReference {
  actorId: GeneratedParentActorId;
  role: GeneratedParentActorRole;
}

export interface GeneratedFamilyReference {
  familyId: GeneratedFamilyId;
}

export interface GeneratedChildProfileReference {
  childProfileId: GeneratedChildProfileId;
  displayName: GeneratedChildProfileDisplayName;
}

export interface GeneratedParentDeviceReference {
  deviceId: GeneratedParentDeviceId;
  childProfileId?: GeneratedChildProfileId;
  label: GeneratedParentDeviceLabel;
  platform: GeneratedParentPlatform;
}

export interface GeneratedParentEvidenceReference {
  evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: GeneratedParentTimestamp;
}

export interface GeneratedParentActionReference {
  actionReferenceId: GeneratedParentActionReferenceId;
  actor: GeneratedParentActorReference;
  policyVersion: GeneratedParentPolicyVersion;
  createdAt: GeneratedParentTimestamp;
}

export interface GeneratedAppInstallPurchaseApprovalAuditEventRef {
  auditEventId: GeneratedAppInstallPurchaseApprovalAuditEventId;
  eventKind: GeneratedAppInstallPurchaseApprovalAuditEventKind;
  recordedAt: GeneratedParentTimestamp;
  evidenceReferences: GeneratedParentEvidenceReference[];
}

export interface GeneratedAppInstallPurchaseApprovalStoreMetadata {
  storeSurface: GeneratedAppInstallPurchaseApprovalStoreSurface;
  sourceState: GeneratedAppInstallPurchaseApprovalSupportState;
  freshness: GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness;
  listingId?: GeneratedAppInstallPurchaseApprovalStoreListingId;
  appTitle?: GeneratedAppInstallPurchaseApprovalAppTitle;
  publisherName?: GeneratedAppInstallPurchaseApprovalPublisherName;
  category?: GeneratedAppInstallPurchaseApprovalCategory;
  ageRating?: GeneratedAppInstallPurchaseApprovalAgeRating;
  refreshedAt?: GeneratedParentTimestamp;
  staleAt?: GeneratedParentTimestamp;
  proofRequirement: GeneratedAppInstallPurchaseApprovalProofRequirement;
}

export interface GeneratedAppInstallPurchaseApprovalStateSnapshot {
  state: GeneratedAppInstallPurchaseApprovalState;
  expiryState: GeneratedAppInstallPurchaseApprovalExpiryState;
  expiresAt?: GeneratedParentTimestamp;
  reviewReason?: GeneratedAppInstallPurchaseApprovalReviewReason;
}

export interface GeneratedAppInstallRequest {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  requestId: GeneratedAppInstallPurchaseApprovalRequestId;
  requestKind: GeneratedAppInstallPurchaseApprovalRequestKind;
  family: GeneratedFamilyReference;
  child: GeneratedChildProfileReference;
  device: GeneratedParentDeviceReference;
  platform: GeneratedParentPlatform;
  storeMetadata: GeneratedAppInstallPurchaseApprovalStoreMetadata;
  approvalState: GeneratedAppInstallPurchaseApprovalStateSnapshot;
  requestedAt: GeneratedParentTimestamp;
  evidenceReferences: GeneratedParentEvidenceReference[];
  auditEventRefs: GeneratedAppInstallPurchaseApprovalAuditEventRef[];
}

export interface GeneratedPurchaseRequest {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  requestId: GeneratedAppInstallPurchaseApprovalRequestId;
  requestKind: GeneratedAppInstallPurchaseApprovalRequestKind;
  family: GeneratedFamilyReference;
  child: GeneratedChildProfileReference;
  device: GeneratedParentDeviceReference;
  platform: GeneratedParentPlatform;
  storeMetadata: GeneratedAppInstallPurchaseApprovalStoreMetadata;
  approvalState: GeneratedAppInstallPurchaseApprovalStateSnapshot;
  requestedAt: GeneratedParentTimestamp;
  evidenceReferences: GeneratedParentEvidenceReference[];
  auditEventRefs: GeneratedAppInstallPurchaseApprovalAuditEventRef[];
  purchaseKind: GeneratedAppInstallPurchaseApprovalPurchaseKind;
  subscriptionPeriod?: GeneratedAppInstallPurchaseApprovalSubscriptionPeriod;
  priceDisplay?: GeneratedAppInstallPurchaseApprovalPriceDisplay;
  billingEntitlementClaim: GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim;
}

export interface GeneratedAppInstallPurchaseApprovalDecision {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  decisionId: GeneratedAppInstallPurchaseApprovalDecisionId;
  requestId: GeneratedAppInstallPurchaseApprovalRequestId;
  requestKind: GeneratedAppInstallPurchaseApprovalRequestKind;
  decisionAction: GeneratedAppInstallPurchaseApprovalDecisionAction;
  resultingState: GeneratedAppInstallPurchaseApprovalStateSnapshot;
  parentAction?: GeneratedParentActionReference;
  decidedAt: GeneratedParentTimestamp;
  auditEventRefs: GeneratedAppInstallPurchaseApprovalAuditEventRef[];
}

export interface GeneratedAppInstallPurchaseApprovalChildFacingState {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  childStateId: GeneratedAppInstallPurchaseApprovalChildStateId;
  requestId: GeneratedAppInstallPurchaseApprovalRequestId;
  requestKind: GeneratedAppInstallPurchaseApprovalRequestKind;
  platform: GeneratedParentPlatform;
  childVisibleStatus: GeneratedAppInstallPurchaseApprovalChildFacingStatus;
  sourceApprovalState: GeneratedAppInstallPurchaseApprovalStateSnapshot;
  deliveryState: GeneratedAppInstallPurchaseApprovalSupportState;
  deliveryRequirement: GeneratedAppInstallPurchaseApprovalProofRequirement;
  auditEventRefs: GeneratedAppInstallPurchaseApprovalAuditEventRef[];
  reportRefs: GeneratedAppInstallPurchaseApprovalReportRef[];
  claimBoundary: GeneratedAppInstallPurchaseApprovalClaimBoundary;
}

export interface GeneratedAppInstallPurchaseApprovalAuditReportIntegration {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  surface: GeneratedAppInstallPurchaseApprovalAuditReportSurface;
  integrationState: GeneratedAppInstallPurchaseApprovalProofIntegrationState;
  auditEventRefs: GeneratedAppInstallPurchaseApprovalAuditEventRef[];
  reportRefs: GeneratedAppInstallPurchaseApprovalReportRef[];
  proofRequirement: GeneratedAppInstallPurchaseApprovalProofRequirement;
  claimBoundary: GeneratedAppInstallPurchaseApprovalClaimBoundary;
}

export interface GeneratedAppInstallPurchaseApprovalPlatformSupportRow {
  platform: GeneratedParentPlatform;
  storeSurface: GeneratedAppInstallPurchaseApprovalStoreSurface;
  contractRequestState: GeneratedAppInstallPurchaseApprovalSupportState;
  storeMetadataState: GeneratedAppInstallPurchaseApprovalSupportState;
  installInterceptionState: GeneratedAppInstallPurchaseApprovalSupportState;
  purchaseInterceptionState: GeneratedAppInstallPurchaseApprovalSupportState;
  subscriptionInterceptionState: GeneratedAppInstallPurchaseApprovalSupportState;
  childPendingState: GeneratedAppInstallPurchaseApprovalSupportState;
  approvalDeliveryState: GeneratedAppInstallPurchaseApprovalSupportState;
  manualRequirement?: GeneratedAppInstallPurchaseApprovalManualRequirement;
  unavailableReason?: GeneratedAppInstallPurchaseApprovalUnavailableReason;
  proofRequirement: GeneratedAppInstallPurchaseApprovalProofRequirement;
  claimBoundary: GeneratedAppInstallPurchaseApprovalClaimBoundary;
}

export interface GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  sourceRowId: GeneratedAppInstallPurchaseApprovalPlatformSourceRowId;
  platform: GeneratedParentPlatform;
  storeSurface: GeneratedAppInstallPurchaseApprovalStoreSurface;
  sourceAuthority: GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority;
  metadataState: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState;
  sourceEvidenceState: GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState;
  fieldsAvailableFromContract: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataField[];
  fieldsRequiringPlatformProof: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataField[];
  requestKindCoverage: GeneratedAppInstallPurchaseApprovalRequestKind[];
  requiredArtifacts: GeneratedAppInstallPurchaseApprovalPlatformSourceArtifactRequirement[];
  limitationReason: GeneratedAppInstallPurchaseApprovalPlatformSourceLimitationReason;
  limitationReportRef: GeneratedAppInstallPurchaseApprovalPlatformSourceReportRef;
  parentManualFallback: GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallback;
  storeIntegrationClaim: GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim;
  platformAdapterClaim: GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim;
  interceptionClaim: GeneratedAppInstallPurchaseApprovalInterceptionClaim;
  claimBoundary: GeneratedAppInstallPurchaseApprovalPlatformSourceClaimBoundary;
  lastCheckedAt: GeneratedParentTimestamp;
}

export interface GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  artifactRowId: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRowId;
  platform: GeneratedParentPlatform;
  storeSurface: GeneratedAppInstallPurchaseApprovalStoreSurface;
  platformSourceRowId: GeneratedAppInstallPurchaseApprovalPackageSourceMetadataRowId;
  packageSourceKind: GeneratedAppInstallPurchaseApprovalPackageSourceKind;
  artifactStatus: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus;
  approvalPathState: GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState;
  packageSourceFieldsRequired: GeneratedAppInstallPurchaseApprovalPackageSourceField[];
  packageSourceFieldsAttached: GeneratedAppInstallPurchaseApprovalPackageSourceField[];
  requestKindCoverage: GeneratedAppInstallPurchaseApprovalRequestKind[];
  requiredArtifacts: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRequirement[];
  artifactEvidenceClaim: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim;
  artifactEvidencePath?: string;
  artifactCapturedAt?: GeneratedParentTimestamp;
  limitationReason: GeneratedAppInstallPurchaseApprovalPackageSourceLimitationReason;
  limitationReportRef: GeneratedAppInstallPurchaseApprovalPackageSourceReportRef;
  storeIntegrationClaim: GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim;
  platformAdapterClaim: GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim;
  interceptionClaim: GeneratedAppInstallPurchaseApprovalInterceptionClaim;
  childDataCustody: GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustody;
  claimBoundary: GeneratedAppInstallPurchaseApprovalPackageSourceClaimBoundary;
  lastCheckedAt: GeneratedParentTimestamp;
}

export interface GeneratedAppInstallPurchaseApprovalContractProof {
  schemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  installRequest: GeneratedAppInstallRequest;
  purchaseRequest: GeneratedPurchaseRequest;
  subscriptionRequest: GeneratedPurchaseRequest;
  approvalDecisions: GeneratedAppInstallPurchaseApprovalDecision[];
  platformSupportMatrix: GeneratedAppInstallPurchaseApprovalPlatformSupportRow[];
  platformSourceMetadata: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow[];
  packageSourceArtifacts: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow[];
  childFacingStates: GeneratedAppInstallPurchaseApprovalChildFacingState[];
  auditReportIntegration: GeneratedAppInstallPurchaseApprovalAuditReportIntegration[];
  nonClaims: GeneratedAppInstallPurchaseApprovalNonClaim[];
  storeIntegrationClaim: GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim;
  billingEntitlementClaim: GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim;
  portalUiClaim: GeneratedAppInstallPurchaseApprovalPortalUiClaim;
  platformAdapterClaim: GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim;
  interceptionClaim: GeneratedAppInstallPurchaseApprovalInterceptionClaim;
  runtimeBlockingSeparation: GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation;
  updatedAt: GeneratedParentTimestamp;
}
"#;

pub fn app_install_purchase_approval_contracts_typescript() -> String {
    APP_INSTALL_PURCHASE_APPROVAL_TYPESCRIPT_TEMPLATE.replace(
        APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION_PLACEHOLDER,
        APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION,
    )
}

fn read_app_install_purchase_typescript_sidecar(path: &str) -> String {
    crate::schema_result_or_unreachable(
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(SCHEMA_SOURCE_DIRECTORY)
                .join(path),
        ),
        APP_INSTALL_PURCHASE_TYPESCRIPT_SIDECAR_READ_ERROR,
    )
}

pub fn app_install_purchase_approval_contract_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/app_install_purchase_approval.rs */

import {
  GeneratedAppInstallPurchaseApprovalAuditReportSurfaces,
  GeneratedAppInstallPurchaseApprovalChildFacingStatuses,
  GeneratedAppInstallPurchaseApprovalDecisionActions,
  GeneratedAppInstallPurchaseApprovalNonClaims,
  GeneratedAppInstallPurchaseApprovalPackageSourceFields,
  GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields,
  GeneratedAppInstallPurchaseApprovalRequestKinds,
  GeneratedAppInstallPurchaseApprovalSupportStates,
  GeneratedParentPlatforms,
  type GeneratedAppInstallPurchaseApprovalAuditReportIntegration,
  type GeneratedAppInstallPurchaseApprovalChildFacingState,
  type GeneratedAppInstallPurchaseApprovalContractProof,
  type GeneratedAppInstallPurchaseApprovalDecision,
  type GeneratedAppInstallPurchaseApprovalNonClaim,
  type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow,
  type GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow,
  type GeneratedAppInstallPurchaseApprovalPlatformSupportRow,
  type GeneratedAppInstallPurchaseApprovalStateSnapshot,
  type GeneratedAppInstallPurchaseApprovalStoreMetadata,
  type GeneratedPurchaseRequest,
} from './app-install-purchase-approval-contracts';

const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;

const RequiredPackageSourceRows = [
  ['windows', 'microsoft-store', 'windows-store-package-identity', 'manual-required', 'manual-required'],
  ['macos', 'mac-app-store', 'macos-bundle-receipt', 'manual-required', 'manual-required'],
  ['linux', 'linux-package-manager', 'linux-package-manager-record', 'unavailable', 'unavailable'],
  ['android', 'google-play', 'android-package-source-record', 'device-proof-required', 'manual-required'],
  ['ios', 'apple-app-store', 'ios-app-source-record', 'device-proof-required', 'manual-required'],
] as const;

const RequiredPlatformRows = GeneratedParentPlatforms;
const RequiredDecisionActions = GeneratedAppInstallPurchaseApprovalDecisionActions;
const RequiredRequestKinds = GeneratedAppInstallPurchaseApprovalRequestKinds;
const RequiredSupportStates = GeneratedAppInstallPurchaseApprovalSupportStates;
const RequiredChildFacingStatuses = GeneratedAppInstallPurchaseApprovalChildFacingStatuses;
const RequiredAuditReportSurfaces = GeneratedAppInstallPurchaseApprovalAuditReportSurfaces;
const RequiredNonClaims = GeneratedAppInstallPurchaseApprovalNonClaims;
const RequiredMetadataFields = GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields;
const RequiredPackageSourceFields = GeneratedAppInstallPurchaseApprovalPackageSourceFields;

export function storeMetadataFreshnessIsConsistentGenerated(
  metadata: GeneratedAppInstallPurchaseApprovalStoreMetadata
): boolean {
  if (metadata.freshness === 'fresh') {
    return metadata.sourceState === 'supported' && metadataFieldsArePresentGenerated(metadata);
  }
  if (metadata.freshness === 'stale') {
    return metadata.sourceState === 'supported' && metadataFieldsArePresentGenerated(metadata);
  }
  if (metadata.freshness === 'manual-required') {
    return metadata.sourceState === 'manual-required' && metadataFieldsAreAbsentGenerated(metadata);
  }
  if (metadata.freshness === 'unavailable') {
    return metadata.sourceState === 'unavailable' && metadataFieldsAreAbsentGenerated(metadata);
  }
  return metadata.sourceState !== 'supported' && metadataFieldsAreAbsentGenerated(metadata);
}

export function approvalStateSnapshotIsConsistentGenerated(
  snapshot: GeneratedAppInstallPurchaseApprovalStateSnapshot
): boolean {
  if (snapshot.state === 'time-box-active') {
    return snapshot.expiryState === 'time-box-active' && snapshot.expiresAt != null && snapshot.reviewReason == null;
  }
  if (snapshot.state === 'expired') {
    return snapshot.expiryState === 'expired' && snapshot.expiresAt != null && snapshot.reviewReason == null;
  }
  if (snapshot.state === 'review-needed') {
    return snapshot.expiryState === 'review-needed' && snapshot.expiresAt == null && snapshot.reviewReason != null;
  }
  return snapshot.expiryState === 'not-expiring' && snapshot.expiresAt == null && snapshot.reviewReason == null;
}

export function purchaseRequestKindIsConsistentGenerated(request: GeneratedPurchaseRequest): boolean {
  if (request.requestKind === 'subscription') {
    return request.purchaseKind === 'subscription' && request.subscriptionPeriod != null;
  }
  return request.purchaseKind !== 'subscription' && request.subscriptionPeriod == null;
}

export function approvalDecisionIsConsistentGenerated(
  decision: GeneratedAppInstallPurchaseApprovalDecision
): boolean {
  if (decision.auditEventRefs.length === 0) {
    return false;
  }
  if (decision.decisionAction === 'approve') {
    return decision.resultingState.state === 'approved' && decision.parentAction != null;
  }
  if (decision.decisionAction === 'deny') {
    return decision.resultingState.state === 'denied' && decision.parentAction != null;
  }
  if (decision.decisionAction === 'time-box') {
    return decision.resultingState.state === 'time-box-active' && decision.parentAction != null;
  }
  return decision.resultingState.state === 'review-needed' && decision.parentAction == null;
}

export function platformSupportRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseApprovalPlatformSupportRow
): boolean {
  const gatedStates = [
    row.storeMetadataState,
    row.installInterceptionState,
    row.purchaseInterceptionState,
    row.subscriptionInterceptionState,
    row.childPendingState,
    row.approvalDeliveryState,
  ];
  const manualRequirementIsPresent = !gatedStates.includes('manual-required') || row.manualRequirement != null;
  const unavailableReasonIsPresent = !gatedStates.includes('unavailable') || row.unavailableReason != null;

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

export function childFacingStateIsConsistentGenerated(
  state: GeneratedAppInstallPurchaseApprovalChildFacingState
): boolean {
  return (
    state.auditEventRefs.length > 0 &&
    state.reportRefs.length > 0 &&
    state.deliveryState !== 'supported' &&
    state.claimBoundary.includes('contract proof') &&
    state.claimBoundary.includes('no platform adapter') &&
    childVisibleStatusMatchesApprovalStateGenerated(state)
  );
}

export function auditReportIntegrationIsHonestGenerated(
  integration: GeneratedAppInstallPurchaseApprovalAuditReportIntegration
): boolean {
  return (
    integration.auditEventRefs.length > 0 &&
    integration.reportRefs.length > 0 &&
    integration.proofRequirement.length > 0 &&
    integration.claimBoundary.includes('contract proof') &&
    integration.claimBoundary.includes('no portal runtime')
  );
}

export function appInstallPurchaseApprovalContractProofIsHonestGenerated(
  proof: GeneratedAppInstallPurchaseApprovalContractProof
): boolean {
  return (
    appInstallPurchaseApprovalContractProofShapesAreCompleteGenerated(proof) &&
    appInstallPurchaseApprovalContractProofClaimsAreContractOnlyGenerated(proof)
  );
}

function appInstallPurchaseApprovalContractProofShapesAreCompleteGenerated(
  proof: GeneratedAppInstallPurchaseApprovalContractProof
): boolean {
  return (
    requestKindsArePresentGenerated(proof) &&
    decisionActionsArePresentGenerated(proof.approvalDecisions) &&
    platformMatrixIsCompleteGenerated(proof.platformSupportMatrix) &&
    platformMatrixContainsAllSupportStatesGenerated(proof.platformSupportMatrix) &&
    platformSourceMetadataRowsAreCompleteGenerated(proof.platformSourceMetadata) &&
    packageSourceArtifactRowsAreCompleteGenerated(proof.packageSourceArtifacts) &&
    childFacingStatesAreCompleteGenerated(proof.childFacingStates) &&
    auditReportIntegrationIsCompleteGenerated(proof.auditReportIntegration) &&
    nonClaimsAreCompleteGenerated(proof.nonClaims)
  );
}

function appInstallPurchaseApprovalContractProofClaimsAreContractOnlyGenerated(
  proof: GeneratedAppInstallPurchaseApprovalContractProof
): boolean {
  return (
    proof.storeIntegrationClaim === 'not-claimed' &&
    proof.billingEntitlementClaim === 'not-claimed' &&
    proof.portalUiClaim === 'not-implemented' &&
    proof.platformAdapterClaim === 'not-implemented' &&
    proof.interceptionClaim === 'not-claimed' &&
    proof.runtimeBlockingSeparation === 'separate-from-generic-app-blocking'
  );
}

function platformSourceMetadataRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow[]
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));

  return (
    rows.length === RequiredPlatformSources.length &&
    rowKeys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => rowKeys.has(`${platform}:${storeSurface}`)) &&
    rows.every((row) => platformSourceMetadataRowIsHonestGenerated(row))
  );
}

function platformSourceMetadataRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow
): boolean {
  if (
    row.requiredArtifacts.length === 0 ||
    row.storeIntegrationClaim !== 'not-claimed' ||
    row.platformAdapterClaim !== 'not-implemented' ||
    row.interceptionClaim !== 'not-claimed' ||
    row.parentManualFallback !== 'contract-only-parent-review' ||
    !row.claimBoundary.includes('no store integration') ||
    !row.claimBoundary.includes('no platform adapter') ||
    !row.claimBoundary.includes('no real install or purchase interception') ||
    !arrayContainsEveryGenerated(row.requestKindCoverage, RequiredRequestKinds) ||
    !arrayIsUniqueGenerated(row.requestKindCoverage) ||
    row.fieldsRequiringPlatformProof.length !== RequiredMetadataFields.length ||
    !arrayContainsEveryGenerated(row.fieldsRequiringPlatformProof, RequiredMetadataFields) ||
    !arrayIsUniqueGenerated(row.fieldsRequiringPlatformProof) ||
    !arrayIsUniqueGenerated(row.fieldsAvailableFromContract) ||
    !platformSourceAuthorityMatchesStoreGenerated(row)
  ) {
    return false;
  }

  if (row.metadataState === 'unavailable') {
    return row.sourceEvidenceState === 'platform-unavailable' && row.fieldsAvailableFromContract.length === 0;
  }

  return (
    row.metadataState === 'manual-required' &&
    row.sourceEvidenceState !== 'platform-unavailable' &&
    row.fieldsAvailableFromContract.length === 0
  );
}

function packageSourceArtifactRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow[]
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));

  return (
    rows.length === RequiredPackageSourceRows.length &&
    rowKeys.size === rows.length &&
    RequiredPackageSourceRows.every(([platform, storeSurface, packageSourceKind, artifactStatus, approvalPathState]) => {
      const row = rows.find((entry) => entry.platform === platform && entry.storeSurface === storeSurface);

      return (
        row !== undefined &&
        row.packageSourceKind === packageSourceKind &&
        row.artifactStatus === artifactStatus &&
        row.approvalPathState === approvalPathState
      );
    }) &&
    rows.every((row) => packageSourceArtifactRowIsHonestGenerated(row))
  );
}

function packageSourceArtifactRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow
): boolean {
  return (
    row.requiredArtifacts.length > 0 &&
    row.artifactEvidenceClaim === 'not-attached' &&
    row.artifactEvidencePath == null &&
    row.artifactCapturedAt == null &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.interceptionClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.claimBoundary.includes('contract proof') &&
    row.claimBoundary.includes('no store integration') &&
    row.claimBoundary.includes('no platform adapter') &&
    row.claimBoundary.includes('no real install or purchase interception') &&
    row.claimBoundary.includes('no child activity data') &&
    arrayContainsEveryGenerated(row.requestKindCoverage, RequiredRequestKinds) &&
    arrayIsUniqueGenerated(row.requestKindCoverage) &&
    row.packageSourceFieldsRequired.length === RequiredPackageSourceFields.length &&
    row.packageSourceFieldsAttached.length === 0 &&
    arrayContainsEveryGenerated(row.packageSourceFieldsRequired, RequiredPackageSourceFields) &&
    arrayIsUniqueGenerated(row.packageSourceFieldsRequired) &&
    packageSourceKindMatchesStoreGenerated(row)
  );
}

function childVisibleStatusMatchesApprovalStateGenerated(
  state: GeneratedAppInstallPurchaseApprovalChildFacingState
): boolean {
  if (state.sourceApprovalState.state === 'pending-parent-review') {
    return state.childVisibleStatus === 'pending-parent-review-visible';
  }
  if (state.sourceApprovalState.state === 'approved') {
    return state.childVisibleStatus === 'approved-visible';
  }
  if (state.sourceApprovalState.state === 'denied') {
    return state.childVisibleStatus === 'denied-visible';
  }
  if (state.sourceApprovalState.state === 'time-box-active') {
    return state.childVisibleStatus === 'time-box-visible';
  }
  return state.sourceApprovalState.state === 'review-needed' && state.childVisibleStatus === 'review-needed-visible';
}

function metadataFieldsArePresentGenerated(
  metadata: GeneratedAppInstallPurchaseApprovalStoreMetadata
): boolean {
  return (
    metadata.listingId != null &&
    metadata.appTitle != null &&
    metadata.publisherName != null &&
    metadata.category != null &&
    metadata.ageRating != null &&
    metadata.refreshedAt != null &&
    metadata.staleAt != null
  );
}

function metadataFieldsAreAbsentGenerated(
  metadata: GeneratedAppInstallPurchaseApprovalStoreMetadata
): boolean {
  return (
    metadata.listingId == null &&
    metadata.appTitle == null &&
    metadata.publisherName == null &&
    metadata.category == null &&
    metadata.ageRating == null &&
    metadata.refreshedAt == null &&
    metadata.staleAt == null
  );
}

function requestKindsArePresentGenerated(proof: GeneratedAppInstallPurchaseApprovalContractProof): boolean {
  return (
    proof.installRequest.requestKind === 'install' &&
    proof.purchaseRequest.requestKind === 'purchase' &&
    proof.subscriptionRequest.requestKind === 'subscription'
  );
}

function decisionActionsArePresentGenerated(
  decisions: readonly GeneratedAppInstallPurchaseApprovalDecision[]
): boolean {
  const actions = new Set(decisions.map((decision) => decision.decisionAction));
  return RequiredDecisionActions.every((action) => actions.has(action));
}

function platformMatrixIsCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseApprovalPlatformSupportRow[]
): boolean {
  const byPlatform = new Map(rows.map((row) => [row.platform, row] as const));
  return byPlatform.size === rows.length && RequiredPlatformRows.every((platform) => byPlatform.has(platform));
}

function platformMatrixContainsAllSupportStatesGenerated(
  rows: readonly GeneratedAppInstallPurchaseApprovalPlatformSupportRow[]
): boolean {
  const states = new Set<typeof RequiredSupportStates[number]>();
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

function childFacingStatesAreCompleteGenerated(
  states: readonly GeneratedAppInstallPurchaseApprovalChildFacingState[]
): boolean {
  const statuses = new Set(states.map((state) => state.childVisibleStatus));
  return (
    RequiredChildFacingStatuses.every((status) => statuses.has(status)) &&
    states.every((state) => childFacingStateIsConsistentGenerated(state))
  );
}

function auditReportIntegrationIsCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseApprovalAuditReportIntegration[]
): boolean {
  const surfaces = new Set(rows.map((row) => row.surface));
  return (
    RequiredAuditReportSurfaces.every((surface) => surfaces.has(surface)) &&
    rows.every((row) => auditReportIntegrationIsHonestGenerated(row))
  );
}

function nonClaimsAreCompleteGenerated(nonClaims: readonly GeneratedAppInstallPurchaseApprovalNonClaim[]): boolean {
  const nonClaimSet = new Set(nonClaims);
  return RequiredNonClaims.every((nonClaim) => nonClaimSet.has(nonClaim));
}

function platformSourceAuthorityMatchesStoreGenerated(
  row: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataRow
): boolean {
  if (row.storeSurface === 'google-play') {
    return row.sourceAuthority === 'google-play-listing';
  }
  if (row.storeSurface === 'apple-app-store') {
    return row.sourceAuthority === 'apple-app-store-listing';
  }
  if (row.storeSurface === 'mac-app-store') {
    return row.sourceAuthority === 'mac-app-store-listing';
  }
  if (row.storeSurface === 'microsoft-store') {
    return row.sourceAuthority === 'microsoft-store-listing';
  }
  return row.sourceAuthority === 'linux-package-manager-index';
}

function packageSourceKindMatchesStoreGenerated(
  row: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRow
): boolean {
  if (row.storeSurface === 'microsoft-store') {
    return row.packageSourceKind === 'windows-store-package-identity';
  }
  if (row.storeSurface === 'mac-app-store') {
    return row.packageSourceKind === 'macos-bundle-receipt';
  }
  if (row.storeSurface === 'linux-package-manager') {
    return row.packageSourceKind === 'linux-package-manager-record';
  }
  if (row.storeSurface === 'google-play') {
    return row.packageSourceKind === 'android-package-source-record';
  }
  return row.packageSourceKind === 'ios-app-source-record';
}

function arrayContainsEveryGenerated<T extends string>(
  values: readonly T[],
  requiredValues: readonly T[]
): boolean {
  const valueSet = new Set(values);
  return requiredValues.every((value) => valueSet.has(value));
}

function arrayIsUniqueGenerated<T extends string>(values: readonly T[]): boolean {
  return new Set(values).size === values.length;
}
"#
    .to_string()
}

pub fn app_install_purchase_runtime_proof_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/app_install_purchase_approval.rs */

import { AppInstallPurchaseApprovalContractRuntime } from './app-install-purchase-approval-contracts';

export interface GeneratedAppInstallPurchaseRuntimeClaimBoundary {
  readonly includes: (needle: string) => boolean;
}

export interface GeneratedAppInstallPurchaseRuntimePlatformArtifactRow {
  readonly platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios';
  readonly storeSurface:
    | 'microsoft-store'
    | 'mac-app-store'
    | 'linux-package-manager'
    | 'google-play'
    | 'apple-app-store';
  readonly storeMetadataArtifactState: 'requires-platform-artifact' | 'platform-unavailable';
  readonly packageSourceArtifactState:
    | 'requires-package-source-artifact'
    | 'requires-device-proof-artifact'
    | 'platform-unavailable';
  readonly childPendingDeliveryState: 'manual-required' | 'unavailable';
  readonly childResultDeliveryState: 'manual-required' | 'unavailable';
  readonly reportIntegrationState: 'contract-only' | 'manual-required';
  readonly runtimeClaimState: 'boundary-only';
  readonly requiredProofRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary;
}

export interface GeneratedAppInstallPurchaseRuntimeChildDeliveryRow {
  readonly childVisibleStatus:
    | 'pending-parent-review-visible'
    | 'approved-visible'
    | 'denied-visible'
    | 'time-box-visible'
    | 'review-needed-visible';
  readonly deliveryState: 'manual-required' | 'unavailable';
  readonly runtimeDeliveryClaim: 'not-delivered';
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary;
}

export interface GeneratedAppInstallPurchaseRuntimeReportIntegrationRow {
  readonly surface:
    | 'request-audit-history'
    | 'parent-decision-audit-history'
    | 'child-facing-state-report'
    | 'platform-limitation-report';
  readonly integrationState: 'contract-only' | 'manual-required';
  readonly runtimeReportClaim: 'not-delivered';
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary;
}

export interface GeneratedAppInstallPurchaseRuntimeStatusReadinessRow {
  readonly childVisibleStatus:
    | 'pending-parent-review-visible'
    | 'approved-visible'
    | 'denied-visible'
    | 'time-box-visible'
    | 'review-needed-visible';
  readonly sourceDeliveryState: 'manual-required' | 'unavailable';
  readonly sourceRuntimeDeliveryClaim: 'not-delivered';
  readonly statusReadinessClaim: 'runtime-status-readiness-only';
  readonly runtimeStatusReaderClaim: 'not-implemented';
  readonly childDeliveryClaim: 'not-delivered';
  readonly reportRuntimeDeliveryClaim: 'not-delivered';
  readonly storeIntegrationClaim: 'not-claimed';
  readonly platformAdapterClaim: 'not-implemented';
  readonly appBlockingClaim: 'not-claimed';
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary;
}

export interface GeneratedAppInstallPurchaseRuntimeProof {
  readonly sourceContractSchemaVersion: typeof AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
  readonly platformRuntimeArtifacts: readonly GeneratedAppInstallPurchaseRuntimePlatformArtifactRow[];
  readonly childDeliveryBoundaries: readonly GeneratedAppInstallPurchaseRuntimeChildDeliveryRow[];
  readonly reportIntegrationBoundaries: readonly GeneratedAppInstallPurchaseRuntimeReportIntegrationRow[];
  readonly statusReadinessBoundaries: readonly GeneratedAppInstallPurchaseRuntimeStatusReadinessRow[];
  readonly nonClaims: readonly (
    | 'no-store-integration'
    | 'no-billing-entitlement-logic'
    | 'no-runtime-status-reader-implementation'
    | 'no-platform-adapter'
    | 'no-child-device-delivery'
    | 'no-runtime-report-delivery'
    | 'no-store-policy-bypass'
    | 'no-real-install-or-purchase-interception'
    | 'not-generic-app-blocking'
  )[];
  readonly knownGaps: readonly unknown[];
}

const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredChildStatuses = [
  'pending-parent-review-visible',
  'approved-visible',
  'denied-visible',
  'time-box-visible',
  'review-needed-visible',
] as const;
const RequiredReportSurfaces = [
  'request-audit-history',
  'parent-decision-audit-history',
  'child-facing-state-report',
  'platform-limitation-report',
] as const;
const RequiredNonClaims = [
  'no-store-integration',
  'no-billing-entitlement-logic',
  'no-runtime-status-reader-implementation',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'not-generic-app-blocking',
] as const;

export function appInstallPurchaseRuntimeProofIsHonestGenerated(
  proof: GeneratedAppInstallPurchaseRuntimeProof
): boolean {
  return (
    proof.sourceContractSchemaVersion === AppInstallPurchaseApprovalContractRuntime.SchemaVersion &&
    platformRuntimeArtifactRowsAreCompleteGenerated(proof.platformRuntimeArtifacts) &&
    childDeliveryRowsAreCompleteGenerated(proof.childDeliveryBoundaries) &&
    reportIntegrationRowsAreCompleteGenerated(proof.reportIntegrationBoundaries) &&
    statusReadinessRowsAreCompleteGenerated(proof.statusReadinessBoundaries) &&
    nonClaimsAreCompleteGenerated(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

export function appInstallPurchaseRuntimePlatformArtifactRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  if (!platformRuntimeRowHasRequiredRefsGenerated(row) || row.runtimeClaimState !== 'boundary-only') {
    return false;
  }
  if (row.platform === 'linux') {
    return linuxPlatformRuntimeRowIsHonestGenerated(row);
  }
  return availablePlatformRuntimeRowIsHonestGenerated(row);
}

export function appInstallPurchaseRuntimeChildDeliveryRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimeChildDeliveryRow
): boolean {
  return (
    row.deliveryState === 'manual-required' &&
    row.runtimeDeliveryClaim === 'not-delivered' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.claimBoundary.includes('no child-device delivery')
  );
}

export function appInstallPurchaseRuntimeReportIntegrationRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimeReportIntegrationRow
): boolean {
  return (
    row.runtimeReportClaim === 'not-delivered' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.claimBoundary.includes('no runtime report delivery')
  );
}

export function appInstallPurchaseRuntimeStatusReadinessRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimeStatusReadinessRow
): boolean {
  return (
    row.sourceDeliveryState === 'manual-required' &&
    row.sourceRuntimeDeliveryClaim === 'not-delivered' &&
    row.statusReadinessClaim === 'runtime-status-readiness-only' &&
    row.runtimeStatusReaderClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.reportRuntimeDeliveryClaim === 'not-delivered' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    statusReadinessBoundaryIsExplicitGenerated(row.claimBoundary)
  );
}

function platformRuntimeArtifactRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseRuntimePlatformArtifactRow[]
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPlatformSources.length &&
    rowKeys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => rowKeys.has(`${platform}:${storeSurface}`)) &&
    rows.every((row) => appInstallPurchaseRuntimePlatformArtifactRowIsHonestGenerated(row))
  );
}

function childDeliveryRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseRuntimeChildDeliveryRow[]
): boolean {
  const statuses = new Set(rows.map((row) => row.childVisibleStatus));
  return (
    rows.length === RequiredChildStatuses.length &&
    RequiredChildStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => appInstallPurchaseRuntimeChildDeliveryRowIsHonestGenerated(row))
  );
}

function reportIntegrationRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseRuntimeReportIntegrationRow[]
): boolean {
  const surfaces = new Set(rows.map((row) => row.surface));
  return (
    rows.length === RequiredReportSurfaces.length &&
    RequiredReportSurfaces.every((surface) => surfaces.has(surface)) &&
    rows.every((row) => appInstallPurchaseRuntimeReportIntegrationRowIsHonestGenerated(row))
  );
}

function statusReadinessRowsAreCompleteGenerated(
  rows: readonly GeneratedAppInstallPurchaseRuntimeStatusReadinessRow[]
): boolean {
  const statuses = new Set(rows.map((row) => row.childVisibleStatus));
  return (
    rows.length === RequiredChildStatuses.length &&
    RequiredChildStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => appInstallPurchaseRuntimeStatusReadinessRowIsHonestGenerated(row))
  );
}

function nonClaimsAreCompleteGenerated(nonClaims: readonly GeneratedAppInstallPurchaseRuntimeProof['nonClaims'][number][]): boolean {
  const nonClaimSet = new Set(nonClaims);
  return RequiredNonClaims.every((nonClaim) => nonClaimSet.has(nonClaim));
}

function platformRuntimeRowHasRequiredRefsGenerated(
  row: GeneratedAppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  return (
    runtimeBoundaryIsExplicitGenerated(row.claimBoundary) &&
    row.requiredProofRefs.length > 0 &&
    row.reportRefs.length > 0
  );
}

function linuxPlatformRuntimeRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  return (
    row.storeMetadataArtifactState === 'platform-unavailable' &&
    row.packageSourceArtifactState === 'platform-unavailable' &&
    row.childPendingDeliveryState === 'unavailable' &&
    row.childResultDeliveryState === 'unavailable'
  );
}

function availablePlatformRuntimeRowIsHonestGenerated(
  row: GeneratedAppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  return (
    row.storeMetadataArtifactState === 'requires-platform-artifact' &&
    (row.packageSourceArtifactState === 'requires-package-source-artifact' ||
      row.packageSourceArtifactState === 'requires-device-proof-artifact') &&
    row.childPendingDeliveryState === 'manual-required' &&
    row.childResultDeliveryState === 'manual-required'
  );
}

function runtimeBoundaryIsExplicitGenerated(
  boundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary
): boolean {
  return (
    boundary.includes('no runtime status reader implementation') &&
    boundary.includes('no store integration') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('not generic app blocking')
  );
}

function statusReadinessBoundaryIsExplicitGenerated(
  boundary: GeneratedAppInstallPurchaseRuntimeClaimBoundary
): boolean {
  return (
    boundary.includes('no runtime status reader implementation') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no store integration') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('not generic app blocking')
  );
}
"#
    .to_string()
}

pub fn app_install_purchase_proof_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(APP_INSTALL_PURCHASE_PROOF_HELPERS_TYPESCRIPT_PATH)
}

pub fn app_install_purchase_report_status_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(
        APP_INSTALL_PURCHASE_REPORT_STATUS_HELPERS_TYPESCRIPT_PATH,
    )
}

pub fn app_install_purchase_platform_provider_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(
        APP_INSTALL_PURCHASE_PLATFORM_PROVIDER_HELPERS_TYPESCRIPT_PATH,
    )
}

pub fn app_install_purchase_platform_evidence_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(
        APP_INSTALL_PURCHASE_PLATFORM_EVIDENCE_HELPERS_TYPESCRIPT_PATH,
    )
}

pub fn app_install_purchase_delivery_runtime_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(
        APP_INSTALL_PURCHASE_DELIVERY_RUNTIME_HELPERS_TYPESCRIPT_PATH,
    )
}

pub fn app_install_purchase_external_runtime_helpers_typescript() -> String {
    read_app_install_purchase_typescript_sidecar(
        APP_INSTALL_PURCHASE_EXTERNAL_RUNTIME_HELPERS_TYPESCRIPT_PATH,
    )
}
