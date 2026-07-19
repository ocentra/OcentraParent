use super::*;

pub(super) fn peer_role_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentPeerRole>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Portal",
            value: AgentPeerRole::Portal,
        },
        ProtocolLiteralDescriptor {
            key: "AgentService",
            value: AgentPeerRole::AgentService,
        },
        ProtocolLiteralDescriptor {
            key: "CloudRelay",
            value: AgentPeerRole::CloudRelay,
        },
    ]
}

pub(super) fn route_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentRoute>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Localhost",
            value: AgentRoute::Localhost,
        },
        ProtocolLiteralDescriptor {
            key: "LocalNetwork",
            value: AgentRoute::LocalNetwork,
        },
        ProtocolLiteralDescriptor {
            key: "CloudRelay",
            value: AgentRoute::CloudRelay,
        },
    ]
}

pub(super) fn log_level_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ocentra_parent_agent_protocol::LogLevel>> {
    use ocentra_parent_agent_protocol::LogLevel;

    vec![
        ProtocolLiteralDescriptor {
            key: "Trace",
            value: LogLevel::Trace,
        },
        ProtocolLiteralDescriptor {
            key: "Debug",
            value: LogLevel::Debug,
        },
        ProtocolLiteralDescriptor {
            key: "Info",
            value: LogLevel::Info,
        },
        ProtocolLiteralDescriptor {
            key: "Warn",
            value: LogLevel::Warn,
        },
        ProtocolLiteralDescriptor {
            key: "Error",
            value: LogLevel::Error,
        },
    ]
}

pub(super) fn peer_default_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentPeer>> {
    vec![ProtocolLiteralDescriptor {
        key: "PortalDev",
        value: AgentPeer {
            peer_id: peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
    }]
}

pub(super) fn target_default_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentMessageTarget>> {
    vec![
        target_default("LocalhostWindowsAgent", AgentRoute::Localhost),
        target_default("LocalNetworkWindowsAgent", AgentRoute::LocalNetwork),
    ]
}

fn target_default(
    key: &'static str,
    route: AgentRoute,
) -> ProtocolLiteralDescriptor<AgentMessageTarget> {
    ProtocolLiteralDescriptor {
        key,
        value: AgentMessageTarget {
            device_id: peer::LOCAL_DEV_AGENT.to_string(),
            platform: local_ai_runtime::PLATFORM_OS_WINDOWS.to_string(),
            route,
        },
    }
}

pub(super) fn lan_household_action_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>>
{
    vec![
        field_descriptor("Assign", lan_pairing::HOUSEHOLD_ACTION_ASSIGN),
        field_descriptor("Rename", lan_pairing::HOUSEHOLD_ACTION_RENAME),
        field_descriptor("Ignore", lan_pairing::HOUSEHOLD_ACTION_IGNORE),
        field_descriptor("Restore", lan_pairing::HOUSEHOLD_ACTION_RESTORE),
        field_descriptor("Trust", lan_pairing::HOUSEHOLD_ACTION_TRUST),
    ]
}

pub(super) fn lan_intent_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![field_descriptor(
        "ConfigurationUpdate",
        value::LAN_INTENT_CONFIGURATION_UPDATE,
    )]
}

pub(super) fn lan_parent_authority_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![field_descriptor(
        "ActiveController",
        value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
    )]
}

pub(super) fn lan_discovery_event_kind_descriptors() -> Vec<
    ProtocolLiteralDescriptor<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind,
    >,
> {
    use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind;

    vec![
        ProtocolLiteralDescriptor {
            key: "InterfaceChanged",
            value: LanDiscoveryEventKind::InterfaceChanged,
        },
        ProtocolLiteralDescriptor {
            key: "ScanStarted",
            value: LanDiscoveryEventKind::ScanStarted,
        },
        ProtocolLiteralDescriptor {
            key: "ScanFinished",
            value: LanDiscoveryEventKind::ScanFinished,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceFound",
            value: LanDiscoveryEventKind::EvidenceFound,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceFound",
            value: LanDiscoveryEventKind::DeviceFound,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceUpdated",
            value: LanDiscoveryEventKind::DeviceUpdated,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOnline",
            value: LanDiscoveryEventKind::DeviceOnline,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOffline",
            value: LanDiscoveryEventKind::DeviceOffline,
        },
        ProtocolLiteralDescriptor {
            key: "AgentDiscovered",
            value: LanDiscoveryEventKind::AgentDiscovered,
        },
        ProtocolLiteralDescriptor {
            key: "AgentConfirmed",
            value: LanDiscoveryEventKind::AgentConfirmed,
        },
        ProtocolLiteralDescriptor {
            key: "UnknownDetected",
            value: LanDiscoveryEventKind::UnknownDetected,
        },
    ]
}
