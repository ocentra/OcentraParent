/* generated from crates/schema/src/app_install_purchase_approval.rs */

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
