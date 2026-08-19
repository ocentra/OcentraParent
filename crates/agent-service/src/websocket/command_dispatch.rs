use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use crate::{
    browser_policy_api::build_browser_policy_event, browser_policy_runtime::BrowserPolicyRuntime,
    browser_runtime::BrowserManagedRuntime, lan_pairing::LanPairingRuntime,
    parent_assistant_api::build_parent_assistant_scaffold_event,
    screen_settings_api::build_screen_settings_event,
    screen_settings_runtime::ScreenSettingsRuntime,
};

use super::{
    activity_command_reports::build_activity_command_report,
    ai_command_reports::build_ai_command_report,
    basic_reports::{build_log_snapshot_report, maybe_basic_report},
    browser_network_command_reports::build_browser_network_command_report,
    command_classifiers::{is_activity_command, is_browser_policy_command, is_lan_runtime_command},
    enforcement_command_reports::build_enforcement_command_report,
    lan_command_reports::build_lan_command_report,
};

pub(super) fn build_command_event(
    command: AgentCommandEnvelope,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    browser_runtime: BrowserManagedRuntime,
    screen_settings: ScreenSettingsRuntime,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        if let Some(event) = maybe_basic_report(command.clone()) {
            return event;
        }

        match command.command.clone() {
            AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply => {
                crate::activity_api::social_source_custody_mutation_payload::build_browser_social_source_custody_mutation_report(command).await
            }
            command_name if is_activity_command(&command_name) => {
                build_activity_command_report(command).await
            }
            AgentCommandName::AgentBrowserInventoryReadModelGet
            | AgentCommandName::AgentBrowserEvidenceRecentGet
            | AgentCommandName::AgentBrowserManagedBridgePoll
            | AgentCommandName::AgentBrowserInterventionReadModelGet
            | AgentCommandName::AgentBrowserRuntimeEventChainStreamGet
            | AgentCommandName::AgentNetworkFlowReadModelGet
            | AgentCommandName::AgentNetworkRuntimeEventChainStreamGet
            | AgentCommandName::AgentNetworkRemoteDeliveryStatusGet
            | AgentCommandName::AgentNetworkLiveCaptureStatusGet
            | AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet
            | AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet
            | AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet
            | AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet
            | AgentCommandName::AgentNetworkWindowsWfpGateStatusGet => {
                build_browser_network_command_report(command, browser_runtime.clone()).await
            }
            AgentCommandName::AgentLocalAiRuntimeStatusGet
            | AgentCommandName::AgentLocalAiChatGenerate
            | AgentCommandName::AgentParentAssistantAnswerGenerate
            | AgentCommandName::AgentParentAssistantMessageSend
            | AgentCommandName::AgentParentAssistantQuickActionStart
            | AgentCommandName::AgentPolicyPreviewReadModelGet
            | AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm
            | AgentCommandName::AgentPolicyRequestParentResolutionResolve => {
                build_ai_command_report(command).await
            }
            command_name if is_browser_policy_command(&command_name) => {
                build_browser_policy_event(browser_policy, command).await
            }
            AgentCommandName::AgentScreenSettingsGet
            | AgentCommandName::AgentScreenSettingsReplace => {
                build_screen_settings_event(screen_settings, command).await
            }
            AgentCommandName::AgentParentAssistantThreadList
            | AgentCommandName::AgentParentAssistantThreadCreate
            | AgentCommandName::AgentParentAssistantThreadOpen
            | AgentCommandName::AgentParentAssistantThreadArchive
            | AgentCommandName::AgentParentAssistantRunCancel
            | AgentCommandName::AgentParentAssistantActionPreview
            | AgentCommandName::AgentParentAssistantActionConfirm
            | AgentCommandName::AgentParentAssistantProviderStatusGet => {
                build_parent_assistant_scaffold_event(command)
            }
            AgentCommandName::AgentEnforcementTimerRecover
            | AgentCommandName::AgentEnforcementTimerExpire
            | AgentCommandName::AgentEnforcementOverrideCancel
            | AgentCommandName::AgentEnforcementExecute
            | AgentCommandName::AgentEnforcementProductControlSpineGet
            | AgentCommandName::AgentEnforcementPolicyDispatchGet
            | AgentCommandName::AgentEnforcementBroadAdapterProofGet
            | AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet => {
                Box::pin(build_enforcement_command_report(command)).await
            }
            command_name if is_lan_runtime_command(&command_name) => {
                build_lan_command_report(&lan_pairing, command).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
