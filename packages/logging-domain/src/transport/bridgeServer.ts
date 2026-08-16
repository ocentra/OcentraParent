import http from 'node:http';
import {
  RunType,
  TestLogScope,
  parseRunTypeOrDefault,
  parseSuiteTypeOrNull,
  parseTestLogScopeOrDefault,
  type RunType as RunTypeValue,
  type TestLogScope as TestLogScopeValue,
  type TestSuiteType,
} from '../test-log/types';
import { clearDirectory, getDefaultLogRoot } from '../test-log/ndjsonPaths';
import { appendTestLogEntries } from '../test-log/ndjsonWriter';
import { wipeNdjsonScope } from '../test-log/wipeNdjsonScope';
import { bridgeEntryToStoredLog } from '../test-log/bridgeConvert';
import { BridgeEntryArraySchema } from './bridgeLogPayload';
import {
  generatedHasRunInfoConflict,
  generatedStaleRunInfoWarning,
  resolveGeneratedBridgeRoute,
} from '../parent-log-runtime';

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
  runType: RunTypeValue;
  suiteType: TestSuiteType | null;
  scope: TestLogScopeValue | null;
  startedAt: number | null;
}

type BridgeRoute = 'health' | 'run-info' | 'run-started' | 'logs' | 'flush' | 'not-found';

interface ParsedRunStartedPayload {
  readonly runId: string | null;
  readonly runType: RunTypeValue;
  readonly suiteType: TestSuiteType | null;
  readonly scope: TestLogScopeValue;
  readonly filePath: string | null;
  readonly wipeAll: boolean;
}

function sendJson(response: http.ServerResponse, statusCode: number, body: object): void {
  response.statusCode = statusCode;
  response.setHeader('Content-Type', 'application/json');
  response.end(JSON.stringify(body));
}

function applyCorsHeaders(response: http.ServerResponse): void {
  response.setHeader('Access-Control-Allow-Origin', '*');
  response.setHeader('Access-Control-Allow-Methods', 'GET,POST,OPTIONS');
  response.setHeader('Access-Control-Allow-Headers', 'Content-Type');
}

function sendBadRequest(response: http.ServerResponse, error: string): void {
  sendJson(response, 400, {
    ok: false,
    error,
  });
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

function staleRunInfoWarning(runInfo: BridgeRunInfoState): string | null {
  return generatedStaleRunInfoWarning(runInfo.runId, runInfo.startedAt, Date.now());
}

function parseRunStartedPayload(rawBody: string): ParsedRunStartedPayload {
  const payload = rawBody.trim().length === 0 ? {} : (JSON.parse(rawBody) as RunStartedPayload);
  return {
    runId: payload.runId ?? null,
    runType: parseRunTypeOrDefault(payload.runType ?? null, RunType.Single),
    suiteType: parseSuiteTypeOrNull(payload.suiteType ?? null),
    scope: parseTestLogScopeOrDefault(payload.scope ?? null, TestLogScope.ParentTest),
    filePath: payload.filePath ?? null,
    wipeAll: payload.wipeAll === true,
  };
}

function wipeRunLogs(rootDir: string, payload: ParsedRunStartedPayload): void {
  if (payload.wipeAll) {
    clearDirectory(rootDir);
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

function updateRunInfo(runInfo: BridgeRunInfoState, payload: ParsedRunStartedPayload): void {
  runInfo.runId = payload.runId;
  runInfo.runType = payload.runType;
  runInfo.suiteType = payload.suiteType;
  runInfo.scope = payload.scope;
  runInfo.startedAt = Date.now();
}

function resolveRoute(method: string, pathname: string): BridgeRoute {
  return resolveGeneratedBridgeRoute(method, pathname);
}

function sendHealthResponse(response: http.ServerResponse): void {
  sendJson(response, 200, { ok: true });
}

function sendRunInfoResponse(response: http.ServerResponse, runInfo: BridgeRunInfoState): void {
  sendJson(response, 200, {
    ok: runInfo.runId != null,
    runId: runInfo.runId,
    runType: runInfo.runType,
    suiteType: runInfo.suiteType,
    scope: runInfo.scope,
    startedAt: runInfo.startedAt,
  });
}

function sendFlushResponse(response: http.ServerResponse, runInfo: BridgeRunInfoState): void {
  sendJson(response, 200, {
    ok: true,
    runId: runInfo.runId,
    flushed: 0,
  });
}

function sendNotFoundResponse(response: http.ServerResponse): void {
  sendJson(response, 404, { ok: false, error: 'not found' });
}

function createBridgeRouteHandlers(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  runInfo: BridgeRunInfoState
): Record<BridgeRoute, () => Promise<void> | void> {
  return {
    health: () => sendHealthResponse(response),
    'run-info': () => sendRunInfoResponse(response, runInfo),
    'run-started': () => handleRunStarted(request, response, rootDir, runInfo),
    logs: () => handleLogs(request, response, rootDir, runInfo),
    flush: () => sendFlushResponse(response, runInfo),
    'not-found': () => sendNotFoundResponse(response),
  };
}

async function handleRunStarted(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  runInfo: BridgeRunInfoState
): Promise<void> {
  try {
    const payload = parseRunStartedPayload(await readRequestBody(request));
    const staleWarning = staleRunInfoWarning(runInfo);
    wipeRunLogs(rootDir, payload);
    updateRunInfo(runInfo, payload);
    sendJson(response, 200, {
      ok: true,
      ...(staleWarning != null ? { warning: staleWarning } : {}),
    });
  } catch (error) {
    void error;
    sendBadRequest(response, 'invalid run-start payload');
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
    if (generatedHasRunInfoConflict(runInfo, payload)) {
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
    void error;
    sendBadRequest(response, 'invalid log payload');
  }
}

export function createBridgeServer(options: BridgeServerOptions = {}): http.Server {
  const rootDir = options.rootDir ?? getDefaultLogRoot();
  const runInfo = createRunInfoState();

  return http.createServer(async (request, response) => {
    const method = request.method ?? 'GET';
    applyCorsHeaders(response);
    if (method === 'OPTIONS') {
      response.statusCode = 204;
      response.end();
      return;
    }

    const url = new URL(request.url ?? '/', `http://${request.headers.host ?? '127.0.0.1'}`);
    const route = resolveRoute(method, url.pathname);
    const routeHandlers = createBridgeRouteHandlers(request, response, rootDir, runInfo);

    await routeHandlers[route]();
  });
}
