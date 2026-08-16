/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

export type BrowserSocialFeedVideoRouteGateRuntimeClaims = {
  readonly browserNavigationBlockedClaimed: boolean;
  readonly browserRedirectClaimed: boolean;
  readonly cssDomHiddenClaimed: boolean;
  readonly tabClosedClaimed: boolean;
  readonly timeLimitAppliedClaimed: boolean;
  readonly childUiRenderedClaimed: boolean;
  readonly parentUiNotifiedClaimed: boolean;
  readonly policyDecisionClaimed: boolean;
  readonly enforcementClaimed: boolean;
  readonly nativeAppControlClaimed: boolean;
  readonly platformConnectorClaimed: boolean;
  readonly feedContentCaptured: boolean;
  readonly videoContentCaptured: boolean;
  readonly recommendationModelClaimed: boolean;
};

export function browserSocialFeedVideoRouteGateClaimsRuntime(value: BrowserSocialFeedVideoRouteGateRuntimeClaims) {
  return BrowserSocialFeedVideoRouteGateRuntimeClaimFields.some((field) => value[field]);
}

const BrowserSocialFeedVideoRouteGateRuntimeClaimFields = [
  'browserNavigationBlockedClaimed',
  'browserRedirectClaimed',
  'cssDomHiddenClaimed',
  'tabClosedClaimed',
  'timeLimitAppliedClaimed',
  'childUiRenderedClaimed',
  'parentUiNotifiedClaimed',
  'policyDecisionClaimed',
  'enforcementClaimed',
  'nativeAppControlClaimed',
  'platformConnectorClaimed',
  'feedContentCaptured',
  'videoContentCaptured',
  'recommendationModelClaimed',
] as const satisfies ReadonlyArray<keyof BrowserSocialFeedVideoRouteGateRuntimeClaims>;
