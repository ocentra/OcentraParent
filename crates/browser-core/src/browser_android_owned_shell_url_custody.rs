#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserAndroidOwnedShellUrlCustodyTemplate {
    pub custody_state: &'static str,
    pub reason_code: &'static str,
    pub raw_url_persisted: bool,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserAndroidOwnedShellUrlCustodyPhysicalEligibilityInput {
    pub physical_device_observed: bool,
    pub package_installed: bool,
    pub explicit_launch_observed: bool,
    pub screenshot_captured: bool,
    pub ui_tree_captured: bool,
    pub browsable_view_intent_declared: bool,
    pub web_view_declared: bool,
    pub local_proof_page_observed: bool,
}

const URL_CUSTODY_NO_CLAIM_FLAGS: BrowserAndroidOwnedShellUrlCustodyTemplate =
    BrowserAndroidOwnedShellUrlCustodyTemplate {
        custody_state: "manual-required",
        reason_code: "android-active-tab-policy-execution-and-enforcement-proof-required",
        raw_url_persisted: false,
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

pub fn browser_android_owned_shell_url_custody_physical_eligible(
    input: &BrowserAndroidOwnedShellUrlCustodyPhysicalEligibilityInput,
) -> bool {
    input.physical_device_observed
        && input.package_installed
        && input.explicit_launch_observed
        && input.screenshot_captured
        && input.ui_tree_captured
        && input.browsable_view_intent_declared
        && input.web_view_declared
        && input.local_proof_page_observed
}

pub fn browser_android_owned_shell_url_custody_physical_template(
) -> BrowserAndroidOwnedShellUrlCustodyTemplate {
    BrowserAndroidOwnedShellUrlCustodyTemplate {
        custody_state: "physical-owned-shell-request-url-ref",
        reason_code: "physical-owned-shell-view-intent-url-ref-custody",
        ..URL_CUSTODY_NO_CLAIM_FLAGS
    }
}

pub fn browser_android_owned_shell_url_custody_manual_template(
) -> BrowserAndroidOwnedShellUrlCustodyTemplate {
    URL_CUSTODY_NO_CLAIM_FLAGS
}

pub fn browser_android_owned_shell_url_custody_typescript() -> String {
    [
        "/* generated from crates/browser-core/src/browser_android_owned_shell_url_custody.rs */",
        "",
        "export type BrowserAndroidOwnedShellUrlCustodyTemplate = {",
        "  readonly custodyState:",
        "    | 'physical-owned-shell-request-url-ref'",
        "    | 'manual-required';",
        "  readonly reasonCode: string;",
        "  readonly rawUrlPersisted: false;",
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
        "const BrowserAndroidOwnedShellUrlCustodyNoClaimFlags = {",
        "  rawUrlPersisted: false,",
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
        "export function browserAndroidOwnedShellUrlCustodyPhysicalEligible(input: {",
        "  readonly physicalDeviceObserved: boolean;",
        "  readonly packageInstalled: boolean;",
        "  readonly explicitLaunchObserved: boolean;",
        "  readonly screenshotCaptured: boolean;",
        "  readonly uiTreeCaptured: boolean;",
        "  readonly browsableViewIntentDeclared: boolean;",
        "  readonly webViewDeclared: boolean;",
        "  readonly localProofPageObserved: boolean;",
        "}): boolean {",
        "  return (",
        "    input.physicalDeviceObserved &&",
        "    input.packageInstalled &&",
        "    input.explicitLaunchObserved &&",
        "    input.screenshotCaptured &&",
        "    input.uiTreeCaptured &&",
        "    input.browsableViewIntentDeclared &&",
        "    input.webViewDeclared &&",
        "    input.localProofPageObserved",
        "  );",
        "}",
        "",
        "export function browserAndroidOwnedShellUrlCustodyPhysicalTemplate(): BrowserAndroidOwnedShellUrlCustodyTemplate {",
        "  return {",
        "    custodyState: 'physical-owned-shell-request-url-ref',",
        "    reasonCode: 'physical-owned-shell-view-intent-url-ref-custody',",
        "    ...BrowserAndroidOwnedShellUrlCustodyNoClaimFlags,",
        "  };",
        "}",
        "",
        "export function browserAndroidOwnedShellUrlCustodyManualTemplate(): BrowserAndroidOwnedShellUrlCustodyTemplate {",
        "  return {",
        "    custodyState: 'manual-required',",
        "    reasonCode: 'android-active-tab-policy-execution-and-enforcement-proof-required',",
        "    ...BrowserAndroidOwnedShellUrlCustodyNoClaimFlags,",
        "  };",
        "}",
        "",
    ]
    .join("\n")
}
