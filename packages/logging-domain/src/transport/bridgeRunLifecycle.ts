import type http from 'node:http';
import { clearLoggingArtifactRoot } from '../local-artifact-tree';
import { wipeNdjsonScope } from '../test-log/wipeNdjsonScope';
import { generatedStaleRunInfoWarning } from '../parent-log-runtime';
import { sendBridgeJson } from './bridgeHttp';
import { BridgeLifecycleConflictError, type BridgeLifecycleStateStore } from './bridgeLifecycleState';
import type { BridgeRunInfoState, BridgeRunStartState } from './bridgeLifecycleStateCodec';
import { parseFlushRunId, parseRunStartedRequest } from './bridgeServerRequest';

function wipeRunLogs(rootDir: string, payload: BridgeRunStartState): void {
  if (payload.wipeAll) {
    clearLoggingArtifactRoot(rootDir);
    return;
  }
  wipeNdjsonScope({
    scope: payload.scope,
    runType: payload.runType,
    suiteType: payload.suiteType,
    filePath: payload.filePath,
    rootDir,
  });
}

export function recoverPendingBridgeStart(rootDir: string, lifecycle: BridgeLifecycleStateStore): void {
  const pending = lifecycle.pendingStart();
  if (pending == null) {
    return;
  }
  wipeRunLogs(rootDir, pending);
  lifecycle.completeStart();
}

export function sendBridgeRunInfo(response: http.ServerResponse, runInfo: BridgeRunInfoState): void {
  sendBridgeJson(response, 200, { ok: runInfo.runId != null, ...runInfo });
}

export async function handleBridgeRunStarted(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore
): Promise<void> {
  let payload: BridgeRunStartState;
  try {
    payload = await parseRunStartedRequest(request);
  } catch {
    sendBridgeJson(response, 400, { ok: false, error: 'invalid run-start payload' });
    return;
  }
  const runInfo = lifecycle.runInfo();
  const warning = generatedStaleRunInfoWarning(runInfo.runId, runInfo.startedAt, Date.now());
  try {
    lifecycle.prepareStart(payload);
    wipeRunLogs(rootDir, payload);
    lifecycle.completeStart();
    sendBridgeJson(response, 200, { ok: true, ...(warning == null ? {} : { warning }) });
  } catch {
    sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
  }
}

export async function handleBridgeFlush(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  lifecycle: BridgeLifecycleStateStore
): Promise<void> {
  let runId: string;
  try {
    runId = await parseFlushRunId(request, lifecycle.runInfo().runId);
  } catch {
    sendBridgeJson(response, 400, { ok: false, error: 'invalid flush payload' });
    return;
  }
  try {
    sendBridgeJson(response, 200, { ok: true, ...lifecycle.flush(runId) });
  } catch (error) {
    if (error === BridgeLifecycleConflictError) {
      sendBridgeJson(response, 409, { ok: false, error: 'unknown bridge run' });
      return;
    }
    sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
  }
}
