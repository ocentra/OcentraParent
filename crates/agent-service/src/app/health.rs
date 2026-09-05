#![forbid(unsafe_code)]

use axum::{extract::State, Json};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentServiceHealthAuthenticationState, ParentServiceHealthReason, ParentServiceHealthRoute,
    ParentServiceHealthSnapshot, ParentServiceHealthState, ParentServiceHealthTraceSnapshot,
    ParentServiceHealthTransport,
};

use super::AppState;

pub(super) async fn handle(State(state): State<AppState>) -> Json<ParentServiceHealthSnapshot> {
    Json(snapshot(&state))
}

fn snapshot(state: &AppState) -> ParentServiceHealthSnapshot {
    if state.parent_local_bridge_admission.is_ready() {
        return ready_snapshot(state);
    }
    degraded_snapshot(state)
}

fn ready_snapshot(state: &AppState) -> ParentServiceHealthSnapshot {
    ParentServiceHealthSnapshot {
        state: ParentServiceHealthState::Ready,
        route: Some(route(state)),
        protocol_schema_version: Some(AGENT_PROTOCOL_SCHEMA_VERSION),
        service_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        transport: Some(ParentServiceHealthTransport::WebSocket),
        authentication_state: ParentServiceHealthAuthenticationState::Authenticated,
        reason: ParentServiceHealthReason::Ready,
        trace: empty_trace(),
    }
}

fn degraded_snapshot(state: &AppState) -> ParentServiceHealthSnapshot {
    ParentServiceHealthSnapshot {
        state: ParentServiceHealthState::Degraded,
        route: Some(route(state)),
        protocol_schema_version: Some(AGENT_PROTOCOL_SCHEMA_VERSION),
        service_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        transport: None,
        authentication_state: ParentServiceHealthAuthenticationState::Unavailable,
        reason: ParentServiceHealthReason::RouteDependencyUnavailable,
        trace: empty_trace(),
    }
}

fn route(state: &AppState) -> ParentServiceHealthRoute {
    if state.network.bind_address().ip().is_loopback() {
        ParentServiceHealthRoute::Localhost
    } else {
        ParentServiceHealthRoute::LocalNetwork
    }
}

fn empty_trace() -> ParentServiceHealthTraceSnapshot {
    ParentServiceHealthTraceSnapshot {
        request_id: None,
        correlation_id: None,
        response_event_id: None,
        request_sent_at: None,
        response_sent_at: None,
    }
}
