use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingText, LanSignedChildAgentClaim, LanSignedChildAgentMessageKind,
};

use crate::lan_pairing::LanPairingRuntime;

const SIGNED_CHILD_MESSAGE_KIND_HELLO: &str = "hello";
const SIGNED_CHILD_MESSAGE_KIND_HEARTBEAT: &str = "heartbeat";
const SIGNED_CHILD_OBSERVED_SUMMARY_PREFIX: &str = "signed child ";
const SIGNED_CHILD_OBSERVED_SUMMARY_ROUTE_SEPARATOR: &str = " observed: route=";
const SIGNED_CHILD_OBSERVED_SUMMARY_INSTALL_SEPARATOR: &str = "; install-id=";

impl LanPairingRuntime {
    pub(crate) fn record_signed_child_agent_passive_observation(
        &self,
        claim: &LanSignedChildAgentClaim,
        observed_at: &LanPairingText,
    ) {
        let message_kind = match claim.message_kind {
            LanSignedChildAgentMessageKind::Hello => SIGNED_CHILD_MESSAGE_KIND_HELLO,
            LanSignedChildAgentMessageKind::Heartbeat => SIGNED_CHILD_MESSAGE_KIND_HEARTBEAT,
        };
        let summary = signed_child_observed_summary(
            &LanPairingText(message_kind.to_string()),
            &LanPairingText(claim.route_id.clone()),
            &LanPairingText(claim.install_id.clone()),
        );
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            let _ = state.record_passive_update(
                LanPassiveDiscoverySource::OcentraBeacon,
                LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
                observed_at.0.as_str(),
                Some(claim.child_device_id.as_str()),
                None,
                summary.0,
            );
        }
    }
}

fn signed_child_observed_summary(
    message_kind: &LanPairingText,
    route_id: &LanPairingText,
    install_id: &LanPairingText,
) -> LanPairingText {
    let mut summary = String::from(SIGNED_CHILD_OBSERVED_SUMMARY_PREFIX);
    summary.push_str(message_kind.0.as_str());
    summary.push_str(SIGNED_CHILD_OBSERVED_SUMMARY_ROUTE_SEPARATOR);
    summary.push_str(route_id.0.as_str());
    summary.push_str(SIGNED_CHILD_OBSERVED_SUMMARY_INSTALL_SEPARATOR);
    summary.push_str(install_id.0.as_str());
    LanPairingText(summary)
}
