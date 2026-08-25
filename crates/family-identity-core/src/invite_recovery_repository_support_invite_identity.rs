use ocentra_schema::account_identity_authority::{AccountIdentityProvider, AccountIdentityRole};

pub(crate) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(crate) fn role_label(role: AccountIdentityRole) -> &'static str {
    match role {
        AccountIdentityRole::ParentOwner => "parent-owner",
        AccountIdentityRole::CoParentGuardian => "co-parent-guardian",
        AccountIdentityRole::Observer => "observer",
        AccountIdentityRole::ChildProfile => "child-profile",
        AccountIdentityRole::ChildDeviceAgent => "child-device-agent",
        AccountIdentityRole::SupportAdmin => "support-admin",
    }
}
