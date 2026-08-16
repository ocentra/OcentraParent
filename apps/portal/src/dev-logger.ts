import {
  type GeneratedLogFields as LogFields,
  type GeneratedLogMessage as LogMessage,
  type GeneratedStackTrace as StackTrace,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { decodeLogMessage } from '@ocentra-parent/logging-domain/logging-contracts';
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

function decodePortalLogMessage(message: unknown): LogMessage {
  return decodeLogMessage(message) as unknown as LogMessage;
}

export function writePortalDevLog(message: unknown, fields: LogFields = {}): void {
  void sendPortalDevLog(message, fields);
}

export function writePortalProofTraceLog(
  message: unknown,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {}
): void {
  void sendPortalProofTraceLog(message, proofTrace, fields);
}

export async function sendPortalDevLog(
  message: unknown,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalDevLogWithContext(decodePortalLogMessage(message), fields, {
    endpoint,
    runtime,
    stackTrace: (new Error().stack ?? String()) as StackTrace,
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalDevLogBridgeUrl(runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime) {
  return resolvePortalDomainDevLogBridgeUrl(runtime);
}

export async function sendPortalProofTraceLog(
  message: unknown,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl(),
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): Promise<boolean> {
  return sendPortalProofTraceLogWithContext(decodePortalLogMessage(message), proofTrace, fields, {
    endpoint,
    runtime,
    stackTrace: (new Error().stack ?? String()) as StackTrace,
    moduleUrl: import.meta.url,
  });
}

export function resolvePortalProofTraceConfig(
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): PortalProofTraceConfig {
  return resolvePortalDomainProofTraceConfig(runtime);
}
