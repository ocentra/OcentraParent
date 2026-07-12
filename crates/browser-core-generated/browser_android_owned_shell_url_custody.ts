/* generated from crates/browser-core/src/browser_android_owned_shell_url_custody.rs */

export type BrowserAndroidOwnedShellUrlCustodyTemplate = {
  readonly custodyState: 'physical-owned-shell-request-url-ref' | 'manual-required';
  readonly reasonCode: string;
  readonly rawUrlPersisted: false;
  readonly exactUrlPolicyClaimed: false;
  readonly knownActiveTabProofClaimed: false;
  readonly physicalDeviceOwnerClaimed: false;
  readonly physicalBrowserRoleRoutingClaimed: false;
  readonly vpnDnsBrowserProofClaimed: false;
  readonly usageStatsRouteProofClaimed: false;
  readonly accessibilityRouteProofClaimed: false;
  readonly finalPolicyExecutionClaimed: false;
  readonly enforcementClaimed: false;
};

const BrowserAndroidOwnedShellUrlCustodyNoClaimFlags = {
  rawUrlPersisted: false,
  exactUrlPolicyClaimed: false,
  knownActiveTabProofClaimed: false,
  physicalDeviceOwnerClaimed: false,
  physicalBrowserRoleRoutingClaimed: false,
  vpnDnsBrowserProofClaimed: false,
  usageStatsRouteProofClaimed: false,
  accessibilityRouteProofClaimed: false,
  finalPolicyExecutionClaimed: false,
  enforcementClaimed: false,
} as const;

export function browserAndroidOwnedShellUrlCustodyPhysicalEligible(input: {
  readonly physicalDeviceObserved: boolean;
  readonly packageInstalled: boolean;
  readonly explicitLaunchObserved: boolean;
  readonly screenshotCaptured: boolean;
  readonly uiTreeCaptured: boolean;
  readonly browsableViewIntentDeclared: boolean;
  readonly webViewDeclared: boolean;
  readonly localProofPageObserved: boolean;
}): boolean {
  return (
    input.physicalDeviceObserved &&
    input.packageInstalled &&
    input.explicitLaunchObserved &&
    input.screenshotCaptured &&
    input.uiTreeCaptured &&
    input.browsableViewIntentDeclared &&
    input.webViewDeclared &&
    input.localProofPageObserved
  );
}

export function browserAndroidOwnedShellUrlCustodyPhysicalTemplate(): BrowserAndroidOwnedShellUrlCustodyTemplate {
  return {
    custodyState: 'physical-owned-shell-request-url-ref',
    reasonCode: 'physical-owned-shell-view-intent-url-ref-custody',
    ...BrowserAndroidOwnedShellUrlCustodyNoClaimFlags,
  };
}

export function browserAndroidOwnedShellUrlCustodyManualTemplate(): BrowserAndroidOwnedShellUrlCustodyTemplate {
  return {
    custodyState: 'manual-required',
    reasonCode: 'android-active-tab-policy-execution-and-enforcement-proof-required',
    ...BrowserAndroidOwnedShellUrlCustodyNoClaimFlags,
  };
}
