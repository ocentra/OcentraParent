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
    const outcome = withLocalArtifactLock(rootDir, () => {
      if (lifecycle.operatorState() != null) {
        return 'manual-required' as const;
      }
      if (lifecycle.pendingStart() != null) {
        return 'recovery-required' as const;
      }
      if (generatedHasRunInfoConflict(lifecycle.runInfo(), payload)) {
        return 'conflict' as const;
      }
      appendTestLogEntries(storedLogs, rootDir);
      return 'stored' as const;
    });
    if (outcome === 'manual-required') {
      sendBridgeJson(response, 423, { ok: false, error: 'bridge lifecycle requires operator resolution' });
      return;
    }
    if (outcome === 'recovery-required') {
      sendBridgeJson(response, 423, { ok: false, error: 'bridge lifecycle recovery requires loopback control' });
      return;
    }
    if (outcome === 'conflict') {
      sendBridgeJson(response, 409, { ok: false, error: 'stale run info mismatch' });
      return;
    }
    sendBridgeJson(response, 200, { ok: true, stored: storedLogs.length });
  } catch {
    sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
  }
}
