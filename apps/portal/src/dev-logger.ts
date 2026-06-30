import {
  type GeneratedLogFields as LogFields,
  type GeneratedLogMessage as LogMessage,
} from '@ocentra-parent/schema-domain/generated/logging-contracts';
import {
  resolvePortalDevLogBridgeUrl as resolvePortalDomainDevLogBridgeUrl,
  resolvePortalProofTraceConfig as resolvePortalDomainProofTraceConfig,
  sendPortalDevLogWithContext,
  sendPortalProofTraceLogWithContext,
  type PortalLoggerRuntime,
  type PortalProofTraceConfig as PortalDomainProofTraceConfig,
  type PortalProofTraceOptions as PortalDomainProofTraceOptions,
} from '@ocentra-parent/portal-domain/dev-logger';

export type PortalProofTraceOptions = PortalDomainProofTraceOptions;
export type PortalProofTraceConfig = PortalDomainProofTraceConfig;

export function writePortalDevLog(message: LogMessage, fields: LogFields = {}): void {
  void sendPortalDevLog(message, fields);
}

export function writePortalProofTraceLog(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {}
): void {
  void sendPortalProofTraceLog(message, proofTrace, fields);
}

export async function sendPortalDevLog(
  message: LogMessage,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalDevLogWithContext(message, fields, {
    endpoint,
    runtime,
    stackTrace: new Error().stack ?? '',
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalDevLogBridgeUrl(runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime) {
  return resolvePortalDomainDevLogBridgeUrl(runtime);
}

export async function sendPortalProofTraceLog(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalProofTraceLogWithContext(message, proofTrace, fields, {
    endpoint,
    runtime,
    stackTrace: new Error().stack ?? '',
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalProofTraceConfig(
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): PortalProofTraceConfig {
  return resolvePortalDomainProofTraceConfig(runtime);
}
