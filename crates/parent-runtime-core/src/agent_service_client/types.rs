use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::AppGamePolicyReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};
use ocentra_schema::parent_ui_bridge::{
    ParentActivityTrackingReadModelResultSnapshot, ParentNetworkRuntimeEventChainStreamSnapshot,
    ParentPolicyPreviewReadModelSnapshot, ParentRouteEventSnapshot,
};

pub(crate) struct LanAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) read_model: LanBrowserAddDeviceReadModel,
}

pub(crate) struct NetworkFlowAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) read_model: ActivityNetworkFlowReadModel,
}

pub(crate) struct NetworkRuntimeEventChainAgentServiceSnapshot {
    pub(crate) stream: ParentNetworkRuntimeEventChainStreamSnapshot,
}

pub(crate) struct PolicyPreviewAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) read_model: ParentPolicyPreviewReadModelSnapshot,
}

pub(crate) struct TrackingReadModelAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) read_model: ParentActivityTrackingReadModelResultSnapshot,
}

pub(crate) struct ScreenReadModelAgentServiceSnapshot {
    pub(crate) read_model: ActivityScreenReadModel,
}

pub(crate) struct AppGameNotificationReadinessAgentServiceSnapshot {
    pub(crate) read_model: AppGameNotificationReadinessReadModel,
}

pub(crate) struct AppGamePolicyReadinessAgentServiceSnapshot {
    pub(crate) read_model: AppGamePolicyReadinessReadModel,
}

pub(crate) struct AppGamePlatformProofStatusAgentServiceSnapshot {
    pub(crate) read_model: AppGamePlatformProofStatusReadModel,
}

pub(crate) struct AppGameChildRuntimeTransportReceiptAgentServiceSnapshot {
    pub(crate) read_model: AppGameChildRuntimeTransportReceiptReadModel,
}

pub(crate) struct AppGameAdapterDispatchPreflightAgentServiceSnapshot {
    pub(crate) read_model: AppGameAdapterDispatchPreflightReadModel,
}

pub(crate) struct AppGameAdapterDispatchResultAgentServiceSnapshot {
    pub(crate) read_model: AppGameAdapterDispatchResultReadModel,
}

pub(crate) struct AppGameTimerParentSurfaceAgentServiceSnapshot {
    pub(crate) read_model: AppGameTimerParentSurfaceReadModel,
}

pub(crate) struct AgentServiceCommandResult {
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) response_event: AgentEventEnvelope,
}

impl AgentServiceCommandResult {
    pub(crate) fn is_rejected(&self) -> bool {
        self.response_event.event == AgentEventName::AgentCommandRejected
    }

    pub(crate) fn rejection_message(&self) -> Option<String> {
        self.is_rejected()
            .then(|| super::transport::rejection_message(&self.response_event))
    }
}
