use ocentra_parent_agent_protocol::AgentEventEnvelope;

use crate::{
    browser_policy_runtime::BrowserPolicyRuntime, lan_pairing::LanPairingRuntime,
    screen_settings_runtime::ScreenSettingsRuntime,
};

pub(crate) async fn handle_command_text_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    super::handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::in_memory(),
        ScreenSettingsRuntime::in_memory(),
        origin,
    )
    .await
}

pub(crate) async fn handle_command_text_with_browser_policy_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    super::handle_command_text(
        text,
        lan_pairing,
        browser_policy,
        ScreenSettingsRuntime::in_memory(),
        origin,
    )
    .await
}

pub(crate) async fn handle_command_text_with_screen_settings_for_test(
    text: &str,
    lan_pairing: LanPairingRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: Option<String>,
) -> AgentEventEnvelope {
    super::handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::in_memory(),
        screen_settings,
        origin,
    )
    .await
}
