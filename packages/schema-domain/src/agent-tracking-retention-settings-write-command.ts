import { EventingEventTypeSchema } from './eventing';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';
import {
  AgentEventDeliveryMode,
  AgentEventEnvelopeSchema,
  AgentDeviceIdSchema,
  AgentMessageIdSchema,
  AgentPeerIdSchema,
  AgentPlatformSchema,
  AgentProtocolSchemaVersion,
  AgentRouteSchema,
} from './event-primitives';

const AgentTrackingSharedRuntimeEventTypeLiteral = {
  PolicyEvaluationRequested: 'policy.evaluation.requested',
  PolicyDecisionCompleted: 'policy.decision.completed',
  AuditEntryCommitted: 'audit.entry.committed',
  PortalReadModelUpdated: 'portal.read_model.updated',
} as const;

export const AgentTrackingRetentionCommandIdSchema = brandedNonEmptyStringSchema(
  'AgentTrackingRetentionCommandId'
);
export const AgentTrackingWriterIntentRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingWriterIntentRef'
);
export const AgentTrackingReadModelProofRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingReadModelProofRef'
);
export const AgentTrackingMutationProofRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingMutationProofRef'
);
export const AgentTrackingAcceptedAtSchema = brandedNonEmptyStringSchema('AgentTrackingAcceptedAt');
export const AgentTrackingEventRefSchema = brandedNonEmptyStringSchema('AgentTrackingEventRef');
export const AgentTrackingLocalServiceStateSnapshotRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingLocalServiceStateSnapshotRef'
);
export const AgentTrackingDurableSettingsStoreRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingDurableSettingsStoreRef'
);
export const AgentTrackingPolicyRuleRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingPolicyRuleRef'
);
export const AgentTrackingReadModelEventIdSchema = brandedNonEmptyStringSchema(
  'AgentTrackingReadModelEventId'
);
export const AgentTrackingRejectionReasonCodeSchema = brandedNonEmptyStringSchema(
  'AgentTrackingRejectionReasonCode'
);

export const AgentTrackingConfigUpdateEventType = {
  Parent: EventingEventTypeSchema.parse('tracking.config.updated.parent'),
  Child: EventingEventTypeSchema.parse('tracking.config.updated.child'),
  Applied: EventingEventTypeSchema.parse('tracking.config.applied.child'),
} as const;

export const AgentTrackingConfigCommandFlowEventType = {
  ChangeRequested: EventingEventTypeSchema.parse('tracking.config.change_requested'),
  ChangeApproved: EventingEventTypeSchema.parse('tracking.config.change_approved'),
  ChangeRejected: EventingEventTypeSchema.parse('tracking.config.change_rejected'),
  PolicyEvaluationRequested: EventingEventTypeSchema.parse(
    AgentTrackingSharedRuntimeEventTypeLiteral.PolicyEvaluationRequested
  ),
  PolicyDecisionCompleted: EventingEventTypeSchema.parse(
    AgentTrackingSharedRuntimeEventTypeLiteral.PolicyDecisionCompleted
  ),
  AuditEntryCommitted: EventingEventTypeSchema.parse(
    AgentTrackingSharedRuntimeEventTypeLiteral.AuditEntryCommitted
  ),
  PortalReadModelUpdated: EventingEventTypeSchema.parse(
    AgentTrackingSharedRuntimeEventTypeLiteral.PortalReadModelUpdated
  ),
} as const;

export const AgentTrackingConfigUpdateTargetScopeLiteral = {
  Family: 'family',
  ChildProfile: 'child-profile',
  ChildDevice: 'child-device',
  DeviceGroup: 'device-group',
} as const;

export const AgentTrackingConfigUpdateResponseStateLiteral = {
  Applied: 'applied',
  Rejected: 'rejected',
} as const;

export const AgentTrackingEffectiveStateLiteral = {
  Enabled: 'enabled',
  Disabled: 'disabled',
  Degraded: 'degraded',
} as const;

export const AgentTrackingDeleteAfterAlertResolutionStateLiteral = {
  DeleteAfterAlertResolved: 'delete-after-alert-resolved',
  RetainAfterAlertResolved: 'retain-after-alert-resolved',
} as const;

export const AgentTrackingParentExportStateLiteral = {
  Prepared: 'prepared',
  NotPrepared: 'not-prepared',
} as const;

export const AgentTrackingEnabledStateLiteral = {
  Enabled: 'enabled',
  Disabled: 'disabled',
} as const;

export const AgentTrackingRuntimeModeLiteral = {
  ObserveOnly: 'observe-only',
  PolicyEligible: 'policy-eligible',
} as const;

export const AgentTrackingAiBoundaryModeLiteral = {
  RequestWhenUncertain: 'request-when-uncertain',
  Disabled: 'disabled',
} as const;

export const AgentTrackingNotificationModeLiteral = {
  ParentPortalOnly: 'portal-only',
  Disabled: 'disabled',
} as const;

export const AgentTrackingDurableSettingsPersistenceStateLiteral = {
  Persisted: 'persisted',
  NotPersisted: 'not-persisted',
} as const;

export const AgentTrackingConfigAckStateLiteral = {
  Received: 'received',
  Missing: 'missing',
} as const;

export const AgentTrackingExecutionClaimStateLiteral = {
  Claimed: 'claimed',
  Unclaimed: 'unclaimed',
} as const;

export const AgentTrackingConfigPolicyDecisionStateLiteral = {
  Approved: 'approved',
  Rejected: 'rejected',
} as const;

export const AgentTrackingConfigAuditOutcomeLiteral = {
  Committed: 'committed',
  Failed: 'failed',
} as const;

export const AgentTrackingConfigPortalUpdateKindLiteral = {
  TrackingConfigState: 'tracking-config-state',
  ManualRequiredState: 'manual-required-state',
} as const;

export const AgentTrackingRetentionSettingsWriteDefaults = {
  CommandId: 'tracking-retention-settings-write-command',
  SettingsKindRetentionWindow: 'retention-window-setting',
  WriterIntentRef: 'tracking-retention-settings-write-retention-window',
  ReadModelProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
  ],
  MutationProofRef:
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
  LocalServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
  DurableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
  WriteStateAccepted: 'service-write-command-accepted',
  WriteStateRejected: 'service-write-command-rejected',
  AcceptedAt: '2026-06-06T19:50:00Z',
} as const;

export const AgentTrackingRetentionSettingsWriteKindSchema = withParser(
  Schema.Literal(
    AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    'delete-after-alert-setting',
    'parent-export-setting',
    'remote-sync-disabled-setting',
    'remote-ai-disabled-setting'
  )
);

export const AgentTrackingRetentionSettingsWriteStateSchema = withParser(
  Schema.Literal(
    AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
    AgentTrackingRetentionSettingsWriteDefaults.WriteStateRejected
  )
);

export const AgentTrackingConfigUpdateResponseStateSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateResponseStateLiteral.Applied,
    AgentTrackingConfigUpdateResponseStateLiteral.Rejected
  )
);

export const AgentTrackingEffectiveStateSchema = withParser(
  Schema.Literal(
    AgentTrackingEffectiveStateLiteral.Enabled,
    AgentTrackingEffectiveStateLiteral.Disabled,
    AgentTrackingEffectiveStateLiteral.Degraded
  )
);

export const AgentTrackingDeleteAfterAlertResolutionStateSchema = withParser(
  Schema.Literal(
    AgentTrackingDeleteAfterAlertResolutionStateLiteral.DeleteAfterAlertResolved,
    AgentTrackingDeleteAfterAlertResolutionStateLiteral.RetainAfterAlertResolved
  )
);

export const AgentTrackingParentExportStateSchema = withParser(
  Schema.Literal(
    AgentTrackingParentExportStateLiteral.Prepared,
    AgentTrackingParentExportStateLiteral.NotPrepared
  )
);

export const AgentTrackingRuntimeEnabledStateSchema = withParser(
  Schema.Literal(AgentTrackingEnabledStateLiteral.Enabled, AgentTrackingEnabledStateLiteral.Disabled)
);

export const AgentTrackingRuntimeModeSchema = withParser(
  Schema.Literal(
    AgentTrackingRuntimeModeLiteral.ObserveOnly,
    AgentTrackingRuntimeModeLiteral.PolicyEligible
  )
);

export const AgentTrackingAiBoundaryModeSchema = withParser(
  Schema.Literal(
    AgentTrackingAiBoundaryModeLiteral.RequestWhenUncertain,
    AgentTrackingAiBoundaryModeLiteral.Disabled
  )
);

export const AgentTrackingNotificationModeSchema = withParser(
  Schema.Literal(
    AgentTrackingNotificationModeLiteral.ParentPortalOnly,
    AgentTrackingNotificationModeLiteral.Disabled
  )
);

export const AgentTrackingRemoteSyncStateSchema = withParser(
  Schema.Literal(AgentTrackingEnabledStateLiteral.Enabled, AgentTrackingEnabledStateLiteral.Disabled)
);

export const AgentTrackingRemoteAiStateSchema = withParser(
  Schema.Literal(AgentTrackingEnabledStateLiteral.Enabled, AgentTrackingEnabledStateLiteral.Disabled)
);

export const AgentTrackingDurableSettingsPersistenceStateSchema = withParser(
  Schema.Literal(
    AgentTrackingDurableSettingsPersistenceStateLiteral.Persisted,
    AgentTrackingDurableSettingsPersistenceStateLiteral.NotPersisted
  )
);

export const AgentTrackingConfigAckStateSchema = withParser(
  Schema.Literal(AgentTrackingConfigAckStateLiteral.Received, AgentTrackingConfigAckStateLiteral.Missing)
);

export const AgentTrackingExecutionClaimStateSchema = withParser(
  Schema.Literal(AgentTrackingExecutionClaimStateLiteral.Claimed, AgentTrackingExecutionClaimStateLiteral.Unclaimed)
);

export const AgentTrackingConfigPolicyDecisionStateSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigPolicyDecisionStateLiteral.Approved,
    AgentTrackingConfigPolicyDecisionStateLiteral.Rejected
  )
);

export const AgentTrackingConfigAuditOutcomeSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigAuditOutcomeLiteral.Committed,
    AgentTrackingConfigAuditOutcomeLiteral.Failed
  )
);

export const AgentTrackingConfigPortalUpdateKindSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigPortalUpdateKindLiteral.TrackingConfigState,
    AgentTrackingConfigPortalUpdateKindLiteral.ManualRequiredState
  )
);

export const AgentTrackingDeleteAfterAlertResolutionState = {
  DeleteAfterAlertResolved: AgentTrackingDeleteAfterAlertResolutionStateSchema.parse(
    AgentTrackingDeleteAfterAlertResolutionStateLiteral.DeleteAfterAlertResolved
  ),
  RetainAfterAlertResolved: AgentTrackingDeleteAfterAlertResolutionStateSchema.parse(
    AgentTrackingDeleteAfterAlertResolutionStateLiteral.RetainAfterAlertResolved
  ),
} as const;

export const AgentTrackingParentExportState = {
  Prepared: AgentTrackingParentExportStateSchema.parse(AgentTrackingParentExportStateLiteral.Prepared),
  NotPrepared: AgentTrackingParentExportStateSchema.parse(AgentTrackingParentExportStateLiteral.NotPrepared),
} as const;

export const AgentTrackingRuntimeEnabledState = {
  Enabled: AgentTrackingRuntimeEnabledStateSchema.parse(AgentTrackingEnabledStateLiteral.Enabled),
  Disabled: AgentTrackingRuntimeEnabledStateSchema.parse(AgentTrackingEnabledStateLiteral.Disabled),
} as const;

export const AgentTrackingRuntimeMode = {
  ObserveOnly: AgentTrackingRuntimeModeSchema.parse(AgentTrackingRuntimeModeLiteral.ObserveOnly),
  PolicyEligible: AgentTrackingRuntimeModeSchema.parse(AgentTrackingRuntimeModeLiteral.PolicyEligible),
} as const;

export const AgentTrackingAiBoundaryMode = {
  RequestWhenUncertain: AgentTrackingAiBoundaryModeSchema.parse(
    AgentTrackingAiBoundaryModeLiteral.RequestWhenUncertain
  ),
  Disabled: AgentTrackingAiBoundaryModeSchema.parse(AgentTrackingAiBoundaryModeLiteral.Disabled),
} as const;

export const AgentTrackingNotificationMode = {
  ParentPortalOnly: AgentTrackingNotificationModeSchema.parse(
    AgentTrackingNotificationModeLiteral.ParentPortalOnly
  ),
  Disabled: AgentTrackingNotificationModeSchema.parse(AgentTrackingNotificationModeLiteral.Disabled),
} as const;

export const AgentTrackingRemoteSyncState = {
  Enabled: AgentTrackingRemoteSyncStateSchema.parse(AgentTrackingEnabledStateLiteral.Enabled),
  Disabled: AgentTrackingRemoteSyncStateSchema.parse(AgentTrackingEnabledStateLiteral.Disabled),
} as const;

export const AgentTrackingRemoteAiState = {
  Enabled: AgentTrackingRemoteAiStateSchema.parse(AgentTrackingEnabledStateLiteral.Enabled),
  Disabled: AgentTrackingRemoteAiStateSchema.parse(AgentTrackingEnabledStateLiteral.Disabled),
} as const;

export const AgentTrackingDurableSettingsPersistenceState = {
  Persisted: AgentTrackingDurableSettingsPersistenceStateSchema.parse(
    AgentTrackingDurableSettingsPersistenceStateLiteral.Persisted
  ),
  NotPersisted: AgentTrackingDurableSettingsPersistenceStateSchema.parse(
    AgentTrackingDurableSettingsPersistenceStateLiteral.NotPersisted
  ),
} as const;

export const AgentTrackingConfigAckState = {
  Received: AgentTrackingConfigAckStateSchema.parse(AgentTrackingConfigAckStateLiteral.Received),
  Missing: AgentTrackingConfigAckStateSchema.parse(AgentTrackingConfigAckStateLiteral.Missing),
} as const;

export const AgentTrackingExecutionClaimState = {
  Claimed: AgentTrackingExecutionClaimStateSchema.parse(AgentTrackingExecutionClaimStateLiteral.Claimed),
  Unclaimed: AgentTrackingExecutionClaimStateSchema.parse(AgentTrackingExecutionClaimStateLiteral.Unclaimed),
} as const;

const AgentTrackingNonEmptyPolicyRuleRefsSchema = Schema.Array(AgentTrackingPolicyRuleRefSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Tracking config flow requires parent policy rule refs')
);

export const AgentTrackingConfigUpdateEventNameSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateEventType.Parent,
    AgentTrackingConfigUpdateEventType.Child,
    AgentTrackingConfigUpdateEventType.Applied
  )
);

export const AgentTrackingConfigUpdateTargetScopeSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateTargetScopeLiteral.Family,
    AgentTrackingConfigUpdateTargetScopeLiteral.ChildProfile,
    AgentTrackingConfigUpdateTargetScopeLiteral.ChildDevice,
    AgentTrackingConfigUpdateTargetScopeLiteral.DeviceGroup
  )
);

export const AgentTrackingRetentionSettingsWriteRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    commandId: AgentTrackingRetentionCommandIdSchema,
    settingsKind: AgentTrackingRetentionSettingsWriteKindSchema,
    requestedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    requestedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionStateSchema,
    requestedParentExportState: AgentTrackingParentExportStateSchema,
    requestedRemoteSyncState: Schema.Literal(AgentTrackingRemoteSyncState.Disabled),
    requestedRemoteAiState: Schema.Literal(AgentTrackingRemoteAiState.Disabled),
    sourceWriterIntentRefs: Schema.Array(AgentTrackingWriterIntentRefSchema),
    sourceReadModelProofRefs: Schema.Array(AgentTrackingReadModelProofRefSchema),
  })
    .pipe(Schema.filter((request) => request.sourceWriterIntentRefs.length > 0 || 'Write request needs intent refs'))
    .pipe(
      Schema.filter(
        (request) => request.sourceReadModelProofRefs.length > 0 || 'Write request needs source read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.settingsKind !== AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow ||
          request.requestedRetentionWindowHours !== null ||
          'Retention-window write requests must include a retention window'
      )
    )
);

export const AgentTrackingRuntimeConfigSchema = withParser(
  Schema.Struct({
    trackingEnabledState: AgentTrackingRuntimeEnabledStateSchema,
    trackingMode: AgentTrackingRuntimeModeSchema,
    aiBoundaryMode: AgentTrackingAiBoundaryModeSchema,
    notificationMode: AgentTrackingNotificationModeSchema,
  })
);

export const AgentTrackingConfigUpdateRequestSchema = withParser(
  Schema.Struct({
    commandId: AgentTrackingRetentionCommandIdSchema,
    runtimeConfig: AgentTrackingRuntimeConfigSchema,
    retentionSettings: AgentTrackingRetentionSettingsWriteRequestSchema,
  }).pipe(
    Schema.filter(
      (request) =>
        request.commandId === request.retentionSettings.commandId ||
        'Tracking config update request commandId must match nested retentionSettings commandId'
    )
  )
);

export const AgentTrackingConfigUpdateTargetSchema = withParser(
  Schema.Struct({
    scope: AgentTrackingConfigUpdateTargetScopeSchema,
    deviceId: AgentDeviceIdSchema,
    platform: AgentPlatformSchema,
    route: AgentRouteSchema,
  })
);

export const ParentTrackingConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    sourceMessageId: AgentMessageIdSchema,
    sourcePeerId: AgentPeerIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingConfigUpdateRequestSchema,
  })
);

export const ChildTrackingConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    parentEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Parent),
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingConfigUpdateRequestSchema,
  })
);

export const TrackingConfigUpdateAppliedEventSchema = withParser(
  Schema.Struct({
    parentEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Parent),
    childEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Child),
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    responseState: AgentTrackingConfigUpdateResponseStateSchema,
    effectiveTrackingState: AgentTrackingEffectiveStateSchema,
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceStateSchema,
  })
);

export const TrackingRuntimeEventNameLiteral = {
  LocationObserved: 'tracking.location.observed',
  EvidenceRecorded: 'tracking.evidence.recorded',
  AiAnalysisRequested: 'tracking.ai.analysis.requested',
  NearbyPlaceClassified: 'tracking.nearby-place.classified',
  GeofenceTransitionDetected: 'tracking.geofence.transition.detected',
  ExpectedPlaceStateEvaluated: 'tracking.expected-place.state.evaluated',
  PolicyViolationDetected: 'tracking.policy.violation.detected',
  ParentAcknowledgementRecorded: 'tracking.parent-acknowledgement.recorded',
  ChildCheckInRecorded: 'tracking.child-check-in.recorded',
  ParentNotificationRequested: 'tracking.parent.notification.requested',
} as const;

export const TrackingEventNameSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateEventType.Parent,
    AgentTrackingConfigUpdateEventType.Child,
    AgentTrackingConfigUpdateEventType.Applied,
    TrackingRuntimeEventNameLiteral.LocationObserved,
    TrackingRuntimeEventNameLiteral.EvidenceRecorded,
    TrackingRuntimeEventNameLiteral.AiAnalysisRequested,
    TrackingRuntimeEventNameLiteral.NearbyPlaceClassified,
    TrackingRuntimeEventNameLiteral.GeofenceTransitionDetected,
    TrackingRuntimeEventNameLiteral.ExpectedPlaceStateEvaluated,
    TrackingRuntimeEventNameLiteral.PolicyViolationDetected,
    TrackingRuntimeEventNameLiteral.ParentAcknowledgementRecorded,
    TrackingRuntimeEventNameLiteral.ChildCheckInRecorded,
    TrackingRuntimeEventNameLiteral.ParentNotificationRequested
  ).pipe(
    Schema.filter(
      (eventName) =>
        EventingEventTypeSchema.safeParse(eventName).success ||
        'Expected tracking event name to satisfy the shared eventing taxonomy'
    )
  )
);

export const TrackingRuntimeConfigUpdatedPayloadSchema = AgentTrackingConfigUpdateRequestSchema;

export const TrackingRuntimeConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingRuntimeConfigUpdatedPayloadSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ConfigUpdated &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.RequestResponse) ||
        'Tracking config update events use the tracking-owned payload schema and request-response delivery'
    )
  )
);

export const TrackingRuntimeChildConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingRuntimeConfigUpdatedPayloadSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ChildConfigUpdated &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget) ||
        'Child tracking config update events use the tracking-owned payload schema and fire-and-forget child delivery'
    )
  )
);

export const TrackingRuntimeChildConfigAppliedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingConfigUpdateAppliedEventSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ChildConfigApplied &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget) ||
        'Child tracking config applied events use the canonical applied payload schema and fire-and-forget delivery'
    )
  )
);

export const TrackingRuntimeEventEnvelopeSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    eventName: TrackingEventNameSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.envelope.eventName === event.eventName ||
        'Expected tracking runtime envelope eventName to match the typed tracking event name'
    ),
    Schema.filter(
      (event) =>
        event.eventName !== TrackingEventName.ConfigUpdated ||
        event.envelope.deliveryMode === AgentEventDeliveryMode.RequestResponse ||
        'Tracking config updates require request-response delivery'
    ),
    Schema.filter(
      (event) =>
        event.eventName === TrackingEventName.ConfigUpdated ||
        event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget ||
        'Tracking runtime evidence-flow events are fire-and-forget events'
    )
  )
);

export const TrackingConfigUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    responseState: AgentTrackingConfigUpdateResponseStateSchema,
    effectiveTrackingState: AgentTrackingEffectiveStateSchema,
    childEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Child),
    target: AgentTrackingConfigUpdateTargetSchema,
    localServiceStateRevision: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceStateSchema,
  })
);

export const TrackingConfigChangeRequestedEventSchema = withParser(
  Schema.Struct({
    changeRequestedEventRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    sourceMessageId: AgentMessageIdSchema,
    sourcePeerId: AgentPeerIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingConfigUpdateRequestSchema,
    requestedAt: AgentTrackingAcceptedAtSchema,
  })
);

export const TrackingConfigPolicyEvaluationRequestedEventSchema = withParser(
  Schema.Struct({
    policyEvaluationRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    parentRuleRefs: AgentTrackingNonEmptyPolicyRuleRefsSchema,
    dryRun: Schema.Boolean,
  })
);

export const TrackingConfigPolicyDecisionCompletedEventSchema = withParser(
  Schema.Struct({
    policyDecisionRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    decisionState: AgentTrackingConfigPolicyDecisionStateSchema,
    parentRuleRefs: AgentTrackingNonEmptyPolicyRuleRefsSchema,
    childRuntimePublishRequired: Schema.Boolean,
  })
);

export const TrackingConfigChangeApprovedEventSchema = withParser(
  Schema.Struct({
    changeApprovedEventRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    approvedAt: AgentTrackingAcceptedAtSchema,
    childRuntimePublishRequired: Schema.Boolean,
  })
);

export const TrackingConfigChangeRejectedEventSchema = withParser(
  Schema.Struct({
    changeRejectedEventRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    rejectedAt: AgentTrackingAcceptedAtSchema,
    rejectionReasonCode: AgentTrackingRejectionReasonCodeSchema,
  })
);

export const TrackingConfigAuditEntryCommittedEventSchema = withParser(
  Schema.Struct({
    auditEntryRef: AgentTrackingEventRefSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    policyDecisionRef: AgentTrackingEventRefSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    auditOutcome: AgentTrackingConfigAuditOutcomeSchema,
  })
);

export const TrackingConfigPortalReadModelUpdatedEventSchema = withParser(
  Schema.Struct({
    readModelRef: AgentTrackingReadModelEventIdSchema,
    previousEventRef: AgentTrackingEventRefSchema,
    auditEntryRef: AgentTrackingEventRefSchema,
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    updateKind: AgentTrackingConfigPortalUpdateKindSchema,
    visibleManualRequired: Schema.Boolean,
    visibleUnavailable: Schema.Boolean,
  })
);

export const AgentTrackingRetentionSettingsWriteResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    commandId: AgentTrackingRetentionCommandIdSchema,
    settingsKind: AgentTrackingRetentionSettingsWriteKindSchema,
    writeState: AgentTrackingRetentionSettingsWriteStateSchema,
    acceptedAt: AgentTrackingAcceptedAtSchema,
    sourceWriterIntentRefs: Schema.Array(AgentTrackingWriterIntentRefSchema),
    sourceReadModelProofRefs: Schema.Array(AgentTrackingReadModelProofRefSchema),
    sourceMutationProofRefs: Schema.Array(AgentTrackingMutationProofRefSchema),
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionStateSchema,
    parentExportState: AgentTrackingParentExportStateSchema,
    remoteSyncState: Schema.Literal(AgentTrackingRemoteSyncState.Disabled),
    remoteAiState: Schema.Literal(AgentTrackingRemoteAiState.Disabled),
    localServiceStateRevision: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    localServiceStateSnapshotRef: AgentTrackingLocalServiceStateSnapshotRefSchema,
    durableSettingsStoreRef: AgentTrackingDurableSettingsStoreRefSchema,
    durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceStateSchema,
    childConfigResponseState: Schema.optional(
      Schema.Union(AgentTrackingConfigUpdateResponseStateSchema, Schema.Null)
    ),
    effectiveTrackingState: Schema.optional(
      Schema.Union(AgentTrackingEffectiveStateSchema, Schema.Null)
    ),
    childConfigAckState: Schema.optionalWith(AgentTrackingConfigAckStateSchema, {
      default: () => AgentTrackingConfigAckState.Missing,
    }),
    commandTransportClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Claimed),
    serviceWritePreflightClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Claimed),
    serviceMutationExecutionState: AgentTrackingExecutionClaimStateSchema,
    portalWritableUiClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    platformRuntimeClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    childDeviceDeliveryClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    providerDeliveryClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    notificationReceiptClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    physicalDeviceClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    authorityClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
    productClaimState: Schema.Literal(AgentTrackingExecutionClaimState.Unclaimed),
  })
    .pipe(Schema.filter((result) => result.sourceWriterIntentRefs.length > 0 || 'Write result needs intent refs'))
    .pipe(
      Schema.filter(
        (result) => result.sourceReadModelProofRefs.length > 0 || 'Write result needs source read-model proof refs'
      )
    )
    .pipe(Schema.filter((result) => result.sourceMutationProofRefs.length > 0 || 'Write result needs proof refs'))
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted ||
          result.commandTransportClaimState === AgentTrackingExecutionClaimState.Claimed ||
          'Accepted write result must prove command transport'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted ||
          result.serviceMutationExecutionState === AgentTrackingExecutionClaimState.Claimed ||
          'Accepted write result must execute the local service mutation'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.settingsKind !== AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow ||
          result.appliedRetentionWindowHours !== null ||
          'Retention-window write results must include the applied retention window'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted ||
          result.localServiceStateRevision !== null ||
          'Accepted write results must include a local service state revision'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted ||
          result.durableSettingsPersistenceState === AgentTrackingDurableSettingsPersistenceState.Persisted ||
          'Accepted write results must persist local durable settings'
      )
    )
);

export type AgentTrackingRetentionSettingsWriteKind = Infer<typeof AgentTrackingRetentionSettingsWriteKindSchema>;
export type AgentTrackingDeleteAfterAlertResolutionState = Infer<
  typeof AgentTrackingDeleteAfterAlertResolutionStateSchema
>;
export type AgentTrackingParentExportState = Infer<typeof AgentTrackingParentExportStateSchema>;
export type AgentTrackingRemoteSyncState = Infer<typeof AgentTrackingRemoteSyncStateSchema>;
export type AgentTrackingRemoteAiState = Infer<typeof AgentTrackingRemoteAiStateSchema>;
export type AgentTrackingDurableSettingsPersistenceState = Infer<
  typeof AgentTrackingDurableSettingsPersistenceStateSchema
>;
export type AgentTrackingConfigAckState = Infer<typeof AgentTrackingConfigAckStateSchema>;
export type AgentTrackingExecutionClaimState = Infer<typeof AgentTrackingExecutionClaimStateSchema>;
export type AgentTrackingRetentionSettingsWriteRequest = Infer<typeof AgentTrackingRetentionSettingsWriteRequestSchema>;
export type AgentTrackingRetentionSettingsWriteResult = Infer<typeof AgentTrackingRetentionSettingsWriteResultSchema>;
export type AgentTrackingRuntimeEnabledState = Infer<typeof AgentTrackingRuntimeEnabledStateSchema>;
export type AgentTrackingRuntimeMode = Infer<typeof AgentTrackingRuntimeModeSchema>;
export type AgentTrackingAiBoundaryMode = Infer<typeof AgentTrackingAiBoundaryModeSchema>;
export type AgentTrackingNotificationMode = Infer<typeof AgentTrackingNotificationModeSchema>;
export type AgentTrackingRuntimeConfig = Infer<typeof AgentTrackingRuntimeConfigSchema>;
export type AgentTrackingConfigUpdateRequest = Infer<typeof AgentTrackingConfigUpdateRequestSchema>;
export type AgentTrackingConfigUpdateTargetScope = Infer<typeof AgentTrackingConfigUpdateTargetScopeSchema>;
export type AgentTrackingConfigUpdateTarget = Infer<typeof AgentTrackingConfigUpdateTargetSchema>;
export type AgentTrackingConfigPolicyDecisionState = Infer<
  typeof AgentTrackingConfigPolicyDecisionStateSchema
>;
export type AgentTrackingConfigAuditOutcome = Infer<typeof AgentTrackingConfigAuditOutcomeSchema>;
export type AgentTrackingConfigPortalUpdateKind = Infer<
  typeof AgentTrackingConfigPortalUpdateKindSchema
>;
export type TrackingEventName = Infer<typeof TrackingEventNameSchema>;
export type TrackingRuntimeEnabledState = AgentTrackingRuntimeEnabledState;
export type TrackingRuntimeConfigUpdatedPayload = Infer<typeof TrackingRuntimeConfigUpdatedPayloadSchema>;
export type TrackingRuntimeConfigUpdatedEvent = Infer<typeof TrackingRuntimeConfigUpdatedEventSchema>;
export type TrackingRuntimeChildConfigUpdatedEvent = Infer<
  typeof TrackingRuntimeChildConfigUpdatedEventSchema
>;
export type TrackingRuntimeChildConfigAppliedEvent = Infer<
  typeof TrackingRuntimeChildConfigAppliedEventSchema
>;
export type TrackingRuntimeEventEnvelope = Infer<typeof TrackingRuntimeEventEnvelopeSchema>;
export type ParentTrackingConfigUpdatedEvent = Infer<typeof ParentTrackingConfigUpdatedEventSchema>;
export type ChildTrackingConfigUpdatedEvent = Infer<typeof ChildTrackingConfigUpdatedEventSchema>;
export type TrackingConfigUpdateAppliedEvent = Infer<typeof TrackingConfigUpdateAppliedEventSchema>;
export type TrackingConfigUpdateResponse = Infer<typeof TrackingConfigUpdateResponseSchema>;
export type TrackingConfigChangeRequestedEvent = Infer<typeof TrackingConfigChangeRequestedEventSchema>;
export type TrackingConfigPolicyEvaluationRequestedEvent = Infer<
  typeof TrackingConfigPolicyEvaluationRequestedEventSchema
>;
export type TrackingConfigPolicyDecisionCompletedEvent = Infer<
  typeof TrackingConfigPolicyDecisionCompletedEventSchema
>;
export type TrackingConfigChangeApprovedEvent = Infer<typeof TrackingConfigChangeApprovedEventSchema>;
export type TrackingConfigChangeRejectedEvent = Infer<typeof TrackingConfigChangeRejectedEventSchema>;
export type TrackingConfigAuditEntryCommittedEvent = Infer<
  typeof TrackingConfigAuditEntryCommittedEventSchema
>;
export type TrackingConfigPortalReadModelUpdatedEvent = Infer<
  typeof TrackingConfigPortalReadModelUpdatedEventSchema
>;

export const TrackingEventName = {
  ConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Parent),
  ChildConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Child),
  ChildConfigApplied: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Applied),
  LocationObserved: TrackingEventNameSchema.parse(TrackingRuntimeEventNameLiteral.LocationObserved),
  EvidenceRecorded: TrackingEventNameSchema.parse(TrackingRuntimeEventNameLiteral.EvidenceRecorded),
  AiAnalysisRequested: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.AiAnalysisRequested
  ),
  NearbyPlaceClassified: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.NearbyPlaceClassified
  ),
  GeofenceTransitionDetected: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.GeofenceTransitionDetected
  ),
  ExpectedPlaceStateEvaluated: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ExpectedPlaceStateEvaluated
  ),
  PolicyViolationDetected: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.PolicyViolationDetected
  ),
  ParentAcknowledgementRecorded: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ParentAcknowledgementRecorded
  ),
  ChildCheckInRecorded: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ChildCheckInRecorded
  ),
  ParentNotificationRequested: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ParentNotificationRequested
  ),
} as const;

export const TrackingRuntimeEnabledState = AgentTrackingRuntimeEnabledState;
