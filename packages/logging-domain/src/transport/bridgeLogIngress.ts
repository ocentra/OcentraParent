import type http from 'node:http';
import { sanitizeBridgeBatchForCustody } from '../core/logCustody';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { bridgeEntryToStoredLog } from '../test-log/bridgeConvert';
import { appendTestLogEntries } from '../test-log/ndjsonWriter';
import { TestLogScope, type StoredTestLogLine } from '../test-log/types';
import { generatedHasRunInfoConflict } from '../parent-log-runtime';
import type { BridgeEntry } from './bridgeLogPayload';
import { readBridgeRequestBody, sendBridgeJson } from './bridgeHttp';
import type { BridgeLifecycleStateStore } from './bridgeLifecycleState';

function storedLogFromCustodiedEntry(entry: BridgeEntry): StoredTestLogLine {
  return bridgeEntryToStoredLog(entry);
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
    payload = sanitizeBridgeBatchForCustody(JSON.parse(await readBridgeRequestBody(request)) as unknown).map((entry) =>
      entry.consumer == null ? { ...entry, consumer: TestLogScope.ParentTest } : entry
    );
    storedLogs = payload.map(storedLogFromCustodiedEntry);
  } catch {
    sendBridgeJson(response, 400, { ok: false, error: 'invalid log payload' });
    return;
  }
  try {
    const conflict = withLocalArtifactLock(rootDir, () => {
      if (generatedHasRunInfoConflict(lifecycle.runInfo(), payload)) {
        return true;
      }
      appendTestLogEntries(storedLogs, rootDir);
      return false;
    });
    if (conflict) {
      sendBridgeJson(response, 409, { ok: false, error: 'stale run info mismatch' });
      return;
    }
    sendBridgeJson(response, 200, { ok: true, stored: storedLogs.length });
  } catch {
    sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
  }
}
