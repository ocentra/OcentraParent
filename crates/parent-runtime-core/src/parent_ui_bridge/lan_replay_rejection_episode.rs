use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{SecondsFormat, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteContext, ParentRouteEventId, ParentRouteEventSnapshot, ParentRouteId,
    ParentRoutePeerId, ParentRoutePeerRole, ParentSubscriptionEvent,
};

const LAN_REPLAY_REJECTION_EVENT: &str = "lan-runtime-event-chain-replay-rejected";
const LAN_REPLAY_REJECTION_SEVERITY: &str = "warn";
static LAN_REPLAY_REJECTION_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Per-subscription state for one active replay-rejection warning episode.
#[derive(Debug, Default)]
pub struct ParentRouteSubscriptionLoadState {
    active_replay_rejection_diagnostic: Option<ParentRouteEventSnapshot>,
}

impl ParentRouteSubscriptionLoadState {
    /// Loads one subscription poll while preserving rejection-episode identity.
    pub fn load(
        &mut self,
        route: ParentRouteId,
        context: Option<&ParentRouteContext>,
    ) -> ParentSubscriptionEvent {
        super::load_parent_subscription_event_with_state(self, route, context, None)
    }

    pub fn load_with_service_health(
        &mut self,
        route: ParentRouteId,
        context: Option<&ParentRouteContext>,
        service_health: &super::ParentAgentServiceHealth,
    ) -> ParentSubscriptionEvent {
        super::load_parent_subscription_event_with_state(self, route, context, Some(service_health))
    }

    pub(super) fn replay_rejection_diagnostic(&mut self) -> ParentRouteEventSnapshot {
        // CLONE-JUSTIFICATION: the state retains the canonical episode warning while each poll owns its emitted event batch.
        self.active_replay_rejection_diagnostic
            .get_or_insert_with(replay_rejection_diagnostic)
            .clone()
    }

    pub(super) fn complete_replay_rejection_episode(&mut self) {
        self.active_replay_rejection_diagnostic = None;
    }
}

fn replay_rejection_diagnostic() -> ParentRouteEventSnapshot {
    let now = Utc::now();
    let sequence = LAN_REPLAY_REJECTION_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let event_id = format!(
        "{LAN_REPLAY_REJECTION_EVENT}-{}-{sequence}",
        now.timestamp_micros()
    );
    ParentRouteEventSnapshot {
        event: Some(String::from(LAN_REPLAY_REJECTION_EVENT)),
        event_id: ParentRouteEventId::parse(event_id),
        correlation_id: None,
        sent_at: Some(now.to_rfc3339_opts(SecondsFormat::Millis, true)),
        source_peer_id: ParentRoutePeerId::parse(String::from(constants::peer::LOCAL_DEV_AGENT)),
        source_role: Some(ParentRoutePeerRole::AgentService),
        target_peer_id: ParentRoutePeerId::parse(String::from(constants::peer::PORTAL_DEV)),
        target_role: Some(ParentRoutePeerRole::Portal),
        severity: Some(String::from(LAN_REPLAY_REJECTION_SEVERITY)),
        payload: None,
        snapshot: None,
        command_result_projection: None,
    }
}
