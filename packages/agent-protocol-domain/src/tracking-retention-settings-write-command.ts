import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const RetentionWriteText = Schema.String.pipe(Schema.minLength(1));

export const AgentTrackingRetentionSettingsWriteKindSchema = withParser(
  Schema.Literal(
    'retention-window-setting',
    'delete-after-alert-setting',
    'parent-export-setting',
    'remote-sync-disabled-setting',
    'remote-ai-disabled-setting'
  )
);

export const AgentTrackingRetentionSettingsWriteStateSchema = withParser(
  Schema.Literal('service-write-command-accepted', 'service-write-command-rejected')
);

export const AgentTrackingConfigUpdateResponseStateSchema = withParser(Schema.Literal('applied', 'rejected'));

export const AgentTrackingEffectiveStateSchema = withParser(Schema.Literal('enabled', 'disabled', 'degraded'));

export const AgentTrackingConfigUpdateEventType = {
  Parent: 'tracking.config.updated.parent',
  Child: 'tracking.config.updated.child',
} as const;

export const AgentTrackingConfigUpdateTargetScopeSchema = withParser(
  Schema.Literal('family', 'child-profile', 'child-device', 'device-group')
);

export const AgentTrackingRetentionSettingsWriteRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    commandId: RetentionWriteText,
    settingsKind: AgentTrackingRetentionSettingsWriteKindSchema,
    requestedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    requestedDeleteAfterAlertResolved: Schema.Boolean,
    requestedParentExport: Schema.Boolean,
    requestedRemoteSyncEnabled: Schema.Literal(false),
    requestedRemoteAiEnabled: Schema.Literal(false),
    sourceWriterIntentRefs: Schema.Array(RetentionWriteText),
    sourceReadModelProofRefs: Schema.Array(RetentionWriteText),
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
          request.settingsKind !== 'retention-window-setting' ||
          request.requestedRetentionWindowHours !== null ||
          'Retention-window write requests must include a retention window'
      )
    )
);

export const AgentTrackingConfigUpdateTargetSchema = withParser(
  Schema.Struct({
    scope: AgentTrackingConfigUpdateTargetScopeSchema,
    deviceId: RetentionWriteText,
    platform: RetentionWriteText,
    route: RetentionWriteText,
  })
);

export const ParentTrackingConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    sourceCommandId: RetentionWriteText,
    sourceMessageId: RetentionWriteText,
    sourcePeerId: RetentionWriteText,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingRetentionSettingsWriteRequestSchema,
  })
);

export const ChildTrackingConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    parentEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Parent),
    sourceCommandId: RetentionWriteText,
    target: AgentTrackingConfigUpdateTargetSchema,
    config: AgentTrackingRetentionSettingsWriteRequestSchema,
  })
);

export const TrackingConfigUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    sourceCommandId: RetentionWriteText,
    responseState: AgentTrackingConfigUpdateResponseStateSchema,
    effectiveTrackingState: AgentTrackingEffectiveStateSchema,
    childEventType: Schema.Literal(AgentTrackingConfigUpdateEventType.Child),
    target: AgentTrackingConfigUpdateTargetSchema,
    localServiceStateRevision: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    durableSettingsPersisted: Schema.Boolean,
  })
);

export const AgentTrackingRetentionSettingsWriteResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    commandId: RetentionWriteText,
    settingsKind: AgentTrackingRetentionSettingsWriteKindSchema,
    writeState: AgentTrackingRetentionSettingsWriteStateSchema,
    acceptedAt: RetentionWriteText,
    sourceWriterIntentRefs: Schema.Array(RetentionWriteText),
    sourceReadModelProofRefs: Schema.Array(RetentionWriteText),
    sourceMutationProofRefs: Schema.Array(RetentionWriteText),
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    localServiceStateRevision: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    localServiceStateSnapshotRef: RetentionWriteText,
    durableSettingsStoreRef: RetentionWriteText,
    durableSettingsPersisted: Schema.Boolean,
    childConfigResponseState: Schema.optionalWith(
      Schema.Union(AgentTrackingConfigUpdateResponseStateSchema, Schema.Null),
      { default: () => null }
    ),
    effectiveTrackingState: Schema.optionalWith(Schema.Union(AgentTrackingEffectiveStateSchema, Schema.Null), {
      default: () => null,
    }),
    childConfigAckReceived: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    commandTransportClaimed: Schema.Literal(true),
    serviceWritePreflightClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Boolean,
    portalWritableUiClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
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
          result.writeState !== 'service-write-command-accepted' ||
          result.commandTransportClaimed ||
          'Accepted write result must prove command transport'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== 'service-write-command-accepted' ||
          result.serviceMutationExecuted ||
          'Accepted write result must execute the local service mutation'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.settingsKind !== 'retention-window-setting' ||
          result.appliedRetentionWindowHours !== null ||
          'Retention-window write results must include the applied retention window'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== 'service-write-command-accepted' ||
          result.localServiceStateRevision !== null ||
          'Accepted write results must include a local service state revision'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== 'service-write-command-accepted' ||
          result.durableSettingsPersisted ||
          'Accepted write results must persist local durable settings'
      )
    )
);

export type AgentTrackingRetentionSettingsWriteKind = Infer<typeof AgentTrackingRetentionSettingsWriteKindSchema>;
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
    commandId: 'tracking-retention-settings-write-command',
    settingsKind: 'retention-window-setting',
    requestedRetentionWindowHours: 168,
    requestedDeleteAfterAlertResolved: false,
    requestedParentExport: false,
    requestedRemoteSyncEnabled: false,
    requestedRemoteAiEnabled: false,
    sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
    sourceReadModelProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
  });
}

export type AgentTrackingRetentionSettingsWriteResultFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentTrackingRetentionSettingsWriteResultParseResult =
  | {
      readonly ok: true;
      readonly value: AgentTrackingRetentionSettingsWriteResult;
    }
  | {
      readonly ok: false;
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
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentTrackingRetentionSettingsWriteResultFailureReason
): AgentTrackingRetentionSettingsWriteResultParseResult {
  return {
    ok: false,
    reason,
  };
}
