import type http from 'node:http';
import { isBridgeLoopbackAddress, isBridgeLoopbackRequest, sendBridgeJson } from './bridgeHttp';
import { recoverPendingBridgeStart } from './bridgeRunLifecycle';
import type { BridgeLifecycleStateStore } from './bridgeLifecycleState';

type BridgeControlRoute = 'run-started' | 'flush';
type BridgeRoute = BridgeControlRoute | 'health' | 'run-info' | 'logs' | 'not-found';
export type BridgeAccessMode = 'loopback-only' | 'disabled';

export function assertBridgeServerAccessHost(
  configuredHost: string,
  controlMode: BridgeAccessMode,
  ingestionMode: BridgeAccessMode
): void {
  if (controlMode === 'loopback-only' && !isBridgeLoopbackAddress(configuredHost)) {
    throw new Error('bridge control operations require an explicit loopback host');
  }
  if (ingestionMode === 'loopback-only' && !isBridgeLoopbackAddress(configuredHost)) {
    throw new Error('bridge log ingestion requires an explicit loopback host');
  }
}

export function prepareBridgeServerRequest(
  route: BridgeRoute,
  request: http.IncomingMessage,
  response: http.ServerResponse,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore,
  controlMode: BridgeAccessMode,
  ingestionMode: BridgeAccessMode
): boolean {
  if (route === 'logs') {
    return permitLogIngestion(request, response, ingestionMode);
  }
  if (!isBridgeControlRoute(route)) {
    return true;
  }
  if (controlMode === 'disabled' || !isBridgeLoopbackRequest(request)) {
    sendBridgeJson(response, 403, { ok: false, error: 'bridge control operation is unavailable' });
    return false;
  }
  recoverPendingBridgeStart(rootDir, lifecycle);
  return true;
}

export function recoverBridgeControlAtStartup(
  mode: BridgeAccessMode,
  rootDir: string,
  lifecycle: BridgeLifecycleStateStore
): void {
  if (mode === 'loopback-only') {
    recoverPendingBridgeStart(rootDir, lifecycle);
  }
}

function permitLogIngestion(
  request: http.IncomingMessage,
  response: http.ServerResponse,
  mode: BridgeAccessMode
): boolean {
  if (mode === 'disabled') {
    sendBridgeJson(response, 423, {
      ok: false,
      error: 'bridge log ingestion requires a trusted authenticated transport identity',
    });
    return false;
  }
  if (!isBridgeLoopbackRequest(request)) {
    sendBridgeJson(response, 403, { ok: false, error: 'bridge log ingestion is unavailable outside loopback' });
    return false;
  }
  return true;
}

function isBridgeControlRoute(route: BridgeRoute): route is BridgeControlRoute {
  return route === 'run-started' || route === 'flush';
}
