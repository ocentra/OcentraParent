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
  return (
    value.browserNavigationBlockedClaimed ||
    value.browserRedirectClaimed ||
    value.cssDomHiddenClaimed ||
    value.tabClosedClaimed ||
    value.timeLimitAppliedClaimed ||
    value.childUiRenderedClaimed ||
    value.parentUiNotifiedClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed ||
    value.feedContentCaptured ||
    value.videoContentCaptured ||
    value.recommendationModelClaimed
  );
}
