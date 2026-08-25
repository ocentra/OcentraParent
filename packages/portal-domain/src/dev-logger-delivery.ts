import {
  GeneratedDevLogEndpoint as DevLogEndpoint,
  GeneratedDevLogHttp as DevLogHttp,
  GeneratedDevLogIdPrefix as DevLogIdPrefix,
  GeneratedLogEntryIdSchema,
  GeneratedLogLevel as LogLevel,
  GeneratedLogSource as LogSource,
  GeneratedLogTimestampSchema,
  type GeneratedDevLogEntry as DevLogEntry,
  type GeneratedLogFields as LogFields,
  type GeneratedLogMessage as LogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { Schema } from 'effect';
import {
  PortalCompatibilityDefaultConfig,
  resolvePortalCompatibilityUrl,
  type PortalLoggerDispatchContextContract,
  type PortalLoggerRuntime,
  type PortalLoggerRuntimeConfigContract,
} from './dev-logger-contracts';
import {
  custodiedPortalCompatibilityBody,
  custodiedPortalLogFields,
  custodiedPortalLogMessage,
  resolvePortalQueueStorage,
  type PortalQueueStorage,
} from './dev-logger-custody';

type PortalFetch = (
  input: string,
  init: {
    readonly method: string;
    readonly headers: Record<string, string>;
    readonly body: string;
    readonly credentials: string;
  }
) => Promise<{ readonly ok: boolean }>;

const portalLogger = Logger.instance;
export const PortalDevLogRunId = `portal-dev-${createPortalLogToken()}`;

export async function sendPortalBridgeMessage(
  message: LogMessage,
  fields: LogFields,
  runtimeConfig: PortalLoggerRuntimeConfigContract | null,
  context: PortalLoggerDispatchContextContract
): Promise<boolean> {
  if (context.endpoint.length === 0) {
    return false;
  }
  const configuration = buildPortalLoggerConfiguration(context.endpoint, runtimeConfig, context.runtime);
  if (configuration.bridgeQueueStorage == null) {
    return false;
  }
  try {
    portalLogger.configure(configuration);
    portalLogger.register(context.moduleUrl);
  } catch {
    return false;
  }
  const before = portalLogger.logQueueDeliveryState();
  portalLogger.logInfo(message, context.stackTrace, fields, true);
  const after = portalLogger.logQueueDeliveryState();
  if (after.rejectedEntries !== before.rejectedEntries) {
    return false;
  }
  if (after.filteredEntries === before.filteredEntries + 1) {
    return true;
  }
  if (after.queuedEntries !== before.queuedEntries + 1) {
    return false;
  }
  return portalLogger.flush().then(
    () => true,
    () => false
  );
}

export async function sendPortalCompatibilityLog(
  message: LogMessage,
  fields: LogFields,
  runtime: PortalLoggerRuntime
): Promise<boolean> {
  const endpoint = resolvePortalCompatibilityUrl(runtime, DevLogEndpoint.Write);
  const fetchFn = (globalThis as { readonly fetch?: PortalFetch }).fetch;
  if (endpoint == null || fetchFn == null) {
    return false;
  }
  try {
    const body = custodiedPortalCompatibilityBody(createPortalCompatibilityEntry(message, fields));
    if (body == null) {
      return false;
    }
    return fetchFn(endpoint, {
      method: DevLogHttp.MethodPost,
      headers: { [DevLogHttp.HeaderContentType]: DevLogHttp.ContentTypeJson },
      body,
      credentials: DevLogHttp.CredentialsSameOrigin,
    }).then(
      (response) => response.ok,
      () => false
    );
  } catch {
    return false;
  }
}

export function createPortalLogToken(): string {
  const runtimeCrypto = (globalThis as { readonly crypto?: { readonly randomUUID?: () => string } }).crypto;
  return runtimeCrypto?.randomUUID?.() ?? `portal-dev-log-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function buildPortalLoggerConfiguration(
  endpoint: string,
  runtimeConfig: PortalLoggerRuntimeConfigContract | null,
  runtime: PortalLoggerRuntime
): PortalLoggerRuntimeConfigContract & {
  readonly bridgeEndpoint: string;
  readonly bridgeQueueStorage: PortalQueueStorage | null;
} {
  const resolved = runtimeConfig ?? PortalCompatibilityDefaultConfig;
  return {
    ...resolved,
    bridgeEndpoint: endpoint,
    runId: runtimeConfig?.runId ?? PortalDevLogRunId,
    bridgeQueueStorage: resolvePortalQueueStorage(runtime),
  };
}

function createPortalCompatibilityEntry(message: LogMessage, fields: LogFields): DevLogEntry {
  const custodiedFields = custodiedPortalLogFields(fields);
  if (custodiedFields == null) {
    throw new Error('portal log fields exceed their custody boundary');
  }
  return {
    schemaVersion: 1,
    id: Schema.decodeUnknownSync(GeneratedLogEntryIdSchema)(`${DevLogIdPrefix.Portal}${createPortalLogToken()}`),
    timestamp: Schema.decodeUnknownSync(GeneratedLogTimestampSchema)(new Date().toISOString()),
    level: LogLevel.Info,
    source: LogSource.Portal,
    message: custodiedPortalLogMessage(message) as LogMessage,
    fields: custodiedFields,
  };
}
