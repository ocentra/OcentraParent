/* generated from crates/browser-core/src/browser_android_owned_shell_runtime.rs */

export type BrowserAndroidOwnedShellRuntimeTemplate = {
  readonly runtimeState: 'physical-visible-owned-shell' | 'emulator-browser-role-routing' | 'manual-required';
  readonly reasonCode: string;
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

const BrowserAndroidOwnedShellRuntimeNoClaimFlags = {
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

export function browserAndroidOwnedShellRuntimePhysicalTemplate(input: {
  readonly packageInstalled: boolean;
  readonly explicitLaunchObserved: boolean;
  readonly screenshotCaptured: boolean;
  readonly uiTreeCaptured: boolean;
}): BrowserAndroidOwnedShellRuntimeTemplate {
  return {
    runtimeState:
      input.packageInstalled && input.explicitLaunchObserved && input.screenshotCaptured && input.uiTreeCaptured
        ? 'physical-visible-owned-shell'
        : 'manual-required',
    reasonCode: 'physical-android-owned-shell-visible-runtime-proof',
    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,
  };
}

export function browserAndroidOwnedShellRuntimeEmulatorTemplate(input: {
  readonly proofLaunchedEmulator: boolean;
  readonly implicitViewIntentLaunchObserved: boolean;
}): BrowserAndroidOwnedShellRuntimeTemplate | null {
  if (!input.proofLaunchedEmulator || !input.implicitViewIntentLaunchObserved) {
    return null;
  }

  return {
    runtimeState: 'emulator-browser-role-routing',
    reasonCode: 'emulator-browser-role-routing-proof-not-physical-default-browser',
    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,
  };
}

export function browserAndroidOwnedShellRuntimeManualTemplate(): BrowserAndroidOwnedShellRuntimeTemplate {
  return {
    runtimeState: 'manual-required',
    reasonCode: 'physical-device-owner-browser-role-exact-url-active-tab-and-enforcement-proof-required',
    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,
  };
}
