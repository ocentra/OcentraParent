/* compatibility shim over Rust-generated logging contracts */

import * as generated from '../generated-logging-contracts';

export const LoggingContractRuntime = generated.LoggingContractRuntime;

export const GeneratedLogLevel = generated.GeneratedLogLevel;
export type GeneratedLogLevel = generated.GeneratedLogLevel;
export type GeneratedLogMessage = generated.GeneratedLogMessage;
export const GeneratedLogSource = generated.GeneratedLogSource;
export type GeneratedLogSource = generated.GeneratedLogSource;
export type GeneratedStackTrace = generated.GeneratedStackTrace;
export type GeneratedAgentIdentity = generated.GeneratedAgentIdentity;
export type GeneratedAgentLogEntry = generated.GeneratedAgentLogEntry;
export type GeneratedAgentLogSnapshot = generated.GeneratedAgentLogSnapshot;
export type GeneratedAgentDeviceId = generated.GeneratedAgentDeviceId;
export type GeneratedAgentHostname = generated.GeneratedAgentHostname;
export type GeneratedAgentPlatform = generated.GeneratedAgentPlatform;
export type GeneratedAgentServiceVersion = generated.GeneratedAgentServiceVersion;
export type GeneratedDevLogEntry = generated.GeneratedDevLogEntry;
export type GeneratedLogEntryId = generated.GeneratedLogEntryId;
export type GeneratedLogFieldValue = generated.GeneratedLogFieldValue;
export type GeneratedLogFields = generated.GeneratedLogFields;
export type GeneratedLogSnapshotState = generated.GeneratedLogSnapshotState;
export type GeneratedLogTimestamp = generated.GeneratedLogTimestamp;
