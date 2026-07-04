import type {
  GeneratedDevLogEntry as GeneratedPortalDevLogEntry,
  GeneratedLogFields as GeneratedPortalLogFields,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import {
  resolvePortalDevLogBridgeUrl as resolvePortalDevLogBridgeUrlImpl,
  resolvePortalProofTraceConfig as resolvePortalProofTraceConfigImpl,
  sendPortalDevLogWithContext as sendPortalDevLogWithContextImpl,
  sendPortalProofTraceLogWithContext as sendPortalProofTraceLogWithContextImpl,
} from './dev-logger-impl';
import type {
  PortalProofTraceOptions as PortalProofTraceOptionsImpl,
  PortalProofTraceConfig as PortalProofTraceConfigImpl,
  PortalLoggerRuntimeConfig as PortalLoggerRuntimeConfigImpl,
  PortalLoggerDispatchContext as PortalLoggerDispatchContextImpl,
  PortalLoggerRuntime as PortalLoggerRuntimeImpl,
} from './dev-logger-impl';

export type PortalProofTraceOptions = PortalProofTraceOptionsImpl;
export type PortalProofTraceConfig = PortalProofTraceConfigImpl;
export type PortalLoggerRuntimeConfig = PortalLoggerRuntimeConfigImpl;
export type PortalLoggerDispatchContext = PortalLoggerDispatchContextImpl;
export type PortalLoggerRuntime = PortalLoggerRuntimeImpl;
export type PortalDevLogEntry = GeneratedPortalDevLogEntry;
export type PortalDevLogFields = GeneratedPortalLogFields;
export const resolvePortalDevLogBridgeUrl = resolvePortalDevLogBridgeUrlImpl;
export const resolvePortalProofTraceConfig = resolvePortalProofTraceConfigImpl;
export const sendPortalDevLogWithContext = sendPortalDevLogWithContextImpl;
export const sendPortalProofTraceLogWithContext = sendPortalProofTraceLogWithContextImpl;
