import {
  DevLogEndpoint,
  DevLogField,
  DevLogHttp,
  DevLogIdPrefix,
  DevLogMessage,
  DevLogEntrySchema,
  LogLevel,
  LogSource,
  decodeLogEntryId,
  decodeLogTimestamp,
  type LogFields,
  type LogMessage,
} from '@ocentra-parent/logging-domain/contracts';

export { DevLogField, DevLogMessage };

export function writePortalDevLog(message: LogMessage, fields: LogFields = {}): void {
  const entry = DevLogEntrySchema.parse({
    schemaVersion: 1,
    id: createPortalLogId(),
    timestamp: decodeLogTimestamp(new Date().toISOString()),
    level: LogLevel.Info,
    source: LogSource.Portal,
    message,
    fields,
  });

  void fetch(DevLogEndpoint.Write, {
    method: DevLogHttp.MethodPost,
    headers: {
      [DevLogHttp.HeaderContentType]: DevLogHttp.ContentTypeJson,
    },
    body: JSON.stringify(entry),
    credentials: DevLogHttp.CredentialsSameOrigin,
  }).catch(() => undefined);
}

function createPortalLogId() {
  const randomId = globalThis.crypto?.randomUUID?.() ?? String(Date.now());
  return decodeLogEntryId(`${DevLogIdPrefix.Portal}${randomId}`);
}
