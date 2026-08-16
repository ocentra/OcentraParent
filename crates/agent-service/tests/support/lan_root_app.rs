pub(crate) mod fields {
    pub(crate) fn fields_from_pairs(
        pairs: Vec<(
            &'static str,
            ocentra_parent_agent_protocol::logging::LogFieldValue,
        )>,
    ) -> ocentra_parent_agent_protocol::logging::LogFields {
        crate::fields::fields_from_pairs(pairs)
    }
}

pub(crate) mod lan_pairing {
    pub(crate) type LanPairingRuntime = crate::lan_pairing::LanPairingRuntime;

    pub(crate) fn route_trust_state(
        target: Option<&ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget>,
    ) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingText {
        crate::lan_pairing_status::selection::route_trust_state(target)
    }
}

pub(crate) mod lan_pairing_status {
    pub(crate) fn pairing_status_event(
        runtime: &crate::lan_pairing::LanPairingRuntime,
        command: ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
    ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
        crate::lan_pairing_status::pairing_status_event(runtime, command)
    }
}

pub(crate) mod lan_pairing_runtime_state {
    pub(crate) mod mdns_advertisement {
        pub(crate) type LanMdnsAdvertisementSyncState =
            crate::lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState;
    }

    pub(crate) mod passive_discovery {
        pub(crate) type LanPassiveDiscoveryLocalNetworkChangeTrigger =
            crate::lan_pairing_runtime_state::passive_discovery::LanPassiveDiscoveryLocalNetworkChangeTrigger;
        pub(crate) type LanPassiveDiscoveryRuntimeObservedState =
            crate::lan_pairing_runtime_state::passive_discovery::LanPassiveDiscoveryRuntimeObservedState;

        pub(crate) fn local_network_change_triggers(
            previous_identity: Option<
                &ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity,
            >,
            current_identity: &ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity,
        ) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
            crate::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
                previous_identity,
                current_identity,
            )
        }

        pub(crate) fn passive_discovery_udp_sources(
        ) -> &'static [ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource]
        {
            crate::lan_pairing_runtime_state::passive_discovery::passive_discovery_udp_sources()
        }
    }

    pub(crate) mod provider_heartbeat {
        pub(crate) type LanAiProviderHeartbeatState =
            crate::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;
    }
}

pub(crate) mod websocket {
    pub(crate) async fn handle_command_text_for_test(
        text: crate::test_text::TestText,
        lan_pairing: crate::lan_pairing::LanPairingRuntime,
        origin: Option<crate::test_text::TestText>,
    ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
        crate::websocket::handle_command_text_for_test(text, lan_pairing, origin).await
    }
}
