export type DisplayText = string;
export type PortalDisplayText = DisplayText;

class DisplayTextDecodeError extends Error {
  constructor(input: unknown) {
    super('DisplayText: expected a non-empty string');
    this.name = 'DisplayTextDecodeError';
    this.cause = input;
  }
}

export function decodeDisplayText(input: unknown): DisplayText {
  if (typeof input !== 'string' || input.length === 0) {
    throw new DisplayTextDecodeError(input);
  }
  return input;
}

export const PortalDevTextToken = {
  Activity: decodeDisplayText('Activity'),
  ActivityDescription: decodeDisplayText('Stored activity'),
  AgentCommands: decodeDisplayText('Device controls'),
  AgentEvents: decodeDisplayText('Device audit'),
  AiRuntime: decodeDisplayText('Local AI'),
  AiRuntimeBody: decodeDisplayText(
    'Local AI should explain and summarize when available, while safety decisions remain typed and auditable.'
  ),
  AiRuntimeDescription: decodeDisplayText('Local model privacy'),
  AppGamePolicyReadiness: decodeDisplayText('App/game policy readiness'),
  AppGameSessions: decodeDisplayText('App and game sessions'),
  AppGameTimerParentSurface: decodeDisplayText('App/game timer parent surface'),
  AppTitle: decodeDisplayText('Ocentra Parent'),
  Approvals: decodeDisplayText('Approvals'),
  ApprovalsBody: decodeDisplayText('Route ask-parent moments into a clear approve, deny, or explain outcome.'),
  AuthBody: decodeDisplayText('Use a parent session before changing rules, approvals, drives, or child-device trust.'),
  AuthClose: decodeDisplayText('Close parent sign in'),
  AuthEyebrow: decodeDisplayText('Parent access'),
  AuthTitle: decodeDisplayText('Protect the family console'),
  AuthUnavailable: decodeDisplayText('Parent identity is not connected on this device yet.'),
  BillingEntitlements: decodeDisplayText('Billing and entitlements'),
  Browser: decodeDisplayText('Web'),
  BrowserBlockBody: decodeDisplayText(
    'Turn a supported browser rule into allow, ask-parent, explain-first, schedule-limit, or block behavior.'
  ),
  BrowserControls: decodeDisplayText('Browser controls'),
  BrowserDescription: decodeDisplayText('Browser evidence'),
  BrowserIntervention: decodeDisplayText('Browser protection'),
  BrowserManagedStatus: decodeDisplayText('Managed browser'),
  CheckHealth: decodeDisplayText('Check health'),
  CommandControlsUnavailable: decodeDisplayText('Review unavailable device controls'),
  CommandResult: decodeDisplayText('Command result'),
  CommandServiceUnavailable: decodeDisplayText('Start or reconnect the local service to enable these device controls.'),
  Commands: decodeDisplayText('Controls'),
  CommandsDescription: decodeDisplayText('Safe device refresh actions that use the real child-device connection.'),
  Connected: decodeDisplayText('Child device connected'),
  CopiedDiagnostics: decodeDisplayText('Diagnostics copied'),
  CopiedResult: decodeDisplayText('Copied'),
  CopyDiagnostics: decodeDisplayText('Copy diagnostics'),
  CopyDiagnosticsFailed: decodeDisplayText('Diagnostics copy failed'),
  CopyResult: decodeDisplayText('Copy result'),
  CopyResultFailed: decodeDisplayText('Copy failed'),
  DataCustodyBody: decodeDisplayText(
    'The child device keeps evidence locally. You decide whether diagnostics or exports leave the device.'
  ),
  DataCustodyTitle: decodeDisplayText('Private by design'),
  DesktopApp: decodeDisplayText('Desktop app'),
  DesktopAppBody: decodeDisplayText(
    'The desktop app hosts the child-device service and native capabilities for Windows, macOS, and Linux.'
  ),
  DevLog: decodeDisplayText('Service log'),
  DeviceDiagnostics: decodeDisplayText('Device diagnostics'),
  DeviceInventory: decodeDisplayText('Device inventory'),
  Devices: decodeDisplayText('Devices'),
  DevicesDescription: decodeDisplayText('Device control'),
  Diagnostics: decodeDisplayText('Support'),
  DiagnosticsDescription: decodeDisplayText('Exports and logs'),
  DisplayTheme: decodeDisplayText('Display theme'),
  DriveConnectionsBody: decodeDisplayText(
    'Prepare backups for a parent-owned drive, school archive, or support bundle when you choose.'
  ),
  DriveConnectionsTitle: decodeDisplayText('Connect your drives'),
  EchoPortalPing: decodeDisplayText('Send connectivity check'),
  EmptyAgentEvents: decodeDisplayText('No device audit events have been reported yet.'),
  Events: decodeDisplayText('Audit'),
  EventsDescription: decodeDisplayText('Validated child-device audit entries from the local service.'),
  ExecuteActivityAppGameAdapterDispatch: decodeDisplayText('Execute scoped adapter dispatch'),
  ExportSync: decodeDisplayText('Export and sync'),
  FrameTuner: decodeDisplayText('App layout'),
  FrameTunerDescription: decodeDisplayText('Layout and content editor for parent portal app surfaces.'),
  GetActivityAppGameAdapterDispatchPreflightReadModel: decodeDisplayText('Refresh adapter dispatch preflight'),
  GetActivityAppGameAdapterDispatchResultReadModel: decodeDisplayText('Refresh adapter dispatch result'),
  GetActivityAppGameAdapterExecutionReadinessReadModel: decodeDisplayText('Refresh adapter readiness'),
  GetActivityAppGameChildRuntimeTransportReceiptReadModel: decodeDisplayText('Refresh child runtime receipts'),
  GetActivityAppGamePlatformProofStatusReadModel: decodeDisplayText('Refresh platform proof status'),
  GetActivityAppGamePolicyReadinessReadModel: decodeDisplayText('Refresh policy readiness'),
  GetActivityAppGameTimerParentSurfaceReadModel: decodeDisplayText('Refresh timer parent surface'),
  GetActivityAppUseReadModel: decodeDisplayText('Refresh activity app use'),
  GetActivityBrowserReadModel: decodeDisplayText('Refresh activity browser'),
  GetActivityGamesReadModel: decodeDisplayText('Refresh activity games'),
  GetActivityIngestStatus: decodeDisplayText('Refresh activity ingest'),
  GetActivityMemoryGraph: decodeDisplayText('Refresh memory links'),
  GetActivityNetworkReadModel: decodeDisplayText('Refresh activity network'),
  GetActivityReportDaily: decodeDisplayText('Build daily activity report'),
  GetActivityReportHistory: decodeDisplayText('Refresh activity report history'),
  GetActivityScreenReadModel: decodeDisplayText('Refresh activity screen'),
  GetActivityTrackingReadModel: decodeDisplayText('Refresh tracking status'),
  GetBrowserEvidenceRecent: decodeDisplayText('Refresh web evidence'),
  GetBrowserInterventionReadModel: decodeDisplayText('Refresh browser protection'),
  GetBrowserRuntimeEventChainStream: decodeDisplayText('Refresh browser runtime'),
  GetLanPairingStatus: decodeDisplayText('Refresh LAN pairing status'),
  GetLocalAiRuntimeStatus: decodeDisplayText('Refresh local AI'),
  GetLogSnapshot: decodeDisplayText('Get log snapshot'),
  GetNetworkFlowReadModel: decodeDisplayText('Refresh network activity'),
  GetNetworkLinuxNftablesLabStatus: decodeDisplayText('Refresh Linux nftables lab'),
  GetNetworkLiveCaptureStatus: decodeDisplayText('Refresh live capture'),
  GetNetworkRemoteDeliveryStatus: decodeDisplayText('Refresh remote delivery'),
  GetNetworkRuntimeEventChainStream: decodeDisplayText('Refresh network runtime'),
  GetNetworkWindowsFirewallLabStatus: decodeDisplayText('Refresh Windows firewall lab'),
  GetNetworkWindowsWfpGateStatus: decodeDisplayText('Refresh Windows WFP gate'),
  GetPolicyPreviewReadModel: decodeDisplayText('Refresh policy decision'),
  GetRecentActivitySummary: decodeDisplayText('Refresh recent activity'),
  GetWatcherStatus: decodeDisplayText('Refresh browser watcher'),
  HeaderBrandLeft: decodeDisplayText("O'centra"),
  HeaderBrandRight: decodeDisplayText('Parent'),
  HeaderHome: decodeDisplayText('Home'),
  HeaderLogin: decodeDisplayText('Login'),
  Logs: decodeDisplayText('Logs'),
  LogsDescription: decodeDisplayText('Local portal and agent service log snapshots.'),
  Memory: decodeDisplayText('Memory'),
  MemoryBody: decodeDisplayText(
    'Derived memory links must cite stored evidence, selected policy versions, or parent actions.'
  ),
  MemoryDescription: decodeDisplayText('Cited local memory'),
  NavGroupDevTools: decodeDisplayText('Dev tools'),
  NavGroupGuide: decodeDisplayText('Guide'),
  NavGroupMonitor: decodeDisplayText('Today'),
  NavGroupOperate: decodeDisplayText('Manage'),
  NetworkFlow: decodeDisplayText('Network activity'),
  NoBrowserIntervention: decodeDisplayText('No browser protection decision is available yet.'),
  NoBrowserManagedStatus: decodeDisplayText('Managed browser status has not been reported yet.'),
  NoCommandResult: decodeDisplayText('Choose a device control to see the latest response.'),
  NoDevLog: decodeDisplayText('No service log snapshot has been reported yet.'),
  NoNetworkFlow: decodeDisplayText('No network activity is available yet.'),
  NoRecentActivity: decodeDisplayText('No recent activity is available yet.'),
  NotReported: decodeDisplayText('Not reported'),
  UnknownEvent: decodeDisplayText('unknown-event'),
  Notifications: decodeDisplayText('Notifications'),
  NotificationsBody: decodeDisplayText(
    'Choose which events deserve a parent alert and which should stay in the daily audit.'
  ),
  Overview: decodeDisplayText('Overview'),
  OverviewDescription: decodeDisplayText('Daily command'),
  Pairing: decodeDisplayText('Pairing'),
  PairingBody: decodeDisplayText(
    'Pair desktop and mobile apps with a parent-owned local trust step before controls are enabled.'
  ),
  ParentPortal: decodeDisplayText('Start here'),
  ParentPortalDescription: decodeDisplayText('Setup and controls map'),
  PendingServiceReadModel: decodeDisplayText('Not connected'),
  PendingTypedIntent: decodeDisplayText('Setup'),
  Policy: decodeDisplayText('Policy'),
  PolicyDescription: decodeDisplayText('Rules and approvals'),
  PolicyModeActive: decodeDisplayText('Active'),
  PolicyPreview: decodeDisplayText('Policy decision'),
  PolicyPreviewNoEnforcement: decodeDisplayText('Protection mode: advisory.'),
  PollManagedBrowserBridge: decodeDisplayText('Refresh managed browser'),
  ProductSurfacePending: decodeDisplayText('No family setting is configured for this area yet.'),
  ProofPanels: decodeDisplayText('Proof panels'),
  ProofPanelsDescription: decodeDisplayText('Tracking, network, and policy proof panels.'),
  RetryStatus: decodeDisplayText('Retry status'),
  RemoteScreen: decodeDisplayText('Remote screen'),
  RootMissing: decodeDisplayText('Portal root element is missing.'),
  RuleBuilder: decodeDisplayText('Rule builder'),
  RuleBuilderBody: decodeDisplayText(
    'Create family rules for web, apps, games, schedules, local AI explanations, and exception requests.'
  ),
  SchedulesBudgets: decodeDisplayText('Schedules and budgets'),
  SchedulesBudgetsBody: decodeDisplayText(
    'Set school, sleep, homework, and weekend windows without hiding what happened outside the window.'
  ),
  ScreenAnalysis: decodeDisplayText('Screen analysis'),
  SettingsRules: decodeDisplayText('Settings'),
  SettingsRulesDescription: decodeDisplayText('Portal settings, alerts, and channels'),
  ThemeDark: decodeDisplayText('Dark'),
  ThemeLight: decodeDisplayText('Light'),
  TrackingStatusSurface: decodeDisplayText('Tracking status proof'),
  TrackingChildCheckInCallParentAction: decodeDisplayText('Call parent'),
  TrackingChildCheckInCopyBoundary: decodeDisplayText('Calm copy, no accusation'),
  TrackingChildCheckInDeliveryBoundary: decodeDisplayText('Child-device delivery not proved'),
  TrackingChildCheckInHelpAction: decodeDisplayText('Need help'),
  TrackingChildCheckInProofBody: decodeDisplayText('Your parent is asking you to check in. Are you safe?'),
  TrackingChildCheckInProofTitle: decodeDisplayText('Child check-in request'),
  TrackingChildCheckInSafeAction: decodeDisplayText("I'm safe"),
  TrackingChildCheckInShareLocationAction: decodeDisplayText('Share current location'),
  TrackingChildRuntimeBoundary: decodeDisplayText('Hosted proof only, not child-agent delivery'),
  TrackingChildRuntimeDisclosure: decodeDisplayText('Tracking request disclosed'),
  TrackingChildRuntimeHelpResponse: decodeDisplayText('Help response visible'),
  TrackingChildRuntimeLocationConsent: decodeDisplayText('Location share asks consent'),
  TrackingChildRuntimeSafeResponse: decodeDisplayText('Safe response visible'),
  TrackingChildRuntimeUiProofBody: decodeDisplayText(
    'Child sees a clear tracking request, safe response, help response, and location-share consent copy.'
  ),
  TrackingChildRuntimeUiProofTitle: decodeDisplayText('Child runtime UI proof'),
  TrackingDeletedEvidenceNotRendered: decodeDisplayText('Deleted evidence not rendered'),
  TrackingEvidenceContracts: decodeDisplayText('Contract/runtime proof'),
  TrackingEvidenceDrawerBoundary: decodeDisplayText(
    'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  TrackingEvidenceDrawerHostedUi: decodeDisplayText('Evidence drawer proof'),
  TrackingEvidenceDrawerHostedUiBody: decodeDisplayText(
    'Hosted route renders a read-only evidence drawer from the selected service-backed citation without evaluating policy or dispatching actions.'
  ),
  TrackingEvidenceDrawerReadOnly: decodeDisplayText('read-only evidence drawer'),
  TrackingEvidencePhysicalMissing: decodeDisplayText('Physical artifact missing'),
  TrackingEvidenceUiFixture: decodeDisplayText('UI fixture proof'),
  TrackingFamilyDashboardActiveEvidence: decodeDisplayText('tracking-family-dashboard-evidence-active-summary'),
  TrackingFamilyDashboardActiveSummary: decodeDisplayText('Family active summary'),
  TrackingFamilyDashboardChildAttention: decodeDisplayText('Child attention summary'),
  TrackingFamilyDashboardChildAttentionEvidence: decodeDisplayText(
    'tracking-family-dashboard-evidence-child-attention'
  ),
  TrackingFamilyDashboardHostedBoundary: decodeDisplayText(
    'Hosted dashboard rollup rendering only; child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  TrackingFamilyDashboardRetentionAudit: decodeDisplayText('Retention audit summary'),
  TrackingFamilyDashboardRetentionAuditEvidence: decodeDisplayText(
    'tracking-family-dashboard-evidence-retention-audit'
  ),
  TrackingFamilyDashboardRollup: decodeDisplayText('Family dashboard tracking rollup'),
  TrackingFamilyDashboardRollupBody: decodeDisplayText(
    'Hosted route renders family active, child attention, and retention audit rollups from existing tracking proof refs without claiming device delivery.'
  ),
  TrackingFamilyDashboardRollupReady: decodeDisplayText('rollup-ready'),
  TrackingManualRequired: decodeDisplayText('Manual proof required'),
  TrackingMissingDeviceCallMarkFoundAction: decodeDisplayText('review-last-known | call-child | mark-found'),
  TrackingMissingDeviceContactRequested: decodeDisplayText('Contact requested state'),
  TrackingMissingDeviceContactRequestedBadge: decodeDisplayText('contact-requested'),
  TrackingMissingDeviceContactRequestedEvidence: decodeDisplayText('location-evidence-last-known-contact-requested'),
  TrackingMissingDeviceContactRequestedState: decodeDisplayText('contact-requested'),
  TrackingMissingDeviceContactStatusEvidence: decodeDisplayText('device-status-contact-action-queued'),
  TrackingMissingDeviceHostedBoundary: decodeDisplayText(
    'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.'
  ),
  TrackingMissingDeviceHostedReadOnlyManualProof: decodeDisplayText('hosted-read-only-missing-device-proof'),
  TrackingMissingDeviceHostedUi: decodeDisplayText('Missing-device state UI'),
  TrackingMissingDeviceHostedUiBody: decodeDisplayText(
    'Hosted route renders last-known, offline, contact-requested, and manual-required missing-device rows from existing WP29 proof without claiming current location or OS lost-mode runtime.'
  ),
  TrackingMissingDeviceLastKnownBadge: decodeDisplayText('last-known'),
  TrackingMissingDeviceLastKnownEvidence: decodeDisplayText('location-evidence-last-known-stale'),
  TrackingMissingDeviceLastKnownOnly: decodeDisplayText('Last-known only state'),
  TrackingMissingDeviceLastKnownState: decodeDisplayText('last-known-only'),
  TrackingMissingDeviceManualEvidence: decodeDisplayText('location-evidence-last-known-manual-required'),
  TrackingMissingDeviceManualPlatformAction: decodeDisplayText('review-last-known | manual-platform-proof'),
  TrackingMissingDeviceManualRequired: decodeDisplayText('Manual platform proof state'),
  TrackingMissingDeviceManualRequiredBadge: decodeDisplayText('manual-required'),
  TrackingMissingDeviceManualRequiredState: decodeDisplayText('manual-required'),
  TrackingMissingDeviceOfflineBadge: decodeDisplayText('offline'),
  TrackingMissingDeviceOfflineContact: decodeDisplayText('contact-state-offline'),
  TrackingMissingDeviceOfflineState: decodeDisplayText('offline'),
  TrackingMissingDeviceOfflineStatusEvidence: decodeDisplayText('device-status-offline-last-known'),
  TrackingMissingDeviceOnlineContact: decodeDisplayText('contact-state-online'),
  TrackingMissingDevicePlatformManualProof: decodeDisplayText(
    'os-lost-mode-api-proof-required | physical-device-proof-required'
  ),
  TrackingMissingDevicePlatformProofEvidence: decodeDisplayText('device-status-platform-proof-required'),
  TrackingMissingDevicePoweredOff: decodeDisplayText('Powered-off offline state'),
  TrackingMissingDevicePoweredOffContact: decodeDisplayText('contact-state-powered-off'),
  TrackingMissingDevicePoweredOffEvidence: decodeDisplayText('location-evidence-last-known-powered-off'),
  TrackingMissingDevicePoweredOffManualProof: decodeDisplayText(
    'powered-off-current-location-proof-forbidden | hosted-read-only-missing-device-proof'
  ),
  TrackingMissingDevicePoweredOffStatusEvidence: decodeDisplayText('device-status-powered-off'),
  TrackingMissingDeviceReviewCheckInAction: decodeDisplayText(
    'review-last-known | ask-child-check-in | call-child | mark-found'
  ),
  TrackingMissingDeviceUnknownContact: decodeDisplayText('contact-state-unknown'),
  TrackingNoProductClaim: decodeDisplayText('No product claim'),
  TrackingNotificationParentSurfaceHistoryIntent: decodeDisplayText('Notification history ready'),
  TrackingNotificationParentSurfaceHostedBoundary: decodeDisplayText(
    'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.'
  ),
  TrackingNotificationParentSurfaceHostedUi: decodeDisplayText('Notification history intent UI'),
  TrackingNotificationParentSurfaceHostedUiBody: decodeDisplayText(
    'Hosted route renders parent notification history, manual action, and provider unavailable rows from existing tracking notification proof refs without claiming provider delivery or receipt runtime.'
  ),
  TrackingNotificationParentSurfaceManualAction: decodeDisplayText('Manual notification action required'),
  TrackingNotificationParentSurfaceProviderUnavailable: decodeDisplayText('Notification provider unavailable'),
  TrackingParentActionAcknowledgeSafe: decodeDisplayText('acknowledge-safe'),
  TrackingParentActionAcknowledgementRecorded: decodeDisplayText('Parent acknowledgement recorded'),
  TrackingParentActionAcknowledgementRecordedStatus: decodeDisplayText('acknowledgement-recorded'),
  TrackingParentActionAlertPolicyReady: decodeDisplayText('alert-policy-ready'),
  TrackingParentActionAskChildCheckIn: decodeDisplayText('ask-child-check-in'),
  TrackingParentActionCheckInPolicyReady: decodeDisplayText('check-in-policy-ready'),
  TrackingParentActionChildCheckInDecision: decodeDisplayText('tracking-decision-check-in'),
  TrackingParentActionChildCheckInEvidence: decodeDisplayText('tracking-parent-action-evidence-4'),
  TrackingParentActionChildCheckInReady: decodeDisplayText('Child check-in action ready'),
  TrackingParentActionChildCheckInRequestReady: decodeDisplayText('child-check-in-request-ready'),
  TrackingParentActionChildCheckInSurface: decodeDisplayText('tracking-parent-action-surface-tracking-alert-check-in'),
  TrackingParentActionChildRuntimeManualProof: decodeDisplayText(
    'child-device-runtime-proof-required | rendered-portal-acknowledgement-ui-proof-required'
  ),
  TrackingParentActionCriticalReviewDecision: decodeDisplayText('tracking-decision-critical-review'),
  TrackingParentActionCriticalReviewEvidence: decodeDisplayText('tracking-parent-action-evidence-5'),
  TrackingParentActionCriticalReviewReady: decodeDisplayText('Critical escalation review ready'),
  TrackingParentActionCriticalReviewSurface: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-critical-review'
  ),
  TrackingParentActionEscalateManualReview: decodeDisplayText('escalate-manual-review'),
  TrackingParentActionEscalationManualProof: decodeDisplayText(
    'critical-escalation-runtime-proof-required | second-guardian-provider-proof-required'
  ),
  TrackingParentActionEscalationReviewReady: decodeDisplayText('escalation-review-ready'),
  TrackingParentActionExceptionActive: decodeDisplayText('Expected exception active'),
  TrackingParentActionExceptionActiveStatus: decodeDisplayText('exception-active'),
  TrackingParentActionExpectedDecision: decodeDisplayText('tracking-decision-expected'),
  TrackingParentActionExpectedEvidence: decodeDisplayText('tracking-parent-action-evidence-2'),
  TrackingParentActionExpectedPlaceAlert: decodeDisplayText('Expected-place parent alert ready'),
  TrackingParentActionExpectedPlaceCheckIn: decodeDisplayText('Expected-place child check-in ready'),
  TrackingParentActionExpectedPlaceHolidayDecision: decodeDisplayText('expected-place-decision-holiday'),
  TrackingParentActionExpectedPlaceHolidayEvidence: decodeDisplayText('expected-place-evidence-holiday'),
  TrackingParentActionExpectedPlaceHolidaySurface: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-holiday'
  ),
  TrackingParentActionExpectedPlaceLateBusDecision: decodeDisplayText('expected-place-decision-late-bus'),
  TrackingParentActionExpectedPlaceLateBusEvidence: decodeDisplayText('expected-place-evidence-late-bus'),
  TrackingParentActionExpectedPlaceLateBusSurface: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-late-bus'
  ),
  TrackingParentActionExpectedPlaceLowAccuracyDecision: decodeDisplayText('expected-place-decision-low-accuracy'),
  TrackingParentActionExpectedPlaceLowAccuracyEvidence: decodeDisplayText('expected-place-evidence-low-accuracy'),
  TrackingParentActionExpectedPlaceLowAccuracySurface: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-low-accuracy'
  ),
  TrackingParentActionExpectedPlaceManual: decodeDisplayText('Expected-place manual review required'),
  TrackingParentActionExpectedPlaceManualProof: decodeDisplayText(
    'tracking-expected-place-manual-proof-expected-place-decision-low-accuracy'
  ),
  TrackingParentActionExpectedPlaceSchoolDecision: decodeDisplayText('expected-place-decision-school'),
  TrackingParentActionExpectedPlaceSchoolEvidence: decodeDisplayText('expected-place-evidence-school-arrival'),
  TrackingParentActionExpectedPlaceSchoolSurface: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-school'
  ),
  TrackingParentActionExpectedPlaceSuppressed: decodeDisplayText('Expected-place suppressed no action'),
  TrackingParentActionExpectedSurface: decodeDisplayText('tracking-parent-action-surface-tracking-alert-expected'),
  TrackingParentActionFalseAlarmDecision: decodeDisplayText('tracking-decision-false-alarm'),
  TrackingParentActionFalseAlarmEvidence: decodeDisplayText('tracking-parent-action-evidence-3'),
  TrackingParentActionFalseAlarmRecorded: decodeDisplayText('False alarm recorded'),
  TrackingParentActionFalseAlarmRecordedStatus: decodeDisplayText('false-alarm-recorded'),
  TrackingParentActionFalseAlarmSurface: decodeDisplayText('tracking-parent-action-surface-tracking-alert-false-alarm'),
  TrackingParentActionHostedReadOnlyManualProof: decodeDisplayText('hosted-read-only-parent-action-proof'),
  TrackingParentActionManualRequired: decodeDisplayText('manual-required'),
  TrackingParentActionManualReview: decodeDisplayText('manual-review'),
  TrackingParentActionMarkExpected: decodeDisplayText('mark-expected'),
  TrackingParentActionMarkFalseAlarm: decodeDisplayText('mark-false-alarm'),
  TrackingParentActionNoAction: decodeDisplayText('no-action'),
  TrackingParentActionNotifyParent: decodeDisplayText('notify-parent'),
  TrackingParentActionReadinessHostedBoundary: decodeDisplayText(
    'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.'
  ),
  TrackingParentActionReadinessHostedUi: decodeDisplayText('Parent action readiness UI'),
  TrackingParentActionReadinessHostedUiBody: decodeDisplayText(
    'Hosted route renders expected-place alert policy and parent acknowledgement action readiness rows from existing tracking proof refs without claiming live mutation or delivery runtime.'
  ),
  TrackingParentActionRequestChildCheckIn: decodeDisplayText('request-child-check-in'),
  TrackingParentActionSafeDecision: decodeDisplayText('tracking-decision-safe'),
  TrackingParentActionSafeEvidence: decodeDisplayText('tracking-parent-action-evidence-1'),
  TrackingParentActionSafeSurface: decodeDisplayText('tracking-parent-action-surface-tracking-alert-safe'),
  TrackingParentActionServiceMutationManualProof: decodeDisplayText(
    'live-service-mutation-proof-required | rendered-portal-acknowledgement-ui-proof-required'
  ),
  TrackingParentActionSuppressedNoAction: decodeDisplayText('suppressed-no-action'),
  TrackingPhysicalDeviceRequired: decodeDisplayText('Physical device proof required'),
  TrackingProofFixture: decodeDisplayText('P1 fixture proof'),
  TrackingProofService: decodeDisplayText('P2 service proof'),
  TrackingRenderedAuthorityRequired: decodeDisplayText('authority-required'),
  TrackingRenderedManualRequired: decodeDisplayText('manual-required'),
  TrackingRenderedUnavailable: decodeDisplayText('unavailable'),
  TrackingReportExportFamilySummary: decodeDisplayText('Family dashboard summary packet'),
  TrackingReportExportFamilySummaryEvidence: decodeDisplayText('tracking-report-export-evidence-family-dashboard'),
  TrackingReportExportHostedBoundary: decodeDisplayText(
    'Hosted report/export packet rendering only; raw location payload export, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  TrackingReportExportHostedUi: decodeDisplayText('Report export read-model UI'),
  TrackingReportExportHostedUiBody: decodeDisplayText(
    'Hosted route renders redacted report/export packet rows from existing read-model proof refs without exposing raw location payloads or claiming product-ready export.'
  ),
  TrackingReportExportLocalCustody: decodeDisplayText('parent-owned-local-export'),
  TrackingReportExportPolicyDrillIn: decodeDisplayText('Policy drill-in export packet'),
  TrackingReportExportPolicyDrillInEvidence: decodeDisplayText('tracking-report-export-evidence-policy-drill-in'),
  TrackingReportExportReadModelReady: decodeDisplayText('report-export-read-model-ready'),
  TrackingReportExportRedactedCustody: decodeDisplayText('parent-owned-redacted-report'),
  TrackingReportExportRedactedReport: decodeDisplayText('Redacted report packet'),
  TrackingReportExportRedactedReportEvidence: decodeDisplayText('tracking-report-export-evidence-redacted-report'),
  TrackingReportExportRetentionAudit: decodeDisplayText('Retention audit export packet'),
  TrackingReportExportRetentionAuditEvidence: decodeDisplayText('tracking-report-export-evidence-retention-audit'),
  TrackingReportPolicyConsumerHostedBoundary: decodeDisplayText(
    'Hosted report/policy consumer rendering only; AI execution, product policy mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, production, and product readiness remain unclaimed.'
  ),
  TrackingReportPolicyConsumerHostedUi: decodeDisplayText('Report policy consumer UI'),
  TrackingReportPolicyConsumerHostedUiBody: decodeDisplayText(
    'Hosted route renders parent report summary, policy drill-in, and retention audit consumer rows from stored journal/read-model refs without claiming product-ready report or policy execution.'
  ),
  TrackingReportPolicyConsumerParentReport: decodeDisplayText('Parent report summary consumer'),
  TrackingReportPolicyConsumerPolicyDrillIn: decodeDisplayText('Policy evidence drill-in consumer'),
  TrackingReportPolicyConsumerPolicyEvidence: decodeDisplayText('tracking-report-policy-evidence-decision'),
  TrackingReportPolicyConsumerPolicyJournal: decodeDisplayText('tracking-journal-row-policy-drill-in'),
  TrackingReportPolicyConsumerPolicyReadModel: decodeDisplayText('tracking-read-model-row-policy-drill-in'),
  TrackingReportPolicyConsumerPolicySurface: decodeDisplayText('parent-policy-evidence-drill-in-row'),
  TrackingReportPolicyConsumerReady: decodeDisplayText('consumer-ready'),
  TrackingReportPolicyConsumerReportEvidence: decodeDisplayText('tracking-report-policy-evidence-summary'),
  TrackingReportPolicyConsumerReportJournal: decodeDisplayText('tracking-journal-row-report-summary'),
  TrackingReportPolicyConsumerReportReadModel: decodeDisplayText('tracking-read-model-row-report-summary'),
  TrackingReportPolicyConsumerReportSurface: decodeDisplayText('parent-report-location-summary-row'),
  TrackingReportPolicyConsumerRetentionAudit: decodeDisplayText('Retention audit export consumer'),
  TrackingReportPolicyConsumerRetentionEvidence: decodeDisplayText('tracking-report-policy-evidence-retention'),
  TrackingReportPolicyConsumerRetentionJournal: decodeDisplayText('tracking-journal-row-retention-export'),
  TrackingReportPolicyConsumerRetentionReadModel: decodeDisplayText('tracking-read-model-row-retention-export'),
  TrackingReportPolicyConsumerRetentionSurface: decodeDisplayText('parent-retention-audit-export-row'),
  TrackingRetentionHistoryHidden: decodeDisplayText('Deleted history hidden'),
  TrackingRetentionSettingsDeleteAfterAlert: decodeDisplayText('Delete-after-alert setting'),
  TrackingRetentionSettingsDeleteAfterAlertEvidence: decodeDisplayText(
    'tracking-retention-settings-evidence-delete-after-alert'
  ),
  TrackingRetentionSettingsHostedBoundary: decodeDisplayText(
    'Hosted retention settings rendering proves local service write execution and durable local persistence only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  TrackingRetentionSettingsHostedUi: decodeDisplayText('Retention settings read-model UI'),
  TrackingRetentionSettingsHostedUiBody: decodeDisplayText(
    'Hosted route renders existing retention settings read-model rows and can send a local service write command without claiming product-ready mutation.'
  ),
  TrackingRetentionSettingsParentExport: decodeDisplayText('Parent export setting'),
  TrackingRetentionSettingsParentExportEvidence: decodeDisplayText(
    'tracking-retention-settings-evidence-parent-export'
  ),
  TrackingRetentionSettingsReadModelReady: decodeDisplayText('settings-read-model-ready'),
  TrackingRetentionSettingsRemoteAiDisabled: decodeDisplayText('Remote AI disabled setting'),
  TrackingRetentionSettingsRemoteAiEvidence: decodeDisplayText(
    'tracking-retention-settings-evidence-remote-ai-disabled'
  ),
  TrackingRetentionSettingsRemoteSyncDisabled: decodeDisplayText('Remote sync disabled setting'),
  TrackingRetentionSettingsRemoteSyncEvidence: decodeDisplayText(
    'tracking-retention-settings-evidence-remote-sync-disabled'
  ),
  TrackingRetentionSettingsWindow: decodeDisplayText('Retention window setting'),
  TrackingRetentionSettingsWindowEvidence: decodeDisplayText('tracking-retention-settings-evidence-window'),
  TrackingRetentionSettingsWritePreflight: decodeDisplayText('Retention local service write result'),
  TrackingRetentionSettingsWritePreflightBody: decodeDisplayText(
    'Portal sends the typed retention settings write command and renders the local service execution result; product-ready mutation remains unclaimed.'
  ),
  TrackingRetentionSettingsWritePreflightBoundary: decodeDisplayText(
    'Portal command/result rendering proves local service mutation execution, local durable settings persistence, and local state revision only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  TrackingServiceDataCoverage: decodeDisplayText('Service data coverage'),
  TrackingServiceReadModel: decodeDisplayText('Service read model'),
  TrackingStateAcknowledged: decodeDisplayText('Parent acknowledged'),
  TrackingStateAlert: decodeDisplayText('Policy alert'),
  TrackingStateAmbiguousNearby: decodeDisplayText('Nearby place ambiguous'),
  TrackingStateChildCheckIn: decodeDisplayText('Child check-in'),
  TrackingStateDisabled: decodeDisplayText('Tracking off'),
  TrackingStateException: decodeDisplayText('Exception active'),
  TrackingStateLowAccuracy: decodeDisplayText('Low accuracy'),
  TrackingStateMissingDevice: decodeDisplayText('Missing device'),
  TrackingStateOffline: decodeDisplayText('Offline last known'),
  TrackingStatePermissionRequired: decodeDisplayText('Permission required'),
  TrackingStateRetentionDeleted: decodeDisplayText('Retention deleted'),
  TrackingStateStale: decodeDisplayText('Stale last known'),
  TrackingStateTemporaryLive: decodeDisplayText('Temporary live'),
  TrackingSupportManualRequired: decodeDisplayText('manual-required'),
  TrackingSupportPlatformUnsupported: decodeDisplayText('platform-unsupported'),
  TrackingSupportRealDeviceRequired: decodeDisplayText('real-device-required'),
  TrackingUnsupportedManualAndroidBackground: decodeDisplayText('Android background location manual required'),
  TrackingUnsupportedManualAndroidGeofence: decodeDisplayText('Android geofence transition manual required'),
  TrackingUnsupportedManualAuthorityHardControl: decodeDisplayText('Authority hard-control proof required'),
  TrackingUnsupportedManualBoundary: decodeDisplayText(
    'Hosted render-state proof only; physical-device, authority, provider delivery, and product readiness remain unclaimed.'
  ),
  TrackingUnsupportedManualDesktopOs: decodeDisplayText('Windows desktop OS location manual required'),
  TrackingUnsupportedManualIosBackground: decodeDisplayText('iOS background location manual required'),
  TrackingUnsupportedManualIosGeofence: decodeDisplayText('iOS geofence transition manual required'),
  TrackingUnsupportedManualProofBody: decodeDisplayText(
    'Unsupported platform and manual-required adapter rows render as degraded states without invented capability.'
  ),
  TrackingUnsupportedManualProofTitle: decodeDisplayText('Unsupported/manual tracking platform proof'),
  TrackingUnsupportedManualWebChildAgent: decodeDisplayText('Web child agent location unavailable'),
} as const;

export type PortalDevTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

export function resolvePortalDevText(token: PortalDevTextTokenValue): DisplayText {
  return token;
}
