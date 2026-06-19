import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameMobileCapabilityMatrixSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-android-ios-capability-matrix')
);

export const BrowserGameMobilePlatformSchema = withParser(Schema.Literal('android', 'ios'));

export const BrowserGameMobileCapabilitySurfaceSchema = withParser(
  Schema.Literal(
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
    'ios-webclip-pwa'
  )
);

export const BrowserGameMobileTargetKindSchema = withParser(
  Schema.Literal(
    'browser-game-web-surface',
    'cloud-gaming-web-session',
    'owned-browser-shell',
    'installed-browser-app',
    'web-domain-token',
    'application-token',
    'device-owner-browser-policy',
    'webclip-or-pwa'
  )
);

export const BrowserGameMobileCapabilityStateSchema = withParser(
  Schema.Literal(
    'manual-device-proof-required',
    'permission-required',
    'entitlement-required',
    'domain-token-limited',
    'app-token-limited',
    'manual-required',
    'unavailable',
    'not-implemented',
    'owned-browser-shell-capable-with-proof'
  )
);

export const BrowserGameMobileProofStateSchema = withParser(
  Schema.Literal(
    'manual-device-proof-required',
    'platform-entitlement-required',
    'permission-grant-required',
    'adapter-not-implemented',
    'existing-parent-domain-proof-ref',
    'unavailable'
  )
);

export const BrowserGameMobilePolicyScopeSchema = withParser(
  Schema.Literal(
    'owned-browser-shell-only',
    'web-domain-token-level',
    'app-token-level',
    'app-level-only',
    'domain-level-only',
    'manual-review-only',
    'not-available'
  )
);

export const BrowserGameMobileCapabilityReasonSchema = withParser(
  Schema.Literal(
    'managed-browser-required',
    'owned-shell-required',
    'android-custom-tabs-not-owned',
    'unmanaged-exact-url-unavailable',
    'device-owner-required',
    'family-controls-entitlement-required',
    'opaque-application-token-required',
    'web-domain-token-limited',
    'cloud-title-unavailable',
    'runtime-signal-unavailable',
    'native-game-boundary',
    'content-frame-analysis-unavailable',
    'app-store-control-unavailable',
    'manual-device-proof-required'
  )
);

export const BrowserGameMobileCapabilityReasonsSchema = Schema.Array(BrowserGameMobileCapabilityReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game mobile capability reasons')
);

export const BrowserGameMobileProofRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game mobile proof refs')
);

export const BrowserGameMobileCapabilityBoundarySchema = brandedNonEmptyStringSchema('BrowserGameMobileCapabilityBoundary');

export type BrowserGameMobilePlatform = Infer<typeof BrowserGameMobilePlatformSchema>;
export type BrowserGameMobileCapabilitySurface = Infer<typeof BrowserGameMobileCapabilitySurfaceSchema>;
export type BrowserGameMobileCapabilityState = Infer<typeof BrowserGameMobileCapabilityStateSchema>;
export type BrowserGameMobileProofState = Infer<typeof BrowserGameMobileProofStateSchema>;

