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

export const AgentTrackingRetentionSettingsWriteResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    commandId: RetentionWriteText,
    settingsKind: AgentTrackingRetentionSettingsWriteKindSchema,
    writeState: AgentTrackingRetentionSettingsWriteStateSchema,
    acceptedAt: RetentionWriteText,
    sourceMutationProofRefs: Schema.Array(RetentionWriteText),
    commandTransportClaimed: Schema.Literal(true),
    serviceWritePreflightClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(false),
    portalWritableUiClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((result) => result.sourceMutationProofRefs.length > 0 || 'Write result needs proof refs'))
    .pipe(
      Schema.filter(
        (result) =>
          result.writeState !== 'service-write-command-accepted' ||
          result.commandTransportClaimed ||
          'Accepted write result must prove command transport'
      )
    )
);

export type AgentTrackingRetentionSettingsWriteKind = Infer<typeof AgentTrackingRetentionSettingsWriteKindSchema>;
export type AgentTrackingRetentionSettingsWriteResult = Infer<typeof AgentTrackingRetentionSettingsWriteResultSchema>;

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
