import {
  AppInstallPurchaseApprovalContractProofSchema,
  type AppInstallPurchaseApprovalPlatformSupportRow,
} from './app-install-purchase-approval';
import { AppInstallPurchaseApprovalPackageSourceArtifactRowSchema } from './app-install-purchase-approval-package-sources';
import { AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema } from './app-install-purchase-approval-platform-sources';
import {
  appInstallPurchaseApprovalAuditReportIntegration,
  appInstallPurchaseApprovalChildFacingStates,
  AppInstallPurchaseApprovalReportRefs,
} from './app-install-purchase-approval-proof-states';

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
const PlatformSourceMetadataFields = [
  'store-listing-id',
  'app-title',
  'publisher-name',
  'category',
  'age-rating',
  'price-display',
  'subscription-period',
  'source-url',
] as const;
const PlatformSourceRequestKinds = ['install', 'purchase', 'subscription'] as const;
const PackageSourceFields = [
  'package-identifier',
  'installer-source',
  'publisher-or-developer',
  'version-or-build',
  'signature-or-receipt',
  'source-captured-at',
] as const;

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
  platformSourceMetadata: [
    platformSourceMetadataRow(
      'platform-source-windows-microsoft-store',
      'windows',
      'microsoft-store',
      'microsoft-store-listing',
      'manual-required',
      'requires-store-artifact-proof',
      [
        'Microsoft Store family purchase or app-request API proof',
        'Windows child-device package source artifact with parent-visible request id',
        'limitation report artifact before product support claim',
      ],
      'Microsoft Store metadata and install or purchase interception require approved Microsoft or Windows store artifacts.'
    ),
    platformSourceMetadataRow(
      'platform-source-macos-mac-app-store',
      'macos',
      'mac-app-store',
      'mac-app-store-listing',
      'manual-required',
      'requires-store-artifact-proof',
      [
        'Mac App Store listing source artifact',
        'macOS child-device package or receipt artifact with parent-visible request id',
        'limitation report artifact before product support claim',
      ],
      'Mac App Store metadata and purchase or install decisions require macOS-specific store and package artifacts.'
    ),
    platformSourceMetadataRow(
      'platform-source-linux-package-manager',
      'linux',
      'linux-package-manager',
      'linux-package-manager-index',
      'unavailable',
      'platform-unavailable',
      [
        'target distro package-manager metadata policy',
        'Linux child-device package source artifact with parent-visible request id',
        'limitation report artifact before product support claim',
      ],
      'Linux package managers do not provide one common app-store approval path in this proof.'
    ),
    platformSourceMetadataRow(
      'platform-source-android-google-play',
      'android',
      'google-play',
      'google-play-listing',
      'manual-required',
      'requires-approved-api-proof',
      [
        'Google Play approved API or managed-family policy proof',
        'Android child-device package source artifact with parent-visible request id',
        'limitation report artifact before product support claim',
      ],
      'Google Play install and purchase metadata require approved Play or device-management source artifacts.'
    ),
    platformSourceMetadataRow(
      'platform-source-ios-apple-app-store',
      'ios',
      'apple-app-store',
      'apple-app-store-listing',
      'manual-required',
      'requires-approved-api-proof',
      [
        'Apple App Store or Family Controls entitlement proof',
        'iOS child-device app source artifact with parent-visible request id',
        'limitation report artifact before product support claim',
      ],
      'Apple App Store install and purchase metadata require approved Apple API entitlement and review proof.'
    ),
  ],
  packageSourceArtifacts: [
    packageSourceArtifactRow(
      'package-source-windows-microsoft-store',
      'windows',
      'microsoft-store',
      'platform-source-windows-microsoft-store',
      'windows-store-package-identity',
      'manual-required',
      'manual-required',
      [
        'Windows package family or app identity artifact',
        'installer/source channel artifact linked to the child request',
        'parent-visible limitation report before product support claim',
      ],
      'Windows package-source identity can support manual review only after host artifacts are attached.'
    ),
    packageSourceArtifactRow(
      'package-source-macos-mac-app-store',
      'macos',
      'mac-app-store',
      'platform-source-macos-mac-app-store',
      'macos-bundle-receipt',
      'manual-required',
      'manual-required',
      [
        'macOS bundle identifier or receipt artifact',
        'installer/source channel artifact linked to the child request',
        'parent-visible limitation report before product support claim',
      ],
      'macOS package-source identity can support manual review only after bundle or receipt artifacts are attached.'
    ),
    packageSourceArtifactRow(
      'package-source-linux-package-manager',
      'linux',
      'linux-package-manager',
      'platform-source-linux-package-manager',
      'linux-package-manager-record',
      'unavailable',
      'unavailable',
      [
        'target distro package-manager record',
        'installer/source channel artifact linked to the child request',
        'parent-visible limitation report before product support claim',
      ],
      'Linux package-source records are distro-specific and do not prove a common install or purchase approval path.'
    ),
    packageSourceArtifactRow(
      'package-source-android-google-play',
      'android',
      'google-play',
      'platform-source-android-google-play',
      'android-package-source-record',
      'device-proof-required',
      'manual-required',
      [
        'Android package name and installer source artifact',
        'signing certificate or Play source artifact linked to the child request',
        'real device or managed-profile proof before product support claim',
      ],
      'Android package-source identity requires device or managed-profile proof before approval support can be claimed.'
    ),
    packageSourceArtifactRow(
      'package-source-ios-apple-app-store',
      'ios',
      'apple-app-store',
      'platform-source-ios-apple-app-store',
      'ios-app-source-record',
      'device-proof-required',
      'manual-required',
      [
        'iOS bundle identifier and App Store source artifact',
        'Family Controls or Screen Time entitlement artifact linked to the child request',
        'real device or TestFlight proof before product support claim',
      ],
      'iOS app-source identity requires Apple entitlement and device proof before approval support can be claimed.'
    ),
  ],
  childFacingStates: appInstallPurchaseApprovalChildFacingStates({
    requestAuditEvent: RequestAuditEvent,
    decisionAuditEvent: DecisionAuditEvent,
  }),
  auditReportIntegration: appInstallPurchaseApprovalAuditReportIntegration({
    requestAuditEvent: RequestAuditEvent,
    decisionAuditEvent: DecisionAuditEvent,
  }),
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
  'Platform-source metadata rows are limitation proof only; no approved store API, entitlement, or package-source artifact is attached yet.',
  'Package-source artifact rows name required package identity/source artifacts but attach no real child-device artifacts.',
  'No billing entitlement state is used as child-safety approval authority.',
  'No portal approval UI exists in this proof.',
  'Child-facing pending/result states are contract rows only; no child-device delivery adapter is implemented.',
  'Audit/report integration is status proof only; no report UI or runtime report delivery is implemented.',
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

function platformSourceMetadataRow(
  sourceRowId:
    | 'platform-source-windows-microsoft-store'
    | 'platform-source-macos-mac-app-store'
    | 'platform-source-linux-package-manager'
    | 'platform-source-android-google-play'
    | 'platform-source-ios-apple-app-store',
  platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios',
  storeSurface: 'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store',
  sourceAuthority:
    | 'microsoft-store-listing'
    | 'mac-app-store-listing'
    | 'linux-package-manager-index'
    | 'google-play-listing'
    | 'apple-app-store-listing',
  metadataState: 'manual-required' | 'unavailable',
  sourceEvidenceState: 'requires-approved-api-proof' | 'requires-store-artifact-proof' | 'platform-unavailable',
  requiredArtifacts: readonly [string, string, string],
  limitationReason: string
) {
  return AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema.parse({
    schemaVersion: 'app-install-purchase-approval-contract-proof',
    sourceRowId,
    platform,
    storeSurface,
    sourceAuthority,
    metadataState,
    sourceEvidenceState,
    fieldsAvailableFromContract: [],
    fieldsRequiringPlatformProof: PlatformSourceMetadataFields,
    requestKindCoverage: PlatformSourceRequestKinds,
    requiredArtifacts,
    limitationReason,
    limitationReportRef: AppInstallPurchaseApprovalReportRefs.PlatformLimitation,
    parentManualFallback: 'contract-only-parent-review',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    interceptionClaim: 'not-claimed',
    claimBoundary:
      'contract proof only; no store integration no platform adapter no real install or purchase interception',
    lastCheckedAt: Timestamp,
  });
}

function packageSourceArtifactRow(
  artifactRowId:
    | 'package-source-windows-microsoft-store'
    | 'package-source-macos-mac-app-store'
    | 'package-source-linux-package-manager'
    | 'package-source-android-google-play'
    | 'package-source-ios-apple-app-store',
  platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios',
  storeSurface: 'microsoft-store' | 'mac-app-store' | 'linux-package-manager' | 'google-play' | 'apple-app-store',
  platformSourceRowId:
    | 'platform-source-windows-microsoft-store'
    | 'platform-source-macos-mac-app-store'
    | 'platform-source-linux-package-manager'
    | 'platform-source-android-google-play'
    | 'platform-source-ios-apple-app-store',
  packageSourceKind:
    | 'windows-store-package-identity'
    | 'macos-bundle-receipt'
    | 'linux-package-manager-record'
    | 'android-package-source-record'
    | 'ios-app-source-record',
  artifactStatus: 'manual-required' | 'device-proof-required' | 'unavailable',
  approvalPathState: 'manual-required' | 'unavailable',
  requiredArtifacts: readonly [string, string, string],
  limitationReason: string
) {
  return AppInstallPurchaseApprovalPackageSourceArtifactRowSchema.parse({
    schemaVersion: 'app-install-purchase-approval-contract-proof',
    artifactRowId,
    platform,
    storeSurface,
    platformSourceRowId,
    packageSourceKind,
    artifactStatus,
    approvalPathState,
    packageSourceFieldsRequired: PackageSourceFields,
    packageSourceFieldsAttached: [],
    requestKindCoverage: PlatformSourceRequestKinds,
    requiredArtifacts,
    artifactEvidenceClaim: 'not-attached',
    artifactEvidencePath: null,
    artifactCapturedAt: null,
    limitationReason,
    limitationReportRef: AppInstallPurchaseApprovalReportRefs.PlatformLimitation,
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    interceptionClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    claimBoundary:
      'contract proof only; no store integration no platform adapter no real install or purchase interception no child activity data',
    lastCheckedAt: Timestamp,
  });
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
