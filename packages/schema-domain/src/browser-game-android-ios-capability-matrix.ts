import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from '@ocentra-parent/schema-domain/capabilities';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BrowserGameMobileCapabilityBoundarySchema,
  BrowserGameMobileCapabilityMatrixSchemaVersionSchema,
  BrowserGameMobileCapabilityReasonsSchema,
  type BrowserGameMobileCapabilitySurface,
  BrowserGameMobileCapabilitySurfaceSchema,
  BrowserGameMobileCapabilityStateSchema,
  BrowserGameMobilePlatformSchema,
  BrowserGameMobilePolicyScopeSchema,
  BrowserGameMobileProofRefsSchema,
  BrowserGameMobileProofStateSchema,
  BrowserGameMobileTargetKindSchema,
} from './browser-game-android-ios-capability-matrix-values';

const BrowserGameMobileCapabilityRowBaseSchema = Schema.Struct({
  platform: BrowserGameMobilePlatformSchema,
  surface: BrowserGameMobileCapabilitySurfaceSchema,
  targetKind: BrowserGameMobileTargetKindSchema,
  parentCapability: ParentControlCapabilityNameSchema,
  parentCapabilityStatus: ParentControlCapabilityStatusSchema,
  capabilityState: BrowserGameMobileCapabilityStateSchema,
  proofState: BrowserGameMobileProofStateSchema,
  policyScope: BrowserGameMobilePolicyScopeSchema,
  proofRefs: BrowserGameMobileProofRefsSchema,
  reasons: BrowserGameMobileCapabilityReasonsSchema,
  exactGameContentClaimed: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  gameChatContentClaimed: Schema.Boolean,
  perGameCloudTitleClaimed: Schema.Boolean,
  runtimeSignalClaimed: Schema.Boolean,
  appStoreOrPurchaseControlClaimed: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameMobileCapabilityRowCandidate = Infer<typeof BrowserGameMobileCapabilityRowBaseSchema>;

export const BrowserGameMobileCapabilityRowSchema = withParser(
  BrowserGameMobileCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        browserGameMobileCapabilityRowIsHonest(row) ||
        'Expected mobile browser-game capability row to stay manual-required, token-limited, entitlement-required, or unavailable without content, runtime, UI, native-game, cloud-title, or enforcement claims'
    )
  )
);

export const BrowserGameMobileCapabilityClaimBoundariesSchema = withParser(
  Schema.Struct({
    exactGameContent: Schema.Literal('not-claimed'),
    cloudStreamFrameAnalysis: Schema.Literal('not-claimed'),
    nativeGameControl: Schema.Literal('not-claimed'),
    nativeLauncherControl: Schema.Literal('not-claimed'),
    gameChatContent: Schema.Literal('not-claimed'),
    perGameCloudTitle: Schema.Literal('not-claimed'),
    runtimeSignals: Schema.Literal('not-claimed'),
    appStoreOrPurchaseControl: Schema.Literal('not-claimed'),
    uiDelivery: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
    reviewerSummary: BrowserGameMobileCapabilityBoundarySchema,
  })
);

const BrowserGameMobileCapabilityMatrixBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameMobileCapabilityMatrixSchemaVersionSchema,
  generatedAt: ParentTimestampSchema,
  proofRefs: BrowserGameMobileProofRefsSchema,
  rows: Schema.Array(BrowserGameMobileCapabilityRowSchema),
  claimBoundaries: BrowserGameMobileCapabilityClaimBoundariesSchema,
});

type BrowserGameMobileCapabilityMatrixCandidate = Infer<typeof BrowserGameMobileCapabilityMatrixBaseSchema>;

export const BrowserGameMobileCapabilityMatrixSchema = withParser(
  BrowserGameMobileCapabilityMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        browserGameMobileCapabilityMatrixIsHonest(matrix) ||
        'Expected mobile browser-game capability matrix to include all Android/iOS surfaces exactly once without runtime or enforcement claims'
    )
  )
);

export const decodeBrowserGameMobileCapabilityMatrix = (input: unknown) =>
  BrowserGameMobileCapabilityMatrixSchema.parse(input);

export type BrowserGameMobileCapabilityRow = Infer<typeof BrowserGameMobileCapabilityRowSchema>;
export type BrowserGameMobileCapabilityMatrix = Infer<typeof BrowserGameMobileCapabilityMatrixSchema>;

const RequiredBrowserGameMobileSurfaces = [
  'android-owned-browser-shell',
  'android-managed-webview-shell',
  'android-custom-tabs',
  'android-installed-browser-app',
  'android-cloud-gaming-browser-session',
  'android-device-owner-browser-policy',
  'ios-family-controls-authorization',
  'ios-safari-web-domain-token',
  'ios-application-token',
  'ios-managed-browser-shell',
  'ios-cloud-gaming-web-session',
  'ios-webclip-pwa',
] as const satisfies ReadonlyArray<BrowserGameMobileCapabilitySurface>;

type BrowserGameMobileCapabilityRowValidator = (row: BrowserGameMobileCapabilityRowCandidate) => boolean;
type BrowserGameMobileCapabilityReason = BrowserGameMobileCapabilityRowCandidate['reasons'][number];

const BrowserGameMobileCapabilityRowValidators = {
  'android-owned-browser-shell': ownedBrowserShellRowIsHonest,
  'android-managed-webview-shell': ownedBrowserShellRowIsHonest,
  'android-custom-tabs': androidCustomTabsRowIsHonest,
  'android-installed-browser-app': androidInstalledBrowserAppRowIsHonest,
  'android-cloud-gaming-browser-session': browserGameMobileCloudSessionIsHonest,
  'android-device-owner-browser-policy': androidDeviceOwnerBrowserPolicyRowIsHonest,
  'ios-family-controls-authorization': iosFamilyControlsAuthorizationRowIsHonest,
  'ios-safari-web-domain-token': iosSafariWebDomainTokenRowIsHonest,
  'ios-application-token': iosApplicationTokenRowIsHonest,
  'ios-managed-browser-shell': iosManagedBrowserShellRowIsHonest,
  'ios-cloud-gaming-web-session': browserGameMobileCloudSessionIsHonest,
  'ios-webclip-pwa': iosApplicationTokenRowIsHonest,
} satisfies Record<BrowserGameMobileCapabilitySurface, BrowserGameMobileCapabilityRowValidator>;

function browserGameMobileCapabilityMatrixIsHonest(matrix: BrowserGameMobileCapabilityMatrixCandidate): boolean {
  const surfaces = new Set(matrix.rows.map((row) => row.surface));
  return (
    surfaces.size === matrix.rows.length && RequiredBrowserGameMobileSurfaces.every((surface) => surfaces.has(surface))
  );
}

function browserGameMobileCapabilityRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  if (browserGameMobileCapabilityRowClaimsRuntime(row)) {
    return false;
  }
  if (!browserGameMobileCapabilityRowHasExpectedPlatform(row)) {
    return false;
  }
  if (browserGameMobileCapabilityRowClaimsUnsupportedUpgrade(row)) {
    return false;
  }
  return BrowserGameMobileCapabilityRowValidators[row.surface as BrowserGameMobileCapabilitySurface](row);
}

function ownedBrowserShellRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'managed-browser-control' &&
    row.proofState === 'manual-device-proof-required' &&
    row.policyScope === 'owned-browser-shell-only' &&
    rowHasReasons(row, ['owned-shell-required', 'managed-browser-required'])
  );
}

function androidCustomTabsRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'managed-browser-control' &&
    row.policyScope === 'manual-review-only' &&
    rowHasReasons(row, ['android-custom-tabs-not-owned', 'unmanaged-exact-url-unavailable'])
  );
}

function androidInstalledBrowserAppRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'package-lifecycle' &&
    row.policyScope === 'app-level-only' &&
    rowHasReasons(row, ['unmanaged-exact-url-unavailable'])
  );
}

function androidDeviceOwnerBrowserPolicyRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'device-owner-policy' &&
    row.policyScope === 'manual-review-only' &&
    rowHasReasons(row, ['device-owner-required'])
  );
}

function iosFamilyControlsAuthorizationRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'family-controls-entitlement' &&
    row.capabilityState === 'entitlement-required' &&
    row.proofState === 'platform-entitlement-required' &&
    row.policyScope === 'manual-review-only' &&
    rowHasReasons(row, ['family-controls-entitlement-required'])
  );
}

function iosSafariWebDomainTokenRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'screen-time-api' &&
    row.capabilityState === 'domain-token-limited' &&
    row.policyScope === 'web-domain-token-level' &&
    rowHasReasons(row, ['web-domain-token-limited'])
  );
}

function iosApplicationTokenRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'family-controls-entitlement' &&
    row.policyScope === 'app-token-level' &&
    rowHasReasons(row, ['opaque-application-token-required', 'native-game-boundary'])
  );
}

function iosManagedBrowserShellRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'managed-browser-control' &&
    row.policyScope === 'owned-browser-shell-only' &&
    rowHasReasons(row, ['owned-shell-required', 'managed-browser-required'])
  );
}

function rowHasReasons(
  row: BrowserGameMobileCapabilityRowCandidate,
  reasons: ReadonlyArray<BrowserGameMobileCapabilityReason>
): boolean {
  return reasons.every((reason) => row.reasons.includes(reason));
}

function browserGameMobileCapabilityRowHasExpectedPlatform(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  if (row.surface.startsWith('android-')) {
    return row.platform === 'android';
  }
  if (row.surface.startsWith('ios-')) {
    return row.platform === 'ios';
  }
  return false;
}

function browserGameMobileCapabilityRowClaimsUnsupportedUpgrade(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return row.capabilityState === 'owned-browser-shell-capable-with-proof';
}

function browserGameMobileCloudSessionIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.targetKind === 'cloud-gaming-web-session' &&
    row.capabilityState === 'manual-required' &&
    row.policyScope === 'manual-review-only' &&
    row.reasons.includes('cloud-title-unavailable') &&
    row.reasons.includes('content-frame-analysis-unavailable')
  );
}

function browserGameMobileCapabilityRowClaimsRuntime(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return BrowserGameMobileCapabilityRuntimeClaimFields.some((field) => row[field] === true);
}

const BrowserGameMobileCapabilityRuntimeClaimFields = [
  'exactGameContentClaimed',
  'cloudStreamFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'nativeLauncherControlClaimed',
  'gameChatContentClaimed',
  'perGameCloudTitleClaimed',
  'runtimeSignalClaimed',
  'appStoreOrPurchaseControlClaimed',
  'uiDeliveredClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameMobileCapabilityRowCandidate>;
