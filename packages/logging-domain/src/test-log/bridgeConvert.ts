import type { BridgeEntry, BridgeLogPayload } from '../transport/bridgeLogPayload';
import type { RunType as RunTypeValue, StoredTestLogLine, TestLogScope as TestLogScopeValue } from './types';
import {
  bridgeEntryToGeneratedStoredLog,
  bridgePayloadToGeneratedStoredLog,
  createGeneratedBridgeEntryFromStoredLog,
  storedGeneratedLogToBridgeEntry,
  storedGeneratedLogToBridgePayload,
  type GeneratedBridgeEntryOverrides,
  type GeneratedBridgePayloadToStoredLogOptions,
} from '../bridge-log-runtime';

export type BridgePayloadToStoredLogOptions = Omit<GeneratedBridgePayloadToStoredLogOptions, 'consumer' | 'runType'> & {
  readonly consumer?: TestLogScopeValue | null;
  readonly runType?: RunTypeValue;
};

export function bridgePayloadToStoredLog(
  payload: BridgeLogPayload,
  options: BridgePayloadToStoredLogOptions
): StoredTestLogLine {
  return bridgePayloadToGeneratedStoredLog(payload, options) as StoredTestLogLine;
}

export function bridgeEntryToStoredLog(entry: BridgeEntry): StoredTestLogLine {
  return bridgeEntryToGeneratedStoredLog(entry) as StoredTestLogLine;
}

export function storedLogToBridgePayload(log: StoredTestLogLine): BridgeLogPayload {
  return storedGeneratedLogToBridgePayload(log) as BridgeLogPayload;
}

export function storedLogToBridgeEntry(log: StoredTestLogLine): BridgeEntry {
  return storedGeneratedLogToBridgeEntry(log) as BridgeEntry;
}

export function createBridgeEntryFromStoredLog(
  log: StoredTestLogLine,
  overrides: Partial<Pick<BridgeEntry, 'consumer' | 'runId' | 'runType' | 'testName'>> = {}
): BridgeEntry {
  return createGeneratedBridgeEntryFromStoredLog(log, overrides as GeneratedBridgeEntryOverrides) as BridgeEntry;
}
