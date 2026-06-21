import http from 'node:http';
import {
  RunType,
  TestLogScope,
  parseRunTypeOrDefault,
  parseSuiteTypeOrNull,
  parseTestLogScopeOrDefault,
} from '@ocentra-parent/schema-domain/test-log/types';
import { clearDirectory, getDefaultLogRoot } from '../test-log/ndjsonPaths';
import { appendTestLogEntries } from '../test-log/ndjsonWriter';
import { wipeNdjsonScope } from '../test-log/wipeNdjsonScope';
import { bridgeEntryToStoredLog } from '../test-log/bridgeConvert';
import { BridgeEntryArraySchema } from '@ocentra-parent/schema-domain/transport/bridgeLogPayload';

export interface BridgeServerOptions {
  readonly host?: string;
  readonly port?: number;
  readonly rootDir?: string;
}

interface RunStartedPayload {
  readonly runId?: string;
  readonly runType?: string | null;
  readonly suiteType?: string | null;
  readonly scope?: string | null;
  readonly filePath?: string | null;
  readonly wipeAll?: boolean;
}

interface BridgeRunInfoState {
  runId: string | null;
  runType: string;
  suiteType: string | null;
  scope: string | null;
  startedAt: number | null;
}

const STALE_RUN_INFO_MAX_AGE_MS = 5 * 60 * 1000;

function sendJson(response: http.ServerResponse, statusCode: number, body: object): void {
  response.statusCode = statusCode;
  response.setHeader('Content-Type', 'application/json');
  response.end(JSON.stringify(body));
}

function readRequestBody(request: http.IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    let body = '';
    request.setEncoding('utf8');
    request.on('data', (chunk) => {
      body += chunk;
    });
    request.on('end', () => resolve(body));
    request.on('error', reject);
  });
}

function createRunInfoState(): BridgeRunInfoState {
  return {
    runId: null,
    runType: RunType.Single,
    suiteType: null,
    scope: null,
    startedAt: null,
  };
}

async function handleRunStarted(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  runInfo: BridgeRunInfoState
): Promise<void> {
  try {
    const rawBody = await readRequestBody(request);
    const payload =
      rawBody.trim().length === 0
        ? {}
        : (JSON.parse(rawBody) as RunStartedPayload);
    const scope = parseTestLogScopeOrDefault(payload.scope ?? null, TestLogScope.ParentTest);
    const staleWarning =
      runInfo.runId != null &&
      runInfo.startedAt != null &&
      Date.now() - runInfo.startedAt > STALE_RUN_INFO_MAX_AGE_MS
        ? 'previous run info was stale and has been replaced'
        : null;
    if (payload.wipeAll === true) {
      clearDirectory(rootDir);
    } else {
      wipeNdjsonScope({
        scope,
        runType: parseRunTypeOrDefault(payload.runType ?? null, RunType.Single),
        suiteType: parseSuiteTypeOrNull(payload.suiteType ?? null),
        filePath: payload.filePath ?? null,
        rootDir,
      });
    }
    runInfo.runId = payload.runId ?? null;
    runInfo.runType = parseRunTypeOrDefault(payload.runType ?? null, RunType.Single);
    runInfo.suiteType = parseSuiteTypeOrNull(payload.suiteType ?? null);
    runInfo.scope = scope;
    runInfo.startedAt = Date.now();
    sendJson(response, 200, {
      ok: true,
      ...(staleWarning != null ? { warning: staleWarning } : {}),
    });
  } catch (error) {
    sendJson(response, 400, {
      ok: false,
      error: error instanceof Error ? error.message : String(error),
    });
  }
}

async function handleLogs(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  runInfo: BridgeRunInfoState
): Promise<void> {
  try {
    const rawBody = await readRequestBody(request);
    const payload = BridgeEntryArraySchema.parse(JSON.parse(rawBody) as unknown);
    if (
      runInfo.runId != null &&
      runInfo.scope != null &&
      payload.some((entry) => entry.consumer === runInfo.scope && entry.runId !== runInfo.runId)
    ) {
      sendJson(response, 409, {
        ok: false,
        error: 'stale run info mismatch',
      });
      return;
    }
    const storedLogs = payload.map(bridgeEntryToStoredLog);
    appendTestLogEntries(storedLogs, rootDir);
    sendJson(response, 200, { ok: true, stored: storedLogs.length });
  } catch (error) {
    sendJson(response, 400, {
      ok: false,
      error: error instanceof Error ? error.message : String(error),
    });
  }
}

export function createBridgeServer(options: BridgeServerOptions = {}): http.Server {
  const rootDir = options.rootDir ?? getDefaultLogRoot();
  const runInfo = createRunInfoState();

  return http.createServer(async (request, response) => {
    const method = request.method ?? 'GET';
    const url = new URL(request.url ?? '/', `http://${request.headers.host ?? '127.0.0.1'}`);

    if (method === 'GET' && url.pathname === '/__health__') {
      sendJson(response, 200, { ok: true });
      return;
    }

    if (method === 'GET' && url.pathname === '/__run_info__') {
      sendJson(response, 200, {
        ok: runInfo.runId != null,
        runId: runInfo.runId,
        runType: runInfo.runType,
        suiteType: runInfo.suiteType,
        scope: runInfo.scope,
        startedAt: runInfo.startedAt,
      });
      return;
    }

    if (method === 'POST' && url.pathname === '/__run_started__') {
      await handleRunStarted(request, response, rootDir, runInfo);
      return;
    }

    if (method === 'POST' && url.pathname === '/__logs__') {
      await handleLogs(request, response, rootDir, runInfo);
      return;
    }

    if ((method === 'GET' || method === 'POST') && url.pathname === '/__flush__') {
      sendJson(response, 200, {
        ok: true,
        runId: runInfo.runId,
        flushed: 0,
      });
      return;
    }

    sendJson(response, 404, { ok: false, error: 'not found' });
  });
}
