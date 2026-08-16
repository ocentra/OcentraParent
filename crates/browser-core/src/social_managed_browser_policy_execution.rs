#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocialManagedBrowserPolicyExecutionTemplate {
    pub execution_state: &'static str,
    pub managed_session_observed: bool,
    pub exact_managed_url_observed: bool,
    pub live_surface_captured_before_mutation: bool,
    pub browser_mutation_observed: bool,
    pub child_intervention_executed: bool,
    pub managed_intervention_enforced: bool,
    pub final_policy_execution_claimed: bool,
    pub unmanaged_browser_claimed: bool,
    pub broad_os_enforcement_claimed: bool,
    pub provider_delivery_attempted: bool,
    pub native_app_control_claimed: bool,
    pub apple_platform_claimed: bool,
    pub raw_url_persisted: bool,
    pub raw_page_content_persisted: bool,
}

const SOCIAL_MANAGED_BROWSER_POLICY_EXECUTION_TEMPLATE:
    SocialManagedBrowserPolicyExecutionTemplate = SocialManagedBrowserPolicyExecutionTemplate {
    execution_state: "managed-browser-intervention-executed",
    managed_session_observed: true,
    exact_managed_url_observed: true,
    live_surface_captured_before_mutation: true,
    browser_mutation_observed: true,
    child_intervention_executed: true,
    managed_intervention_enforced: true,
    final_policy_execution_claimed: true,
    unmanaged_browser_claimed: false,
    broad_os_enforcement_claimed: false,
    provider_delivery_attempted: false,
    native_app_control_claimed: false,
    apple_platform_claimed: false,
    raw_url_persisted: false,
    raw_page_content_persisted: false,
};

pub fn social_managed_browser_policy_execution_template(
) -> SocialManagedBrowserPolicyExecutionTemplate {
    SOCIAL_MANAGED_BROWSER_POLICY_EXECUTION_TEMPLATE
}

pub fn social_managed_browser_policy_execution_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_managed_browser_policy_execution.ts"
    ))
    .to_string()
}
