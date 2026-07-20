#[path = "parent_agent_protocol_bridge_ts_part3_commands.rs"]
mod commands;
#[path = "parent_agent_protocol_bridge_ts_part3_descriptors.rs"]
mod descriptors;
#[path = "parent_agent_protocol_bridge_ts_part3_events.rs"]
mod events;
#[path = "parent_agent_protocol_bridge_ts_part3_fields.rs"]
mod fields;
#[path = "parent_agent_protocol_bridge_ts_part3_network_refs.rs"]
mod network_refs;

use self::commands::command_descriptors;
use self::descriptors::{
    lan_discovery_event_kind_descriptors, lan_household_action_kind_descriptors,
    lan_intent_kind_descriptors, lan_parent_authority_descriptors, log_level_descriptors,
    peer_default_descriptors, peer_role_descriptors, route_descriptors, target_default_descriptors,
};
use self::events::event_descriptors;
use self::fields::field_descriptors;
use self::network_refs::{
    network_android_vpn_service_gate_status_ref_descriptors,
    network_apple_network_extension_gate_status_ref_descriptors,
    network_linux_nftables_lab_status_ref_descriptors, network_live_capture_status_ref_descriptors,
    network_remote_delivery_status_ref_descriptors,
    network_windows_firewall_lab_status_ref_descriptors,
    network_windows_wfp_gate_status_ref_descriptors,
};

fn lan_value_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "{} {} {} {} export const {} = {} as const; export type {} = (typeof {})[number]; export const {} = {}.LanHouseholdActionDeviceKind;",
        literal_typescript(
            names.lan_household_action_kind_const,
            names.lan_household_action_kind_type,
            &lan_household_action_kind_descriptors(),
        ),
        literal_typescript(
            names.lan_intent_kind_const,
            names.lan_intent_kind_type,
            &lan_intent_kind_descriptors(),
        ),
        literal_typescript(
            names.lan_parent_authority_const,
            names.lan_parent_authority_type,
            &lan_parent_authority_descriptors(),
        ),
        literal_typescript(
            names.lan_discovery_event_kind_const,
            names.lan_discovery_event_kind_type,
            &lan_discovery_event_kind_descriptors(),
        ),
        names.lan_household_device_kind_values_const,
        json_literal(&lan_pairing::HOUSEHOLD_DEVICE_KINDS),
        names.lan_household_device_kind_type,
        names.lan_household_device_kind_values_const,
        names.lan_household_action_device_kind_field_const,
        names.field_const
    )
}

fn literal_typescript<T: Serialize>(
    const_name: &str,
    type_name: &str,
    descriptors: &[ProtocolLiteralDescriptor<T>],
) -> String {
    format!(
        "{} export type {} = (typeof {})[keyof typeof {}];",
        const_object_typescript(const_name, descriptors),
        type_name,
        const_name,
        const_name,
    )
}

fn const_object_typescript<T: Serialize>(
    name: &str,
    descriptors: &[ProtocolLiteralDescriptor<T>],
) -> String {
    let entries = descriptors
        .iter()
        .map(|descriptor| format!("{}: {}", descriptor.key, json_literal(&descriptor.value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("export const {name} = {{ {entries} }} as const;")
}

fn peer_target_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export interface {} {{ readonly peerId: string; readonly role: {}; }} export interface {} {{ readonly deviceId: string; readonly platform: string; readonly route: {}; }}",
        names.peer_type, names.peer_role_type, names.target_type, names.route_type
    )
}

fn command_envelope_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export interface {} {{ readonly schemaVersion: number; readonly messageId: string; readonly sentAt: string; readonly source: {}; readonly target: {}; readonly command: {}; readonly payload: {}; }} {}",
        names.command_envelope_type,
        names.peer_type,
        names.target_type,
        names.command_type,
        names.payload_type,
        command_envelope_decoder_typescript(names)
    )
}

fn command_envelope_decoder_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export function {}(value: unknown): {} {{ const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => {{ const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) {{ throw new TypeError(`${{field}} must be a non-empty string`); }} return fieldValue; }}; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => {{ const fieldValue = record[field]; if (typeof fieldValue !== 'number') {{ throw new TypeError(`${{field}} must be a number`); }} return fieldValue; }}; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => {{ const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== {}.SchemaVersion) {{ throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); }} return schemaVersion; }}; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => {{ const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) {{ throw new TypeError(`${{field}} is not a Rust-owned protocol literal`); }} return fieldValue as T; }}; const readPeer = (candidate: unknown): {} => {{ if (!isRecord(candidate)) {{ throw new TypeError('peer must be an object'); }} return {{ peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values({})) }}; }}; const readTarget = (candidate: unknown): {} => {{ if (!isRecord(candidate)) {{ throw new TypeError('target must be an object'); }} return {{ deviceId: readString(candidate, 'deviceId'), platform: readString(candidate, 'platform'), route: readLiteral(candidate, 'route', Object.values({})) }}; }}; const readPayload = (candidate: unknown): {} => {{ if (!isRecord(candidate)) {{ throw new TypeError('payload must be an object'); }} for (const payloadValue of Object.values(candidate)) {{ if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') {{ throw new TypeError('payload values must be primitive protocol values'); }} }} return candidate as {}; }}; if (!isRecord(value)) {{ throw new TypeError('command envelope must be an object'); }} return {{ schemaVersion: readSchemaVersion(value), messageId: readString(value, 'messageId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readTarget(value['target']), command: readLiteral(value, 'command', Object.values({})), payload: readPayload(value['payload']) }}; }}",
        names.command_decoder_fn,
        names.command_envelope_type,
        names.runtime_const,
        names.peer_type,
        names.peer_role_const,
        names.target_type,
        names.route_const,
        names.payload_type,
        names.payload_type,
        names.command_const
    )
}

fn log_level_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "{} export type {} = (typeof {})[keyof typeof {}];",
        const_object_typescript(names.log_level_const, &log_level_descriptors()),
        names.log_level_type,
        names.log_level_const,
        names.log_level_const
    )
}

fn event_envelope_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export interface {} {{ readonly schemaVersion: number; readonly eventId: string; readonly correlationId: string; readonly sentAt: string; readonly source: {}; readonly target: {}; readonly event: {}; readonly severity: {}; readonly payload: {}; readonly snapshot: unknown | null; }} {}",
        names.event_envelope_type,
        names.peer_type,
        names.peer_type,
        names.event_type,
        names.log_level_type,
        names.payload_type,
        event_envelope_decoder_typescript(names)
    )
}

fn event_envelope_decoder_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export function {}(value: unknown): {} {{ const isRecord = (candidate: unknown): candidate is Readonly<Record<string, unknown>> => typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); const readString = (record: Readonly<Record<string, unknown>>, field: string): string => {{ const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) {{ throw new TypeError(`${{field}} must be a non-empty string`); }} return fieldValue; }}; const readNumber = (record: Readonly<Record<string, unknown>>, field: string): number => {{ const fieldValue = record[field]; if (typeof fieldValue !== 'number') {{ throw new TypeError(`${{field}} must be a number`); }} return fieldValue; }}; const readSchemaVersion = (record: Readonly<Record<string, unknown>>): number => {{ const schemaVersion = readNumber(record, 'schemaVersion'); if (schemaVersion !== {}.SchemaVersion) {{ throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); }} return schemaVersion; }}; const readLiteral = <T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T => {{ const fieldValue = readString(record, field); if (!allowed.includes(fieldValue as T)) {{ throw new TypeError(`${{field}} is not a Rust-owned protocol literal`); }} return fieldValue as T; }}; const readPeer = (candidate: unknown): {} => {{ if (!isRecord(candidate)) {{ throw new TypeError('peer must be an object'); }} return {{ peerId: readString(candidate, 'peerId'), role: readLiteral(candidate, 'role', Object.values({})) }}; }}; const readPayload = (candidate: unknown): {} => {{ if (!isRecord(candidate)) {{ throw new TypeError('payload must be an object'); }} for (const payloadValue of Object.values(candidate)) {{ if (payloadValue !== null && typeof payloadValue !== 'string' && typeof payloadValue !== 'number' && typeof payloadValue !== 'boolean') {{ throw new TypeError('payload values must be primitive protocol values'); }} }} return candidate as {}; }}; if (!isRecord(value)) {{ throw new TypeError('event envelope must be an object'); }} return {{ schemaVersion: readSchemaVersion(value), eventId: readString(value, 'eventId'), correlationId: readString(value, 'correlationId'), sentAt: readString(value, 'sentAt'), source: readPeer(value['source']), target: readPeer(value['target']), event: readLiteral(value, 'event', Object.values({})), severity: readLiteral(value, 'severity', Object.values({})), payload: readPayload(value['payload']), snapshot: value['snapshot'] ?? null }}; }}",
        names.event_decoder_fn,
        names.event_envelope_type,
        names.runtime_const,
        names.peer_type,
        names.peer_role_const,
        names.payload_type,
        names.payload_type,
        names.event_const,
        names.log_level_const
    )
}

fn primitive_decoders_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "function decodeNonEmptyProtocolString(value: unknown, label: string): string {{ if (typeof value !== 'string' || value.length === 0) {{ throw new TypeError(`${{label}} must be a non-empty Rust-owned protocol string`); }} return value; }} export function {}(value: unknown): string {{ return decodeNonEmptyProtocolString(value, 'messageId'); }} export function {}(value: unknown): string {{ return decodeNonEmptyProtocolString(value, 'timestamp'); }} export function {}(value: unknown): string {{ return decodeNonEmptyProtocolString(value, 'serializedMessage'); }} export function {}(value: unknown): value is string {{ return typeof value === 'string'; }}",
        names.message_id_decoder_fn,
        names.timestamp_decoder_fn,
        names.serialized_message_decoder_fn,
        names.log_text_guard_fn
    )
}

fn parent_route_event_id_typescript() -> String {
    "export function isParentRouteEventId(value: unknown): value is string { const suffix = ParentAgentProtocolDelimiter.EventIdSuffix; return typeof value === 'string' && Object.values(ParentAgentEvent).some((eventName) => value.startsWith(`${eventName}${suffix}`)) && value.length > value.indexOf(suffix) + suffix.length; }\nexport function decodeParentRouteEventId(value: unknown): string { if (!isParentRouteEventId(value)) { throw new TypeError('eventId must be a Rust-owned parent route event id'); } return value; }".to_string()
}

fn standalone_parent_route_event_id_typescript() -> String {
    parent_route_event_id_typescript()
}

fn command(
    key: &'static str,
    value: AgentCommandName,
) -> ProtocolLiteralDescriptor<AgentCommandName> {
    ProtocolLiteralDescriptor { key, value }
}

fn event(key: &'static str, value: AgentEventName) -> ProtocolLiteralDescriptor<AgentEventName> {
    ProtocolLiteralDescriptor { key, value }
}

fn field_descriptor(
    key: &'static str,
    value: &'static str,
) -> ProtocolLiteralDescriptor<&'static str> {
    ProtocolLiteralDescriptor { key, value }
}

fn json_literal<T: Serialize>(value: &T) -> String {
    schema_result_or_unreachable(
        serde_json::to_string(value),
        "protocol bridge literal serializes",
    )
}
