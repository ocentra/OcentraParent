import {
  GeneratedDevLogBridge as DevLogBridge,
  type GeneratedLogFields as LogFields,
  type GeneratedLogMessage as LogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import {
  type PortalLoggerDispatchContextContract,
  type PortalLoggerRuntime as PortalLoggerRuntimeContract,
  type PortalLoggerRuntimeConfigContract,
  type PortalProofTraceConfigContract,
  type PortalProofTraceOptionsContract,
} from './dev-logger-contracts';
import { PortalDevLogRunId, sendPortalBridgeMessage, sendPortalCompatibilityLog } from './dev-logger-delivery';
import { buildPortalProofTraceDispatch, resolvePortalProofTraceConfigValue } from './dev-logger-proof-trace';

type PortalImportMetaWithEnv = ImportMeta & { readonly env?: Record<string, string | undefined> };

export type PortalProofTraceOptions = PortalProofTraceOptionsContract;
export type PortalProofTraceConfig = PortalProofTraceConfigContract;
export type PortalLoggerRuntimeConfig = PortalLoggerRuntimeConfigContract;
export type PortalLoggerDispatchContext = PortalLoggerDispatchContextContract;
export type PortalLoggerRuntime = PortalLoggerRuntimeContract;

export function resolvePortalDevLogBridgeUrl(runtime: PortalLoggerRuntime = globalThis): string {
  const envUrl = getPortalEnv()[DevLogBridge.EnvironmentUrl]?.trim() ?? '';
  if (envUrl.length > 0) {
    return envUrl;
  }
  const runtimeUrl = runtime[DevLogBridge.GlobalUrlKey];
  return typeof runtimeUrl === 'string' && runtimeUrl.trim().length > 0 ? runtimeUrl.trim() : DevLogBridge.DefaultUrl;
}

export function resolvePortalProofTraceConfig(runtime: PortalLoggerRuntime = globalThis): PortalProofTraceConfig {
  return resolvePortalProofTraceConfigValue(runtime, getPortalEnv());
}

export async function sendPortalDevLogWithContext(
  message: LogMessage,
  fields: LogFields = {},
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  try {
    return (
      (await sendPortalBridgeMessage(message, fields, null, context)) ||
      (await sendPortalCompatibilityLog(message, fields, context.runtime))
    );
  } catch {
    return false;
  }
}

export async function sendPortalProofTraceLogWithContext(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  try {
    const dispatch = buildPortalProofTraceDispatch(
      fields,
      proofTrace,
      resolvePortalProofTraceConfig(context.runtime),
      PortalDevLogRunId
    );
    return (
      dispatch != null &&
      ((await sendPortalBridgeMessage(message, dispatch.fields, dispatch.runtimeConfig, context)) ||
        (await sendPortalCompatibilityLog(message, dispatch.fields, context.runtime)))
    );
  } catch {
    return false;
  }
}

function getPortalEnv(): Record<string, string | undefined> {
  return (import.meta as PortalImportMetaWithEnv).env ?? {};
}
