#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserAndroidOwnedShellRuntimeTemplate {
    pub runtime_state: &'static str,
    pub reason_code: &'static str,
    pub exact_url_policy_claimed: bool,
    pub known_active_tab_proof_claimed: bool,
    pub physical_device_owner_claimed: bool,
    pub physical_browser_role_routing_claimed: bool,
    pub vpn_dns_browser_proof_claimed: bool,
    pub usage_stats_route_proof_claimed: bool,
    pub accessibility_route_proof_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
}

const NO_CLAIM_FLAGS: BrowserAndroidOwnedShellRuntimeTemplate =
    BrowserAndroidOwnedShellRuntimeTemplate {
        runtime_state: "manual-required",
        reason_code:
            "physical-device-owner-browser-role-exact-url-active-tab-and-enforcement-proof-required",
        exact_url_policy_claimed: false,
        known_active_tab_proof_claimed: false,
        physical_device_owner_claimed: false,
        physical_browser_role_routing_claimed: false,
        vpn_dns_browser_proof_claimed: false,
        usage_stats_route_proof_claimed: false,
        accessibility_route_proof_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
    };

pub fn browser_android_owned_shell_runtime_physical_template(
    package_installed: bool,
    explicit_launch_observed: bool,
    screenshot_captured: bool,
    ui_tree_captured: bool,
) -> BrowserAndroidOwnedShellRuntimeTemplate {
    BrowserAndroidOwnedShellRuntimeTemplate {
        runtime_state: if package_installed
            && explicit_launch_observed
            && screenshot_captured
            && ui_tree_captured
        {
            "physical-visible-owned-shell"
        } else {
            "manual-required"
        },
        reason_code: "physical-android-owned-shell-visible-runtime-proof",
        ..NO_CLAIM_FLAGS
    }
}

pub fn browser_android_owned_shell_runtime_emulator_template(
    proof_launched_emulator: bool,
    implicit_view_intent_launch_observed: bool,
) -> Option<BrowserAndroidOwnedShellRuntimeTemplate> {
    if !proof_launched_emulator || !implicit_view_intent_launch_observed {
        return None;
    }

    Some(BrowserAndroidOwnedShellRuntimeTemplate {
        runtime_state: "emulator-browser-role-routing",
        reason_code: "emulator-browser-role-routing-proof-not-physical-default-browser",
        ..NO_CLAIM_FLAGS
    })
}

pub fn browser_android_owned_shell_runtime_manual_template(
) -> BrowserAndroidOwnedShellRuntimeTemplate {
    NO_CLAIM_FLAGS
}

pub fn browser_android_owned_shell_runtime_typescript() -> String {
    [
        "/* generated from crates/browser-core/src/browser_android_owned_shell_runtime.rs */",
        "",
        "export type BrowserAndroidOwnedShellRuntimeTemplate = {",
        "  readonly runtimeState:",
        "    | 'physical-visible-owned-shell'",
        "    | 'emulator-browser-role-routing'",
        "    | 'manual-required';",
        "  readonly reasonCode: string;",
        "  readonly exactUrlPolicyClaimed: false;",
        "  readonly knownActiveTabProofClaimed: false;",
        "  readonly physicalDeviceOwnerClaimed: false;",
        "  readonly physicalBrowserRoleRoutingClaimed: false;",
        "  readonly vpnDnsBrowserProofClaimed: false;",
        "  readonly usageStatsRouteProofClaimed: false;",
        "  readonly accessibilityRouteProofClaimed: false;",
        "  readonly finalPolicyExecutionClaimed: false;",
        "  readonly enforcementClaimed: false;",
        "};",
        "",
        "const BrowserAndroidOwnedShellRuntimeNoClaimFlags = {",
        "  exactUrlPolicyClaimed: false,",
        "  knownActiveTabProofClaimed: false,",
        "  physicalDeviceOwnerClaimed: false,",
        "  physicalBrowserRoleRoutingClaimed: false,",
        "  vpnDnsBrowserProofClaimed: false,",
        "  usageStatsRouteProofClaimed: false,",
        "  accessibilityRouteProofClaimed: false,",
        "  finalPolicyExecutionClaimed: false,",
        "  enforcementClaimed: false,",
        "} as const;",
        "",
        "export function browserAndroidOwnedShellRuntimePhysicalTemplate(input: {",
        "  readonly packageInstalled: boolean;",
        "  readonly explicitLaunchObserved: boolean;",
        "  readonly screenshotCaptured: boolean;",
        "  readonly uiTreeCaptured: boolean;",
        "}): BrowserAndroidOwnedShellRuntimeTemplate {",
        "  return {",
        "    runtimeState:",
        "      input.packageInstalled &&",
        "      input.explicitLaunchObserved &&",
        "      input.screenshotCaptured &&",
        "      input.uiTreeCaptured",
        "        ? 'physical-visible-owned-shell'",
        "        : 'manual-required',",
        "    reasonCode: 'physical-android-owned-shell-visible-runtime-proof',",
        "    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,",
        "  };",
        "}",
        "",
        "export function browserAndroidOwnedShellRuntimeEmulatorTemplate(input: {",
        "  readonly proofLaunchedEmulator: boolean;",
        "  readonly implicitViewIntentLaunchObserved: boolean;",
        "}): BrowserAndroidOwnedShellRuntimeTemplate | null {",
        "  if (!input.proofLaunchedEmulator || !input.implicitViewIntentLaunchObserved) {",
        "    return null;",
        "  }",
        "",
        "  return {",
        "    runtimeState: 'emulator-browser-role-routing',",
        "    reasonCode: 'emulator-browser-role-routing-proof-not-physical-default-browser',",
        "    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,",
        "  };",
        "}",
        "",
        "export function browserAndroidOwnedShellRuntimeManualTemplate(): BrowserAndroidOwnedShellRuntimeTemplate {",
        "  return {",
        "    runtimeState: 'manual-required',",
        "    reasonCode:",
        "      'physical-device-owner-browser-role-exact-url-active-tab-and-enforcement-proof-required',",
        "    ...BrowserAndroidOwnedShellRuntimeNoClaimFlags,",
        "  };",
        "}",
        "",
    ]
    .join("\n")
}
