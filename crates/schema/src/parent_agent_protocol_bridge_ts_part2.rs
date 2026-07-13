mod parent_agent_protocol_bridge_ts_part2_tracking;
mod parent_agent_protocol_bridge_ts_part2_typescript;

fn tracking_retention_settings_write_typescript(names: &ProtocolBridgeNames) -> String {
    parent_agent_protocol_bridge_ts_part2_tracking::tracking_retention_settings_write_typescript(
        names,
    )
}

fn activity_surface_contract_typescript(names: &ProtocolBridgeNames) -> String {
    parent_agent_protocol_bridge_ts_part2_typescript::activity_surface_contract_typescript(names)
}
