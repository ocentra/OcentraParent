import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';
import { PortalProductText, PortalProductTextToken } from './portal-product-text';

export const PortalDevTextToken = {
  AppTitle: decodeTextTokenId('portal.dev.appTitle'),
  Subtitle: decodeTextTokenId('portal.dev.subtitle'),
  Reconnect: decodeTextTokenId('portal.dev.reconnect'),
  ...PortalProductTextToken,
  AgentCommands: decodeTextTokenId('portal.dev.agentCommands'),
  AgentEvents: decodeTextTokenId('portal.dev.agentEvents'),
  ActivityTimeline: decodeTextTokenId('portal.dev.activityTimeline'),
  DeviceDiagnostics: decodeTextTokenId('portal.dev.deviceDiagnostics'),
  DevLog: decodeTextTokenId('portal.dev.devLog'),
  EvidenceStore: decodeTextTokenId('portal.dev.evidenceStore'),
  BrowserEvidence: decodeTextTokenId('portal.dev.browserEvidence'),
  BrowserIntervention: decodeTextTokenId('portal.dev.browserIntervention'),
  BrowserManagedStatus: decodeTextTokenId('portal.dev.browserManagedStatus'),
  ActivityMemoryGraph: decodeTextTokenId('portal.dev.activityMemoryGraph'),
  NetworkFlow: decodeTextTokenId('portal.dev.networkFlow'),
  PolicyPreview: decodeTextTokenId('portal.dev.policyPreview'),
  AppGameNotificationParentSurface: decodeTextTokenId('portal.dev.appGameNotificationParentSurface'),
  AppGameNotificationParentSurfaceBody: decodeTextTokenId('portal.dev.appGameNotificationParentSurfaceBody'),
  AppGameNotificationParentSurfaceNoData: decodeTextTokenId('portal.dev.appGameNotificationParentSurfaceNoData'),
  AppGameNotificationParentSurfaceNoRuntimeClaim: decodeTextTokenId(
    'portal.dev.appGameNotificationParentSurfaceNoRuntimeClaim'
  ),
  AppGamePolicyReadiness: decodeTextTokenId('portal.dev.appGamePolicyReadiness'),
  AppGamePolicyReadinessBody: decodeTextTokenId('portal.dev.appGamePolicyReadinessBody'),
  AppGamePolicyReadinessNoData: decodeTextTokenId('portal.dev.appGamePolicyReadinessNoData'),
  AppGamePolicyReadinessNoProductClaim: decodeTextTokenId('portal.dev.appGamePolicyReadinessNoProductClaim'),
  AppGamePolicyReadinessParserRejected: decodeTextTokenId('portal.dev.appGamePolicyReadinessParserRejected'),
  TrackingStatusSurface: decodeTextTokenId('portal.dev.trackingStatusSurface'),
  TrackingStatusSurfaceBody: decodeTextTokenId('portal.dev.trackingStatusSurfaceBody'),
  TrackingServiceReadModel: decodeTextTokenId('portal.dev.trackingServiceReadModel'),
  TrackingServiceDataCoverage: decodeTextTokenId('portal.dev.trackingServiceDataCoverage'),
  TrackingEvidenceDrawerHostedUi: decodeTextTokenId('portal.dev.trackingEvidenceDrawerHostedUi'),
  TrackingEvidenceDrawerHostedUiBody: decodeTextTokenId('portal.dev.trackingEvidenceDrawerHostedUiBody'),
  TrackingEvidenceDrawerReadOnly: decodeTextTokenId('portal.dev.trackingEvidenceDrawerReadOnly'),
  TrackingEvidenceDrawerBoundary: decodeTextTokenId('portal.dev.trackingEvidenceDrawerBoundary'),
  TrackingFamilyDashboardRollup: decodeTextTokenId('portal.dev.trackingFamilyDashboardRollup'),
  TrackingFamilyDashboardRollupBody: decodeTextTokenId('portal.dev.trackingFamilyDashboardRollupBody'),
  TrackingFamilyDashboardActiveSummary: decodeTextTokenId('portal.dev.trackingFamilyDashboardActiveSummary'),
  TrackingFamilyDashboardChildAttention: decodeTextTokenId('portal.dev.trackingFamilyDashboardChildAttention'),
  TrackingFamilyDashboardRetentionAudit: decodeTextTokenId('portal.dev.trackingFamilyDashboardRetentionAudit'),
  TrackingFamilyDashboardRollupReady: decodeTextTokenId('portal.dev.trackingFamilyDashboardRollupReady'),
  TrackingFamilyDashboardActiveEvidence: decodeTextTokenId('portal.dev.trackingFamilyDashboardActiveEvidence'),
  TrackingFamilyDashboardChildAttentionEvidence: decodeTextTokenId(
    'portal.dev.trackingFamilyDashboardChildAttentionEvidence'
  ),
  TrackingFamilyDashboardRetentionAuditEvidence: decodeTextTokenId(
    'portal.dev.trackingFamilyDashboardRetentionAuditEvidence'
  ),
  TrackingFamilyDashboardHostedBoundary: decodeTextTokenId('portal.dev.trackingFamilyDashboardHostedBoundary'),
  TrackingRetentionSettingsHostedUi: decodeTextTokenId('portal.dev.trackingRetentionSettingsHostedUi'),
  TrackingRetentionSettingsHostedUiBody: decodeTextTokenId('portal.dev.trackingRetentionSettingsHostedUiBody'),
  TrackingRetentionSettingsWindow: decodeTextTokenId('portal.dev.trackingRetentionSettingsWindow'),
  TrackingRetentionSettingsDeleteAfterAlert: decodeTextTokenId('portal.dev.trackingRetentionSettingsDeleteAfterAlert'),
  TrackingRetentionSettingsParentExport: decodeTextTokenId('portal.dev.trackingRetentionSettingsParentExport'),
  TrackingRetentionSettingsRemoteSyncDisabled: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsRemoteSyncDisabled'
  ),
  TrackingRetentionSettingsRemoteAiDisabled: decodeTextTokenId('portal.dev.trackingRetentionSettingsRemoteAiDisabled'),
  TrackingRetentionSettingsReadModelReady: decodeTextTokenId('portal.dev.trackingRetentionSettingsReadModelReady'),
  TrackingRetentionSettingsWindowEvidence: decodeTextTokenId('portal.dev.trackingRetentionSettingsWindowEvidence'),
  TrackingRetentionSettingsDeleteAfterAlertEvidence: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsDeleteAfterAlertEvidence'
  ),
  TrackingRetentionSettingsParentExportEvidence: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsParentExportEvidence'
  ),
  TrackingRetentionSettingsRemoteSyncEvidence: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsRemoteSyncEvidence'
  ),
  TrackingRetentionSettingsRemoteAiEvidence: decodeTextTokenId('portal.dev.trackingRetentionSettingsRemoteAiEvidence'),
  TrackingRetentionSettingsHostedBoundary: decodeTextTokenId('portal.dev.trackingRetentionSettingsHostedBoundary'),
  TrackingFirstTarget: decodeTextTokenId('portal.dev.trackingFirstTarget'),
  TrackingProofFixture: decodeTextTokenId('portal.dev.trackingProofFixture'),
  TrackingProofService: decodeTextTokenId('portal.dev.trackingProofService'),
  TrackingManualRequired: decodeTextTokenId('portal.dev.trackingManualRequired'),
  TrackingPhysicalDeviceRequired: decodeTextTokenId('portal.dev.trackingPhysicalDeviceRequired'),
  TrackingNoProductClaim: decodeTextTokenId('portal.dev.trackingNoProductClaim'),
  TrackingStateDisabled: decodeTextTokenId('portal.dev.trackingStateDisabled'),
  TrackingStatePermissionRequired: decodeTextTokenId('portal.dev.trackingStatePermissionRequired'),
  TrackingStateStale: decodeTextTokenId('portal.dev.trackingStateStale'),
  TrackingStateOffline: decodeTextTokenId('portal.dev.trackingStateOffline'),
  TrackingStateLowAccuracy: decodeTextTokenId('portal.dev.trackingStateLowAccuracy'),
  TrackingStateAmbiguousNearby: decodeTextTokenId('portal.dev.trackingStateAmbiguousNearby'),
  TrackingStateAlert: decodeTextTokenId('portal.dev.trackingStateAlert'),
  TrackingStateAcknowledged: decodeTextTokenId('portal.dev.trackingStateAcknowledged'),
  TrackingStateException: decodeTextTokenId('portal.dev.trackingStateException'),
  TrackingStateChildCheckIn: decodeTextTokenId('portal.dev.trackingStateChildCheckIn'),
  TrackingChildCheckInProofTitle: decodeTextTokenId('portal.dev.trackingChildCheckInProofTitle'),
  TrackingChildCheckInProofBody: decodeTextTokenId('portal.dev.trackingChildCheckInProofBody'),
  TrackingChildCheckInSafeAction: decodeTextTokenId('portal.dev.trackingChildCheckInSafeAction'),
  TrackingChildCheckInHelpAction: decodeTextTokenId('portal.dev.trackingChildCheckInHelpAction'),
  TrackingChildCheckInShareLocationAction: decodeTextTokenId('portal.dev.trackingChildCheckInShareLocationAction'),
  TrackingChildCheckInCallParentAction: decodeTextTokenId('portal.dev.trackingChildCheckInCallParentAction'),
  TrackingChildCheckInDeliveryBoundary: decodeTextTokenId('portal.dev.trackingChildCheckInDeliveryBoundary'),
  TrackingChildCheckInCopyBoundary: decodeTextTokenId('portal.dev.trackingChildCheckInCopyBoundary'),
  TrackingChildRuntimeUiProofTitle: decodeTextTokenId('portal.dev.trackingChildRuntimeUiProofTitle'),
  TrackingChildRuntimeUiProofBody: decodeTextTokenId('portal.dev.trackingChildRuntimeUiProofBody'),
  TrackingChildRuntimeDisclosure: decodeTextTokenId('portal.dev.trackingChildRuntimeDisclosure'),
  TrackingChildRuntimeSafeResponse: decodeTextTokenId('portal.dev.trackingChildRuntimeSafeResponse'),
  TrackingChildRuntimeHelpResponse: decodeTextTokenId('portal.dev.trackingChildRuntimeHelpResponse'),
  TrackingChildRuntimeLocationConsent: decodeTextTokenId('portal.dev.trackingChildRuntimeLocationConsent'),
  TrackingChildRuntimeBoundary: decodeTextTokenId('portal.dev.trackingChildRuntimeBoundary'),
  TrackingUnsupportedManualProofTitle: decodeTextTokenId('portal.dev.trackingUnsupportedManualProofTitle'),
  TrackingUnsupportedManualProofBody: decodeTextTokenId('portal.dev.trackingUnsupportedManualProofBody'),
  TrackingUnsupportedManualAndroidBackground: decodeTextTokenId(
    'portal.dev.trackingUnsupportedManualAndroidBackground'
  ),
  TrackingUnsupportedManualAndroidGeofence: decodeTextTokenId('portal.dev.trackingUnsupportedManualAndroidGeofence'),
  TrackingUnsupportedManualIosBackground: decodeTextTokenId('portal.dev.trackingUnsupportedManualIosBackground'),
  TrackingUnsupportedManualIosGeofence: decodeTextTokenId('portal.dev.trackingUnsupportedManualIosGeofence'),
  TrackingUnsupportedManualDesktopOs: decodeTextTokenId('portal.dev.trackingUnsupportedManualDesktopOs'),
  TrackingUnsupportedManualWebChildAgent: decodeTextTokenId('portal.dev.trackingUnsupportedManualWebChildAgent'),
  TrackingUnsupportedManualAuthorityHardControl: decodeTextTokenId(
    'portal.dev.trackingUnsupportedManualAuthorityHardControl'
  ),
  TrackingUnsupportedManualBoundary: decodeTextTokenId('portal.dev.trackingUnsupportedManualBoundary'),
  TrackingSupportManualRequired: decodeTextTokenId('portal.dev.trackingSupportManualRequired'),
  TrackingSupportPlatformUnsupported: decodeTextTokenId('portal.dev.trackingSupportPlatformUnsupported'),
  TrackingSupportRealDeviceRequired: decodeTextTokenId('portal.dev.trackingSupportRealDeviceRequired'),
  TrackingRenderedManualRequired: decodeTextTokenId('portal.dev.trackingRenderedManualRequired'),
  TrackingRenderedUnavailable: decodeTextTokenId('portal.dev.trackingRenderedUnavailable'),
  TrackingRenderedAuthorityRequired: decodeTextTokenId('portal.dev.trackingRenderedAuthorityRequired'),
  TrackingStateTemporaryLive: decodeTextTokenId('portal.dev.trackingStateTemporaryLive'),
  TrackingStateMissingDevice: decodeTextTokenId('portal.dev.trackingStateMissingDevice'),
  TrackingStateRetentionDeleted: decodeTextTokenId('portal.dev.trackingStateRetentionDeleted'),
  TrackingRetentionHistoryHidden: decodeTextTokenId('portal.dev.trackingRetentionHistoryHidden'),
  TrackingDeletedEvidenceNotRendered: decodeTextTokenId('portal.dev.trackingDeletedEvidenceNotRendered'),
  TrackingEvidenceContracts: decodeTextTokenId('portal.dev.trackingEvidenceContracts'),
  TrackingEvidenceUiFixture: decodeTextTokenId('portal.dev.trackingEvidenceUiFixture'),
  TrackingEvidencePhysicalMissing: decodeTextTokenId('portal.dev.trackingEvidencePhysicalMissing'),
  LiveActivity: decodeTextTokenId('portal.dev.liveActivity'),
  NoActivityStatus: decodeTextTokenId('portal.dev.noActivityStatus'),
  NoBrowserEvidence: decodeTextTokenId('portal.dev.noBrowserEvidence'),
  NoBrowserIntervention: decodeTextTokenId('portal.dev.noBrowserIntervention'),
  NoBrowserManagedStatus: decodeTextTokenId('portal.dev.noBrowserManagedStatus'),
  NoActivityMemoryGraph: decodeTextTokenId('portal.dev.noActivityMemoryGraph'),
  NoDevLog: decodeTextTokenId('portal.dev.noDevLog'),
  NoEvents: decodeTextTokenId('portal.dev.noEvents'),
  NoLocalAiRuntimeStatus: decodeTextTokenId('portal.dev.noLocalAiRuntimeStatus'),
  NoNetworkFlow: decodeTextTokenId('portal.dev.noNetworkFlow'),
  NoPolicyPreview: decodeTextTokenId('portal.dev.noPolicyPreview'),
  PolicyPreviewNoEnforcement: decodeTextTokenId('portal.dev.policyPreviewNoEnforcement'),
  NoRecentActivity: decodeTextTokenId('portal.dev.noRecentActivity'),
  NotReported: decodeTextTokenId('portal.dev.notReported'),
  RecentActivity: decodeTextTokenId('portal.dev.recentActivity'),
  CommandResult: decodeTextTokenId('portal.dev.commandResult'),
  CopyDiagnostics: decodeTextTokenId('portal.dev.copyDiagnostics'),
  CopiedDiagnostics: decodeTextTokenId('portal.dev.copiedDiagnostics'),
  CopyDiagnosticsFailed: decodeTextTokenId('portal.dev.copyDiagnosticsFailed'),
  CopyResult: decodeTextTokenId('portal.dev.copyResult'),
  CopiedResult: decodeTextTokenId('portal.dev.copiedResult'),
  CopyResultFailed: decodeTextTokenId('portal.dev.copyResultFailed'),
  NoCommandResult: decodeTextTokenId('portal.dev.noCommandResult'),
  LatestSnapshot: decodeTextTokenId('portal.dev.latestSnapshot'),
  CheckHealth: decodeTextTokenId('portal.dev.command.checkHealth'),
  GetLogSnapshot: decodeTextTokenId('portal.dev.command.getLogSnapshot'),
  EchoPortalPing: decodeTextTokenId('portal.dev.command.echoPortalPing'),
  GetWatcherStatus: decodeTextTokenId('portal.dev.command.getWatcherStatus'),
  GetActivityIngestStatus: decodeTextTokenId('portal.dev.command.getActivityIngestStatus'),
  GetRecentActivitySummary: decodeTextTokenId('portal.dev.command.getRecentActivitySummary'),
  GetBrowserEvidenceRecent: decodeTextTokenId('portal.dev.command.getBrowserEvidenceRecent'),
  GetActivityMemoryGraph: decodeTextTokenId('portal.dev.command.getActivityMemoryGraph'),
  GetActivityReportDaily: decodeTextTokenId('portal.dev.command.getActivityReportDaily'),
  GetActivityReportHistory: decodeTextTokenId('portal.dev.command.getActivityReportHistory'),
  GetActivityScreenReadModel: decodeTextTokenId('portal.dev.command.getActivityScreenReadModel'),
  GetActivityAppUseReadModel: decodeTextTokenId('portal.dev.command.getActivityAppUseReadModel'),
  GetActivityBrowserReadModel: decodeTextTokenId('portal.dev.command.getActivityBrowserReadModel'),
  GetActivityGamesReadModel: decodeTextTokenId('portal.dev.command.getActivityGamesReadModel'),
  GetActivityNetworkReadModel: decodeTextTokenId('portal.dev.command.getActivityNetworkReadModel'),
  GetBrowserInterventionReadModel: decodeTextTokenId('portal.dev.command.getBrowserInterventionReadModel'),
  PollManagedBrowserBridge: decodeTextTokenId('portal.dev.command.pollManagedBrowserBridge'),
  GetNetworkFlowReadModel: decodeTextTokenId('portal.dev.command.getNetworkFlowReadModel'),
  GetActivityTrackingReadModel: decodeTextTokenId('portal.dev.command.getActivityTrackingReadModel'),
  GetActivityAppGamePolicyReadinessReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGamePolicyReadinessReadModel'
  ),
  GetLocalAiRuntimeStatus: decodeTextTokenId('portal.dev.command.getLocalAiRuntimeStatus'),
  GetPolicyPreviewReadModel: decodeTextTokenId('portal.dev.command.getPolicyPreviewReadModel'),
  RootMissing: decodeTextTokenId('portal.dev.rootMissing'),
} as const;

export type PortalDevTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

export const PortalDevText: Record<PortalDevTextTokenValue, DisplayText> = {
  [PortalDevTextToken.AppTitle]: decodeDisplayText('Ocentra Parent'),
  [PortalDevTextToken.Subtitle]: decodeDisplayText('Family safety for local child devices'),
  [PortalDevTextToken.Reconnect]: decodeDisplayText('Reconnect'),
  ...PortalProductText,
  [PortalDevTextToken.AgentCommands]: decodeDisplayText('Device controls'),
  [PortalDevTextToken.AgentEvents]: decodeDisplayText('Device audit'),
  [PortalDevTextToken.ActivityTimeline]: decodeDisplayText('Activity timeline'),
  [PortalDevTextToken.DeviceDiagnostics]: decodeDisplayText('Device diagnostics'),
  [PortalDevTextToken.DevLog]: decodeDisplayText('Service log'),
  [PortalDevTextToken.EvidenceStore]: decodeDisplayText('Evidence store'),
  [PortalDevTextToken.BrowserEvidence]: decodeDisplayText('Browser evidence'),
  [PortalDevTextToken.BrowserIntervention]: decodeDisplayText('Browser protection'),
  [PortalDevTextToken.BrowserManagedStatus]: decodeDisplayText('Managed browser'),
  [PortalDevTextToken.ActivityMemoryGraph]: decodeDisplayText('Memory links'),
  [PortalDevTextToken.NetworkFlow]: decodeDisplayText('Network activity'),
  [PortalDevTextToken.PolicyPreview]: decodeDisplayText('Policy decision'),
  [PortalDevTextToken.AppGameNotificationParentSurface]: decodeDisplayText('App/game notification surface'),
  [PortalDevTextToken.AppGameNotificationParentSurfaceBody]: decodeDisplayText(
    'Redacted app/game alert rows show setup and drill-in refs only.'
  ),
  [PortalDevTextToken.AppGameNotificationParentSurfaceNoData]: decodeDisplayText(
    'No app/game notification parent-surface intent has been reported yet.'
  ),
  [PortalDevTextToken.AppGameNotificationParentSurfaceNoRuntimeClaim]: decodeDisplayText(
    'Portal renders intent rows only; provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed.'
  ),
  [PortalDevTextToken.AppGamePolicyReadiness]: decodeDisplayText('App/game policy readiness'),
  [PortalDevTextToken.AppGamePolicyReadinessBody]: decodeDisplayText(
    'Service-backed readiness only; no policy execution or adapter dispatch is claimed.'
  ),
  [PortalDevTextToken.AppGamePolicyReadinessNoData]: decodeDisplayText(
    'No app/game policy readiness read model has been reported yet.'
  ),
  [PortalDevTextToken.AppGamePolicyReadinessNoProductClaim]: decodeDisplayText(
    'Readiness rendering only; policy execution and adapter dispatch are not proved.'
  ),
  [PortalDevTextToken.AppGamePolicyReadinessParserRejected]: decodeDisplayText(
    'Latest policy readiness event did not match the shared parser.'
  ),
  [PortalDevTextToken.TrackingStatusSurface]: decodeDisplayText('Tracking status proof'),
  [PortalDevTextToken.TrackingStatusSurfaceBody]: decodeDisplayText('Location states are fixture proof only.'),
  [PortalDevTextToken.TrackingServiceReadModel]: decodeDisplayText('Service read model'),
  [PortalDevTextToken.TrackingServiceDataCoverage]: decodeDisplayText('Service data coverage'),
  [PortalDevTextToken.TrackingEvidenceDrawerHostedUi]: decodeDisplayText('Evidence drawer proof'),
  [PortalDevTextToken.TrackingEvidenceDrawerHostedUiBody]: decodeDisplayText(
    'Hosted route renders a read-only evidence drawer from the selected service-backed citation without evaluating policy or dispatching actions.'
  ),
  [PortalDevTextToken.TrackingEvidenceDrawerReadOnly]: decodeDisplayText('read-only evidence drawer'),
  [PortalDevTextToken.TrackingEvidenceDrawerBoundary]: decodeDisplayText(
    'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingFamilyDashboardRollup]: decodeDisplayText('Family dashboard tracking rollup'),
  [PortalDevTextToken.TrackingFamilyDashboardRollupBody]: decodeDisplayText(
    'Hosted route renders family active, child attention, and retention audit rollups from existing tracking proof refs without claiming device delivery.'
  ),
  [PortalDevTextToken.TrackingFamilyDashboardActiveSummary]: decodeDisplayText('Family active summary'),
  [PortalDevTextToken.TrackingFamilyDashboardChildAttention]: decodeDisplayText('Child attention summary'),
  [PortalDevTextToken.TrackingFamilyDashboardRetentionAudit]: decodeDisplayText('Retention audit summary'),
  [PortalDevTextToken.TrackingFamilyDashboardRollupReady]: decodeDisplayText('rollup-ready'),
  [PortalDevTextToken.TrackingFamilyDashboardActiveEvidence]: decodeDisplayText(
    'tracking-family-dashboard-evidence-active-summary'
  ),
  [PortalDevTextToken.TrackingFamilyDashboardChildAttentionEvidence]: decodeDisplayText(
    'tracking-family-dashboard-evidence-child-attention'
  ),
  [PortalDevTextToken.TrackingFamilyDashboardRetentionAuditEvidence]: decodeDisplayText(
    'tracking-family-dashboard-evidence-retention-audit'
  ),
  [PortalDevTextToken.TrackingFamilyDashboardHostedBoundary]: decodeDisplayText(
    'Hosted dashboard rollup rendering only; child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsHostedUi]: decodeDisplayText('Retention settings read-model UI'),
  [PortalDevTextToken.TrackingRetentionSettingsHostedUiBody]: decodeDisplayText(
    'Hosted route renders existing retention settings read-model rows without writable settings or service mutation.'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsWindow]: decodeDisplayText('Retention window setting'),
  [PortalDevTextToken.TrackingRetentionSettingsDeleteAfterAlert]: decodeDisplayText('Delete-after-alert setting'),
  [PortalDevTextToken.TrackingRetentionSettingsParentExport]: decodeDisplayText('Parent export setting'),
  [PortalDevTextToken.TrackingRetentionSettingsRemoteSyncDisabled]: decodeDisplayText('Remote sync disabled setting'),
  [PortalDevTextToken.TrackingRetentionSettingsRemoteAiDisabled]: decodeDisplayText('Remote AI disabled setting'),
  [PortalDevTextToken.TrackingRetentionSettingsReadModelReady]: decodeDisplayText('settings-read-model-ready'),
  [PortalDevTextToken.TrackingRetentionSettingsWindowEvidence]: decodeDisplayText(
    'tracking-retention-settings-evidence-window'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsDeleteAfterAlertEvidence]: decodeDisplayText(
    'tracking-retention-settings-evidence-delete-after-alert'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsParentExportEvidence]: decodeDisplayText(
    'tracking-retention-settings-evidence-parent-export'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsRemoteSyncEvidence]: decodeDisplayText(
    'tracking-retention-settings-evidence-remote-sync-disabled'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsRemoteAiEvidence]: decodeDisplayText(
    'tracking-retention-settings-evidence-remote-ai-disabled'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsHostedBoundary]: decodeDisplayText(
    'Hosted retention settings rendering only; writable product settings, service mutation, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingFirstTarget]: decodeDisplayText('First target'),
  [PortalDevTextToken.TrackingProofFixture]: decodeDisplayText('P1 fixture proof'),
  [PortalDevTextToken.TrackingProofService]: decodeDisplayText('P2 service proof'),
  [PortalDevTextToken.TrackingManualRequired]: decodeDisplayText('Manual proof required'),
  [PortalDevTextToken.TrackingPhysicalDeviceRequired]: decodeDisplayText('Physical device proof required'),
  [PortalDevTextToken.TrackingNoProductClaim]: decodeDisplayText('No product claim'),
  [PortalDevTextToken.TrackingStateDisabled]: decodeDisplayText('Tracking off'),
  [PortalDevTextToken.TrackingStatePermissionRequired]: decodeDisplayText('Permission required'),
  [PortalDevTextToken.TrackingStateStale]: decodeDisplayText('Stale last known'),
  [PortalDevTextToken.TrackingStateOffline]: decodeDisplayText('Offline last known'),
  [PortalDevTextToken.TrackingStateLowAccuracy]: decodeDisplayText('Low accuracy'),
  [PortalDevTextToken.TrackingStateAmbiguousNearby]: decodeDisplayText('Nearby place ambiguous'),
  [PortalDevTextToken.TrackingStateAlert]: decodeDisplayText('Policy alert'),
  [PortalDevTextToken.TrackingStateAcknowledged]: decodeDisplayText('Parent acknowledged'),
  [PortalDevTextToken.TrackingStateException]: decodeDisplayText('Exception active'),
  [PortalDevTextToken.TrackingStateChildCheckIn]: decodeDisplayText('Child check-in'),
  [PortalDevTextToken.TrackingChildCheckInProofTitle]: decodeDisplayText('Child check-in request'),
  [PortalDevTextToken.TrackingChildCheckInProofBody]: decodeDisplayText(
    'Your parent is asking you to check in. Are you safe?'
  ),
  [PortalDevTextToken.TrackingChildCheckInSafeAction]: decodeDisplayText("I'm safe"),
  [PortalDevTextToken.TrackingChildCheckInHelpAction]: decodeDisplayText('Need help'),
  [PortalDevTextToken.TrackingChildCheckInShareLocationAction]: decodeDisplayText('Share current location'),
  [PortalDevTextToken.TrackingChildCheckInCallParentAction]: decodeDisplayText('Call parent'),
  [PortalDevTextToken.TrackingChildCheckInDeliveryBoundary]: decodeDisplayText('Child-device delivery not proved'),
  [PortalDevTextToken.TrackingChildCheckInCopyBoundary]: decodeDisplayText('Calm copy, no accusation'),
  [PortalDevTextToken.TrackingChildRuntimeUiProofTitle]: decodeDisplayText('Child runtime UI proof'),
  [PortalDevTextToken.TrackingChildRuntimeUiProofBody]: decodeDisplayText(
    'Child sees a clear tracking request, safe response, help response, and location-share consent copy.'
  ),
  [PortalDevTextToken.TrackingChildRuntimeDisclosure]: decodeDisplayText('Tracking request disclosed'),
  [PortalDevTextToken.TrackingChildRuntimeSafeResponse]: decodeDisplayText('Safe response visible'),
  [PortalDevTextToken.TrackingChildRuntimeHelpResponse]: decodeDisplayText('Help response visible'),
  [PortalDevTextToken.TrackingChildRuntimeLocationConsent]: decodeDisplayText('Location share asks consent'),
  [PortalDevTextToken.TrackingChildRuntimeBoundary]: decodeDisplayText('Hosted proof only, not child-agent delivery'),
  [PortalDevTextToken.TrackingUnsupportedManualProofTitle]: decodeDisplayText(
    'Unsupported/manual tracking platform proof'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualProofBody]: decodeDisplayText(
    'Unsupported platform and manual-required adapter rows render as degraded states without invented capability.'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualAndroidBackground]: decodeDisplayText(
    'Android background location manual required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualAndroidGeofence]: decodeDisplayText(
    'Android geofence transition manual required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualIosBackground]: decodeDisplayText(
    'iOS background location manual required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualIosGeofence]: decodeDisplayText(
    'iOS geofence transition manual required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualDesktopOs]: decodeDisplayText(
    'Windows desktop OS location manual required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualWebChildAgent]: decodeDisplayText(
    'Web child agent location unavailable'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualAuthorityHardControl]: decodeDisplayText(
    'Authority hard-control proof required'
  ),
  [PortalDevTextToken.TrackingUnsupportedManualBoundary]: decodeDisplayText(
    'Hosted render-state proof only; physical-device, authority, provider delivery, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingSupportManualRequired]: decodeDisplayText('manual-required'),
  [PortalDevTextToken.TrackingSupportPlatformUnsupported]: decodeDisplayText('platform-unsupported'),
  [PortalDevTextToken.TrackingSupportRealDeviceRequired]: decodeDisplayText('real-device-required'),
  [PortalDevTextToken.TrackingRenderedManualRequired]: decodeDisplayText('manual-required'),
  [PortalDevTextToken.TrackingRenderedUnavailable]: decodeDisplayText('unavailable'),
  [PortalDevTextToken.TrackingRenderedAuthorityRequired]: decodeDisplayText('authority-required'),
  [PortalDevTextToken.TrackingStateTemporaryLive]: decodeDisplayText('Temporary live'),
  [PortalDevTextToken.TrackingStateMissingDevice]: decodeDisplayText('Missing device'),
  [PortalDevTextToken.TrackingStateRetentionDeleted]: decodeDisplayText('Retention deleted'),
  [PortalDevTextToken.TrackingRetentionHistoryHidden]: decodeDisplayText('Deleted history hidden'),
  [PortalDevTextToken.TrackingDeletedEvidenceNotRendered]: decodeDisplayText('Deleted evidence not rendered'),
  [PortalDevTextToken.TrackingEvidenceContracts]: decodeDisplayText('Contract/runtime proof'),
  [PortalDevTextToken.TrackingEvidenceUiFixture]: decodeDisplayText('UI fixture proof'),
  [PortalDevTextToken.TrackingEvidencePhysicalMissing]: decodeDisplayText('Physical artifact missing'),
  [PortalDevTextToken.LiveActivity]: decodeDisplayText('Live activity'),
  [PortalDevTextToken.NoActivityStatus]: decodeDisplayText('Activity status has not been reported yet.'),
  [PortalDevTextToken.NoBrowserEvidence]: decodeDisplayText('No web evidence is available yet.'),
  [PortalDevTextToken.NoBrowserIntervention]: decodeDisplayText('No browser protection decision is available yet.'),
  [PortalDevTextToken.NoBrowserManagedStatus]: decodeDisplayText('Managed browser status has not been reported yet.'),
  [PortalDevTextToken.NoActivityMemoryGraph]: decodeDisplayText('No evidence-cited memory links are available yet.'),
  [PortalDevTextToken.NoDevLog]: decodeDisplayText('No service log snapshot has been reported yet.'),
  [PortalDevTextToken.NoEvents]: decodeDisplayText('No audit entries are available yet.'),
  [PortalDevTextToken.NoLocalAiRuntimeStatus]: decodeDisplayText('Local AI status has not been reported yet.'),
  [PortalDevTextToken.NoNetworkFlow]: decodeDisplayText('No network activity is available yet.'),
  [PortalDevTextToken.NoPolicyPreview]: decodeDisplayText('No policy decision has been reported yet.'),
  [PortalDevTextToken.PolicyPreviewNoEnforcement]: decodeDisplayText('Protection mode: advisory.'),
  [PortalDevTextToken.NoRecentActivity]: decodeDisplayText('No recent activity is available yet.'),
  [PortalDevTextToken.NotReported]: decodeDisplayText('Not reported'),
  [PortalDevTextToken.RecentActivity]: decodeDisplayText('Recent activity'),
  [PortalDevTextToken.CommandResult]: decodeDisplayText('Command result'),
  [PortalDevTextToken.CopyDiagnostics]: decodeDisplayText('Copy diagnostics'),
  [PortalDevTextToken.CopiedDiagnostics]: decodeDisplayText('Diagnostics copied'),
  [PortalDevTextToken.CopyDiagnosticsFailed]: decodeDisplayText('Diagnostics copy failed'),
  [PortalDevTextToken.CopyResult]: decodeDisplayText('Copy result'),
  [PortalDevTextToken.CopiedResult]: decodeDisplayText('Copied'),
  [PortalDevTextToken.CopyResultFailed]: decodeDisplayText('Copy failed'),
  [PortalDevTextToken.NoCommandResult]: decodeDisplayText('Choose a device control to see the latest response.'),
  [PortalDevTextToken.LatestSnapshot]: decodeDisplayText('Latest device snapshot'),
  [PortalDevTextToken.CheckHealth]: decodeDisplayText('Check health'),
  [PortalDevTextToken.GetLogSnapshot]: decodeDisplayText('Get log snapshot'),
  [PortalDevTextToken.EchoPortalPing]: decodeDisplayText('Send connectivity check'),
  [PortalDevTextToken.GetWatcherStatus]: decodeDisplayText('Refresh browser watcher'),
  [PortalDevTextToken.GetActivityIngestStatus]: decodeDisplayText('Refresh activity ingest'),
  [PortalDevTextToken.GetRecentActivitySummary]: decodeDisplayText('Refresh recent activity'),
  [PortalDevTextToken.GetBrowserEvidenceRecent]: decodeDisplayText('Refresh web evidence'),
  [PortalDevTextToken.GetActivityMemoryGraph]: decodeDisplayText('Refresh memory links'),
  [PortalDevTextToken.GetActivityReportDaily]: decodeDisplayText('Build daily activity report'),
  [PortalDevTextToken.GetActivityReportHistory]: decodeDisplayText('Refresh activity report history'),
  [PortalDevTextToken.GetActivityScreenReadModel]: decodeDisplayText('Refresh activity screen'),
  [PortalDevTextToken.GetActivityAppUseReadModel]: decodeDisplayText('Refresh activity app use'),
  [PortalDevTextToken.GetActivityBrowserReadModel]: decodeDisplayText('Refresh activity browser'),
  [PortalDevTextToken.GetActivityGamesReadModel]: decodeDisplayText('Refresh activity games'),
  [PortalDevTextToken.GetActivityNetworkReadModel]: decodeDisplayText('Refresh activity network'),
  [PortalDevTextToken.GetBrowserInterventionReadModel]: decodeDisplayText('Refresh browser protection'),
  [PortalDevTextToken.PollManagedBrowserBridge]: decodeDisplayText('Refresh managed browser'),
  [PortalDevTextToken.GetNetworkFlowReadModel]: decodeDisplayText('Refresh network activity'),
  [PortalDevTextToken.GetActivityTrackingReadModel]: decodeDisplayText('Refresh tracking status'),
  [PortalDevTextToken.GetActivityAppGamePolicyReadinessReadModel]: decodeDisplayText('Refresh policy readiness'),
  [PortalDevTextToken.GetLocalAiRuntimeStatus]: decodeDisplayText('Refresh local AI'),
  [PortalDevTextToken.GetPolicyPreviewReadModel]: decodeDisplayText('Refresh policy decision'),
  [PortalDevTextToken.RootMissing]: decodeDisplayText('Portal root element is missing.'),
};

const MissingPortalDevTextTokenMessage = decodeDisplayText('Missing portal dev text token.');

export function resolvePortalDevText(token: PortalDevTextTokenValue): DisplayText {
  const text = PortalDevText[token];
  if (text === undefined) {
    throw new Error(MissingPortalDevTextTokenMessage);
  }
  return text;
}
