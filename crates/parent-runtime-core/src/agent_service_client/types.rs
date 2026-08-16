use ocentra_parent_agent_protocol::activity_surface::{
    ActivityAppUseReadModel, ActivityBrowserReadModel, ActivityGamesReadModel,
    ActivityScreenReadModel,
};
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::AppGamePolicyReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;
use ocentra_parent_agent_protocol::browser_intervention::BrowserInterventionReadModel;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventHistoryState,
};
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName,
};
use ocentra_schema::parent_ui_bridge::{
    ParentActivityTrackingReadModelResultSnapshot, ParentNetworkRuntimeEventChainStreamSnapshot,
    ParentPolicyPreviewReadModelSnapshot, ParentRouteEventSnapshot,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AgentServiceError(String);

pub(crate) type AgentServiceResult<T> = Result<T, AgentServiceError>;
#[derive(Clone, Copy)]
pub(crate) struct AgentCommandText<'a>(pub(crate) &'a str);

impl std::fmt::Display for AgentServiceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for AgentServiceError {}

impl AgentServiceError {
    pub(crate) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}

pub(crate) struct LanAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) read_model: LanBrowserAddDeviceReadModel,
}

pub(crate) struct LanRuntimeReplaySnapshot {
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) history_state: LanDiscoveryEventHistoryState,
    pub(crate) latest_event_id: Option<String>,
    pub(crate) latest_observed_at: Option<String>,
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

pub(crate) struct AppUseReadModelAgentServiceSnapshot {
    pub(crate) read_model: ActivityAppUseReadModel,
}

pub(crate) struct GamesReadModelAgentServiceSnapshot {
    pub(crate) read_model: ActivityGamesReadModel,
}

pub(crate) struct BrowserManagedStatusAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) status: BrowserManagedSessionStatus,
}

pub(crate) struct BrowserInventoryReadModelAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) read_model: BrowserInventoryReadModel,
}

pub(crate) struct BrowserActivityReadModelAgentServiceSnapshot {
    pub(crate) read_model: ActivityBrowserReadModel,
}

pub(crate) struct BrowserInterventionReadModelAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) read_model: BrowserInterventionReadModel,
}

pub(crate) struct AppGameNotificationReadinessAgentServiceSnapshot {
    pub(crate) read_model: AppGameNotificationReadinessReadModel,
    pub(crate) status_read_models: Option<AppGameNotificationStatusReadModels>,
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
    pub(crate) command: AgentCommandName,
    pub(crate) command_message_id: String,
    pub(crate) events: Vec<ParentRouteEventSnapshot>,
    pub(crate) response_event: AgentEventEnvelope,
}

impl AgentServiceCommandResult {
    pub(crate) fn is_rejected(&self) -> bool {
        self.response_event.event == AgentEventName::AgentCommandRejected
    }

    pub(crate) fn rejection_message(&self) -> Option<AgentServiceError> {
        self.is_rejected().then(|| {
            AgentServiceError::from_display(super::transport::rejection_message(
                &self.response_event,
            ))
        })
    }
}
