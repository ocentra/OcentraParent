fn runtime_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export const {} = {{ SchemaVersion: {}, MessageIdPrefix: {} }} as const; export type {} = string | number | boolean | null; export type {} = Readonly<Record<string, {}>>;",
        names.runtime_const,
        AGENT_PROTOCOL_SCHEMA_VERSION,
        json_literal(&PARENT_AGENT_MESSAGE_ID_PREFIX),
        names.payload_value_type,
        names.payload_type,
        names.payload_value_type
    )
}

fn delimiter_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export const {} = {{ List: {}, EventIdSuffix: {} }} as const; export type {} = (typeof {})[keyof typeof {}];",
        names.delimiter_const,
        json_literal(&delimiter::LIST),
        json_literal(&delimiter::HYPHEN),
        names.delimiter_type,
        names.delimiter_const,
        names.delimiter_const
    )
}

fn transport_typescript(names: &ProtocolBridgeNames) -> String {
    [
        literal_typescript(
            names.peer_role_const,
            names.peer_role_type,
            &peer_role_descriptors(),
        ),
        literal_typescript(names.route_const, names.route_type, &route_descriptors()),
        peer_target_typescript(names),
        const_object_typescript(names.peer_defaults_const, &peer_default_descriptors()),
        const_object_typescript(names.target_defaults_const, &target_default_descriptors()),
        command_envelope_typescript(names),
        log_level_typescript(names),
        event_envelope_typescript(names),
        primitive_decoders_typescript(names),
    ]
    .join(" ")
}

fn field_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "{} export type {} = (typeof {})[keyof typeof {}];",
        const_object_typescript(names.field_const, &field_descriptors()),
        names.field_type,
        names.field_const,
        names.field_const
    )
}

fn bridge_prefix(names: &ProtocolBridgeNames) -> &str {
    names
        .runtime_const
        .strip_suffix("ProtocolRuntime")
        .unwrap_or(names.runtime_const)
}

fn browser_runtime_contract_decoders_typescript(
    names: &ProtocolBridgeNames,
    prefix: &str,
    contract_names: &RuntimeEventContractNames,
) -> String {
    let payload_type = format!("{prefix}BrowserRuntimeEventPayload");
    let entry_type = format!("{prefix}BrowserRuntimeEventChainEntry");
    let stream_type = format!("{prefix}BrowserRuntimeEventChainStream");
    let action_intent_candidate_type = format!("{prefix}BrowserRuntimeActionIntentCandidate");
    let payload_decoder_fn = format!("decode{prefix}BrowserRuntimeEventPayload");
    let entry_decoder_fn = format!("decode{prefix}BrowserRuntimeEventChainEntry");
    let stream_decoder_fn = format!("decode{prefix}BrowserRuntimeEventChainStream");
    let payload_schema_const = format!("{prefix}BrowserRuntimeEventPayloadSchema");
    let entry_schema_const = format!("{prefix}BrowserRuntimeEventChainEntrySchema");
    let stream_schema_const = format!("{prefix}BrowserRuntimeEventChainStreamSchema");
    let phase_event_type_const = format!("{prefix}BrowserRuntimePhaseEventType");
    let helper_prefix = format!("__{prefix}BrowserRuntime");
    let browser_event_type = &contract_names.browser_event_type_const;
    let browser_phase = &contract_names.browser_phase_const;
    let browser_capability_status = &contract_names.browser_capability_status_const;
    let browser_custody_label = &contract_names.browser_custody_label_const;
    let browser_query_visibility = &contract_names.browser_query_visibility_const;
    replace_tokens(
        parent_agent_protocol_bridge_ts_runtime_01_template(),
        &[
            ("__RUNTIME_CONST__", names.runtime_const),
            ("__BROWSER_EVENT_TYPE_CONST__", browser_event_type),
            ("__BROWSER_EVENT_TYPE_TYPE__", browser_event_type),
            ("__BROWSER_PHASE_CONST__", browser_phase),
            ("__BROWSER_PHASE_TYPE__", browser_phase),
            (
                "__BROWSER_CAPABILITY_STATUS_CONST__",
                browser_capability_status,
            ),
            (
                "__BROWSER_CAPABILITY_STATUS_TYPE__",
                browser_capability_status,
            ),
            ("__BROWSER_CUSTODY_LABEL_CONST__", browser_custody_label),
            ("__BROWSER_CUSTODY_LABEL_TYPE__", browser_custody_label),
            (
                "__BROWSER_QUERY_VISIBILITY_CONST__",
                browser_query_visibility,
            ),
            (
                "__BROWSER_QUERY_VISIBILITY_TYPE__",
                browser_query_visibility,
            ),
            ("__BROWSER_PAYLOAD_TYPE__", &payload_type),
            ("__BROWSER_ENTRY_TYPE__", &entry_type),
            ("__BROWSER_STREAM_TYPE__", &stream_type),
            (
                "__BROWSER_ACTION_INTENT_CANDIDATE_TYPE__",
                &action_intent_candidate_type,
            ),
            ("__BROWSER_PAYLOAD_DECODER_FN__", &payload_decoder_fn),
            ("__BROWSER_ENTRY_DECODER_FN__", &entry_decoder_fn),
            ("__BROWSER_STREAM_DECODER_FN__", &stream_decoder_fn),
            ("__BROWSER_PAYLOAD_SCHEMA_CONST__", &payload_schema_const),
            ("__BROWSER_ENTRY_SCHEMA_CONST__", &entry_schema_const),
            ("__BROWSER_STREAM_SCHEMA_CONST__", &stream_schema_const),
            (
                "__BROWSER_PHASE_EVENT_TYPE_CONST__",
                &phase_event_type_const,
            ),
            ("__HELPER_PREFIX__", &helper_prefix),
        ],
    )
}

fn parent_agent_protocol_bridge_ts_runtime_01_template() -> String {
    [
        include_str!("parent_agent_protocol_bridge_ts_runtime_parent_agent_protocol_bridge_ts_runtime_01_01.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_runtime_parent_agent_protocol_bridge_ts_runtime_01_02.template.txt"),
    ]
    .concat()
}
fn parent_agent_protocol_bridge_ts_runtime_02_template() -> String {
    [
        include_str!("parent_agent_protocol_bridge_ts_runtime_parent_agent_protocol_bridge_ts_runtime_02_01.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_runtime_parent_agent_protocol_bridge_ts_runtime_02_02.template.txt"),
    ]
    .concat()
}
