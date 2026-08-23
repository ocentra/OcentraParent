import type http from 'node:http';
import { redactStructuredLogValue } from '../core/log-redaction';
import { bridgeEntryToStoredLog } from '../test-log/bridgeConvert';
import { appendTestLogEntries } from '../test-log/ndjsonWriter';
import type { StoredTestLogLine } from '../test-log/types';
import { generatedHasRunInfoConflict } from '../parent-log-runtime';
import { BridgeEntryArraySchema, type BridgeEntry } from './bridgeLogPayload';
import { readBridgeRequestBody, sendBridgeJson } from './bridgeHttp';
import type { BridgeLifecycleStateStore } from './bridgeLifecycleState';

function redactedStoredLog(entry: BridgeEntry): StoredTestLogLine {
  const stored = bridgeEntryToStoredLog(entry);
  return stored.data == null
    ? stored
    : { ...stored, data: JSON.stringify(redactStructuredLogValue(JSON.parse(stored.data) as unknown)) };
}

export async function handleBridgeLogs(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore
): Promise<void> {
  let payload;
  let storedLogs: StoredTestLogLine[];
  try {
    payload = BridgeEntryArraySchema.parse(JSON.parse(await readBridgeRequestBody(request)) as unknown);
    storedLogs = payload.map(redactedStoredLog);
  } catch {
    sendBridgeJson(response, 400, { ok: false, error: 'invalid log payload' });
    return;
  }
  if (generatedHasRunInfoConflict(lifecycle.runInfo(), payload)) {
    sendBridgeJson(response, 409, { ok: false, error: 'stale run info mismatch' });
    return;
  }
  try {
    appendTestLogEntries(storedLogs, rootDir);
    lifecycle.recordStored(payload);
    sendBridgeJson(response, 200, { ok: true, stored: storedLogs.length });
  } catch {
    sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
  }
}
