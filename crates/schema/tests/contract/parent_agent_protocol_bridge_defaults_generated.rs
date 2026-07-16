use crate::support::assert_contract_contains_all;
use ocentra_schema::parent_ui_bridge_ts::{
    agent_protocol_domain_contracts_typescript, parent_ui_bridge_typescript,
    portal_contracts_typescript,
};

pub(super) fn assert_parent_agent_protocol_bridge_defaults() {
    let parent_generated = parent_ui_bridge_typescript();
    let portal_generated = portal_contracts_typescript();

    assert_contract_contains_all(
        crate::contract_text!(parent_generated.as_str()),
        crate::contract_texts![
            r#"PortalDev: {"peerId":"portal-dev","role":"portal"}"#,
            r#"LocalhostWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"localhost"}"#,
            r#"LocalNetworkWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"local-network"}"#,
            r#"MessageIdPrefix: "cmd-""#,
            "export interface ParentAgentCommandEnvelope {",
            "export function decodeParentAgentCommandEnvelope(value: unknown): ParentAgentCommandEnvelope",
            "export interface ParentAgentEventEnvelope {",
            "export function decodeParentAgentEventEnvelope(value: unknown): ParentAgentEventEnvelope",
            r#"Trace: "trace""#,
            r#"LanRouteId: "routeId""#,
            r#"ParentAssistantQuickActionId: "quickActionId""#,
        ],
    );

    assert_contract_contains_all(
        crate::contract_text!(portal_generated.as_str()),
        crate::contract_texts![
            r#"PortalDev: {"peerId":"portal-dev","role":"portal"}"#,
            r#"LocalNetworkWindowsAgent: {"deviceId":"local-dev-agent","platform":"windows","route":"local-network"}"#,
            r#"MessageIdPrefix: "cmd-""#,
            "export interface GeneratedPortalAgentCommandEnvelope {",
            "export function decodeGeneratedPortalAgentCommandEnvelope(value: unknown): GeneratedPortalAgentCommandEnvelope",
            "export interface GeneratedPortalAgentEventEnvelope {",
            "export function decodeGeneratedPortalAgentEventEnvelope(value: unknown): GeneratedPortalAgentEventEnvelope",
            r#"Trace: "trace""#,
            r#"LanRouteId: "routeId""#,
            r#"ParentAssistantQuickActionId: "quickActionId""#,
        ],
    );

    let generated = agent_protocol_domain_contracts_typescript();
    assert_contract_contains_all(
        crate::contract_text!(generated.as_str()),
        crate::contract_texts![
            "export const ParentAgentProtocolRuntime = {",
            r#"MessageIdPrefix: "cmd-""#,
            "export interface ParentAgentCommandEnvelope {",
            "export interface ParentAgentEventEnvelope {",
            "export function decodeParentAgentCommandEnvelope(value: unknown): ParentAgentCommandEnvelope",
            "export function decodeParentAgentEventEnvelope(value: unknown): ParentAgentEventEnvelope",
            "export function decodeParentAgentMessageId(value: unknown): string",
            "export function decodeParentSerializedAgentMessage(value: unknown): string",
            "export function isParentAgentProtocolLogText(value: unknown): value is string",
            "export const ParentAgentActivitySurfaceSchemaVersion = 1 as const;",
            "export const ParentAgentActivityReadModelState = {",
            "export const ParentAgentActivitySurfaceRequestSchema =",
            "export const ParentAgentActivityReportDocumentSchema =",
            "export const ParentAgentActivitySurfaceAdapterOperationManifest =",
            "export type ParentAgentActivityReportDocument =",
        ],
    );

    assert_contract_contains_all(
        crate::contract_text!(parent_generated.as_str()),
        crate::contract_texts![
            r#"Trust: "trust""#,
            r#"Ignore: "ignore""#,
            r#"Restore: "restore""#,
            r#"Rename: "rename""#,
            r#"ConfigurationUpdate: "configuration-update""#,
            r#"ActiveController: "active-controller""#,
            r#"ParentAgentLanHouseholdDeviceKindValues = ["mobile","desktop","laptop","tablet","router","unknown"]"#,
            "export const ParentAgentLanHouseholdActionDeviceKindField = ParentAgentProtocolField.LanHouseholdActionDeviceKind;",
        ],
    );
}
