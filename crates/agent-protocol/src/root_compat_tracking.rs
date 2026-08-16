pub const TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE: &str =
    tracking::read_model::TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE;
pub const TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE: &str =
    tracking::read_model::TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE;
pub const TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS: &str =
    tracking::read_model::TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS;
pub type TrackingAcceptedAt = tracking::identifiers::TrackingAcceptedAt;
pub type TrackingAcknowledgementId = tracking::identifiers::TrackingAcknowledgementId;
pub type TrackingAcknowledgementState = tracking::identifiers::TrackingAcknowledgementState;
pub type TrackingAiAnalysisRequestedEvent =
    tracking::runtime_event::TrackingAiAnalysisRequestedEvent;
pub type TrackingAiAnalysisRequirement = tracking::runtime_event::TrackingAiAnalysisRequirement;
pub type TrackingAiBoundaryMode = tracking::runtime_event::TrackingAiBoundaryMode;
pub type TrackingAiPurpose = tracking::identifiers::TrackingAiPurpose;
pub type TrackingAiRequestId = tracking::identifiers::TrackingAiRequestId;
pub type TrackingAlertEvaluationId = tracking::identifiers::TrackingAlertEvaluationId;
pub type TrackingAlertSeverity = tracking::identifiers::TrackingAlertSeverity;
pub type TrackingCapabilityStatus = tracking::identifiers::TrackingCapabilityStatus;
pub type TrackingCheckInId = tracking::identifiers::TrackingCheckInId;
pub type TrackingCheckInState = tracking::identifiers::TrackingCheckInState;
pub type TrackingChildCheckInDeliveryState =
    tracking::runtime_event::TrackingChildCheckInDeliveryState;
pub type TrackingChildCheckInRecordedEvent =
    tracking::runtime_event::TrackingChildCheckInRecordedEvent;
pub type TrackingChildCheckInRequestedEvent =
    tracking::runtime_event::TrackingChildCheckInRequestedEvent;
pub type TrackingChildCheckInRequestReceipt =
    tracking::runtime_event::TrackingChildCheckInRequestReceipt;
pub type TrackingChildCheckInRequestState =
    tracking::runtime_event::TrackingChildCheckInRequestState;
pub type TrackingChildDeviceId = tracking::identifiers::TrackingChildDeviceId;
pub type TrackingChildProfileId = tracking::identifiers::TrackingChildProfileId;
pub type TrackingConfidenceBasis = tracking::identifiers::TrackingConfidenceBasis;
pub type TrackingConfigAckState =
    tracking::retention_settings_write_command::TrackingConfigAckState;
pub type TrackingConfigAuditEntryCommittedEvent =
    tracking::config_update_event::TrackingConfigAuditEntryCommittedEvent;
pub type TrackingConfigAuditOutcome = tracking::config_update_event::TrackingConfigAuditOutcome;
pub type TrackingConfigChangeApprovedEvent =
    tracking::config_update_event::TrackingConfigChangeApprovedEvent;
pub type TrackingConfigChangeRejectedEvent =
    tracking::config_update_event::TrackingConfigChangeRejectedEvent;
pub type TrackingConfigChangeRequestedEvent =
    tracking::config_update_event::TrackingConfigChangeRequestedEvent;
pub type TrackingConfigEffectiveState = tracking::config_update_event::TrackingConfigEffectiveState;
pub type TrackingConfigPolicyDecisionCompletedEvent =
    tracking::config_update_event::TrackingConfigPolicyDecisionCompletedEvent;
pub type TrackingConfigPolicyDecisionState =
    tracking::config_update_event::TrackingConfigPolicyDecisionState;
pub type TrackingConfigPolicyEvaluationRequestedEvent =
    tracking::config_update_event::TrackingConfigPolicyEvaluationRequestedEvent;
pub type TrackingConfigPortalReadModelUpdatedEvent =
    tracking::config_update_event::TrackingConfigPortalReadModelUpdatedEvent;
pub type TrackingConfigPortalUpdateKind =
    tracking::config_update_event::TrackingConfigPortalUpdateKind;
pub type TrackingConfigUpdateAppliedEvent =
    tracking::config_update_event::TrackingConfigUpdateAppliedEvent;
pub type TrackingConfigUpdateEventName =
    tracking::config_update_event::TrackingConfigUpdateEventName;
pub type TrackingConfigUpdateRequest = tracking::config_update_event::TrackingConfigUpdateRequest;
pub type TrackingConfigUpdateResponse = tracking::config_update_event::TrackingConfigUpdateResponse;
pub type TrackingConfigUpdateResponseState =
    tracking::config_update_event::TrackingConfigUpdateResponseState;
pub type TrackingConfigUpdateTargetScope =
    tracking::config_update_event::TrackingConfigUpdateTargetScope;
pub type TrackingDeleteAfterAlertResolutionState =
    tracking::retention_settings_write_command::TrackingDeleteAfterAlertResolutionState;
pub type TrackingDurableSettingsPersistenceState =
    tracking::retention_settings_write_command::TrackingDurableSettingsPersistenceState;
pub type TrackingDurableSettingsStoreRef = tracking::identifiers::TrackingDurableSettingsStoreRef;
pub type TrackingEvidenceRecordedEvent = tracking::runtime_event::TrackingEvidenceRecordedEvent;
pub type TrackingEvidenceRef = tracking::identifiers::TrackingEvidenceRef;
pub type TrackingEvaluationId = tracking::identifiers::TrackingEvaluationId;
pub type TrackingExecutionClaimState =
    tracking::retention_settings_write_command::TrackingExecutionClaimState;
pub type TrackingExpectedPlaceExceptionState =
    tracking::runtime_event::TrackingExpectedPlaceExceptionState;
pub type TrackingExpectedPlaceRef = tracking::identifiers::TrackingExpectedPlaceRef;
pub type TrackingExpectedPlaceState = tracking::identifiers::TrackingExpectedPlaceState;
pub type TrackingExpectedPlaceStateEvaluatedEvent =
    tracking::runtime_event::TrackingExpectedPlaceStateEvaluatedEvent;
pub type TrackingGeofenceRuleRef = tracking::identifiers::TrackingGeofenceRuleRef;
pub type TrackingGeofenceTransitionDetectedEvent =
    tracking::runtime_event::TrackingGeofenceTransitionDetectedEvent;
pub type TrackingLocationObservedEvent = tracking::runtime_event::TrackingLocationObservedEvent;
pub type TrackingLocationRelation = tracking::identifiers::TrackingLocationRelation;
pub type TrackingLocalServiceStateSnapshotRef =
    tracking::identifiers::TrackingLocalServiceStateSnapshotRef;
pub type TrackingMissingDeviceEvaluationId =
    tracking::identifiers::TrackingMissingDeviceEvaluationId;
pub type TrackingMissingDeviceState = tracking::identifiers::TrackingMissingDeviceState;
pub type TrackingMutationProofRef = tracking::identifiers::TrackingMutationProofRef;
pub type TrackingNearbyPlaceAmbiguityState =
    tracking::identifiers::TrackingNearbyPlaceAmbiguityState;
pub type TrackingNearbyPlaceClassifiedEvent =
    tracking::runtime_event::TrackingNearbyPlaceClassifiedEvent;
pub type TrackingNearbyPlaceProviderKind = tracking::identifiers::TrackingNearbyPlaceProviderKind;
pub type TrackingNearbyPlaceRequestId = tracking::identifiers::TrackingNearbyPlaceRequestId;
pub type TrackingNotificationChannel = tracking::identifiers::TrackingNotificationChannel;
pub type TrackingNotificationId = tracking::identifiers::TrackingNotificationId;
pub type TrackingNotificationMode = tracking::runtime_event::TrackingNotificationMode;
pub type TrackingObservationId = tracking::identifiers::TrackingObservationId;
pub type TrackingParentAcknowledgementRecordedEvent =
    tracking::runtime_event::TrackingParentAcknowledgementRecordedEvent;
pub type TrackingParentActionRequirement = tracking::runtime_event::TrackingParentActionRequirement;
pub type TrackingParentDefinedPlaceId = tracking::identifiers::TrackingParentDefinedPlaceId;
pub type TrackingParentDefinedPlaceState = tracking::identifiers::TrackingParentDefinedPlaceState;
pub type TrackingParentExportState =
    tracking::retention_settings_write_command::TrackingParentExportState;
pub type TrackingPlaceCategory = tracking::identifiers::TrackingPlaceCategory;
pub type TrackingPolicyRuleRef = tracking::identifiers::TrackingPolicyRuleRef;
pub type TrackingPolicySeverity = tracking::identifiers::TrackingPolicySeverity;
pub type TrackingPolicyViolationDetectedEvent =
    tracking::runtime_event::TrackingPolicyViolationDetectedEvent;
pub type TrackingPolicyViolationId = tracking::identifiers::TrackingPolicyViolationId;
pub type TrackingProviderRef = tracking::identifiers::TrackingProviderRef;
pub type TrackingReadModel = tracking::read_model::TrackingReadModel;
pub type TrackingReadModelCapabilityStatus =
    tracking::identifiers::TrackingReadModelCapabilityStatus;
pub type TrackingReadModelCount = tracking::read_model::TrackingReadModelCount;
pub type TrackingReadModelCountValue = tracking::identifiers::TrackingReadModelCountValue;
pub type TrackingReadModelCustodyLabel = tracking::identifiers::TrackingReadModelCustodyLabel;
pub type TrackingReadModelDeletedAt = tracking::identifiers::TrackingReadModelDeletedAt;
pub type TrackingReadModelDeviceId = tracking::identifiers::TrackingReadModelDeviceId;
pub type TrackingReadModelEventId = tracking::identifiers::TrackingReadModelEventId;
pub type TrackingReadModelGeneratedAt = tracking::identifiers::TrackingReadModelGeneratedAt;
pub type TrackingReadModelKind = tracking::identifiers::TrackingReadModelKind;
pub type TrackingReadModelObservedAt = tracking::identifiers::TrackingReadModelObservedAt;
pub type TrackingReadModelObserver = tracking::identifiers::TrackingReadModelObserver;
pub type TrackingReadModelPlatform = tracking::identifiers::TrackingReadModelPlatform;
pub type TrackingReadModelQueryVisibility = tracking::identifiers::TrackingReadModelQueryVisibility;
pub type TrackingReadModelRow = tracking::read_model::TrackingReadModelRow;
pub type TrackingReadModelSubjectDisplayName =
    tracking::identifiers::TrackingReadModelSubjectDisplayName;
pub type TrackingReadModelSubjectId = tracking::identifiers::TrackingReadModelSubjectId;
pub type TrackingReadModelSubjectKind = tracking::identifiers::TrackingReadModelSubjectKind;
pub type TrackingReasonCode = tracking::identifiers::TrackingReasonCode;
pub type TrackingRemoteAiState = tracking::retention_settings_write_command::TrackingRemoteAiState;
pub type TrackingRemoteSyncState =
    tracking::retention_settings_write_command::TrackingRemoteSyncState;
pub type TrackingRetentionCommandId = tracking::identifiers::TrackingRetentionCommandId;
pub type TrackingRetentionSettingsWriteRequest =
    tracking::retention_settings_write_command::TrackingRetentionSettingsWriteRequest;
pub type TrackingRetentionSettingsWriteResult =
    tracking::retention_settings_write_command::TrackingRetentionSettingsWriteResult;
pub type TrackingRetentionWriteState = tracking::identifiers::TrackingRetentionWriteState;
pub type TrackingRuntimeConfig = tracking::runtime_event::TrackingRuntimeConfig;
pub type TrackingRuntimeEnabledState = tracking::runtime_event::TrackingRuntimeEnabledState;
pub type TrackingRuntimeMode = tracking::runtime_event::TrackingRuntimeMode;
pub type TrackingScheduleId = tracking::identifiers::TrackingScheduleId;
pub type TrackingTemporaryLiveSessionId = tracking::identifiers::TrackingTemporaryLiveSessionId;
pub type TrackingTemporaryLiveState = tracking::identifiers::TrackingTemporaryLiveState;
pub type TrackingTimestamp = tracking::identifiers::TrackingTimestamp;
pub type TrackingTransitionId = tracking::identifiers::TrackingTransitionId;
pub type TrackingTransitionKind = tracking::identifiers::TrackingTransitionKind;
pub type TrackingUncertaintyCode = tracking::identifiers::TrackingUncertaintyCode;
pub type V08BroadAdapterRuntimeClaimState =
    enforcement_broad_adapter_proof::V08BroadAdapterRuntimeClaimState;
pub type V08BroadAdapterRuntimeEvidenceState =
    enforcement_broad_adapter_proof::V08BroadAdapterRuntimeEvidenceState;
pub type V08BroadAdapterRuntimeProofEntry =
    enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofEntry;
pub type V08BroadAdapterRuntimeProofReadModel =
    enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
pub type V08BroadAdapterRuntimeSurface =
    enforcement_broad_adapter_proof::V08BroadAdapterRuntimeSurface;
pub type V08BrowserDomainAdapterExecutionState =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterExecutionState;
pub type V08BrowserDomainAdapterProofCapabilityName =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityName;
pub type V08BrowserDomainAdapterProofCapabilityStatus =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityStatus;
pub type V08BrowserDomainAdapterProofClaimState =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofClaimState;
pub type V08BrowserDomainAdapterProofEntry =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEntry;
pub type V08BrowserDomainAdapterProofEvidenceKind =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEvidenceKind;
pub type V08BrowserDomainAdapterProofReadModel =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
pub type V08BrowserDomainAdapterProofSurface =
    enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofSurface;
pub type V08CrossPlatformAdapterExecutionState =
    enforcement_cross_platform_capability_proof::V08CrossPlatformAdapterExecutionState;
pub type V08CrossPlatformCapabilityStatus =
    enforcement_cross_platform_capability_proof::V08CrossPlatformCapabilityStatus;
pub type V08CrossPlatformEnforcementCapabilityClaimState =
    enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityClaimState;
pub type V08CrossPlatformEnforcementCapabilityName =
    enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityName;
pub type V08CrossPlatformEnforcementCapabilityProofEntry =
    enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofEntry;
pub type V08CrossPlatformEnforcementCapabilityProofReadModel = enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
pub type V08CrossPlatformEnforcementCapabilitySurface =
    enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilitySurface;
pub type V08EnforcementIntegrityRuntimeAuditAuditState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditAuditState;
pub type V08EnforcementIntegrityRuntimeAuditChildState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditChildState;
pub type V08EnforcementIntegrityRuntimeAuditEntry =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditEntry;
pub type V08EnforcementIntegrityRuntimeAuditExecution =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditExecution;
pub type V08EnforcementIntegrityRuntimeAuditIntegrityState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditIntegrityState;
pub type V08EnforcementIntegrityRuntimeAuditIntentState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditIntentState;
pub type V08EnforcementIntegrityRuntimeAuditReadModel =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel;
pub type V08EnforcementIntegrityRuntimeAuditResult =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditResult;
pub type V08EnforcementIntegrityRuntimeAuditRollbackState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditRollbackState;
pub type V08EnforcementIntegrityRuntimeAuditSurface =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditSurface;
pub type V08EnforcementIntegrityRuntimeAuditTimerState =
    enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditTimerState;
pub type V08EnforcementProductControlCapabilityName =
    enforcement_product_control_spine::V08EnforcementProductControlCapabilityName;
pub type V08EnforcementProductControlCapabilityStatus =
    enforcement_product_control_spine::V08EnforcementProductControlCapabilityStatus;
pub type V08EnforcementProductControlClaimState =
    enforcement_product_control_spine::V08EnforcementProductControlClaimState;
pub type V08EnforcementProductControlDevicePolicyState =
    enforcement_product_control_spine::V08EnforcementProductControlDevicePolicyState;
pub type V08EnforcementProductControlExecutionState =
    enforcement_product_control_spine::V08EnforcementProductControlExecutionState;
pub type V08EnforcementProductControlParentAction =
    enforcement_product_control_spine::V08EnforcementProductControlParentAction;
pub type V08EnforcementProductControlSpineEntry =
    enforcement_product_control_spine::V08EnforcementProductControlSpineEntry;
pub type V08EnforcementProductControlSpineReadModel =
    enforcement_product_control_spine::V08EnforcementProductControlSpineReadModel;
pub type V08EnforcementProductControlSurface =
    enforcement_product_control_spine::V08EnforcementProductControlSurface;
pub type V08EnforcementProductControlSurfaceKind =
    enforcement_product_control_spine::V08EnforcementProductControlSurfaceKind;
pub type V08OsAdapterProductProofAuditState =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofAuditState;
pub type V08OsAdapterProductProofEntry =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofEntry;
pub type V08OsAdapterProductProofParentOverrideState =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofParentOverrideState;
pub type V08OsAdapterProductProofReadModel =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofReadModel;
pub type V08OsAdapterProductProofSurface =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofSurface;
pub type V08OsAdapterProductProofTimerRecoveryState =
    enforcement_os_adapter_product_proof::V08OsAdapterProductProofTimerRecoveryState;
pub type V08SupportedAdapterAuditReferenceState =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterAuditReferenceState;
pub type V08SupportedAdapterCapability =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterCapability;
pub type V08SupportedAdapterPlatformSupportState =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterPlatformSupportState;
pub type V08SupportedAdapterRefusalReason =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRefusalReason;
pub type V08SupportedAdapterResult =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterResult;
pub type V08SupportedAdapterRollbackReferenceState =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRollbackReferenceState;
pub type V08SupportedAdapterRuntimeBoundary =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeBoundary;
pub type V08SupportedAdapterRuntimeProofEntry =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofEntry;
pub type V08SupportedAdapterRuntimeProofReadModel =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofReadModel;
pub type V08SupportedAdapterRuntimeState =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeState;
pub type V08SupportedAdapterTargetIdentityState =
    enforcement_supported_adapter_runtime_proof::V08SupportedAdapterTargetIdentityState;
pub type V08WindowsAppControlAdminRequirement =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlAdminRequirement;
pub type V08WindowsAppControlEventState =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlEventState;
pub type V08WindowsAppControlPolicyMutationState =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlPolicyMutationState;
pub type V08WindowsAppControlProofState =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlProofState;
pub type V08WindowsAppControlReadinessState =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlReadinessState;
pub type V08WindowsAppControlRuleIdentityKind =
    enforcement_browser_domain_adapter_proof::V08WindowsAppControlRuleIdentityKind;
pub type V09ProductionDiscoveryHouseholdProofState =
    lan_pairing::V09ProductionDiscoveryHouseholdProofState;
pub type V09ProductionDiscoveryHouseholdRuntimeOwner =
    lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner;
pub type WindowsAdapterCapabilityOutcome =
    windows_adapter_capability::WindowsAdapterCapabilityOutcome;
pub type WindowsAdapterCapabilityProof = windows_adapter_capability::WindowsAdapterCapabilityProof;
pub type WindowsAdapterCapabilityProofEntry =
    windows_adapter_capability::WindowsAdapterCapabilityProofEntry;
pub type WindowsAdapterCapabilitySurface =
    windows_adapter_capability::WindowsAdapterCapabilitySurface;

pub fn child_tracking_config_updated_event_from_parent(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> ChildTrackingConfigUpdatedEvent {
    tracking::config_update_event::child_tracking_config_updated_event_from_parent(parent_event)
}

pub fn default_tracking_retention_settings_write_request() -> TrackingRetentionSettingsWriteRequest
{
    tracking::retention_settings_write_command::default_tracking_retention_settings_write_request()
}

pub fn default_tracking_runtime_config() -> TrackingRuntimeConfig {
    tracking::runtime_event::default_tracking_runtime_config()
}

pub fn default_tracking_config_update_request() -> TrackingConfigUpdateRequest {
    tracking::config_update_event::default_tracking_config_update_request()
}

pub fn parent_tracking_config_updated_event_from_command(
    command: &AgentCommandEnvelope,
    request: TrackingConfigUpdateRequest,
) -> ParentTrackingConfigUpdatedEvent {
    tracking::config_update_event::parent_tracking_config_updated_event_from_command(
        command, request,
    )
}

pub fn policy_eligible_tracking_runtime_config() -> TrackingRuntimeConfig {
    tracking::runtime_event::policy_eligible_tracking_runtime_config()
}

pub fn policy_preview_finding_kinds_csv(kinds: &[PolicyPreviewFindingKind]) -> Option<String> {
    activity::policy_preview::policy_preview_finding_kinds_csv(kinds)
}

pub fn tracking_acknowledgement_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingAcknowledgementId {
    tracking::identifiers::tracking_acknowledgement_id_from_violation_id(violation_id)
}

pub fn tracking_ai_request_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingAiRequestId {
    tracking::identifiers::tracking_ai_request_id_from_evidence_ref(evidence_ref)
}

pub fn tracking_alert_evaluation_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingAlertEvaluationId {
    tracking::identifiers::tracking_alert_evaluation_id_from_violation_id(violation_id)
}

pub fn tracking_check_in_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingCheckInId {
    tracking::identifiers::tracking_check_in_id_from_observation_id(observation_id)
}

pub fn tracking_config_audit_entry_committed_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
    previous_event_ref: impl Into<String>,
    audit_outcome: TrackingConfigAuditOutcome,
) -> TrackingConfigAuditEntryCommittedEvent {
    tracking::config_update_event::tracking_config_audit_entry_committed_event(
        decision_event,
        previous_event_ref,
        audit_outcome,
    )
}

pub fn tracking_config_change_approved_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
) -> TrackingConfigChangeApprovedEvent {
    tracking::config_update_event::tracking_config_change_approved_event(decision_event)
}

pub fn tracking_config_change_rejected_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
    rejection_reason_code: impl Into<String>,
) -> TrackingConfigChangeRejectedEvent {
    tracking::config_update_event::tracking_config_change_rejected_event(
        decision_event,
        rejection_reason_code,
    )
}

pub fn tracking_config_change_requested_event(
    parent_action_event_ref: impl Into<String>,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> TrackingConfigChangeRequestedEvent {
    tracking::config_update_event::tracking_config_change_requested_event(
        parent_action_event_ref,
        parent_event,
    )
}

pub fn tracking_config_policy_decision_completed_event(
    evaluation_event: &TrackingConfigPolicyEvaluationRequestedEvent,
    decision_state: TrackingConfigPolicyDecisionState,
    child_runtime_publish_required: bool,
) -> TrackingConfigPolicyDecisionCompletedEvent {
    tracking::config_update_event::tracking_config_policy_decision_completed_event(
        evaluation_event,
        decision_state,
        child_runtime_publish_required,
    )
}

pub fn tracking_config_policy_evaluation_requested_event(
    requested_event: &TrackingConfigChangeRequestedEvent,
    parent_rule_refs: Vec<TrackingPolicyRuleRef>,
    dry_run: bool,
) -> TrackingConfigPolicyEvaluationRequestedEvent {
    tracking::config_update_event::tracking_config_policy_evaluation_requested_event(
        requested_event,
        parent_rule_refs,
        dry_run,
    )
}

pub fn tracking_config_portal_read_model_updated_event(
    audit_event: &TrackingConfigAuditEntryCommittedEvent,
    update_kind: TrackingConfigPortalUpdateKind,
    visible_manual_required: bool,
    visible_unavailable: bool,
) -> TrackingConfigPortalReadModelUpdatedEvent {
    tracking::config_update_event::tracking_config_portal_read_model_updated_event(
        audit_event,
        update_kind,
        visible_manual_required,
        visible_unavailable,
    )
}

pub fn tracking_config_update_applied_event_from_child(
    child_event: &ChildTrackingConfigUpdatedEvent,
    response_state: TrackingConfigUpdateResponseState,
    effective_tracking_state: TrackingConfigEffectiveState,
    local_service_state_revision: u64,
    durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
) -> TrackingConfigUpdateAppliedEvent {
    tracking::config_update_event::tracking_config_update_applied_event_from_child(
        child_event,
        response_state,
        effective_tracking_state,
        local_service_state_revision,
        durable_settings_persistence_state,
    )
}

pub fn tracking_durable_settings_store_ref() -> TrackingDurableSettingsStoreRef {
    tracking::retention_settings_write_command::tracking_durable_settings_store_ref()
}

pub fn tracking_evaluation_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingEvaluationId {
    tracking::identifiers::tracking_evaluation_id_from_observation_id(observation_id)
}

pub fn tracking_evidence_ref_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingEvidenceRef {
    tracking::identifiers::tracking_evidence_ref_from_observation_id(observation_id)
}

pub fn tracking_local_service_state_snapshot_ref() -> TrackingLocalServiceStateSnapshotRef {
    tracking::retention_settings_write_command::tracking_local_service_state_snapshot_ref()
}

pub fn tracking_missing_device_evaluation_id_from_child_device_id(
    child_device_id: &TrackingChildDeviceId,
) -> TrackingMissingDeviceEvaluationId {
    tracking::identifiers::tracking_missing_device_evaluation_id_from_child_device_id(
        child_device_id,
    )
}

pub fn tracking_mutation_proof_ref() -> TrackingMutationProofRef {
    tracking::retention_settings_write_command::tracking_mutation_proof_ref()
}

pub fn tracking_nearby_place_request_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingNearbyPlaceRequestId {
    tracking::identifiers::tracking_nearby_place_request_id_from_evidence_ref(evidence_ref)
}

pub fn tracking_notification_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingNotificationId {
    tracking::identifiers::tracking_notification_id_from_violation_id(violation_id)
}

pub fn tracking_parent_defined_place_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingParentDefinedPlaceId {
    tracking::identifiers::tracking_parent_defined_place_id_from_evidence_ref(evidence_ref)
}

pub fn tracking_read_model_payload(read_model: &TrackingReadModel) -> LogFields {
    tracking::read_model_payload::tracking_read_model_payload(read_model)
}

pub fn tracking_retention_accepted_at() -> TrackingAcceptedAt {
    tracking::retention_settings_write_command::tracking_retention_accepted_at()
}

pub fn tracking_retention_write_state_accepted() -> TrackingRetentionWriteState {
    tracking::retention_settings_write_command::tracking_retention_write_state_accepted()
}

pub fn tracking_retention_write_state_rejected() -> TrackingRetentionWriteState {
    tracking::retention_settings_write_command::tracking_retention_write_state_rejected()
}

pub fn tracking_temporary_live_session_id_from_child_device_id(
    child_device_id: &TrackingChildDeviceId,
) -> TrackingTemporaryLiveSessionId {
    tracking::identifiers::tracking_temporary_live_session_id_from_child_device_id(child_device_id)
}

pub fn tracking_transition_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingTransitionId {
    tracking::identifiers::tracking_transition_id_from_observation_id(observation_id)
}

pub fn tracking_violation_id_from_ai_request_and_rule_ref(
    ai_request_id: &TrackingAiRequestId,
    policy_rule_ref: &TrackingPolicyRuleRef,
) -> TrackingPolicyViolationId {
    tracking::identifiers::tracking_violation_id_from_ai_request_and_rule_ref(
        ai_request_id,
        policy_rule_ref,
    )
}

pub fn tracking_violation_id_from_evaluation_and_rule_ref(
    evaluation_id: &TrackingEvaluationId,
    policy_rule_ref: &TrackingPolicyRuleRef,
) -> TrackingPolicyViolationId {
    tracking::identifiers::tracking_violation_id_from_evaluation_and_rule_ref(
        evaluation_id,
        policy_rule_ref,
    )
}
