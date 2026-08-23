import http from 'node:http';
import { getDefaultLogRoot } from '../test-log/ndjsonPaths';
import { ensureLocalArtifactRoot } from '../local-artifact-path';
import { localArtifactDirectoryDurability } from '../local-artifact-path';
import { recoverLocalArtifactAppends } from '../local-artifact-append';
import { assertLoggingArtifactRootLayout } from '../local-artifact-tree';
import { resolveGeneratedBridgeRoute } from '../parent-log-runtime';
import { applyBridgeCorsHeaders, hasBridgeJsonContentType, sendBridgeJson } from './bridgeHttp';
import { handleBridgeLogs } from './bridgeLogIngress';
import { BridgeLifecycleStateStore } from './bridgeLifecycleState';
import {
  handleBridgeFlush,
  handleBridgeRunStarted,
  recoverPendingBridgeStart,
  sendBridgeRunInfo,
} from './bridgeRunLifecycle';

export interface BridgeServerOptions {
  readonly host?: string;
  readonly port?: number;
  readonly rootDir?: string;
}

type BridgeRoute = ReturnType<typeof resolveGeneratedBridgeRoute>;

function routeBridgeRequest(
  route: BridgeRoute,
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore
): Promise<void> | void {
  switch (route) {
    case 'health':
      return sendBridgeJson(response, 200, { ok: true, directoryDurability: localArtifactDirectoryDurability() });
    case 'run-info':
      return sendBridgeRunInfo(response, lifecycle.runInfo());
    case 'run-started':
      return handleBridgeRunStarted(request, response, rootDir, lifecycle);
    case 'logs':
      return handleBridgeLogs(request, response, rootDir, lifecycle);
    case 'flush':
      return handleBridgeFlush(request, response, rootDir, lifecycle);
    default:
      return sendBridgeJson(response, 404, { ok: false, error: 'not found' });
  }
}

export function createBridgeServer(options: BridgeServerOptions = {}): http.Server {
  const rootDir = ensureLocalArtifactRoot(options.rootDir ?? getDefaultLogRoot());
  assertLoggingArtifactRootLayout(rootDir);
  recoverLocalArtifactAppends(rootDir);
  const lifecycle = new BridgeLifecycleStateStore(rootDir);
  recoverPendingBridgeStart(rootDir, lifecycle);

  return http.createServer(async (request, response) => {
    applyBridgeCorsHeaders(request, response);
    if ((request.method ?? 'GET') === 'OPTIONS') {
      response.statusCode = 204;
      response.end();
      return;
    }
    if (request.method === 'POST' && !hasBridgeJsonContentType(request)) {
      sendBridgeJson(response, 415, { ok: false, error: 'application/json is required' });
      return;
    }
    try {
      recoverPendingBridgeStart(rootDir, lifecycle);
      const url = new URL(request.url ?? '/', 'http://127.0.0.1');
      await routeBridgeRequest(
        resolveGeneratedBridgeRoute(request.method ?? 'GET', url.pathname),
        request,
        response,
        rootDir,
        lifecycle
      );
    } catch {
      sendBridgeJson(response, 503, { ok: false, error: 'log bridge storage unavailable' });
    }
  });
}
