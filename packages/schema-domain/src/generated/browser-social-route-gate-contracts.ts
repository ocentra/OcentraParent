/* generated from crates/schema/src/browser_social_route_gate_ts.rs */

export const GeneratedBrowserSocialAccountCreationGateSchemaVersion = 1 as const;
export const GeneratedBrowserSocialAccountCreationGateActionValues = [
  'allow-navigation-candidate',
  'hold-for-parent-approval',
  'block-submit-candidate',
  'manual-review-required',
  'unknown-flow-warn-only',
] as const;
export const GeneratedBrowserSocialAccountCreationGateStateValues = [
  'planned',
  'manual-required',
  'unavailable',
] as const;
export const GeneratedBrowserSocialAccountCreationGateReasonValues = [
  'signup-flow',
  'login-flow',
  'account-switch-flow',
  'form-shape-detected',
  'parent-policy-requires-approval',
  'policy-block-candidate',
  'manual-required',
  'unknown-flow',
] as const;

export const GeneratedBrowserSocialFeedRouteSchemaVersion = 1 as const;
export const GeneratedBrowserSocialFeedSurfaceHintValues = [
  'home-feed',
  'following-feed',
  'explore-feed',
  'reels-feed',
  'shorts-feed',
  'single-short-video',
] as const;
export const GeneratedBrowserSocialFeedSurfaceKindValues = [
  'dynamic-feed',
  'short-video-surface',
  'single-short-video',
  'manual-required',
] as const;

export const GeneratedBrowserSocialFeedVideoRouteGateSchemaVersion = 1 as const;
export const GeneratedBrowserSocialFeedVideoRouteGateTargetKindValues = [
  'social-feed-route',
  'social-short-video-route',
  'social-video-route',
  'manual-required',
] as const;
export const GeneratedBrowserSocialFeedVideoRouteGateActionValues = [
  'allow-route-candidate',
  'warn-route-candidate',
  'parent-review-candidate',
  'block-route-candidate',
  'limit-route-candidate',
  'manual-review-required',
  'unknown-route-warn-only',
] as const;
export const GeneratedBrowserSocialFeedVideoRouteGateStateValues = [
  'planned',
  'manual-required',
  'unavailable',
] as const;
export const GeneratedBrowserSocialFeedVideoRouteGateReasonValues = [
  'dynamic-feed-route',
  'short-video-route',
  'single-video-route',
  'metadata-available',
  'metadata-partial',
  'parent-policy-match',
  'schedule-limit-candidate',
  'policy-block-candidate',
  'parent-review-required',
  'manual-required',
  'unknown-evidence',
] as const;

export const GeneratedBrowserSocialFormShapeSchemaVersion = 1 as const;
export const GeneratedBrowserSocialFormShapeKindValues = [
  'signup-form',
  'login-form',
  'account-switch-form',
  'unknown-form',
] as const;
export const GeneratedBrowserSocialFormShapeDetectionStateValues = [
  'detected',
  'not-detected',
  'manual-required',
] as const;
export const GeneratedBrowserSocialFormControlKindValues = [
  'email-input',
  'username-input',
  'password-input',
  'display-name-input',
  'birthdate-input',
  'phone-input',
  'submit-button',
  'account-switch-link',
  'unknown-control',
] as const;

export const GeneratedBrowserSocialUnmanagedBypassSchemaVersion = 1 as const;
export const GeneratedBrowserSocialUnmanagedBypassTargetStateValues = [
  'bypass-detected',
  'manual-required',
  'unavailable',
] as const;
export const GeneratedBrowserSocialUnmanagedBypassReasonValues = [
  'possible-social-bypass-process',
  'supported-browser-outside-managed-session',
  'unsupported-browser-social-attempt',
  'portable-browser-social-attempt',
  'tor-browser-social-attempt',
  'browser-like-social-attempt',
  'managed-browser-required',
  'exact-url-unavailable',
  'manual-required',
  'allowed-unmanaged-exception',
] as const;

export const GeneratedBrowserSocialVideoMetadataSchemaVersion = 1 as const;
export const GeneratedBrowserSocialVideoMetadataSourceKindValues = [
  'platform-page-metadata',
  'open-graph',
  'schema-org-video-object',
  'manual-required',
] as const;
export const GeneratedBrowserSocialVideoMetadataStateValues = [
  'available',
  'partial',
  'manual-required',
] as const;
