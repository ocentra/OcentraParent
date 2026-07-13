pub(crate) mod fields {
    pub(crate) const fields_from_pairs: fn(
        Vec<(
            &'static str,
            ocentra_parent_agent_protocol::logging::LogFieldValue,
        )>,
    )
        -> ocentra_parent_agent_protocol::logging::LogFields = crate::fields::fields_from_pairs;
}

pub(crate) mod lan_pairing {
    pub(crate) type LanPairingRuntime = crate::lan_pairing::LanPairingRuntime;

    pub(crate) const route_trust_state:
        fn(
            Option<&ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget>,
        ) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingText =
        crate::lan_pairing_status::selection::route_trust_state;
}

pub(crate) mod lan_pairing_status {
    pub(crate) const pairing_status_event:
        fn(
            &crate::lan_pairing::LanPairingRuntime,
            ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
        ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope =
        crate::lan_pairing_status::pairing_status_event;
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

        pub(crate) const local_network_change_triggers: fn(
            Option<&ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity>,
            &ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity,
        ) -> Vec<
            LanPassiveDiscoveryLocalNetworkChangeTrigger,
        > = crate::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers;

        pub(crate) const passive_discovery_udp_sources:
            fn() -> &'static [ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource] =
            crate::lan_pairing_runtime_state::passive_discovery::passive_discovery_udp_sources;
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
