import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';
import { PortalDevTrackingReportText, PortalDevTrackingReportTextToken } from './portal-dev-tracking-report-text';
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
  AppGameTimerParentSurface: decodeTextTokenId('portal.dev.appGameTimerParentSurface'),
  AppGameTimerParentSurfaceBody: decodeTextTokenId('portal.dev.appGameTimerParentSurfaceBody'),
  AppGameTimerParentSurfaceNoData: decodeTextTokenId('portal.dev.appGameTimerParentSurfaceNoData'),
  AppGameTimerParentSurfaceNoRuntimeClaim: decodeTextTokenId('portal.dev.appGameTimerParentSurfaceNoRuntimeClaim'),
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
  ...PortalDevTrackingReportTextToken,
  TrackingNotificationParentSurfaceHostedUi: decodeTextTokenId('portal.dev.trackingNotificationParentSurfaceHostedUi'),
  TrackingNotificationParentSurfaceHostedUiBody: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHostedUiBody'
  ),
  TrackingNotificationParentSurfaceHistoryIntent: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHistoryIntent'
  ),
  TrackingNotificationParentSurfaceManualAction: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceManualAction'
  ),
  TrackingNotificationParentSurfaceProviderUnavailable: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceProviderUnavailable'
  ),
  TrackingNotificationParentSurfaceHistoryIntentReady: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHistoryIntentReady'
  ),
  TrackingNotificationParentSurfaceManualActionRequired: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceManualActionRequired'
  ),
  TrackingNotificationParentSurfaceProviderUnavailableStatus: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceProviderUnavailableStatus'
  ),
  TrackingNotificationParentSurfaceHomeDecision: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomeDecision'
  ),
  TrackingNotificationParentSurfaceSchoolDecision: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolDecision'
  ),
  TrackingNotificationParentSurfaceUnavailableDecision: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailableDecision'
  ),
  TrackingNotificationParentSurfaceLocationEvidence: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceLocationEvidence'
  ),
  TrackingNotificationParentSurfaceHomeAttempt: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomeAttempt'
  ),
  TrackingNotificationParentSurfaceSchoolAttempt: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolAttempt'
  ),
  TrackingNotificationParentSurfaceUnavailableAttempt: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailableAttempt'
  ),
  TrackingNotificationParentSurfaceHomeReceiptRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomeReceiptRequirement'
  ),
  TrackingNotificationParentSurfaceSchoolReceiptRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolReceiptRequirement'
  ),
  TrackingNotificationParentSurfaceUnavailableReceiptRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailableReceiptRequirement'
  ),
  TrackingNotificationParentSurfaceHomePreferenceRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomePreferenceRequirement'
  ),
  TrackingNotificationParentSurfaceSchoolPreferenceRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolPreferenceRequirement'
  ),
  TrackingNotificationParentSurfaceUnavailablePreferenceRequirement: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailablePreferenceRequirement'
  ),
  TrackingNotificationParentSurfaceHomeManualProof: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomeManualProof'
  ),
  TrackingNotificationParentSurfaceSchoolManualProof: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolManualProof'
  ),
  TrackingNotificationParentSurfaceUnavailableManualProof: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailableManualProof'
  ),
  TrackingNotificationParentSurfaceHomeSummary: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHomeSummary'
  ),
  TrackingNotificationParentSurfaceSchoolSummary: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceSchoolSummary'
  ),
  TrackingNotificationParentSurfaceUnavailableSummary: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceUnavailableSummary'
  ),
  TrackingNotificationParentSurfaceHostedBoundary: decodeTextTokenId(
    'portal.dev.trackingNotificationParentSurfaceHostedBoundary'
  ),
  TrackingParentActionReadinessHostedUi: decodeTextTokenId('portal.dev.trackingParentActionReadinessHostedUi'),
  TrackingParentActionReadinessHostedUiBody: decodeTextTokenId('portal.dev.trackingParentActionReadinessHostedUiBody'),
  TrackingParentActionReadinessHostedBoundary: decodeTextTokenId(
    'portal.dev.trackingParentActionReadinessHostedBoundary'
  ),
  TrackingParentActionExpectedPlaceAlert: decodeTextTokenId('portal.dev.trackingParentActionExpectedPlaceAlert'),
  TrackingParentActionExpectedPlaceCheckIn: decodeTextTokenId('portal.dev.trackingParentActionExpectedPlaceCheckIn'),
  TrackingParentActionExpectedPlaceSuppressed: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceSuppressed'
  ),
  TrackingParentActionExpectedPlaceManual: decodeTextTokenId('portal.dev.trackingParentActionExpectedPlaceManual'),
  TrackingParentActionAcknowledgementRecorded: decodeTextTokenId(
    'portal.dev.trackingParentActionAcknowledgementRecorded'
  ),
  TrackingParentActionExceptionActive: decodeTextTokenId('portal.dev.trackingParentActionExceptionActive'),
  TrackingParentActionFalseAlarmRecorded: decodeTextTokenId('portal.dev.trackingParentActionFalseAlarmRecorded'),
  TrackingParentActionChildCheckInReady: decodeTextTokenId('portal.dev.trackingParentActionChildCheckInReady'),
  TrackingParentActionCriticalReviewReady: decodeTextTokenId('portal.dev.trackingParentActionCriticalReviewReady'),
  TrackingParentActionAlertPolicyReady: decodeTextTokenId('portal.dev.trackingParentActionAlertPolicyReady'),
  TrackingParentActionCheckInPolicyReady: decodeTextTokenId('portal.dev.trackingParentActionCheckInPolicyReady'),
  TrackingParentActionSuppressedNoAction: decodeTextTokenId('portal.dev.trackingParentActionSuppressedNoAction'),
  TrackingParentActionManualRequired: decodeTextTokenId('portal.dev.trackingParentActionManualRequired'),
  TrackingParentActionAcknowledgementRecordedStatus: decodeTextTokenId(
    'portal.dev.trackingParentActionAcknowledgementRecordedStatus'
  ),
  TrackingParentActionExceptionActiveStatus: decodeTextTokenId('portal.dev.trackingParentActionExceptionActiveStatus'),
  TrackingParentActionFalseAlarmRecordedStatus: decodeTextTokenId(
    'portal.dev.trackingParentActionFalseAlarmRecordedStatus'
  ),
  TrackingParentActionChildCheckInRequestReady: decodeTextTokenId(
    'portal.dev.trackingParentActionChildCheckInRequestReady'
  ),
  TrackingParentActionEscalationReviewReady: decodeTextTokenId('portal.dev.trackingParentActionEscalationReviewReady'),
  TrackingParentActionNotifyParent: decodeTextTokenId('portal.dev.trackingParentActionNotifyParent'),
  TrackingParentActionAskChildCheckIn: decodeTextTokenId('portal.dev.trackingParentActionAskChildCheckIn'),
  TrackingParentActionNoAction: decodeTextTokenId('portal.dev.trackingParentActionNoAction'),
  TrackingParentActionManualReview: decodeTextTokenId('portal.dev.trackingParentActionManualReview'),
  TrackingParentActionAcknowledgeSafe: decodeTextTokenId('portal.dev.trackingParentActionAcknowledgeSafe'),
  TrackingParentActionMarkExpected: decodeTextTokenId('portal.dev.trackingParentActionMarkExpected'),
  TrackingParentActionMarkFalseAlarm: decodeTextTokenId('portal.dev.trackingParentActionMarkFalseAlarm'),
  TrackingParentActionRequestChildCheckIn: decodeTextTokenId('portal.dev.trackingParentActionRequestChildCheckIn'),
  TrackingParentActionEscalateManualReview: decodeTextTokenId('portal.dev.trackingParentActionEscalateManualReview'),
  TrackingParentActionExpectedPlaceSchoolDecision: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceSchoolDecision'
  ),
  TrackingParentActionExpectedPlaceLateBusDecision: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLateBusDecision'
  ),
  TrackingParentActionExpectedPlaceHolidayDecision: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceHolidayDecision'
  ),
  TrackingParentActionExpectedPlaceLowAccuracyDecision: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLowAccuracyDecision'
  ),
  TrackingParentActionSafeDecision: decodeTextTokenId('portal.dev.trackingParentActionSafeDecision'),
  TrackingParentActionExpectedDecision: decodeTextTokenId('portal.dev.trackingParentActionExpectedDecision'),
  TrackingParentActionFalseAlarmDecision: decodeTextTokenId('portal.dev.trackingParentActionFalseAlarmDecision'),
  TrackingParentActionChildCheckInDecision: decodeTextTokenId('portal.dev.trackingParentActionChildCheckInDecision'),
  TrackingParentActionCriticalReviewDecision: decodeTextTokenId(
    'portal.dev.trackingParentActionCriticalReviewDecision'
  ),
  TrackingParentActionExpectedPlaceSchoolEvidence: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceSchoolEvidence'
  ),
  TrackingParentActionExpectedPlaceLateBusEvidence: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLateBusEvidence'
  ),
  TrackingParentActionExpectedPlaceHolidayEvidence: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceHolidayEvidence'
  ),
  TrackingParentActionExpectedPlaceLowAccuracyEvidence: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLowAccuracyEvidence'
  ),
  TrackingParentActionSafeEvidence: decodeTextTokenId('portal.dev.trackingParentActionSafeEvidence'),
  TrackingParentActionExpectedEvidence: decodeTextTokenId('portal.dev.trackingParentActionExpectedEvidence'),
  TrackingParentActionFalseAlarmEvidence: decodeTextTokenId('portal.dev.trackingParentActionFalseAlarmEvidence'),
  TrackingParentActionChildCheckInEvidence: decodeTextTokenId('portal.dev.trackingParentActionChildCheckInEvidence'),
  TrackingParentActionCriticalReviewEvidence: decodeTextTokenId(
    'portal.dev.trackingParentActionCriticalReviewEvidence'
  ),
  TrackingParentActionExpectedPlaceSchoolSurface: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceSchoolSurface'
  ),
  TrackingParentActionExpectedPlaceLateBusSurface: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLateBusSurface'
  ),
  TrackingParentActionExpectedPlaceHolidaySurface: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceHolidaySurface'
  ),
  TrackingParentActionExpectedPlaceLowAccuracySurface: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceLowAccuracySurface'
  ),
  TrackingParentActionSafeSurface: decodeTextTokenId('portal.dev.trackingParentActionSafeSurface'),
  TrackingParentActionExpectedSurface: decodeTextTokenId('portal.dev.trackingParentActionExpectedSurface'),
  TrackingParentActionFalseAlarmSurface: decodeTextTokenId('portal.dev.trackingParentActionFalseAlarmSurface'),
  TrackingParentActionChildCheckInSurface: decodeTextTokenId('portal.dev.trackingParentActionChildCheckInSurface'),
  TrackingParentActionCriticalReviewSurface: decodeTextTokenId('portal.dev.trackingParentActionCriticalReviewSurface'),
  TrackingParentActionHostedReadOnlyManualProof: decodeTextTokenId(
    'portal.dev.trackingParentActionHostedReadOnlyManualProof'
  ),
  TrackingParentActionExpectedPlaceManualProof: decodeTextTokenId(
    'portal.dev.trackingParentActionExpectedPlaceManualProof'
  ),
  TrackingParentActionServiceMutationManualProof: decodeTextTokenId(
    'portal.dev.trackingParentActionServiceMutationManualProof'
  ),
  TrackingParentActionChildRuntimeManualProof: decodeTextTokenId(
    'portal.dev.trackingParentActionChildRuntimeManualProof'
  ),
  TrackingParentActionEscalationManualProof: decodeTextTokenId('portal.dev.trackingParentActionEscalationManualProof'),
  TrackingMissingDeviceHostedUi: decodeTextTokenId('portal.dev.trackingMissingDeviceHostedUi'),
  TrackingMissingDeviceHostedUiBody: decodeTextTokenId('portal.dev.trackingMissingDeviceHostedUiBody'),
  TrackingMissingDeviceHostedBoundary: decodeTextTokenId('portal.dev.trackingMissingDeviceHostedBoundary'),
  TrackingMissingDeviceLastKnownOnly: decodeTextTokenId('portal.dev.trackingMissingDeviceLastKnownOnly'),
  TrackingMissingDevicePoweredOff: decodeTextTokenId('portal.dev.trackingMissingDevicePoweredOff'),
  TrackingMissingDeviceContactRequested: decodeTextTokenId('portal.dev.trackingMissingDeviceContactRequested'),
  TrackingMissingDeviceManualRequired: decodeTextTokenId('portal.dev.trackingMissingDeviceManualRequired'),
  TrackingMissingDeviceLastKnownState: decodeTextTokenId('portal.dev.trackingMissingDeviceLastKnownState'),
  TrackingMissingDeviceOfflineState: decodeTextTokenId('portal.dev.trackingMissingDeviceOfflineState'),
  TrackingMissingDeviceContactRequestedState: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceContactRequestedState'
  ),
  TrackingMissingDeviceManualRequiredState: decodeTextTokenId('portal.dev.trackingMissingDeviceManualRequiredState'),
  TrackingMissingDeviceLastKnownBadge: decodeTextTokenId('portal.dev.trackingMissingDeviceLastKnownBadge'),
  TrackingMissingDeviceOfflineBadge: decodeTextTokenId('portal.dev.trackingMissingDeviceOfflineBadge'),
  TrackingMissingDeviceContactRequestedBadge: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceContactRequestedBadge'
  ),
  TrackingMissingDeviceManualRequiredBadge: decodeTextTokenId('portal.dev.trackingMissingDeviceManualRequiredBadge'),
  TrackingMissingDeviceOfflineContact: decodeTextTokenId('portal.dev.trackingMissingDeviceOfflineContact'),
  TrackingMissingDevicePoweredOffContact: decodeTextTokenId('portal.dev.trackingMissingDevicePoweredOffContact'),
  TrackingMissingDeviceOnlineContact: decodeTextTokenId('portal.dev.trackingMissingDeviceOnlineContact'),
  TrackingMissingDeviceUnknownContact: decodeTextTokenId('portal.dev.trackingMissingDeviceUnknownContact'),
  TrackingMissingDeviceLastKnownEvidence: decodeTextTokenId('portal.dev.trackingMissingDeviceLastKnownEvidence'),
  TrackingMissingDeviceOfflineStatusEvidence: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceOfflineStatusEvidence'
  ),
  TrackingMissingDevicePoweredOffEvidence: decodeTextTokenId('portal.dev.trackingMissingDevicePoweredOffEvidence'),
  TrackingMissingDevicePoweredOffStatusEvidence: decodeTextTokenId(
    'portal.dev.trackingMissingDevicePoweredOffStatusEvidence'
  ),
  TrackingMissingDeviceContactRequestedEvidence: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceContactRequestedEvidence'
  ),
  TrackingMissingDeviceContactStatusEvidence: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceContactStatusEvidence'
  ),
  TrackingMissingDeviceManualEvidence: decodeTextTokenId('portal.dev.trackingMissingDeviceManualEvidence'),
  TrackingMissingDevicePlatformProofEvidence: decodeTextTokenId(
    'portal.dev.trackingMissingDevicePlatformProofEvidence'
  ),
  TrackingMissingDeviceReviewCheckInAction: decodeTextTokenId('portal.dev.trackingMissingDeviceReviewCheckInAction'),
  TrackingMissingDeviceCallMarkFoundAction: decodeTextTokenId('portal.dev.trackingMissingDeviceCallMarkFoundAction'),
  TrackingMissingDeviceManualPlatformAction: decodeTextTokenId('portal.dev.trackingMissingDeviceManualPlatformAction'),
  TrackingMissingDeviceHostedReadOnlyManualProof: decodeTextTokenId(
    'portal.dev.trackingMissingDeviceHostedReadOnlyManualProof'
  ),
  TrackingMissingDevicePoweredOffManualProof: decodeTextTokenId(
    'portal.dev.trackingMissingDevicePoweredOffManualProof'
  ),
  TrackingMissingDevicePlatformManualProof: decodeTextTokenId('portal.dev.trackingMissingDevicePlatformManualProof'),
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
  TrackingRetentionSettingsWritePreflight: decodeTextTokenId('portal.dev.trackingRetentionSettingsWritePreflight'),
  TrackingRetentionSettingsWritePreflightBody: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsWritePreflightBody'
  ),
  TrackingRetentionSettingsWritePreflightButton: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsWritePreflightButton'
  ),
  TrackingRetentionSettingsWritePreflightBoundary: decodeTextTokenId(
    'portal.dev.trackingRetentionSettingsWritePreflightBoundary'
  ),
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
  GetBrowserRuntimeEventChainStream: decodeTextTokenId('portal.dev.command.getBrowserRuntimeEventChainStream'),
  GetNetworkFlowReadModel: decodeTextTokenId('portal.dev.command.getNetworkFlowReadModel'),
  GetNetworkRuntimeEventChainStream: decodeTextTokenId('portal.dev.command.getNetworkRuntimeEventChainStream'),
  GetNetworkRemoteDeliveryStatus: decodeTextTokenId('portal.dev.command.getNetworkRemoteDeliveryStatus'),
  GetNetworkLiveCaptureStatus: decodeTextTokenId('portal.dev.command.getNetworkLiveCaptureStatus'),
  GetNetworkLinuxNftablesLabStatus: decodeTextTokenId('portal.dev.command.getNetworkLinuxNftablesLabStatus'),
  GetNetworkWindowsFirewallLabStatus: decodeTextTokenId('portal.dev.command.getNetworkWindowsFirewallLabStatus'),
  GetNetworkWindowsWfpGateStatus: decodeTextTokenId('portal.dev.command.getNetworkWindowsWfpGateStatus'),
  GetActivityTrackingReadModel: decodeTextTokenId('portal.dev.command.getActivityTrackingReadModel'),
  GetActivityAppGamePolicyReadinessReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGamePolicyReadinessReadModel'
  ),
  GetActivityAppGameAdapterExecutionReadinessReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGameAdapterExecutionReadinessReadModel'
  ),
  GetActivityAppGamePlatformProofStatusReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGamePlatformProofStatusReadModel'
  ),
  GetActivityAppGameChildRuntimeTransportReceiptReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGameChildRuntimeTransportReceiptReadModel'
  ),
  GetActivityAppGameAdapterDispatchPreflightReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGameAdapterDispatchPreflightReadModel'
  ),
  GetActivityAppGameAdapterDispatchResultReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGameAdapterDispatchResultReadModel'
  ),
  ExecuteActivityAppGameAdapterDispatch: decodeTextTokenId('portal.dev.command.executeActivityAppGameAdapterDispatch'),
  GetActivityAppGameTimerParentSurfaceReadModel: decodeTextTokenId(
    'portal.dev.command.getActivityAppGameTimerParentSurfaceReadModel'
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
  [PortalDevTextToken.AppGameTimerParentSurface]: decodeDisplayText('App/game timer parent surface'),
  [PortalDevTextToken.AppGameTimerParentSurfaceBody]: decodeDisplayText(
    'Service-backed parent-surface timer rows only; no runtime scheduling or enforcement is claimed.'
  ),
  [PortalDevTextToken.AppGameTimerParentSurfaceNoData]: decodeDisplayText(
    'No app/game timer parent-surface read model has been reported yet.'
  ),
  [PortalDevTextToken.AppGameTimerParentSurfaceNoRuntimeClaim]: decodeDisplayText(
    'Parent-surface rendering only; active timer state-store is shown only when reported by the service. Live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.'
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
  ...PortalDevTrackingReportText,
  [PortalDevTextToken.TrackingNotificationParentSurfaceHostedUi]: decodeDisplayText('Notification history intent UI'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHostedUiBody]: decodeDisplayText(
    'Hosted route renders parent notification history, manual action, and provider unavailable rows from existing tracking notification proof refs without claiming provider delivery or receipt runtime.'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntent]: decodeDisplayText('Notification history ready'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceManualAction]: decodeDisplayText(
    'Manual notification action required'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailable]: decodeDisplayText(
    'Notification provider unavailable'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHistoryIntentReady]: decodeDisplayText('history-intent-ready'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceManualActionRequired]:
    decodeDisplayText('manual-action-required'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceProviderUnavailableStatus]:
    decodeDisplayText('provider-unavailable'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomeDecision]: decodeDisplayText(
    'tracking-decision-home-arrival'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolDecision]: decodeDisplayText(
    'tracking-decision-left-expected-place'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableDecision]: decodeDisplayText(
    'tracking-decision-provider-unavailable'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceLocationEvidence]: decodeDisplayText(
    'location-evidence-geofence-entry'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomeAttempt]: decodeDisplayText(
    'tracking-provider-attempt-home-arrival'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolAttempt]: decodeDisplayText(
    'tracking-provider-attempt-left-school'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableAttempt]: decodeDisplayText(
    'tracking-provider-attempt-unavailable'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomeReceiptRequirement]: decodeDisplayText(
    'receipt-ingestion-required-home-arrival'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolReceiptRequirement]: decodeDisplayText(
    'manual-receipt-required-left-school'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableReceiptRequirement]:
    decodeDisplayText('provider-receipt-unavailable'),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomePreferenceRequirement]: decodeDisplayText(
    'parent-notification-preference-required-home-arrival'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolPreferenceRequirement]: decodeDisplayText(
    'quiet-hours-requirement-left-school'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailablePreferenceRequirement]: decodeDisplayText(
    'source-unavailable-preference-required'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomeManualProof]: decodeDisplayText(
    'provider-delivery-runtime-required | receipt-webhook-runtime-required'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolManualProof]: decodeDisplayText(
    'manual-provider-review-required | quiet-hours-runtime-required'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableManualProof]: decodeDisplayText(
    'provider-adapter-unavailable | manual-parent-history-review-required'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHomeSummary]: decodeDisplayText(
    'tracking-notification-redacted-summary-tracking-alert-home-arrival'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceSchoolSummary]: decodeDisplayText(
    'tracking-notification-redacted-summary-tracking-alert-left-expected-place'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceUnavailableSummary]: decodeDisplayText(
    'tracking-notification-redacted-summary-tracking-alert-provider-unavailable'
  ),
  [PortalDevTextToken.TrackingNotificationParentSurfaceHostedBoundary]: decodeDisplayText(
    'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingParentActionReadinessHostedUi]: decodeDisplayText('Parent action readiness UI'),
  [PortalDevTextToken.TrackingParentActionReadinessHostedUiBody]: decodeDisplayText(
    'Hosted route renders expected-place alert policy and parent acknowledgement action readiness rows from existing tracking proof refs without claiming live mutation or delivery runtime.'
  ),
  [PortalDevTextToken.TrackingParentActionReadinessHostedBoundary]: decodeDisplayText(
    'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceAlert]: decodeDisplayText('Expected-place parent alert ready'),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceCheckIn]: decodeDisplayText(
    'Expected-place child check-in ready'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceSuppressed]: decodeDisplayText(
    'Expected-place suppressed no action'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceManual]: decodeDisplayText(
    'Expected-place manual review required'
  ),
  [PortalDevTextToken.TrackingParentActionAcknowledgementRecorded]: decodeDisplayText(
    'Parent acknowledgement recorded'
  ),
  [PortalDevTextToken.TrackingParentActionExceptionActive]: decodeDisplayText('Expected exception active'),
  [PortalDevTextToken.TrackingParentActionFalseAlarmRecorded]: decodeDisplayText('False alarm recorded'),
  [PortalDevTextToken.TrackingParentActionChildCheckInReady]: decodeDisplayText('Child check-in action ready'),
  [PortalDevTextToken.TrackingParentActionCriticalReviewReady]: decodeDisplayText('Critical escalation review ready'),
  [PortalDevTextToken.TrackingParentActionAlertPolicyReady]: decodeDisplayText('alert-policy-ready'),
  [PortalDevTextToken.TrackingParentActionCheckInPolicyReady]: decodeDisplayText('check-in-policy-ready'),
  [PortalDevTextToken.TrackingParentActionSuppressedNoAction]: decodeDisplayText('suppressed-no-action'),
  [PortalDevTextToken.TrackingParentActionManualRequired]: decodeDisplayText('manual-required'),
  [PortalDevTextToken.TrackingParentActionAcknowledgementRecordedStatus]: decodeDisplayText('acknowledgement-recorded'),
  [PortalDevTextToken.TrackingParentActionExceptionActiveStatus]: decodeDisplayText('exception-active'),
  [PortalDevTextToken.TrackingParentActionFalseAlarmRecordedStatus]: decodeDisplayText('false-alarm-recorded'),
  [PortalDevTextToken.TrackingParentActionChildCheckInRequestReady]: decodeDisplayText('child-check-in-request-ready'),
  [PortalDevTextToken.TrackingParentActionEscalationReviewReady]: decodeDisplayText('escalation-review-ready'),
  [PortalDevTextToken.TrackingParentActionNotifyParent]: decodeDisplayText('notify-parent'),
  [PortalDevTextToken.TrackingParentActionAskChildCheckIn]: decodeDisplayText('ask-child-check-in'),
  [PortalDevTextToken.TrackingParentActionNoAction]: decodeDisplayText('no-action'),
  [PortalDevTextToken.TrackingParentActionManualReview]: decodeDisplayText('manual-review'),
  [PortalDevTextToken.TrackingParentActionAcknowledgeSafe]: decodeDisplayText('acknowledge-safe'),
  [PortalDevTextToken.TrackingParentActionMarkExpected]: decodeDisplayText('mark-expected'),
  [PortalDevTextToken.TrackingParentActionMarkFalseAlarm]: decodeDisplayText('mark-false-alarm'),
  [PortalDevTextToken.TrackingParentActionRequestChildCheckIn]: decodeDisplayText('request-child-check-in'),
  [PortalDevTextToken.TrackingParentActionEscalateManualReview]: decodeDisplayText('escalate-manual-review'),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolDecision]: decodeDisplayText(
    'expected-place-decision-school'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusDecision]: decodeDisplayText(
    'expected-place-decision-late-bus'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceHolidayDecision]: decodeDisplayText(
    'expected-place-decision-holiday'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracyDecision]: decodeDisplayText(
    'expected-place-decision-low-accuracy'
  ),
  [PortalDevTextToken.TrackingParentActionSafeDecision]: decodeDisplayText('tracking-decision-safe'),
  [PortalDevTextToken.TrackingParentActionExpectedDecision]: decodeDisplayText('tracking-decision-expected'),
  [PortalDevTextToken.TrackingParentActionFalseAlarmDecision]: decodeDisplayText('tracking-decision-false-alarm'),
  [PortalDevTextToken.TrackingParentActionChildCheckInDecision]: decodeDisplayText('tracking-decision-check-in'),
  [PortalDevTextToken.TrackingParentActionCriticalReviewDecision]: decodeDisplayText(
    'tracking-decision-critical-review'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolEvidence]: decodeDisplayText(
    'expected-place-evidence-school-arrival'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusEvidence]: decodeDisplayText(
    'expected-place-evidence-late-bus'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceHolidayEvidence]: decodeDisplayText(
    'expected-place-evidence-holiday'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracyEvidence]: decodeDisplayText(
    'expected-place-evidence-low-accuracy'
  ),
  [PortalDevTextToken.TrackingParentActionSafeEvidence]: decodeDisplayText('tracking-parent-action-evidence-1'),
  [PortalDevTextToken.TrackingParentActionExpectedEvidence]: decodeDisplayText('tracking-parent-action-evidence-2'),
  [PortalDevTextToken.TrackingParentActionFalseAlarmEvidence]: decodeDisplayText('tracking-parent-action-evidence-3'),
  [PortalDevTextToken.TrackingParentActionChildCheckInEvidence]: decodeDisplayText('tracking-parent-action-evidence-4'),
  [PortalDevTextToken.TrackingParentActionCriticalReviewEvidence]: decodeDisplayText(
    'tracking-parent-action-evidence-5'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceSchoolSurface]: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-school'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLateBusSurface]: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-late-bus'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceHolidaySurface]: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-holiday'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceLowAccuracySurface]: decodeDisplayText(
    'tracking-expected-place-ui-readiness-expected-place-decision-low-accuracy'
  ),
  [PortalDevTextToken.TrackingParentActionSafeSurface]: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-safe'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedSurface]: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-expected'
  ),
  [PortalDevTextToken.TrackingParentActionFalseAlarmSurface]: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-false-alarm'
  ),
  [PortalDevTextToken.TrackingParentActionChildCheckInSurface]: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-check-in'
  ),
  [PortalDevTextToken.TrackingParentActionCriticalReviewSurface]: decodeDisplayText(
    'tracking-parent-action-surface-tracking-alert-critical-review'
  ),
  [PortalDevTextToken.TrackingParentActionHostedReadOnlyManualProof]: decodeDisplayText(
    'hosted-read-only-parent-action-proof'
  ),
  [PortalDevTextToken.TrackingParentActionExpectedPlaceManualProof]: decodeDisplayText(
    'tracking-expected-place-manual-proof-expected-place-decision-low-accuracy'
  ),
  [PortalDevTextToken.TrackingParentActionServiceMutationManualProof]: decodeDisplayText(
    'live-service-mutation-proof-required | rendered-portal-acknowledgement-ui-proof-required'
  ),
  [PortalDevTextToken.TrackingParentActionChildRuntimeManualProof]: decodeDisplayText(
    'child-device-runtime-proof-required | rendered-portal-acknowledgement-ui-proof-required'
  ),
  [PortalDevTextToken.TrackingParentActionEscalationManualProof]: decodeDisplayText(
    'critical-escalation-runtime-proof-required | second-guardian-provider-proof-required'
  ),
  [PortalDevTextToken.TrackingMissingDeviceHostedUi]: decodeDisplayText('Missing-device state UI'),
  [PortalDevTextToken.TrackingMissingDeviceHostedUiBody]: decodeDisplayText(
    'Hosted route renders last-known, offline, contact-requested, and manual-required missing-device rows from existing WP29 proof without claiming current location or OS lost-mode runtime.'
  ),
  [PortalDevTextToken.TrackingMissingDeviceHostedBoundary]: decodeDisplayText(
    'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingMissingDeviceLastKnownOnly]: decodeDisplayText('Last-known only state'),
  [PortalDevTextToken.TrackingMissingDevicePoweredOff]: decodeDisplayText('Powered-off offline state'),
  [PortalDevTextToken.TrackingMissingDeviceContactRequested]: decodeDisplayText('Contact requested state'),
  [PortalDevTextToken.TrackingMissingDeviceManualRequired]: decodeDisplayText('Manual platform proof state'),
  [PortalDevTextToken.TrackingMissingDeviceLastKnownState]: decodeDisplayText('last-known-only'),
  [PortalDevTextToken.TrackingMissingDeviceOfflineState]: decodeDisplayText('offline'),
  [PortalDevTextToken.TrackingMissingDeviceContactRequestedState]: decodeDisplayText('contact-requested'),
  [PortalDevTextToken.TrackingMissingDeviceManualRequiredState]: decodeDisplayText('manual-required'),
  [PortalDevTextToken.TrackingMissingDeviceLastKnownBadge]: decodeDisplayText('last-known'),
  [PortalDevTextToken.TrackingMissingDeviceOfflineBadge]: decodeDisplayText('offline'),
  [PortalDevTextToken.TrackingMissingDeviceContactRequestedBadge]: decodeDisplayText('contact-requested'),
  [PortalDevTextToken.TrackingMissingDeviceManualRequiredBadge]: decodeDisplayText('manual-required'),
  [PortalDevTextToken.TrackingMissingDeviceOfflineContact]: decodeDisplayText('contact-state-offline'),
  [PortalDevTextToken.TrackingMissingDevicePoweredOffContact]: decodeDisplayText('contact-state-powered-off'),
  [PortalDevTextToken.TrackingMissingDeviceOnlineContact]: decodeDisplayText('contact-state-online'),
  [PortalDevTextToken.TrackingMissingDeviceUnknownContact]: decodeDisplayText('contact-state-unknown'),
  [PortalDevTextToken.TrackingMissingDeviceLastKnownEvidence]: decodeDisplayText('location-evidence-last-known-stale'),
  [PortalDevTextToken.TrackingMissingDeviceOfflineStatusEvidence]: decodeDisplayText(
    'device-status-offline-last-known'
  ),
  [PortalDevTextToken.TrackingMissingDevicePoweredOffEvidence]: decodeDisplayText(
    'location-evidence-last-known-powered-off'
  ),
  [PortalDevTextToken.TrackingMissingDevicePoweredOffStatusEvidence]: decodeDisplayText('device-status-powered-off'),
  [PortalDevTextToken.TrackingMissingDeviceContactRequestedEvidence]: decodeDisplayText(
    'location-evidence-last-known-contact-requested'
  ),
  [PortalDevTextToken.TrackingMissingDeviceContactStatusEvidence]: decodeDisplayText(
    'device-status-contact-action-queued'
  ),
  [PortalDevTextToken.TrackingMissingDeviceManualEvidence]: decodeDisplayText(
    'location-evidence-last-known-manual-required'
  ),
  [PortalDevTextToken.TrackingMissingDevicePlatformProofEvidence]: decodeDisplayText(
    'device-status-platform-proof-required'
  ),
  [PortalDevTextToken.TrackingMissingDeviceReviewCheckInAction]: decodeDisplayText(
    'review-last-known | ask-child-check-in | call-child | mark-found'
  ),
  [PortalDevTextToken.TrackingMissingDeviceCallMarkFoundAction]: decodeDisplayText(
    'review-last-known | call-child | mark-found'
  ),
  [PortalDevTextToken.TrackingMissingDeviceManualPlatformAction]: decodeDisplayText(
    'review-last-known | manual-platform-proof'
  ),
  [PortalDevTextToken.TrackingMissingDeviceHostedReadOnlyManualProof]: decodeDisplayText(
    'hosted-read-only-missing-device-proof'
  ),
  [PortalDevTextToken.TrackingMissingDevicePoweredOffManualProof]: decodeDisplayText(
    'powered-off-current-location-proof-forbidden | hosted-read-only-missing-device-proof'
  ),
  [PortalDevTextToken.TrackingMissingDevicePlatformManualProof]: decodeDisplayText(
    'os-lost-mode-api-proof-required | physical-device-proof-required'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsHostedUi]: decodeDisplayText('Retention settings read-model UI'),
  [PortalDevTextToken.TrackingRetentionSettingsHostedUiBody]: decodeDisplayText(
    'Hosted route renders existing retention settings read-model rows and can send a local service write command without claiming product-ready mutation.'
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
    'Hosted retention settings rendering proves local service write execution and durable local persistence only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsWritePreflight]: decodeDisplayText(
    'Retention local service write result'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsWritePreflightBody]: decodeDisplayText(
    'Portal sends the typed retention settings write command and renders the local service execution result; product-ready mutation remains unclaimed.'
  ),
  [PortalDevTextToken.TrackingRetentionSettingsWritePreflightButton]: decodeDisplayText('Send retention local write'),
  [PortalDevTextToken.TrackingRetentionSettingsWritePreflightBoundary]: decodeDisplayText(
    'Portal command/result rendering proves local service mutation execution, local durable settings persistence, and local state revision only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
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
  [PortalDevTextToken.GetBrowserRuntimeEventChainStream]: decodeDisplayText('Refresh browser runtime'),
  [PortalDevTextToken.GetNetworkFlowReadModel]: decodeDisplayText('Refresh network activity'),
  [PortalDevTextToken.GetNetworkRuntimeEventChainStream]: decodeDisplayText('Refresh network runtime'),
  [PortalDevTextToken.GetNetworkRemoteDeliveryStatus]: decodeDisplayText('Refresh remote delivery'),
  [PortalDevTextToken.GetNetworkLiveCaptureStatus]: decodeDisplayText('Refresh live capture'),
  [PortalDevTextToken.GetNetworkLinuxNftablesLabStatus]: decodeDisplayText('Refresh Linux nftables lab'),
  [PortalDevTextToken.GetNetworkWindowsFirewallLabStatus]: decodeDisplayText('Refresh Windows firewall lab'),
  [PortalDevTextToken.GetNetworkWindowsWfpGateStatus]: decodeDisplayText('Refresh Windows WFP gate'),
  [PortalDevTextToken.GetActivityTrackingReadModel]: decodeDisplayText('Refresh tracking status'),
  [PortalDevTextToken.GetActivityAppGamePolicyReadinessReadModel]: decodeDisplayText('Refresh policy readiness'),
  [PortalDevTextToken.GetActivityAppGameAdapterExecutionReadinessReadModel]:
    decodeDisplayText('Refresh adapter readiness'),
  [PortalDevTextToken.GetActivityAppGamePlatformProofStatusReadModel]: decodeDisplayText(
    'Refresh platform proof status'
  ),
  [PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel]: decodeDisplayText(
    'Refresh child runtime receipts'
  ),
  [PortalDevTextToken.GetActivityAppGameAdapterDispatchPreflightReadModel]: decodeDisplayText(
    'Refresh adapter dispatch preflight'
  ),
  [PortalDevTextToken.GetActivityAppGameAdapterDispatchResultReadModel]: decodeDisplayText(
    'Refresh adapter dispatch result'
  ),
  [PortalDevTextToken.ExecuteActivityAppGameAdapterDispatch]: decodeDisplayText('Execute scoped adapter dispatch'),
  [PortalDevTextToken.GetActivityAppGameTimerParentSurfaceReadModel]: decodeDisplayText('Refresh timer parent surface'),
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
