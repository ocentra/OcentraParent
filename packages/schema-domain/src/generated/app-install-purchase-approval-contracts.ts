/* generated from crates/schema/src/app_install_purchase_approval.rs */

export const AppInstallPurchaseApprovalContractRuntime = {
  SchemaVersion: 'app-install-purchase-approval-contract-proof',
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
