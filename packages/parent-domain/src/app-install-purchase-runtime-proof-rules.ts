type AppInstallPurchaseRuntimeProofPlatform = 'windows' | 'macos' | 'linux' | 'android' | 'ios';
type AppInstallPurchaseRuntimeProofStoreSurface =
  | 'microsoft-store'
  | 'mac-app-store'
  | 'linux-package-manager'
  | 'google-play'
  | 'apple-app-store';
type AppInstallPurchaseRuntimeArtifactState =
  | 'requires-platform-artifact'
  | 'requires-package-source-artifact'
  | 'requires-device-proof-artifact'
  | 'platform-unavailable';
type AppInstallPurchaseRuntimeDeliveryState = 'manual-required' | 'unavailable';
type AppInstallPurchaseRuntimeReportIntegrationState = 'contract-only' | 'manual-required';
type AppInstallPurchaseRuntimeClaimState = 'boundary-only';
type AppInstallPurchaseRuntimeChildVisibleStatus =
  | 'pending-parent-review-visible'
  | 'approved-visible'
  | 'denied-visible'
  | 'time-box-visible'
  | 'review-needed-visible';
type AppInstallPurchaseRuntimeReportSurface =
  | 'request-audit-history'
  | 'parent-decision-audit-history'
  | 'child-facing-state-report'
  | 'platform-limitation-report';
type AppInstallPurchaseRuntimeNonClaim =
  | 'no-store-integration'
  | 'no-billing-entitlement-logic'
  | 'no-runtime-status-reader-implementation'
  | 'no-platform-adapter'
  | 'no-child-device-delivery'
  | 'no-runtime-report-delivery'
  | 'no-store-policy-bypass'
  | 'no-real-install-or-purchase-interception'
  | 'not-generic-app-blocking';

interface AppInstallPurchaseRuntimeClaimBoundary {
  readonly includes: (needle: string) => boolean;
}

interface AppInstallPurchaseRuntimePlatformArtifactRow {
  readonly platform: AppInstallPurchaseRuntimeProofPlatform;
  readonly storeSurface: AppInstallPurchaseRuntimeProofStoreSurface;
  readonly storeMetadataArtifactState: Extract<
    AppInstallPurchaseRuntimeArtifactState,
    'requires-platform-artifact' | 'platform-unavailable'
  >;
  readonly packageSourceArtifactState: Extract<
    AppInstallPurchaseRuntimeArtifactState,
    'requires-package-source-artifact' | 'requires-device-proof-artifact' | 'platform-unavailable'
  >;
  readonly childPendingDeliveryState: AppInstallPurchaseRuntimeDeliveryState;
  readonly childResultDeliveryState: AppInstallPurchaseRuntimeDeliveryState;
  readonly reportIntegrationState: AppInstallPurchaseRuntimeReportIntegrationState;
  readonly runtimeClaimState: AppInstallPurchaseRuntimeClaimState;
  readonly requiredProofRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: AppInstallPurchaseRuntimeClaimBoundary;
}

interface AppInstallPurchaseRuntimeChildDeliveryRow {
  readonly childVisibleStatus: AppInstallPurchaseRuntimeChildVisibleStatus;
  readonly deliveryState: AppInstallPurchaseRuntimeDeliveryState;
  readonly runtimeDeliveryClaim: 'not-delivered';
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: AppInstallPurchaseRuntimeClaimBoundary;
}

interface AppInstallPurchaseRuntimeReportIntegrationRow {
  readonly surface: AppInstallPurchaseRuntimeReportSurface;
  readonly integrationState: AppInstallPurchaseRuntimeReportIntegrationState;
  readonly runtimeReportClaim: 'not-delivered';
  readonly auditEventRefs: readonly unknown[];
  readonly reportRefs: readonly unknown[];
  readonly claimBoundary: AppInstallPurchaseRuntimeClaimBoundary;
}

interface AppInstallPurchaseRuntimeStatusReadinessRow {
  readonly childVisibleStatus: AppInstallPurchaseRuntimeChildVisibleStatus;
  readonly sourceDeliveryState: AppInstallPurchaseRuntimeDeliveryState;
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
  readonly claimBoundary: AppInstallPurchaseRuntimeClaimBoundary;
}

interface AppInstallPurchaseRuntimeProof {
  readonly sourceContractSchemaVersion: 'app-install-purchase-approval-contract-proof';
  readonly platformRuntimeArtifacts: readonly AppInstallPurchaseRuntimePlatformArtifactRow[];
  readonly childDeliveryBoundaries: readonly AppInstallPurchaseRuntimeChildDeliveryRow[];
  readonly reportIntegrationBoundaries: readonly AppInstallPurchaseRuntimeReportIntegrationRow[];
  readonly statusReadinessBoundaries: readonly AppInstallPurchaseRuntimeStatusReadinessRow[];
  readonly nonClaims: readonly AppInstallPurchaseRuntimeNonClaim[];
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

export function appInstallPurchaseRuntimeProofIsHonest(proof: AppInstallPurchaseRuntimeProof): boolean {
  return (
    proof.sourceContractSchemaVersion === 'app-install-purchase-approval-contract-proof' &&
    platformRuntimeArtifactRowsAreComplete(proof.platformRuntimeArtifacts) &&
    childDeliveryRowsAreComplete(proof.childDeliveryBoundaries) &&
    reportIntegrationRowsAreComplete(proof.reportIntegrationBoundaries) &&
    statusReadinessRowsAreComplete(proof.statusReadinessBoundaries) &&
    nonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

export function appInstallPurchaseRuntimePlatformArtifactRowIsHonest(
  row: AppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  if (!platformRuntimeRowHasRequiredRefs(row) || row.runtimeClaimState !== 'boundary-only') {
    return false;
  }
  if (row.platform === 'linux') {
    return linuxPlatformRuntimeRowIsHonest(row);
  }
  return availablePlatformRuntimeRowIsHonest(row);
}

export function appInstallPurchaseRuntimeChildDeliveryRowIsHonest(
  row: AppInstallPurchaseRuntimeChildDeliveryRow
): boolean {
  return (
    row.deliveryState === 'manual-required' &&
    row.runtimeDeliveryClaim === 'not-delivered' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.claimBoundary.includes('no child-device delivery')
  );
}

export function appInstallPurchaseRuntimeReportIntegrationRowIsHonest(
  row: AppInstallPurchaseRuntimeReportIntegrationRow
): boolean {
  return (
    row.runtimeReportClaim === 'not-delivered' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.claimBoundary.includes('no runtime report delivery')
  );
}

export function appInstallPurchaseRuntimeStatusReadinessRowIsHonest(
  row: AppInstallPurchaseRuntimeStatusReadinessRow
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
    statusReadinessBoundaryIsExplicit(row.claimBoundary)
  );
}

function platformRuntimeArtifactRowsAreComplete(
  rows: readonly AppInstallPurchaseRuntimePlatformArtifactRow[]
): boolean {
  const rowKeys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPlatformSources.length &&
    rowKeys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => rowKeys.has(`${platform}:${storeSurface}`)) &&
    rows.every((row) => appInstallPurchaseRuntimePlatformArtifactRowIsHonest(row))
  );
}

function childDeliveryRowsAreComplete(rows: readonly AppInstallPurchaseRuntimeChildDeliveryRow[]): boolean {
  const statuses = new Set(rows.map((row) => row.childVisibleStatus));
  return (
    rows.length === RequiredChildStatuses.length &&
    RequiredChildStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => appInstallPurchaseRuntimeChildDeliveryRowIsHonest(row))
  );
}

function reportIntegrationRowsAreComplete(rows: readonly AppInstallPurchaseRuntimeReportIntegrationRow[]): boolean {
  const surfaces = new Set(rows.map((row) => row.surface));
  return (
    rows.length === RequiredReportSurfaces.length &&
    RequiredReportSurfaces.every((surface) => surfaces.has(surface)) &&
    rows.every((row) => appInstallPurchaseRuntimeReportIntegrationRowIsHonest(row))
  );
}

function statusReadinessRowsAreComplete(rows: readonly AppInstallPurchaseRuntimeStatusReadinessRow[]): boolean {
  const statuses = new Set(rows.map((row) => row.childVisibleStatus));
  return (
    rows.length === RequiredChildStatuses.length &&
    RequiredChildStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => appInstallPurchaseRuntimeStatusReadinessRowIsHonest(row))
  );
}

function nonClaimsAreComplete(nonClaims: readonly AppInstallPurchaseRuntimeNonClaim[]): boolean {
  const nonClaimSet = new Set(nonClaims);
  return RequiredNonClaims.every((nonClaim) => nonClaimSet.has(nonClaim));
}

function platformRuntimeRowHasRequiredRefs(row: AppInstallPurchaseRuntimePlatformArtifactRow): boolean {
  return runtimeBoundaryIsExplicit(row.claimBoundary) && row.requiredProofRefs.length > 0 && row.reportRefs.length > 0;
}

function linuxPlatformRuntimeRowIsHonest(row: AppInstallPurchaseRuntimePlatformArtifactRow): boolean {
  return (
    row.storeMetadataArtifactState === 'platform-unavailable' &&
    row.packageSourceArtifactState === 'platform-unavailable' &&
    row.childPendingDeliveryState === 'unavailable' &&
    row.childResultDeliveryState === 'unavailable'
  );
}

function availablePlatformRuntimeRowIsHonest(row: AppInstallPurchaseRuntimePlatformArtifactRow): boolean {
  return (
    row.storeMetadataArtifactState === 'requires-platform-artifact' &&
    (row.packageSourceArtifactState === 'requires-package-source-artifact' ||
      row.packageSourceArtifactState === 'requires-device-proof-artifact') &&
    row.childPendingDeliveryState === 'manual-required' &&
    row.childResultDeliveryState === 'manual-required'
  );
}

function runtimeBoundaryIsExplicit(boundary: AppInstallPurchaseRuntimeClaimBoundary): boolean {
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

function statusReadinessBoundaryIsExplicit(boundary: AppInstallPurchaseRuntimeClaimBoundary): boolean {
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
