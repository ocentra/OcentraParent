fn lan_value_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "{} {} {} export const {} = {} as const; export type {} = (typeof {})[number]; export const {} = {}.LanHouseholdActionDeviceKind;",
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

fn peer_role_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentPeerRole>> {
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

fn route_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentRoute>> {
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

fn log_level_descriptors() -> Vec<ProtocolLiteralDescriptor<ocentra_parent_agent_protocol::LogLevel>>
{
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

fn peer_default_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentPeer>> {
    vec![ProtocolLiteralDescriptor {
        key: "PortalDev",
        value: AgentPeer {
            peer_id: peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
    }]
}

fn target_default_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentMessageTarget>> {
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

fn lan_household_action_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("Assign", lan_pairing::HOUSEHOLD_ACTION_ASSIGN),
        field_descriptor("Rename", lan_pairing::HOUSEHOLD_ACTION_RENAME),
        field_descriptor("Ignore", lan_pairing::HOUSEHOLD_ACTION_IGNORE),
        field_descriptor("Restore", lan_pairing::HOUSEHOLD_ACTION_RESTORE),
        field_descriptor("Trust", lan_pairing::HOUSEHOLD_ACTION_TRUST),
    ]
}

fn lan_intent_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![field_descriptor(
        "ConfigurationUpdate",
        value::LAN_INTENT_CONFIGURATION_UPDATE,
    )]
}

fn lan_parent_authority_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![field_descriptor(
        "ActiveController",
        value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
    )]
}

fn command_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentCommandName>> {
    let mut descriptors = Vec::new();
    descriptors.extend(command_descriptors_activity());
    descriptors.extend(command_descriptors_browser_network());
    descriptors.extend(command_descriptors_local_policy_lan());
    descriptors
}

fn command_descriptors_activity() -> Vec<ProtocolLiteralDescriptor<AgentCommandName>> {
    vec![
        command("HealthCheck", AgentCommandName::AgentHealthCheck),
        command("LogSnapshotGet", AgentCommandName::AgentLogSnapshotGet),
        command("DevEcho", AgentCommandName::AgentDevEcho),
        command("WatchStatusGet", AgentCommandName::AgentWatchStatusGet),
        command(
            "ActivityIngestStatusGet",
            AgentCommandName::AgentActivityIngestStatusGet,
        ),
        command(
            "ActivityRecentSummaryGet",
            AgentCommandName::AgentActivityRecentSummaryGet,
        ),
        command(
            "ActivityMemoryGraphGet",
            AgentCommandName::AgentActivityMemoryGraphGet,
        ),
        command(
            "ActivityReportDailyGenerate",
            AgentCommandName::AgentActivityReportDailyGenerate,
        ),
        command(
            "ActivityReportWeeklyGenerate",
            AgentCommandName::AgentActivityReportWeeklyGenerate,
        ),
        command(
            "ActivityReportMonthlyGenerate",
            AgentCommandName::AgentActivityReportMonthlyGenerate,
        ),
        command(
            "ActivityReportSave",
            AgentCommandName::AgentActivityReportSave,
        ),
        command(
            "ActivityReportHistoryList",
            AgentCommandName::AgentActivityReportHistoryList,
        ),
        command(
            "ActivityScreenReadModelGet",
            AgentCommandName::AgentActivityScreenReadModelGet,
        ),
        command(
            "ActivityAppUseReadModelGet",
            AgentCommandName::AgentActivityAppUseReadModelGet,
        ),
        command(
            "ActivityBrowserReadModelGet",
            AgentCommandName::AgentActivityBrowserReadModelGet,
        ),
        command(
            "ActivityGamesReadModelGet",
            AgentCommandName::AgentActivityGamesReadModelGet,
        ),
        command(
            "ActivityAppGameBoundaryReadModelGet",
            AgentCommandName::AgentActivityAppGameBoundaryReadModelGet,
        ),
        command(
            "ActivityAppGamePolicyReadinessReadModelGet",
            AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
        ),
    ]
}

fn command_descriptors_browser_network() -> Vec<ProtocolLiteralDescriptor<AgentCommandName>> {
    vec![
        command(
            "ActivityAppGameNotificationReadinessReadModelGet",
            AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet,
        ),
        command(
            "ActivityAppGameAdapterExecutionReadinessReadModelGet",
            AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet,
        ),
        command(
            "ActivityAppGamePlatformProofStatusReadModelGet",
            AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
        ),
        command(
            "ActivityAppGameChildRuntimeTransportReceiptReadModelGet",
            AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
        ),
        command(
            "ActivityAppGameAdapterDispatchPreflightReadModelGet",
            AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
        ),
        command(
            "ActivityAppGameAdapterDispatchResultReadModelGet",
            AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        ),
        command(
            "ActivityAppGameAdapterDispatchExecute",
            AgentCommandName::AgentActivityAppGameAdapterDispatchExecute,
        ),
        command(
            "ActivityAppGameTimerParentSurfaceReadModelGet",
            AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
        ),
        command(
            "ActivityAppGameTimerParentPreferenceSetupRequest",
            AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
        ),
        command(
            "BrowserSocialDashboardReadModelGet",
            AgentCommandName::AgentBrowserSocialDashboardReadModelGet,
        ),
        command(
            "BrowserSocialAuditExplanationReadModelGet",
            AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet,
        ),
        command(
            "BrowserSocialAlertReportReadModelGet",
            AgentCommandName::AgentBrowserSocialAlertReportReadModelGet,
        ),
        command(
            "BrowserSocialAlertReportParentSurfaceReadModelGet",
            AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet,
        ),
        command(
            "BrowserSocialParentNotificationDeliveryReadModelGet",
            AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet,
        ),
        command(
            "BrowserSocialSourceCustodyMutationApply",
            AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply,
        ),
        command(
            "ActivityNetworkReadModelGet",
            AgentCommandName::AgentActivityNetworkReadModelGet,
        ),
        command(
            "ActivityTrackingRetentionSettingsWrite",
            AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        ),
        command(
            "BrowserEvidenceRecentGet",
            AgentCommandName::AgentBrowserEvidenceRecentGet,
        ),
        command(
            "BrowserManagedBridgePoll",
            AgentCommandName::AgentBrowserManagedBridgePoll,
        ),
        command(
            "BrowserInventoryReadModelGet",
            AgentCommandName::AgentBrowserInventoryReadModelGet,
        ),
        command(
            "BrowserInterventionReadModelGet",
            AgentCommandName::AgentBrowserInterventionReadModelGet,
        ),
    ]
}

fn command_descriptors_local_policy_lan() -> Vec<ProtocolLiteralDescriptor<AgentCommandName>> {
    vec![
        command(
            "BrowserRuntimeEventChainStreamGet",
            AgentCommandName::AgentBrowserRuntimeEventChainStreamGet,
        ),
        command(
            "NetworkFlowReadModelGet",
            AgentCommandName::AgentNetworkFlowReadModelGet,
        ),
        command(
            "LanPairingStatusGet",
            AgentCommandName::AgentLanPairingStatusGet,
        ),
        command(
            "NetworkRuntimeEventChainStreamGet",
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        ),
        command(
            "LanRuntimeEventChainStreamGet",
            AgentCommandName::AgentLanRuntimeEventChainStreamGet,
        ),
        command(
            "NetworkRemoteDeliveryStatusGet",
            AgentCommandName::AgentNetworkRemoteDeliveryStatusGet,
        ),
        command(
            "NetworkLiveCaptureStatusGet",
            AgentCommandName::AgentNetworkLiveCaptureStatusGet,
        ),
        command(
            "NetworkLinuxNftablesLabStatusGet",
            AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet,
        ),
        command(
            "NetworkWindowsFirewallLabStatusGet",
            AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet,
        ),
        command(
            "NetworkWindowsWfpGateStatusGet",
            AgentCommandName::AgentNetworkWindowsWfpGateStatusGet,
        ),
        command(
            "NetworkAndroidVpnServiceGateStatusGet",
            AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet,
        ),
        command(
            "NetworkAppleNetworkExtensionGateStatusGet",
            AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet,
        ),
        command(
            "ActivityTrackingReadModelGet",
            AgentCommandName::AgentActivityTrackingReadModelGet,
        ),
        command(
            "LocalAiRuntimeStatusGet",
            AgentCommandName::AgentLocalAiRuntimeStatusGet,
        ),
        command(
            "LocalAiChatGenerate",
            AgentCommandName::AgentLocalAiChatGenerate,
        ),
        command(
            "ParentAssistantAnswerGenerate",
            AgentCommandName::AgentParentAssistantAnswerGenerate,
        ),
        command(
            "PolicyPreviewReadModelGet",
            AgentCommandName::AgentPolicyPreviewReadModelGet,
        ),
        command(
            "PolicyRequestAssistantPreviewConfirm",
            AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm,
        ),
        command("BrowserPolicyGet", AgentCommandName::AgentBrowserPolicyGet),
        command(
            "BrowserPolicyPreview",
            AgentCommandName::AgentBrowserPolicyPreview,
        ),
        command(
            "BrowserPolicyPatch",
            AgentCommandName::AgentBrowserPolicyPatch,
        ),
        command(
            "BrowserPolicyReplace",
            AgentCommandName::AgentBrowserPolicyReplace,
        ),
        command(
            "BrowserPolicyRollback",
            AgentCommandName::AgentBrowserPolicyRollback,
        ),
        command(
            "ScreenSettingsGet",
            AgentCommandName::AgentScreenSettingsGet,
        ),
        command(
            "ScreenSettingsReplace",
            AgentCommandName::AgentScreenSettingsReplace,
        ),
        command(
            "EnforcementExecute",
            AgentCommandName::AgentEnforcementExecute,
        ),
        command(
            "EnforcementTimerRecover",
            AgentCommandName::AgentEnforcementTimerRecover,
        ),
        command(
            "EnforcementTimerExpire",
            AgentCommandName::AgentEnforcementTimerExpire,
        ),
        command(
            "EnforcementOverrideCancel",
            AgentCommandName::AgentEnforcementOverrideCancel,
        ),
        command(
            "EnforcementProductControlSpineGet",
            AgentCommandName::AgentEnforcementProductControlSpineGet,
        ),
        command(
            "EnforcementPolicyDispatchGet",
            AgentCommandName::AgentEnforcementPolicyDispatchGet,
        ),
        command(
            "EnforcementBroadAdapterProofGet",
            AgentCommandName::AgentEnforcementBroadAdapterProofGet,
        ),
        command(
            "EnforcementSupportedAdapterRuntimeProofGet",
            AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet,
        ),
        command(
            "ParentAssistantThreadList",
            AgentCommandName::AgentParentAssistantThreadList,
        ),
        command(
            "ParentAssistantThreadCreate",
            AgentCommandName::AgentParentAssistantThreadCreate,
        ),
        command(
            "ParentAssistantThreadOpen",
            AgentCommandName::AgentParentAssistantThreadOpen,
        ),
        command(
            "ParentAssistantThreadArchive",
            AgentCommandName::AgentParentAssistantThreadArchive,
        ),
        command(
            "ParentAssistantMessageSend",
            AgentCommandName::AgentParentAssistantMessageSend,
        ),
        command(
            "ParentAssistantRunCancel",
            AgentCommandName::AgentParentAssistantRunCancel,
        ),
        command(
            "ParentAssistantQuickActionStart",
            AgentCommandName::AgentParentAssistantQuickActionStart,
        ),
        command(
            "ParentAssistantActionPreview",
            AgentCommandName::AgentParentAssistantActionPreview,
        ),
        command(
            "ParentAssistantActionConfirm",
            AgentCommandName::AgentParentAssistantActionConfirm,
        ),
        command(
            "ParentAssistantProviderStatusGet",
            AgentCommandName::AgentParentAssistantProviderStatusGet,
        ),
        command(
            "LanPairingProofSubmit",
            AgentCommandName::AgentLanPairingProofSubmit,
        ),
        command(
            "LanPairingRouteSelect",
            AgentCommandName::AgentLanPairingRouteSelect,
        ),
        command(
            "LanPairingRouteRevoke",
            AgentCommandName::AgentLanPairingRouteRevoke,
        ),
        command(
            "LanPairingBrowserDiscoveryScan",
            AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        ),
        command(
            "LanPairingAddDeviceRequest",
            AgentCommandName::AgentLanPairingAddDeviceRequest,
        ),
        command(
            "LanPairingSignedChildAgentObserve",
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
        ),
        command(
            "LanPairingControllerLeaseRenew",
            AgentCommandName::AgentLanPairingControllerLeaseRenew,
        ),
        command(
            "LanPairingControllerLeaseRelease",
            AgentCommandName::AgentLanPairingControllerLeaseRelease,
        ),
        command(
            "LanPairingControllerLeaseTakeover",
            AgentCommandName::AgentLanPairingControllerLeaseTakeover,
        ),
        command(
            "LanAiProviderStatusGet",
            AgentCommandName::AgentLanAiProviderStatusGet,
        ),
        command("LanAiJobSubmit", AgentCommandName::AgentLanAiJobSubmit),
    ]
}

fn event_descriptors() -> Vec<ProtocolLiteralDescriptor<AgentEventName>> {
    let mut descriptors = Vec::new();
    descriptors.extend(event_descriptors_activity());
    descriptors.extend(event_descriptors_app_game());
    descriptors.extend(event_descriptors_browser_network());
    descriptors.extend(event_descriptors_local_policy_lan());
    descriptors
}

fn event_descriptors_activity() -> Vec<ProtocolLiteralDescriptor<AgentEventName>> {
    vec![
        event("ConnectionReady", AgentEventName::AgentConnectionReady),
        event("CommandRejected", AgentEventName::AgentCommandRejected),
        event("HealthReported", AgentEventName::AgentHealthReported),
        event(
            "LogSnapshotReported",
            AgentEventName::AgentLogSnapshotReported,
        ),
        event("DevEchoed", AgentEventName::AgentDevEchoed),
        event(
            "WatchStatusReported",
            AgentEventName::AgentWatchStatusReported,
        ),
        event(
            "ActivityIngestStatusReported",
            AgentEventName::AgentActivityIngestStatusReported,
        ),
        event(
            "ActivityRecentSummaryReported",
            AgentEventName::AgentActivityRecentSummaryReported,
        ),
        event(
            "ActivityMemoryGraphReported",
            AgentEventName::AgentActivityMemoryGraphReported,
        ),
        event(
            "ActivityReportGenerated",
            AgentEventName::AgentActivityReportGenerated,
        ),
        event(
            "ActivityReportSaved",
            AgentEventName::AgentActivityReportSaved,
        ),
        event(
            "ActivityReportHistoryReported",
            AgentEventName::AgentActivityReportHistoryReported,
        ),
        event(
            "ActivityScreenReadModelReported",
            AgentEventName::AgentActivityScreenReadModelReported,
        ),
        event(
            "ActivityAppUseReadModelReported",
            AgentEventName::AgentActivityAppUseReadModelReported,
        ),
        event(
            "ActivityBrowserReadModelReported",
            AgentEventName::AgentActivityBrowserReadModelReported,
        ),
        event(
            "ActivityGamesReadModelReported",
            AgentEventName::AgentActivityGamesReadModelReported,
        ),
        event(
            "ActivityAppGameBoundaryReadModelReported",
            AgentEventName::AgentActivityAppGameBoundaryReadModelReported,
        ),
    ]
}

fn event_descriptors_app_game() -> Vec<ProtocolLiteralDescriptor<AgentEventName>> {
    vec![
        event(
            "ActivityAppGameNotificationReadinessReadModelReported",
            AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
        ),
        event(
            "ActivityAppGameAdapterExecutionReadinessReadModelReported",
            AgentEventName::AgentActivityAppGameAdapterExecutionReadinessReadModelReported,
        ),
        event(
            "ActivityAppGamePlatformProofStatusReadModelReported",
            AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported,
        ),
        event(
            "ActivityAppGameChildRuntimeTransportReceiptReadModelReported",
            AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported,
        ),
        event(
            "ActivityAppGameAdapterDispatchPreflightReadModelReported",
            AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported,
        ),
        event(
            "ActivityAppGameAdapterDispatchResultReadModelReported",
            AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported,
        ),
        event(
            "ActivityAppGameAdapterDispatchExecuted",
            AgentEventName::AgentActivityAppGameAdapterDispatchExecuted,
        ),
        event(
            "ActivityAppGameTimerParentSurfaceReadModelReported",
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        ),
        event(
            "ActivityAppGameTimerParentPreferenceSetupRequested",
            AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
        ),
    ]
}

fn event_descriptors_browser_network() -> Vec<ProtocolLiteralDescriptor<AgentEventName>> {
    vec![
        event(
            "BrowserSocialDashboardReadModelReported",
            AgentEventName::AgentBrowserSocialDashboardReadModelReported,
        ),
        event(
            "BrowserSocialAuditExplanationReadModelReported",
            AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported,
        ),
        event(
            "BrowserSocialAlertReportReadModelReported",
            AgentEventName::AgentBrowserSocialAlertReportReadModelReported,
        ),
        event(
            "BrowserSocialAlertReportParentSurfaceReadModelReported",
            AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
        ),
        event(
            "BrowserSocialParentNotificationDeliveryReadModelReported",
            AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported,
        ),
        event(
            "BrowserSocialSourceCustodyMutationApplied",
            AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied,
        ),
        event(
            "ActivityNetworkReadModelReported",
            AgentEventName::AgentActivityNetworkReadModelReported,
        ),
        event(
            "BrowserEvidenceRecentReported",
            AgentEventName::AgentBrowserEvidenceRecentReported,
        ),
        event(
            "BrowserManagedStatusReported",
            AgentEventName::AgentBrowserManagedStatusReported,
        ),
        event(
            "BrowserInventoryReadModelReported",
            AgentEventName::AgentBrowserInventoryReadModelReported,
        ),
        event(
            "BrowserInterventionReadModelReported",
            AgentEventName::AgentBrowserInterventionReadModelReported,
        ),
        event(
            "BrowserRuntimeEventChainStreamReported",
            AgentEventName::AgentBrowserRuntimeEventChainStreamReported,
        ),
        event(
            "NetworkFlowReadModelReported",
            AgentEventName::AgentNetworkFlowReadModelReported,
        ),
        event(
            "NetworkRuntimeEventChainStreamReported",
            AgentEventName::AgentNetworkRuntimeEventChainStreamReported,
        ),
        event(
            "LanRuntimeEventChainStreamReported",
            AgentEventName::AgentLanRuntimeEventChainStreamReported,
        ),
        event(
            "NetworkRemoteDeliveryStatusReported",
            AgentEventName::AgentNetworkRemoteDeliveryStatusReported,
        ),
        event(
            "NetworkLiveCaptureStatusReported",
            AgentEventName::AgentNetworkLiveCaptureStatusReported,
        ),
    ]
}

fn event_descriptors_local_policy_lan() -> Vec<ProtocolLiteralDescriptor<AgentEventName>> {
    vec![
        event(
            "NetworkLinuxNftablesLabStatusReported",
            AgentEventName::AgentNetworkLinuxNftablesLabStatusReported,
        ),
        event(
            "NetworkWindowsFirewallLabStatusReported",
            AgentEventName::AgentNetworkWindowsFirewallLabStatusReported,
        ),
        event(
            "NetworkWindowsWfpGateStatusReported",
            AgentEventName::AgentNetworkWindowsWfpGateStatusReported,
        ),
        event(
            "NetworkAndroidVpnServiceGateStatusReported",
            AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported,
        ),
        event(
            "NetworkAppleNetworkExtensionGateStatusReported",
            AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported,
        ),
        event(
            "ActivityTrackingReadModelReported",
            AgentEventName::AgentActivityTrackingReadModelReported,
        ),
        event(
            "ActivityTrackingRetentionSettingsWriteReported",
            AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
        ),
        event(
            "LocalAiRuntimeStatusReported",
            AgentEventName::AgentLocalAiRuntimeStatusReported,
        ),
        event(
            "LocalAiChatGenerationReported",
            AgentEventName::AgentLocalAiChatGenerationReported,
        ),
        event(
            "PolicyPreviewReadModelReported",
            AgentEventName::AgentPolicyPreviewReadModelReported,
        ),
        event(
            "PolicyRequestAssistantPreviewConfirmReported",
            AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported,
        ),
        event(
            "BrowserPolicyReported",
            AgentEventName::AgentBrowserPolicyReported,
        ),
        event(
            "BrowserPolicyPreviewed",
            AgentEventName::AgentBrowserPolicyPreviewed,
        ),
        event(
            "BrowserPolicyPatchAccepted",
            AgentEventName::AgentBrowserPolicyPatchAccepted,
        ),
        event(
            "BrowserPolicyPatchRejected",
            AgentEventName::AgentBrowserPolicyPatchRejected,
        ),
        event(
            "BrowserPolicyReplaceAccepted",
            AgentEventName::AgentBrowserPolicyReplaceAccepted,
        ),
        event(
            "BrowserPolicyReplaceRejected",
            AgentEventName::AgentBrowserPolicyReplaceRejected,
        ),
        event(
            "BrowserPolicyRollbackAccepted",
            AgentEventName::AgentBrowserPolicyRollbackAccepted,
        ),
        event(
            "BrowserPolicyRollbackRejected",
            AgentEventName::AgentBrowserPolicyRollbackRejected,
        ),
        event(
            "ScreenSettingsReported",
            AgentEventName::AgentScreenSettingsReported,
        ),
        event(
            "ScreenSettingsReplaceAccepted",
            AgentEventName::AgentScreenSettingsReplaceAccepted,
        ),
        event(
            "ScreenSettingsReplaceRejected",
            AgentEventName::AgentScreenSettingsReplaceRejected,
        ),
        event(
            "EnforcementAuditReported",
            AgentEventName::AgentEnforcementAuditReported,
        ),
        event(
            "EnforcementTimerReported",
            AgentEventName::AgentEnforcementTimerReported,
        ),
        event(
            "EnforcementProductControlSpineReported",
            AgentEventName::AgentEnforcementProductControlSpineReported,
        ),
        event(
            "EnforcementPolicyDispatchReported",
            AgentEventName::AgentEnforcementPolicyDispatchReported,
        ),
        event(
            "EnforcementBroadAdapterProofReported",
            AgentEventName::AgentEnforcementBroadAdapterProofReported,
        ),
        event(
            "EnforcementSupportedAdapterRuntimeProofReported",
            AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported,
        ),
        event(
            "ActivityAppGamePolicyReadinessReadModelReported",
            AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
        ),
        event(
            "ParentAssistantAnswerReported",
            AgentEventName::AgentParentAssistantAnswerReported,
        ),
        event(
            "ParentAssistantThreadUpdated",
            AgentEventName::AgentParentAssistantThreadUpdated,
        ),
        event(
            "ParentAssistantMessageAccepted",
            AgentEventName::AgentParentAssistantMessageAccepted,
        ),
        event(
            "ParentAssistantRunStarted",
            AgentEventName::AgentParentAssistantRunStarted,
        ),
        event(
            "ParentAssistantMessageDelta",
            AgentEventName::AgentParentAssistantMessageDelta,
        ),
        event(
            "ParentAssistantMessageCompleted",
            AgentEventName::AgentParentAssistantMessageCompleted,
        ),
        event(
            "ParentAssistantActionPreviewed",
            AgentEventName::AgentParentAssistantActionPreviewed,
        ),
        event(
            "ParentAssistantActionConfirmed",
            AgentEventName::AgentParentAssistantActionConfirmed,
        ),
        event(
            "ParentAssistantProviderDegraded",
            AgentEventName::AgentParentAssistantProviderDegraded,
        ),
        event(
            "ParentAssistantErrorReported",
            AgentEventName::AgentParentAssistantErrorReported,
        ),
        event(
            "LanPairingStatusReported",
            AgentEventName::AgentLanPairingStatusReported,
        ),
        event(
            "LanPairingBrowserDiscoveryReported",
            AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        ),
        event(
            "LanPairingAddDeviceReported",
            AgentEventName::AgentLanPairingAddDeviceReported,
        ),
        event(
            "LanPairingSignedChildAgentReported",
            AgentEventName::AgentLanPairingSignedChildAgentReported,
        ),
        event(
            "LanPairingAuditReported",
            AgentEventName::AgentLanPairingAuditReported,
        ),
        event("LanAiJobReported", AgentEventName::AgentLanAiJobReported),
    ]
}

fn network_remote_delivery_status_ref_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>>
{
    vec![
        field_descriptor(
            "StatusRef",
            network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF,
        ),
        field_descriptor(
            "EventChainJournalRef",
            network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF,
        ),
        field_descriptor(
            "ReceiptLedgerRef",
            network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF,
        ),
        field_descriptor(
            "LocalReceiptAckRef",
            network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF,
        ),
        field_descriptor(
            "DurableEnvelopeRef",
            network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF,
        ),
        field_descriptor(
            "DurableStoreRef",
            network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF,
        ),
        field_descriptor(
            "DurableReplayRef",
            network_flow::TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF,
        ),
        field_descriptor(
            "DurableDeleteExportRef",
            network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF,
        ),
        field_descriptor(
            "DurableSupportStatusRef",
            network_flow::TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF,
        ),
        field_descriptor("OutboxRef", network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF),
        field_descriptor(
            "OutboxHandoffRef",
            network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF,
        ),
        field_descriptor(
            "OutboxReplayRef",
            network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF,
        ),
        field_descriptor(
            "OutboxSupportStatusRef",
            network_flow::TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF,
        ),
        field_descriptor(
            "TransportDispatchStateRef",
            network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF,
        ),
        field_descriptor(
            "BlockedDispatchRef",
            network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF,
        ),
        field_descriptor(
            "FutureTransportSeamRef",
            network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF,
        ),
        field_descriptor(
            "FixtureTransportRef",
            network_flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF,
        ),
        field_descriptor(
            "FixtureDispatchAttemptRef",
            network_flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF,
        ),
        field_descriptor(
            "FixtureAckRef",
            network_flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF,
        ),
        field_descriptor(
            "DeleteExportPropagationRef",
            network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF,
        ),
        field_descriptor(
            "RemoteDeleteReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF,
        ),
        field_descriptor(
            "RemoteExportReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF,
        ),
        field_descriptor(
            "ProviderRouteRef",
            network_flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF,
        ),
        field_descriptor(
            "ChildDeviceRouteRef",
            network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF,
        ),
        field_descriptor(
            "ProviderDeliveryReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF,
        ),
        field_descriptor(
            "ChildDeviceDeliveryReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF,
        ),
        field_descriptor(
            "CrossProcessCustodyStatusRef",
            network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF,
        ),
        field_descriptor(
            "CrossProcessReplayReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF,
        ),
        field_descriptor(
            "RemoteRetentionReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF,
        ),
        field_descriptor(
            "RemoteDeleteCustodyReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF,
        ),
        field_descriptor(
            "RemoteExportCustodyReadinessRef",
            network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF,
        ),
        field_descriptor(
            "CrossProcessReplayRef",
            network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF,
        ),
        field_descriptor(
            "CrossProcessReplayStoreRef",
            network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF,
        ),
        field_descriptor(
            "CrossProcessReplayCursorRef",
            network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF,
        ),
        field_descriptor(
            "ExternalCrossProcessTransportRef",
            network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF,
        ),
        field_descriptor(
            "ExternalCrossProcessTransportEnvelopeRef",
            network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF,
        ),
        field_descriptor(
            "ExternalCrossProcessTransportAckRef",
            network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF,
        ),
    ]
}

fn network_live_capture_status_ref_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("StatusRef", network_flow::TEST_LIVE_CAPTURE_STATUS_REF),
        field_descriptor(
            "Row13StatusRef",
            network_flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF,
        ),
        field_descriptor(
            "ExecutionStatusRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF,
        ),
        field_descriptor(
            "RawStorageStatusRef",
            network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF,
        ),
        field_descriptor(
            "WindowsProofRef",
            network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF,
        ),
        field_descriptor(
            "ManualProofRef",
            network_flow::TEST_LIVE_CAPTURE_MANUAL_PROOF_REF,
        ),
        field_descriptor(
            "LinuxProofRef",
            network_flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF,
        ),
        field_descriptor(
            "MacosProofRef",
            network_flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF,
        ),
        field_descriptor(
            "InterfaceRef",
            network_flow::TEST_LIVE_CAPTURE_INTERFACE_REF,
        ),
        field_descriptor("DriverRef", network_flow::TEST_LIVE_CAPTURE_DRIVER_REF),
        field_descriptor(
            "PermissionRef",
            network_flow::TEST_LIVE_CAPTURE_PERMISSION_REF,
        ),
        field_descriptor(
            "BoundedCaptureRef",
            network_flow::TEST_LIVE_CAPTURE_BOUNDED_REF,
        ),
        field_descriptor(
            "CleanStopRef",
            network_flow::TEST_LIVE_CAPTURE_CLEAN_STOP_REF,
        ),
        field_descriptor("QuotaRef", network_flow::TEST_LIVE_CAPTURE_QUOTA_REF),
        field_descriptor(
            "RetentionRef",
            network_flow::TEST_LIVE_CAPTURE_RETENTION_REF,
        ),
        field_descriptor("CustodyRef", network_flow::TEST_LIVE_CAPTURE_CUSTODY_REF),
        field_descriptor(
            "PrivateTrafficExclusionRef",
            network_flow::TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF,
        ),
        field_descriptor(
            "WindowsExecutionRef",
            network_flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF,
        ),
        field_descriptor(
            "ManualExecutionRef",
            network_flow::TEST_LIVE_CAPTURE_MANUAL_EXECUTION_REF,
        ),
        field_descriptor(
            "LinuxExecutionRef",
            network_flow::TEST_LIVE_CAPTURE_LINUX_EXECUTION_REF,
        ),
        field_descriptor(
            "MacosExecutionRef",
            network_flow::TEST_LIVE_CAPTURE_MACOS_EXECUTION_REF,
        ),
        field_descriptor(
            "DriverInvocationRef",
            network_flow::TEST_LIVE_CAPTURE_DRIVER_INVOCATION_REF,
        ),
        field_descriptor(
            "InterfaceObservationRef",
            network_flow::TEST_LIVE_CAPTURE_INTERFACE_OBSERVATION_REF,
        ),
        field_descriptor(
            "ExecutionPermissionRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_PERMISSION_REF,
        ),
        field_descriptor(
            "BoundedWindowRef",
            network_flow::TEST_LIVE_CAPTURE_BOUNDED_WINDOW_REF,
        ),
        field_descriptor(
            "ExecutionCleanStopRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_CLEAN_STOP_REF,
        ),
        field_descriptor(
            "ExecutionCustodyRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_CUSTODY_REF,
        ),
        field_descriptor(
            "ExecutionRetentionRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_RETENTION_REF,
        ),
        field_descriptor(
            "MetadataSanitizationRef",
            network_flow::TEST_LIVE_CAPTURE_METADATA_SANITIZATION_REF,
        ),
        field_descriptor(
            "ExecutionPrivateTrafficExclusionRef",
            network_flow::TEST_LIVE_CAPTURE_EXECUTION_PRIVATE_TRAFFIC_EXCLUSION_REF,
        ),
        field_descriptor(
            "RawManifestRef",
            network_flow::TEST_RAW_CAPTURE_MANIFEST_REF,
        ),
        field_descriptor(
            "RawStorageLocationRef",
            network_flow::TEST_RAW_CAPTURE_STORAGE_LOCATION_REF,
        ),
        field_descriptor(
            "RawEncryptionRef",
            network_flow::TEST_RAW_CAPTURE_ENCRYPTION_REF,
        ),
        field_descriptor("RawQuotaRef", network_flow::TEST_RAW_CAPTURE_QUOTA_REF),
        field_descriptor(
            "RawRetentionRef",
            network_flow::TEST_RAW_CAPTURE_RETENTION_REF,
        ),
        field_descriptor(
            "RawDeleteExportRef",
            network_flow::TEST_RAW_CAPTURE_DELETE_EXPORT_REF,
        ),
        field_descriptor(
            "RawCustodyChainRef",
            network_flow::TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF,
        ),
        field_descriptor(
            "RawPrivateTrafficExclusionRef",
            network_flow::TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF,
        ),
    ]
}

fn network_linux_nftables_lab_status_ref_descriptors(
) -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "StatusRef",
            network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF,
        ),
        field_descriptor("LabRef", network_flow::TEST_LINUX_NFTABLES_LAB_REF),
        field_descriptor(
            "LinuxAdapterGateRef",
            network_flow::TEST_LINUX_ADAPTER_GATE_REF,
        ),
        field_descriptor(
            "PolicyDecisionRef",
            network_flow::TEST_LINUX_ADAPTER_POLICY_DECISION_REF,
        ),
        field_descriptor(
            "ParentRuleRef",
            network_flow::TEST_LINUX_ADAPTER_PARENT_RULE_REF,
        ),
        field_descriptor("EvidenceRef", network_flow::TEST_LINUX_ADAPTER_EVIDENCE_REF),
        field_descriptor("DistroRef", network_flow::TEST_LINUX_DISTRO_REF),
        field_descriptor("KernelRef", network_flow::TEST_LINUX_KERNEL_REF),
        field_descriptor("TableName", network_flow::TEST_LINUX_NFTABLES_TABLE_NAME),
        field_descriptor("ChainName", network_flow::TEST_LINUX_NFTABLES_CHAIN_NAME),
        field_descriptor(
            "TargetRemoteAddress",
            network_flow::TEST_LINUX_NFTABLES_TARGET_REMOTE_ADDRESS,
        ),
        field_descriptor(
            "CreateTableCommandRef",
            network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF,
        ),
        field_descriptor(
            "CreateChainCommandRef",
            network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_COMMAND_REF,
        ),
        field_descriptor(
            "AddRuleCommandRef",
            network_flow::TEST_LINUX_NFTABLES_ADD_RULE_COMMAND_REF,
        ),
        field_descriptor(
            "VerifyRuleCommandRef",
            network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_COMMAND_REF,
        ),
        field_descriptor(
            "DeleteTableCommandRef",
            network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_COMMAND_REF,
        ),
        field_descriptor(
            "VerifyRemovedCommandRef",
            network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_COMMAND_REF,
        ),
        field_descriptor(
            "CreateTableOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "CreateChainOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_OUTPUT_SHA256,
        ),
        field_descriptor(
            "AddRuleOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_ADD_RULE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "VerifyRuleOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "DeleteTableOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "VerifyRemovedOutputSha256",
            network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_OUTPUT_SHA256,
        ),
    ]
}

fn network_windows_firewall_lab_status_ref_descriptors(
) -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "StatusRef",
            network_flow::TEST_WINDOWS_FIREWALL_LAB_STATUS_REF,
        ),
        field_descriptor("LabRef", network_flow::TEST_WINDOWS_FIREWALL_LAB_REF),
        field_descriptor(
            "FirewallAdapterPlanRef",
            network_flow::TEST_WINDOWS_FIREWALL_ADAPTER_PLAN_REF,
        ),
        field_descriptor(
            "PolicyDecisionRef",
            network_flow::TEST_WINDOWS_FIREWALL_POLICY_DECISION_REF,
        ),
        field_descriptor(
            "ParentRuleRef",
            network_flow::TEST_WINDOWS_FIREWALL_PARENT_RULE_REF,
        ),
        field_descriptor(
            "EvidenceRef",
            network_flow::TEST_WINDOWS_FIREWALL_EVIDENCE_REF,
        ),
        field_descriptor(
            "WindowsOsScopeRef",
            network_flow::TEST_WINDOWS_FIREWALL_OS_SCOPE_REF,
        ),
        field_descriptor("TargetRef", network_flow::TEST_WINDOWS_FIREWALL_TARGET_REF),
        field_descriptor(
            "FirewallRuleRef",
            network_flow::TEST_WINDOWS_FIREWALL_RULE_REF,
        ),
        field_descriptor("RuleName", network_flow::TEST_WINDOWS_FIREWALL_RULE_NAME),
        field_descriptor(
            "TargetRemoteAddress",
            network_flow::TEST_WINDOWS_FIREWALL_TARGET_REMOTE_ADDRESS,
        ),
        field_descriptor(
            "ApplyRuleCommandRef",
            network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_COMMAND_REF,
        ),
        field_descriptor(
            "VerifyPresentCommandRef",
            network_flow::TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_COMMAND_REF,
        ),
        field_descriptor(
            "RollbackRuleCommandRef",
            network_flow::TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_COMMAND_REF,
        ),
        field_descriptor(
            "VerifyRemovedCommandRef",
            network_flow::TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_COMMAND_REF,
        ),
        field_descriptor(
            "ApplyRuleOutputSha256",
            network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "VerifyPresentOutputSha256",
            network_flow::TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_OUTPUT_SHA256,
        ),
        field_descriptor(
            "RollbackRuleOutputSha256",
            network_flow::TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_OUTPUT_SHA256,
        ),
        field_descriptor(
            "VerifyRemovedOutputSha256",
            network_flow::TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_OUTPUT_SHA256,
        ),
    ]
}

fn network_windows_wfp_gate_status_ref_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>>
{
    vec![
        field_descriptor("StatusRef", network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF),
        field_descriptor("WfpGateRef", network_flow::TEST_WINDOWS_WFP_GATE_REF),
        field_descriptor(
            "PolicyDecisionRef",
            network_flow::TEST_WINDOWS_WFP_POLICY_DECISION_REF,
        ),
        field_descriptor(
            "ParentRuleRef",
            network_flow::TEST_WINDOWS_WFP_PARENT_RULE_REF,
        ),
        field_descriptor("EvidenceRef", network_flow::TEST_WINDOWS_WFP_EVIDENCE_REF),
        field_descriptor(
            "LocalAiResultRef",
            network_flow::TEST_WINDOWS_WFP_LOCAL_AI_RESULT_REF,
        ),
        field_descriptor("TargetRef", network_flow::TEST_WINDOWS_WFP_TARGET_REF),
        field_descriptor(
            "WfpProviderRef",
            network_flow::TEST_WINDOWS_WFP_PROVIDER_REF,
        ),
        field_descriptor("WfpLayerRef", network_flow::TEST_WINDOWS_WFP_LAYER_REF),
        field_descriptor(
            "AdministratorPermissionProofRef",
            network_flow::TEST_WINDOWS_WFP_ADMIN_PERMISSION_PROOF_REF,
        ),
        field_descriptor(
            "DriverSigningProofRef",
            network_flow::TEST_WINDOWS_WFP_DRIVER_SIGNING_PROOF_REF,
        ),
        field_descriptor(
            "DriverPackageProofRef",
            network_flow::TEST_WINDOWS_WFP_DRIVER_PACKAGE_PROOF_REF,
        ),
        field_descriptor(
            "ProviderRegistrationPlanRef",
            network_flow::TEST_WINDOWS_WFP_PROVIDER_REGISTRATION_PLAN_REF,
        ),
        field_descriptor(
            "LayerCapabilityMatrixRef",
            network_flow::TEST_WINDOWS_WFP_LAYER_CAPABILITY_MATRIX_REF,
        ),
        field_descriptor(
            "RollbackPlanRef",
            network_flow::TEST_WINDOWS_WFP_ROLLBACK_PLAN_REF,
        ),
        field_descriptor(
            "LabResultArtifactRef",
            network_flow::TEST_WINDOWS_WFP_LAB_RESULT_ARTIFACT_REF,
        ),
        field_descriptor(
            "AuditEventRef",
            network_flow::TEST_WINDOWS_WFP_AUDIT_EVENT_REF,
        ),
    ]
}

fn network_android_vpn_service_gate_status_ref_descriptors(
) -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "StatusRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF,
        ),
        field_descriptor(
            "AndroidVpnServiceGateRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF,
        ),
        field_descriptor(
            "PolicyDecisionRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_POLICY_DECISION_REF,
        ),
        field_descriptor(
            "ParentRuleRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_PARENT_RULE_REF,
        ),
        field_descriptor(
            "EvidenceRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_EVIDENCE_REF,
        ),
        field_descriptor(
            "LocalAiResultRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_LOCAL_AI_RESULT_REF,
        ),
        field_descriptor(
            "PackageRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_PACKAGE_REF,
        ),
        field_descriptor("VpnServiceRef", network_flow::TEST_ANDROID_VPN_SERVICE_REF),
        field_descriptor(
            "VpnServiceDeclarationRef",
            network_flow::TEST_ANDROID_VPN_SERVICE_DECLARATION_REF,
        ),
        field_descriptor(
            "UserConsentProofRef",
            network_flow::TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF,
        ),
        field_descriptor(
            "PhysicalDeviceProofRef",
            network_flow::TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF,
        ),
        field_descriptor(
            "PackageIdentityProofRef",
            network_flow::TEST_ANDROID_VPN_PACKAGE_IDENTITY_PROOF_REF,
        ),
        field_descriptor(
            "VirtualInterfaceProofRef",
            network_flow::TEST_ANDROID_VPN_VIRTUAL_INTERFACE_PROOF_REF,
        ),
        field_descriptor(
            "TrafficObservationProofRef",
            network_flow::TEST_ANDROID_VPN_TRAFFIC_OBSERVATION_PROOF_REF,
        ),
        field_descriptor(
            "RollbackPlanRef",
            network_flow::TEST_ANDROID_VPN_ROLLBACK_PLAN_REF,
        ),
        field_descriptor(
            "AuditEventRef",
            network_flow::TEST_ANDROID_VPN_AUDIT_EVENT_REF,
        ),
        field_descriptor(
            "DeviceOwnerProofRef",
            network_flow::TEST_ANDROID_VPN_DEVICE_OWNER_PROOF_REF,
        ),
    ]
}

fn network_apple_network_extension_gate_status_ref_descriptors(
) -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "StatusRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF,
        ),
        field_descriptor(
            "AppleNetworkExtensionGateRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF,
        ),
        field_descriptor(
            "PolicyDecisionRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_POLICY_DECISION_REF,
        ),
        field_descriptor(
            "ParentRuleRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_PARENT_RULE_REF,
        ),
        field_descriptor(
            "EvidenceRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_EVIDENCE_REF,
        ),
        field_descriptor(
            "LocalAiResultRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_LOCAL_AI_RESULT_REF,
        ),
        field_descriptor(
            "BundleRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_BUNDLE_REF,
        ),
        field_descriptor(
            "NetworkExtensionRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_REF,
        ),
        field_descriptor(
            "DeveloperTeamProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF,
        ),
        field_descriptor(
            "EntitlementApprovalProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF,
        ),
        field_descriptor(
            "ProvisioningProfileProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_PROVISIONING_PROFILE_PROOF_REF,
        ),
        field_descriptor(
            "SigningProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_SIGNING_PROOF_REF,
        ),
        field_descriptor(
            "DeviceOrTestFlightProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVICE_OR_TESTFLIGHT_PROOF_REF,
        ),
        field_descriptor(
            "NetworkExtensionDeclarationRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF,
        ),
        field_descriptor(
            "ExtensionConfigurationProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_CONFIGURATION_PROOF_REF,
        ),
        field_descriptor(
            "RollbackPlanRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF,
        ),
        field_descriptor(
            "AuditEventRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_AUDIT_EVENT_REF,
        ),
        field_descriptor(
            "SupervisionOrMdmProofRef",
            network_flow::TEST_APPLE_NETWORK_EXTENSION_SUPERVISION_OR_MDM_PROOF_REF,
        ),
    ]
}

fn field_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("ActivityDigest", field::ACTIVITY_DIGEST),
        field_descriptor("ActivityFamilySources", field::ACTIVITY_FAMILY_SOURCES),
        field_descriptor("ActivityReadModel", field::ACTIVITY_READ_MODEL),
        field_descriptor("ActivityReadModelKind", field::ACTIVITY_READ_MODEL_KIND),
        field_descriptor("ActivityReportDocument", field::ACTIVITY_REPORT_DOCUMENT),
        field_descriptor("ActivityReportFrequency", field::ACTIVITY_REPORT_FREQUENCY),
        field_descriptor("ActivityReportId", field::ACTIVITY_REPORT_ID),
        field_descriptor("ActivityReports", field::ACTIVITY_REPORTS),
        field_descriptor("ActivitySurfaceState", field::ACTIVITY_SURFACE_STATE),
        field_descriptor(
            "ActivityTrackingRetentionSettingsWriteResult",
            field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
        ),
        field_descriptor("ClaimBoundary", field::CLAIM_BOUNDARY),
        field_descriptor("DeviceId", field::DEVICE_ID),
        field_descriptor("EventRef", field::EVENT_REF),
        field_descriptor("EventType", field::EVENT_TYPE),
        field_descriptor("FamilyId", field::FAMILY_ID),
        field_descriptor("Origin", field::ORIGIN),
        field_descriptor("Payload", field::PAYLOAD),
        field_descriptor("StartedAt", field::STARTED_AT),
        field_descriptor("StaleAt", field::STALE_AT),
        field_descriptor(
            "BrowserSocialAlertReportReadModel",
            field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialAlertReportParentSurfaceReadModel",
            field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialDashboardReadModel",
            field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialParentNotificationDeliveryReadModel",
            field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentAdapterExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_ADAPTER_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentCandidates",
            field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildAcceptedEventRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildAcceptedRows",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildCommandRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildInterventionExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentDispatchAttempts",
            field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentEnforcementExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffCandidates",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffOutboxRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentParentReadModelRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeEventChainStream",
            field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM,
        ),
        field_descriptor(
            "BrowserRuntimeExactUrlRows",
            field::BROWSER_RUNTIME_EXACT_URL_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeFailedRows",
            field::BROWSER_RUNTIME_FAILED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeInterventionCommandEvents",
            field::BROWSER_RUNTIME_INTERVENTION_COMMAND_EVENTS,
        ),
        field_descriptor(
            "BrowserRuntimeManualRequiredRows",
            field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeObservedRows",
            field::BROWSER_RUNTIME_OBSERVED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeReadModelProjectionEvents",
            field::BROWSER_RUNTIME_READ_MODEL_PROJECTION_EVENTS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderAttemptRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDispatchRequiredRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DISPATCH_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableResultRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_RESULT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableStoreRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_STORE_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderManualReceiptRequiredRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_MANUAL_RECEIPT_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReadModelRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_READ_MODEL_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReceiptBoundaryRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_BOUNDARY_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReceiptProofRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderSupportStatusRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_SUPPORT_STATUS_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeStreamedEvents",
            field::BROWSER_RUNTIME_STREAMED_EVENTS,
        ),
        field_descriptor("LanAiJobId", field::LAN_AI_JOB_ID),
        field_descriptor("LanAiJobState", field::LAN_AI_JOB_STATE),
        field_descriptor("LanAiJobStatus", field::LAN_AI_JOB_STATUS),
        field_descriptor(
            "LanAiProviderCustodyLabel",
            field::LAN_AI_PROVIDER_CUSTODY_LABEL,
        ),
        field_descriptor(
            "LanAiProviderRoutingState",
            field::LAN_AI_PROVIDER_ROUTING_STATE,
        ),
        field_descriptor(
            "LanControllerLeaseExpiresAt",
            field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
        ),
        field_descriptor("LanControllerLeaseId", field::LAN_CONTROLLER_LEASE_ID),
        field_descriptor(
            "LanControllerLeaseIssuedAt",
            field::LAN_CONTROLLER_LEASE_ISSUED_AT,
        ),
        field_descriptor("LanCanonicalDeviceId", field::LAN_CANONICAL_DEVICE_ID),
        field_descriptor("LanChildDeviceId", field::LAN_CHILD_DEVICE_ID),
        field_descriptor("LanControllerDeviceId", field::LAN_CONTROLLER_DEVICE_ID),
        field_descriptor(
            "LanHouseholdActionId",
            lan_pairing::HOUSEHOLD_ACTION_ID_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionKind",
            lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionChildProfileId",
            lan_pairing::HOUSEHOLD_ACTION_CHILD_PROFILE_ID_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionDisplayName",
            lan_pairing::HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionDeviceKind",
            lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionRevokedAt",
            lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD,
        ),
        field_descriptor("LanIntentId", field::LAN_INTENT_ID),
        field_descriptor("LanIntentKind", field::LAN_INTENT_KIND),
        field_descriptor("LanPairingId", field::LAN_PAIRING_ID),
        field_descriptor("LanParentAuthority", field::LAN_PARENT_AUTHORITY),
        field_descriptor("LanParentActorId", field::LAN_PARENT_ACTOR_ID),
        field_descriptor("LanParentDeviceId", field::LAN_PARENT_DEVICE_ID),
        field_descriptor("LanProofDigest", field::LAN_PROOF_DIGEST),
        field_descriptor("LanRouteId", field::LAN_ROUTE_ID),
        field_descriptor("LoadState", field::LOAD_STATE),
        field_descriptor(
            "LocalAiAdapterReadinessState",
            field::LOCAL_AI_ADAPTER_READINESS_STATE,
        ),
        field_descriptor("LocalAiCapabilityFlags", field::LOCAL_AI_CAPABILITY_FLAGS),
        field_descriptor("LocalAiDegradedState", field::LOCAL_AI_DEGRADED_STATE),
        field_descriptor("LocalAiExecutionState", field::LOCAL_AI_EXECUTION_STATE),
        field_descriptor("LocalAiModelId", field::LOCAL_AI_MODEL_ID),
        field_descriptor("LocalAiPrivacyMode", field::LOCAL_AI_PRIVACY_MODE),
        field_descriptor("LocalAiProviderId", field::LOCAL_AI_PROVIDER_ID),
        field_descriptor("LocalAiProviderSource", field::LOCAL_AI_PROVIDER_SOURCE),
        field_descriptor("LocalAiResourceClass", field::LOCAL_AI_RESOURCE_CLASS),
        field_descriptor(
            "LocalAiRuntimeReferenceId",
            field::LOCAL_AI_RUNTIME_REFERENCE_ID,
        ),
        field_descriptor(
            "LocalAiUnavailableReason",
            field::LOCAL_AI_UNAVAILABLE_REASON,
        ),
        field_descriptor("Message", field::MESSAGE),
        field_descriptor(
            "NetworkAndroidVpnServiceGateStatus",
            network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
        ),
        field_descriptor(
            "NetworkAppleNetworkExtensionGateStatus",
            network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
        ),
        field_descriptor(
            "NetworkLinuxNftablesLabStatus",
            network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
        ),
        field_descriptor(
            "NetworkLiveCaptureStatus",
            network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
        ),
        field_descriptor(
            "NetworkRuntimeDeadLetters",
            field::NETWORK_RUNTIME_DEAD_LETTERS,
        ),
        field_descriptor(
            "NetworkRuntimeDeliveredRows",
            field::NETWORK_RUNTIME_DELIVERED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeEnforcementCommandEvents",
            field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS,
        ),
        field_descriptor(
            "NetworkRuntimeEventChainStream",
            field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
        ),
        field_descriptor(
            "NetworkRuntimeFailedRows",
            field::NETWORK_RUNTIME_FAILED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeManualRequiredRows",
            field::NETWORK_RUNTIME_MANUAL_REQUIRED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeObservedRows",
            field::NETWORK_RUNTIME_OBSERVED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimePublishReports",
            field::NETWORK_RUNTIME_PUBLISH_REPORTS,
        ),
        field_descriptor(
            "NetworkRuntimeStoredEvents",
            field::NETWORK_RUNTIME_STORED_EVENTS,
        ),
        field_descriptor(
            "NetworkRuntimeStreamedEvents",
            field::NETWORK_RUNTIME_STREAMED_EVENTS,
        ),
        field_descriptor(
            "NetworkRemoteDeliveryStatus",
            field::NETWORK_REMOTE_DELIVERY_STATUS,
        ),
        field_descriptor(
            "NetworkWindowsFirewallLabStatus",
            network_flow::FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS,
        ),
        field_descriptor(
            "NetworkWindowsWfpGateStatus",
            network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
        ),
        field_descriptor("Online", field::ONLINE),
        field_descriptor(
            "ParentAssistantAnswerState",
            field::PARENT_ASSISTANT_ANSWER_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiAuthorizationState",
            field::PARENT_ASSISTANT_API_AUTHORIZATION_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiCustodyLabel",
            field::PARENT_ASSISTANT_API_CUSTODY_LABEL,
        ),
        field_descriptor(
            "ParentAssistantApiDeletionState",
            field::PARENT_ASSISTANT_API_DELETION_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiProviderBoundary",
            field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
        ),
        field_descriptor(
            "ParentAssistantApiRetentionState",
            field::PARENT_ASSISTANT_API_RETENTION_STATE,
        ),
        field_descriptor(
            "ParentAssistantCitationCount",
            field::PARENT_ASSISTANT_CITATION_COUNT,
        ),
        field_descriptor(
            "ParentAssistantEvidenceSummary",
            field::PARENT_ASSISTANT_EVIDENCE_SUMMARY,
        ),
        field_descriptor(
            "ParentAssistantProviderRoute",
            field::PARENT_ASSISTANT_PROVIDER_ROUTE,
        ),
        field_descriptor(
            "ParentAssistantRequestId",
            field::PARENT_ASSISTANT_REQUEST_ID,
        ),
        field_descriptor(
            "ParentAssistantQuickActionId",
            field::PARENT_ASSISTANT_QUICK_ACTION_ID,
        ),
        field_descriptor(
            "ParentAssistantPromptTemplateId",
            field::PARENT_ASSISTANT_PROMPT_TEMPLATE_ID,
        ),
        field_descriptor(
            "ParentAssistantStarterCategory",
            field::PARENT_ASSISTANT_STARTER_CATEGORY,
        ),
        field_descriptor(
            "ParentAssistantInputText",
            field::PARENT_ASSISTANT_INPUT_TEXT,
        ),
        field_descriptor(
            "ParentAssistantInputSource",
            field::PARENT_ASSISTANT_INPUT_SOURCE,
        ),
        field_descriptor("RangeEnd", field::RANGE_END),
        field_descriptor("RangeStart", field::RANGE_START),
        field_descriptor("Reason", field::REASON),
        field_descriptor("RequestedAt", field::REQUESTED_AT),
        field_descriptor("Returned", field::RETURNED),
        field_descriptor("ScopeKind", field::SCOPE_KIND),
        field_descriptor("Transport", field::TRANSPORT),
    ]
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
