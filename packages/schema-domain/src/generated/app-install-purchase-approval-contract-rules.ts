/* generated from crates/schema/src/app_install_purchase_approval.rs */

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
