import {
  type GeneratedLogFields as LogFields,
  type GeneratedLogMessage as LogMessage,
  type GeneratedStackTrace as StackTrace,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
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

export function writePortalDevLog(message: LogMessage | string, fields: LogFields = {}): void {
  void sendPortalDevLog(message, fields);
}

export function writePortalProofTraceLog(
  message: LogMessage | string,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {}
): void {
  void sendPortalProofTraceLog(message, proofTrace, fields);
}

export async function sendPortalDevLog(
  message: LogMessage | string,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalDevLogWithContext(message as LogMessage, fields, {
    endpoint,
    runtime,
    stackTrace: (new Error().stack ?? '') as StackTrace,
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalDevLogBridgeUrl(runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime) {
  return resolvePortalDomainDevLogBridgeUrl(runtime);
}

export async function sendPortalProofTraceLog(
  message: LogMessage | string,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalProofTraceLogWithContext(message as LogMessage, proofTrace, fields, {
    endpoint,
    runtime,
    stackTrace: (new Error().stack ?? '') as StackTrace,
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalProofTraceConfig(
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): PortalProofTraceConfig {
  return resolvePortalDomainProofTraceConfig(runtime);
}
