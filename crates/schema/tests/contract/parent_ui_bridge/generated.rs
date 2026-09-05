use crate::support::{extract_typescript_block, ValueOrUnreachable as _};
use ocentra_schema::parent_ui_bridge::{
    ParentServiceHealthAuthenticationState, PARENT_BRIDGE_COMMAND_DISPATCH,
    PARENT_BRIDGE_COMMAND_LOAD_ROUTE, PARENT_BRIDGE_COMMAND_SUBSCRIBE,
    PARENT_BRIDGE_COMMAND_UNSUBSCRIBE, PARENT_DEV_BRIDGE_ROUTE_DISPATCH,
    PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE, PARENT_ROUTE_HASH_PREFIX,
    PARENT_ROUTE_HASH_QUERY_SEPARATOR, PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX,
    PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS, PARENT_UI_BRIDGE_SCHEMA_VERSION,
};
use ocentra_schema::parent_ui_bridge_ts::{
    parent_ui_bridge_typescript, parent_ui_screen_bridge_typescript, portal_contracts_typescript,
};

const OPEN_BRACE: char = 123u8 as char;
const CLOSE_BRACE: char = 125u8 as char;

macro_rules! line_with_open_brace {
    ($prefix:expr $(,)?) => {
        format!("{}{}", $prefix, OPEN_BRACE)
    };
}

macro_rules! line_with_close_brace {
    ($suffix:expr $(,)?) => {
        format!("{}{}", CLOSE_BRACE, $suffix)
    };
}

macro_rules! assert_has_line {
    ($source:expr, $expected:expr $(,)?) => {{
        let expected_value = $expected;
        let expected_line: &str = expected_value.as_ref();
        let expected_trimmed = expected_line.trim();
        let source_text: &str = ($source).as_ref();
        let actual_line = source_text
            .lines()
            .map(str::trim)
            .find(|line| *line == expected_trimmed);
        assert_eq!(
            actual_line,
            Some(expected_trimmed),
            "missing generated line: {expected_line}"
        );
    }};
}

macro_rules! assert_has_fragment {
    ($source:expr, $expected:expr $(,)?) => {{
        let expected_value = $expected;
        let expected_fragment: &str = expected_value.as_ref();
        assert_eq!(
            ($source)
                .match_indices(expected_fragment)
                .next()
                .map(|(_, fragment)| fragment),
            Some(expected_fragment),
            "missing generated fragment: {expected_fragment}"
        );
    }};
}

#[test]
fn generated_service_health_authentication_states_match_rust_serialization() {
    let generated = parent_ui_bridge_typescript();
    let serialized_states = [
        ParentServiceHealthAuthenticationState::Authenticated,
        ParentServiceHealthAuthenticationState::Unauthenticated,
        ParentServiceHealthAuthenticationState::Unavailable,
    ]
    .map(|state| {
        serde_json::to_value(state).value_or_unreachable(crate::assert_context!(
            "service health authentication state serializes",
        ))
    });

    assert_eq!(
        serialized_states,
        [
            serde_json::json!("authenticated"),
            serde_json::json!("unauthenticated"),
            serde_json::json!("unavailable"),
        ],
    );
    assert_has_line!(
        &generated,
        &line_with_open_brace!("export const ParentServiceHealthAuthenticationState = "),
    );
    assert_has_line!(&generated, "  Authenticated: 'authenticated',");
    assert_has_line!(&generated, "  Unauthenticated: 'unauthenticated',");
    assert_has_line!(&generated, "  Unavailable: 'unavailable',");
    assert_has_line!(
        &generated,
        "(typeof ParentServiceHealthAuthenticationState)[keyof typeof ParentServiceHealthAuthenticationState];",
    );
}

#[test]
fn parent_route_hash_prefix_matches_canonical_portal_routes() {
    assert_eq!(PARENT_ROUTE_HASH_PREFIX, "#/");
}

#[test]
fn generated_typescript_bridge_uses_rust_constants() {
    let generated = parent_ui_bridge_typescript();

    assert_has_line!(
        &generated,
        &line_with_open_brace!("export interface ParentDesktopDistributionSnapshot "),
    );
    assert_has_line!(
        &generated,
        "  readonly parentDesktopDistribution?: ParentDesktopDistributionSnapshot | null;",
    );
    for decoder in [
        "export function decodeParentDesktopDistributionSnapshot(",
        "export function decodeParentRouteSnapshot(value: unknown): ParentRouteSnapshot {",
        "export function decodeParentRouteSnapshotForRoute(",
        "export function decodeParentUiActionResult(value: unknown): ParentUiActionResult {",
        "export function decodeParentSubscriptionEvent(value: unknown): ParentSubscriptionEvent {",
        "export function decodeParentRouteSubscriptionId(value: unknown): ParentRouteSubscriptionId {",
        "export function decodeParentBridgeUnsubscribeResult(value: unknown): boolean {",
    ] {
        assert_has_line!(&generated, decoder);
    }
    assert_has_line!(
        &generated,
        &line_with_open_brace!("export interface ParentActivityMemoryGraphReadModelSnapshot "),
    );
    assert_has_line!(
        &generated,
        "export function decodeParentActivityMemoryGraphDigest(",
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                &line_with_open_brace!("export const ParentBridgeCommand = "),
                &line_with_close_brace!(" as const;")
            )
        ),
        crate::ts_block!(format!(
            "LoadRoute: '{PARENT_BRIDGE_COMMAND_LOAD_ROUTE}',\n  Dispatch: '{PARENT_BRIDGE_COMMAND_DISPATCH}',\n  Subscribe: '{PARENT_BRIDGE_COMMAND_SUBSCRIBE}',\n  Unsubscribe: '{PARENT_BRIDGE_COMMAND_UNSUBSCRIBE}',"
        ))
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                &line_with_open_brace!("export const ParentDevBridgeRoute = "),
                &line_with_close_brace!(" as const;")
            )
        ),
        crate::ts_block!(format!(
            "LoadRoute: '{PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE}',\n  Dispatch: '{PARENT_DEV_BRIDGE_ROUTE_DISPATCH}',"
        ))
    );
    assert_has_line!(
        &generated,
        &format!("  SchemaVersion: {PARENT_UI_BRIDGE_SCHEMA_VERSION},"),
    );
    assert_has_line!(
        &generated,
        &format!("  DevRouteSubscriptionPollMs: {PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS},"),
    );
    assert_has_line!(
        &generated,
        &format!("  RouteSubscriptionEventPrefix: '{PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX}',"),
    );
    assert_has_line!(
        &generated,
        &format!("  RouteHashPrefix: '{PARENT_ROUTE_HASH_PREFIX}',"),
    );
    assert_has_line!(
        &generated,
        &format!("  RouteHashQuerySeparator: '{PARENT_ROUTE_HASH_QUERY_SEPARATOR}',"),
    );
}

#[test]
fn route_titles_and_dev_diagnostics_are_rust_owned_bridge_metadata() {
    let generated = parent_ui_bridge_typescript();

    assert_has_line!(
        &generated,
        "export const ParentRoutes: readonly ParentRouteId[] = [",
    );
    assert_has_line!(&generated, "  ParentRoute.Devices,");
    assert_has_line!(&generated, "  ParentRoute.AppLayout,");
    assert_has_line!(&generated, "  ParentRoute.FrameTuner,");
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const ParentDevDiagnosticRoutes: readonly ParentRouteId[] = [",
                "] as const;"
            )
        ),
        crate::ts_block!(
            "ParentRoute.Diagnostics,\n  ParentRoute.ProofPanels,\n  ParentRoute.AppLayout,\n  ParentRoute.FrameTuner,\n  ParentRoute.Commands,\n  ParentRoute.Events,\n  ParentRoute.Logs,"
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const ParentAppGameParentSurfaceRoutes: readonly ParentRouteId[] = [",
                "] as const;"
            )
        ),
        crate::ts_block!("ParentRoute.AppGameSessions,")
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const ParentBrowserParentSurfaceRoutes: readonly ParentRouteId[] = [",
                "] as const;"
            )
        ),
        crate::ts_block!("ParentRoute.Browser,")
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                &line_with_open_brace!("export const ParentRouteGroup = "),
                &line_with_close_brace!(" as const;")
            )
        ),
        crate::ts_block!(
            "Monitor: 'monitor',\n  Guide: 'guide',\n  Operate: 'operate',\n  DevTools: 'dev-tools',"
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const ParentSidebarRouteGroups: readonly ParentRouteGroupId[] = [",
                "] as const;"
            )
        ),
        crate::ts_block!(
            "ParentRouteGroup.Monitor,\n  ParentRouteGroup.Guide,\n  ParentRouteGroup.Operate,\n  ParentRouteGroup.DevTools,"
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const ParentSidebarRoutes: readonly ParentRouteId[] = ParentRoutes.filter(",
                ");"
            )
        ),
        crate::ts_block!("(route) => ParentRouteMetadata[route].sidebar")
    );
}

#[test]
fn generated_typescript_carries_rust_owned_portal_text_value_edges() {
    let generated = parent_ui_bridge_typescript();

    assert_has_line!(&generated, "export type ParentPortalDetailValue = string;");
    assert_has_line!(
        &generated,
        "export type ParentPortalClipboardText = string;",
    );
    assert_has_line!(
        &generated,
        "export type ParentTrackingStatusProofArtifact = string;",
    );
    assert_has_line!(
        &generated,
        &line_with_open_brace!(
            "export function decodeParentPortalDetailValue(value: string): ParentPortalDetailValue ",
        ),
    );
    assert_has_line!(
        &generated,
        &line_with_open_brace!(
            "export function decodeParentPortalClipboardText(value: string): ParentPortalClipboardText ",
        ),
    );
    assert_has_line!(
        &generated,
        "export function decodeParentTrackingStatusProofArtifact(",
    );
    assert_has_line!(&generated, "  value: string");
    assert_has_line!(
        &generated,
        &line_with_open_brace!("): ParentTrackingStatusProofArtifact "),
    );
}

#[test]
fn generated_typescript_artifact_stays_checked_in() {
    let checked_in = include_str!("../../../../../apps/portal/generated/parent-ui-bridge.ts");
    let generated = parent_ui_bridge_typescript();

    assert_eq!(checked_in, generated);
}

#[test]
fn generated_screen_bridge_artifact_stays_checked_in() {
    let checked_in =
        include_str!("../../../../../apps/portal/generated/parent-ui-screen-bridge.ts");
    let generated = parent_ui_screen_bridge_typescript();

    assert_eq!(checked_in, generated);
    assert_has_line!(
        &generated,
        &line_with_open_brace!(
            "export function parentScreenEvidenceSettingsWritableUiProof(): ParentScreenEvidenceSettingsUiProof ",
        ),
    );
    assert_has_line!(
        &generated,
        &line_with_open_brace!(
            "export function parentScreenControlSettingsPortalProof(): ParentScreenControlSettingsPortalProof ",
        ),
    );
    assert_has_line!(
        &generated,
        &line_with_open_brace!(
            "export function decodeParentScreenSettingsUpdateResponse(value: unknown): ParentScreenSettingsUpdateResponse | null ",
        ),
    );
    assert_has_line!(
        &generated,
        &format!(
            "export const ParentScreenSettingsUpdateStatus = {OPEN_BRACE} Accepted: 'accepted', Rejected: 'rejected' {CLOSE_BRACE} as const;"
        ),
    );
    assert_has_line!(
        &generated,
        "export const ParentScreenOptionalVisibilityCapabilityProofGeneratedAt = '2026-06-07T05:55:00Z' as const;",
    );
}

#[test]
fn generated_portal_domain_portal_contracts_artifact_stays_checked_in() {
    let checked_in =
        include_str!("../../../../../packages/portal-domain/src/generated-portal-contracts.ts");
    let generated = portal_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_has_fragment!(
        &generated,
        &line_with_open_brace!("export const GeneratedPortalRouteLiteral = "),
    );
    assert_has_fragment!(
        &generated,
        "export type GeneratedTrackingStatusProofArtifact = typeof GeneratedTrackingStatusProofArtifactSchema.Type;",
    );
    assert_has_fragment!(
        &generated,
        r#""CommandId":"tracking-retention-settings-write-command""#,
    );
    assert_has_fragment!(
        &generated,
        r#""proofId":"tracking-notification-parent-surface-history-proof""#,
    );
    crate::support::assert_contract_has_lines(
        crate::contract_text!(&generated),
        crate::contract_texts![
            line_with_open_brace!(
                "export interface GeneratedPortalScreenSummaryPanelDetailSnapshot "
            ),
            line_with_open_brace!("export interface GeneratedPortalScreenSummaryPanelRowSnapshot "),
            line_with_open_brace!("export interface GeneratedPortalScreenSummaryPanelSnapshot "),
            "readonly summaryDetails: readonly GeneratedPortalScreenSummaryPanelDetailSnapshot[];",
            "readonly rows: readonly GeneratedPortalScreenSummaryPanelRowSnapshot[];",
            line_with_open_brace!("export const GeneratedPortalTrackingContracts = "),
            line_with_open_brace!(
                "export interface GeneratedPortalActivityMemoryGraphReadModelSnapshot ",
            ),
            "export function decodeGeneratedPortalActivityMemoryGraphDigest(",
            "function isGeneratedPortalActivityMemoryGraphTraceSnapshot(",
            "ActivityTrackingReadModel: ",
            "RetentionSettingsWrite: ",
            "NotificationParentSurfaceHistory: ",
            "interface GeneratedParentActivityTrackingReadModelSnapshot ",
            "interface GeneratedTrackingRetentionSettingsWriteResult ",
            "const GeneratedDefaultTrackingNotificationParentSurfaceHistoryReadModel =",
        ],
    );
}

#[test]
fn generated_portal_domain_lan_add_device_contracts_are_rust_owned() {
    let generated = portal_contracts_typescript();

    for expected in [
        line_with_open_brace!("export interface GeneratedPortalLanAddDeviceReadModelSnapshot "),
        "readonly scanSummary: GeneratedPortalLanAddDeviceScanSummarySnapshot;".to_string(),
        "readonly discoveredDevices: readonly GeneratedPortalLanBrowserAddDeviceDiscoveryDeviceSnapshot[];"
            .to_string(),
        "readonly canonicalHouseholdDevices: readonly GeneratedPortalLanCanonicalHouseholdDeviceSnapshot[];"
            .to_string(),
        "readonly trustedDeviceRegistry: readonly GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot[];"
            .to_string(),
        "readonly householdDeviceDecisions: readonly GeneratedPortalLanHouseholdDeviceDecisionSnapshot[];"
            .to_string(),
        "readonly discoveryEventHistory: GeneratedPortalLanDiscoveryEventHistorySnapshot;"
            .to_string(),
        "readonly selectedDeviceReadiness: GeneratedPortalLanSelectedDeviceReadinessSnapshot;"
            .to_string(),
        "readonly lanDiscoverySourceMatrix: GeneratedPortalLanDiscoverySourceMatrixSnapshot | null;"
            .to_string(),
        line_with_open_brace!("export interface GeneratedPortalLanSignedDiscoveryRelayAdapterRowSnapshot "),
        line_with_open_brace!("export interface GeneratedPortalLanDiscoveryEventHistorySnapshot "),
        line_with_open_brace!("export interface GeneratedPortalLanSelectedDeviceReadinessSnapshot "),
        line_with_open_brace!("export interface GeneratedPortalLanDiscoveryEvidenceRecordSnapshot "),
        line_with_open_brace!("export interface GeneratedPortalLanDiscoverySourceMatrixSnapshot "),
    ] {
        assert_has_line!(&generated, &expected);
    }
}

#[test]
fn generated_portal_social_read_model_payload_fields_are_rust_owned() {
    let generated = portal_contracts_typescript();

    for expected in [
        r#"BrowserSocialAlertReportReadModel: "browserSocialAlertReportReadModel""#,
        r#"BrowserSocialAlertReportParentSurfaceReadModel: "browserSocialAlertReportParentSurfaceReadModel""#,
        r#"BrowserSocialDashboardReadModel: "browserSocialDashboardReadModel""#,
        r#"BrowserSocialParentNotificationDeliveryReadModel: "browserSocialParentNotificationDeliveryReadModel""#,
    ] {
        assert_has_fragment!(&generated, expected);
    }

    for expected in [
        "export type GeneratedPortalSocialAlertReportReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;",
        "export type GeneratedPortalSocialAlertReportParentSurfaceReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;",
        "export type GeneratedPortalSocialParentNotificationDeliveryReadModelSnapshot = GeneratedPortalRouteEventPayloadRecord;",
        "export type GeneratedPortalSocialDashboardUxSnapshot = GeneratedPortalRouteEventPayloadRecord;",
        &line_with_open_brace!("export const GeneratedPortalSocialReadModelPayloadField = "),
        "AlertReport: GeneratedPortalAgentProtocolField.BrowserSocialAlertReportReadModel,",
        "AlertReportParentSurface: GeneratedPortalAgentProtocolField.BrowserSocialAlertReportParentSurfaceReadModel,",
        "ParentNotificationDelivery: GeneratedPortalAgentProtocolField.BrowserSocialParentNotificationDeliveryReadModel,",
        "Dashboard: GeneratedPortalAgentProtocolField.BrowserSocialDashboardReadModel,",
    ] {
        assert_has_line!(&generated, expected);
    }
}

#[test]
fn portal_domain_portal_contracts_adapter_stays_generated_backed() {
    let adapter =
        include_str!("../../../../../packages/portal-domain/src/portal-contract-adapter.ts");
    let text_contracts =
        include_str!("../../../../../packages/portal-domain/src/portal-contract-text-contracts.ts");

    assert!(adapter
        .lines()
        .any(|line| line.contains("from './generated-portal-contracts'")));
    assert_has_line!(
        adapter,
        "export const PortalRouteLiteral = GeneratedPortalRouteLiteral;",
    );
    assert_has_line!(
        adapter,
        "export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;",
    );
    assert_has_line!(
        adapter,
        "export const PortalConnectionState = GeneratedPortalConnectionState;",
    );
    assert_has_line!(
        text_contracts,
        "export type PortalDetailValue = GeneratedPortalDetailValue;",
    );
    assert_has_line!(
        text_contracts,
        "export const decodePortalDetailValue = (input: unknown): PortalDetailValue =>",
    );
    assert_has_line!(
        text_contracts,
        "export const decodeTrackingStatusProofArtifact = (input: unknown): TrackingStatusProofArtifact =>",
    );
}
