import { EventingEventTypeSchema } from '@ocentra-parent/event-domain/eventing';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import {
  AgentDeviceIdSchema,
  AgentMessageIdSchema,
  AgentPeerIdSchema,
  AgentPlatformSchema,
  AgentRouteSchema,
} from '@ocentra-parent/event-domain/primitives';

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
export const AgentTrackingLocalServiceStateSnapshotRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingLocalServiceStateSnapshotRef'
);
export const AgentTrackingDurableSettingsStoreRefSchema = brandedNonEmptyStringSchema(
  'AgentTrackingDurableSettingsStoreRef'
);

export const AgentTrackingConfigUpdateEventType = {
  Parent: EventingEventTypeSchema.parse('tracking.config.updated.parent'),
  Child: EventingEventTypeSchema.parse('tracking.config.updated.child'),
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

export const AgentTrackingConfigUpdateEventNameSchema = withParser(
  Schema.Literal(AgentTrackingConfigUpdateEventType.Parent, AgentTrackingConfigUpdateEventType.Child)
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
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
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
    config: AgentTrackingRetentionSettingsWriteRequestSchema,
  })
);

export const ChildTrackingConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    parentEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Parent),
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingRetentionSettingsWriteRequestSchema,
  })
);

export const TrackingConfigUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    sourceCommandId: AgentTrackingRetentionCommandIdSchema,
    responseState: AgentTrackingConfigUpdateResponseStateSchema,
    effectiveTrackingState: AgentTrackingEffectiveStateSchema,
    childEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Child),
    target: AgentTrackingConfigUpdateTargetSchema,
    localServiceStateRevision: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceStateSchema,
  })
);

export const AgentTrackingRetentionSettingsWriteResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
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
    childConfigResponseState: Schema.optionalWith(
      Schema.Union(AgentTrackingConfigUpdateResponseStateSchema, Schema.Null),
      { default: () => null }
    ),
    effectiveTrackingState: Schema.optionalWith(Schema.Union(AgentTrackingEffectiveStateSchema, Schema.Null), {
      default: () => null,
    }),
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
export type AgentTrackingConfigUpdateTargetScope = Infer<typeof AgentTrackingConfigUpdateTargetScopeSchema>;
export type AgentTrackingConfigUpdateTarget = Infer<typeof AgentTrackingConfigUpdateTargetSchema>;
export type ParentTrackingConfigUpdatedEvent = Infer<typeof ParentTrackingConfigUpdatedEventSchema>;
export type ChildTrackingConfigUpdatedEvent = Infer<typeof ChildTrackingConfigUpdatedEventSchema>;
export type TrackingConfigUpdateResponse = Infer<typeof TrackingConfigUpdateResponseSchema>;

export function defaultAgentTrackingRetentionSettingsWriteRequest(): AgentTrackingRetentionSettingsWriteRequest {
  return AgentTrackingRetentionSettingsWriteRequestSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    requestedRetentionWindowHours: 168,
    requestedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionState.RetainAfterAlertResolved,
    requestedParentExportState: AgentTrackingParentExportState.NotPrepared,
    requestedRemoteSyncState: AgentTrackingRemoteSyncState.Disabled,
    requestedRemoteAiState: AgentTrackingRemoteAiState.Disabled,
    sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
    sourceReadModelProofRefs: AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
  });
}

export type AgentTrackingRetentionSettingsWriteResultFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export const AgentTrackingRetentionSettingsWriteResultParseState = {
  Parsed: 'parsed',
  Failed: 'failed',
} as const;

export type AgentTrackingRetentionSettingsWriteResultParseState =
  (typeof AgentTrackingRetentionSettingsWriteResultParseState)[
    keyof typeof AgentTrackingRetentionSettingsWriteResultParseState
  ];

export type AgentTrackingRetentionSettingsWriteResultParseResult =
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Parsed;
      readonly value: AgentTrackingRetentionSettingsWriteResult;
    }
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Failed;
      readonly reason: AgentTrackingRetentionSettingsWriteResultFailureReason;
    };

export function parseAgentTrackingRetentionSettingsWriteResultEvent(
  event: AgentEventEnvelope
): AgentTrackingRetentionSettingsWriteResultParseResult {
  if (event.event !== AgentEvent.ActivityTrackingRetentionSettingsWriteReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentTrackingRetentionSettingsWriteResultSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Parsed,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentTrackingRetentionSettingsWriteResultFailureReason
): AgentTrackingRetentionSettingsWriteResultParseResult {
  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason,
  };
}
