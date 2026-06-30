/* generated from crates/schema/src/app_install_purchase_proof_helpers.ts.txt */

import {
  AppInstallPurchaseApprovalContractRuntime,
  GeneratedAppInstallPurchaseApprovalPackageSourceFields,
  GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields,
  GeneratedAppInstallPurchaseApprovalRequestKinds,
  type GeneratedAppInstallPurchaseApprovalAuditReportSurface,
  type GeneratedAppInstallPurchaseApprovalDecisionAction,
  type GeneratedAppInstallPurchaseApprovalExpiryState,
  type GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState,
  type GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus,
  type GeneratedAppInstallPurchaseApprovalPackageSourceKind,
  type GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority,
  type GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState,
  type GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState,
  type GeneratedAppInstallPurchaseApprovalProofIntegrationState,
  type GeneratedAppInstallPurchaseApprovalRequestKind,
  type GeneratedAppInstallPurchaseApprovalState,
  type GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness,
  type GeneratedAppInstallPurchaseApprovalStoreSurface,
  type GeneratedParentPlatform,
} from './app-install-purchase-approval-contracts';

const RequiredPlatformSourcesGenerated = [
  { platform: 'windows', storeSurface: 'microsoft-store' },
  { platform: 'macos', storeSurface: 'mac-app-store' },
  { platform: 'linux', storeSurface: 'linux-package-manager' },
  { platform: 'android', storeSurface: 'google-play' },
  { platform: 'ios', storeSurface: 'apple-app-store' },
] as const;

const RequiredPackageSourceRowsGenerated = [
  {
    platform: 'windows',
    storeSurface: 'microsoft-store',
    packageSourceKind: 'windows-store-package-identity',
    artifactStatus: 'manual-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'macos',
    storeSurface: 'mac-app-store',
    packageSourceKind: 'macos-bundle-receipt',
    artifactStatus: 'manual-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'linux',
    storeSurface: 'linux-package-manager',
    packageSourceKind: 'linux-package-manager-record',
    artifactStatus: 'unavailable',
    approvalPathState: 'unavailable',
  },
  {
    platform: 'android',
    storeSurface: 'google-play',
    packageSourceKind: 'android-package-source-record',
    artifactStatus: 'device-proof-required',
    approvalPathState: 'manual-required',
  },
  {
    platform: 'ios',
    storeSurface: 'apple-app-store',
    packageSourceKind: 'ios-app-source-record',
    artifactStatus: 'device-proof-required',
    approvalPathState: 'manual-required',
  },
] as const;

const PlatformSourceRequestKindsGenerated = GeneratedAppInstallPurchaseApprovalRequestKinds;
const PlatformSourceMetadataFieldsGenerated = GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataFields;
const PackageSourceFieldsGenerated = GeneratedAppInstallPurchaseApprovalPackageSourceFields;

export const AppInstallPurchaseApprovalReportRefsGenerated = {
  RequestAudit: 'app-install-purchase-request-audit-report-ref',
  DecisionAudit: 'app-install-purchase-decision-audit-report-ref',
  ChildFacing: 'app-install-purchase-child-facing-report-ref',
  PlatformLimitation: 'app-install-purchase-platform-limitation-report-ref',
} as const;

const ContractBoundaryGenerated =
  'contract proof only; no platform adapter no store integration no portal runtime no child-device delivery';

const ChildFacingStateInputsGenerated = [
  {
    childStateId: 'child-state-install-pending-parent-review',
    requestId: 'install-request-proof-1',
    requestKind: 'install',
    childVisibleStatus: 'pending-parent-review-visible',
    sourceApprovalState: {
      state: 'pending-parent-review',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'request',
  },
  {
    childStateId: 'child-state-install-approved',
    requestId: 'install-request-proof-1',
    requestKind: 'install',
    childVisibleStatus: 'approved-visible',
    sourceApprovalState: {
      state: 'approved',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-purchase-denied',
    requestId: 'purchase-request-proof-1',
    requestKind: 'purchase',
    childVisibleStatus: 'denied-visible',
    sourceApprovalState: {
      state: 'denied',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-subscription-time-box',
    requestId: 'subscription-request-proof-1',
    requestKind: 'subscription',
    childVisibleStatus: 'time-box-visible',
    sourceApprovalState: {
      state: 'time-box-active',
      expiryState: 'time-box-active',
      expiresAt: '2026-06-10T07:10:00.000Z',
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-purchase-review-needed',
    requestId: 'purchase-request-proof-1',
    requestKind: 'purchase',
    childVisibleStatus: 'review-needed-visible',
    sourceApprovalState: {
      state: 'review-needed',
      expiryState: 'review-needed',
      expiresAt: null,
      reviewReason: 'age rating changed',
    },
    auditSource: 'decision',
  },
] as const;

export function appInstallPurchaseApprovalPlatformSourceMetadataRowIsHonestGenerated(
  row: {
    readonly storeSurface: string;
    readonly sourceAuthority: string;
    readonly metadataState: string;
    readonly sourceEvidenceState: string;
    readonly fieldsAvailableFromContract: readonly unknown[];
    readonly fieldsRequiringPlatformProof: readonly string[];
    readonly requestKindCoverage: readonly string[];
    readonly requiredArtifacts: readonly unknown[];
    readonly parentManualFallback: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly interceptionClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  }
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
    !arrayContainsEveryGenerated(row.requestKindCoverage, PlatformSourceRequestKindsGenerated) ||
    !arrayIsUniqueGenerated(row.requestKindCoverage) ||
    row.fieldsRequiringPlatformProof.length !== PlatformSourceMetadataFieldsGenerated.length ||
    !arrayContainsEveryGenerated(row.fieldsRequiringPlatformProof, PlatformSourceMetadataFieldsGenerated) ||
    !arrayIsUniqueGenerated(row.fieldsRequiringPlatformProof) ||
    !arrayIsUniqueGenerated(row.fieldsAvailableFromContract) ||
    !platformSourceAuthorityMatchesStoreGenerated(row.storeSurface, row.sourceAuthority)
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

export function appInstallPurchaseApprovalPlatformSourceMetadataRowsAreCompleteGenerated(
  rows: ReadonlyArray<{
    readonly platform: string;
    readonly storeSurface: string;
    readonly sourceAuthority: string;
    readonly metadataState: string;
    readonly sourceEvidenceState: string;
    readonly fieldsAvailableFromContract: readonly unknown[];
    readonly fieldsRequiringPlatformProof: readonly string[];
    readonly requestKindCoverage: readonly string[];
    readonly requiredArtifacts: readonly unknown[];
    readonly parentManualFallback: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly interceptionClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  }>
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPlatformSourcesGenerated.length &&
    RequiredPlatformSourcesGenerated.every((source) => rowKeys.has(`${source.platform}:${source.storeSurface}`)) &&
    rows.every((row) => appInstallPurchaseApprovalPlatformSourceMetadataRowIsHonestGenerated(row))
  );
}

export function appInstallPurchaseApprovalPackageSourceArtifactRowIsHonestGenerated(
  row: {
    readonly storeSurface: string;
    readonly packageSourceKind: string;
    readonly artifactStatus: string;
    readonly approvalPathState: string;
    readonly packageSourceFieldsRequired: readonly string[];
    readonly packageSourceFieldsAttached: readonly unknown[];
    readonly requestKindCoverage: readonly string[];
    readonly requiredArtifacts: readonly unknown[];
    readonly artifactEvidenceClaim: string;
    readonly artifactEvidencePath: unknown | null;
    readonly artifactCapturedAt: unknown | null;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly interceptionClaim: string;
    readonly childDataCustody: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  }
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
    arrayContainsEveryGenerated(row.requestKindCoverage, PlatformSourceRequestKindsGenerated) &&
    arrayIsUniqueGenerated(row.requestKindCoverage) &&
    row.packageSourceFieldsRequired.length === PackageSourceFieldsGenerated.length &&
    row.packageSourceFieldsAttached.length === 0 &&
    arrayContainsEveryGenerated(row.packageSourceFieldsRequired, PackageSourceFieldsGenerated) &&
    arrayIsUniqueGenerated(row.packageSourceFieldsRequired) &&
    packageSourceKindMatchesStoreGenerated(row.storeSurface, row.packageSourceKind) &&
    artifactStatusMatchesApprovalPathGenerated(row.artifactStatus, row.approvalPathState)
  );
}

export function appInstallPurchaseApprovalPackageSourceArtifactRowsAreCompleteGenerated(
  rows: ReadonlyArray<{
    readonly platform: string;
    readonly storeSurface: string;
    readonly packageSourceKind: string;
    readonly artifactStatus: string;
    readonly approvalPathState: string;
    readonly packageSourceFieldsRequired: readonly string[];
    readonly packageSourceFieldsAttached: readonly unknown[];
    readonly requestKindCoverage: readonly string[];
    readonly requiredArtifacts: readonly unknown[];
    readonly artifactEvidenceClaim: string;
    readonly artifactEvidencePath: unknown | null;
    readonly artifactCapturedAt: unknown | null;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly interceptionClaim: string;
    readonly childDataCustody: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  }>
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPackageSourceRowsGenerated.length &&
    rowKeys.size === rows.length &&
    RequiredPackageSourceRowsGenerated.every((source) => {
      const row = rows.find((entry) => `${entry.platform}:${entry.storeSurface}` === `${source.platform}:${source.storeSurface}`);
      return (
        row !== undefined &&
        row.packageSourceKind === source.packageSourceKind &&
        row.artifactStatus === source.artifactStatus &&
        row.approvalPathState === source.approvalPathState
      );
    }) &&
    rows.every((row) => appInstallPurchaseApprovalPackageSourceArtifactRowIsHonestGenerated(row))
  );
}

export function appInstallPurchaseApprovalChildFacingStatesGenerated(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return ChildFacingStateInputsGenerated.map(({ auditSource, ...stateInput }) => ({
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
    platform: 'android',
    deliveryState: 'manual-required',
    deliveryRequirement: 'real child-device agent delivery proof before child-visible status can be claimed',
    reportRefs: [AppInstallPurchaseApprovalReportRefsGenerated.ChildFacing],
    claimBoundary: ContractBoundaryGenerated,
    ...stateInput,
    auditEventRefs: [auditSource === 'request' ? input.requestAuditEvent : input.decisionAuditEvent],
  }));
}

export function appInstallPurchaseApprovalAuditReportIntegrationGenerated(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return [
    buildApprovalAuditReportRowGenerated(
      'request-audit-history',
      'contract-only',
      [input.requestAuditEvent],
      [AppInstallPurchaseApprovalReportRefsGenerated.RequestAudit]
    ),
    buildApprovalAuditReportRowGenerated(
      'parent-decision-audit-history',
      'contract-only',
      [input.decisionAuditEvent],
      [AppInstallPurchaseApprovalReportRefsGenerated.DecisionAudit]
    ),
    buildApprovalAuditReportRowGenerated(
      'child-facing-state-report',
      'manual-required',
      [input.requestAuditEvent, input.decisionAuditEvent],
      [AppInstallPurchaseApprovalReportRefsGenerated.ChildFacing]
    ),
    buildApprovalAuditReportRowGenerated(
      'platform-limitation-report',
      'manual-required',
      [input.requestAuditEvent],
      [AppInstallPurchaseApprovalReportRefsGenerated.PlatformLimitation]
    ),
  ] as const;
}

export function summarizeAppInstallPurchaseApprovalSupportStatesGenerated(
  rows: ReadonlyArray<{
    readonly contractRequestState: 'supported' | 'manual-required' | 'unavailable';
    readonly storeMetadataState: 'supported' | 'manual-required' | 'unavailable';
    readonly installInterceptionState: 'supported' | 'manual-required' | 'unavailable';
    readonly purchaseInterceptionState: 'supported' | 'manual-required' | 'unavailable';
    readonly subscriptionInterceptionState: 'supported' | 'manual-required' | 'unavailable';
    readonly childPendingState: 'supported' | 'manual-required' | 'unavailable';
    readonly approvalDeliveryState: 'supported' | 'manual-required' | 'unavailable';
  }>
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

export function buildAppInstallPurchaseApprovalRequestGenerated(
  requestId: 'install-request-proof-1' | 'purchase-request-proof-1' | 'subscription-request-proof-1',
  requestKind: GeneratedAppInstallPurchaseApprovalRequestKind,
  platform: Extract<GeneratedParentPlatform, 'android'>,
  timestamp: string,
  expiryTimestamp: string,
  evidenceReference: unknown,
  requestAuditEvent: unknown
) {
  return {
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
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
    storeMetadata: buildAppInstallPurchaseApprovalStoreMetadataGenerated('parent-manual-entry', 'fresh', timestamp, expiryTimestamp),
    approvalState: {
      state: 'pending-parent-review',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    requestedAt: timestamp,
    evidenceReferences: [evidenceReference],
    auditEventRefs: [requestAuditEvent],
  } as const;
}

export function buildAppInstallPurchaseApprovalStoreMetadataGenerated(
  storeSurface: Extract<
    GeneratedAppInstallPurchaseApprovalStoreSurface,
    'parent-manual-entry' | 'google-play' | 'apple-app-store'
  >,
  freshness: Extract<
    GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness,
    'fresh' | 'stale' | 'manual-required'
  >,
  timestamp: string,
  expiryTimestamp: string
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
    refreshedAt: freshness === 'manual-required' ? null : timestamp,
    staleAt: freshness === 'manual-required' ? null : expiryTimestamp,
    proofRequirement: `${storeSurface} metadata remains contract proof until platform source artifacts exist`,
  } as const;
}

export function buildAppInstallPurchaseApprovalDecisionGenerated(
  decisionAction: GeneratedAppInstallPurchaseApprovalDecisionAction,
  state: Extract<GeneratedAppInstallPurchaseApprovalState, 'approved' | 'denied' | 'time-box-active' | 'review-needed'>,
  expiryState: Extract<
    GeneratedAppInstallPurchaseApprovalExpiryState,
    'not-expiring' | 'time-box-active' | 'review-needed'
  >,
  expiresAt: string | null,
  reviewReason: 'age rating changed' | null,
  parentAction: unknown | null,
  timestamp: string,
  decisionAuditEvent: unknown
) {
  return {
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
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
    decidedAt: timestamp,
    auditEventRefs: [decisionAuditEvent],
  } as const;
}

export function buildAppInstallPurchaseApprovalPlatformSupportRowGenerated(
  platform: GeneratedParentPlatform,
  storeSurface: Extract<
    GeneratedAppInstallPurchaseApprovalStoreSurface,
    'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store'
  >,
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

export function buildAppInstallPurchaseApprovalPlatformSourceMetadataRowGenerated(
  sourceRowId:
    | 'platform-source-windows-microsoft-store'
    | 'platform-source-macos-mac-app-store'
    | 'platform-source-linux-package-manager'
    | 'platform-source-android-google-play'
    | 'platform-source-ios-apple-app-store',
  platform: GeneratedParentPlatform,
  storeSurface: Extract<
    GeneratedAppInstallPurchaseApprovalStoreSurface,
    'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store'
  >,
  sourceAuthority: GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority,
  metadataState: GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState,
  sourceEvidenceState: GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState,
  requiredArtifacts: readonly [string, string, string],
  limitationReason: string,
  timestamp: string
) {
  return {
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
    sourceRowId,
    platform,
    storeSurface,
    sourceAuthority,
    metadataState,
    sourceEvidenceState,
    fieldsAvailableFromContract: [],
    fieldsRequiringPlatformProof: PlatformSourceMetadataFieldsGenerated,
    requestKindCoverage: PlatformSourceRequestKindsGenerated,
    requiredArtifacts,
    limitationReason,
    limitationReportRef: AppInstallPurchaseApprovalReportRefsGenerated.PlatformLimitation,
    parentManualFallback: 'contract-only-parent-review',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    interceptionClaim: 'not-claimed',
    claimBoundary:
      'contract proof only; no store integration no platform adapter no real install or purchase interception',
    lastCheckedAt: timestamp,
  } as const;
}

export function buildAppInstallPurchaseApprovalPackageSourceArtifactRowGenerated(
  artifactRowId:
    | 'package-source-windows-microsoft-store'
    | 'package-source-macos-mac-app-store'
    | 'package-source-linux-package-manager'
    | 'package-source-android-google-play'
    | 'package-source-ios-apple-app-store',
  platform: GeneratedParentPlatform,
  storeSurface: Extract<
    GeneratedAppInstallPurchaseApprovalStoreSurface,
    'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store'
  >,
  platformSourceRowId:
    | 'platform-source-windows-microsoft-store'
    | 'platform-source-macos-mac-app-store'
    | 'platform-source-linux-package-manager'
    | 'platform-source-android-google-play'
    | 'platform-source-ios-apple-app-store',
  packageSourceKind: GeneratedAppInstallPurchaseApprovalPackageSourceKind,
  artifactStatus: GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus,
  approvalPathState: GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState,
  requiredArtifacts: readonly [string, string, string],
  limitationReason: string,
  timestamp: string
) {
  return {
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
    artifactRowId,
    platform,
    storeSurface,
    platformSourceRowId,
    packageSourceKind,
    artifactStatus,
    approvalPathState,
    packageSourceFieldsRequired: PackageSourceFieldsGenerated,
    packageSourceFieldsAttached: [],
    requestKindCoverage: PlatformSourceRequestKindsGenerated,
    requiredArtifacts,
    artifactEvidenceClaim: 'not-attached',
    artifactEvidencePath: null,
    artifactCapturedAt: null,
    limitationReason,
    limitationReportRef: AppInstallPurchaseApprovalReportRefsGenerated.PlatformLimitation,
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    interceptionClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    claimBoundary:
      'contract proof only; no store integration no platform adapter no real install or purchase interception no child activity data',
    lastCheckedAt: timestamp,
  } as const;
}

export function buildAppInstallPurchaseApprovalAuditEventGenerated(
  auditEventId: 'audit-request-recorded-1' | 'audit-parent-decision-recorded-1',
  eventKind: 'request-recorded' | 'parent-decision-recorded',
  timestamp: string,
  evidenceReference: unknown
) {
  return {
    auditEventId,
    eventKind,
    recordedAt: timestamp,
    evidenceReferences: [evidenceReference],
  } as const;
}

export function summarizeAppInstallPurchaseRuntimeProofGenerated(proof: {
  readonly platformRuntimeArtifacts: readonly { readonly runtimeClaimState: string; readonly storeMetadataArtifactState: string }[];
  readonly childDeliveryBoundaries: readonly unknown[];
  readonly reportIntegrationBoundaries: readonly unknown[];
  readonly statusReadinessBoundaries: readonly { readonly statusReadinessClaim: string; readonly runtimeStatusReaderClaim: string }[];
}) {
  return {
    platformRows: proof.platformRuntimeArtifacts.length,
    childDeliveryRows: proof.childDeliveryBoundaries.length,
    reportIntegrationRows: proof.reportIntegrationBoundaries.length,
    statusReadinessRows: proof.statusReadinessBoundaries.length,
    boundaryOnlyRows: proof.platformRuntimeArtifacts.filter((row) => row.runtimeClaimState === 'boundary-only').length,
    unavailablePlatformRows: proof.platformRuntimeArtifacts.filter(
      (row) => row.storeMetadataArtifactState === 'platform-unavailable'
    ).length,
    statusReadinessOnlyRows: proof.statusReadinessBoundaries.filter(
      (row) => row.statusReadinessClaim === 'runtime-status-readiness-only'
    ).length,
    statusReaderImplementedRows: proof.statusReadinessBoundaries.filter(
      (row) => row.runtimeStatusReaderClaim !== 'not-implemented'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseRuntimePlatformArtifactRowGenerated(
  sourceRow: {
    readonly platform: string;
    readonly storeSurface: string;
    readonly sourceRowId: string;
    readonly metadataState: string;
    readonly requiredArtifacts: readonly string[];
    readonly limitationReportRef: string;
  },
  packageRows: ReadonlyArray<{
    readonly platform: string;
    readonly storeSurface: string;
    readonly artifactRowId: string;
    readonly artifactStatus: string;
    readonly requiredArtifacts: readonly string[];
    readonly limitationReportRef: string;
  }>,
  runtimeSchemaVersion: string,
  runtimeBoundary: string
) {
  const packageRow = packageRows.find(
    (row) => row.platform === sourceRow.platform && row.storeSurface === sourceRow.storeSurface
  );
  if (packageRow === undefined) {
    throw new Error(`missing package-source artifact row for ${sourceRow.platform}:${sourceRow.storeSurface}`);
  }
  return {
    schemaVersion: runtimeSchemaVersion,
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    platformSourceRowId: sourceRow.sourceRowId,
    packageSourceArtifactRowId: packageRow.artifactRowId,
    storeMetadataArtifactState:
      sourceRow.metadataState === 'unavailable' ? 'platform-unavailable' : 'requires-platform-artifact',
    packageSourceArtifactState: packageSourceRuntimeStateGenerated(packageRow.artifactStatus),
    childPendingDeliveryState: sourceRow.metadataState === 'unavailable' ? 'unavailable' : 'manual-required',
    childResultDeliveryState: sourceRow.metadataState === 'unavailable' ? 'unavailable' : 'manual-required',
    reportIntegrationState: 'manual-required',
    runtimeClaimState: 'boundary-only',
    requiredProofRefs: [...sourceRow.requiredArtifacts, ...packageRow.requiredArtifacts],
    reportRefs:
      sourceRow.limitationReportRef === packageRow.limitationReportRef
        ? [sourceRow.limitationReportRef]
        : [sourceRow.limitationReportRef, packageRow.limitationReportRef],
    claimBoundary: runtimeBoundary,
  } as const;
}

export function buildAppInstallPurchaseRuntimeChildDeliveryRowGenerated(
  state: {
    readonly childStateId: string;
    readonly requestId: string;
    readonly requestKind: string;
    readonly platform: string;
    readonly childVisibleStatus: string;
    readonly sourceApprovalState: unknown;
    readonly deliveryState: string;
    readonly auditEventRefs: readonly { readonly auditEventId: string }[];
    readonly reportRefs: readonly string[];
  },
  runtimeSchemaVersion: string,
  runtimeBoundary: string
) {
  return {
    schemaVersion: runtimeSchemaVersion,
    childStateId: state.childStateId,
    requestId: state.requestId,
    requestKind: state.requestKind,
    platform: state.platform,
    childVisibleStatus: state.childVisibleStatus,
    sourceApprovalState: state.sourceApprovalState,
    deliveryState: state.deliveryState,
    runtimeDeliveryClaim: 'not-delivered',
    auditEventRefs: state.auditEventRefs.map((event) => event.auditEventId),
    reportRefs: state.reportRefs,
    claimBoundary: runtimeBoundary,
  } as const;
}

export function buildAppInstallPurchaseRuntimeReportIntegrationRowGenerated(
  row: {
    readonly surface: string;
    readonly integrationState: string;
    readonly auditEventRefs: readonly { readonly auditEventId: string }[];
    readonly reportRefs: readonly string[];
  },
  runtimeSchemaVersion: string,
  runtimeBoundary: string
) {
  return {
    schemaVersion: runtimeSchemaVersion,
    surface: row.surface,
    integrationState: row.integrationState,
    runtimeReportClaim: 'not-delivered',
    auditEventRefs: row.auditEventRefs.map((event) => event.auditEventId),
    reportRefs: row.reportRefs,
    claimBoundary: runtimeBoundary,
  } as const;
}

export function buildAppInstallPurchaseRuntimeStatusReadinessRowGenerated(
  state: {
    readonly childStateId: string;
    readonly requestId: string;
    readonly requestKind: string;
    readonly platform: string;
    readonly childVisibleStatus: string;
    readonly sourceApprovalState: unknown;
    readonly deliveryState: string;
    readonly auditEventRefs: readonly { readonly auditEventId: string }[];
    readonly reportRefs: readonly string[];
  },
  runtimeSchemaVersion: string,
  runtimeBoundary: string
) {
  return {
    schemaVersion: runtimeSchemaVersion,
    statusReadinessRowId: `app-install-status-readiness-${state.childVisibleStatus}`,
    sourceChildStateId: state.childStateId,
    sourceRequestId: state.requestId,
    requestKind: state.requestKind,
    platform: state.platform,
    childVisibleStatus: state.childVisibleStatus,
    sourceApprovalState: state.sourceApprovalState,
    sourceDeliveryState: state.deliveryState,
    sourceRuntimeDeliveryClaim: 'not-delivered',
    statusReadinessClaim: 'runtime-status-readiness-only',
    runtimeStatusReaderClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    reportRuntimeDeliveryClaim: 'not-delivered',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    appBlockingClaim: 'not-claimed',
    auditEventRefs: state.auditEventRefs.map((event) => event.auditEventId),
    reportRefs: state.reportRefs,
    claimBoundary: runtimeBoundary,
  } as const;
}

function buildApprovalAuditReportRowGenerated(
  surface: GeneratedAppInstallPurchaseApprovalAuditReportSurface,
  integrationState: GeneratedAppInstallPurchaseApprovalProofIntegrationState,
  auditEventRefs: readonly unknown[],
  reportRefs: readonly string[]
) {
  return {
    schemaVersion: AppInstallPurchaseApprovalContractRuntime.SchemaVersion,
    surface,
    integrationState,
    auditEventRefs,
    reportRefs,
    proofRequirement: 'contract status only; report UI and runtime report delivery need separate proof',
    claimBoundary: 'contract proof only; no portal runtime no platform adapter no store integration',
  } as const;
}

function platformSourceAuthorityMatchesStoreGenerated(storeSurface: string, sourceAuthority: string): boolean {
  if (storeSurface === 'google-play') {
    return sourceAuthority === 'google-play-listing';
  }
  if (storeSurface === 'apple-app-store') {
    return sourceAuthority === 'apple-app-store-listing';
  }
  if (storeSurface === 'mac-app-store') {
    return sourceAuthority === 'mac-app-store-listing';
  }
  if (storeSurface === 'microsoft-store') {
    return sourceAuthority === 'microsoft-store-listing';
  }
  return sourceAuthority === 'linux-package-manager-index';
}

function packageSourceKindMatchesStoreGenerated(storeSurface: string, packageSourceKind: string): boolean {
  if (storeSurface === 'microsoft-store') {
    return packageSourceKind === 'windows-store-package-identity';
  }
  if (storeSurface === 'mac-app-store') {
    return packageSourceKind === 'macos-bundle-receipt';
  }
  if (storeSurface === 'linux-package-manager') {
    return packageSourceKind === 'linux-package-manager-record';
  }
  if (storeSurface === 'google-play') {
    return packageSourceKind === 'android-package-source-record';
  }
  return packageSourceKind === 'ios-app-source-record';
}

function artifactStatusMatchesApprovalPathGenerated(artifactStatus: string, approvalPathState: string): boolean {
  if (approvalPathState === 'unavailable') {
    return artifactStatus === 'unavailable';
  }
  return artifactStatus === 'manual-required' || artifactStatus === 'device-proof-required';
}

function packageSourceRuntimeStateGenerated(artifactStatus: string) {
  if (artifactStatus === 'unavailable') {
    return 'platform-unavailable';
  }
  if (artifactStatus === 'device-proof-required') {
    return 'requires-device-proof-artifact';
  }
  return 'requires-package-source-artifact';
}

function arrayContainsEveryGenerated<T extends string>(values: readonly T[], requiredValues: readonly T[]): boolean {
  const valueSet = new Set(values);
  return requiredValues.every((value) => valueSet.has(value));
}

function arrayIsUniqueGenerated(values: readonly unknown[]): boolean {
  return new Set(values).size === values.length;
}
