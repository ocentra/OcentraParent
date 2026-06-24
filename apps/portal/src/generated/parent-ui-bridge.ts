/* generated from crates/schema/src/parent_ui_bridge.rs */

export type ParentRouteId =
  | 'overview'
  | 'assistant'
  | 'start'
  | 'activity'
  | 'browser'
  | 'browser-settings'
  | 'policy'
  | 'policy-apps'
  | 'policy-games'
  | 'policy-screen'
  | 'policy-network'
  | 'policy-tracking'
  | 'policy-remote-screen'
  | 'rule-management'
  | 'schedules'
  | 'approvals'
  | 'enforcement'
  | 'privacy-design'
  | 'memory'
  | 'memory-settings'
  | 'ai-guide'
  | 'ai-runtime'
  | 'api-providers'
  | 'reports-guide'
  | 'screen-analysis'
  | 'app-game-sessions'
  | 'network-activity'
  | 'devices'
  | 'lan-pairing'
  | 'capability-status'
  | 'notifications'
  | 'notification-channels'
  | 'drive-connections'
  | 'export-retention'
  | 'remote-access'
  | 'report-compiler'
  | 'audit-history'
  | 'subscription'
  | 'entitlements'
  | 'platforms-install'
  | 'install-updates'
  | 'diagnostics'
  | 'proof-panels'
  | 'settings-rules'
  | 'app-layout'
  | 'commands'
  | 'events'
  | 'logs';

export type ParentBridgeConnectionState = 'disconnected' | 'connecting' | 'connected' | 'error';

export type ParentRouteDataSource = 'host-bridge' | 'rust-read-model' | 'dev-diagnostics' | 'unavailable';

export type ParentPortalTone = 'cyan' | 'gold' | 'purple' | 'red' | 'muted';

export type ParentPortalParentAccessState =
  | 'active-controller'
  | 'observer-only'
  | 'unauthenticated'
  | 'proof-missing';

export interface ParentRouteContext {
  readonly selectedChildDeviceId?: string | null;
}

export interface ParentPortalRowSnapshot {
  readonly label: string;
  readonly order: number;
  readonly signalScore: number;
  readonly readyCount: number;
  readonly gapCount: number;
  readonly primaryArea: string;
  readonly trend: string;
  readonly tone: ParentPortalTone;
}

export interface ParentPortalShellStatusCardSnapshot {
  readonly id: string;
  readonly label: string;
  readonly value: string;
  readonly detail: string;
  readonly tone: ParentPortalTone;
}

export interface ParentPortalShellStatusSnapshot {
  readonly routeLabel: string;
  readonly parentAccessState: ParentPortalParentAccessState;
  readonly globalConnectionState: string;
  readonly routeCapabilityState: string;
  readonly dataSourceLabel: string;
  readonly cards: readonly ParentPortalShellStatusCardSnapshot[];
}

export interface ParentRouteEventSnapshot {
  readonly event?: string | null;
  readonly eventId?: string | null;
  readonly sentAt?: string | null;
  readonly severity?: string | null;
  readonly payload?: Record<string, unknown> | null;
}

export interface ParentRouteLiveActivitySnapshot {
  readonly recentSummary?: Record<string, unknown> | null;
  readonly ingestStatus?: Record<string, unknown> | null;
  readonly activityScreenReadModel?: Record<string, unknown> | null;
  readonly browserManagedEvent?: ParentRouteEventSnapshot | null;
  readonly browserManagedStatus?: Record<string, unknown> | null;
  readonly browserRuntimeEventChainStream?: Record<string, unknown> | null;
  readonly browserSocialProviderReceiptStreamStatusIntent?: Record<string, unknown> | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusIntent?: Record<string, unknown> | null;
  readonly localAiRuntimeStatusEvent?: ParentRouteEventSnapshot | null;
  readonly lanAiJobEvent?: ParentRouteEventSnapshot | null;
  readonly parentAssistantBoundaryEvent?: ParentRouteEventSnapshot | null;
  readonly activityMemoryGraphReadModel?: Record<string, unknown> | null;
  readonly networkFlowEvent?: ParentRouteEventSnapshot | null;
  readonly networkFlowReadModel?: Record<string, unknown> | null;
  readonly networkRuntimeEventChainStream?: Record<string, unknown> | null;
  readonly lanPairingBrowserDiscoveryEvent?: ParentRouteEventSnapshot | null;
  readonly lanAddDeviceReadModel?: Record<string, unknown> | null;
  readonly policyPreviewEvent?: ParentRouteEventSnapshot | null;
  readonly policyPreviewReadModel?: Record<string, unknown> | null;
  readonly appGameNotificationParentSurfaceIntentReadModel?: Record<string, unknown> | null;
  readonly appGamePolicyReadinessReadModel?: Record<string, unknown> | null;
  readonly appGamePlatformProofStatusReadModel?: Record<string, unknown> | null;
  readonly appGameChildRuntimeTransportReceiptReadModel?: Record<string, unknown> | null;
  readonly appGameAdapterDispatchPreflightReadModel?: Record<string, unknown> | null;
  readonly appGameAdapterDispatchResultReadModel?: Record<string, unknown> | null;
  readonly appGameAdapterDispatchExecutedResult?: Record<string, unknown> | null;
  readonly appGameTimerParentSurfaceReadModel?: Record<string, unknown> | null;
  readonly appGameTimerParentPreferenceSetupRequestedResult?: Record<string, unknown> | null;
  readonly browserInterventionEvent?: ParentRouteEventSnapshot | null;
  readonly browserInterventionReadModel?: Record<string, unknown> | null;
  readonly activityTrackingReadModelEvent?: ParentRouteEventSnapshot | null;
  readonly activityTrackingReadModel?: Record<string, unknown> | null;
  readonly activityTrackingRetentionSettingsWriteResult?: Record<string, unknown> | null;
}

export interface ParentRouteBrowserPanelsSnapshot {
  readonly socialAuditExplanation?: Record<string, unknown> | null;
  readonly socialAlertReport?: Record<string, unknown> | null;
  readonly socialAlertReportParentSurface?: Record<string, unknown> | null;
  readonly socialParentNotificationDelivery?: Record<string, unknown> | null;
  readonly socialDashboard?: Record<string, unknown> | null;
}

export interface ParentRouteSummary {
  readonly title: string;
  readonly routeCapability: string;
  readonly parentAccess: string;
  readonly household: string;
  readonly childDevice: string;
}

export interface ParentRouteSnapshot {
  readonly schemaVersion: number;
  readonly route: ParentRouteId;
  readonly generatedAt: string;
  readonly seasonLabel: string;
  readonly lastUpdated: string;
  readonly connectionState: ParentBridgeConnectionState;
  readonly commandEnabled: boolean;
  readonly agentEndpoint: string;
  readonly dataSource: ParentRouteDataSource;
  readonly summary: ParentRouteSummary;
  readonly diagnosticPanelsEnabled: boolean;
  readonly parentPortalRows?: readonly ParentPortalRowSnapshot[] | null;
  readonly parentPortalShellStatus?: ParentPortalShellStatusSnapshot | null;
  readonly liveActivity?: ParentRouteLiveActivitySnapshot | null;
  readonly browserPanels?: ParentRouteBrowserPanelsSnapshot | null;
  readonly screenSettingsServiceResponse?: Record<string, unknown> | null;
}

export type ParentUiActionKind =
  | 'refresh-route'
  | 'reconnect'
  | 'agent-command-requested'
  | 'lan-pairing-browser-discovery-scan-requested'
  | 'network-flow-read-model-refresh-requested'
  | 'tracking-retention-settings-write-requested'
  | 'screen-settings-get-requested'
  | 'screen-settings-replace-requested'
  | 'app-game-adapter-dispatch-execute-requested'
  | 'app-game-timer-parent-preference-setup-requested';

export interface ParentUiAction {
  readonly action: ParentUiActionKind;
  readonly route: ParentRouteId;
  readonly command?: string | null;
  readonly payload: Record<string, string | number | boolean | null>;
}

export interface ParentUiActionResult {
  readonly schemaVersion: number;
  readonly accepted: boolean;
  readonly connectionState: ParentBridgeConnectionState;
  readonly message: string;
  readonly snapshot: ParentRouteSnapshot | null;
}

export interface ParentSubscriptionEvent {
  readonly schemaVersion: number;
  readonly route: ParentRouteId;
  readonly snapshot: ParentRouteSnapshot;
}

export interface HostBridge {
  loadRoute(route: ParentRouteId, context?: ParentRouteContext): Promise<ParentRouteSnapshot>;
  dispatch(action: ParentUiAction): Promise<ParentUiActionResult>;
  subscribe(
    route: ParentRouteId,
    context: ParentRouteContext,
    onEvent: (event: ParentSubscriptionEvent) => void
  ): Promise<() => void>;
}
