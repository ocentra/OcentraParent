#![forbid(unsafe_code)]

use crate::policy_source::PolicyConsumerDomain;

pub(crate) fn policy_event_domain_name(domain: PolicyConsumerDomain) -> &'static str {
    match domain {
        PolicyConsumerDomain::App => "app",
        PolicyConsumerDomain::Browser => "browser",
        PolicyConsumerDomain::Network => "network",
        PolicyConsumerDomain::Tracking => "tracking",
        PolicyConsumerDomain::Screen => "screen",
        PolicyConsumerDomain::Ai => "ai",
    }
}
