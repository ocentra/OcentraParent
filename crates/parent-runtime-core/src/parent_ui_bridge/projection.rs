//! Transport-independent projection of typed agent-service responses.
//!
//! This boundary does not establish a session, authorize a command, or bypass the
//! authenticated parent-local transport owner. It only applies already-observed,
//! typed protocol envelopes to the same parsers and assemblers used after a real
//! production transport response.

use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventEnvelope};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteId, ParentRouteSnapshot, ParentSubscriptionEvent,
};

use crate::agent_service_client::snapshots_app_game::{
    app_game_adapter_dispatch_preflight_snapshot_from_result,
    app_game_adapter_dispatch_result_snapshot_from_result,
    app_game_child_runtime_transport_receipt_snapshot_from_result,
    app_game_notification_readiness_snapshot_from_result,
    app_game_platform_proof_status_snapshot_from_result,
    app_game_policy_readiness_snapshot_from_result,
    app_game_timer_parent_surface_snapshot_from_result,
};
use crate::agent_service_client::snapshots_browser::{
    browser_activity_read_model_snapshot_from_result,
    browser_evidence_read_model_snapshot_from_result,
    browser_intervention_read_model_snapshot_from_result,
    browser_inventory_read_model_snapshot_from_result, browser_managed_status_snapshot_from_result,
};
use crate::agent_service_client::snapshots_common::parent_route_event_snapshot;
use crate::agent_service_client::snapshots_lan::{
    lan_runtime_replay_events_from_result, lan_snapshot_from_result,
    network_flow_snapshot_from_result, network_runtime_event_chain_snapshot_from_result,
    policy_preview_snapshot_from_result,
};
use crate::agent_service_client::snapshots_tracking::{
    activity_app_use_read_model_snapshot_from_result,
    activity_games_read_model_snapshot_from_result,
    activity_screen_read_model_snapshot_from_result, tracking_read_model_snapshot_from_result,
};
use crate::agent_service_client::types::AgentServiceCommandResult;

use super::lan_replay_rejection_episode::ParentRouteSubscriptionLoadState;
use super::lan_route::{is_lan_command_route, LanRouteQuery};
use super::route_requirements::*;
use super::route_snapshot::build_parent_route_snapshot_from_dependencies;
use super::route_snapshot::dependencies::{DependencyFailures, ParentRouteSnapshotDependencies};
use super::{build_parent_subscription_event_from_parts, ParentSubscriptionReplay};

mod action;
#[path = "projection/browser_social.rs"]
mod browser_social;

/// One typed response batch for projection only.
pub struct ParentAgentServiceProjectionResponse {
    result: AgentServiceCommandResult,
}

impl ParentAgentServiceProjectionResponse {
    /// Builds a projection response from the exact typed envelopes observed for
    /// one command. The last envelope is the command response; earlier envelopes
    /// remain visible as route events.
    pub fn from_envelopes(
        command: AgentCommandName,
        command_message_id: String,
        envelopes: Vec<AgentEventEnvelope>,
    ) -> Result<Self, String> {
        let response_event = envelopes
            .last()
            .cloned()
            .ok_or_else(|| "parent projection requires a typed response envelope".to_string())?;
        let request_sent_at = response_event.sent_at.clone();
        let events = envelopes
            .into_iter()
            .map(|event| parent_route_event_snapshot(&event))
            .collect();
        Ok(Self {
            result: AgentServiceCommandResult {
                command,
                command_message_id,
                request_nonce: String::new(),
                request_sent_at,
                events,
                response_event,
            },
        })
    }
}

/// A finite set of already-observed responses consumed by pure projection.
pub struct ParentAgentServiceProjection {
    responses: Vec<ParentAgentServiceProjectionResponse>,
}

impl ParentAgentServiceProjection {
    pub fn new(responses: Vec<ParentAgentServiceProjectionResponse>) -> Self {
        Self { responses }
    }

    /// Projects one route snapshot without opening a transport or minting owner
    /// authority. Missing or malformed required responses degrade through the
    /// same dependency-failure snapshot used by production loading.
    pub fn route_snapshot(mut self, route: ParentRouteId) -> ParentRouteSnapshot {
        let lan_route_query = self.project_lan_query(&route);
        let dependencies = self.project_dependencies(&route, None);
        build_parent_route_snapshot_from_dependencies(
            route,
            &lan_route_query,
            None,
            None,
            None,
            dependencies,
        )
    }

    /// Projects one subscription poll while preserving the caller-owned replay
    /// rejection episode state.
    pub fn subscription_event(
        mut self,
        state: &mut ParentRouteSubscriptionLoadState,
        route: ParentRouteId,
    ) -> ParentSubscriptionEvent {
        let lan_route_query = self.project_lan_query(&route);
        let replay = if matches!(lan_route_query, LanRouteQuery::Available(_)) {
            match self
                .take(&AgentCommandName::AgentLanRuntimeEventChainStreamGet)
                .and_then(lan_runtime_replay_events_from_result)
            {
                Ok(replay) => ParentSubscriptionReplay::Reported(replay),
                Err(_) => ParentSubscriptionReplay::Rejected,
            }
        } else {
            ParentSubscriptionReplay::NotRequested
        };
        let dependencies = self.project_dependencies(&route, None);
        let snapshot = build_parent_route_snapshot_from_dependencies(
            route.clone(),
            &lan_route_query,
            None,
            None,
            None,
            dependencies,
        );
        build_parent_subscription_event_from_parts(state, route, &lan_route_query, replay, snapshot)
    }

    fn project_lan_query(&mut self, route: &ParentRouteId) -> LanRouteQuery {
        if !is_lan_command_route(route) && route != &ParentRouteId::Start {
            return LanRouteQuery::NotRequired;
        }
        match self
            .take(&AgentCommandName::AgentLanPairingStatusGet)
            .and_then(lan_snapshot_from_result)
        {
            Ok(snapshot) => LanRouteQuery::Available(Box::new(snapshot)),
            Err(error) => LanRouteQuery::Unavailable(error),
        }
    }

    fn project_dependencies(
        &mut self,
        route: &ParentRouteId,
        network_flow_override: Option<
            crate::agent_service_client::types::NetworkFlowAgentServiceSnapshot,
        >,
    ) -> ParentRouteSnapshotDependencies {
        let mut loaded = ParentRouteSnapshotDependencies::default();
        loaded.network_flow_snapshot = network_flow_override.or_else(|| {
            self.project_required(
                route_requires_network_flow_read_model(route),
                "network-flow-read-model",
                &AgentCommandName::AgentNetworkFlowReadModelGet,
                network_flow_snapshot_from_result,
                &mut loaded.dependency_failures,
            )
        });
        let has_network_flow = loaded.network_flow_snapshot.is_some();
        loaded.network_runtime_event_chain_snapshot = self.project_required(
            has_network_flow || route_requires_network_runtime_event_chain_stream(route),
            "network-runtime-event-chain",
            &AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
            network_runtime_event_chain_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.policy_preview_snapshot = self.project_required(
            has_network_flow || route_requires_policy_preview_read_model(route),
            "policy-preview-read-model",
            &AgentCommandName::AgentPolicyPreviewReadModelGet,
            policy_preview_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        self.project_activity_dependencies(route, &mut loaded);
        self.project_browser_dependencies(route, &mut loaded);
        self.project_app_game_dependencies(route, &mut loaded);
        loaded
    }

    fn project_activity_dependencies(
        &mut self,
        route: &ParentRouteId,
        loaded: &mut ParentRouteSnapshotDependencies,
    ) {
        loaded.tracking_read_model_snapshot = self.project_required(
            route_requires_tracking_read_model(route),
            "tracking-read-model",
            &AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.screen_read_model_snapshot = self.project_required(
            route_requires_screen_summary_read_model(route),
            "screen-read-model",
            &AgentCommandName::AgentActivityScreenReadModelGet,
            activity_screen_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
    }

    fn project_browser_dependencies(
        &mut self,
        route: &ParentRouteId,
        loaded: &mut ParentRouteSnapshotDependencies,
    ) {
        loaded.browser_activity_read_model_snapshot = self.project_required(
            route_requires_browser_activity_read_model(route),
            "browser-activity-read-model",
            &AgentCommandName::AgentActivityBrowserReadModelGet,
            browser_activity_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.browser_inventory_read_model_snapshot = self.project_required(
            route_requires_browser_inventory_read_model(route),
            "browser-inventory-read-model",
            &AgentCommandName::AgentBrowserInventoryReadModelGet,
            browser_inventory_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.browser_evidence_read_model_snapshot = self.project_required(
            route_requires_browser_evidence_read_model(route),
            "browser-evidence-read-model",
            &AgentCommandName::AgentBrowserEvidenceRecentGet,
            browser_evidence_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.browser_managed_status_snapshot = self.project_required(
            route_requires_browser_managed_status(route),
            "browser-managed-status",
            &AgentCommandName::AgentBrowserManagedBridgePoll,
            browser_managed_status_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.browser_intervention_read_model_snapshot = self.project_required(
            route_requires_browser_read_models(route),
            "browser-intervention-read-model",
            &AgentCommandName::AgentBrowserInterventionReadModelGet,
            browser_intervention_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        self.project_browser_social_dependencies(route, loaded);
    }

    fn project_app_game_dependencies(
        &mut self,
        route: &ParentRouteId,
        loaded: &mut ParentRouteSnapshotDependencies,
    ) {
        let required = route_requires_activity_app_game_read_models(route);
        loaded.app_use_read_model_snapshot = self.project_required(
            required,
            "app-use-read-model",
            &AgentCommandName::AgentActivityAppUseReadModelGet,
            activity_app_use_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        loaded.games_read_model_snapshot = self.project_required(
            required,
            "games-read-model",
            &AgentCommandName::AgentActivityGamesReadModelGet,
            activity_games_read_model_snapshot_from_result,
            &mut loaded.dependency_failures,
        );
        self.project_app_game_status_dependencies(
            route_requires_app_game_session_read_models(route),
            loaded,
        );
    }

    fn project_app_game_status_dependencies(
        &mut self,
        required: bool,
        loaded: &mut ParentRouteSnapshotDependencies,
    ) {
        loaded.app_game_notification_readiness_snapshot = self.project_required(
            required,
            "app-game-notification-readiness",
            &AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet,
            |result| app_game_notification_readiness_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_policy_readiness_snapshot = self.project_required(
            required,
            "app-game-policy-readiness",
            &AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
            |result| app_game_policy_readiness_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_platform_proof_status_snapshot = self.project_required(
            required,
            "app-game-platform-proof-status",
            &AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
            |result| app_game_platform_proof_status_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_child_runtime_transport_receipt_snapshot = self.project_required(
            required,
            "app-game-child-runtime-transport-receipt",
            &AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
            |result| app_game_child_runtime_transport_receipt_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_adapter_dispatch_preflight_snapshot = self.project_required(
            required,
            "app-game-adapter-dispatch-preflight",
            &AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
            |result| app_game_adapter_dispatch_preflight_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_adapter_dispatch_result_snapshot = self.project_required(
            required,
            "app-game-adapter-dispatch-result",
            &AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
            |result| app_game_adapter_dispatch_result_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
        loaded.app_game_timer_parent_surface_snapshot = self.project_required(
            required,
            "app-game-timer-parent-surface",
            &AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
            |result| app_game_timer_parent_surface_snapshot_from_result(&result),
            &mut loaded.dependency_failures,
        );
    }

    fn project_required<T>(
        &mut self,
        required: bool,
        label: &'static str,
        command: &AgentCommandName,
        project: impl FnOnce(AgentServiceCommandResult) -> Result<T, String>,
        failures: &mut DependencyFailures,
    ) -> Option<T> {
        required
            .then(|| failures.capture(label, self.take(command).and_then(project)))
            .flatten()
    }

    fn take(&mut self, command: &AgentCommandName) -> Result<AgentServiceCommandResult, String> {
        let position = self
            .responses
            .iter()
            .position(|response| &response.result.command == command)
            .ok_or_else(|| format!("parent projection missing response for {command:?}"))?;
        Ok(self.responses.remove(position).result)
    }
}
