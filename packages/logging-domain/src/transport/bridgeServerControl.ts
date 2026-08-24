import type http from 'node:http';
import { isBridgeLoopbackRequest, sendBridgeJson } from './bridgeHttp';
import { recoverPendingBridgeStart } from './bridgeRunLifecycle';
import type { BridgeLifecycleStateStore } from './bridgeLifecycleState';

type BridgeControlRoute = 'run-started' | 'flush';
type BridgeRoute = BridgeControlRoute | 'health' | 'run-info' | 'logs' | 'not-found';
type BridgeControlMode = 'loopback-only' | 'disabled';

export function prepareBridgeControlRequest(
  route: BridgeRoute,
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore,
  mode: BridgeControlMode
): boolean {
  if (!isBridgeControlRoute(route)) {
    return true;
  }
  if (mode === 'disabled' || !isBridgeLoopbackRequest(request)) {
    sendBridgeJson(response, 403, { ok: false, error: 'bridge control operation is unavailable' });
    return false;
  }
  recoverPendingBridgeStart(rootDir, lifecycle);
  return true;
}

export function recoverBridgeControlAtStartup(
  mode: BridgeControlMode,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore
): void {
  if (mode === 'loopback-only') {
    recoverPendingBridgeStart(rootDir, lifecycle);
  }
}

function isBridgeControlRoute(route: BridgeRoute): route is BridgeControlRoute {
  return route === 'run-started' || route === 'flush';
}
